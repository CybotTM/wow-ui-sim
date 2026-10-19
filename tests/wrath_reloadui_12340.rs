//! Clean-room 12340 lifecycle boundaries. Original cached Blizzard code is unchanged.
#![cfg(feature = "client-wrath")]

#[path = "common/wrath_full_ui.rs"]
mod original;

use wow_ui_sim::lua_api::state::AddonInfo;

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
