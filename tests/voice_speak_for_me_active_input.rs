//! Explicit supplied-input contract; no native default/event/transport claim.

#![cfg(feature = "retail-12-0-5")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn speak_for_me_active_is_one_false_boolean_by_default() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(type(C_VoiceChat.IsSpeakForMeActive) == 'function', 'missing Speak For Me query')
        assert(select('#', C_VoiceChat.IsSpeakForMeActive()) == 1)
        assert(type(C_VoiceChat.IsSpeakForMeActive()) == 'boolean')
        assert(C_VoiceChat.IsSpeakForMeActive() == false)
        "#,
    )
    .unwrap();
}

#[test]
fn retained_speak_for_me_query_reads_live_independent_environment_input() {
    let first = WowLuaEnv::new().unwrap();
    let second = WowLuaEnv::new().unwrap();
    for env in [&first, &second] {
        env.exec(
            r#"
            assert(type(C_VoiceChat.IsSpeakForMeActive) == 'function', 'missing Speak For Me query')
            retained_speak_query = C_VoiceChat.IsSpeakForMeActive
            retained_speak_value = retained_speak_query()
            assert(retained_speak_value == false)
            "#,
        )
        .unwrap();
    }

    first.state().borrow_mut().voice_chat.is_speak_for_me_active = true;
    first
        .exec(
            r#"
            assert(retained_speak_query() == true)
            assert(C_VoiceChat.IsSpeakForMeActive() == true)
            assert(retained_speak_value == false)
            assert(select('#', retained_speak_query()) == 1)
            assert(type(retained_speak_query()) == 'boolean')
            assert(C_VoiceChat.IsTranscribing() == false)
            assert(C_VoiceChat.IsTranscriptionAllowed() == false)
            "#,
        )
        .unwrap();
    second
        .exec("assert(retained_speak_query() == false)")
        .unwrap();

    second
        .state()
        .borrow_mut()
        .voice_chat
        .is_speak_for_me_active = true;
    {
        let mut state = first.state().borrow_mut();
        state.voice_chat.is_speak_for_me_active = false;
        state.voice_chat.is_transcribing = true;
        state.voice_chat.channels.clear();
        state.voice_channels.clear();
        state.voice_chat.active_channel_id = None;
        state.voice_chat.connection_status = 0;
    }
    first
        .exec(
            r#"
            assert(retained_speak_query() == false)
            assert(C_VoiceChat.IsTranscribing() == true)
            assert(#C_VoiceChat.GetChannels() == 0)
            assert(C_VoiceChat.GetActiveChannelID() == nil)
            "#,
        )
        .unwrap();
    second
        .exec("assert(retained_speak_query() == true)")
        .unwrap();
}
