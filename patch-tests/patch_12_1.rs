use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

#[path = "patch_12_1/player_choice.rs"]
mod player_choice;
#[path = "patch_12_1/ptr_feedback.rs"]
mod ptr_feedback;
#[path = "patch_12_1/shake.rs"]
mod shake;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path()
        .expect("Blizzard UI cache should be available")
}

fn player_choice_toc() -> PathBuf {
    blizzard_ui_dir()
        .join("Blizzard_PlayerChoice")
        .join("Blizzard_PlayerChoice.toc")
}

fn load_game_ui_without_player_choice() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];
    wow_ui_sim::xml::register_intrinsic_templates();

    for (name, toc_path) in
        discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game)
    {
        load_addon(&env.loader_env(), &toc_path)
            .unwrap_or_else(|error| panic!("[load {name}] FAILED: {error}"));
    }

    env
}
