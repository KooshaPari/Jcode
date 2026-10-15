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

## CIMD hosting: where the document can actually be served (2026-10-15)

Tested candidate hosts before recommending the append. Two of them look right and
are not, which is worth writing down so nobody repeats the check.

### Both obvious domain choices fail

| Host | Probe | Result |
|---|---|---|
| `https://kooshapari.com/oauth/client-metadata.json` | anonymous GET | **404** |
| `https://jcode.sh/oauth/client-metadata.json` | anonymous GET | **200 — but `text/html`, 98,237 bytes** |
| `https://jcode.sh/definitely-not-real-xyz` (control) | anonymous GET | **200, `text/html`, 98,425 bytes** |

`jcode.sh` is a Cloudflare-hosted SPA (`server: cloudflare`) that returns the
HTML shell with **HTTP 200 for every path**. So the 200 on the CIMD path is the
identical SPA-fallback trap already found on Portal's
`/.well-known/oauth-client-metadata.json`. **Content-type must be checked; a 200
proves nothing on an SPA host.** Neither `kooshapari.com` nor `jcode.sh` can
serve the document as it stands.

### raw.githubusercontent is the viable route, with one caveat

`KooshaPari/Jcode` is `private: false, visibility: public, fork: true`, and
anonymous raw fetches work — e.g. `README.md` and `Cargo.toml` both return 200.
So a CIMD document committed to that repo would be anonymously fetchable.

**Caveat, stated honestly:** raw serves JSON as `content-type: text/plain`.
Verified against a real file (`fork-prompt-session-rich-summary.json` -> 200,
`text/plain`). Whether Spotify's auth backend accepts a client-metadata document
served as `text/plain` is **not established**, because the only allow-listed
document that could serve as a comparison point, Xirp's, is served as
`application/json` from a normal web host. If the fetcher is strict, raw is the
wrong host and a small static host or Cloudflare Worker returning
`application/json` would be required.

Two of my own earlier 404s during this check were my error, not a property of
raw: I probed `package.json` and `biome.json`, neither of which exists at Jcode
master root. Corrected by listing the directory first.

### Net hosting recommendation
1. **Preferred:** a static host that serves `application/json` at a stable HTTPS
   path — a small Cloudflare Worker is the natural fit given the account already
   runs one for `jcode.sh`.
2. **Fallback:** a file committed to a public repo and served via
   `raw.githubusercontent.com`, accepting the `text/plain` risk and testing it
   against the real authorization endpoint before relying on it.

Either way, the allow-list pattern appended in App Settings must match the URL
actually used, and the redirect port must be re-derived by
`scripts/derive_callback_port.py` for that exact URL. The port is 18586 for
`https://kooshapari.com/oauth/client-metadata.json` and **35683** for
`https://jcode.sh/oauth/client-metadata.json`, so it changes with the choice.

### Bridge state during this work
The Firefox bridge flapped again and stopped responding, and `browser setup
firefox` could not complete — it needs a manual "Add" click in the extension
prompt. This is the same flap already documented in STATE.md section 13. It did
not block this work, because everything above was done with `curl`.

## CIMD hosting: the deployment is now scoped (2026-10-15)

Verified against the Cloudflare API and by probe, so the operator's step is a
single known action rather than a design decision.

### `kooshapari.com` is a Cloudflare zone in your own account

```
/zones -> kooshapari.com (active, 6c9edab581e9c7b8fdb6a83adc6878ea)
         pheno.shop, pheno.studio, phenotype.space
whoami -> user:2a8cf34986db70be316acea959d9b9b9b9, kooshapari@gmail.com
```

That is the decisive fact: the domain named in my recommended pattern is one you
control in Cloudflare, so a Worker route on it is a first-party operation.

### Today `kooshapari.com` is served by Vercel, and the path 404s honestly

| Path | Status | Content-Type |
|---|---|---|
| `/` | 200 | text/html |
| `/index.html` | 200 | text/html |
| `/.well-known/oauth-client-metadata.json` | 404 | text/plain |
| `/oauth/client-metadata.json` | 404 | text/plain |

`server: Vercel`. Note this is the **good** failure mode: Vercel returns a real
`404 text/plain` for unknown paths rather than an HTML fallback with a 200. So
`kooshapari.com` is a trustworthy host for the pattern — the 404 today simply
means nothing is deployed at that path yet.

### No existing Worker can be reused

Enumerated all 12 Workers: `agent-digest`, `dispensary-collector`,
`job-collector`, `job-collector-verification-preview`, `phenotype-apex-landing`,
`phenotype-pages-proxy`, `relay-forwarder`, `shop-redirect`, `studio-redirect`,
`tracera-edge`, `tracera-edge-dev`, `tracera-edge-preview`. **Every one has
`routes: []`**, so none owns a custom domain, and none is a fit — they do email,
KV/D1/R2 collection, redirects, and a tracera API. A dedicated Worker is
required.

Also checked Pages: only `job-cockpit` (dash.kooshapari.com) and
`harnessdesk-preview` exist. `jcode.sh` is in **none** of this account's zones,
which is consistent with it being an external/Vercel-hosted domain and is why it
was not an option.

### Recommended deployment, exactly

A new Worker with a route scoped to **only** the CIMD path, so it cannot affect
any other path on the domain:

- Worker name: `jcode-cimd`
- Route: `kooshapari.com/oauth/client-metadata.json` (exact path, not a wildcard)
- Response: `content-type: application/json`, `cache-control` short, CORS not
  required (server-to-server fetch)
- Body: the committed template with `client_id` set to that same URL,
  `client_name` set to our own identity, and `redirect_uris` containing
  `http://127.0.0.1:18586/oauth/callback`

**Order matters.** Deploy and verify the URL returns real JSON *first*, then
append the allow-list pattern in App Settings. Adding the pattern before the
document is served would allow a client_id that cannot be resolved.

### The exact port, once more
`18586` for `https://kooshapari.com/oauth/client-metadata.json`, derived by
`scripts/derive_callback_port.py` and cross-checked against the shipped
`mcp-remote` JavaScript. Recompute if the URL changes.
