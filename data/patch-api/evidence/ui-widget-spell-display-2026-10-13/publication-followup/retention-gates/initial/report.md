# SpellDisplay follow-up retention audit — FAIL (links only)

Revision: `6de1bbbb009f50855858c907f7cedcda1a433cee`; parent: `844a63adb95c7111e1286fa668e632f7cd237806`. Independent verifier followed verify/SKILL.md, artifact mode bounded to saved TEXT. No fixes or tracked changes.

## Coverage and verdict

| Boundary | Result | Evidence |
|---|---|---|
| Copy integrity | PASS | 244/244 frozen originals → all16 ordered substitutions → CRLF/CR normalization → exact complete public-byte equality. |
| Sizes/digests | PASS | Original3524808 bytes; public3301369 bytes. Every selected original/public SHA256 and size matches its applicable private/public metadata; planned167 unchanged, added77 retained. |
| Inventory/commit | PASS | 244uniqueTEXT+2metadata;246 added committed files; scoped committed/working-tree diff empty. Both metadata readback digests match private targeted-readback. |
| Privacy | PASS | No retained private identity/path leaks after full-text and recursive decoded JSON/JSONL key/string traversal. All12 standalone reports included. Both private maps mode0600. |
| Historical red-current | PASS | All33 committed blob/path/mode tree entries identical across parent/current; scoped diff empty. Historical payloads were not opened or rehashed. |
| Changed owning documentation | FAIL | Eight documents inspected;24/26 changed links resolve. All9 anchor occurrences exist and identify meaningful owner/checkpoint sections. Two report links broken below. |

## Findings — do not fix

1. `data/patch-api/evidence/ui-widget-spell-display-2026-10-13/README.md:29` links nonexistent `publication-followup/independent/retail-runtime-current/report.md`. Relevant retained saved report exists at `publication-followup/independent/retail-post-removal-runtime/report.md` (report/proof selections text-source-241/242). Its bounded28/28 plus normal-startup text matches the owner's intended claim; link does not reach it.
2. Same README`:33` links nonexistent `publication-followup/independent/default-check-current/report.md`. Relevant retained saved report exists at `publication-followup/independent/defaultcheck/report.md` (report/proof selections text-source-243/244). Its saved733-record/default-check text matches the intended owner scope; link does not reach it.

Other changed owner/spec/wiki links preserve historical versus current status, separate retention from runtime/source authentication, and retain native/Classic/world-loot/render/full-parent exclusions. Presence and semantic matching of saved reports are not fresh runtime/build/source acceptance.

## Complete recursive privacy read

Traversal used full decoded UTF-8, all JSON/JSONL keys/string values and recursively embedded JSON/JSONL to exhaustion, including escaped, percent-encoded, Unicode/hex, slash/backslash, HTML and case variants; no truncation, token/line/record/depth cap. Counters cover246public files plus changed prose: 144 JSON-file/JSONL records, 3724 JSON records in saved logs/Markdown, 202621 nodes, 138142 string values/keys, 7022 embedded decodes, 138481 privacy variants. Counters use independent units, not a claimed replay of staged counter definitions. Generic `/tmp` sandbox mount and descriptive category labels are not private identity paths.

## Limits and future addition

PTR final independent report is outside this frozen manifest. Its manifest checkpoint explicitly calls for a future additive selection; absence is **not** a missing-evidence failure for this244-file audit. No discovery/wait for new PTR output.

Copy integrity authenticates the frozen selected TEXT projection only. No runtime launch, retest, build, source-correctness/source-to-binary authentication, binary/archive/library access or authentication, native parity, full-parent acceptance, delegation, or fixes. Private originals/mappings and canonical packet untouched.

## Delivery/readback

`proof.json` fully saved and read back byte-equal and JSON-equal: 155873 bytes; SHA256 `bfa65b96ae4047a442d1524f35f297dfbd0c7aa4ed9a8f9ce1c1b546bca4ffe6`. Per-file proof includes all244 original/public sizes/digests and equality results, changed documentation evidence, link/anchor destinations, historical metadata outcome and exact findings. `report.md` fully saved and immediately read back byte-equal. Outputs confined to requested verification directory.
