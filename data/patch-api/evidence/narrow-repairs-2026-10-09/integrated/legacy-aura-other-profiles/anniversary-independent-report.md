# Anniversary independent saved-partition admission

**PASS — 8 exact selectors, 8 selected, 8 passed, 0 failed; all 8 saved exits 0.**

Scope: `/home/osso/.local/state/wow-ui-sim/verification/legacy-aura-other-profiles-current/20261011T015231Z`. Read/followed verify skill as assigned verifier; no delegation or execution.

## Exact coverage

| Index | Selector (`aura_api::`) | Literal assertion scope | Selected/pass/fail | Exit |
|---:|---|---|---|---:|
| 0 | `test_unit_buff_returns_data_for_player` | Nonempty player buff name | 1/1/0 | 0 |
| 1 | `test_unit_buff_returns_all_fields` | Name, spellId and duration nonnil only (not all tuple fields) | 1/1/0 | 0 |
| 2 | `test_unit_buff_past_end_returns_nil` | Index 100 nil | 1/1/0 | 0 |
| 3 | `test_unit_buff_non_player_returns_nil` | Target index 1 nil | 1/1/0 | 0 |
| 4 | `test_unit_buff_count_between_4_and_8` | Player contiguous count between 4 and 8 | 1/1/0 | 0 |
| 5 | `test_unit_aura_helpful_returns_data` | HELPFUL player name nonempty | 1/1/0 | 0 |
| 6 | `test_unit_aura_harmful_returns_nil` | HARMFUL player index 1 nil | 1/1/0 | 0 |
| 7 | `test_unit_debuff_returns_nil` | Player debuff index 1 nil | 1/1/0 | 0 |

Selectors match the first eight `#[test]` functions in `tests/aura_api.rs` (lines 13–102). Each argv uses the sealed Anniversary integration binary, `timeout 90`, `--exact --nocapture --test-threads=1`. Each complete stdout contains one matching named `... ok`, `running 1 test`, and one result: 1 passed, 0 failed, 0 ignored, 0 measured, 8022 filtered out. All 16 complete stdout/stderr streams were read; verbatim streams and timestamps are in `anniversary-evidence.json`. Stderr contains environment startup timing lines, ending `WowLuaEnv::new complete`, without test failures.

## Saved composition

`anniversary/executions.json` agrees exactly with Anniversary in `results.json` and `outcome.json`. Saved outcome still says `independent_admission: PENDING`; this report admits only Anniversary, without mutating aggregate receipts.

Legacy pre/post maps match all 3846 entries, with zero differences, and equal accepted unit2 baseline and Anniversary precompile/postcompile/postexecution maps. `source_equal` and `artifact_unchanged` are true. No live source/binary hashes were computed. Accepted unit2 independent receipt: `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/independent-report.md`; its Anniversary source/binary identity acceptance is composed, not re-executed or expanded here.

Legacy artifact metadata equals unit2 Anniversary metadata: 433309856 bytes, saved SHA256 `51e77ea6d2c90ef7217bfce41eff4454c52b68035896579f4bf818fdb8aa108c`, sealed copy `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/20261010T235452Z/anniversary/integration-test`. Single saved integration compiler record has matching features `casc,client-anniversary,gui,rodio,sound`, matching test profile (opt 1, line-tables-only, debug assertions and overflow checks), and target `tests/integration.rs`. Compile exit 0, stream_errors empty, source_equal true; submission revision `24c727075a69a8a4bbf813156248e0547f3150f9`.

## Admission / controller / cgroup

Attempt 1 admitted: available 32.32240676879883 GiB (threshold 6), load1 2.47 (threshold <24), relevant Cargo list empty. Saved systemd submission exit 0; controller has exactly one completed record naming this epoch; controller stderr empty. Before/after membership matches `/user.slice/user-1000.slice/user@1000.service/agents.slice/wow-legacy-aura-other-20261011t015231z.service`; CPU limit unchanged `1200000 100000` (12 CPU quota), memory limit unchanged 17179869184 bytes (16 GiB). Aggregate cgroup usage 113971 → 5786821 µs; nr_throttled 0 → 0; memory.peak 26845184 → 1815957504 bytes. Shared epoch counters are not Anniversary-exclusive measurements.

## Boundaries

**Missing BlizzardSource remains BLOCKED.** This is not a bare-library ban: the explicit saved eight-test Anniversary partition is admitted; source-dependent lanes remain blocked and untouched. No transfer to other profiles, source-dependent startup, native BCC, exact tuple/caster ordering, or whole-profile completion. No tests, replay, builds, ops, delegation, Bash, rg, fd, live hashes, or repo edits performed. Only requested report/evidence files written in this epoch.
