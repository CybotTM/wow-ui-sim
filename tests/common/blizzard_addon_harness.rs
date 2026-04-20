use std::path::Path;

use wow_ui_sim::loader::{
    BlizzardAddonOverride, discover_blizzard_addon_closure_for_screen_with_overrides, load_addon,
};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

use super::panel_fixtures::{blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons};

#[allow(dead_code)]
pub fn load_blizzard_addon_closure_into_env(
    env: &WowLuaEnv,
    ui_dir: &Path,
    roots: &[&str],
    overrides: &[BlizzardAddonOverride<'_>],
) -> Vec<String> {
    let mut loaded = Vec::new();
    for (name, toc_path) in discover_blizzard_addon_closure_for_screen_with_overrides(
        ui_dir,
        ScreenKind::Game,
        roots,
        overrides,
    ) {
        if let Err(error) = load_addon(&env.loader_env(), &toc_path) {
            panic!("{name} should load in the Blizzard addon closure harness: {error}");
        }
        loaded.push(name);
    }
    loaded
}

#[allow(dead_code)]
pub fn build_blizzard_addon_closure_env(
    ui_dir: &Path,
    roots: &[&str],
    overrides: &[BlizzardAddonOverride<'_>],
) -> (WowLuaEnv, Vec<String>) {
    let env = new_blizzard_addon_env(ui_dir);
    let loaded = load_blizzard_addon_closure_into_env(&env, ui_dir, roots, overrides);
    (env, loaded)
}

#[allow(dead_code)]
pub fn new_blizzard_addon_env(ui_dir: &Path) -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    let ui = ui_dir.to_path_buf();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui];
    }

    env
}

#[allow(dead_code)]
pub fn with_blizzard_addon_closure_in_dir<R, F>(
    ui_dir: &Path,
    roots: &[&str],
    overrides: &[BlizzardAddonOverride<'_>],
    assertions: F,
) -> R
where
    F: FnOnce(&WowLuaEnv, &[String]) -> R,
{
    let (env, loaded) = build_blizzard_addon_closure_env(ui_dir, roots, overrides);
    assertions(&env, &loaded)
}

#[allow(dead_code)]
pub fn with_blizzard_addon_closure<R, F>(
    roots: &[&str],
    overrides: &[BlizzardAddonOverride<'_>],
    assertions: F,
) -> R
where
    F: FnOnce(&WowLuaEnv, &[String]) -> R,
{
    with_blizzard_addon_closure_in_dir(&blizzard_ui_dir(), roots, overrides, assertions)
}

#[allow(dead_code)]
pub fn with_blizzard_addon_startup_shape_in_dir<R, F>(
    ui_dir: &Path,
    roots: &[&str],
    overrides: &[BlizzardAddonOverride<'_>],
    assertions: F,
) -> R
where
    F: FnOnce(&WowLuaEnv, &[String]) -> R,
{
    let env = new_blizzard_addon_env(ui_dir);
    load_panel_addons(&env);
    {
        let mut state = env.state().borrow_mut();
        state.lua_errors.clear();
        state.lua_error_records.clear();
        state.lua_error_counts.clear();
    }
    let loaded = load_blizzard_addon_closure_into_env(&env, ui_dir, roots, overrides);
    ensure_player_frame_stub(&env);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    assertions(&env, &loaded)
}

#[allow(dead_code)]
pub fn with_blizzard_addon_startup_shape<R, F>(
    roots: &[&str],
    overrides: &[BlizzardAddonOverride<'_>],
    assertions: F,
) -> R
where
    F: FnOnce(&WowLuaEnv, &[String]) -> R,
{
    with_blizzard_addon_startup_shape_in_dir(&blizzard_ui_dir(), roots, overrides, assertions)
}

fn ensure_player_frame_stub(env: &WowLuaEnv) {
    env.exec(
        r#"
        if not PlayerFrame then
            PlayerFrame = CreateFrame("Frame", "PlayerFrame", UIParent)
        end
        PlayerFrame.unit = "player"
        "#,
    )
    .expect("failed to create PlayerFrame stub for startup smoke tests");
}

#[allow(dead_code)]
pub fn with_blizzard_addon_smoke_shape_in_dir<R, F>(
    ui_dir: &Path,
    roots: &[&str],
    overrides: &[BlizzardAddonOverride<'_>],
    assertions: F,
) -> R
where
    F: FnOnce(&WowLuaEnv, &[String]) -> R,
{
    let env = new_blizzard_addon_env(ui_dir);
    load_panel_addons(&env);
    clear_recorded_lua_errors(&env);
    let loaded = load_blizzard_addon_closure_into_env(&env, ui_dir, roots, overrides);
    assertions(&env, &loaded)
}

#[allow(dead_code)]
pub fn with_blizzard_addon_smoke_shape<R, F>(
    roots: &[&str],
    overrides: &[BlizzardAddonOverride<'_>],
    assertions: F,
) -> R
where
    F: FnOnce(&WowLuaEnv, &[String]) -> R,
{
    with_blizzard_addon_smoke_shape_in_dir(&blizzard_ui_dir(), roots, overrides, assertions)
}
