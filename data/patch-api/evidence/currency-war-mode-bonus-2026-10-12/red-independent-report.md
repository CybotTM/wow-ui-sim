# Independent initial RED gate — C_CurrencyInfo.DoesWarModeBonusApply

**AUTHENTIC INITIAL RED established. Feature NOT implemented or GREEN.**

Revision: `4d4b80c432c7c13667a92e87ad42fbac03562070`. Epoch: `red-compile-20261012T073500599307Z`.

## Artifact authentication

Single read-only streaming pass of this new `integration-sealed`: **467732696 bytes**, SHA256 **35bc2c79e3b11052f0a79227eb7e3c377c4ba07e8aa0f91d7af452ee21e34789**. Both equal supplied expectations. Immediate auth receipt written before long record inspection/output. No other binary/rlib read, authentication rerun or artifact execution.

## Setup and provenance

Recorded compilation: `/usr/bin/cargo test --test integration --no-run --message-format=json --offline --locked -j 12`, exit **0**, `source_unchanged: true`. Full compile JSON stream read and parsed: **574432 bytes, 741 records**. Exactly one selected integration compiler-artifact record, `fresh: false`, equal to artifact.json cargo_record; terminal `build-finished success: true`. Six existing iced manifest deprecation warnings are present; not a clean-warning claim. Recorded build duration 42.93s.

Toolchain: Cargo 1.99.0 (5f94df478, 2026-08-27); rustc 1.99.0 (b940084d7, 2026-09-28), x86_64-unknown-linux-gnu, LLVM 23.1.1. Selected compiler features include `client-retail`, `profile-retail`, `default`, `retail-12-0-0`, `retail-12-0-5`, `retail-12-0-7`, `retail-12-1-0`, plus GUI/CASC/sound and capability features recorded verbatim in proof JSON. **Not PTR evidence.**

Stored test-format result: exit 0, empty stdout/stderr. Stored manual test-readability review: no violations; positive identity proof explicitly deferred. No format/readability command rerun. Scoped committed test-source byte comparison against current test file: equal, git show exit 0. These are test-stage records, not producer verification.

Compile admission and both runtime admission records say admitted. Runtime submission exit 0 launched the sealed path, filter `currency_war_mode_bonus::`, `--nocapture --test-threads=1`, CASC and sound disabled, 90s runtime bound. Recorded terminal observation: `ActiveState=failed`, `SubState=failed`, `Result=exit-code`, `ExecMainStatus=101`, empty ControlGroup. Compile-only outcome.json (`COMPILED_NOT_EXECUTED`, zero cases) is a prior compilation-phase snapshot; runtime records and full raw streams establish subsequent execution.

## Exact RED coverage

**5 selected logical tests; 0 PASS, 5 FAIL; 0 ignored; 10758 filtered out; runtime exit 101; 0.39s.** Each environment finishes initialization. All Rust panic sites are `tests/currency_war_mode_bonus.rs:24:10`; inner Lua failures below identify the actual boundary.

| Test | Observed first failure | Reached / not reached |
|---|---|---|
| absent_records_return_two_nil_flags | `currency bonus query returns two values` | First `get(90001)` reaches helper arity assertion. Nil-flags assertion, 90002 and repeat call not reached. |
| requires_number_without_string_coercion | `required nonnumeric currency input must be rejected` | No-argument pcall fails required rejection assertion. Error type/context, explicit nil, string/boolean/table/function cases and final numeric control not reached. |
| tainted_secrets_denied_before_validation | `secret denied` | Tainted-entry assertion reached; first valid secret pcall fails combined denial assertion. Exact pcall result/message not logged. Auth diagnostic, precedence, preservation/declassification, invalid secret and restored caller controls not reached. |
| unrepresented_numbers_have_no_flags | `currency bonus query returns two values` | First input 0 reaches arity assertion. Nil flags and remaining numeric inputs not reached. No positive identity mapping control exists. |
| untainted_secrets_preserve_wrapper_and_caller | `currency bonus query returns two values` | Wrapper construction and untainted entry reached; first secret call reaches arity assertion. Second query, returned nil flags, wrapper/value preservation and final caller assertion not reached. |

Failed pcall-based tests are **not** successes. No numeric-validation, authentication-order or preservation claim follows from merely catching an error. No exact actual output count is captured: only failure of expected two-value assertion.

## Dedicated source versus readable public surface

`src/c_api/item_spell/c_currency.rs:12-36` lists dedicated currency methods; `DoesWarModeBonusApply` is not in that list. Registration at :45-49 installs the listed methods. No owned flag producer is part of this test-stage scope.

Separately, `src/lua_api/env_init/runtime_surface_bootstrap.lua:64-78` supplies a generic namespace __index that creates/caches a nil-returning Lua closure; :112-126 attaches that metatable and :209-215 seeds C_* namespace names. This source explains why unsupported dedicated behavior can remain readable/callable publicly. Recorded numeric/untainted-secret calls reach `currency bonus query returns two values`, **not** an attempt-to-call-nil error. Attribute RED to unsupported query semantics at the dedicated model seam, consistent with public generic fallback, **not** an invented absent readable function. Runtime raw-field state, exact actual arity, native-stub/callable identity and caller identity were not captured and are not proved.

## Scope still open

Selected query policy only: explicitly supplied per-currency independently nullable flags, no normal membership seed, absent `(nil,nil)`. Initial tests have no positive supplied-state mapping, independent true/false/nil flags, updates/removal or identity non-aliasing proof. Native absence/catalog eligibility is not established. Separate PvP/quest/faction caller models, normal startup publication, Blizzard consumers, PTR execution, native identity and full-goal completion remain unimplemented/unproved here. A docs-only subsequent HEAD does not invalidate identical test bytes; no request to block docs commits.

## Evidence artifacts

Full compile stdout/stderr and runtime stdout/stderr are preserved verbatim in `red-independent-proof.json`, alongside all selected records and case reachability. This verifier performed no runtime/tests/build/check/format, delegation, network, guard, operations or polling. Only authentication receipt and the two requested final reports were written.

### Full raw runtime stdout

```text

running 5 tests
test currency_war_mode_bonus::currency_war_mode_bonus_absent_records_return_two_nil_flags ... FAILED
test currency_war_mode_bonus::currency_war_mode_bonus_requires_number_without_string_coercion ... FAILED
test currency_war_mode_bonus::currency_war_mode_bonus_tainted_secrets_denied_before_validation ... FAILED
test currency_war_mode_bonus::currency_war_mode_bonus_unrepresented_numbers_have_no_flags ... FAILED
test currency_war_mode_bonus::currency_war_mode_bonus_untainted_secrets_preserve_wrapper_and_caller ... FAILED

failures:

failures:
    currency_war_mode_bonus::currency_war_mode_bonus_absent_records_return_two_nil_flags
    currency_war_mode_bonus::currency_war_mode_bonus_requires_number_without_string_coercion
    currency_war_mode_bonus::currency_war_mode_bonus_tainted_secrets_denied_before_validation
    currency_war_mode_bonus::currency_war_mode_bonus_unrepresented_numbers_have_no_flags
    currency_war_mode_bonus::currency_war_mode_bonus_untainted_secrets_preserve_wrapper_and_caller

test result: FAILED. 0 passed; 5 failed; 0 ignored; 0 measured; 10758 filtered out; finished in 0.39s

```

### Full raw runtime stderr

```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.014s] [Startup] SimState::default complete in 13.85ms
[  0.016s] [Startup] rilua VM created in 1.16ms
[  0.016s] [Startup] builtin frames initialized in 860.36µs
[  0.017s] [Startup] template registry cleared in 339.79µs
[  0.017s] [Startup] intrinsic templates registered in 87.99µs
[  0.017s] [Startup] initial app_data lua handle installed in 611.00ns
[  0.133s] [Startup] init_lua_state complete in 116.09ms
[  0.133s] [Startup] final app_data lua handle installed in 190.00ns
[  0.133s] [Startup] initial screen globals installed in 75.32µs
[  0.133s] [Startup] WowLuaEnv::new complete

thread 'currency_war_mode_bonus::currency_war_mode_bonus_absent_records_return_two_nil_flags' (904035) panicked at /home/osso/Projects/wow/wow-ui-sim/tests/currency_war_mode_bonus.rs:24:10:
public currency War Mode query behavior: Lua(Runtime(RuntimeError { message: "currency bonus query returns two values", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[  0.140s] [Startup] WowLuaEnv::new begin
[  0.141s] [Startup] SimState::default complete in 1.62ms
[  0.141s] [Startup] rilua VM created in 64.82µs
[  0.141s] [Startup] builtin frames initialized in 34.64µs
[  0.141s] [Startup] template registry cleared in 2.66µs
[  0.141s] [Startup] intrinsic templates registered in 4.59µs
[  0.141s] [Startup] initial app_data lua handle installed in 180.00ns
[  0.197s] [Startup] init_lua_state complete in 56.13ms
[  0.197s] [Startup] final app_data lua handle installed in 161.00ns
[  0.198s] [Startup] initial screen globals installed in 64.84µs
[  0.198s] [Startup] WowLuaEnv::new complete

thread 'currency_war_mode_bonus::currency_war_mode_bonus_requires_number_without_string_coercion' (904036) panicked at /home/osso/Projects/wow/wow-ui-sim/tests/currency_war_mode_bonus.rs:24:10:
public currency War Mode query behavior: Lua(Runtime(RuntimeError { message: "required nonnumeric currency input must be rejected", level: 0, traceback: [] }))
[  0.201s] [Startup] WowLuaEnv::new begin
[  0.203s] [Startup] SimState::default complete in 1.31ms
[  0.203s] [Startup] rilua VM created in 60.23µs
[  0.203s] [Startup] builtin frames initialized in 29.26µs
[  0.203s] [Startup] template registry cleared in 1.06µs
[  0.203s] [Startup] intrinsic templates registered in 3.96µs
[  0.203s] [Startup] initial app_data lua handle installed in 100.00ns
[  0.260s] [Startup] init_lua_state complete in 57.64ms
[  0.260s] [Startup] final app_data lua handle installed in 150.00ns
[  0.260s] [Startup] initial screen globals installed in 52.35µs
[  0.260s] [Startup] WowLuaEnv::new complete

thread 'currency_war_mode_bonus::currency_war_mode_bonus_tainted_secrets_denied_before_validation' (904037) panicked at /home/osso/Projects/wow/wow-ui-sim/tests/currency_war_mode_bonus.rs:24:10:
public currency War Mode query behavior: Lua(Runtime(RuntimeError { message: "secret denied", level: 0, traceback: [] }))
[  0.264s] [Startup] WowLuaEnv::new begin
[  0.265s] [Startup] SimState::default complete in 1.46ms
[  0.266s] [Startup] rilua VM created in 66.67µs
[  0.266s] [Startup] builtin frames initialized in 29.22µs
[  0.266s] [Startup] template registry cleared in 1.06µs
[  0.266s] [Startup] intrinsic templates registered in 3.87µs
[  0.266s] [Startup] initial app_data lua handle installed in 80.00ns
[  0.324s] [Startup] init_lua_state complete in 57.92ms
[  0.324s] [Startup] final app_data lua handle installed in 80.00ns
[  0.324s] [Startup] initial screen globals installed in 50.55µs
[  0.324s] [Startup] WowLuaEnv::new complete

thread 'currency_war_mode_bonus::currency_war_mode_bonus_unrepresented_numbers_have_no_flags' (904038) panicked at /home/osso/Projects/wow/wow-ui-sim/tests/currency_war_mode_bonus.rs:24:10:
public currency War Mode query behavior: Lua(Runtime(RuntimeError { message: "currency bonus query returns two values", level: 0, traceback: [] }))
[  0.327s] [Startup] WowLuaEnv::new begin
[  0.328s] [Startup] SimState::default complete in 1.04ms
[  0.328s] [Startup] rilua VM created in 63.16µs
[  0.328s] [Startup] builtin frames initialized in 21.61µs
[  0.328s] [Startup] template registry cleared in 2.01µs
[  0.328s] [Startup] intrinsic templates registered in 3.87µs
[  0.328s] [Startup] initial app_data lua handle installed in 90.00ns
[  0.386s] [Startup] init_lua_state complete in 58.56ms
[  0.386s] [Startup] final app_data lua handle installed in 391.00ns
[  0.386s] [Startup] initial screen globals installed in 60.66µs
[  0.386s] [Startup] WowLuaEnv::new complete

thread 'currency_war_mode_bonus::currency_war_mode_bonus_untainted_secrets_preserve_wrapper_and_caller' (904039) panicked at /home/osso/Projects/wow/wow-ui-sim/tests/currency_war_mode_bonus.rs:24:10:
public currency War Mode query behavior: Lua(Runtime(RuntimeError { message: "currency bonus query returns two values", level: 0, traceback: [] }))
```
