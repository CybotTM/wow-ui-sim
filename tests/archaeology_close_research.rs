//! Integration tests for the legacy `CloseResearch()` global, the
//! server-side close hint fired by `ArchaeologyFrame_OnHide`
//! (`Blizzard_ArchaeologyUI/Blizzard_ArchaeologyUI.lua:51`) and
//! `ArchaeologyFrame_ShowFailed` (`:201`). The simulator records the
//! call timestamp into `state.archaeology.last_close_request`; addons
//! only need the call not to error on nil.

use std::time::Instant;
use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn close_research_default_is_none() {
    let env = env();
    let st = env.state().borrow();
    assert!(
        st.archaeology.last_close_request.is_none(),
        "fresh ArchaeologyState has not received a close hint yet",
    );
}

#[test]
fn close_research_records_a_timestamp() {
    let env = env();
    let before = Instant::now();
    env.exec("CloseResearch()").unwrap();
    let after = Instant::now();
    let st = env.state().borrow();
    let recorded = st
        .archaeology
        .last_close_request
        .expect("CloseResearch must record a timestamp on the archaeology state");
    assert!(
        recorded >= before && recorded <= after,
        "the recorded timestamp must fall within the call window",
    );
}

#[test]
fn close_research_overwrites_prior_timestamp() {
    let env = env();
    env.exec("CloseResearch()").unwrap();
    let first = env.state().borrow().archaeology.last_close_request.unwrap();
    // Subsequent OnHide / ShowFailed calls should advance the timestamp;
    // the field is "most recent close request", not "first".
    std::thread::sleep(std::time::Duration::from_millis(2));
    env.exec("CloseResearch()").unwrap();
    let second = env.state().borrow().archaeology.last_close_request.unwrap();
    assert!(
        second > first,
        "the second CloseResearch call must overwrite the previous timestamp",
    );
}

#[test]
fn close_research_returns_no_lua_values() {
    let env = env();
    let return_count: i32 = env.eval("return select('#', CloseResearch())").unwrap();
    assert_eq!(
        return_count, 0,
        "CloseResearch is documented as returning nothing; addons rely on the bare `CloseResearch()` call",
    );
}

#[test]
fn close_research_is_registered_as_a_function() {
    let env = env();
    let kind: String = env.eval("return type(CloseResearch)").unwrap();
    assert_eq!(
        kind, "function",
        "ArchaeologyFrame_OnHide / ArchaeologyFrame_ShowFailed call this directly; missing it would error on nil",
    );
}
