//! Staged standalone clock tests; no executed RED or timing proof.
//! Simulator nanosecond-clock policy only, not native WoW frequency parity.
#![cfg(feature = "retail-12-0-0")]

use std::time::Duration;
use wow_ui_sim::lua_api::WowLuaEnv;

fn frequency(env: &WowLuaEnv) -> f64 {
    env.eval("return C_AddOnProfiler.GetTicksPerSecond()")
        .expect("standalone GetTicksPerSecond query")
}

#[test]
fn zero_argument_query_returns_exactly_one_positive_number() {
    let env = WowLuaEnv::new().expect("Lua environment");
    let valid: bool = env
        .eval(
            r#"
            local function validate(...)
                if select('#', ...) ~= 1 then return false end
                local value = ...
                return type(value) == 'number'
                    and value > 0 and value < math.huge
            end
            return validate(C_AddOnProfiler.GetTicksPerSecond())
            "#,
        )
        .expect("zero-argument query result validation");
    assert!(
        valid,
        "query must return exactly one finite positive Lua number"
    );
}

#[test]
fn frequency_is_exact_nanosecond_policy_and_deterministic() {
    // Literal is a private expectation, not a repository production constant.
    for _ in 0..2 {
        let env = WowLuaEnv::new().expect("Lua environment");
        for _ in 0..8 {
            assert_eq!(frequency(&env), 1_000_000_000.0);
        }
    }
}

#[test]
fn frequency_converts_fixed_duration_nanoseconds_consistently() {
    let env = WowLuaEnv::new().expect("Lua environment");
    let ticks_per_second = frequency(&env);
    assert_eq!(ticks_per_second, 1_000_000_000.0);

    // Fixed inputs, not sleeps or measured callbacks. Proposed tick unit is
    // Duration::as_nanos(); this does not test any production elapsed field.
    for (nanoseconds, expected_seconds, expected_milliseconds) in [
        (0_u64, 0.0, 0.0),
        (1, 0.000_000_001, 0.000_001),
        (250_000_000, 0.25, 250.0),
        (1_000_000_000, 1.0, 1_000.0),
        (2_250_000_000, 2.25, 2_250.0),
    ] {
        let duration = Duration::from_nanos(nanoseconds);
        let ticks = duration.as_nanos();
        assert_eq!(ticks, u128::from(nanoseconds));
        let seconds = ticks as f64 / ticks_per_second;
        let milliseconds = ticks as f64 * 1_000.0 / ticks_per_second;
        assert_eq!(seconds, expected_seconds);
        assert_eq!(milliseconds, expected_milliseconds);
        assert_eq!(seconds, duration.as_secs_f64());
    }
}
