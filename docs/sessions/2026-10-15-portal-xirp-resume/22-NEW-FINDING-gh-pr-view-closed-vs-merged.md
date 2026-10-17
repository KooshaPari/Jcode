# §22 NEW FINDING — standalone: `gh pr view CLOSED` ≠ abandoned (2026-10-17)

## What happened

During a routine status check on PR 103 (a follow-up to PR 67 on
`KooshaPari/KCode`), `gh pr view 103 --json mergeable` returned
`CONFLICTING`. Investigation revealed that PR 67's `gh pr view` output
(`state=CLOSED, mergedAt=null, mergeCommit=null`) was misleading.

The truth was on master: commit `fed23782d` ("chore: prepare Xirp custom
harness compatibility discovery (#67)", authored by `jcode-test
<jcode-test@local>`, our harness test signer) carried the ledger
trailers `tx-validated: cargo-check (passed, 0 errors)` and
`tx-intent: Merge PR #67 (chore/xirp-custom-harness-discovery) into
master via harness-compliant squash-merge path.`

The PR was integrated via the **harness-compliant squash-merge fallback
path** documented in AGENTS.md ("Merge Strategy: Direct-Push When `gh
pr merge` Is Fail-Closed"), which uses `git push` after a local
squash-merge, bypassing GitHub's merge button. From GitHub's perspective
the PR was never merged; from master's perspective it was.

## The pattern (operator takeaway)

> `gh pr view <N>` returning `state=CLOSED` is *not* sufficient evidence
> that the PR was abandoned. Always cross-check with
> `git log <upstream> --grep="#<N>"` for harness squash-merges that
> bypassed the GH merge button.

## The corollary (state-record hygiene)

Any prior session record that said "PR N is CLOSED, not merged" must
be either re-verified against `git log` or re-written. The earlier §16
of this session's STATE.md said exactly that and was wrong; it was
re-written in this same work arc.

## Why it was a good outcome

- The work landed on master via a documented, ledger-trailed,
  harness-compliant path.
- The follow-up additions on this branch (§18–§20, vendor draft, etc.)
  are correctly the subject of PR 103.
- The misread only affected the body framing ("Closes #67" vs
  "Supersedes #67"), not the work itself. The body was re-framed
  before PR 103 was marked ready-for-review.

## Detection heuristics (for future agents)

| Signal | Implication |
|--------|-------------|
| `gh pr view <N>` → `state=CLOSED, mergedAt=null` AND a commit on `<upstream>` with title containing `(#<N>)` | **Likely harness squash-merge.** The PR was integrated via `git push`, not via the GH API. Treat as merged, not abandoned. |
| `gh pr view <N>` → `state=CLOSED, mergedAt=null` AND `git log <upstream> --grep="#<N>"` returns nothing | Truly abandoned (or merged on a different branch). Standard "open a new PR" path applies. |
| `gh pr view <N>` → `state=MERGED, mergedAt=<date>, mergeCommit=<sha>` | Standard GH merge button. Use `mergeCommit` to confirm. |
| A commit on `<upstream>` whose trailer block contains `tx-intent: ... via harness-compliant squash-merge path ...` | **Always** treat the referenced PR as integrated, regardless of `gh` state. |

## Resolution steps (worked example)

1. Verified the branch on `KooshaPari/KCode` was a strict superset of
   master's session-dir snapshot (633-line STATE.md vs master's
   424-line; ours had §18–§20, the §8.2.e APPENDICES addition, the
   vendor draft, the pr-67-reopen-decision file, and the SHA-pointer
   fix).
2. `git fetch origin master`
3. `git merge origin/master --no-ff` (resolved 8 add/add files)
4. `git checkout --ours <file>` for each of the 8 conflicting files
5. `git add` + `git commit` with ledger trailers
6. `git push --no-verify` (no pre-push hook present in this worktree)
7. Re-checked `gh pr view 103 --json mergeable` → `MERGEABLE`
8. `gh pr edit 103 --body-file pr-67-body.md` (retargeted the body)
9. `gh pr ready 103` (DRAFT → ready-for-review)

## Cross-references

- Session STATE.md §22 (this same finding, in session context)
- Session STATE.md §21 (PR 103 rebase onto current master)
- PR 67 (the integrated PR; visible as `state=CLOSED` in `gh`, real
  commit on master as `fed23782d`)
- PR 103 (the follow-up additions; now MERGEABLE+CLEAN+ready)
- AGENTS.md "Merge Strategy: Direct-Push When `gh pr merge` Is
  Fail-Closed" (the path that produced `fed23782d`)
