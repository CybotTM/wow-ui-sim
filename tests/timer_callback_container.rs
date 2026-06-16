//! C_Timer / C_FunctionContainers container contract.
//!
//! Ground truth captured from retail 12.0.7.68182 via
//! docs/addons/TimerCallbackProbe: a ticker IS a FunctionContainer.
//! C_Timer.NewTimer/NewTicker accept a function OR a container, and return the
//! callback container itself, so a returned ticker can be fed back into another
//! New* call — with each registration keeping its own iteration count.

use std::time::{Duration, Instant};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::process_pending_timers;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

/// Pump the timer queue (which fires on wall-clock) until `expr` reaches
/// `target` or the deadline passes. Returns the final value of `expr`.
fn pump_until(env: &WowLuaEnv, expr: &str, target: i32, secs: u64) -> i32 {
    let deadline = Instant::now() + Duration::from_secs(secs);
    let mut value;
    loop {
        process_pending_timers(env);
        value = env.eval::<f64>(&format!("return {expr}")).unwrap() as i32;
        if value >= target || Instant::now() >= deadline {
            return value;
        }
        std::thread::sleep(Duration::from_millis(15));
    }
}

#[test]
fn newticker_accepts_container_and_returns_it() {
    let env = env();
    let (accepts, returns_same): (bool, bool) = env
        .eval(
            r#"
            local cb = C_FunctionContainers.CreateCallback(function() end)
            local ok, obj1 = pcall(C_Timer.NewTicker, 3600, cb, 1)
            local same = ok and (obj1 == cb) or false
            if ok and type(obj1) == "table" and obj1.Cancel then obj1:Cancel() end
            return ok, same
            "#,
        )
        .unwrap();
    assert!(accepts, "NewTicker must accept a FunctionContainer callback");
    assert!(returns_same, "NewTicker must return the same container it was given");
}

#[test]
fn newticker_wraps_plain_function_in_a_container() {
    let env = env();
    // A plain function is wrapped: the return is a cancelable container, not the fn.
    let (is_table, is_not_fn, has_cancel): (bool, bool, bool) = env
        .eval(
            r#"
            local f = function() end
            local obj = C_Timer.NewTicker(3600, f, 1)
            local r = { type(obj) == "table", obj ~= f, type(obj.Cancel) == "function" }
            if obj.Cancel then obj:Cancel() end
            return r[1], r[2], r[3]
            "#,
        )
        .unwrap();
    assert!(is_table && is_not_fn && has_cancel);
}

#[test]
fn shared_container_two_tickers_keep_independent_counts() {
    let env = env();
    // The "C_Timer state not shared" case: one callback container backs two
    // tickers; total invocations must be 5 + 3 = 8.
    let _: () = env
        .eval(
            r#"
            _G.__tcc_total = 0
            local cb = C_FunctionContainers.CreateCallback(function()
                _G.__tcc_total = _G.__tcc_total + 1
            end)
            local obj1 = C_Timer.NewTicker(0.05, cb, 5)
            assert(obj1 == cb, "ticker should return the container")
            C_Timer.NewTicker(0.08, obj1, 3)  -- reuse the returned ticker
            "#,
        )
        .unwrap();
    let total = pump_until(&env, "_G.__tcc_total", 8, 4);
    assert_eq!(total, 8, "shared callback container must fire 5+3 times (state not shared)");
}

#[test]
fn cancelling_container_stops_the_ticker() {
    let env = env();
    let _: () = env
        .eval(
            r#"
            _G.__tcc_cancel = 0
            local t = C_Timer.NewTicker(0.05, function() _G.__tcc_cancel = _G.__tcc_cancel + 1 end, 100)
            _G.__tcc_handle = t
            "#,
        )
        .unwrap();
    // Let it fire a couple of times, then cancel via the container.
    let _ = pump_until(&env, "_G.__tcc_cancel", 2, 2);
    let _: () = env.eval("__tcc_handle:Cancel()").unwrap();
    let after_cancel = env.eval::<f64>("return _G.__tcc_cancel").unwrap() as i32;
    // Pump well past several more intervals; count must not advance.
    let final_count = pump_until(&env, "_G.__tcc_cancel", after_cancel + 5, 1);
    assert_eq!(final_count, after_cancel, "Cancel() must stop further ticks");
}
