# Xirp custom harness integration: Jcode
Date: 2026-10-14
Status: discovery and acceptance contract; integration not implemented or verified.

## Intent
Jcode must be a first-class custom harness in Xirp, retaining its native provider
credentials and permission behavior. Merely attaching the Jcode repository to
Portal or running its TUI in a shell does not satisfy this requirement.

## Verified surfaces and unresolved assumptions
- Spotify's Xirp docs list Claude Code, Codex, Gemini CLI, Pi and Cursor.
- Coding Agents settings expose tested agent-specific launch controls. The docs
  inspected do not define a custom harness registration or ACP client interface.
  This is an evidence gap, not proof that the installed app has no extension.
- Xirp supports standalone project terminals and an installed xirp CLI.
- This fork implements an ACP adapter in src/cli/acp.rs, exposed as jcode acp.
  ACP support on Jcode does not imply ACP support on Xirp.
- Jcode README documents stdio MCP servers; remote HTTP/SSE entries are skipped.
- Portal MCP supports CIMD OAuth or scoped static tokens. A stdio-to-remote
  transport/auth bridge or native remote MCP support is needed for this path.
- Transcript upload uses native formats. No Jcode transcript importer was
  identified in the inspected docs. Do not impersonate a supported harness.
- Workspace launch requires an eligible catalog Component; resource attachments
  alone do not satisfy that prerequisite.

## Bounded work
| ID | Work | Acceptance evidence |
| --- | --- | --- |
| XJ-01 | Discover installed Xirp custom-command/adapter contract | CLI help and app version; documented or vendor-confirmed registration schema |
| XJ-02 | Register explicit Jcode identity and launch in a worktree | Launch, interrupt and resume use Jcode; checkout isolation verified |
| XJ-03 | Connect Portal context | Authenticated tool listing and scoped Workspace read; no secrets in repo/config examples |
| XJ-04 | Map working, waiting, idle, failed and completed states | Independent state transitions observed; permission waits remain waits |
| XJ-05 | Preserve transcripts and provenance | Tool calls, results, edits and interrupted/resumed sessions survive round-trip |
| XJ-06 | Capture usage without inventing cost | Token totals reconcile; unknown price remains unknown |
| XJ-07 | Verify failure and rollback | Missing CLI, expired auth, network loss and malformed events fail clearly; unregister restores prior configuration |

First execute XJ-01. Do not write an adapter against guessed Xirp flags, hook
formats or transcript schemas. If Xirp exposes no supported custom harness
contract, obtain that contract from Spotify before implementing XJ-02/04/05.
Jcode-side transport work can proceed separately after choosing an authenticated
MCP design and inspecting the current transport implementation.

## Read-only Mac probe
From this checkout:
```sh
python3 scripts/probe_xirp_jcode.py
```
The probe invokes only each installed CLI's --help, with a 15-second timeout.
It does not launch sessions, inspect credential/config files, install packages,
change settings, or upload output. CLI implementations themselves may have
startup side effects; review before execution. Output stays on stdout. Review
output before sharing. The probe cannot establish runtime compatibility.

The current assistant environment has no native access to the owner's Mac.
The probe has not been executed on the installed Xirp app.

## Constraints
GitHub Actions spend is permanently zero. Use local validation or free included
capacity. Preserve the separate Tracera/AgilePlus pilot and specification gates.
No credential minting, permission expansion or automatic transcript upload is
part of this discovery change.

## Sources inspected
- https://backstage.spotify.com/docs/xirp
- https://backstage.spotify.com/docs/xirp/settings
- https://backstage.spotify.com/docs/xirp/sessions
- https://backstage.spotify.com/docs/xirp/workspaces/launching-sessions
- https://backstage.spotify.com/docs/portal/core-features-and-plugins/mcp/connecting-agents
- This fork: README.md, src/cli/mod.rs, src/cli/args.rs, src/cli/acp.rs
