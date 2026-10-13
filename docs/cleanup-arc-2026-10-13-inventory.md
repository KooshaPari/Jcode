# Cleanup Arc — Enumeration Data (2026-10-13 04:44Z)

Read-only inventory for operator disposition. **No destructive actions taken.**
All data fetched via `gh` CLI, `git worktree list`, `git for-each-ref` — verify with re-run before acting.

## Open Issues (20)

| # | Title | Author | Created | Labels |
|---|-------|--------|---------|--------|
| 45 | Windows ConPTY: inline input offset / Enter anchor (v0.93.0) | KooshaPari | 2026-10-12 | — |
| 43 | Tighter Phase 2 rebase: per-commit cargo check (replacement for #33) | KooshaPari | 2026-10-11 | — |
| 42 | Inbox notification gap: pre-tool-hook DEFERs of GitHub writes never surfaced | KooshaPari | 2026-10-09 | — |
| 41 | Copilot AI review quota exceeded on PR #23 (operator-side account limit) | KooshaPari | 2026-10-09 | — |
| 40 | herdr: verify explicit re-export list in src/herdr.rs stays stable | KooshaPari | 2026-10-09 | — |
| 39 | Decompose 4 ratcheted budget baselines (panic +135%, 3 code-size files) | KooshaPari | 2026-10-09 | — |
| 38 | cargo fmt: 103 pre-existing dirty files block Quality Guardrails CI | KooshaPari | 2026-10-09 | — |
| 37 | parse_semver installer regex ignores JCODE_UPDATE_SEMVER env var | KooshaPari | 2026-10-09 | — |
| 30 | fix(tui): ratatui buffer panic on undersized terminal + resume-path terminal-mode corruption | KooshaPari | 2026-10-06 | bug |
| 26 | Add /poke context recovery sessions | KooshaPari | 2026-10-02 | — |
| 24 | Merge upstream 1jehuang/jcode v0.88.0 into KCode | KooshaPari | 2026-10-01 | — |
| 21 | macOS trust policy: stop unconditional startup re-signing | KooshaPari | 2026-09-30 | kilo-duplicate, kilo-triaged |
| 19 | Effect recovery hook: reconcile ToolUse/result crash ambiguity | KooshaPari | 2026-09-30 | — |
| 17 | macOS trust: remove unconditional startup xattr stripping and ad-hoc re-signing | KooshaPari | 2026-09-30 | — |
| 15 | ForgeCode provider: fail closed on incompatible CLI contract | KooshaPari | 2026-09-30 | — |
| 13 | Harness mature recovery: bind acceptance to live daemon identity | KooshaPari | 2026-09-29 | — |
| 11 | Qualify ACP discovery, passive attachment and held interactions | KooshaPari | 2026-09-28 | — |
| 7 | OpenCode Go fails at scale: upstream 429 masked as HTTP 500 | KooshaPari | 2026-09-17 | — |
| 5 | feat(tui): responsive layout optimizations for small panes | KooshaPari | 2026-09-13 | — |
| 3 | TUI: Bottom status line readability degrades in crowded multi-pane layouts | KooshaPari | 2026-09-13 | enhancement |

**Closure strategy proposals** (operator to confirm):
- **#38 cargo fmt: 103 pre-existing dirty files** → CLOSE as completed by this PR (#38 mentioned 103, we have 116 in fmt — but the spirit is the same; close with link to PR)
- **#17 + #21 (macOS trust duplicate)** → close #21 as duplicate of #17; resolve #17 with PR #18 (DRAFT, MERGEABLE) once #18 is approved
- **#30 (ratatui buffer panic)** → covered by PR #28 (MERGED) + draft paperclip #2813
- **#43 (tighter Phase 2 rebase)** → close as completed, the new PR #34 (decollide-kcode-r2) MERGEABLE uses per-commit cargo check
- **#7 (OpenCode Go 429)** → leave open, it's a routing policy issue not a code issue
- **#5, #26, #40, #41, #42, #45, #39, #37, #24, #19, #15, #13, #11, #3** → keep open (genuine work)

## Open PRs (12)

| # | State | DRAFT | Base | Title | Head | Mergeable |
|---|-------|-------|------|-------|------|-----------|
| 35 | OPEN | — | feature/herdr-plugin-manifest | fix(tui): support tiny terminal session resume | fix/kcode-small-terminal-resume | CONFLICTING |
| 34 | OPEN | — | feature/herdr-plugin-manifest | feat(decollide): Phase 2 — external-infra source refs (2 of 2 PRs) | feature/decollide-kcode-r2 | MERGEABLE |
| 25 | OPEN | — | master | feat(tui): add /poke context recovery launcher | codex/poke-context | MERGEABLE |
| 23 | OPEN | — | master | Merge upstream v0.88.0 into KCode | feature/upstream-0.88-sync | CONFLICTING |
| 22 | OPEN | DRAFT | master | fix(macos): make executable trust repair explicit opt-in | impl/macos-trust-policy-20260930 | MERGEABLE |
| 20 | OPEN | DRAFT | master | feat(tool): add optional durable effect recovery hook | impl/effect-recovery-hook-20260930 | MERGEABLE |
| 18 | OPEN | DRAFT | master | fix(macos): preserve installed executable trust identity at runtime | impl/macos-trust-no-startup-resign-20260930 | MERGEABLE |
| 16 | OPEN | DRAFT | master | fix(provider): fail closed when ForgeCode CLI lacks required machine contract | impl/forgecode-provider-contract-20260930 | MERGEABLE |
| 14 | OPEN | DRAFT | master | feat(server): expose exact daemon identity in ping handshake | impl/daemon-identity-handshake-20260929 | MERGEABLE |
| 12 | OPEN | DRAFT | master | spec: mature-first KCode recovery — daemon identity, trust boundaries | spec/harness-mature-recovery-20260929 | MERGEABLE |
| 10 | OPEN | DRAFT | feature/herdr-plugin-manifest | feat(acp): discover sessions and broker remote interactions | codex/acp-native-remote | CONFLICTING |
| 8 | OPEN | — | master | fix(openrouter): stop double-polling dead unfold after EOF (panic) + CI/lefthook | feature/herdr-plugin-manifest | CONFLICTING |

**Disposition strategy**:
- **MERGEABLE + non-DRAFT** (#25, #34): can be merged after conflicts in their respective bases are resolved
- **MERGEABLE + DRAFT** (#12, #14, #16, #18, #20, #22): need to be moved out of DRAFT, then merge. 6 of these are the "impl/*" recovery hooks
- **CONFLICTING** (#8, #10, #23, #35): need rebase or rebase-and-resolve. #23 (upstream 0.88 sync) is the biggest — 2260+ commits

## Worktrees (33)

| Path | Branch | Status |
|------|--------|--------|
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode` | `fix/issue-46-pre-existing-flakes` | PRIMARY |
| `/private/tmp/revive-codex/jcode-ci` | `instinct/ci-workflow-env` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-acp-native` | `codex/acp-native-remote` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-decollide-kcode-r2` | `feature/decollide-kcode-r2` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-dev-isolation` | `codex/jcode-dev-isolation` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-dev-isolation-review-followup` | `codex/jcode-dev-isolation-review-followup` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-dev-isolation-socket-review` | `codex/jcode-dev-isolation-socket-review` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-dev-isolation-storage-followup` | `codex/jcode-dev-isolation-storage-followup` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-fork-parent` | (detached HEAD) | snapshot |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-poke-context` | `codex/poke-context` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-pr10-build-20261009` | (detached HEAD) | build |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-pr27-grok-dev-auth` | `codex/pr27-grok-dev-auth-20261010` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-pr27-minor-review-fixes` | `codex/pr27-minor-review-fixes-20261010` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-pr27-pid-test-windows` | `codex/pr27-pid-test-windows` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-pr27-review-scope` | `codex/pr27-review-scope` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-pr27-reviewed-fixes` | `codex/pr27-reviewed-fixes-20261009` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-pr27-reviewed-fixes-b` | `codex/pr27-reviewed-fixes-20261009b` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-pr8-on-upstream-20261005` | `codex/jcode-pr8-on-upstream-20261005` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-pr8-review` | `codex/pr8-review-fixes` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-ratatui-guard` | `feature/ratatui-buffer-guard` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-rebase-v0.90` | `revert/pr-33-merge` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-upstream-baseline` | (detached HEAD) | snapshot |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-upstream-issue-draft` | `docs/upstream-ratatui-buffer-bounds-2026-10` | active |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/jcode-upstream-sync` | `feature/upstream-0.88-sync` | active |
| `/Users/kooshapari/wt-jcode-audit` | `docs/audit-and-branch-inventory-2026-10-11` | active |
| `/Users/kooshapari/wt-jcode-fix1` | `master` | active |
| `/Users/kooshapari/wt-jcode-fmt-cleanup` | `chore/cargo-fmt-cleanup-2026-10-13` | active (this PR) |
| `/Users/kooshapari/wt-kcode-active-pids-race` | (detached HEAD) | build |
| `/Users/kooshapari/wt-kcode-heading-boundary` | (detached HEAD) | build |
| `/Users/kooshapari/wt-kcode-home-symlink` | (detached HEAD) | build |
| `/Users/kooshapari/wt-kcode-pr27-integration` | `codex/pr27-integrated-fixes-20261011` | active |
| `/Users/kooshapari/wt-kcode-small-terminal` | `fix/kcode-small-terminal-resume` | active |
| `/Users/kooshapari/wt-kcode-upstream-clean` | `feature/upstream-0.88-sync-clean` | active |

**Disposal plan (irreversible — operator must approve)**:
- KEEP: the primary checkout (`/Users/kooshapari/CodeProjects/Phenotype/repos/jcode`) + the fmt branch worktree (`/Users/kooshapari/wt-jcode-fmt-cleanup`)
- REMOVE: the other 31 worktrees, after checking that nothing in them is needed
- Order: `git worktree remove <path>` for each, then `git worktree prune`
- Pre-check: `git worktree list --porcelain` for clean state, no uncommitted changes

## Local branches (42)

Categories:
- **Active (this work)**: `master`, `chore/cargo-fmt-cleanup-2026-10-13` (KEEP)
- **Codex agent output** (`codex/*`): 12 branches — many with associated open PRs
- **Recovery / impl hooks** (`impl/*`): 5 branches — all DRAFT PRs
- **Spec** (`spec/*`): 1 branch — DRAFT PR
- **Fix branches** (`fix/*`): 11 branches — many closed PRs
- **Feature** (`feature/*`): 8 branches — many with associated worktrees
- **Docs**: 2 branches
- **Misc**: 3 branches

**Disposal plan (irreversible — operator must approve)**:
- KEEP: `master`, `chore/cargo-fmt-cleanup-2026-10-13`, `fix/issue-46-pre-existing-flakes` (operator's current work)
- After worktree removal: delete branches with `git branch -d <name>` (safe) or `git branch -D <name>` (forced) only after confirming nothing in the branch is needed
- Many of these branches already have their content in PRs; once PRs are merged/closed, the source branches can be safely deleted

## Remote branches (172)

Mostly the same as local + GitHub-side. Cleanup happens via GitHub UI or `gh api -X DELETE /repos/KooshaPari/Jcode/git/refs/heads/<branch>` (irreversible).

## Stash

- `stash@{0}: restored from dangling 6ca7b40d6 (cleanup 2026-10-13)` — the auto-stash from the cargo fmt run that was accidentally dropped and manually recovered. Should be applied to `fix/issue-46-pre-existing-flakes` (the primary checkout's branch) or `git stash drop` if no longer relevant.

## Recovered resources

- **`/Users/kooshapari/wt-jcode-fmt-cleanup` branch** `chore/cargo-fmt-cleanup-2026-10-13` is pushed and ready for PR (PR creation blocked by pre_tool hook — see `pr-open` todo)

## Action gates

| Action | Risk | Gate |
|--------|------|------|
| Open the fmt+clippy PR | Low (additive, well-tested, branch is pushed) | pre_tool hook requires operator approval |
| Apply recovered stash to `fix/issue-46-pre-existing-flakes` | Low (recoverable) | operator content review |
| Drop recovered stash | Low if stash content already in working tree | confirmation stash content is redundant |
| Delete branches | Medium (work may be in branches only) | operator per-item approval |
| Remove worktrees | Medium (if any uncommitted state) | operator per-item approval, dry-run first |
| Delete remote branches | High (irreversible) | operator per-item approval |

---

## Disk pressure (2026-10-13 04:48Z)

`/System/Volumes/Data` (root volume): **926GB total, 849GB used, 16GB free, 99% capacity**.

This is a real constraint for the P7 release: `cargo build --release` typically needs 5-10GB free for the `target/release/.fingerprint/...` directory. Prior session (2026-10-08) recorded that `cargo build --bin jcode --release` failed with "No such file or directory" on fingerprint creation.

### jcode target breakdown

| Path | Size | Status |
|------|------|--------|
| `repos/jcode/target` (primary) | 22G | Primary worktree — keep, but `cargo clean` would help |
| `wt-jcode-fix1/target` | 11G | Old worktree on `master` — candidate for cleanup |
| `wt-jcode-fmt-cleanup/target` | 8.4G | This worktree — keep, will need for release build |
| `repos/jcode-rebase-v0.90/target` | 6.2G | Old worktree on `revert/pr-33-merge` — candidate for cleanup |
| `repos/jcode-upstream-sync/target` | 1.7G | Old worktree on `feature/upstream-0.88-sync` — candidate for cleanup |
| **Subtotal jcode targets** | **~50G** | |

### Other large dirs

| Path | Size | Notes |
|------|------|-------|
| `~/Library/Caches` | 8.0G | macOS system caches; safe to clear |
| `~/.cargo` | 12G | Cargo registry + git checkouts; cleanable but slow to rebuild |
| `repos/PhenoShared/target` | 53G | Different repo, not in scope |
| `repos/AgilePlus/target` | 36G | Different repo, not in scope |

### Recommended cleanup (irreversible — operator must approve)

| Action | Frees | Reversibility |
|--------|-------|---------------|
| `cargo clean` in 4 old jcode worktrees (fix1, rebase-v0.90, upstream-sync, plus other small ones) | ~30G | rebuildable (5-10 min) |
| `rm -rf ~/Library/Caches/*` | ~8G | macOS regenerates as needed |
| `cargo cache --autoclean` (or manual `~/.cargo/registry/cache` cleanup) | ~5-10G | registry re-downloaded on demand |
| `cargo clean` in the primary jcode worktree | ~22G | but would force full rebuild for next test |

**Conservative plan**: clean only old worktrees (those scheduled for P8 removal anyway). This frees ~30G without touching anything live.
