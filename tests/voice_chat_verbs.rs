//! Integration tests for `src/lua_api/globals/voice_chat_verbs.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

fn fired(env: &WowLuaEnv, name: &str) -> bool {
    env.state()
        .borrow()
        .events
        .pending()
        .iter()
        .any(|e| e.name == name)
}

// ── Defaults ──────────────────────────────────────────────────────────────────

#[test]
fn voice_chat_defaults_are_full_volume_unmuted_no_headset() {
    let env = env();
    let st = env.state().borrow();
    assert!((st.voice_chat.microphone_volume - 1.0).abs() < 1e-6);
    assert!((st.voice_chat.output_volume - 1.0).abs() < 1e-6);
    assert!(!st.voice_chat.muted);
    assert!(!st.voice_chat.deafened);
    assert!(!st.voice_chat.headset_mode);
}

// ── GetMicrophoneVolume ───────────────────────────────────────────────────────

#[test]
fn get_microphone_volume_reads_current_value() {
    let env = env();
    env.state().borrow_mut().voice_chat.microphone_volume = 0.42;
    let v: f64 = env.eval("return VoiceChat_GetMicrophoneVolume()").unwrap();
    assert!((v - 0.42).abs() < 1e-5);
}

// ── SetMicrophoneVolume ───────────────────────────────────────────────────────

#[test]
fn set_microphone_volume_stores_and_fires_event() {
    let env = env();
    env.exec("VoiceChat_SetMicrophoneVolume(0.5)").unwrap();
    assert!((env.state().borrow().voice_chat.microphone_volume - 0.5).abs() < 1e-5);
    assert!(fired(&env, "VOICE_CHAT_MICROPHONE_VOLUME_CHANGED"));
}

#[test]
fn set_microphone_volume_clamps_out_of_range() {
    let env = env();
    env.exec(
        "VoiceChat_SetMicrophoneVolume(2.0)
               VoiceChat_SetMicrophoneVolume(-1.0)",
    )
    .unwrap();
    let v = env.state().borrow().voice_chat.microphone_volume;
    assert!((v - 0.0).abs() < 1e-6, "negative input must clamp to 0");
}

// ── SetOutputVolume ───────────────────────────────────────────────────────────

#[test]
fn set_output_volume_stores_and_fires_event() {
    let env = env();
    env.exec("VoiceChat_SetOutputVolume(0.25)").unwrap();
    assert!((env.state().borrow().voice_chat.output_volume - 0.25).abs() < 1e-5);
    assert!(fired(&env, "VOICE_CHAT_OUTPUT_VOLUME_CHANGED"));
}

// ── VoiceChatHeadsetModeCheck ─────────────────────────────────────────────────

#[test]
fn headset_mode_check_flags_state_and_fires_event() {
    let env = env();
    env.exec("VoiceChatHeadsetModeCheck()").unwrap();
    assert!(env.state().borrow().voice_chat.headset_mode);
    assert!(fired(&env, "VOICE_CHAT_HEADSET_MODE_CHANGED"));
}
