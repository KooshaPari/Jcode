# m3.1 session-restore fix — outcome (2026-10-15T01:36Z)

This supersedes the "what's next" section of `01_HANDOFF.md`, which was
written when the arc was paused mid-flight.

## Status: COMPLETE in both repos

### Upstream jcode (`KooshaPari/Jcode`, `master`)

| Commit | What |
|---|---|
| `eb0949585` | Regression test (shipped earlier, unvalidated) |
| `6eb00b35d` | Production fix — `crates/jcode-app-core/src/agent/turn_execution.rs` |

The fix, in the `Err` branch of the session-model restore:

```rust
if let Err(_e) = crate::provider::set_model_with_auth_refresh(...) {
    let fallback = self.provider_model();
    logging::warn(&format!(
        "Saved session model '{}' not available, falling back to provider default '{}'",
        model, fallback
    ));
    self.session.model = Some(fallback);
    self.persist_session_best_effort("model fallback");
} else {
    self.reconcile_explicit_provider_pin_route();
}
```

Previously this branch logged an error and left the dead model in the
persisted session, so every subsequent resume repeated the failure.

### KCode fork (`jcode-rebase-v0.90`, `revert/pr-33-merge`)

`29c240ffb` — cherry-pick of `6eb00b35d`. Applied with no conflicts and
`cargo check -p jcode-app-core --all-targets` reports 0 errors.

## Evidence

The prior handoff noted the test shipped with `tx-validated: cargo test
... (pending)`, meaning its validating run never completed. That gap is
now closed from both directions.

**Fail-before**, reproduced in an isolated worktree at the pre-fix commit
`eb0949585`:

```
test agent::tests::restore_session_falls_back_to_provider_default_when_saved_model_is_rejected ... FAILED
panicked at crates/jcode-app-core/src/agent_tests.rs:1171:
assertion `left == right` failed: agent.session.model should be the
provider's default after set_model failure, not the rejected saved value
  left: Some("MiniMax-M3.1-Flash-Preview")
 right: Some("unknown")
test result: FAILED. 0 passed; 1 failed
```

**Pass-after:** target test passes; full `jcode-app-core --lib` suite
reports `1274 passed; 0 failed; 28 ignored`.

**Ratchet gates on the fixed tree:** panic 140, swallowed-error 3461,
test-size 47 tracked, code-size 107 tracked — all OK.

## Second operator observation: model picker

`m3.1-flash-preview not properly available in model picker`

Root-caused and independently re-verified. This is a **code bug**, not a
config typo.

`crates/jcode-base/src/provider/startup.rs:97-100` resolves the built-in
login-provider profile FIRST:

```rust
if let Some(profile) =
    crate::provider_catalog::resolve_openai_compatible_profile_selection(pref)
{
    crate::provider_catalog::apply_openai_compatible_profile_env(Some(profile));
} else if cfg.providers.contains_key(pref) {   // <-- unreachable for "openai-compatible"
    // user-declared [providers.<pref>] block is read here
}
```

`openai_compatible_profile_by_id("openai-compatible")`
(`provider_catalog.rs:309-316`) matches the hard-coded built-in profile,
so the `else if` that would read the user's `[providers.openai-compatible]`
block is dead code. The block IS parsed — `models` and
`context_window = 1000000` — and then discarded.

Two defects stack:

1. **Shadowing** (above). Built-in wins; runtime points at
   `api.openai.com`.
2. **Malformed anyway.** The operator's config block has no `base_url` and
   no `api_key_env`. `normalize_api_base` rejects `""`, so even with
   precedence reversed it would fail with `invalid base_url ''`. The block's
   comment claims the key lives in `~/.local/share/opencode/auth.json` under
   `minimax-coding-plan`, but nothing in the config references that file or
   key name.

Net effect: no key is set, the live catalog fetch 401s, and the picker falls
back to a static list that never contained `MiniMax-M3.1-Flash-Preview`.
`provider-doctor` cannot observe this because its offline tier uses a
synthetic fixture catalog.

### Recommended fixes (operator decision, not applied)

- **Config-side (unblocks immediately):** rename the section to
  `[providers.minimax]`, add `base_url = "https://api.minimax.io/v1"` and
  `api_key_env = "OPENAI_COMPAT_API_KEY"`, and point `default_provider` at
  it. Avoids the built-in id collision entirely and exercises the existing,
  well-tested `apply_named_provider_profile_env_from_config` path.
- **Code-side (fixes it for everyone):** in `startup.rs:97-113`, check
  `cfg.providers.contains_key(pref)` first so a user-declared block beats a
  built-in id. Needs a test covering a shadowing id.
- **Not recommended:** renaming the built-in profile id. Larger blast radius.

## Panic-budget ratchet

`origin/master` baseline is 140 across 46 files and the gate is GREEN.
The prior handoff described it as RED; that was stale — commit `29c85cbdf`
had already refreshed the baseline.

Four refactor branches reduce it to **62**, below the original 77:

| Branch | Change |
|---|---|
| `fix/panic-budget-wave1-pb1` | TUI SSH auth 19 -> 0 (PR #66) |
| `fix/panic-budget-sdk` | `jcode-sdk` auth/ssh 13 -> 0 |
| `fix/panic-budget-acp` | telemetry 5 -> 1, concurrency 2 -> 0, fake_acp 8 -> 0 |
| `fix/panic-budget-small` | 17 small modules 32 -> 0 |

All four merge onto `origin/master` with zero conflicts (28 files,
+384/-200), verified in a scratch worktree.

**The KCode fork cannot take these as cherry-picks.** All four conflict
against `revert/pr-33-merge`; the fork's history diverged 5076 commits
after the PR #33 revert, so the surrounding code differs. The fork also
carries 145 panics against its own 77 baseline. It needs a fresh refactor
pass, not a replay.

## Infrastructure note

The `opencode-go` route returned an identical auth error on two separate
workers (`kitten`, `turkey`) and killed a third (`dromedary`) after 19M
tokens of picker exploration. Dispatch demoted cross-provider to
`openai-compatible:MiniMax-M3` (a separate credential) and work continued.
The 5-key rotation documented for `~/ocgo.env.rtf` may be needed.

Separately, two harness server reloads destroyed in-flight background cargo
work. Long Rust builds should be launched detached (`nohup ... & disown`)
rather than via a tracked background task.