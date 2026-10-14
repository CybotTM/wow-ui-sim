# Invalid draw-layer existing-artifact RED audit

Verdict: CONFIRMED normal Lua RED; required invalid-name rejection FAILS. Artifact audit complete, not implementation completion.

## Scope and provenance
Only the six specified admission/argv/launcher/result/stdout/stderr files were audited in full. No workloads, builds, hashes, service queries/mutations, canonical edits, delegation, or alpha accepted public payload reads.
Bundle timestamps record October 14, 2026 (UTC); they are artifact metadata, not an assertion of today's date.
result.json records source 71127a8e164c9ebeeee3439251984d08402483dc, retail profile, and preproducer normal Lua scope. This is source711 epoch evidence, not a producer draw-layer edit or proof of a fix. Source tree and binary identity were not independently inspected; reused receipt stat is not a content attestation.

## Six-file audit
- admission.json: allowed=true, retail; memory admission snapshot, worker_max=2684354560, worker_high=2147483648. Admission alone is not test success.
- argv.json: bounded timeout 86s, kill-after 2s; bwrap read-only root/source mount and target mount, isolated network/tmp; existing artifacts/wow-sim; CASC=0, sound disabled, --no-addons, --no-saved-vars, --exec-lua followed by lua-errors. Recorded argv audited, never executed.
- launcher.txt: exit-code result, process status=1/FAILURE, runtime 8.901s, CPU 7.588s, memory peak 1.6G, swap 0B. Not a timeout result.
- result.json: launcher_exit=1; expected_cases=0 and summaries=[] are not Rust test passes. stdout=664 bytes and stderr=15226 bytes match files read.
- stdout: full JSON plus two markers. Five BACKGROUND/BORDER/ARTWORK/OVERLAY/HIGHLIGHT texture creations roundtrip through GetDrawLayer before DRAW_LAYER_VALID_BASELINE_COMPLETE. Invalid CreateTexture pcall returns true; region count changes 5 -> 6. First assert(not ok) raises 'CreateTexture accepted invalid draw layer'. Following unchanged-count assertion and DRAW_LAYER_INVALID_REJECTION_COMPLETE are unreached. Mutation is observed, not a separately executed assertion failure. Invalid texture's actual draw layer is not queried.
- stderr: fully read through final summary; probe error and secondary Blizzard_ScriptErrors processing traceback; Status: FAILED; 2 unique errors, 2 occurrences; attributed owners 0, unattributed occurrences 2. Startup loading proceeds; this is not quiet-clean Lua output.

## Coverage matrix
| Claim | Result | Proof level |
|---|---|---|
| Five valid layer roundtrips | PASS | Baseline marker after five assertions |
| Invalid-name rejection | FAIL / RED | pcall true, first assertion error, exit 1 |
| Region count invariant | Observed violated; assertion unreached | Printed 5 -> 6 |
| Final rejection completion | Unreached | Absent stdout marker; prior throwing assertion |
| New Rust tests | NOT PROVEN | No Rust test runner/results in bundle |
| Native WoW behavior | NOT PROVEN | Simulator-only artifacts |
| Historical regression/transition | NOT PROVEN | Single recorded epoch, no historical comparison |
| All-profile behavior | NOT PROVEN | Retail only |
| Producer draw-layer edit/fix | NOT PROVEN | Preproducer source711 recorded; no producer edit audit |

The full two-entry Lua error JSON is preserved in proof.json. First entry is the probe assertion; second repeats that failure through ScriptErrors GetErrorData at Blizzard_ScriptErrors.lua:59 and :72. Two unique messages do not establish two independent draw-layer defects.
