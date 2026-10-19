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

#[path = "common/wrath_reload_reentry.rs"]
mod reentry;

#[path = "common/wrath_reload_writer.rs"]
mod placement_writer;

#[path = "common/wrath_reload_entry_payload.rs"]
mod entry_payload;

#[path = "common/wrath_reload_persistence_controls.rs"]
mod persistence_controls;

use wow_ui_sim::lua_api::state::AddonInfo;

#[test]
fn saved_reload_preserves_scalar_large_values_and_readonly_wtf() {
    persistence_controls::assert_reload_readback();
}

#[test]
fn saved_guarded_defaults_preserve_nested_large_values_and_readonly_wtf() {
    persistence_controls::assert_guarded_reload_readback();
}

#[test]
fn wrath_startup_reload_pew_is_argless_in_fresh_original_ui() {
    entry_payload::assert_reload_pew_arity();
}

#[test]
fn wrath_startup_original_stopwatch_show_hide_survives_reload() {
    entry_payload::assert_original_stopwatch_reload_paths();
}

#[test]
fn wrath_preservation_writer_uses_authenticated_grammar_and_nearest_rounding() {
    placement_writer::assert_writer_grammar_and_rounding();
}

#[test]
fn wrath_preservation_disabled_persistence_writes_no_layout() {
    placement_writer::assert_disabled_persistence_writes_no_layout();
}

#[test]
fn wrath_preservation_writer_rewrites_an_empty_snapshot_header() {
    placement_writer::assert_empty_writer();
}

#[test]
fn wrath_shutdown_combat_reentry_receives_argless_regen_disabled() {
    reentry::assert_reentry(true);
}

#[test]
fn wrath_shutdown_noncombat_reentry_omits_regen_disabled() {
    reentry::assert_reentry(false);
}

#[test]
fn wrath_preservation_anchor_selection_uses_first_corresponding_point_on_ties() {
    preservation::assert_anchor_tie();
}

#[test]
fn wrath_preservation_reader_accepts_tolerant_records_and_resets_missing_fields() {
    preservation::assert_tolerant_reader();
}

#[test]
fn wrath_preservation_original_world_map_restores_only_eligible_named_placements() {
    preservation::assert_placed_frames();
}

#[test]
fn wrath_preservation_original_mirror_bars_cancel_but_totems_and_player_survive() {
    preservation::assert_selective_timers();
}

#[test]
fn wrath_startup_original_ui_parent_cached_readiness_and_argless_entry() {
    startup::assert_startup_readiness([true, true]);
}

#[test]
fn wrath_startup_original_ui_parent_waits_for_each_account_chunk() {
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
fn wrath_shutdown_original_logout_handlers_persist_after_combat_shutdown() {
    shutdown::assert_shutdown(true, true);
}

#[test]
fn wrath_shutdown_out_of_world_omits_leaving_without_rejecting_logout() {
    shutdown::assert_shutdown(false, false);
}

#[test]
fn original_forbidden_popup_disables_selected_addon_through_modeled_state() {
    let env = original::load_original_wrath_startup();
    env.state().borrow_mut().addons.push(AddonInfo {
        folder_name: "ReloadDisposable".into(),
        enabled: true,
        ..Default::default()
    });
    env.exec("OldReloadSentinel = {}; StaticPopupDialogs.ADDON_ACTION_FORBIDDEN.OnAccept({}, 'ReloadDisposable'); assert(OldReloadSentinel ~= nil)").unwrap();
    assert_eq!(
        env.eval::<i32>("return C_AddOns.GetAddOnEnableState('ReloadDisposable')")
            .unwrap(),
        0
    );
    assert!(env.state().borrow().ui_reload_requested);
    assert!(
        env.state()
            .borrow()
            .addons
            .iter()
            .any(|addon| addon.enabled)
    );
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
    assert_eq!(
        env.eval::<i32>("return C_AddOns.GetAddOnEnableState('ReloadDisposable')")
            .unwrap(),
        0
    );
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
