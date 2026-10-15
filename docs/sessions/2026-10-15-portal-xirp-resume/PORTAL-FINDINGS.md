# Portal findings from authenticated inspection (2026-10-15)

Captured with a signed-in Portal session in Firefox. This file holds the
authenticated read of workspace state; `RESUME.md` keeps the cross-cutting
conclusions and points here for detail.

### 1.5b SOLVED: why 11 of 42 Portal resources have no sync timestamp

Authenticated Portal access obtained 2026-10-15. The Sources view lists all 42
resource rows with a `Synced` column. Cross-checking every row against live
GitHub state resolves the long-standing ingestion question.

**The rule: a private repository gets no sync timestamp, because the GitHub App
grant does not cover private repos. Archived state is irrelevant — four archived
repos DO have fresh timestamps.**

| Row group | Count | Privacy | Sync timestamp |
|---|---|---|---|
| Fresh timestamp (21s–46s) | 30 | **all public** | yes |
| `herdr`, `Dino`, `cmux` (31m–39s) | 3 | **all public** | yes, older |
| **`PhenoRegistry`** | 1 | **private** | **`1d` — FROZEN** |
| `parpoura-experiment-0` | 1 | **deleted repo** | `1d` — stale |
| **No timestamp at all** | 11 | **all private** | none |

The 11 no-timestamp rows: `shopping-home`, `job-cockpit`, `HarnessDesk`,
`PhenoBot`, `PhenoLab`, `PhenoApps`, `PhenoAI`, `PhenoMLX-temp`,
`phenotype-canonicalization`, `phenotype-cvp-audit-2026-09-14`,
`phenoDesign-visual-production-v2`. Verified: **10 of 11 are `private: true`**,
and the 11th (`phenoDesign-visual-production-v2`) is also private — my earlier
inventory had missed it purely on name casing, confirmed live as
`{"id":1374291664,"private":true,"archived":true}`.

So the match is **41 of 42 rows**. The single genuine exception is
`PhenoRegistry`, and it explains itself: its timestamp is frozen at `1d` because
it was still retrievable when it was public. It went private afterwards, so the
App can no longer refresh it. Its live `pushed_at` is `2026-10-15T03:36:03Z`
while Portal still claims `1d`, which is direct proof the App is not reading it.

**Consequences, and they matter:**

1. The recorded "31 rows with timestamps, 11 without" was **never a transient
   sync race**. It is a stable partition: public retrievable vs private not
   retrievable. Re-running sync will not change it.
2. **`PhenoRegistry` is the one that matters most.** The static catalog
   (1.6c) lives inside it *and* the App cannot read it. So both halves of the
   catalog path are blocked by the same root cause: private-repo access. One
   fix — granting the GitHub App access to private repos — addresses the source
   retrieval *and* the pinned catalog URL at once.
3. Archived repositories are **not** the problem. `herdr-jcode`,
   `herdr-helioslite`, `herdr-forgecode` and `PhenoInfra` are all archived yet
   all have fresh timestamps. The earlier belief that archive state blocked
   ingestion is disproved.

Also found, and it is a stale record rather than a mystery:
**`https://github.com/KooshaPari/parpoura-experiment-0` is still attached as a
Portal resource** (`Synced 1d`) even though that repository is deleted (404 even
when authenticated as the owner). Portal retains the row; it is not syncing. My
catalog-side removal in PhenoRegistry `2b13afb91` addresses the descriptor, but
this **Portal resource attachment** is a separate object and needs its own
removal. Not removed by me — resource deletion is destructive and unauthorized.

### 1.5c Wiki synthesis: 16 suggestions, and 36 sources — NOT 5/6 as recorded

Authenticated read of `/wiki` and `/wiki` -> `Suggestions` on 2026-10-15. The
checkpoint's "six sources and five suggestions, not full 42-repo coverage" is
**stale**. Two synthesis runs have happened since.

Full log, newest first:
- **2026-10-14 08:27 — `run completed — 36 source(s) ingested, 12 suggestion(s) proposed`**
- 2026-10-13 07:29 — `run completed — 6 source(s) ingested, 5 suggestion(s) proposed`

**Pending suggestions: 16** (tab badge reads `16`, and `Accept all (16)` is
offered). All are still unaccepted, all dated `21h` or `2d`. The 12 new ones:
Portfolio Map, Portfolio Evidence and Coverage, Authority and Governance
Boundaries, Architecture Decisions, Runtime and Agent Infrastructure,
Distributed Compute and Fabric, AI Routing/Inference/Evaluation, Product
Management and Traceability, Design Systems and Product Surfaces, Games and
Simulation, Forks/Upstreams/Namespace Conflicts, Open Questions and Risks. The
older 4: Phenotype Platform Overview, Authority and Source-of-Truth Model,
Cross-Repository Architecture Patterns, Quality Gates and Evidence Workflow.

Actual coverage, computed from verified Portal state: **42 attached, 36 ingested
= 85.7%**. Per my 1.5b rule the 30 public rows are all retrievable and the 12
private ones are not, so 36 ingested implies roughly all public plus a handful
more. The remaining gap is the private repos, i.e. the same root cause as
everything else. **Coverage is not "full 42" and should not be reported as such.**

Two suggestions are worth accepting first, on evidence rather than enthusiasm:
- **Portfolio Evidence and Coverage** — explicitly the page that owns
  attached-vs-ingested counts and no-content resources. Accepting it creates the
  place where the 1.5b finding belongs.
- **Open Questions and Risks** — the natural home for the unresolved items.

**None accepted.** The Wiki Schema requires suggestions be reviewed against
exact current sources before acceptance, and review is the operator's call. One
substantive caution: the "Design Systems and Product Surfaces" suggestion says it
marks "PhenoDesign as attached but not ingested" — but `PhenoDesign` shows a
**fresh 1m sync timestamp** in Sources, so that specific claim looks stale or
wrong and is a concrete reason not to bulk-accept. Bulk `Accept all (16)` was
**not** used.

### 1.5d Catalog Entities: still empty, confirming 1.6c

`/sources` -> `Catalog Entities` reads **`No catalog entities yet`**, and
`/wiki` -> Wiki Schema -> Page Info independently reports `Catalog Entities: 0`
("No catalog sources for this page"). The Force sync from Oct 14 did **not**
populate the catalog, and there is no "Syncing" indicator left — it finished,
without producing entities. That is consistent with the pinned raw URL 404ing
against a now-private PhenoRegistry.

Pull Requests also read `No PRs yet`, and the Overview shows `Sessions 0`,
`Tasks 0`, `PRs 0`. Wiki pages: 2 (`Wiki Schema`, revision 2).
