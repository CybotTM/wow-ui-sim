use std::path::PathBuf;

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

pub fn new_game_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    let ui = blizzard_ui_dir();
    env.state().borrow_mut().addon_base_paths = vec![ui];
    env
}

pub fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}
