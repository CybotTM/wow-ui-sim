//! Temporary `C_TTSSettings` user-preference defaults.
//!
//! Text-to-speech user preferences are not modeled yet. Channel-level TTS state
//! stays owned by the chat/channel subsystem; these defaults only fill the
//! generic speech preference methods until a real settings backend exists.

const TTS_SETTINGS_DEFAULTS_LUA: &str = r#"
C_TTSSettings = C_TTSSettings or __wow_namespace()

local function installTtsSettingsDefault(name, fn)
    if rawget(C_TTSSettings, name) == nil then
        C_TTSSettings[name] = fn
    end
end

installTtsSettingsDefault("GetSpeechVolume", function()
    return 100
end)

installTtsSettingsDefault("SetSpeechVolume", function()
end)

installTtsSettingsDefault("GetSpeechRate", function()
    return 0
end)

installTtsSettingsDefault("SetSpeechRate", function()
end)

installTtsSettingsDefault("GetVoiceOptionID", function()
    return 0
end)

installTtsSettingsDefault("SetVoiceOptionID", function()
end)

installTtsSettingsDefault("SetVoiceOption", function()
end)
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(TTS_SETTINGS_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_tts_settings_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (i32, i32, i32) = env
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

        assert_eq!(result, (100, 0, 0));
    }

    #[test]
    fn preserves_existing_tts_settings_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_TTSSettings.GetSpeechVolume()
                return 25
            end
            "#,
        )
        .expect("fixture should install existing tts settings provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let volume: i32 = env
            .eval("return C_TTSSettings.GetSpeechVolume()")
            .expect("existing tts settings provider should remain callable");

        assert_eq!(volume, 25);
    }
}
