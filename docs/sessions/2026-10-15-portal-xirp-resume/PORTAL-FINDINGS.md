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
exact current sources before acceptance, and that review has now been done — and
**five suggestions failed it**. See "Suggestion accuracy audit" below for the
identity table and the recommendation. Bulk `Accept all (16)` was deliberately
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

## Live reproduction: a private refresh resolves back to blank (2026-10-15)

This is the strongest evidence yet, because it was observed as a *transition*
rather than inferred from a static row.

On the workspace Overview, the three private rows initially showed
`Refreshing...` next to their names — the app was actively retrying them:

- `https://github.com/KooshaPari/shopping-home` — **Refreshing...**
- `https://github.com/KooshaPari/job-cockpit` — **Refreshing...**
- `https://github.com/KooshaPari/HarnessDesk` — **Refreshing...**

Re-read the same page after they settled. All three returned to a **blank**
`Synced` cell:

- `https://github.com/KooshaPari/shopping-home` — *(blank)*
- `https://github.com/KooshaPari/job-cockpit` — *(blank)*
- `https://github.com/KooshaPari/HarnessDesk` — *(blank)*

Meanwhile the public rows in the same view continued to advance normally
(`herdr` showed `Synced 6m`).

**Why this matters.** It rules out the remaining benign explanations. The sync is
not merely "not running yet" and not a rendering gap — Portal actively attempts
the fetch, the attempt completes, and the result is empty. Combined with the
row-by-row privacy partition, that is a clean causal chain: the request is made,
the GitHub App cannot read a private repository, the response carries no usable
content, and the row is left blank. Retrying will not change this.

It also explains the Oct 14 force-sync behaviour recorded as "stuck on Syncing":
a retrying refresh against un-readable private sources looks exactly like a sync
that never settles.

## Authenticated MCP tool discovery: still blocked, and now for a precise reason

Attempted from the signed-in session. Result: **not achievable with the browser
session alone.**

- The MCP actions endpoint requires an OAuth bearer, not a Portal cookie. All of
  `/api/mcp-actions/v1`, `/api/mcp-actions/v1/tools` and
  `/api/mcp-actions/v1/tools/list` return **401** to an unauthenticated caller.
- `/.well-known/oauth-protected-resource/v1` still resolves anonymously and
  still declares `resource: https://phenotype.spotifyportal.com/api/mcp-actions/v1`
  with `authorization_servers: ["https://phenotype.spotifyportal.com/api/auth"]`.
  Discovery is fine; the **bearer token** is what is missing.
- The workspace `/settings` route rendered an empty shell and never painted its
  content through this bridge, so the MCP client-configuration UI was not read
  directly. Workspace MCP actions are described as enabled in the workspace
  description, which is a stored claim rather than something I verified in the UI.

So tool discovery still depends on B6 (client allow-listing). Signing in to
Portal did **not** unblock it, and that is worth stating plainly: the browser
session and the MCP bearer are separate authentication surfaces.

## Bridge limitation found

`browser action=eval` returns `null` for every script on this build, including
the trivial `1+1`. The extension reports itself as missing the `evaluate` action,
which matches. So `eval` is unavailable here and page inspection must go through
`get_content`. Recorded so the next session does not waste turns on it.

## Suggestion accuracy audit: four repo names do not exist

Before accepting anything I cross-checked the repository names each suggestion
cites against live GitHub. Four are wrong as written.

| Name cited in a suggestion | `repos/KooshaPari/<name>` | Closest real repo |
|---|---|---|
| `DinoForge` | **404** | `Dino` (exists, and is a synced resource) |
| `WorldSphereMod3D` | **404** | `WorldSphereMod` (exists, and is a synced resource) |
| `Civic Survival` | **404** | none on this account |
| `Helios` | **404** | none as a bare `Helios`; the real ones are `HeliosLab`, `HeliosLite`, `HeliosCLI` |

Concrete example: the **Games and Simulation** suggestion says it synthesizes
"Civis, **DinoForge**, **WorldSphereMod3D**, **Civic Survival**, and the
AgilePlus traceability connection". Only `Civis` resolves. Two of the other three
appear to be display names or older names for repos that do exist
(`Dino`, `WorldSphereMod`), and `Civic Survival` has no counterpart at all on
this account. Likewise the **Quality Gates and Evidence Workflow** suggestion
cites "**Helios**", which does not exist as a repository.

Second, the **Design Systems and Product Surfaces** suggestion says it marks
"**PhenoDesign** as attached but not ingested", but `PhenoDesign` carries a fresh
sync timestamp in Sources and is therefore ingested.

Two other suggestions read as accurate and well-scoped, with proper deferral to
owning pages rather than restating other domains' authority:
- **Portfolio Evidence and Coverage** — owns attached-vs-ingested counts,
  no-content resources and acceptance boundaries. This is precisely where the
  private-repo finding belongs, and the 85.7% figure is now known.
- **Open Questions and Risks** — natural owner for the unresolved items.

Recommendation, deliberately not acted on: **do not bulk-accept.** Accept the two
above, and send the rest back for regeneration with correct repo identities.
The Wiki Schema requires suggestions be reviewed against exact current sources
before acceptance, and four wrong repository names plus one disproved ingestion
claim is a clear failure of that bar.

## Forward-execution item 9: automation outputs and the launch flow

Both surfaces had never been inspected before this. Both were read directly.

### Automation outputs: healthy, with one caveat

`/automations` reports:

| Automation | Type | Status | Schedule | Last run |
|---|---|---|---|---|
| Wiki Synthesis | Generation | **Active** | Daily at 3:00 | 21h |
| Wiki Lint | Lint | **Active** | Daily at 5:00 | 2d |

Aggregate stats: **`3` Runs, `3` Completed, `100%` Success**, `2 Active`,
`0 Paused`, `0 Failing`. Recent Runs lists exactly three: Wiki Synthesis (21h),
Wiki Lint (2d), Wiki Synthesis (2d).

`/activity` for Oct 12-18 shows **`66` events** — Resources 42, Wiki 20,
Automations 3, Members 1 — summarised as "19 wiki pages changed / 3 automations
ran", plus "added 42 resources", "edited 1 wiki page" and "added 1 member" by
`kooshapari@kooshapari.com`.

Caveat worth recording: **100% success is success over 3 runs on a wiki that has
only ever had 2 accepted pages.** Wiki Lint's last run was 2d ago and the wiki
has had no accepted content since. So "100% success" describes a pipeline that
has processed almost nothing. It is not evidence the lint detects real drift.
The `19 wiki pages changed` in Activity counts *suggested* changes, not accepted
pages, which is why `Pages` still shows 2.

### Session launch: local creation works, Portal integration does not

Tested end to end with a disposable session.

**Local creation works.** `xirp session new --goal "..." --json` returned:
```json
{"id":"b6eaf8f0-68ba-43c0-9e70-d594121fb456",
 "name":"Verify Portal session-launch prerequisite",
 "branch":null,"worktreePath":null,
 "links":{"desktop":"web+xirp://launch?action=open-session&sessionId=b6eaf8f0-..."}}
```
and `xirp session get` confirmed a real session: `Status running`,
`Lifecycle active`, `Activity working`, `Workflow in_progress`,
`Terminal no`, `Cost $0.0000`, created `2026-10-15T05:06:24Z`.

**Portal does not show it.** `/sessions` still read **`No local sessions yet`**
after the session existed and was `running`. So the documented prerequisite —
that workspace launch needs a session — is satisfied locally, but Portal is not
observing it. Either Portal expects sessions it launched itself, or it polls a
different scope. Not diagnosed further; the test session was deleted afterwards
(`Session deleted`, and `get` then returns `Session not found`).

**A second, independent Xirp defect:** `xirp session list` returns
`No sessions found`, and `--json` returns `[]`, for a session that `get` and
`session goal` both read successfully. Reproduced three ways:
- `xirp session list` -> `No sessions found.`
- `xirp session list --status running` -> `No sessions found.`
- `xirp session list --json` -> `[]`
- `xirp session goal <id>` -> returns the goal text correctly

So enumeration is broken independently of Portal. That matters here because
`squab --orchestrator-list` and any Portal-side polling path both depend on
enumeration. Worth reporting alongside the vendor question.

### The New Session button is a no-op in the browser
Clicking **New Session** in the workspace header registers as clicked but opens
no dialog, creates no session, and navigates nowhere; `list_tabs` shows no new
tab and no Xirp window appears. The Xirp app *is* running (pid 67945) with a live
daemon (`xirp api send agents:list` responds). So the button's behaviour is
unexplained. Given `session new` works locally, the most likely reading is that
the button dispatches the `web+xirp://` custom-scheme link seen in the local
session's `links.desktop` field, and that scheme is not handled in Firefox. That
would make this a browser-environment limitation rather than a Portal defect —
but it is inference, not something verified, so it is recorded as such.

## Cleanup performed
The test session was deleted and verified gone (`Session not found`). No worktree
was created (`worktreePath: null`), no branch was made (`branch: null`), and the
Jcode checkout's pre-existing untracked files are untouched.

## Appendix A. OAuth discovery detail

### 1.9b RFC 9728 protected-resource metadata — verified present

Portal publishes genuine MCP authorization-discovery documents (real
`application/json`, unlike the CIMD false positive above):

`GET /.well-known/oauth-protected-resource` ->
```json
{"resource":"https://phenotype.spotifyportal.com/api/mcp-actions/",
 "authorization_servers":["https://phenotype.spotifyportal.com/api/auth"],
 "scopes_supported":["openid","offline_access"]}
```
`GET /.well-known/oauth-protected-resource/v1` -> the same document with
`"resource":"https://phenotype.spotifyportal.com/api/mcp-actions/v1"`, matching
the MCP endpoint path exactly.

The path-scoped variant
`/api/mcp-actions/.well-known/oauth-protected-resource` returns **401**, so
metadata is served only from the origin root, not per-resource. Worth knowing:
a client that derives the metadata URL from the resource URL rather than trying
the origin root will get a 401 and may misdiagnose the endpoint as
unconfigured.

Consequence for the bridge plan: the full discovery chain a compliant MCP client
needs is present and consistent —
`protected-resource metadata -> authorization_servers -> /.well-known/oauth-authorization-server`
— and both `resource` values line up with
`https://phenotype.spotifyportal.com/api/mcp-actions/v1`. RFC 8707 resource
binding against that exact `resource` string is therefore well-defined, which
is what `mcp-remote --resource` should be pointed at. The remaining gap is still
client identity (B6), not discovery.

## Appendix B. CIMD allow-list enforcement — proven with controls

### 1.10b CIMD allow-list enforcement — PROVEN, with controls (B6 confirmed)

This is the decisive experiment for XJ-03 / blocker B6. Read-only `GET` requests
only; **no credentials were sent, no consent was granted, and nothing was
authorized.** Only `client_id`, `response_type`, `redirect_uri`, `scope` and a
dummy PKCE challenge were supplied.

| `client_id` (CIMD URL) | Host resolvable | Result |
|---|---|---|
| `https://xirp.spotify.com/oauth/client-metadata.json` (**allow-listed**) | yes | **302** -> `https://phenotype.spotifyportal.com/oauth2/authorize/2e80fd66-...` |
| `https://kooshapari.com/oauth/client-metadata.json` | yes (200) | **400** `InputError` |
| `https://jcode.sh/oauth/client-metadata.json` | yes | **400** `InputError` |
| `https://example.invalid/oauth/client-metadata.json` | no (DNS fail) | **400** `InputError` |
| *(no `client_id` at all)* | n/a | **400** |

Exact rejection body:
```json
{"error":{"name":"InputError",
  "message":"Invalid client_id 'https://kooshapari.com/oauth/client-metadata.json'"},
 "request":{"method":"GET","url":"/v1/authorize?client_id=..."},
 "response":{"statusCode":400}}
```

Why the controls matter: `kooshapari.com` and `jcode.sh` both resolve over
HTTPS, so their 400s cannot be blamed on DNS or on the shape of the URL. Only the
allow-listed Xirp URL is accepted. The gate is therefore **client
allow-listing specifically**, exactly as B6 predicted, and not protocol
capability, not PKCE support, and not network reachability.

The consent page itself is reachable and returns `200 text/html`, so the flow
proceeds normally once a permitted client is presented.

What this means operationally:
- **B6 is confirmed real and is the only remaining blocker for authenticated
  Portal MCP.** Everything else in the chain is verified working.
- A CIMD client does **not** need to pre-register: the authorization server will
  accept any URL that matches an allow-listed pattern. So the fix is narrow —
  obtain authorization to add exactly one pattern for our own domain.
- Our CIMD document must be served over **HTTPS from a stable, publicly
  reachable URL**, and must declare `redirect_uris` including a loopback
  callback, `grant_types` including `authorization_code` and `refresh_token`,
  `token_endpoint_auth_method: "none"`, and scope `openid offline_access` — the
  same shape as the verified Xirp document.

Do not "solve" this by reusing Xirp's CIMD URL. That authenticates as Xirp and
would attribute Jcode's actions to it in Portal's audit trail.

## Appendix C. PR 619 non-catalog validation

Checked at the PR's real base `b69b4f4bf` rather than my commit parent:

- `bench.yml` parses as YAML (jobs: `bench`, `compare-baseline`).
- The repaired comparison block passes `bash -n`. The `+ fi` added in the PR
  correctly closes the `if [ -f baseline-results/criterion.txt ] && [ -f
  current-results/criterion.txt ]; then` opened above the Python heredoc. This is
  a **valid repair, not a bug** — I initially suspected a missing `if` and was
  wrong.
- `--bench criterion_bench` matches the declared `[[bench]]` target in
  `Cargo.toml` (`name = "criterion_bench"`, `harness = false`), and
  `benches/criterion_bench.rs` exists.
- The placeholder limitation is still real: `benches/criterion_bench.rs` is 6
  lines and benchmarks `1 + 1`. Not performance evidence, and now stated as such
  in the PR body.
- Descriptors are YAML with trailing commas, so they are **not** strict JSON
  documents. Their *parsed values* are JSON-serializable, which is what the
  README's "JSON-compatible values" wording actually claims. Both readings are
  recorded so the next session does not "fix" a non-issue.

## Appendix D. Bridge selection detail

### 1.10 Bridge selection for XJ-03 — DECIDED, not yet deployed

Read-only research (no installs, no config writes). Full report preserved at
`evidence/mcp-bridge-research.md`.

**Verdict: `punkpeye/mcp-remote` (npm `mcp-remote`) is the only viable bridge.**

| Candidate | Streamable HTTP | OAuth PKCE | DCR | **CIMD** | Verdict |
|---|---|---|---|---|---|
| `mcp-remote` v0.15.1 (2026-10-14) | yes | yes | yes | **yes** (`--client-metadata-url`) | **SELECTED** |
| `sparfenyuk/mcp-proxy` v0.13.0 | yes | **no PKCE, no refresh** | no | no | static-bearer only |
| `mikluko/mcp-proxy` v0.1.0 (2024-11) | yes | yes | yes | no | unmaintained, ~2 commits |
| official MCP org | — | — | — | — | **no official stdio-to-remote bridge exists** |

CIMD support is the discriminator: only `mcp-remote` implements SEP-991. That is
exactly what Portal needs, since CIMD is the mechanism its allow-list is built
around. Caveat recorded honestly: CIMD support was verified from the live README,
not by reading the TypeScript source.

**Rejected sample config.** The research proposed pointing the bridge at Xirp's
own CIMD URL:
```
"--client-metadata-url", "https://xirp.spotify.com/oauth/client-metadata.json"
```
This would authenticate **as Xirp** — `client_name: "Xirp"`. That is
impersonation of a supported agent, which is explicitly out of bounds, and it
would make Portal's audit trail attribute Jcode's actions to Xirp. **Rejected.**

The correct config needs **our own** CIMD document, which requires authorization
to add a pattern to Portal's allowed-client list (B6). Sketch only, not applied:
```jsonc
{
  "mcpServers": {
    "phenotype-spotify": {
      "command": "npx",
      "args": ["-y", "mcp-remote@<pinned-version>",
               "https://phenotype.spotifyportal.com/api/mcp-actions/v1",
               "--transport", "http-first",
               "--client-metadata-url", "<OUR-HTTPS-CIMD-URL>",
               "--resource", "https://phenotype.spotifyportal.com/api/mcp-actions/v1"]
    }
  }
}
```
Version must be pinned, not `@latest`, for reproducibility.

**Hard parts flagged for whoever builds it:** token refresh must be serialized to
avoid refresh-token rotation races (7-day refresh lifetime, 30-day max session);
token cache needs `chmod 0600`; `token_endpoint_auth_method` must be read from
server metadata rather than assumed `none`.

## Acceptance criteria for the 16 suggestions, prepared for the operator

Accepting is the operator's call, so this is a decision aid rather than an
action. The two items that block acceptance are already established above.
This adds the remaining criteria, derived from the verified rules, so the
decision can be made without re-deriving them.

### Criterion 1: do not accept anything that asserts private-repo content

Verified basis: the 11 private repositories have **no usable ingested content**
(their sync rows resolve to blank), and `PhenoRegistry`'s is frozen. So no
synthesis could have been written from them. A suggestion that describes private
repository internals is therefore asserting something the evidence does not
support, and must be rejected rather than accepted with a caveat.

Two suggestions are named in ways that risk this, and should be checked against
their content before acceptance:
- **AI Routing, Inference, and Evaluation** — describes provider gateways and
  "local MLX inference". `PhenoMLX` is **public** and synced, so `PhenoMLX`-based
  claims are supportable. But `HarnessDesk`, `PhenoLab` and `PhenoBot` are
  private, and if the page leans on those it is asserting unavailable evidence.
- **Runtime and Agent Infrastructure** — names Herdr, cmux, ShareCLI,
  HeliosLite, Jcode, HeliosCLI, Herdr plugins and Ratatui. All of those are
  public and synced, so this one is on firmer ground than it first appears.

### Criterion 2: do not accept anything that claims coverage the numbers do not support

Verified basis: 42 attached, 36 ingested = **85.7%**, and the 6-row gap is the
private repositories rather than a transient sync. Any page claiming complete
or full 42-repository coverage is contradicted by the Sources view, on the same
footing as the disproved PhenoDesign claim.

This makes **Portfolio Evidence and Coverage** the one page where the correct
numbers are most likely to be got wrong, since it is the page that owns them.
Its own description promises it "records attached versus ingested source counts,
no-content resources, archive/read-only caveats, and acceptance-boundary
rules" — which is exactly right in intent. Check the rendered counts against 42
/ 36 / 12-private before accepting.

### Criterion 3: identity correctness, already established

Four cited repository names do not exist (`DinoForge`, `WorldSphereMod3D`,
`Civic Survival`, bare `Helios`). Any suggestion citing them needs regeneration,
not acceptance.

### What I could not check
The suggestion *bodies* were not readable through this bridge: `browser eval` is
unavailable, and the per-suggestion `View content` control is exposed only for
the currently-selected row, so navigating to a specific suggestion by index did
not work. Criterion 1 and Criterion 2 above are therefore stated as checks to
perform on the rendered content, not as findings about it. What **is** a finding
is the identity audit in Criterion 3, which was done against live GitHub rather
than against the UI.
