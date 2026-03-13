use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

#[test]
fn login_screen_updates_glue_login_state() {
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_mode(ScreenKind::Login);

    assert!(env.eval::<bool>("return InGlue()").unwrap());
    assert!(env.eval::<bool>("return C_Glue.IsOnGlueScreen()").unwrap());
    assert!(!env.eval::<bool>("return IsLoggedIn()").unwrap());

    let (aurora_state, connected_to_wow, wow_connection_state, has_realm_list): (i32, bool, i32, bool) =
        env.eval("return C_Login.GetState()").unwrap();
    let expected_aurora_state: i32 = env.eval("return LE_AURORA_STATE_NONE").unwrap();
    assert_eq!(aurora_state, expected_aurora_state);
    assert!(!connected_to_wow);
    assert_eq!(wow_connection_state, 0);
    assert!(!has_realm_list);
}

#[test]
fn character_select_screen_updates_glue_login_state() {
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_mode(ScreenKind::CharacterSelect);

    assert!(env.eval::<bool>("return InGlue()").unwrap());
    assert!(env.eval::<bool>("return C_Glue.IsOnGlueScreen()").unwrap());
    assert!(!env.eval::<bool>("return IsLoggedIn()").unwrap());

    let (_aurora_state, connected_to_wow, wow_connection_state, has_realm_list): (i32, bool, i32, bool) =
        env.eval("return C_Login.GetState()").unwrap();
    assert!(connected_to_wow);
    assert_eq!(wow_connection_state, 0);
    assert!(!has_realm_list);
}
