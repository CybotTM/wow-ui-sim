# Current Calendar day Green retention gate — PASS

Scope: current Retail/PTR packets published by 47c119847, fcde8eeee, ebe53f801. Independent artifact verification only. 108/108 checks PASS; zero accepted defects. Full per-file evidence in proof.json.

## Coverage matrix

| Exact scope | Evidence | Gate result |
| --- | --- | --- |
| Retail selected TEXT | 37/37 whole ordered literal substitutions then LF; 34 epoch texts + 3 source-gate texts | PASS |
| PTR selected TEXT | 39/39, distinct inventory schema respected | PASS |
| Committed public inventory | Retail 39 files; PTR 41 files, including generated README and manifest | PASS |
| Recursive privacy | Whole text, JSON keys/values and all JSONL records; escaped slash, Unicode/hex, URL and repeated HTML decoding | PASS, zero mapping-identity findings |
| Private copies | Retail 79 + PTR 83 = 162 files; all regular nonsymlink files mode 0600 | PASS |

All 76 original snapshots, staged projections and published selected texts were read in full. Original/projected bytes and text SHA256 receipts checked; Retail replacement counts recomputed. Every selected public blob compared exactly with publication-commit contents. Line/record preservation and ordered opaque-address tokens checked; whole byte equality covers IDs, scalars, keys, nested values and embedded encoded forms without reserializing historical JSON or clipping. Text hashing only.

Owning links resolve through five parent traversals to docs/specs/calendar-day-event-records.md; actual contract content and headings retained, not placeholder links. Public status remains Published selected TEXT; independent retention/privacy gate PENDING. README selection/pending-runtime text is historical and untouched. No retroactive gate selection.

## Retained runtime observations — not executions by this gate

| Profile | Day producer | Numeric-index regressions | Normal startup |
| --- | --- | --- | --- |
| Retail | 9/9, zero failures, exit 0 | 11/11, zero failures, exit 0 | Marker + [] ; exit 0; CLEAN; zero Lua errors |
| PTR retry1 | 9/9, zero failures, exit 0 | 11/11, zero failures, exit 0 | Marker + [] ; exit 0; CLEAN; zero Lua errors |

All six complete runtime stderr texts inspected; no warning lines. Admission allows launch and launcher metadata says success/0. Historical runtime result byte counts are not substituted for projection receipts: Retail normal stderr original 14702, retained 14699; PTR normal stderr original 14694, retained 14709. Whole projection receipts pass despite legitimate privacy replacement size changes.

PTR's three original BLOCKED_ZERO_LAUNCH records remain separate, each directory containing only blocked-before-admission.json. Field-name mismatch happened before admission/argv/launch, not behavior failure. Retained Python adapters explicitly map dev/inode/size/mtime_ns to st_dev/st_ino/st_size/st_mtime_ns; parsed as TEXT, never executed. PTR seal and receipt dictionaries use those different names but values match exactly under that explicit mapping.

## Source/provenance boundary

Retail retains the real historical source-only report, proof and 115-byte cargo-fmt-check.log. fcde8eeee adds that ignored *.log file; exact committed bytes verified. Historical formatting result: exit 0 once, empty command stdout/stderr. Source proof explicitly says runtime UNPROVED and no compile/tests/runtime run. That historical source gate is separate from retained runtime observations; neither was newly selected before finish.

Both compile JSONL streams retained whole; each has 741 records and a successful build-finished record. Six manifest deprecations plus one six-warning aggregate retained. Seals/receipts/auth reports are old copy-integrity metadata only, checked for cross-record consistency, not refreshed or rehashed. PTR source-scope metadata does not authenticate source contents, external dependencies, cache or settings.

## Generated receipts and historical private snapshots

Current public README receipts use actual entire bytes/digests:
- Retail: 1199 bytes, SHA256 5b8298cd53004e72e10cc186ca93c046d3cec023b3345aa55f196ede07b41bbd.
- PTR: 1702 bytes, SHA256 7495f662ee143bc8c70e58ba8706d6ff7ea94a5fb965bc828331125502555858.

Private generated stage README/manifest copies predate publication. Stage README sizes are Retail 942 and PTR 1445, versus current public 1199 and 1702. Stage manifests differ only in status and generated receipts; all 76 selected TEXT inventory entries remain identical. PTR private report files_inventory records these historical generated sizes/digests, not current publication receipts. They were not copied into current checks and were not altered. Publication README diffs retained in proof.json.

## Mutation and limits

Git names/status metadata shows all three publication commits add only current green packet paths; old RED, perf and Spell packets unchanged. No old payload reopened. Tracked worktree clean at observed HEAD. No builds, test/runtime reruns, authentication, binary/library/source/archive hashes, source archive reads, delegation or tracked edits. No native parity, rendering, installed CASC, full Calendar or whole-parent completion claim.

Administrative verifier schema/cwd/state incidents and corrections are recorded in proof.json. No packet payload was changed. Output files are private mode 0600, file/directory fsynced and compared by full readback.
