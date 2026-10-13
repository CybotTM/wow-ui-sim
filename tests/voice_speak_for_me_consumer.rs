//! Bounded cached edit-box consumer decision; not event/native/render proof.

#![cfg(all(feature = "gui", feature = "retail-12-0-5"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_no_runtime_errors(env: &WowLuaEnv, stage: &str) {
    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        errors.is_empty(),
        "{stage}: full runtime errors: {errors:#?}"
    );
}

fn exec_consumer_step(env: &WowLuaEnv, stage: &str, lua: &str) {
    let result = env.exec(lua);
    assert_no_runtime_errors(env, stage);
    result.unwrap_or_else(|error| panic!("{stage}: {error}"));
}

fn supply_voice_inputs(env: &WowLuaEnv, active: bool, muted: bool) {
    let mut state = env.state().borrow_mut();
    state.voice_chat.is_speak_for_me_active = active;
    state.voice_chat.muted = muted;
}

#[test]
fn cached_voice_edit_box_consumes_live_speak_activity_and_muted_state() {
    let env = crate::chat_frame::setup_env();
    assert_no_runtime_errors(&env, "full cached constructor/startup");
    exec_consumer_step(
        &env,
        "cached consumer prerequisites",
        r#"
        assert(ChatFrame3, 'missing real ChatFrame3')
        assert(ChatFrame3.editBox, 'missing real voice edit box')
        assert(ChatFrame3.editBox.prompt, 'missing real voice prompt')
        assert(type(VoiceTranscriptionFrame_UpdateEditBox) == 'function',
            'missing cached edit-box consumer')
        assert(type(C_VoiceChat.IsSpeakForMeActive) == 'function',
            'missing Speak For Me query')
        "#,
    );

    supply_voice_inputs(&env, false, false);
    exec_consumer_step(
        &env,
        "inactive/unmuted consumer",
        r#"
        assert(C_VoiceChat.IsSpeakForMeActive() == false)
        assert(C_VoiceChat.IsMuted() == false)
        ChatFrame3.editBox:Show()
        assert(ChatFrame3.editBox:IsShown())
        VoiceTranscriptionFrame_UpdateEditBox(ChatFrame3)
        assert(ChatFrame3.editBox.disableActivate == true)
        assert(ChatFrame3.editBox.prompt:GetText() == '')
        assert(not ChatFrame3.editBox:IsShown())
        "#,
    );

    supply_voice_inputs(&env, true, false);
    exec_consumer_step(
        &env,
        "active/unmuted consumer",
        r#"
        assert(C_VoiceChat.IsSpeakForMeActive() == true)
        assert(C_VoiceChat.IsMuted() == false)
        VoiceTranscriptionFrame_UpdateEditBox(ChatFrame3)
        assert(ChatFrame3.editBox.disableActivate == false)
        assert(ChatFrame3.editBox.prompt:GetText() == REMOTE_TEXT_TO_SPEECH)
        "#,
    );

    supply_voice_inputs(&env, true, true);
    exec_consumer_step(
        &env,
        "active/muted consumer",
        r#"
        assert(C_VoiceChat.IsSpeakForMeActive() == true)
        assert(C_VoiceChat.IsMuted() == true)
        VoiceTranscriptionFrame_UpdateEditBox(ChatFrame3)
        assert(C_VoiceChat.IsSpeakForMeActive() == true)
        assert(ChatFrame3.editBox.disableActivate == true)
        assert(ChatFrame3.editBox.prompt:GetText() == VOICE_TRANSCRIPTION_MUTED)
        local r, g, b = ChatFrame3.editBox.prompt:GetTextColor()
        local expectedR, expectedG, expectedB = RED_FONT_COLOR:GetRGB()
        assert(math.abs(r - expectedR) < 0.000001)
        assert(math.abs(g - expectedG) < 0.000001)
        assert(math.abs(b - expectedB) < 0.000001)
        "#,
    );

    supply_voice_inputs(&env, false, false);
    exec_consumer_step(
        &env,
        "inactive/unmuted restoration",
        r#"
        assert(C_VoiceChat.IsSpeakForMeActive() == false)
        assert(C_VoiceChat.IsMuted() == false)
        ChatFrame3.editBox:Show()
        assert(ChatFrame3.editBox:IsShown())
        VoiceTranscriptionFrame_UpdateEditBox(ChatFrame3)
        assert(ChatFrame3.editBox.disableActivate == true)
        assert(ChatFrame3.editBox.prompt:GetText() == '')
        assert(not ChatFrame3.editBox:IsShown())
        "#,
    );
}
