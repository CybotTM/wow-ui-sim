# Configuration-baseline selection receipts

Implementation revision: `67598f4303d0dc7c4a8ad1abd202531a9c705d20`. Selected builds only; no automatic profile matrix. [Selection contract](../../../../../../docs/specs/method-surface-baselines.md).

| Boundary | Saved result | Proof scope |
|---|---|---|
| Default Retail comparisons, `042623` | Two exact cases PASS, exit 0 each | Current compiled default and root fixture pair |
| Explicit Wrath comparisons, `042623` | Two exact cases PASS, exit 0 each | Bare library, current selected Wrath fixture pair; no Blizzard-source load |
| Wrath one-row negative, `043308` | One expected FAIL, exit 101; exactly `Button:EnableSubtitles` added | Exact comparison detects one deleted complete line; fixture restored |
| Wrath ignored refresh, `043308` | One helper PASS, exit 0 | Source writes only selected paths; all 16 protected input files have unchanged hashes after execution |
| Missing Wrath extra fixture, `043648` | One expected FAIL, exit 101; selected-path read error | No root fallback; all 16 input hashes restored in `finally` |
| Default formatting/check | Both exit 0 | `cargo fmt --all -- --check`; `cargo check --offline --locked --tests`; existing warnings retained privately |

`retention-manifest.json` names 33 byte-exact copied receipts/streams with source paths, sizes, and SHA256. Private originals remain untouched. Compiler stdout/stderr, binaries, and broad source metadata remain private; saved artifact receipts retain feature/build identity and seals. No binary rehash or command replay was required for retention.

The initial negative worker stopped before mutation (`Negative-control row not uniquely present`); that private failed attempt remains preserved. Corrected worker uses exact complete-line matching, not substring matching. `042623/results.json` contains only the completed Retail aggregate because the initial controller later stopped; Wrath evidence comes from its two complete per-case outputs and `execution-results.json`, not an invented aggregate.

Independent audit of positive source/build/case evidence exists; final controls and retained-document audit are pending. Copying receipts is not independent verification.

Older Retail configurations and uncaptured flag combinations lack fixtures and are not claimed passing. PTR/Mists/Era/Anniversary/Forever selection has copied source data, not fresh comparator GREEN. Frozen discovery and default fixtures remain unchanged. Historical `031711` results remain 1 PASS/11 FAIL against the former shared default, and suite469 remains FAIL. Native parity UNKNOWN; parent handoff OPEN.
