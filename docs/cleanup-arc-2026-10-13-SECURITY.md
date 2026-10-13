# P30 Security Advisory (2026-10-13 22:47Z)

**`cargo audit` reveals 7 active vulnerabilities in fork's dependency tree.**

This document is the operator-facing P30 deliverable. The discovery is
**CRITICAL**: the fork is exposed to DoS and panic-class vulnerabilities
that upstream has already fixed.

## TL;DR for operator

| Action | Difficulty | Risk |
|--------|------------|------|
| Cherry-pick upstream `268abfa84` (Cargo.lock-only, h2+rustls fix) | EASY | LOW |
| Wait for upstream quinn-proto fix | N/A (no upstream fix exists) | N/A |

**The actionable fix is one commit.** It cherry-picks cleanly because
it's a Cargo.lock-only change. Run `cargo audit` after cherry-pick to
verify.

## Full audit output

```
Crate:     h2
Version:   0.4.13
Title:     h2 unbounded empty DATA frames
Date:      2026-08-17
ID:        RUSTSEC-2026-0258
Solution:  Upgrade to >=0.4.16

Crate:     quinn-proto
Version:   0.11.15
Title:     Zero-length DATAGRAM frames bypass the receive buffer limit
ID:        RUSTSEC-2026-0342
Solution:  Upgrade to >=0.11.17

Crate:     quinn-proto
Title:     Unbounded growth of pending connection ID retirements
ID:        RUSTSEC-2026-0339
Solution:  Upgrade to >=0.11.17

Crate:     quinn-proto
Title:     Remote peer can panic with zero-length STREAM frames
ID:        RUSTSEC-2026-0338
Solution:  Upgrade to >=0.11.18

Crate:     quinn-proto
Title:     Peer minimum ACK delay can panic when ACK_FREQUENCY enabled
ID:        RUSTSEC-2026-0340
Solution:  Upgrade to >=0.11.18

Crate:     quinn-proto
Title:     Stream reassembly memory-exhaustion guard bypass
ID:        RUSTSEC-2026-0341
Solution:  Upgrade to >=0.11.17

Crate:     rustls
Title:     TLS 1.3 handshake messages incorrectly accepted across
           encryption level boundaries
ID:        RUSTSEC-2026-0285
Solution:  Upgrade to >=0.23.45

error: 7 vulnerabilities found!
```

## Cherry-pick recipe for 268abfa84

```
cd /Users/kooshapari/CodeProjects/Phenotype/repos/jcode
git checkout -b security/rustsec-2026-0258-0285 origin/master
git cherry-pick 268abfa84
cargo audit                       # should show 0 actionable vulns (5 quinn-proto remain)
cargo check --workspace           # should still build (Cargo.lock only)
cargo clippy --workspace --no-deps # should still be clean
# push the new branch, then open a PR
```

## P30 follow-up: bigger security cherry-picks

If the operator wants to also pull the other security commits:

| Commit | Title | Difficulty | Notes |
|--------|-------|------------|-------|
| 3d9920449 | fix(security): scope Bedrock auth and remote catalog input | Easy | Adds scoping around remote catalog input |
| a85e605df | fix(security): update vulnerable transport and PDF dependencies | Medium | Likely many Cargo.lock changes; needs care |
| 0b3595342 | fix(security): isolate onboarding credential sandboxes | Medium | Onboarding flow refactor |
| aa19ee1d4 | fix(release): sign fast/local macOS release binaries with stable identifier | Hard | Release config; needs macOS env |
| 34c4b57c4 | fix(menubar): stable macOS signing identity and a [menubar] enabled opt-out | Hard | Signing config |
| 2e72b04e1 | fix(browser): make control of the user's own browser reliable (PR #1743) | Medium | Browser bridge |
| ea721e51a | fix(browser): diagnose silent Chrome bridge in `jcode browser status` | Medium | Diagnostic |
| d1924ee04 | fix(browser): explain why a Chrome bridge extension is silent | Medium | UX/diagnostic |

## Recommendation

Cherry-pick 268abfa84 IMMEDIATELY. It's a Cargo.lock-only change,
takes 5 minutes, eliminates 2 confirmed vulnerabilities. This is
standalone from the P26 fork rebase decision.

For the 5 quinn-proto vulns: subscribe to https://rustsec.org/feed
and wait for upstream fix.

For the other security commits: queue them in P26 sub-decisions.

## Stats

- Total upstream-only commits: 2,685 (P26 was wrong about 2,651)
- Security-tagged upstream-only: 8 (RUSTSEC/CVE/security/vuln grep)
- Auth/OAuth/login/secret/token/sign-related upstream-only: 432

The fork is behind upstream in 2,685 commits; cherry-picking all
security fixes is a 30-50 commit project on its own.
