//! Port of the master-era global overrides — see
//! `src/lua_api/globals/compat_overrides.rs`.
//!
//! Covers: `print` → `SimState.console_output`, `A_Print`, `next`-on-frame
//! short-circuit, `ipairs`-on-frame children iterator. `getmetatable` /
//! `setmetatable` are intentionally not intercepted; the
//! `setmetatable_mixin_still_works` test pins that decision.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

#[test]
fn print_writes_to_console_output() {
    let env = env();
    env.exec(r#"print("hello", 42, true)"#).unwrap();
    let output = env.state().borrow().console_output.clone();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], "hello\t42\ttrue");
}

#[test]
fn print_handles_nil_and_tables() {
    let env = env();
    env.exec(r#"print(nil, {}, function() end)"#).unwrap();
    let output = env.state().borrow().console_output.clone();
    assert_eq!(output[0], "nil\ttable\tfunction");
}

#[test]
fn print_formats_integers_without_decimal() {
    let env = env();
    env.exec(r#"print(7, -3, 0)"#).unwrap();
    let output = env.state().borrow().console_output.clone();
    assert_eq!(output[0], "7\t-3\t0");
}

#[test]
fn a_print_routes_to_sim_print() {
    let env = env();
    env.exec(r#"A_Print("from A_Print")"#).unwrap();
    let output = env.state().borrow().console_output.clone();
    assert_eq!(output.last().map(String::as_str), Some("from A_Print"));
}

#[test]
fn a_print_is_resilient_when_print_overridden() {
    // A_Print reads the captured print from the registry — even if addon
    // code overwrites _G.print, A_Print should still work.
    let env = env();
    env.exec(
        r#"
        _G.print = function() end
        A_Print("still captured")
        "#,
    )
    .unwrap();
    let output = env.state().borrow().console_output.clone();
    assert_eq!(output.last().map(String::as_str), Some("still captured"));
}

#[test]
fn next_on_frame_yields_nothing() {
    let env = env();
    let count: i64 = env
        .eval(
            r#"
            local f = CreateFrame("Frame", nil, UIParent)
            local n = 0
            for k, v in pairs(f) do n = n + 1 end
            return n
            "#,
        )
        .unwrap();
    assert_eq!(count, 0, "frames aren't iterable tables");
}

#[test]
fn next_on_plain_table_still_works() {
    let env = env();
    let sum: i64 = env
        .eval(
            r#"
            local total = 0
            for k, v in pairs({ a = 1, b = 2, c = 3 }) do total = total + v end
            return total
            "#,
        )
        .unwrap();
    assert_eq!(sum, 6);
}

#[test]
fn ipairs_on_frame_yields_children_in_order() {
    let env = env();
    let (c1_name, c2_name, c3_name): (String, String, String) = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "IpairsParent", UIParent)
            local c1 = CreateFrame("Frame", "Child1", parent)
            local c2 = CreateFrame("Frame", "Child2", parent)
            local c3 = CreateFrame("Frame", "Child3", parent)
            local names = {}
            for i, child in ipairs(parent) do
                names[i] = child:GetName()
            end
            return names[1] or "", names[2] or "", names[3] or ""
            "#,
        )
        .unwrap();
    assert_eq!(c1_name, "Child1");
    assert_eq!(c2_name, "Child2");
    assert_eq!(c3_name, "Child3");
}

#[test]
fn ipairs_on_childless_frame_yields_zero_iterations() {
    let env = env();
    let n: i64 = env
        .eval(
            r#"
            local f = CreateFrame("Frame", nil, UIParent)
            local n = 0
            for i, child in ipairs(f) do n = n + 1 end
            return n
            "#,
        )
        .unwrap();
    assert_eq!(n, 0);
}

#[test]
fn ipairs_on_plain_array_still_works() {
    let env = env();
    let total: i64 = env
        .eval(
            r#"
            local total = 0
            for i, v in ipairs({ 10, 20, 30 }) do total = total + v end
            return total
            "#,
        )
        .unwrap();
    assert_eq!(total, 60);
}

#[test]
fn setmetatable_mixin_still_works() {
    // setmetatable/getmetatable aren't intercepted by this module because
    // rilua's native userdata metatable handling already supports the
    // mixin pattern. Pin that by using it here.
    let env = env();
    let greeting: String = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", nil, UIParent)
            setmetatable(frame, { __index = { SayHi = function() return "hi!" end } })
            return frame:SayHi()
            "#,
        )
        .unwrap();
    assert_eq!(greeting, "hi!");
}
