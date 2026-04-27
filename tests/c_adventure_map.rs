//! Integration tests for the `C_AdventureMap` namespace registered in
//! `src/lua_api/globals/adventure_map.rs`.
//!
//! `GetMapID()` is read during `AdventureMapMixin:OnShow` and forwarded to
//! `MapCanvasMixin:SetMapID` (see `Blizzard_AdventureMap.lua:45`).
//!
//! `Close()` is an async hint to the server that the player closed the
//! adventure map; the simulator stamps `state.adventure_map.last_closed`
//! with the elapsed game time so tests can assert it was invoked. The
//! function reference is stored directly on
//! `UIPanelWindows["AdventureMapFrame"].showFailedFunc`, so it must be
//! present at addon load time (see `Blizzard_AdventureMap.lua:56`).

use wow_ui_sim::lua_api::{AdventureMapInset, WowLuaEnv};

#[test]
fn c_adventure_map_namespace_is_a_table() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env.eval("return type(C_AdventureMap)").unwrap();
    assert_eq!(kind, "table");
}

#[test]
fn get_map_id_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetMapID)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_map_id_defaults_to_zero() {
    let env = WowLuaEnv::new().expect("env");
    let map_id: f64 = env.eval("return C_AdventureMap.GetMapID()").unwrap();
    assert!(map_id.abs() < 1e-6);
}

#[test]
fn get_map_id_returns_seeded_value() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.map_id = 619;

    let map_id: f64 = env.eval("return C_AdventureMap.GetMapID()").unwrap();
    assert!((map_id - 619.0).abs() < 1e-6);
}

#[test]
fn get_map_id_returns_a_number_type() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.map_id = 42;

    let kind: String = env
        .eval("return type(C_AdventureMap.GetMapID())")
        .unwrap();
    assert_eq!(kind, "number");
}

#[test]
fn close_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env.eval("return type(C_AdventureMap.Close)").unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn close_returns_no_values() {
    let env = WowLuaEnv::new().expect("env");
    let nothing: bool = env
        .eval("return select('#', C_AdventureMap.Close()) == 0")
        .unwrap();
    assert!(nothing, "Close should return zero values");
}

#[test]
fn close_records_a_timestamp_on_state() {
    let env = WowLuaEnv::new().expect("env");
    assert!(
        env.state().borrow().adventure_map.last_closed.is_none(),
        "last_closed should be None before any Close call"
    );

    env.exec("C_AdventureMap.Close()").unwrap();

    let last_closed = env
        .state()
        .borrow()
        .adventure_map
        .last_closed
        .expect("Close should populate last_closed");
    assert!(
        last_closed >= 0.0,
        "last_closed should be a non-negative elapsed-time value, got {last_closed}"
    );
}

#[test]
fn close_overwrites_previous_timestamp() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.last_closed = Some(1.0);

    env.exec("C_AdventureMap.Close()").unwrap();

    let last_closed = env.state().borrow().adventure_map.last_closed.unwrap();
    assert!(
        last_closed != 1.0,
        "Close should overwrite the seed timestamp"
    );
}

#[test]
fn close_can_be_stored_as_a_direct_reference() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        UIPanelWindows = UIPanelWindows or {}
        UIPanelWindows.AdventureMapFrame = { showFailedFunc = C_AdventureMap.Close }
        UIPanelWindows.AdventureMapFrame.showFailedFunc()
        "#,
    )
    .unwrap();

    assert!(
        env.state().borrow().adventure_map.last_closed.is_some(),
        "showFailedFunc reference should reach the simulator and stamp last_closed"
    );
}

#[test]
fn get_num_map_insets_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetNumMapInsets)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_num_map_insets_returns_nil_when_unloaded() {
    let env = WowLuaEnv::new().expect("env");
    let is_nil: bool = env
        .eval("return C_AdventureMap.GetNumMapInsets() == nil")
        .unwrap();
    assert!(
        is_nil,
        "GetNumMapInsets must return nil before inset metadata is published \
         so AdventureMapMixin:RefreshInsets can short-circuit"
    );
}

#[test]
fn get_num_map_insets_returns_zero_when_loaded_empty() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(Vec::new());

    let count: f64 = env
        .eval("return C_AdventureMap.GetNumMapInsets()")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_num_map_insets_returns_seeded_length() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![
        AdventureMapInset::default(),
        AdventureMapInset::default(),
        AdventureMapInset::default(),
    ]);

    let count: f64 = env
        .eval("return C_AdventureMap.GetNumMapInsets()")
        .unwrap();
    assert!((count - 3.0).abs() < 1e-6);
}

#[test]
fn refresh_insets_guard_short_circuits_on_nil() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        _G.__refresh_ran = false
        local numInsets = C_AdventureMap.GetNumMapInsets()
        if numInsets and numInsets > 0 then
            _G.__refresh_ran = true
        end
        "#,
    )
    .unwrap();

    let ran: bool = env.eval("return _G.__refresh_ran").unwrap();
    assert!(
        !ran,
        "RefreshInsets-style guard must skip the body when the count is nil"
    );
}
