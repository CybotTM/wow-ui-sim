# Independent postcommit retention verification

**OVERALL: PASS**

Commit: `987f6743f`. Retained directory: `data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/four-profile-stock-startup`.
Private capture: `/home/osso/.local/state/wow-ui-sim/verification/four-profile-stock-startup-current/20261011T003325Z`.

Scope: saved artifacts only. No tests, builds, replay, live source inspection, binary rehash, delegation, Bash, rg, fd, or repo edits. Compiler/raw-stream/saved-baseline hashes only; historical provenance proof remains bounded.

## Checks

- **Committed retention and checkout identity: PASS** — 27/27 retained files checked against target commit (26 payloads + manifest)
- **Hash manifest: PASS** — 26/26 payload SHA256 values; complete manifest coverage, manifest excludes itself
- **Byte copies: PASS** — 20/20 exact bytes: 3 top-level + 16 profile receipts + Mists JSON
- **Diagnostic excerpts: PASS** — 4/4 files, 17/17 exact cited private stream lines
- **Authored compiler summary: PASS** — 4/4 recomputed streams: each 653 fresh + 74 build-script records + 1 successful finish; 0 nonfresh; 2612 fresh total
- **Authored retention summary: PASS** — 16/16 stream fingerprints match private streams/evidence; saved baseline 3846 entries and hash match; revision matches 4 saved submissions; no live source/binary hashing
- **Exact compile window sum: PASS** — wrath=0.852145, mists=0.377244, era=0.234412, anniversary=0.208569; sum=1.672370s (cached artifact materialization, not fresh compilation)
- **Captured exits and markers: PASS** — 8/8 receipt exits agree with private admission; Mists completion 0/marker 1; other completions 1/marker 0; all Lua-errors exits 1; 3 blocked profiles have empty output, not zero-error results
- **Mists error signatures: PASS** — 2/2 exact messages, count 1 each, hashes match admission; self-anchor and ScriptErrors wrapper, not proven independent causes
- **Bounded provenance admission: PASS** — Private admission preserved: provenance PASS, completion 1/4, clean pass 0; outcome admission PENDING; historical source/binary proofs not repeated
- **New documentation links only: PASS** — 3 changed docs: 7 newly added link occurrences (6 target files), 2 fragment links resolve to new heading; no old links audited
- **Private identifier scan: PASS** — 9 retained summaries/excerpts; 1 private EditMode/WTF selection record; 5 derived account, compound selector, realm, character and realm/character-path identifiers; 0 identifier findings. Private values withheld. The numeric sub-selector alone is not an identifying value and is not treated as a leak.
- **Raw private streams not retained: PASS** — No raw compiler/runtime streams, source maps, binaries or private detailed evidence retained; Mists JSON is sole permitted structured stdout copy

## Captured runtime results

| Profile | Completion exit | Completion markers | Lua-errors exit |
|---|---:|---:|---:|
| wrath | 1 | 0 | 1 |
| mists | 0 | 1 | 1 |
| era | 1 | 0 | 1 |
| anniversary | 1 | 0 | 1 |

Mists: 2 unique errors / 2 occurrences, exact signatures match private admission. Wrath/Era/Anniversary: blocked before stock Blizzard load. Completion 1/4; no clean-startup acceptance.

## Newly added doc references

- `docs/wiki/index.md`: Stock receipt matrix → `docs/wiki/investigations/stock-api-startup-2026-10-10.md#later-four-profile-normal-captures--2026-10-11-utc`.
- `docs/wiki/log.md`: stock receipt matrix → `docs/wiki/investigations/stock-api-startup-2026-10-10.md#later-four-profile-normal-captures--2026-10-11-utc`.
- `docs/wiki/investigations/stock-api-startup-2026-10-10.md`: independent report → `data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/four-profile-stock-startup/independent-verification-report.md`.
- `docs/wiki/investigations/stock-api-startup-2026-10-10.md`: Exact Mists JSON → `data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/four-profile-stock-startup/mists/lua-errors.json`.
- `docs/wiki/investigations/stock-api-startup-2026-10-10.md`: original hidden-state assertion → `docs/wiki/investigations/mists-cast-startup-visibility.md`.
- `docs/wiki/investigations/stock-api-startup-2026-10-10.md`: Compiler summary → `data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/four-profile-stock-startup/compiler-summary.json`.
- `docs/wiki/investigations/stock-api-startup-2026-10-10.md`: Retained summaries/excerpts → `data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/four-profile-stock-startup/retention-summary.json`.

Privacy: derived private identifier values and raw private lines omitted. Scan covers retained summaries/excerpts, not a general anonymization claim.

Verification is retention integrity, not new startup, native, GUI, full-suite, dependency closure, current-source, or binary provenance proof.

## Verification predicate corrections

Initial execution predicate over-required the sync diagnostic in completion stderr. Installation failure is present in both blocked invocations; the sync diagnostic is present in lua-errors stderr only. All 8 exits and 4 completion-marker counts match admission. Initial privacy predicates included generic timing labels and a bare numeric sub-selector; neither is a private identifier. Final scan uses actual EditMode/WTF account selection, compound selector, realm, character and realm/character path. No tests or hashes rerun during these corrections.
