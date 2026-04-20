use std::path::Path;

use wow_ui_sim::loader::{
    BlizzardAddonOverride, discover_blizzard_addon_closure_for_screen_with_overrides, load_addon,
};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

use super::panel_fixtures::blizzard_ui_dir;

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
    let env = WowLuaEnv::new().expect("failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    let ui = ui_dir.to_path_buf();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui];
    }

    let loaded = load_blizzard_addon_closure_into_env(&env, ui_dir, roots, overrides);
    (env, loaded)
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
