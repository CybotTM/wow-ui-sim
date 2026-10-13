//! Query-only profile contract; native activity producers/defaults remain unknown.

#![cfg(any(feature = "client-mists", feature = "client-wowforever"))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn documented_profile_query_returns_one_false_boolean() {
    let env = WowLuaEnv::new().unwrap();
    env.state().borrow_mut().voice_chat.is_speak_for_me_active = false;
    env.exec(
        r#"
        local value = C_VoiceChat.IsSpeakForMeActive()
        assert(select('#', C_VoiceChat.IsSpeakForMeActive()) == 1, 'activity arity')
        assert(type(value) == 'boolean', 'activity must be a boolean, got ' .. type(value))
        assert(value == false, 'supplied false activity')
        "#,
    )
    .unwrap();
}

#[test]
fn documented_profile_retained_query_reads_live_isolated_activity() {
    let first = WowLuaEnv::new().unwrap();
    let second = WowLuaEnv::new().unwrap();
    first
        .exec("retained_activity_query = C_VoiceChat.IsSpeakForMeActive")
        .unwrap();
    first.state().borrow_mut().voice_chat.is_speak_for_me_active = true;
    first
        .exec(
            r#"
            local value = retained_activity_query()
            assert(select('#', retained_activity_query()) == 1, 'retained activity arity')
            assert(type(value) == 'boolean', 'retained activity must be a boolean, got ' .. type(value))
            assert(value == true, 'supplied true activity')
            "#,
        )
        .unwrap();
    second
        .exec("assert(C_VoiceChat.IsSpeakForMeActive() == false, 'isolated second environment')")
        .unwrap();
    first.state().borrow_mut().voice_chat.is_speak_for_me_active = false;
    first
        .exec("assert(retained_activity_query() == false, 'live false restoration')")
        .unwrap();
}
