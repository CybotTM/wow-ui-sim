#![cfg(feature = "client-ptr")]

use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen_with_overrides, load_addon};
use wow_ui_sim::screen::ScreenKind;

#[test]
fn ptr_frame_controls_refresh_stores_both_hover_flags() {
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
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
    env.exec(
        r#"
        local manager = CreateFromMixins(GamepadFrameControlsManagerMixin)
        manager.isUIFocused = true
        manager:RefreshGamepadCursorHovering()
        assert(C_GamePad.GetAllowHoverEventsWithFreeLook() == false)
        manager.isUIFocused = false
        manager:RefreshGamepadCursorHovering()
        assert(C_GamePad.GetAllowHoverEventsWithFreeLook() == true)
    "#,
    )
    .unwrap();
}
