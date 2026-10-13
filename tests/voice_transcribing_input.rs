//! Prospective explicit-input contract. Query producer intentionally not supplied.
//! Generated integration module; behavioral RED/GREEN remain unrun.

#![cfg(feature = "retail-12-0-5")]

use wow_ui_sim::c_api::c_voice_chat_channels::VoiceChannel;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn transcription_default_is_one_false_boolean() {
    let env = WowLuaEnv::new().unwrap();
    assert!(!env.state().borrow().voice_chat.is_transcribing);
    env.exec(
        r#"
        assert(type(C_VoiceChat.IsTranscribing) == 'function')
        assert(select('#', C_VoiceChat.IsTranscribing()) == 1)
        assert(type(C_VoiceChat.IsTranscribing()) == 'boolean')
        assert(C_VoiceChat.IsTranscribing() == false)
        "#,
    )
    .unwrap();
}

#[test]
fn retained_transcription_query_reads_live_environment_input() {
    let first = WowLuaEnv::new().unwrap();
    let second = WowLuaEnv::new().unwrap();
    for env in [&first, &second] {
        env.exec(
            r#"
            retained_transcription_query = C_VoiceChat.IsTranscribing
            retained_transcription_value = retained_transcription_query()
            assert(retained_transcription_value == false)
            "#,
        )
        .unwrap();
    }

    first.state().borrow_mut().voice_chat.is_transcribing = true;
    first
        .exec(
            r#"
            assert(retained_transcription_query() == true)
            assert(C_VoiceChat.IsTranscribing() == true)
            assert(retained_transcription_value == false)
            assert(select('#', retained_transcription_query()) == 1)
            assert(type(retained_transcription_query()) == 'boolean')
            "#,
        )
        .unwrap();
    second
        .exec("assert(retained_transcription_query() == false)")
        .unwrap();

    second.state().borrow_mut().voice_chat.is_transcribing = true;
    first.state().borrow_mut().voice_chat.is_transcribing = false;
    first
        .exec("assert(retained_transcription_query() == false)")
        .unwrap();
    second
        .exec("assert(retained_transcription_query() == true)")
        .unwrap();
}

#[test]
fn transcription_input_is_independent_of_permission_and_channel_snapshots() {
    let env = WowLuaEnv::new().unwrap();
    {
        let mut state = env.state().borrow_mut();
        state.voice_chat.is_transcribing = true;
        state.voice_channels.push(VoiceChannel {
            name: "Explicit Party Snapshot".into(),
            channel_id: 71,
            channel_type: 1,
            club_id: "9007199254740993".into(),
            stream_id: "20".into(),
            volume: 0.75,
            is_active: true,
            is_muted: false,
            is_transmitting: false,
            is_transcribing: false,
            members: Vec::new(),
        });
    }
    env.exec(
        r#"
        assert(C_VoiceChat.IsTranscriptionAllowed() == false)
        assert(C_VoiceChat.IsTranscribing() == true)
        retained_channel_snapshot = C_VoiceChat.GetChannelForChannelType(1)
        assert(retained_channel_snapshot.isTranscribing == false)
        retained_channel_snapshot.isTranscribing = true
        assert(C_VoiceChat.GetChannelForChannelType(1).isTranscribing == false)
        assert(C_VoiceChat.IsTranscribing() == true)
        "#,
    )
    .unwrap();

    {
        let mut state = env.state().borrow_mut();
        state.voice_chat.is_transcribing = false;
        state.voice_channels[0].is_transcribing = true;
    }
    env.exec(
        r#"
        assert(C_VoiceChat.IsTranscriptionAllowed() == false)
        assert(C_VoiceChat.GetChannelForChannelType(1).isTranscribing == true)
        assert(C_VoiceChat.GetChannelForCommunityStream(
            '9007199254740993', '20').isTranscribing == true)
        assert(C_VoiceChat.IsTranscribing() == false)
        retained_channel_snapshot.isTranscribing = false
        assert(C_VoiceChat.GetChannelForChannelType(1).isTranscribing == true)
        assert(C_VoiceChat.IsTranscribing() == false)
        "#,
    )
    .unwrap();

    env.state().borrow_mut().voice_channels.clear();
    env.exec("assert(C_VoiceChat.IsTranscribing() == false)")
        .unwrap();
    {
        let mut state = env.state().borrow_mut();
        state.voice_chat.channels.clear();
        state.voice_chat.active_channel_id = None;
        state.voice_chat.connection_status = 0;
        state.voice_chat.is_transcribing = true;
    }
    env.exec(
        r#"
        assert(#C_VoiceChat.GetChannels() == 0)
        assert(C_VoiceChat.GetActiveChannelID() == nil)
        assert(C_VoiceChat.IsTranscriptionAllowed() == false)
        assert(C_VoiceChat.IsTranscribing() == true)
        "#,
    )
    .unwrap();
}
