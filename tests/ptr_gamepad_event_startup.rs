#![cfg(feature = "client-ptr")]

use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen_with_overrides, load_addon};
use wow_ui_sim::screen::ScreenKind;

#[test]
fn ptr_frame_controls_registers_global_region_mouse_down_during_original_load() {
    let ui = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().expect("PTR cache");
    let env = crate::common::blizzard_addon_harness::new_blizzard_addon_env(&ui);
    let closure = discover_blizzard_addon_closure_for_screen_with_overrides(
        &ui,
        ScreenKind::Game,
        &["Blizzard_GamepadSharedUtility"],
        &[],
    );
    assert!(
        closure
            .iter()
            .any(|(name, _)| name == "Blizzard_GamepadSharedUtility")
    );
    for (_, toc) in closure {
        load_addon(&env.loader_env(), &toc).expect("cached TOC must load");
    }
    env.exec("assert(EventRegistry.frameEventFrame:IsEventRegistered('GLOBAL_REGION_MOUSE_DOWN'))")
        .unwrap();
}
