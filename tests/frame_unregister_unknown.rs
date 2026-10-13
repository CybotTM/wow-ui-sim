//! Selected removal-policy regression; native behavior remains unproved.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

const FIXTURE: &str = include_str!("fixtures/frame_unregister_unknown.lua");

#[test]
fn unregister_unknown_preserves_known_event_delivery_and_strict_registration() {
    let env = WowLuaEnv::new().expect("create bare simulator environment");
    env.exec(FIXTURE)
        .expect("state-based unknown removal and known event lifecycle controls");
}
