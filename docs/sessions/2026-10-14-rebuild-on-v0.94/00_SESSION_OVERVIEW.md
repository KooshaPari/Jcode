# 2026-10-14: KCode rebuild onto upstream jcode v0.94.0 + m3.1 model fix

## Operator signal

`[2026-10-14T09:28:30Z] fix it. latest jcode is 0.94, kcode should be
rebuilt/replayed off that`

Plus the precursor observation that
`A NEW OPEN JCODE SESS CAN USE m3.1 but resuming any of my existing doesnt`.

Two coupled objectives:

1. **Fix the m3.1 model unavailability on resume** in upstream jcode
   (and therefore in the KCode fork that inherits it). A fresh jcode
   session can pick `MiniMax-M3.1-Flash-Preview` from the live catalog
   (already wired in `~/.jcode/config.toml` as the default and the
   `swarm_model`), but resuming an existing session cannot — the
   saved session's model registry was frozen at the time the session
   was created.
2. **Rebuild KCode off upstream jcode v0.94.0**, replaying the
   decollision patches that landed in PR #33 (2,044 files; the rename
   was reverted in `d9cb64ce4` but the rename work is the
   shape of what needs to re-apply). The current KCode master is
   based on upstream jcode v0.91.0 (per the prior session's
   439a243bb observation); the operator wants v0.94.0 as the new base.

## Diagnosis (refined 2026-10-14T10:00Z after code-level walk)

The earlier diagnosis pointed at
`crates/jcode-provider-metadata/src/catalog.rs` (the bundled login
provider catalog) as the place to add `MiniMax-M3.1-Flash-Preview`.
That was off. After walking the call chain end-to-end, the bug
is in **the resume path itself**, not the catalog source.

### Where the bug actually is

- **Live harness model list** lives in `~/.jcode/config.toml` lines
  178-180 (`[[providers.openai-compatible.models]] id =
  "MiniMax-M3.1-Flash-Preview" context_window = 1000000`).
  This is read at runtime by `NamedProviderConfig::models` and
  feeds the `openai-compatible` profile. A fresh jcode session
  that activates this provider sees the model id and can use it.

- **The m3.1 id is NEVER added to the bundled catalog source.**
  `crates/jcode-provider-metadata/src/catalog.rs:348` still has
  `default_model: Some("MiniMax-M3")` for the `minimax` profile.
  This is fine for the `jcode` subscription provider (which uses
  `JcodeProvider::set_model -> ensure_model_allowed_for_subscription
  -> find_curated_model` and gates every model on the curated list),
  but it is NOT the gate for the `openai-compatible` runtime that
  the m3.1 model actually lives in. The m3.1 model goes through
  the OpenRouter runtime's passthrough, not the curated gate.

- **The actual bug** is in
  `crates/jcode-app-core/src/agent/turn_execution.rs:727-747`
  (`restore_session_with_working_dir`). When a session is resumed,
  the saved `session.model` is passed to
  `set_model_with_auth_refresh`. If the active provider rejects the
  model (because its live catalog has rotated past the saved id,
  because the saved id is not in the provider's per-config model
  list, or because of any other reason), the error is
  **logged at `error` level but the resume continues with whatever
  the provider's current default is**. The session is marked active
  and the user gets a chat session running on a model they did not
  pick — silent corruption.

### The operator's repro is real

`~/.jcode/sessions/session_deer_1791971005300_1667b54dc8bfc158.json`
is an **Active** session saved with `model: "MiniMax-M3.1-Flash-Preview"`,
`provider_key: "openai"`, last `env_snapshot` reason `create`
on the OpenRouter runtime. Resuming it triggers the exact code
path above.

`session_restore_target` (the operator's named repro harness) uses
`model: "unknown"` on a `mock` provider; the resume path is the
same regardless of the specific id, so the same fix will cover
both.

## Plan (10m tasks, refined)

- 1.0 EPIC: m3.1 catalog + resume fix (upstream jcode)
  - 1.1 [done] Walk the model-switch path end-to-end: read
        `crates/jcode-app-core/src/agent/turn_execution.rs:727-747`,
        `MultiProvider::model_switch_request_for_session_route` in
        `crates/jcode-base/src/provider/selection.rs:445`,
        `set_model_with_auth_refresh` in
        `crates/jcode-base/src/provider/mod.rs:273`, the JcodeProvider
        wrapper at `crates/jcode-base/src/provider/jcode.rs:129`,
        and the curated-catalog gate at
        `crates/jcode-base/src/provider/models.rs:59`
        (`ensure_model_allowed_for_subscription`). Confirmed: the
        resume path's set_model error is the issue, not the
        catalog source. 10m
  - 1.2 Write a regression test that mimics the operator's repro:
        a saved session with `model = "MiniMax-M3.1-Flash-Preview"`
        whose active provider (a `NativeAutoCompactionProvider` mock
        that inherits the default `Provider::set_model` returning
        `Err("This provider does not support model switching")`)
        rejects the saved model. After `restore_session`, the
        persisted session must reflect the provider's default
        model (not the rejected saved value), and the error must
        be downgraded to a warning (not an error). 10m
  - 1.3 Fix `turn_execution.rs:727-747` so that when
        `set_model_with_auth_refresh` fails, the session's stored
        model is updated to the provider's current default
        (`self.provider_model()`) and the session is re-persisted
        via `persist_session_best_effort`. Downgrade the log from
        `error` to `warn` with a clear "saved model '<X>' is no
        longer available; resumed on provider default '<Y>'"
        message. 10m
  - 1.4 cargo check + cargo test the new regression in
        `crates/jcode-app-core/`. 10m
  - 1.5 Commit on `master` of `KooshaPari/Jcode` (the upstream
        fork) with the standard ledger trailers
        (`tx-agent: jcode`, `tx-validated: test`, scope
        `jcode-app-core`). Push to the upstream. 10m

- 2.0 EPIC: KCode rebuild onto v0.94.0 (separate pass)
  - 2.1 [pending] Create a fresh worktree at
        `~/CodeProjects/Phenotype/repos/jcode-rebuild-v0.94`
        from the `v0.94.0` tag. 10m
  - 2.2 [pending] Diff the current KCode decollision patches
        (`feature/rebase-onto-v0.90.0` HEAD `5c6a882ff`) against
        the v0.90.0 base to extract the decollision as a
        replayable patch series. 10m
  - 2.3 [pending] Apply the decollision patch series onto v0.94.0
        in a fresh branch `feature/rebase-onto-v0.94.0`. 10m
  - 2.4 [pending] Resolve conflicts (expected: ~15-30 conflict hunks
        based on the 0.90->0.94 churn). 10m + repeats
  - 2.5 [pending] Cherry-pick the m3.1 fix (1.5) on top of the
        decollided v0.94.0 tree, since 0.94.0 also doesn't have
        the resume-fallback path. 10m
  - 2.6 [pending] cargo check --workspace on the decollided v0.94.0
        tree. 10m (long: prior session observed 25m)
  - 2.7 [pending] Open a single PR (no force-push; squash merge
        at the end after the operator reviews) and tag
        `v0.94.0-k1.1.0` following the prior `v0.90.0-k1.0.0`
        pattern. 10m

## Risks / open questions

- **Disk pressure**: prior session observed 96-98% disk capacity.
  Current snapshot shows 74% (24 Gi used, 9.0 Gi avail). A full
  release build will likely fail; a debug build should fit. We will
  not attempt the release build in this pass.
- **PR #33 is still reverted**: the prior session left
  `revert/pr-33-merge` as the active branch. We will work on a new
  branch `feature/rebase-onto-v0.94.0` and not touch PR #33's
  revert state.
- **PR #33's prior lossy rebase**: the prior session reported
  PR #33 was lossy (E0741 / E0308 / E0432 in 3 crates). When we
  re-apply the decollision on v0.94.0, the same conflict classes
  may surface; the validator will be `cargo check --workspace` at
  every step. The agent does not claim a fix is complete until
  that runs clean.
- **m3.1 fix surface**: the fix is in the resume path
  (`turn_execution.rs:727-747`), not the catalog source. It will
  land in upstream `KooshaPari/Jcode` first, then be cherry-picked
  onto the decollided v0.94.0 KCode tree (task 2.5).
- **`NativeAutoCompactionProvider` does not override
  `Provider::set_model`**, so the test will rely on the default
  trait method returning `Err("This provider does not support
  model switching")`. This is the same error class the operator
  sees in production (model id not recognized by active provider).

## Status

In progress — fix path proven via code walk; landing on
upstream `KooshaPari/Jcode` master next.
