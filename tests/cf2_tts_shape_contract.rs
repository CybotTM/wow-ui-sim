//! Original TTS preference round trips plus documented shapes; runtime proof deferred.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]

#[path = "common/cf2_shape.rs"]
mod shape;

#[path = "common/cf2_cvar_storage.rs"]
mod storage;

fn assert_original_tts_preference_round_trips(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    shape::check(
        env,
        r#"
original(TextToSpeech_SetRate, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
original(TextToSpeech_SetVolume, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
original(TextToSpeechFrame_UpdateSliders, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
local controls = TextToSpeechFrame.PanelContainer
original(controls.AdjustRateSlider.Slider:GetScript('OnValueChanged'), 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
original(controls.AdjustVolumeSlider.Slider:GetScript('OnValueChanged'), 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
for _, pair in ipairs({{4, 37}, {7, 63}}) do
    assert(TextToSpeech_SetRate(TextToSpeechFrame, pair[1]))
    assert(TextToSpeech_SetVolume(TextToSpeechFrame, pair[2]))
    assert(C_TTSSettings.GetSpeechRate() == pair[1], 'original rate callback must store its input')
    assert(C_TTSSettings.GetSpeechVolume() == pair[2], 'original volume callback must store its input')
    TextToSpeechFrame_UpdateSliders(TextToSpeechFrame)
    assert(controls.AdjustRateSlider.Slider:GetValue() == pair[1])
    assert(controls.AdjustVolumeSlider.Slider:GetValue() == pair[2])
end
assert(not TextToSpeech_SetRate(TextToSpeechFrame, -11))
assert(not TextToSpeech_SetRate(TextToSpeechFrame, 11))
assert(not TextToSpeech_SetVolume(TextToSpeechFrame, -1))
assert(not TextToSpeech_SetVolume(TextToSpeechFrame, 101))
assert(C_TTSSettings.GetSpeechRate() == 7)
assert(C_TTSSettings.GetSpeechVolume() == 63)
"#,
    );
}

// retail: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:65 (GetSpeechRate).
// retail: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:74 (GetSpeechVolume).
// retail: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:165 (SetSpeechRate).
// retail: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:175 (SetSpeechVolume).
// ptr: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:65 (GetSpeechRate).
// ptr: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:74 (GetSpeechVolume).
// ptr: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:165 (SetSpeechRate).
// ptr: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:175 (SetSpeechVolume).
// wowforever: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:65 (GetSpeechRate).
// wowforever: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:74 (GetSpeechVolume).
// wowforever: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:165 (SetSpeechRate).
// wowforever: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:175 (SetSpeechVolume).
// mists: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:62 (GetSpeechRate).
// mists: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:71 (GetSpeechVolume).
// mists: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:156 (SetSpeechRate).
// mists: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:165 (SetSpeechVolume).
// era: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:62 (GetSpeechRate).
// era: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:71 (GetSpeechVolume).
// era: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:156 (SetSpeechRate).
// era: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:165 (SetSpeechVolume).
// anniversary: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:62 (GetSpeechRate).
// anniversary: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:71 (GetSpeechVolume).
// anniversary: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:156 (SetSpeechRate).
// anniversary: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:165 (SetSpeechVolume).
// Source-predicted PASS: numeric getters and void setters already shape-compatible; original sliders only, no round-trip values; not executed; native semantics unknown.
#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]
#[test]
fn original_tts_numeric_preferences_contract() {
    storage::run_isolated(
        "cf2_tts_shape_contract::original_tts_numeric_preferences_contract",
        || {
            let env = shape::load(&["Blizzard_ChatFrame"]);
            assert_original_tts_preference_round_trips(&env);
            shape::check(
                &env,
                r#"

local rate = scalar('number', C_TTSSettings.GetSpeechRate())
local volume = scalar('number', C_TTSSettings.GetSpeechVolume())
assert(select('#', C_TTSSettings.SetSpeechRate(rate)) == 0)
assert(select('#', C_TTSSettings.SetSpeechVolume(volume)) == 0)
original(TextToSpeechFrameAdjustRateSlider_OnLoad, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
original(TextToSpeechFrameAdjustVolumeSlider_OnLoad, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
local container = TextToSpeechFrame.PanelContainer
TextToSpeechFrameAdjustRateSlider_OnLoad(container.AdjustRateSlider)
TextToSpeechFrameAdjustVolumeSlider_OnLoad(container.AdjustVolumeSlider)
local rateSlider = container.AdjustRateSlider.Slider
local volumeSlider = container.AdjustVolumeSlider.Slider
local rateCallback = rateSlider:GetScript('OnValueChanged')
local volumeCallback = volumeSlider:GetScript('OnValueChanged')
original(rateCallback, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
original(volumeCallback, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
rateCallback(rateSlider, rate)
volumeCallback(volumeSlider, volume)
TextToSpeechFrame_UpdateSliders(TextToSpeechFrame)

"#,
            );
        },
    );
}

// retail: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:26 (GetCharacterSettingsSaved).
// retail: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:113 (MarkCharacterSettingsSaved).
// ptr: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:26 (GetCharacterSettingsSaved).
// ptr: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:113 (MarkCharacterSettingsSaved).
// wowforever: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:26 (GetCharacterSettingsSaved).
// wowforever: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:113 (MarkCharacterSettingsSaved).
// mists: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:25 (GetCharacterSettingsSaved).
// mists: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:108 (MarkCharacterSettingsSaved).
// era: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:25 (GetCharacterSettingsSaved).
// era: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:108 (MarkCharacterSettingsSaved).
// anniversary: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:25 (GetCharacterSettingsSaved).
// anniversary: Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua:108 (MarkCharacterSettingsSaved).
// Source-predicted FAIL: saved getter and marker missing; boolean and void calls, original CheckLoad accepts actual result; not executed; native semantics unknown.
#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]
#[test]
fn original_tts_saved_settings_shape_blocker() {
    storage::run_isolated(
        "cf2_tts_shape_contract::original_tts_saved_settings_shape_blocker",
        || {
            let env = shape::load(&["Blizzard_ChatFrame"]);
            shape::check(
                &env,
                r#"

scalar('boolean', C_TTSSettings.GetCharacterSettingsSaved())
assert(select('#', C_TTSSettings.MarkCharacterSettingsSaved()) == 0)

"#,
            );
            assert_original_tts_preference_round_trips(&env);
        },
    );
}

// Keep direct shape preflights from masking the original CheckLoad boundary.
// Keep CheckLoad before numeric acceptance so preference RED cannot mask its API boundary.
// Constant-true saved markers remain indistinguishable: ASK-USER initial/reset oracle.
#[test]
fn original_tts_saved_settings_consumer_blocker() {
    storage::run_isolated(
        "cf2_tts_shape_contract::original_tts_saved_settings_consumer_blocker",
        || {
            let env = shape::load(&["Blizzard_ChatFrame"]);
            shape::check(
                &env,
                r#"

original(TextToSpeechFrame_CheckLoad, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
original(TextToSpeechFrame_OnEvent, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
-- Supplied legacy settings exercise the original saved-getter branch for either boolean.
-- No expected migration result, ownership, saved flag, default or persistence is asserted.
TEXTTOSPEECH_CONFIG = {speechRate=4, speechVolume=37, enabledChatTypes={}, enabledChannelTypes={}}
assert(not TextToSpeechFrame.loaded, 'normal unloaded frame prerequisite')
TextToSpeechFrame_OnEvent(TextToSpeechFrame, 'ADDON_LOADED', 'Blizzard_ClientSavedVariables')
TextToSpeechFrame_OnEvent(TextToSpeechFrame, 'VARIABLES_LOADED')
TextToSpeechFrame_OnEvent(TextToSpeechFrame, 'PLAYER_ENTERING_WORLD')
assert(TextToSpeechFrame.loaded == true)
assert(C_TTSSettings.GetCharacterSettingsSaved() == true)
TextToSpeechFrame_UpdateSliders(TextToSpeechFrame)

"#,
            );
            assert_original_tts_preference_round_trips(&env);
        },
    );
}

#[test]
fn original_tts_documented_preference_pairs_contract() {
    storage::run_isolated(
        "cf2_tts_shape_contract::original_tts_documented_preference_pairs_contract",
        || {
            let env = shape::load(&["Blizzard_ChatFrame"]);
            assert_original_tts_preference_round_trips(&env);
            shape::check(
                &env,
                r#"
original(TextToSpeech_SetSelectedVoice, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
original(TextToSpeechFrame_SetChatTypeEnabled, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
original(TextToSpeechFrame_SetChannelEnabled, 'Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua')
for _, voiceType in ipairs({Enum.TtsVoiceType.Standard, Enum.TtsVoiceType.Alternate}) do
    for _, id in ipairs({7, 9}) do
        TextToSpeech_SetSelectedVoice({voiceID=id}, voiceType)
        assert(C_TTSSettings.GetVoiceOptionID(voiceType) == id)
    end
end
-- Explicit names need no invented catalogue/default. Cross name/ID resolution is not asserted.
for _, name in ipairs({'Supplied voice A', 'Supplied voice B'}) do
    C_TTSSettings.SetVoiceOptionName(Enum.TtsVoiceType.Standard, name)
    assert(C_TTSSettings.GetVoiceOptionName(Enum.TtsVoiceType.Standard) == name)
end
for _, setting in ipairs({
    Enum.TtsBoolSetting.PlaySoundSeparatingChatLineBreaks,
    Enum.TtsBoolSetting.AddCharacterNameToSpeech,
    Enum.TtsBoolSetting.PlayActivitySoundWhenNotFocused,
    Enum.TtsBoolSetting.AlternateSystemVoice,
    Enum.TtsBoolSetting.NarrateMyMessages,
}) do
    for _, enabled in ipairs({false, true}) do
        assert(select('#', C_TTSSettings.SetSetting(setting, enabled)) == 0)
        assert(C_TTSSettings.GetSetting(setting) == enabled)
    end
end
local channel = {name='TTS fixture', shortcut='TTS', localID=7, instanceID=19, zoneChannelID=0, channelType=0}
for _, enabled in ipairs({false, true, false}) do
    assert(TextToSpeechFrame_SetChatTypeEnabled('SAY', enabled) == enabled)
    assert(TextToSpeechFrame_GetChatTypeEnabled('SAY') == enabled)
    assert(TextToSpeechFrame_SetChannelEnabled(channel, enabled) == enabled)
    assert(C_TTSSettings.GetChannelEnabled(channel) == enabled)
end
-- Do not assert initial/reset marker policy or native channelKey encoding.
"#,
            );
        },
    );
}

// wowforever: Blizzard_APIDocumentationGenerated/CVarDocumentation.lua:171 (SetTempCVar).
// wowforever: Blizzard_APIDocumentationGenerated/CVarDocumentation.lua:114 (RemoveTempCVar).
// Source-predicted FAIL: temporary CVar members missing; nullable value and void calls through normal mode callbacks; not executed; native semantics unknown.
#[cfg(any(feature = "client-wowforever"))]
#[test]
fn original_gamepad_temp_cvar_shape_blocker() {
    storage::run_isolated(
        "cf2_tts_shape_contract::original_gamepad_temp_cvar_shape_blocker",
        || {
            let env = shape::load(&["Blizzard_Gamepad"]);
            shape::check(
                &env,
                r#"

-- Original Gamepad/Core.lua passes a number to the documented nullable cstring parameter.
assert(select('#', C_CVar.SetTempCVar('SoftTargetEnemy', 1)) == 0)
assert(select('#', C_CVar.SetTempCVar('SoftTargetEnemy', nil)) == 0)
assert(select('#', C_CVar.RemoveTempCVar('SoftTargetEnemy')) == 0)
EventUtil.TriggerOnVariablesLoaded()
local frame = EnumerateFrames()
local reached = false
while frame do
    if frame:IsEventRegistered('INPUT_DEVICE_INTERFACE_TRANSITION') then
        local handler = frame:GetScript('OnEvent')
        local info = handler and debug.getinfo(handler, 'S')
        if info and info.source:match('Blizzard_SharedXML/InputUtil.lua$') then
            original(handler, 'Blizzard_SharedXML/InputUtil.lua')
            local types = Enum.InputDeviceInterfaceType
            handler(frame, 'INPUT_DEVICE_INTERFACE_TRANSITION', types.Mkb, types.Gamepad)
            handler(frame, 'INPUT_DEVICE_INTERFACE_TRANSITION', types.Gamepad, types.Mkb)
            reached = true
        end
    end
    frame = EnumerateFrames(frame)
end
assert(reached, 'normal original Gamepad transition listener prerequisite')

"#,
            );
        },
    );
}
