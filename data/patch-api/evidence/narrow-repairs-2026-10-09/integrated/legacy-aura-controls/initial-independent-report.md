# Independent legacy-aura receipt admission

**Blocked receipt consistency: PASS. Functional admission: RESOURCE_BLOCKED / NOT EXECUTED.** Wrath 0 cases; Mists 0 cases; total 0/16 intended profile-case executions. No test passes, failures, selected-one counts or test exit codes exist for this epoch. Initial “ran16” claim is rejected; caller correction agrees with saved evidence.

Epoch: `20261011T003802Z` (October 11, 2026, 00:38:02 UTC). Read and followed `/home/osso/AgentConfig/skills/verify/SKILL.md` as independent verifier. Saved-receipt inspection only; no execution, build, replay, retry, live resource inspection, delegation, Bash, rg, fd or repository edits. Outputs are this report and `independent-evidence.json` only.

## Admission/controller

| Profile | Available GiB | Load1 | Relevant Cargo | Admitted | Executed |
|---|---:|---:|---|---|---:|
| Wrath | 25.720077514648438 | 26.71 | `[]` | false | 0 |
| Mists | 25.67504119873047 | 26.71 | `[]` | false | 0 |

Worker predicate is available GiB >= 6 AND load1 < 24 AND no relevant Cargo. Both fail precisely load1 < 24; memory and recorded Cargo predicates pass. Resource rejection occurs before artifact metadata reading, binary hashing and test command launch. `results.json` equals `outcome.json.profiles`: two `RESOURCE_BLOCKED`, `cases_executed: 0` records. Outcome's independent admission remains `PENDING`; this separate report does not rewrite main receipts.

Parent `submission.json` records systemd submission exit 0, empty stdout, and stderr `Running as unit: wow-legacy-aura-20261011t003802z.service; invocation ID: 74a6d6e6431c4c0f94dc4d31394c7703`. Submission success is not test success or a saved service terminal exit code. Parent `controller.stdout` has exactly one JSON `type: completed` record, executed code byte-equal to request code, console exactly the epoch path, value null; parent `controller.stderr` is 0 bytes. `request.jsonl.code` equals `worker.py` exactly. No saved terminal service exit receipt is present; controller completion is recorded separately.

Epoch inventory before verifier outputs is exactly eight files: cgroup-before/after, source-before/after, outcome, results, and two profile admission JSONs. Each profile directory contains only admission.json. There are **zero test stdout files, zero test stderr files, zero executions.json files, zero epoch artifact.json files**. Missing test logs are expected on the blocked branch, not successful empty output. Per-command selected count and exit are unavailable, not 1 or 0.

## Intended selection, not executed coverage

Eight unique selectors per profile in worker/request/controller code; sixteen intended commands across two profiles. Intended argv is `/usr/bin/timeout 90 <unit2 sealed profile integration-test> aura_api::<name> --exact --nocapture --test-threads=1`, cwd `/home/osso/Projects/wow/wow-ui-sim`. Exact selection intent does not establish actual test discovery or one-case execution. Worker's unused validation checks only a successful summary `(1 passed, 0 failed)`; no such summary exists here.

| Selector suffix | Source assertion (`tests/aura_api.rs`) | Proof this epoch |
|---|---|---|
| test_unit_buff_returns_data_for_player | lines 14–18: first player buff name nonempty | none |
| test_unit_buff_returns_all_fields | lines 21–35: only name, tenth-slot spellId and fifth-slot duration non-nil | none |
| test_unit_buff_past_end_returns_nil | lines 38–42: player index 100 first return nil | none |
| test_unit_buff_non_player_returns_nil | lines 45–49: target index 1 first return nil in default fixture | none |
| test_unit_buff_count_between_4_and_8 | lines 52–67: contiguous first-return enumeration count 4–8 | none |
| test_unit_aura_helpful_returns_data | lines 74–78: HELPFUL first player name nonempty | none |
| test_unit_aura_harmful_returns_nil | lines 81–87: HARMFUL first player return nil | none |
| test_unit_debuff_returns_nil | lines 94–98: first player debuff return nil | none |

Tests construct the default `WowLuaEnv`; no concrete custom aura/caster fixture is seeded by these eight controls. “all_fields” does not assert all fields: icon, count, dispelName, expirationTime, source/caster, isStealable and nspp are destructured but unchecked. No exact tuple length/value/order contract, caster mapping/order, native client behavior, harmful populated fixture, full profile or whole handoff completion follows, even if these controls later pass.

## Saved cgroup evidence

Before and after membership both exactly `/user.slice/user-1000.slice/user@1000.service/agents.slice/wow-legacy-aura-20261011t003802z.service`, matching submitted agents.slice unit. Both `cpu.max` are `1200000 100000` (12 CPU equivalents quota), both `memory.max` are `17179869184` (16 GiB), matching submission CPUQuota=1200% / MemoryMax=16G. CPU usage 386517→491562 us (+105045); user 213057→267586 (+54529); system 173459→223976 (+50517); periods 5→6; throttled periods/usec both 0 throughout. nice, forced-idle, bursts, burst-usec remain 0. Memory peak 51650560→52240384 bytes (+589824). These are saved controller cgroup endpoint counters, not test costs, compile costs, live availability or continuous monitoring. No OOM/event counters were saved; do not infer OOM-free execution from them.

## Identity/source composition boundary

Reuse accepted `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/independent-report.md` and its epoch `20261010T235452Z`, plus retention report `/home/osso/.local/state/wow-ui-sim/verification/unit2-retention-current/report.md`. Previous independent acceptance established sealed integration artifact identities and the 3846-input source scope (`src`, `tests`, Cargo.toml, Cargo.lock, build.rs, .cargo/config.toml), proof-scope blobs matching cc4f4c782 despite documentation-only successor 24c727075a69a8a4bbf813156248e0547f3150f9. That prior acceptance is reused, not rerun or promoted to new functional evidence.

Read saved Wrath metadata: 435601288 bytes, SHA256 `f458b5f445a816cbf61b4a76aade638c18c3660ae6c3a450968a597da0fb1a9f`, features casc/client-wrath/gui/rodio/sound. Read saved Mists metadata: 438312496 bytes, SHA256 `0cd805af673f8bbafb9d8af38a3eb9454fd7cb036aa325277e2eeefd2b6ed33c`, features casc/client-mists/gui/prefork-full-ui/rodio/sound. Both are test build profiles (debug assertions and overflow checks true, line-tables-only debuginfo, opt-level 1). These identify intended reference artifacts only: blocked worker did not reach new binary pre/post seals, so this epoch cannot establish their contemporaneous byte identity or use.

Parsed full saved source maps: before 3846, after 3846, both exactly equal accepted unit2 baseline-source.json; zero missing/added/changed entries. Main's saved source endpoints compose with prior accepted source identity. No rehash of binaries or 3846 live inputs performed. Only inspected `tests/aura_api.rs` hashed once to bind source assertion analysis: `21b3e7bfd5cb3fbe6798cf3280ce72f00a73382b5dece457d7eb6b3cd96bf48a`, equal both endpoints. Endpoint agreement does not prove continuous immutability, whole-tree revision identity, dependencies, hermeticity or native parity.

**Conclusion:** accept faithfully recorded blocked outcome and source-map composition; do not admit any legacy-aura functional pass. Zero executed controls, zero test exits, no retry performed.
