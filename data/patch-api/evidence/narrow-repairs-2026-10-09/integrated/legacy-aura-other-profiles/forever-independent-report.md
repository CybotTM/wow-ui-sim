# Forever independent saved-partition report

**PASS: 8/8 existing legacy assertions; eight recorded exits 0; exactly one named case per invocation.**

Epoch: `/home/osso/.local/state/wow-ui-sim/verification/legacy-aura-other-profiles-current/20261011T015231Z`. Verification follows verify/SKILL.md as artifact inspection; no commands replayed or delegated.

## Exact coverage

| Index | Exact selector | Assertion boundary | Recorded result |
|---:|---|---|---|
| 0 | `aura_api::test_unit_buff_returns_data_for_player` | Nonempty player buff name | 1 passed / 0 failed; exit 0; 8338 filtered |
| 1 | `aura_api::test_unit_buff_returns_all_fields` | Name, spellId, duration are non-nil only (not all tuple fields) | 1 passed / 0 failed; exit 0; 8338 filtered |
| 2 | `aura_api::test_unit_buff_past_end_returns_nil` | Player buff index 100 returns nil | 1 passed / 0 failed; exit 0; 8338 filtered |
| 3 | `aura_api::test_unit_buff_non_player_returns_nil` | Target buff index 1 returns nil | 1 passed / 0 failed; exit 0; 8338 filtered |
| 4 | `aura_api::test_unit_buff_count_between_4_and_8` | Player fixture buff count between 4 and 8 | 1 passed / 0 failed; exit 0; 8338 filtered |
| 5 | `aura_api::test_unit_aura_helpful_returns_data` | Nonempty player HELPFUL aura name | 1 passed / 0 failed; exit 0; 8338 filtered |
| 6 | `aura_api::test_unit_aura_harmful_returns_nil` | Player HARMFUL aura returns nil | 1 passed / 0 failed; exit 0; 8338 filtered |
| 7 | `aura_api::test_unit_debuff_returns_nil` | Player debuff index 1 returns nil | 1 passed / 0 failed; exit 0; 8338 filtered |

These are the first eight tests in `tests/aura_api.rs`, not modern selectors. `test_unit_buff_returns_all_fields` checks only three non-nil values; its name does not prove full tuple compatibility.

## Composition and admission

Accepted Unit2 report: `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/independent-report.md`; accepted evidence endpoint `2026-10-11T00:06:46.420355+00:00`, proof revision `cc4f4c782f66c955e635d857bfc85a79a021ca24`. Forever metadata matches accepted seal `ca952f984697f20e6b2ac4183b106c1aeb842ade679d8098ad1fc49874586e7b` (439867624 bytes), build profile, and single `client-wowforever` feature. Saved compiler record identifies `tests/integration.rs`.

Five recorded before/after source maps equal the 3846-entry Unit2 baseline: zero missing, extra, or changed entries. No live source/binary hash was computed. Worker source records pre-execution source and binary validation; completed controller plus Forever results records post-execution `artifact_unchanged=true` and `source_equal=true`. This is recorded guard composition, not a new live or hermetic identity proof.

Execution list agrees exactly with Forever results and outcome. Controller has one completed record naming this epoch and the exact saved worker; stderr empty; submission exit 0. A separate systemd process exit receipt is not present; completion is established by the saved controller record, not invented exit evidence. Original global independent_admission remains PENDING and is not edited.

Admission attempt 1: available 32.36018371582031 GiB (threshold 6), load1 2.47 (threshold <24), relevant cargo list empty. Recorded admitted=true agrees with policy.

Before/after cgroup membership matches submitted agents.slice service; CPU max `1200000 100000` (1200%), memory max 17179869184 bytes (16 GiB). Shared epoch memory peak rises from 26845184 to 1815957504 bytes, below cap; nr_throttled remains 0. These counters cover the four-profile worker, not Forever-exclusive resource attribution.

## Checks

- PASS — eight_exact_legacy_cases
- PASS — results_outcome_execution_agreement
- PASS — recorded_artifact_postguard
- PASS — recorded_source_guard
- PASS — metadata_accepted_unit2_composition
- PASS — compiler_metadata_composition
- PASS — controller_completed
- PASS — admission
- PASS — cgroup_limits_membership

## Limits

Saved Forever legacy assertion admission only. No new native keys/catalog/aliases/models. No exact tuple, caster ordering, native parity, startup-lane, or whole-profile acceptance. No repository edits. Only the two requested epoch reports are written.

## Full saved stdout/stderr and exits

### 0: `aura_api::test_unit_buff_returns_data_for_player` — exit 0

stdout (176 bytes):
```text

running 1 test
test aura_api::test_unit_buff_returns_data_for_player ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8338 filtered out; finished in 0.11s

```

stderr (639 bytes):
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.001s] [Startup] SimState::default complete in 1.45ms
[  0.002s] [Startup] rilua VM created in 86.54µs
[  0.002s] [Startup] builtin frames initialized in 50.54µs
[  0.002s] [Startup] template registry cleared in 3.63µs
[  0.002s] [Startup] intrinsic templates registered in 6.89µs
[  0.002s] [Startup] initial app_data lua handle installed in 411.00ns
[  0.103s] [Startup] init_lua_state complete in 101.16ms
[  0.103s] [Startup] final app_data lua handle installed in 160.00ns
[  0.103s] [Startup] initial screen globals installed in 61.34µs
[  0.103s] [Startup] WowLuaEnv::new complete
```

### 1: `aura_api::test_unit_buff_returns_all_fields` — exit 0

stdout (171 bytes):
```text

running 1 test
test aura_api::test_unit_buff_returns_all_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8338 filtered out; finished in 0.11s

```

stderr (638 bytes):
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.97ms
[  0.002s] [Startup] rilua VM created in 90.51µs
[  0.002s] [Startup] builtin frames initialized in 64.19µs
[  0.002s] [Startup] template registry cleared in 4.20µs
[  0.002s] [Startup] intrinsic templates registered in 5.89µs
[  0.002s] [Startup] initial app_data lua handle installed in 421.00ns
[  0.102s] [Startup] init_lua_state complete in 99.44ms
[  0.102s] [Startup] final app_data lua handle installed in 180.00ns
[  0.102s] [Startup] initial screen globals installed in 55.28µs
[  0.102s] [Startup] WowLuaEnv::new complete
```

### 2: `aura_api::test_unit_buff_past_end_returns_nil` — exit 0

stdout (173 bytes):
```text

running 1 test
test aura_api::test_unit_buff_past_end_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8338 filtered out; finished in 0.11s

```

stderr (638 bytes):
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 2.21ms
[  0.002s] [Startup] rilua VM created in 85.73µs
[  0.002s] [Startup] builtin frames initialized in 53.71µs
[  0.002s] [Startup] template registry cleared in 3.92µs
[  0.002s] [Startup] intrinsic templates registered in 6.72µs
[  0.002s] [Startup] initial app_data lua handle installed in 270.00ns
[  0.102s] [Startup] init_lua_state complete in 99.72ms
[  0.102s] [Startup] final app_data lua handle installed in 170.00ns
[  0.102s] [Startup] initial screen globals installed in 52.23µs
[  0.102s] [Startup] WowLuaEnv::new complete
```

### 3: `aura_api::test_unit_buff_non_player_returns_nil` — exit 0

stdout (175 bytes):
```text

running 1 test
test aura_api::test_unit_buff_non_player_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8338 filtered out; finished in 0.13s

```

stderr (640 bytes):
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.75ms
[  0.002s] [Startup] rilua VM created in 86.70µs
[  0.002s] [Startup] builtin frames initialized in 53.76µs
[  0.002s] [Startup] template registry cleared in 5.32µs
[  0.002s] [Startup] intrinsic templates registered in 7.25µs
[  0.002s] [Startup] initial app_data lua handle installed in 250.00ns
[  0.130s] [Startup] init_lua_state complete in 128.15ms
[  0.130s] [Startup] final app_data lua handle installed in 251.00ns
[  0.130s] [Startup] initial screen globals installed in 103.45µs
[  0.130s] [Startup] WowLuaEnv::new complete
```

### 4: `aura_api::test_unit_buff_count_between_4_and_8` — exit 0

stdout (174 bytes):
```text

running 1 test
test aura_api::test_unit_buff_count_between_4_and_8 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8338 filtered out; finished in 0.11s

```

stderr (641 bytes):
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 2.22ms
[  0.002s] [Startup] rilua VM created in 115.50µs
[  0.002s] [Startup] builtin frames initialized in 79.64µs
[  0.002s] [Startup] template registry cleared in 6.78µs
[  0.002s] [Startup] intrinsic templates registered in 10.63µs
[  0.002s] [Startup] initial app_data lua handle installed in 651.00ns
[  0.110s] [Startup] init_lua_state complete in 107.83ms
[  0.110s] [Startup] final app_data lua handle installed in 220.00ns
[  0.110s] [Startup] initial screen globals installed in 70.25µs
[  0.110s] [Startup] WowLuaEnv::new complete
```

### 5: `aura_api::test_unit_aura_helpful_returns_data` — exit 0

stdout (173 bytes):
```text

running 1 test
test aura_api::test_unit_aura_helpful_returns_data ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8338 filtered out; finished in 0.13s

```

stderr (640 bytes):
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.54ms
[  0.002s] [Startup] rilua VM created in 132.86µs
[  0.002s] [Startup] builtin frames initialized in 66.18µs
[  0.002s] [Startup] template registry cleared in 5.71µs
[  0.002s] [Startup] intrinsic templates registered in 8.00µs
[  0.002s] [Startup] initial app_data lua handle installed in 431.00ns
[  0.123s] [Startup] init_lua_state complete in 121.51ms
[  0.123s] [Startup] final app_data lua handle installed in 180.00ns
[  0.123s] [Startup] initial screen globals installed in 85.23µs
[  0.123s] [Startup] WowLuaEnv::new complete
```

### 6: `aura_api::test_unit_aura_harmful_returns_nil` — exit 0

stdout (172 bytes):
```text

running 1 test
test aura_api::test_unit_aura_harmful_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8338 filtered out; finished in 0.12s

```

stderr (639 bytes):
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.67ms
[  0.002s] [Startup] rilua VM created in 91.44µs
[  0.002s] [Startup] builtin frames initialized in 55.52µs
[  0.002s] [Startup] template registry cleared in 3.30µs
[  0.002s] [Startup] intrinsic templates registered in 8.56µs
[  0.002s] [Startup] initial app_data lua handle installed in 450.00ns
[  0.116s] [Startup] init_lua_state complete in 113.95ms
[  0.116s] [Startup] final app_data lua handle installed in 171.00ns
[  0.116s] [Startup] initial screen globals installed in 63.93µs
[  0.116s] [Startup] WowLuaEnv::new complete
```

### 7: `aura_api::test_unit_debuff_returns_nil` — exit 0

stdout (166 bytes):
```text

running 1 test
test aura_api::test_unit_debuff_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8338 filtered out; finished in 0.14s

```

stderr (639 bytes):
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.83ms
[  0.002s] [Startup] rilua VM created in 90.07µs
[  0.002s] [Startup] builtin frames initialized in 88.00µs
[  0.002s] [Startup] template registry cleared in 8.46µs
[  0.002s] [Startup] intrinsic templates registered in 7.43µs
[  0.002s] [Startup] initial app_data lua handle installed in 301.00ns
[  0.137s] [Startup] init_lua_state complete in 135.00ms
[  0.137s] [Startup] final app_data lua handle installed in 150.00ns
[  0.137s] [Startup] initial screen globals installed in 83.71µs
[  0.137s] [Startup] WowLuaEnv::new complete
```

