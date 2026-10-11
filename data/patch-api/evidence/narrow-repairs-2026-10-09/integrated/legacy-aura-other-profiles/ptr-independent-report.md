# PTR independent saved-artifact admission

**PASS — 8/8 exact legacy cases passed; 0 failed, 8 exits 0.**

Epoch: `/home/osso/.local/state/wow-ui-sim/verification/legacy-aura-other-profiles-current/20261011T015231Z`. Read/followed `/home/osso/AgentConfig/skills/verify/SKILL.md`; assigned artifact-only verifier, no delegation. No tests, builds, replay, operations, Bash, rg/fd, live source inspection or source/binary hashing. Only saved receipts parsed; only requested report/evidence written.

## Selector and result admission

The eight selectors below are the user-specified first-eight legacy `tests/aura_api.rs` set, not modern `C_*` selectors. Every full stdout/stderr was read. Each stdout has exactly one named `... ok`, `running 1 test`, and `1 passed; 0 failed; 0 ignored; 0 measured; 9895 filtered out`. Every argv targets the same sealed PTR integration binary with `timeout 90`, `--exact`, `--nocapture`, `--test-threads=1`. All stderr consists of completed bare `WowLuaEnv::new` startup timing records; no failing diagnostic. Full logs reproduced below, not inferred from exit alone.

`ptr/executions.json` equals PTR executions in `results.json`; PTR result equals PTR entry in `outcome.json`, including source/artifact guards. Original outcome remains `independent_admission: PENDING`; this separate report admits PTR only and does not rewrite it or admit other profiles. Execution window: 2026-10-11T01:52:31.683650+00:00 through 2026-10-11T01:52:32.574490+00:00.

## Identity and guard composition

`ptr/artifact.json` exactly equals accepted unit2 epoch PTR receipt. Recorded seal: `363984787c902985c21d540dc69aa68baee9891ed2c46c892b8344b343af10a9`, 466938512 bytes, `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/20261010T235452Z/ptr/integration-test`. Accepted unit2 evidence names revision `cc4f4c782f66c955e635d857bfc85a79a021ca24` and endpoint `2026-10-11T00:06:46.420355+00:00`, same seal/size, empty PTR faults. Saved compiler record identifies test target `integration`, source `tests/integration.rs`, identical feature vector and build profile, exactly one client feature `client-ptr`. Compile receipt exit 0/source_equal true; previously accepted compilation has manifest warnings and is not warning-free proof.

Legacy controller executed saved request/worker verbatim. Its pre-execution guard rejects source map or binary seal mismatch before saving artifact/executing; post-PTR receipt records `artifact_unchanged: true`, `source_equal: true`. These are saved guard receipts, not newly recomputed hashes or hermetic provenance.

All 3846 entries are identical across legacy source-before/source-after, unit2 baseline, and unit2 PTR before-compile/after-compile/after-execution maps: zero missing/changed entries. Accepted unit2 endpoint reported zero current-source/blob mismatches. Saved `tests/aura_api.rs` entry exists with hash `21b3e7bfd5cb3fbe6798cf3280ce72f00a73382b5dece457d7eb6b3cd96bf48a`. No missing-AuraFile or mixed-selector inference; agent1909's rejected conclusion is not reused. Source-content ordering is supplied by this assignment, not reconstructed from hashes. Equality composes saved endpoints only; does not establish current live-tree identity or continuous immutability between snapshots.

## Admission, cgroup, controller

PTR admitted attempt 1: 32.86014175415039 GiB available >= 6, load1 2.51 < 24, relevant Cargo list empty. Submission exit 0. Both saved cgroup snapshots name `/user.slice/user-1000.slice/user@1000.service/agents.slice/wow-legacy-aura-other-20261011t015231z.service`; cpu.max `1200000 100000`, memory.max `17179869184` (16 GiB), matching submitted CPUQuota=1200%/MemoryMax=16G. Shared worker epoch peak rises 26845184 -> 1815957504 bytes; nr_throttled and throttled_usec remain 0. This cgroup spans all worker profiles, not exclusive PTR cost or per-test membership proof.

One controller completed record names this exact epoch; controller stderr is empty (0 bytes). Saved request code, worker.py and controller executed payload equal. Completion record is not an independent systemd exit receipt; no live service status inspected.

## Limits

Admitted: eight existing limited legacy assertions in bare library harness. Not exact legacy tuple values/arity/caster ordering, native WoW parity, modern C_* coverage, vendor/full-UI/cache loading, GUI, whole-profile completion, or source/cache/user startup acceptance. Test name `returns_all_fields` alone does not expand assertion strength. **sourcecacheuser KEEPBLOCKED unchanged.** Accepted unit2 normal startup proof stays separate; it is not promoted into this aura lane. Other profiles outside partition.

Issues: [].

## Full saved PTR output

### 0: `aura_api::test_unit_buff_returns_data_for_player` — exit 0

Full stdout:
```text

running 1 test
test aura_api::test_unit_buff_returns_data_for_player ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9895 filtered out; finished in 0.10s

```
Full stderr:
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.99ms
[  0.002s] [Startup] rilua VM created in 90.48µs
[  0.002s] [Startup] builtin frames initialized in 85.77µs
[  0.002s] [Startup] template registry cleared in 3.47µs
[  0.002s] [Startup] intrinsic templates registered in 5.04µs
[  0.002s] [Startup] initial app_data lua handle installed in 270.00ns
[  0.097s] [Startup] init_lua_state complete in 95.13ms
[  0.097s] [Startup] final app_data lua handle installed in 170.00ns
[  0.097s] [Startup] initial screen globals installed in 82.85µs
[  0.097s] [Startup] WowLuaEnv::new complete
```

### 1: `aura_api::test_unit_buff_returns_all_fields` — exit 0

Full stdout:
```text

running 1 test
test aura_api::test_unit_buff_returns_all_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9895 filtered out; finished in 0.10s

```
Full stderr:
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.59ms
[  0.002s] [Startup] rilua VM created in 146.95µs
[  0.002s] [Startup] builtin frames initialized in 88.81µs
[  0.002s] [Startup] template registry cleared in 7.60µs
[  0.002s] [Startup] intrinsic templates registered in 8.63µs
[  0.002s] [Startup] initial app_data lua handle installed in 261.00ns
[  0.095s] [Startup] init_lua_state complete in 92.62ms
[  0.095s] [Startup] final app_data lua handle installed in 160.00ns
[  0.095s] [Startup] initial screen globals installed in 50.66µs
[  0.095s] [Startup] WowLuaEnv::new complete
```

### 2: `aura_api::test_unit_buff_past_end_returns_nil` — exit 0

Full stdout:
```text

running 1 test
test aura_api::test_unit_buff_past_end_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9895 filtered out; finished in 0.10s

```
Full stderr:
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.52ms
[  0.002s] [Startup] rilua VM created in 88.17µs
[  0.002s] [Startup] builtin frames initialized in 59.99µs
[  0.002s] [Startup] template registry cleared in 3.92µs
[  0.002s] [Startup] intrinsic templates registered in 6.09µs
[  0.002s] [Startup] initial app_data lua handle installed in 170.00ns
[  0.094s] [Startup] init_lua_state complete in 92.04ms
[  0.094s] [Startup] final app_data lua handle installed in 181.00ns
[  0.094s] [Startup] initial screen globals installed in 52.34µs
[  0.094s] [Startup] WowLuaEnv::new complete
```

### 3: `aura_api::test_unit_buff_non_player_returns_nil` — exit 0

Full stdout:
```text

running 1 test
test aura_api::test_unit_buff_non_player_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9895 filtered out; finished in 0.10s

```
Full stderr:
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 2.05ms
[  0.002s] [Startup] rilua VM created in 93.91µs
[  0.002s] [Startup] builtin frames initialized in 126.40µs
[  0.002s] [Startup] template registry cleared in 5.24µs
[  0.002s] [Startup] intrinsic templates registered in 7.96µs
[  0.002s] [Startup] initial app_data lua handle installed in 561.00ns
[  0.097s] [Startup] init_lua_state complete in 94.84ms
[  0.097s] [Startup] final app_data lua handle installed in 421.00ns
[  0.097s] [Startup] initial screen globals installed in 66.00µs
[  0.097s] [Startup] WowLuaEnv::new complete
```

### 4: `aura_api::test_unit_buff_count_between_4_and_8` — exit 0

Full stdout:
```text

running 1 test
test aura_api::test_unit_buff_count_between_4_and_8 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9895 filtered out; finished in 0.10s

```
Full stderr:
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.90ms
[  0.002s] [Startup] rilua VM created in 85.72µs
[  0.002s] [Startup] builtin frames initialized in 56.36µs
[  0.002s] [Startup] template registry cleared in 3.22µs
[  0.002s] [Startup] intrinsic templates registered in 4.86µs
[  0.002s] [Startup] initial app_data lua handle installed in 240.00ns
[  0.095s] [Startup] init_lua_state complete in 92.91ms
[  0.095s] [Startup] final app_data lua handle installed in 180.00ns
[  0.095s] [Startup] initial screen globals installed in 53.09µs
[  0.095s] [Startup] WowLuaEnv::new complete
```

### 5: `aura_api::test_unit_aura_helpful_returns_data` — exit 0

Full stdout:
```text

running 1 test
test aura_api::test_unit_aura_helpful_returns_data ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9895 filtered out; finished in 0.10s

```
Full stderr:
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.56ms
[  0.002s] [Startup] rilua VM created in 87.96µs
[  0.002s] [Startup] builtin frames initialized in 55.81µs
[  0.002s] [Startup] template registry cleared in 3.93µs
[  0.002s] [Startup] intrinsic templates registered in 5.60µs
[  0.002s] [Startup] initial app_data lua handle installed in 541.00ns
[  0.094s] [Startup] init_lua_state complete in 92.52ms
[  0.094s] [Startup] final app_data lua handle installed in 431.00ns
[  0.094s] [Startup] initial screen globals installed in 51.63µs
[  0.094s] [Startup] WowLuaEnv::new complete
```

### 6: `aura_api::test_unit_aura_harmful_returns_nil` — exit 0

Full stdout:
```text

running 1 test
test aura_api::test_unit_aura_harmful_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9895 filtered out; finished in 0.11s

```
Full stderr:
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 2.07ms
[  0.002s] [Startup] rilua VM created in 89.92µs
[  0.002s] [Startup] builtin frames initialized in 58.85µs
[  0.002s] [Startup] template registry cleared in 4.17µs
[  0.002s] [Startup] intrinsic templates registered in 5.58µs
[  0.002s] [Startup] initial app_data lua handle installed in 341.00ns
[  0.102s] [Startup] init_lua_state complete in 99.92ms
[  0.102s] [Startup] final app_data lua handle installed in 171.00ns
[  0.102s] [Startup] initial screen globals installed in 64.51µs
[  0.102s] [Startup] WowLuaEnv::new complete
```

### 7: `aura_api::test_unit_debuff_returns_nil` — exit 0

Full stdout:
```text

running 1 test
test aura_api::test_unit_debuff_returns_nil ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9895 filtered out; finished in 0.10s

```
Full stderr:
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.58ms
[  0.002s] [Startup] rilua VM created in 89.94µs
[  0.002s] [Startup] builtin frames initialized in 56.23µs
[  0.002s] [Startup] template registry cleared in 3.94µs
[  0.002s] [Startup] intrinsic templates registered in 4.98µs
[  0.002s] [Startup] initial app_data lua handle installed in 261.00ns
[  0.101s] [Startup] init_lua_state complete in 98.81ms
[  0.101s] [Startup] final app_data lua handle installed in 391.00ns
[  0.101s] [Startup] initial screen globals installed in 66.64µs
[  0.101s] [Startup] WowLuaEnv::new complete
```
