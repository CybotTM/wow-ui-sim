//! Native Wrath callback fixture shared by F04 and F05.
//!
//! Load the committed profile's FrameXML TOC in native order. Never synthesize
//! modern mixins, parentKeys, event routers or replacement callback functions.
#[path = "wrath_action_assertions.rs"]
pub mod assertions;

use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

pub fn is_wrath() -> bool {
    ACTIVE == ClientProfile::Wrath
}

pub fn build_wrath_env() -> WowLuaEnv {
    assert!(is_wrath(), "native Wrath fixture requires client-wrath");
    let env = WowLuaEnv::new().expect("create Wrath callback environment");
    env.set_screen_size(1024.0, 768.0);
    super::panel_fixtures::seed_addon_search_paths(&env);
    super::load_profile_framexml(&env);
    env.apply_post_load_workarounds();
    wow_ui_sim::startup::fire_startup_events(&env);
    env.fire_event("ACTIONBAR_SHOWGRID").unwrap();
    env.fire_event("UPDATE_BINDINGS").unwrap();
    env.exec(
        r#"
        assert(MainMenuBar and ActionButton1 and ShapeshiftBarFrame and PetActionBarFrame)
        local source = debug.getinfo(ActionButtonDown, "S").source
        assert(string.find(source, "ActionButton.lua", 1, true), source)
        "#,
    )
    .expect("native action frames and original callback must load");
    env
}

pub fn wrath_env() -> super::LockedEnv {
    super::lock_env(build_wrath_env)
}
