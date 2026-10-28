#![cfg(feature = "client-ptr")]

use wow_ui_sim::c_api::c_game_pad::{MappedStick, MappedStickSnapshot};
use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen_with_overrides, load_addon};
use wow_ui_sim::screen::ScreenKind;

fn assert_initializer(sticks: &[(&str, f64, f64)], left: bool, right: bool) {
    let ui = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().expect("PTR cache");
    let env = crate::common::blizzard_addon_harness::new_blizzard_addon_env(&ui);
    env.state().borrow_mut().gamepad_mapped_sticks = Some(MappedStickSnapshot {
        sticks: sticks
            .iter()
            .map(|&(name, x, y)| MappedStick {
                config_name: name.into(),
                x,
                y,
            })
            .collect(),
    });
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
    env.exec(&format!(
        "assert(inputBindingAxisListener.left.isCentered == {left}); assert(inputBindingAxisListener.right.isCentered == {right})"
    )).unwrap();
}

#[test]
fn ptr_movement_uncentered_camera_centered_at_original_load() {
    assert_initializer(&[("Movement", 0.3, 0.4), ("Camera", 0.0, 0.0)], false, true);
}

#[test]
fn ptr_reordered_camera_uncentered_movement_centered_at_original_load() {
    assert_initializer(
        &[("Camera", 0.0, -0.75), ("Movement", 0.0, 0.0)],
        true,
        false,
    );
}
