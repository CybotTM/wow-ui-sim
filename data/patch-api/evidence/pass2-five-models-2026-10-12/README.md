# Five modeled APIs and receiver containment

Recorded October 12, 2026 local / October 13 UTC. Independent retention audit PASS: 132/132 complete copies, 139 public files, 12 relative links, zero privacy/fidelity defects. Bounded runtime reports retain their original source scope.

| Scope | Source | Result |
|---|---|---|
| Historical pre-producer RED | `76cf33ef327bbd9305706d8dd3f375b609e824d4` | 46 tests: 12 PASS / 33 behavioral FAIL / 1 Ping cache-fixture FAIL |
| Five producers plus receivers | `c490d77f15dce96ba757ff2e1f0a24faeaf2ba9e` | 53 requested plus 10 bounded regressions: 63 PASS, sixteen exit-0 invocations |
| Separate stock startup | c490 Rust; docs-only `2691111d5` | One invocation, exit 0, `[]`, zero Lua errors/warnings; original WTF read-only |
| Default Cargo check | `2691111d5ab85328ebcc5dae70eb38303b0ce2d3` | Exit 0; 733 valid JSON records, zero compiler messages/errors, six inherited manifest deprecation warnings |

## Retained evidence

- [RED report](red/verification-report.md), [46 per-test results](red/results.json), complete stdout/stderr for each selected test under `red/streams/`.
- [GREEN report](green/verification-report.md), [requested results](green/green-results.json), [regression results](green/bounded-regression-results.json), complete sixteen-invocation streams under `green/streams/`.
- [Startup result](startup/result.json), [stdout](startup/stdout), [stderr](startup/stderr).
- [Check report](check/verification-report.md), [full stderr](check/check.stderr), [JSON stream counts](check/stream-summary.json). Cargo artifact JSON is summarized, not copied.
- [Retention manifest](retention-manifest.json) identifies 132 complete sanitized text copies. Generated result tables are separate. Originals remain private; executable/library bytes and digests are not published. Text-file hashes authenticate retained text only.

### Retained stderr metadata correction — 2026-10-15

The `startup/stderr` entry now distinguishes the original capture (`source_bytes`, `source_sha256`) from the retained redacted payload (`retained_bytes`, `sha256`). Exactly three account/realm/player substitutions explain the three-byte difference; the private original and retained log are unchanged. The previous retained-byte/hash fields incorrectly described the unredacted source. This is metadata reconciliation, not a rerun or a change to the captured startup outcome. Independent static verification of `ba38287ab` passed: all 132 retained paths, sizes and hashes match; the other 131 entries are unchanged. Report: `/home/osso/.local/state/wow-ui-sim/verification/retention-stderr-correction-next/report.md`. This does not add runtime coverage.

RED's new 37 cases reached 4 PASS / 32 behavioral FAIL / 1 fixture FAIL. CVar RED completed 60 matrix cases, failed one, left 27 unreached—not 88 completed. GREEN's four CVar tests reach all 88 cases. Historical RED is not rewritten by GREEN. VertexColor endpoints stay on VertexColor receivers; Texture getters remain Texture-only. Current method comparison allows exactly two documented Texture getter rows; native captures remain unchanged.

Supplied item stats/set lists, absolute-ms Ping interval, GUID-backed live-party AI membership and existing Retail combat CVar policy are bounded models. Graph/publication success does not establish native parity. No ordinary gameplay data feed, full ping scheduler/lifecycle, GUI/native input, other-profile, performance-causation, provider-stream, credit accounting or whole-handoff acceptance is claimed. Earlier e397 47-test proof remains a separate epoch. Historical full-suite 41cb remains FAIL; new suite submission is not a result.
