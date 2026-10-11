# Cleanup Closure Notes — 2026-10-11

## Clean history branch created

`feature/upstream-0.88-sync-clean` is a clean-history alternative to
`feature/upstream-0.88-sync`, branched from the round-17 base `3cb835a5e`.

### Diff vs. original

| | Original `feature/upstream-0.88-sync` | Clean `feature/upstream-0.88-sync-clean` |
|---|---|---|
| Head | `545d6d7f4` | current head of clean branch |
| Cascade commits since `3cb835a5e` | 14 | 13 (1 redundant skipped) |
| Redundant `1f6c19ab8` (wrong-syntax lint config) | preserved | **dropped** |
| `1f6c19ab8` is a 1-file (Cargo.toml) change that was superseded by `85305af66` in the same cascade — same intent, correct syntax. Skipping it removes the noise without losing any functionality. |
| Verification | PR #23 all 16 required checks PASS, only Kilo advisory FAILED (known flake) | TBD — local cargo check is the next gate |

### How to use this branch

This is a **non-destructive alternative**. It does NOT replace the
existing PR #23; it sits alongside it. The operator can:

- **Option A (default):** Merge PR #23 as-is, ignore this branch.
- **Option B:** Use this branch as the merge target instead — close PR #23 with a "superseded by clean branch" comment, open a new PR from `feature/upstream-0.88-sync-clean` to `master`.
- **Option C:** Cherry-pick specific commits from this branch onto `master` post-merge if a fully-cleaner history is desired later.

### Issue closure dossier

`docs/sessions/20261011-cleanup-closure/01_ISSUE_CLOSURE_DOSSIER.md`
contains the resolution evidence for issues #37 and #38, plus the
comment text the agent wanted to post (hook-deferred to the operator
inbox per the known inbox notification gap — see
`docs/sessions/20260924-upstream-0.88-sync/10_INBOX_NOTIFICATION_GAP.md`).

