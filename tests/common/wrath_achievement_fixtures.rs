//! Native Wrath achievement consumers. Cached FrameXML expectations are INFERRED:
//! these tests are simulator evidence, not authenticated 12340 client execution.

use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

pub fn assert_native(script: &str) -> bool {
    if ACTIVE != ClientProfile::Wrath {
        return false;
    }
    let env = load_core();
    let loaded = super::blizzard_addon_harness::load_blizzard_addon_closure_into_env(
        &env,
        &super::blizzard_ui_dir(),
        &["Blizzard_AchievementUI"],
        &[],
    );
    assert_eq!(loaded, ["Blizzard_AchievementUI"]);
    super::fire_addon_loaded(&env, "Blizzard_AchievementUI");
    let errors: Vec<_> = env.state().borrow().lua_errors.iter()
        .filter(|message| message.contains("Achievement") || message.contains("Achivement"))
        .cloned().collect();
    assert!(errors.is_empty(), "native AchievementUI load errors: {errors:?}");
    env.exec(script).expect("native Wrath achievement consumer should satisfy its contract");
    true
}

pub fn assert_lazy(script: &str) -> bool {
    if ACTIVE != ClientProfile::Wrath {
        return false;
    }
    let env = load_core();
    env.exec(script).expect("native Wrath achievement lazy-load consumer should succeed");
    true
}

const _: [fn(&str) -> bool; 2] = [assert_native, assert_lazy];

fn load_core() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("create Wrath achievement environment");
    env.set_screen_size(1024.0, 768.0);
    env.state().borrow_mut().addon_base_paths = vec![super::blizzard_ui_dir()];
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc()
        .expect("native Wrath FrameXML TOC should exist");
    load_addon(&env.loader_env(), &toc).expect("native Wrath FrameXML should load in TOC order");
    env
}
