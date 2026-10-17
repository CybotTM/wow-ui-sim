//! Separate marker-state root exposed after the R12 capability producer.
#![cfg(any(feature = "client-era", feature = "client-anniversary"))]

use wow_ui_sim::loader::{discover_blizzard_startup_addons_for_screen, load_startup_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

#[test]
fn classic_b_marker_state_original_startup_evaluates_selection() {
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    wow_ui_sim::xml::register_intrinsic_templates();
    for addon in discover_blizzard_startup_addons_for_screen(&ui, ScreenKind::Game) {
        load_startup_addon(&env.loader_env(), &addon.toc_path, addon.kind, None)
            .unwrap_or_else(|error| panic!("load {}: {error}", addon.name));
    }
    env.exec(
        "assert(C_AddOns.IsAddOnLoaded('Blizzard_CompactRaidFrames')); \
         assert(IsRaidMarkerSystemEnabled()); \
         assert(CompactRaidFrameManager.displayFrame.leaderOptions.raidMarkerDropdown)",
    )
    .expect("original startup reaches the enabled real raid-marker dropdown");
    let marker_errors: Vec<_> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|error| error.contains("Blizzard_CompactRaidFrameManager.lua"))
        .cloned()
        .collect();
    assert!(
        marker_errors.is_empty(),
        "original enabled dropdown must evaluate its selection callback without error: {marker_errors:?}"
    );
    env.exec("for i=1,8 do assert(IsRaidMarkerActive(i) == false) end")
        .expect("default marker state is explicitly empty simulator policy");
    env.state()
        .borrow_mut()
        .plain_global_inputs
        .active_raid_markers
        .insert(3);
    env.exec("assert(IsRaidMarkerActive(3)); assert(not IsRaidMarkerActive(1)); CompactRaidFrameManager_SetupRaidMarkerDropdown(CompactRaidFrameManager)")
        .expect("original dropdown selection uses the supplied active marker without placement calls");
}
