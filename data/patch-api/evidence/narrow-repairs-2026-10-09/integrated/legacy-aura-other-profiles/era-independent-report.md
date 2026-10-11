# ERA independent saved-artifact admission

**PASS — 8/8 bounded legacy aura cases; 8 exact commands; each selects 1, passes 1, fails 0, exits 0.**

Epoch: `/home/osso/.local/state/wow-ui-sim/verification/legacy-aura-other-profiles-current/20261011T015231Z`. No execution/replay or repo changes. Agent1910 mixed modern-selector/hash claim is excluded.

## Exact receipts

| Index | Exact selector | Selected | Passed/failed | Exit | Filtered |
|---:|---|---:|---|---:|---:|
| 0 | `aura_api::test_unit_buff_returns_data_for_player` | 1 | 1/0 | 0 | 8022 |
| 1 | `aura_api::test_unit_buff_returns_all_fields` | 1 | 1/0 | 0 | 8022 |
| 2 | `aura_api::test_unit_buff_past_end_returns_nil` | 1 | 1/0 | 0 | 8022 |
| 3 | `aura_api::test_unit_buff_non_player_returns_nil` | 1 | 1/0 | 0 | 8022 |
| 4 | `aura_api::test_unit_buff_count_between_4_and_8` | 1 | 1/0 | 0 | 8022 |
| 5 | `aura_api::test_unit_aura_helpful_returns_data` | 1 | 1/0 | 0 | 8022 |
| 6 | `aura_api::test_unit_aura_harmful_returns_nil` | 1 | 1/0 | 0 | 8022 |
| 7 | `aura_api::test_unit_debuff_returns_nil` | 1 | 1/0 | 0 | 8022 |

All eight full stdout/stderr streams were read. Named `... ok` records, one result summary, running count, flags and sealed ERA binary path match each execution receipt. Stderr contains environment-initialization timings through `WowLuaEnv::new complete`; this is not source-dependent Blizzard startup evidence.

## Controller and outcome

ERA executions equal ERA entries in both results.json and outcome.json. Controller has one completed evaluation, exact epoch console path, empty stderr; executed worker equals saved request and worker.py. Submission exit 0. Original aggregate independent_admission remains PENDING; only this ERA partition is admitted. Other profiles are not reviewed.

## Saved identity/source composition

ERA seal: `869a9db256f221337bf7a40df8e4d9d5c5c5e35124e97daa25b11567c5dc2cff`, 433600848 bytes. Metadata equals accepted unit2 ERA receipt; accepted independent identity evidence matches seal and size. Saved compiler record names integration test target tests/integration.rs, features ['casc', 'client-era', 'gui', 'rodio', 'sound'], profile {'debug_assertions': True, 'debuginfo': 'line-tables-only', 'opt_level': '1', 'overflow_checks': True, 'test': True}.

All 3846 entries in legacy before/after maps equal accepted unit2 baseline and ERA before/after-compile/after-execution maps: 0 missing, extra or changed entries. tests/aura_api.rs recorded digest: `21b3e7bfd5cb3fbe6798cf3280ce72f00a73382b5dece457d7eb6b3cd96bf48a`. Accepted source revision `cc4f4c782f66c955e635d857bfc85a79a021ca24`; ERA submission docs-only successor is recorded in accepted evidence.

Worker records pre-execution seal check, post-execution artifact_unchanged=true and source_equal=true. This review composes those guards with accepted unit2 identity maps; it does NOT recompute any live hash, prove hermetic build provenance or claim present checkout/binary identity. The eight requested legacy selectors are established from actual receipts, not agent1910. Physical first-eight source declaration order is not independently established because no saved source text is supplied; no live source read substitutes for saved-only evidence.

## Resources

ERA admission attempt 1: available 32.564910888671875 GiB >=6; load1 2.47 <24; relevant cargo []. Actual saved before/after membership `/user.slice/user-1000.slice/user@1000.service/agents.slice/wow-legacy-aura-other-20261011t015231z.service` matches submitted unit, CPUQuota=1200% and MemoryMax=16G: cpu.max=`1200000 100000`, memory.max=`17179869184`. Shared epoch memory.peak=1815957504 bytes, below limit; after cpu.stat nr_throttled=0. No ERA-only peak/per-child membership claim. No live cgroup query.

## BLOCKED and excluded

**ERA source-dependent startup remains BLOCKED per user: missing source, no provisioning, no replay.** No native claim; no exact tuple/caster ordering, whole-profile completion or other-profile admission. Saved initialization logs do not lift this blocker.

## Full ERA output (verbatim)

### 0 stdout — exit 0
```text

running 1 test
test aura_api::test_unit_buff_returns_data_for_player ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8022 filtered out; finished in 0.11s
```
### 0 stderr
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 2.02ms
[  0.002s] [Startup] rilua VM created in 112.66µs
[  0.002s] [Startup] builtin frames initialized in 67.09µs
[  0.002s] [Startup] template registry cleared in 10.18µs
[  0.002s] [Startup] intrinsic templates registered in 5.36µs
[  0.002s] [Startup] initial app_data lua handle installed in 501.00ns
[  0.105s] [Startup] init_lua_state complete in 102.57ms
[  0.105s] [Startup] final app_data lua handle installed in 211.00ns
[  0.105s] [Startup] initial screen globals installed in 72.68µs
[  0.105s] [Startup] WowLuaEnv::new complete
```

### 1 stdout — exit 0
```text

running 1 test
test aura_api::test_unit_buff_returns_all_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8022 filtered out; finished in 0.10s
```
### 1 stderr
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.73ms
[  0.002s] [Startup] rilua VM created in 109.29µs
[  0.002s] [Startup] builtin frames initialized in 69.98µs
[  0.002s] [Startup] template registry cleared in 10.73µs
[  0.002s] [Startup] intrinsic templates registered in 6.16µs
[  0.002s] [Startup] initial app_data lua handle installed in 260.00ns
[  0.096s] [Startup] init_lua_state complete in 93.61ms
[  0.096s] [Startup] final app_data lua handle installed in 171.00ns
[  0.096s] [Startup] initial screen globals installed in 77.24µs
[  0.096s] [Startup] WowLuaEnv::new complete
```

### 2 stdout — exit 0
```text

running 1 test
test aura_api::test_unit_buff_past_end_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8022 filtered out; finished in 0.09s
```
### 2 stderr
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.61ms
[  0.002s] [Startup] rilua VM created in 102.52µs
[  0.002s] [Startup] builtin frames initialized in 68.96µs
[  0.002s] [Startup] template registry cleared in 8.06µs
[  0.002s] [Startup] intrinsic templates registered in 5.16µs
[  0.002s] [Startup] initial app_data lua handle installed in 1.48µs
[  0.091s] [Startup] init_lua_state complete in 89.61ms
[  0.091s] [Startup] final app_data lua handle installed in 170.00ns
[  0.092s] [Startup] initial screen globals installed in 45.66µs
[  0.092s] [Startup] WowLuaEnv::new complete
```

### 3 stdout — exit 0
```text

running 1 test
test aura_api::test_unit_buff_non_player_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8022 filtered out; finished in 0.11s
```
### 3 stderr
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.54ms
[  0.002s] [Startup] rilua VM created in 94.73µs
[  0.002s] [Startup] builtin frames initialized in 62.44µs
[  0.002s] [Startup] template registry cleared in 6.97µs
[  0.002s] [Startup] intrinsic templates registered in 5.97µs
[  0.002s] [Startup] initial app_data lua handle installed in 381.00ns
[  0.104s] [Startup] init_lua_state complete in 101.89ms
[  0.104s] [Startup] final app_data lua handle installed in 160.00ns
[  0.104s] [Startup] initial screen globals installed in 59.72µs
[  0.104s] [Startup] WowLuaEnv::new complete
```

### 4 stdout — exit 0
```text

running 1 test
test aura_api::test_unit_buff_count_between_4_and_8 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8022 filtered out; finished in 0.09s
```
### 4 stderr
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.93ms
[  0.002s] [Startup] rilua VM created in 100.98µs
[  0.002s] [Startup] builtin frames initialized in 63.07µs
[  0.002s] [Startup] template registry cleared in 10.23µs
[  0.002s] [Startup] intrinsic templates registered in 4.78µs
[  0.002s] [Startup] initial app_data lua handle installed in 381.00ns
[  0.091s] [Startup] init_lua_state complete in 89.26ms
[  0.091s] [Startup] final app_data lua handle installed in 170.00ns
[  0.091s] [Startup] initial screen globals installed in 50.10µs
[  0.091s] [Startup] WowLuaEnv::new complete
```

### 5 stdout — exit 0
```text

running 1 test
test aura_api::test_unit_aura_helpful_returns_data ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8022 filtered out; finished in 0.10s
```
### 5 stderr
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 2.02ms
[  0.002s] [Startup] rilua VM created in 91.75µs
[  0.002s] [Startup] builtin frames initialized in 62.81µs
[  0.002s] [Startup] template registry cleared in 7.28µs
[  0.002s] [Startup] intrinsic templates registered in 6.36µs
[  0.002s] [Startup] initial app_data lua handle installed in 341.00ns
[  0.099s] [Startup] init_lua_state complete in 97.02ms
[  0.099s] [Startup] final app_data lua handle installed in 170.00ns
[  0.099s] [Startup] initial screen globals installed in 69.00µs
[  0.099s] [Startup] WowLuaEnv::new complete
```

### 6 stdout — exit 0
```text

running 1 test
test aura_api::test_unit_aura_harmful_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8022 filtered out; finished in 0.09s
```
### 6 stderr
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.96ms
[  0.002s] [Startup] rilua VM created in 96.82µs
[  0.002s] [Startup] builtin frames initialized in 60.68µs
[  0.002s] [Startup] template registry cleared in 5.80µs
[  0.002s] [Startup] intrinsic templates registered in 5.86µs
[  0.002s] [Startup] initial app_data lua handle installed in 461.00ns
[  0.090s] [Startup] init_lua_state complete in 87.79ms
[  0.090s] [Startup] final app_data lua handle installed in 391.00ns
[  0.090s] [Startup] initial screen globals installed in 51.75µs
[  0.090s] [Startup] WowLuaEnv::new complete
```

### 7 stdout — exit 0
```text

running 1 test
test aura_api::test_unit_debuff_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8022 filtered out; finished in 0.11s
```
### 7 stderr
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.53ms
[  0.002s] [Startup] rilua VM created in 93.13µs
[  0.002s] [Startup] builtin frames initialized in 55.00µs
[  0.002s] [Startup] template registry cleared in 7.76µs
[  0.002s] [Startup] intrinsic templates registered in 5.41µs
[  0.002s] [Startup] initial app_data lua handle installed in 210.00ns
[  0.103s] [Startup] init_lua_state complete in 101.25ms
[  0.103s] [Startup] final app_data lua handle installed in 160.00ns
[  0.103s] [Startup] initial screen globals installed in 67.64µs
[  0.103s] [Startup] WowLuaEnv::new complete
```
