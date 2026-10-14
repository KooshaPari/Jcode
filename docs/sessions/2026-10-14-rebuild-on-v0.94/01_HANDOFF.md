# Handoff — m3.1 fix arc paused 2026-10-14T23:29Z

Operator signalled `give full handoff so you can return to protfoio\personal
wrk`. Stopping work; this document is the self-contained resumption point.

## TL;DR

- **koosha-phenotype**: arc complete, main @ `c8c2b49`, pushed.
- **upstream jcode** (`KooshaPari/Jcode`): m3.1 fix is **half-shipped**.
  The regression test landed + was pushed (`eb0949585` + ratchet refresh
  `b7d9dc117`). The production fix in `turn_execution.rs:727-747` has
  **not** been written, and the test has not been verified to fail-then-pass.
- **KCode fork** (`jcode-rebase-v0.90`): session overview refined and
  pushed at `4c172c4f2` on `revert/pr-33-merge`. The KCode rebase onto
  v0.94.0 has not started. Once the upstream fix lands, it cherry-picks
  cleanly onto the KCode tree (no merge work needed because the resume
  path is identical in both repos).
- **NEW operator observation (2026-10-14T23:28Z)**: "m3.1-flash-preview
  not properly available in model picker refer to jcode docs". This is
  a **separate symptom** from the resume bug — the picker UI does not
  show m3.1-flash-preview, even though `~/.jcode/config.toml` lists it
  as the `openai-compatible` provider's default + per-model entry. Not
  yet diagnosed. jcode docs (read this turn) point at
  `PROVIDER_DOCTOR.md`'s debugging flow: run
  `jcode provider-doctor openai-compatible --tier offline` and check
  `picker_live_models` / `picker_fallback_labeling` /
  `model_switch_route` checkpoints.

## Repo state

### koosha-phenotype — DONE for this arc

- `main` @ `c8c2b49`, local = origin, pushed.
- 714/714 tests, 0/27 detector.
- 5/5 grader findings terminal in `00_FINDINGS.md`.
- 7 same-day commits, latest `c8c2b49 docs(review): reconcile
  HUMAN_REVIEW.md against 6ad2a63 (7th same-day pass, docs-only
  carry-forward)`.
- Loose `docs/2026-09-XX-*.md` at 26 (deferred per session overview).

### upstream jcode — half-shipped

Path: `~/CodeProjects/Phenotype/repos/jcode`
Remote: `origin = https://github.com/KooshaPari/Jcode.git`,
`upstream = https://github.com/1jehuang/jcode.git`
Working branch: `master`
HEAD: `b7d9dc117` (pushed to `origin/master`).

New commits since session start:

- `eb0949585 test(agent): regression for restore_session falling back
  to provider default on rejected model` — adds
  `restore_session_falls_back_to_provider_default_when_saved_model_is_rejected`
  at `crates/jcode-app-core/src/agent_tests.rs:1119-1187`. Asserts:
  1. status is `Active` after restore,
  2. `agent.session.model` is the provider's default (`"unknown"`),
  not the rejected saved value (`"MiniMax-M3.1-Flash-Preview"`),
  3. the migration is re-persisted to disk so a future
  `Session::load` sees the corrected model.

  **Author/committer: `jcode-test <jcode-test@local>`** (NOT
  `KooshaPari`). The `tx-validated: cargo test ... (pending)`
  trailer marks the test as **unverified** — it was committed
  before the test was actually run. Cargo test on this was the
  background task `429310h7ht` (launched 10:18Z, no output captured,
  task not found at 23:28Z) — the cargo test that should have
  validated the test never produced results.

- `b7d9dc117 chore(ratchet): refresh test_size_budget baseline for
  agent_tests.rs growth` — automatic ratchet baseline refresh
  after the test grew the file from 2232 to 2302 LOC.

- Both commits are on `origin/master`. Working tree is clean (only
  3 untracked docs files from a prior session:
  `docs/drafts/`, `docs/sessions/2026-10-12-cleanup-and-release/`).

### KCode fork — session overview refined, no code changes

Path: `~/CodeProjects/Phenotype/repos/jcode-rebase-v0.90`
Active branch: `revert/pr-33-merge` @ `4c172c4f2`, pushed to
`KooshaPari/Jcode`.

New commit since session start:

- `4c172c4f2 docs(sessions): refine m3.1 diagnosis to point at
  resume path, not catalog` — replaces the old (wrong) diagnosis
  with the code-walk-confirmed one: bug is in
  `turn_execution.rs:727-747`, not in
  `crates/jcode-provider-metadata/src/catalog.rs`. Updates the
  10m task list under EPIC 1.0 to point at the resume path.

Working tree: 1 untracked `docs/sessions/2026-10-08-pilot-readiness/`
from a prior session; leave it alone.

## What's next (in order, smallest first)

### Immediate — finish the m3.1 fix

1. **Verify the test fails on the current (unfixed) tree.** This
   proves the bug is real and the test catches it. Run:
   `cd ~/CodeProjects/Phenotype/repos/jcode && cargo test -p
   jcode-app-core --lib restore_session_falls_back_to_provider_default_when_saved_model_is_rejected`
   in a background task and watch for the expected `assertion
   failed: agent.session.model.as_deref() == Some("unknown")`. If
   the test **passes** on the unfixed tree, the test is wrong —
   the fix is to make the test actually exercise the bug path;
   don't ship a passing test that doesn't catch anything.
   If the test **fails** as expected, proceed to step 2.

2. **Write the production fix** at
   `crates/jcode-app-core/src/agent/turn_execution.rs:727-747`.
   Change:
   ```rust
   if let Err(e) = crate::provider::set_model_with_auth_refresh(...) {
       logging::error(&format!(
           "Failed to restore session model '{}' via '{}': {}",
           model, model_request, e
       ));
   } else {
       self.reconcile_explicit_provider_pin_route();
   }
   ```
   to (semantically):
   ```rust
   if let Err(e) = crate::provider::set_model_with_auth_refresh(...) {
       let fallback = self.provider_model();
       logging::warn(&format!(
           "Saved model '{}' is no longer available ({}); \
            resuming on provider default '{}'",
           model, e, fallback
       ));
       self.session.model = Some(fallback);
       self.persist_session_best_effort("restore model fallback");
   } else {
       self.reconcile_explicit_provider_pin_route();
   }
   ```
   Note the **log-level downgrade** from `error` to `warn`. The
   saved model being unavailable is a known graceful-degradation
   case (catalog rot, m3.1 ↔ m3 swap, etc.), not an error.

3. **Verify the test now passes.** Same cargo test command.
   Confirm `tx-validated: cargo test ...` in the upcoming commit
   reflects a real pass, not the placeholder `(pending)`.

4. **Commit the fix on `master` of `KooshaPari/Jcode`**, then
   push. The commit subject should be something like
   `fix(agent): fall back to provider default when restoring
   session model fails`. Body should reference the test
   commit (`eb0949585`) and the operator's repro
   (`session_deer_1791971005300_1667b54dc8bfc158.json`).
   Keep `tx-validated: test` and add a `tx-task:` trailer
   pointing at this session.

5. **Cherry-pick to KCode fork** at `jcode-rebase-v0.90` on the
   `revert/pr-33-merge` branch. The same patch applies because
   the resume path is byte-identical in both repos. The
   `tx-agent: jcode` trailer will be the same; the cherry-pick
   commit's body should explicitly call out the cross-repo
   source commit.

6. **Update the kcode session overview's Status section** in
   `jcode-rebase-v0.90/docs/sessions/2026-10-14-rebuild-on-v0.94/00_SESSION_OVERVIEW.md`
   to mark EPIC 1.0 complete and remove the "(in progress)"
   language. Commit + push.

### Then — investigate the model picker observation

7. **Read the jcode docs the operator pointed at** (already
   done this turn, see `jcode_docs` results above). The relevant
   doc is `PROVIDER_DOCTOR.md`, "Typical debugging flow" #1:
   "My picker is broken / shows the wrong models." Run
   `jcode provider-doctor openai-compatible --tier offline` and
   check the `picker_live_models`, `picker_fallback_labeling`,
   `model_switch_route` checkpoints.

8. **If the offline tier flags a bug**, walk the picker source.
   The picker's model list comes from a union of:
   - `NamedProviderConfig::models` (from `~/.jcode/config.toml`
     per-provider `[[providers.<name>.models]]`),
   - the live `/models` endpoint (when the provider is online),
   - the bundled catalog
     (`crates/jcode-provider-metadata/src/catalog.rs`).
   The m3.1 entry is in (1). If the picker filters it out, the
   bug is in the union/filter logic, not in config or catalog.
   Likely suspects: `crates/jcode-provider-core/src/models.rs`,
   `crates/jcode-provider-core/src/selection.rs`, the
   `MultiProvider::known_models_for_provider` /
   `picker_live_models` helper. Don't fix until you have the
   diagnostic output.

9. **Document the picker diagnosis** in a new section at the
   bottom of this handoff doc, or in a new
   `01_MODEL_PICKER_DIAGNOSIS.md` sibling.

### Later — KCode rebase onto v0.94.0

10. **Out of scope for this turn.** The KCode rebase onto
    v0.94.0 is a much larger arc (75 commits, ~15-30 conflict
    hunks expected based on prior PR #33 lossy rebase). The
    plan is captured in `00_SESSION_OVERVIEW.md` EPIC 2.0.
    Schedule a dedicated session for it; don't start today.

## Open follow-ups (preserved from prior summary)

- ~26 loose `docs/2026-09-XX-*.md` files in koosha-phenotype
  (clusters: resume authority 3, production construction 3,
  2026-09-10 post-release 5, pre-2026-09-09 8,
  CAREER_CONTEXT_ANDURIL_* 5, ad-hoc audits 2, inventories 2).
- `/colophon` route Neubrutalism (parked).
- Catalog refresh ticket Bento (parked).
- CSS file sizes not re-measured (5 in 300-339 warning band).
- Phantom PR UI state on GitHub (permanent until elicitate
  installed).
- `VERCEL_TOKEN` unset → hosted gate unrun.
- `elicitate` not installed → `gh pr close` / `gh pr merge`
  blocked.
- No Firefox/playwright/chromium in env → cross-browser
  UNKNOWN.

## Tool / harness notes (preserved)

- macOS `cat -A` BSD-incompatible; use `od -c`, `xxd`, or
  `diff <(od -c a) <(od -c b)` for byte inspection.
- `replace` tool's `glob` does NOT support shell brace
  expansion `{a,b,c}`; use `replace_all: true` with
  single-pattern glob, or batch `edit` calls.
- For multi-file edits where files have different structures,
  prefer batched individual `edit` calls over `replace` with
  a glob.
- For byte-level inspection, large `xargs grep` over thousands
  of files exceeds 120s/300s foreground timeout and 200k
  token budget; use `rg` (ripgrep) or pre-filter with
  `wc -l` first, and cancel runaway background tasks with
  `bg cancel <task_id>`.
- For listing many session files, use `rg -l` first to get
  the shortlist, then iterate with `for f in $(rg -l ...)`.
- Cargo test on a fresh `target/` is ~5-25m. **Always run
  in background** with `bg run_in_background`; never block
  the orchestrator on cargo. Capture the output to a file
  path you can `tail` rather than streaming it to context.
- The harness auto-commits test additions authored under
  `jcode-test@local` with `tx-validated: ... (pending)`.
  **Don't trust a `(pending)` trailer** — it means the test
  was committed but the validating cargo invocation did not
  complete. Replace with the actual result before any
  production fix commit.

## Files touched this session

- `~/CodeProjects/Phenotype/repos/jcode-rebase-v0.90/docs/sessions/2026-10-14-rebuild-on-v0.94/00_SESSION_OVERVIEW.md`
  — diagnosis refined + pushed (`4c172c4f2`).
- `~/CodeProjects/Phenotype/repos/jcode/crates/jcode-app-core/src/agent_tests.rs`
  — `restore_session_falls_back_to_provider_default_when_saved_model_is_rejected`
  added (`eb0949585`, pushed; ratchet baseline `b7d9dc117`,
  pushed).
- This handoff doc
  (`docs/sessions/2026-10-14-rebuild-on-v0.94/01_HANDOFF.md`,
  uncommitted).
