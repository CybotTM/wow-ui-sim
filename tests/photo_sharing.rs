//! `C_PhotoSharing.IsAuthorized` / `IsEnabled` — SimState-backed round-trip.

use wow_ui_sim::lua_api::WowLuaEnv;

fn probes(env: &WowLuaEnv) -> (bool, bool) {
    env.eval(
        r#"
        return C_PhotoSharing.IsAuthorized(), C_PhotoSharing.IsEnabled()
        "#,
    )
    .unwrap()
}

#[test]
fn defaults_both_false() {
    let env = WowLuaEnv::new().unwrap();
    let (auth, enabled) = probes(&env);
    assert!(!auth);
    assert!(!enabled);
}

#[test]
fn admin_drives_authorized_flag() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPhotoSharingAuthorized(true)").unwrap();
    let (auth, enabled) = probes(&env);
    assert!(auth);
    assert!(!enabled, "enabled axis is independent");
}

#[test]
fn admin_drives_enabled_flag() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPhotoSharingEnabled(true)").unwrap();
    let (auth, enabled) = probes(&env);
    assert!(!auth, "authorized axis is independent");
    assert!(enabled);
}

#[test]
fn both_can_be_true_together() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPhotoSharingAuthorized(true)").unwrap();
    env.exec("A_Admin.SetPhotoSharingEnabled(true)").unwrap();
    assert_eq!(probes(&env), (true, true));
}

#[test]
fn no_arg_setters_default_to_true() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPhotoSharingAuthorized()").unwrap();
    env.exec("A_Admin.SetPhotoSharingEnabled()").unwrap();
    assert_eq!(probes(&env), (true, true));
}

#[test]
fn admin_can_toggle_back_to_false() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPhotoSharingAuthorized(true)").unwrap();
    env.exec("A_Admin.SetPhotoSharingAuthorized(false)").unwrap();
    assert_eq!(probes(&env).0, false);
}
