//! Temporary `C_TTSSettings` fallback surface.
//!
//! Text-to-speech user preferences are not modeled yet. These methods expose
//! stable default values while leaving channel-level TTS settings in the
//! chat/channel subsystem.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_tts_settings_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_TTSSettings")?;
    table_set_rust_fn_static(state, ns, "GetSpeechVolume", get_speech_volume)?;
    table_set_rust_fn_static(state, ns, "SetSpeechVolume", tts_settings_noop)?;
    table_set_rust_fn_static(state, ns, "GetSpeechRate", get_speech_rate)?;
    table_set_rust_fn_static(state, ns, "SetSpeechRate", tts_settings_noop)?;
    table_set_rust_fn_static(state, ns, "GetVoiceOptionID", get_voice_option_id)?;
    table_set_rust_fn_static(state, ns, "SetVoiceOptionID", tts_settings_noop)?;
    table_set_rust_fn_static(state, ns, "SetVoiceOption", tts_settings_noop)
}

fn get_speech_volume(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(100.0));
    Ok(1)
}

fn get_speech_rate(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_voice_option_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn tts_settings_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn tts_settings_defaults_are_callable() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let (volume, rate, voice_option): (i32, i32, i32) = env
            .eval(
                r#"
                C_TTSSettings.SetSpeechVolume(50)
                C_TTSSettings.SetSpeechRate(5)
                C_TTSSettings.SetVoiceOptionID(1, 2)
                C_TTSSettings.SetVoiceOption(1, 2)
                return C_TTSSettings.GetSpeechVolume(),
                    C_TTSSettings.GetSpeechRate(),
                    C_TTSSettings.GetVoiceOptionID(1)
                "#,
            )
            .expect("tts settings defaults should be callable");

        assert_eq!(volume, 100);
        assert_eq!(rate, 0);
        assert_eq!(voice_option, 0);
    }
}
