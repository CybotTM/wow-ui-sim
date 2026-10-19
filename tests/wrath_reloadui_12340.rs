//! Clean-room 12340 lifecycle boundaries. Original cached Blizzard code is unchanged.
#![cfg(feature = "client-wrath")]

#[path = "common/wrath_full_ui.rs"]
mod original;

#[path = "common/wrath_reload_shutdown.rs"]
mod shutdown;

#[path = "common/wrath_reload_saved_variables.rs"]
mod saved_variables;

#[path = "common/wrath_reload_startup.rs"]
mod startup;

#[path = "common/wrath_reload_preservation.rs"]
mod preservation;

use wow_ui_sim::lua_api::state::AddonInfo;

#[test]
fn original_world_map_restores_only_eligible_named_placements() {
    preservation::assert_placed_frames();
}

#[test]
fn original_mirror_bars_cancel_but_totems_and_player_survive() {
    preservation::assert_selective_timers();
}

#[test]
fn original_ui_parent_cached_readiness_and_argless_wrath_entry() {
    startup::assert_startup_readiness([true, true]);
}

#[test]
fn original_ui_parent_waits_for_each_account_chunk() {
    for ready in [[false, true], [true, false], [false, false]] {
        startup::assert_startup_readiness(ready);
    }
}

#[test]
fn saved_scope_files_execute_once_and_override_scalar_and_nonempty_defaults() {
    saved_variables::assert_scope_execution_order();
}

#[test]
fn original_stopwatch_restores_saved_variables_after_code_before_event() {
    saved_variables::assert_original_restore_after_code();
}

#[test]
fn original_logout_handlers_persist_after_in_world_combat_shutdown() {
    shutdown::assert_shutdown(true, true);
}

#[test]
fn out_of_world_shutdown_omits_leaving_without_rejecting_logout() {
    shutdown::assert_shutdown(false, false);
}

#[test]
fn original_forbidden_popup_defers_reload_through_cinematic() {
    let mut env = original::load_original_wrath_startup();
    env.state().borrow_mut().addons.push(AddonInfo {
        folder_name: "ReloadDisposable".into(),
        enabled: true,
        ..Default::default()
    });
    env.state().borrow_mut().cinematic_active = true;
    env.exec(
        "OldReloadSentinel = {}; \
         StaticPopupDialogs.ADDON_ACTION_FORBIDDEN.OnAccept({}, 'ReloadDisposable'); \
         assert(OldReloadSentinel ~= nil, 'popup returns in old VM')",
    )
    .unwrap();
    assert!(env.state().borrow().ui_reload_requested);
    assert!(!env.state().borrow().addons.last().unwrap().enabled);
    assert!(
        env.state()
            .borrow()
            .addons
            .iter()
            .any(|addon| addon.enabled)
    );
    assert!(
        !env.service_reload(&mut None).unwrap(),
        "12340 Idle must keep the request pending while cinematic playback is active"
    );
    assert!(env.state().borrow().ui_reload_requested);
    assert!(env.eval::<bool>("return OldReloadSentinel ~= nil").unwrap());
    env.state().borrow_mut().cinematic_active = false;
    assert!(env.service_reload(&mut None).unwrap());
    assert!(env.eval::<bool>("return OldReloadSentinel == nil").unwrap());
}

#[test]
fn request_returns_zero_values_coalesces_and_does_not_reject_combat() {
    let mut env = wow_ui_sim::lua_api::WowLuaEnv::new().unwrap();
    env.state().borrow_mut().player.in_combat = true;
    env.exec("assert(select('#', ReloadUI()) == 0); ReloadUI(); OldReloadSentinel = true")
        .unwrap();
    assert!(env.state().borrow().ui_reload_requested);
    assert!(
        env.eval::<bool>("return OldReloadSentinel == true")
            .unwrap()
    );
    assert!(env.service_reload(&mut None).unwrap());
    assert!(!env.service_reload(&mut None).unwrap());
}
