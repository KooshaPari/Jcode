# Appendices A-D: OAuth discovery, CIMD proof, PR 619 validation, bridge choice

Supporting material for `PORTAL-FINDINGS.md`. Read that file first for the
authenticated Portal observations.

# Appendices: OAuth and discovery detail

Supporting material for `APPENDICES.md`. Holds the RFC 8414 / RFC 9728 discovery
records and the CIMD allow-list proof.

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
