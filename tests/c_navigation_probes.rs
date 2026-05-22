use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn navigation_fallbacks_return_safe_empty_defaults() {
    let env = env();
    let (
        was_clamped,
        target_state,
        has_screen_position,
        distance,
        nearest_token_is_nil,
        frame_is_nil,
    ): (bool, i32, bool, i32, bool, bool) = env
        .eval(
            r#"
            return
                C_Navigation.WasClampedToScreen(),
                C_Navigation.GetTargetState(),
                C_Navigation.HasValidScreenPosition(),
                C_Navigation.GetDistance(),
                C_Navigation.GetNearestPartyMemberToken() == nil,
                C_Navigation.GetFrame() == nil
            "#,
        )
        .unwrap();

    assert!(!was_clamped);
    assert_eq!(target_state, 0);
    assert!(!has_screen_position);
    assert_eq!(distance, 0);
    assert!(nearest_token_is_nil);
    assert!(frame_is_nil);
}
