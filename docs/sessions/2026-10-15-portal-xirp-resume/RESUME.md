# Spotify Portal + Xirp setup — resume record

Last updated: 2026-10-14 (session 2026-10-15T00:04Z–00:2xZ local, America/Los_Angeles)
Owner: KooshaPari · Host: this Mac (darwin/arm64)
Purpose: single source of truth so a future local session can resume without
reading chat history. Update after every meaningful change.

## How to read this file

Sections separate **verified outcomes** (something observed, with the command
that observed it) from **claims** (believed, not proven) and **blockers**.
Anything in "Claims" must not be reported as working setup.

---

## 1. Verified outcomes (evidence attached)

### 1.1 Installed versions
| Component | Observed value | How |
|---|---|---|
| Xirp CLI | `xirp 0.49.0+app.db5402dc7` | `xirp --version` |
| Xirp app | `/Applications/Xirp.app`, asar 43,952,738 bytes | `ls -la` |
| Jcode CLI | `jcode v0.97.0 (94392e100)` | `jcode --version` |
| KCode fork | `kcode v0.0.0-dev (bb6174b21)` | `kcode --version` |
| Orchestrator | `@chirp/squab` **0.11.11** (bundled inside Xirp) | asar `package.json` |

Jcode launcher chain: `~/.local/bin/jcode` -> symlink ->
`~/.jcode/builds/current/jcode`.

### 1.2 XJ-01 ANSWERED NEGATIVELY — no custom-harness contract exists

This is the single most important finding of this session. Xirp cannot register a
custom harness. Five independent lines of evidence, all read-only:

1. `squab --available-harnesses` -> schema `squab.available-harnesses/v3`,
   `"count": 5` -> claude, codex, gemini, pi, cursor. Jcode absent.
2. `xirp api send agents:list` -> identical 5 entries, all
   `"visibility": "public"`, flags `--launch-{claude,codex,gemini,pi,cursor}`.
3. `xirp api list` -> daemon message catalog contains only `agents:list` and
   `harness:install|update|uninstall` (+ `:cancel`). **No create/register/add.**
4. Reserved argv flags: `--launch-*` and `--move-session-to-*` are owned by the
   orchestrator and rejected as user-supplied target flags.
5. Binary scan of the 44 MB bundle for `registerHarness`, `register-harness`,
   `customHarness`, `custom-harness`, `addHarness`, `agentRegistry`,
   `harnessRegistry` -> **zero matches**. Unknown agent yields the literal error
   `no session adapter registered for agent "<name>"`.

Consequence: XJ-02, XJ-04, XJ-05 are unimplementable against this build. They
require vendor work inside Spotify's `@chirp/squab`. XJ-06 is partly implementable
(Jcode can report tokens) but has no consumer without a squab adapter. XJ-03 and
XJ-07 can proceed on the Jcode side.

**Not done, deliberately:** Jcode was NOT renamed to a supported agent name, no
transcript format was forged, no `jcode`->`claude` binary shim was created. All
three would make Xirp display false provenance.

### 1.3 Jcode ACP adapter is real and working
- `src/cli/acp.rs` = 2,195 lines, `ACP_PROTOCOL_VERSION = 1`.
- Wired as `jcode acp` (subcommand `Acp` in `src/cli/args.rs`; `pub mod acp;`
  in `src/cli/mod.rs`).
- Live handshake returned a valid ACP `initialize` result:
  ```json
  {"id":1,"jsonrpc":"2.0","result":{"agentCapabilities":{"loadSession":true,
   "mcpCapabilities":{"http":false,"sse":false},
   "promptCapabilities":{"audio":false,"embeddedContext":true,"image":true},
   "sessionCapabilities":{"close":{},"resume":{}}},
   "agentInfo":{"name":"jcode","title":"Jcode","version":"0.97.0"},
   "authMethods":[],"protocolVersion":1}}
  ```
`agentInfo.name` = `jcode` is the correct honest identity.

### 1.4 MCP transport: native remote MCP does NOT exist (README confirmed accurate)
Verified in source, not only docs:
- `crates/jcode-base/src/mcp/protocol.rs:208` — `pub url: Option<String>`
  annotated "Unused by jcode today"; `headers` likewise.
- Same file, merge path: `merged.servers.retain(...)` keeps only `cfg.is_stdio()`
  and logs `MCP: Skipping non-stdio server '<name>' (<transport>); HTTP/SSE
  transports are not yet supported`.
- `jcode acp` advertises `mcpCapabilities {"http": false, "sse": false}`.
- Live: `jcode mcp list` shows `agileplus [http; enabled] no OAuth credentials
  stored` — that entry is `{"type":"http","url":"http://127.0.0.1:8765/mcp"}` in
  `~/.claude.json`, and Jcode is skipping it.

Route decision (per "native remote vs maintained bridge" requirement): native is
ruled out by source. The only supported path is a maintained
stdio-to-remote bridge in front of
`https://phenotype.spotifyportal.com/api/mcp-actions/v1`, carrying CIMD OAuth or
a scoped static token. **Bridge not yet selected or pinned.**

### 1.5 Repo inventory REFRESHED — the 42 figure is wrong
Read-only `gh api` pagination over owner repos, filtered `name !~ /^[Zz][Zz]/i`:

| Metric | Observed (2026-10-14) | Prior checkpoint |
|---|---|---|
| Total non-zz | **46** | 42 |
| Public | **29** | 30 |
| Private | **17** | 12 |
| Archived | **14** | 9 |
| Disabled | 0 | — |
| Forks | 13 | — |
| Active (non-archived, non-disabled) | **32** | — |

Raw owned repos before filter: 70. Dropped zz*: 24.
Owner type = **User** (verified `/users/KooshaPari` -> `type: User`). Do not invent
an organization.
Artifacts: `/tmp/xirp-probe-20261015/repo-inventory.json` (22 KB) and
`repo-inventory-summary.txt` (8.9 KB). **These are in /tmp and will not survive a
reboot — re-run or relocate if they matter.**

Jcode immutable repo ID = 1365855438, public, not archived, not disabled.
Confirms the checkpoint value.

Note: the +4 delta vs 42 is NOT explained by this run (read-only; nothing changed).
Most likely the prior figure predated several repos, or applied a fork/archive
exclusion this run did not. Do not assert which without checking Portal's stored
list. Archived repos are reference-only per the Wiki Schema rev 2 requirement.

### 1.6 ROOT CAUSE of the 42-vs-46 discrepancy — SOLVED

The gap is **not** a counting-rule difference. It is three distinct things.

**(a) The static catalog is a PUBLIC-ONLY subset, by design.**
`catalog/spotify-portal/catalog-info.yaml` on `chore/portal-non-zz-catalog-20261009`
lists exactly **31** repository descriptors plus `owner.yaml`. Its own description
says "Public non-zz repository snapshot ... Private records belong only in
Portal." So 31 is a *public* count, and it will never equal the 46 total.

**Correction to an earlier error in this same record:** I first reported
`kcode.yaml` as stale because I compared *filenames* to repo names. That was
wrong. `kcode.yaml` correctly declares
`github.com/project-slug: KooshaPari/Jcode`, repo id `1365855438`, and carries
`phenotype.dev/former-name: "KCode"`. It is accurate. Likewise `github.yaml`
correctly maps to `KooshaPari/.github` (id 1199812615), which my zz-filtered
inventory omitted only because the name begins with a dot. Do not "fix" either.

**(c) Eighteen current repos have no descriptor at all.** All are private or
archived, which is consistent with the public-only design, except `.github`
(see above, it IS described).
- Private, active (6): `HarnessDesk`, `job-cockpit`, `PhenoApps`, `PhenoBot`,
  `PhenoLab`, `shopping-home`
- Private, archived (10): `.audit-run-v37`, `PhenoAI`,
  `phenoDesign-visual-production-v2`, `PhenoMLX-temp`,
  `phenotype-canonicalization`, `phenotype-cvp-audit-2026-09-14`, `Pine`,
  `ResearchLedger`, `SessionLedger`, `Substrate`

**Timing evidence that no new repositories explain the delta.** Newest non-zz
repo `created_at` is `herdr` at 2026-10-09T06:32:17Z, i.e. on or before the Oct 9
snapshot date. So the +4 cannot be new repositories. Four repos (`Pine`,
`ResearchLedger`, `SessionLedger`, `Substrate`) were pushed on 2026-10-13/14 and
are now **archived**, consistent with an archive pass that ran after the
snapshot. Archived count went 9 -> 14.

Rule of thumb: **46 = total owned non-zz; 31 = public static catalog; 42 = Portal
GitHub App selected count, now suspect.**

### 1.6b PR 619 local validation — 29 of 31 clean, 2 real defects

Ran entirely locally against the PR head `8be56b97` (extracted from a tarball, no
CI, $0). No files in the registry repo were modified.

**Passing:**
- All 31 descriptors parse as YAML without error.
- Referential integrity is perfect: all 32 `catalog-info.yaml` targets resolve
  (31 repositories + `owner.yaml`), and there are zero unreferenced descriptors.
- Secret scan across the whole `catalog/spotify-portal/` tree: clean.
- 29 of 31 descriptors match live GitHub on immutable repo id **and** archived
  flag. Archived flags for `herdr-forgecode`, `herdr-helioslite`, `herdr-jcode`,
  `PhenoInfra` are all correctly `true`.

**Two genuine defects to fix in PR 619:**

1. `parpoura-experiment-0.yaml` describes a repository that **no longer exists**.
   `gh api repos/KooshaPari/parpoura-experiment-0` returns
   `{"message":"Not Found",...,"status":"404"}`. The repo appears to have been
   renamed to `zz-pause-inc-Parpoura`, which the non-zz rule excludes. So this
   descriptor now points at a 404 and also violates the non-zz scope rule.
2. `phenoregistry.yaml` is **private** on GitHub (verified live:
   `{"id":1201113097,"name":"PhenoRegistry","private":true}`) yet sits in the
   public catalog, whose own description says private records belong only in
   Portal. The checkpoint notes PhenoRegistry became private *after* this
   snapshot was taken, which explains the drift.

Both are evidence-backed and low-risk to correct, but I am **not** applying them
yet: changing the descriptor set changes what Portal ingests, and the original
destructive-preview root cause is still unknown (B2 blocks observing ingestion).
Fix, then verify ingestion rather than guess.

### 1.7 Session detection/sync of externally-launched sessions: NOT supported for Jcode

Direct answer to "does Xirp pick up Jcode sessions I started myself?" — **no.**

| Command | Result |
|---|---|
| `xirp session list` | `No sessions found.` |
| `xirp session import jcode --preview --json` | `No transcript import adapter is available for jcode` |
| `xirp session import claude --preview --json` | `No claude session found` |
| `xirp session import --harness jcode ...` | `--harness does not apply to import; ...` |
| `xirp session attach <id> --harness jcode` | requires an interactive terminal |

The wording difference is the proof: `claude` fails *after* adapter resolution
("no session found"), `jcode` fails *at* adapter resolution ("no import adapter
is available"). Two live `jcode` processes were running at test time, so this is
not an absence-of-sessions artifact.

Mechanism: squab adapters hardcode `sessionRoot(cwd)` per agent
(`~/.claude/projects/<slug>`, `~/.codex/sessions`, `~/.gemini/projects`, pi
`sessions`). No generic transcript scanner, no configurable path. Therefore a
hand-started Jcode session cannot appear in `xirp session list`, cannot be
imported, cannot be attached, and cannot be uploaded to Portal via transcript.

This is a wiring gap, not a data gap: Jcode does persist transcripts at
`~/.jcode/sessions/session_<name>_<ms>_<hash>.json` (7,661 entries) plus an
`edit-stats/` tree. Format is JSON, not the JSONL the Claude/Pi adapters read, so
a future adapter needs a real parser.

### 1.8 Xirp daemon quirk worth knowing
`xirp features` fails with
`Could not connect to the daemon at ws://localhost:3849: ECONNREFUSED`
while `xirp api ...` succeeds against the same daemon. Use
`xirp api send agents:list` for health checks, not `xirp features`.

---


### 1.9 Portal OAuth discovery — verified, and one near-miss avoided

Probed directly, unauthenticated, with `curl`. All read-only.

**Real, verified — the authorization server metadata** at
`https://phenotype.spotifyportal.com/.well-known/oauth-authorization-server`
(genuine `application/json`):

| Field | Value |
|---|---|
| `issuer` | `https://phenotype.spotifyportal.com/api/auth` |
| `authorization_endpoint` | `https://phenotype.spotifyportal.com/api/auth/v1/authorize` |
| `token_endpoint` | `https://phenotype.spotifyportal.com/api/auth/v1/token` |
| `userinfo_endpoint` | `https://phenotype.spotifyportal.com/api/auth/v1/userinfo` |
| `jwks_uri` | `https://phenotype.spotifyportal.com/api/auth/.well-known/jwks.json` |
| `scopes_supported` | `openid`, `offline_access` |
| `grant_types_supported` | `authorization_code`, `refresh_token` |
| `code_challenge_methods_supported` | `S256`, `plain` |
| `token_endpoint_auth_methods_supported` | `client_secret_basic`, `client_secret_post`, `none` |
| `registration_endpoint` | **absent** |

**Near-miss, corrected:** the path
`https://phenotype.spotifyportal.com/.well-known/oauth-client-metadata.json`
returns **HTTP 200**, which looks like a working CIMD document. It is not. It is
the SPA fallback: `content-type: text/html`, 13,761 bytes. A control request to
`/.well-known/definitely-not-real-xyz.json` returns the identical 200 /
`text/html` / 13,761 bytes. So **any** `200` on this host proves nothing; the
content type must be checked. Do not record Portal as publishing a CIMD document.

**What this means for the bridge decision (XJ-03):**
- PKCE is supported, including `S256`. Good: no client secret is strictly
  required (`token_endpoint_auth_methods_supported` includes `none`).
- There is **no `registration_endpoint`**, so OAuth Dynamic Client Registration
  is not offered. A bridge cannot self-register a client id at runtime.
- CIMD requires the client to publish a *reachable HTTPS* client-metadata
  document and for the authorization server to allow-list its URL as a client
  pattern. Portal's allowed patterns are currently `https://claude.ai/*`,
  `https://chatgpt.com/oauth/codex/*/client.json`,
  `https://backstage.spotify.com/chirp.json`,
  `https://xirp.spotify.com/oauth/client-metadata.json`. None of those is ours,
  and widening the list is explicitly out of bounds without authorization.
- Verified reference shape, from the Xirp document that IS allow-listed
  (`https://xirp.spotify.com/oauth/client-metadata.json`, HTTP 200 JSON):
  ```json
  {"client_id":"https://xirp.spotify.com/oauth/client-metadata.json",
   "client_name":"Xirp",
   "redirect_uris":["http://127.0.0.1/callback"],
   "response_types":["code"],
   "grant_types":["authorization_code","refresh_token"],
   "token_endpoint_auth_method":"none",
   "scope":"openid offline_access"}
  ```
  Note `token_endpoint_auth_method: "none"` and a loopback redirect. That is the
  shape any Jcode bridge would need to match, once a pattern is allowed.

Net: the auth server is capable of PKCE + public client. The blocker is
**client allow-listing**, not protocol capability. That requires either a
pre-registered static client id, or operator authorization to add a CIMD URL to
the allowed patterns. Neither can be minted from this session.


## 2. Exact revisions at time of writing

| Artifact | Revision | State |
|---|---|---|
| `KooshaPari/Jcode` PR #67 | head `e4583f6a9679078625de726fd3512c93bc27c5a2` | OPEN, DRAFT, base `master` |
| PR #67 branch | `chore/xirp-custom-harness-discovery-20261014` | same SHA on origin |
| Local jcode checkout | `~/CodeProjects/Phenotype/repos/jcode`, `master` @ `cd974614d2c8e7e36357724a73be61dbb62e16e1` | has unrelated untracked work, left untouched |
| Isolated PR-67 worktree | `/tmp/jcode-pr67-wt` on branch `chore/xirp-custom-harness-discovery-20261014` | **/tmp — recreate after reboot** |
| `KooshaPari/PhenoRegistry` PR #619 | head `8be56b971813265f468e88aa90a616992abe41c9` | OPEN, DRAFT, MERGEABLE |
| Local registry checkout | `~/CodeProjects/Phenotype/repos/phenotype-registry`, branch `polish/audit-sibling-links-20261014` @ `c480ba31f0f612e12154c34f19ac4ba47540c1aa` | **different branch from PR 619** |

Gotcha that cost time: inside the jcode checkout, plain `gh pr view 67`
resolves against **upstream `1jehuang/jcode`**, not the fork. Always pass
`-R KooshaPari/Jcode`. Symptom is
`Could not resolve to a PullRequest with the number of 67`.

---

## 3. Completed work this session

1. Established Xirp/Jcode/KCode versions and binary paths.
2. Launched Xirp.app (it was installed but not running).
3. Captured full Xirp CLI surface (`xirp skill`, all subcommand help).
4. **Answered XJ-01 with reproducible evidence** (5 independent proofs).
5. Ran `scripts/probe_xirp_jcode.py` on this Mac for the first time — both CLIs
   exit 0, no timeouts. Recorded that the probe alone was insufficient.
6. Verified Jcode's ACP adapter end-to-end with a real JSON-RPC handshake.
7. Resolved the README-vs-binary MCP contradiction in favor of the README,
   with the exact skip-logic source line and a live skipped server as proof.
8. Refreshed the repo inventory; established 46 (not 42) and owner=User.
9. Confirmed both PRs (#67, #619) still exist at their checkpoint SHAs.
10. Rewrote `docs/XIRP_CUSTOM_HARNESS.md` in PR 67 with the evidence, corrected
    the stale "probe not executed / no Mac access" claim, and drafted the vendor
    question.
11. Created this resume record.

## 4. Remaining tasks

| # | Task | Blocked by |
|---|---|---|
| A | Commit + push PR 67 doc updates | **DONE** (`ff5e0a47f`, `8d1002352`) |
| B | Verify Portal catalog force-sync outcome (was "Syncing" on Oct 14) | B2 browser bridge |
| C | Diagnose private-source retrieval failure with actual errors | B2 browser bridge |
| D | Rename `kcode.yaml` -> `jcode.yaml` and reconcile descriptors (1.6) | B2 — defer so ingestion is observed, not guessed |
| E | Decide bridge for Portal MCP (XJ-03) | B6 client allow-listing |
| F | Authorize + send vendor question | **operator authorization** |
| G | Complete local PR 619 checks; keep description accurate | local validation, $0 CI |
| H | Verify authenticated Portal tool discovery + bounded workspace read | B2 + B6 |
| I | Review 5 pending wiki suggestions against exact current sources | B2 browser bridge |
| J | Full launch -> context -> result -> transcript flow | blocked on B and F |
| K | Apply corrected PR 67 description | inbox approval (deferred by hook) |

## 5. Blockers (explicit)

- **B1 — Vendor lock on XJ-02/04/05.** Not a skill gap. Spotify must change
  `@chirp/squab`. No amount of Jcode-side work unblocks these. Exact missing
  interface: an `AgentSessionAdapter` implementation + launchable spec for
  `jcode`, OR a generic ACP-backed harness entry point in squab.
- **B2 (NEW, blocks Portal inspection) — browser bridge not connected.** Portal
  catalog/source/wiki work needs a live signed-in browser session. The bridge
  fails in **both** available browsers:
  - Safari: `xcodebuild failed` during `safari-web-extension-converter`; manual
    enable required in Safari > Settings > Extensions.
  - Chrome: extension not installed in the Default profile. Manual steps:
    `chrome://extensions` -> Developer mode -> Load unpacked ->
    `/Users/kooshapari/.jcode/browser/extensions/chromium`
    (expected extension id `ijifgeepmnbalajhfjnpbnobfobflfkk`), then re-run
    `jcode browser setup chrome`.
  This needs ~2 minutes of human action in a browser. Until then, tasks B, C, H
  and I cannot start. Firefox and Edge are installed but untested as bridges.
- **B3 — Portal MCP is reachable but unauthenticated.** Confirmed by direct probe:
  `GET https://phenotype.spotifyportal.com/api/mcp-actions/v1` -> `200` on the
  site root, and the MCP endpoint correctly returns
  `{"error":{"name":"AuthenticationError","message":"Missing credentials"},...,"statusCode":401}`.
  No Portal MCP server is configured in `~/.jcode/mcp.json` (1 server total, none
  Portal-related) and `spotifyportal` appears 0 times in `~/.claude.json`. So
  authenticated tool discovery still needs a real credential, which must come
  from the Portal UI (CIMD OAuth or a scoped static token), not from chat.
- **B4 — Transient files in /tmp.** Inventory JSON, probe output, extracted asar,
  and the PR-67 worktree all live in /tmp and are lost on reboot. Relocate any of
  these that must survive.
- **B5 — Unauthorized actions deliberately NOT taken.** No support message sent.
  No allowed-client-pattern widening. No credential minting/replacement. No
  destructive catalog preview applied. No transcript uploaded. No PR merged.
- **B6 (NEW) — Portal OAuth client allow-listing.** The authorization server
  supports PKCE (`S256`) and public clients (`token_endpoint_auth_method:
  none`), so the protocol is capable. The obstacle is identity: there is **no**
  `registration_endpoint`, so Dynamic Client Registration is unavailable, and CIMD
  requires a reachable HTTPS client-metadata document whose URL matches an
  allow-listed pattern. The current patterns are `https://claude.ai/*`,
  `https://chatgpt.com/oauth/codex/*/client.json`,
  `https://backstage.spotify.com/chirp.json`,
  `https://xirp.spotify.com/oauth/client-metadata.json`. None is ours.
  Unblocking needs either a pre-registered static client id or operator
  authorization to widen the pattern list. Both are outside current authority.
  Details and the verified reference document shape are in 1.9.
- **B7 — `gh pr edit` requires inbox approval.** The corrected PR 67 description
  is staged at `/tmp/pr67-body-2.md`; the command was deferred by the pre_tool
  hook and has not been applied. The live PR body still contains two now-false
  claims (probe reported both CLIs absent; app was not probed).

### 5.1 CI cost check — $0 confirmed for this session's push
`git push` of commit `ff5e0a47f` (docs only) triggered **no** workflow runs:
`gh run list --branch chore/xirp-custom-harness-discovery-20261014` returns
`no workflow runs recorded for this branch`. The two runs stuck in `queued`
(`Require Linked Issue`, `CI`, ids 38644605078 / 38644605215) belong to branch
`fix/ci-unused-deps` at sha `c76abc890`, created 2026-10-14T22:00:58Z, i.e.
before this session's push at ~00:21Z. They are not attributable to this work and
are not a remediation target here (paid Actions remains permanently excluded).

## 6. Constraints that remain in force

- GitHub Actions spend stays at **$0**. Use local validation or free capacity.
  Paid Actions is permanently excluded as remediation.
- Preserve the Tracera + AgilePlus pilot and its specification/design gates.
  AgilePlus #1100 merged at `ca999fd05741f9db19945e7f54057043b77b078b`.
  Tracera #1101 last head `f6392ca2350d3f68a7b49b305bef8cdbb8efba4e`.
- Do not merge merely because a PR is mergeable. #619 is MERGEABLE and still
  draft, with known-incomplete validation. Leave it.
- Never request passwords or OTPs in chat; use secure input.
- Portal is an index/context projection. Repository intent, accepted specs, ADRs,
  tests and evidence keep their own authority. No universal PhenoRegistry
  override. Tracera/AgilePlus retain product-management authority.
- Workspace launch needs an eligible catalog **Component** resolving to a
  repository. Attaching repository Resources alone is insufficient.

## 7. Open questions

1. ~~Why did the prior count say 42 when GitHub reports 46?~~ **SOLVED — see 1.6.**
   Three different numbers were being compared: 46 total owned non-zz, 31 public
   static catalog, 42 Portal App selected. Plus 3 stale descriptor entries
   (`kcode.yaml`, `github.yaml`, `parpoura-experiment-0.yaml`).
2. Is the Oct 14 catalog force-sync still running, and did it complete? Root
   cause of the original preview (3 Locations only, predicted deletion of both
   pilot Projects) is still unknown. Do not apply a destructive preview.
   **Blocked on B2 (browser bridge).**
3. Are the 4 additional repos private and therefore invisible to Portal, or
   genuinely absent from the GitHub App installation? Partially answered: 17 of
   46 are private, and the GitHub App was recorded as having exactly 42 selected.
   That 42 is now suspect and must be compared against Portal's stored list.
   **Blocked on B2.**
4. Does squab have any build-time (not runtime) harness extension, e.g. an env var
   or a generated catalog file read at startup? Not yet exhaustively ruled out.
   Lower priority: the runtime path is definitively closed, and a build-time hook
   would not be user-usable anyway.
5. **NEW — should `kcode.yaml` be renamed to `jcode.yaml` in PR 619?** The
   descriptor references a repo name that no longer exists. This is a safe,
   evidence-backed fix but touches the catalog that Portal ingests, so it is
   deferred until B2 lets us observe ingestion rather than guess at it.

## 8. Evidence file map (all under /tmp, ephemeral)

| Path | Contents |
|---|---|
| `/tmp/xirp-probe-20261015/agents-final.txt` | `agents:list` raw output |
| `/tmp/xirp-probe-20261015/skill.md` | Xirp full command reference |
| `/tmp/xirp-probe-20261015/session-help.txt` | `xirp session --help` |
| `/tmp/xirp-probe-20261015/features.txt` | `xirp features` + the ECONNREFUSED error |
| `/tmp/xirp-probe-20261015/probe-output.json` | PR-67 probe output (first run on Mac) |
| `/tmp/xirp-probe-20261015/acp-stderr.txt` | `jcode acp` stderr (empty = clean) |
| `/tmp/xirp-probe-20261015/repo-inventory.json` | 46 non-zz repos with immutable IDs |
| `/tmp/xirp-probe-20261015/repo-inventory-summary.txt` | human-readable inventory |
| `/tmp/xirp-asar-extract/` | extracted Xirp app.asar (461 .js files) |
| `/tmp/jcode-pr67-wt/` | isolated worktree for PR 67 |