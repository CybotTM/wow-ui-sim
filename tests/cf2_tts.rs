//! CF-2 original consumers. Authored only; RED awaits an admitted sealed runner.

use std::path::PathBuf;
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

fn require_isolated_persistent_storage() {
    // Cargo's deps convention gives EACH environment a different CVar file.
    // Persistence proof instead requires the recipe's sealed, isolated process.
    let executable = std::env::current_exe().expect("locate sealed test executable");
    assert!(
        !executable.parent().is_some_and(|p| p.ends_with("deps")),
        "run sealed outside deps; per-environment temporary stores cannot prove persistence"
    );
    let isolated = PathBuf::from(
        std::env::var_os("CF2_TTS_ISOLATED_DATA_HOME")
            .expect("runner must attest child-only copied CVar storage"),
    );
    assert!(isolated.is_absolute());
    assert_eq!(std::env::var_os("XDG_DATA_HOME"), Some(isolated.into()));
}

fn new_cache_env() -> WowLuaEnv {
    require_isolated_persistent_storage();
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    eprintln!("CF-2 original consumer cache: {}", ui.display());
    crate::common::blizzard_addon_harness::new_blizzard_addon_env(&ui)
}

fn load_original_addon(env: &WowLuaEnv, addon: &str) {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let loaded = crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env(
        env,
        &ui,
        &[addon],
        &[],
    );
    assert!(loaded.iter().any(|name| name == addon));
}

fn assert_original_function(env: &WowLuaEnv, function: &str, suffix: &str) {
    env.exec(&format!(
        r#"
        local info = debug.getinfo({function}, 'S')
        local source = info.source:gsub('\\', '/')
        local suffix = '{suffix}'
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'unchanged cached consumer required')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)
        "#,
    ))
    .expect("original cached Lua function provenance");
}

fn complete_original_tts_load(env: &WowLuaEnv) {
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec(
        r#"
        assert(TextToSpeechFrame and not TextToSpeechFrame.loaded)
        -- Explicit absence of legacy migration input, not a replacement API.
        TEXTTOSPEECH_CONFIG = nil
        TextToSpeechFrame_OnEvent(TextToSpeechFrame, 'ADDON_LOADED', 'Blizzard_ClientSavedVariables')
        TextToSpeechFrame_OnEvent(TextToSpeechFrame, 'VARIABLES_LOADED')
        TextToSpeechFrame_OnEvent(TextToSpeechFrame, 'PLAYER_ENTERING_WORLD')
        assert(TextToSpeechFrame.loaded == true)
        assert(C_TTSSettings.GetCharacterSettingsSaved() == true)
        "#,
    )
    .expect("original CheckLoad must mark character settings and update real controls");
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

#[test]
fn original_tts_slider_settings_survive_mark_saved_and_second_environment() {
    if ACTIVE == ClientProfile::Wrath {
        eprintln!("CF-2 shared TTS consumer N/A for Wrath; not passing proof");
        return;
    }
    let env = new_cache_env();
    // Seed the LIVE CVar store used by the Lua API, not a parallel Lua table.
    assert!(
        env.state()
            .borrow()
            .cvars
            .set("TTSUseCharacterSettings", "1")
    );
    load_original_addon(&env, "Blizzard_ChatFrame");
    for function in [
        "TextToSpeech_SetRate",
        "TextToSpeech_SetVolume",
        "TextToSpeechFrame_OnEvent",
        "TextToSpeechFrame_CheckLoad",
    ] {
        assert_original_function(
            &env,
            function,
            "Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua",
        );
    }
    env.exec(
        r#"
        assert(TextToSpeech_SetRate(TextToSpeechFrame, 4))
        assert(TextToSpeech_SetVolume(TextToSpeechFrame, 37))
        "#,
    )
    .expect("original slider callbacks must accept explicit in-range settings");
    complete_original_tts_load(&env);
    assert_tts_values_and_controls(&env);
    drop(env);

    let reloaded = new_cache_env();
    assert_eq!(
        reloaded
            .state()
            .borrow()
            .cvars
            .get("TTSUseCharacterSettings")
            .as_deref(),
        Some("1"),
        "second environment must reopen the same child-only persisted CVar file"
    );
    // Read BEFORE original CheckLoad can mark anything in the second environment.
    reloaded.exec("assert(C_TTSSettings.GetCharacterSettingsSaved() == true); assert(C_TTSSettings.GetSpeechRate() == 4); assert(C_TTSSettings.GetSpeechVolume() == 37)")
        .expect("saved character marker and actual values must survive environment destruction");
    load_original_addon(&reloaded, "Blizzard_ChatFrame");
    complete_original_tts_load(&reloaded);
    assert_tts_values_and_controls(&reloaded);
}

fn assert_tts_values_and_controls(env: &WowLuaEnv) {
    env.exec(
        r#"
        assert(C_TTSSettings.GetSpeechRate() == 4)
        assert(C_TTSSettings.GetSpeechVolume() == 37)
        local controls = TextToSpeechFrame.PanelContainer
        assert(controls.AdjustRateSlider.Slider:GetValue() == 4)
        assert(controls.AdjustVolumeSlider.Slider:GetValue() == 37)
        "#,
    )
    .expect("original TTS update must render saved values, not constant defaults");
}

fn drive_original_input_transition(env: &WowLuaEnv, new: &str, old: &str) {
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec(&format!(
        "local types = Enum.InputDeviceInterfaceType; \
         Cf2InputListener:GetScript('OnEvent')(Cf2InputListener, \
         'INPUT_DEVICE_INTERFACE_TRANSITION', types.{new}, types.{old})"
    ))
    .expect("original InputUtil listener must run original Gamepad/Core callbacks");
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

fn assert_live_cvars(env: &WowLuaEnv, enemy: &str) {
    assert_eq!(
        env.state().borrow().cvars.get("SoftTargetEnemy").as_deref(),
        Some(enemy)
    );
    assert_eq!(
        env.state()
            .borrow()
            .cvars
            .get("cameraDistanceMaxZoomFactor")
            .as_deref(),
        Some("1.9")
    );
    env.exec(&format!(
        "assert(GetCVar('SoftTargetEnemy') == '{enemy}'); \
         assert(C_CVar.GetCVar('SoftTargetEnemy') == '{enemy}'); \
         assert(GetCVar('cameraDistanceMaxZoomFactor') == '1.9')"
    ))
    .expect("Lua and Rust must observe the same effective CVar storage");
}

#[test]
fn original_gamepad_temp_cvars_restore_persisted_values_without_touching_ordinary_cvar() {
    if ACTIVE != ClientProfile::WowForever {
        eprintln!("CF-2 Gamepad/Core consumer N/A for {ACTIVE:?}; not passing proof");
        return;
    }
    let env = new_cache_env();
    assert!(env.state().borrow().cvars.set("SoftTargetEnemy", "2"));
    assert!(
        env.state()
            .borrow()
            .cvars
            .set("cameraDistanceMaxZoomFactor", "1.9")
    );
    load_original_addon(&env, "Blizzard_Gamepad");
    assert_original_function(
        &env,
        "InputUtil.RegisterMKBInit",
        "Blizzard_SharedXML/InputUtil.lua",
    );
    env.exec(
        r#"
        EventUtil.TriggerOnVariablesLoaded()
        local frame = EnumerateFrames()
        local count = 0
        while frame do
            if frame:IsEventRegistered('INPUT_DEVICE_INTERFACE_TRANSITION') then
                local handler = frame:GetScript('OnEvent')
                local info = handler and debug.getinfo(handler, 'S')
                if info and info.source:gsub('\\', '/'):match('Blizzard_SharedXML/InputUtil.lua$') then
                    count = count + 1
                    Cf2InputListener = frame
                    print('CF-2 input listener provenance', info.source, info.linedefined, info.lastlinedefined)
                end
            end
            frame = EnumerateFrames(frame)
        end
        assert(count == 1, 'exact original InputUtil transition listener required')
        "#,
    )
    .expect("original Gamepad registration and original event listener must exist");
    // Direct unchanged listener invocation exposes errors dispatch might record.
    drive_original_input_transition(&env, "Mkb", "Gamepad");
    assert_live_cvars(&env, "2");
    drive_original_input_transition(&env, "Gamepad", "Mkb");
    assert_live_cvars(&env, "1");
    // A live temporary override must not replace the underlying persisted value.
    let during_override = new_cache_env();
    assert_live_cvars(&during_override, "2");
    drop(during_override);
    drive_original_input_transition(&env, "Mkb", "Gamepad");
    assert_live_cvars(&env, "2");
    drive_original_input_transition(&env, "Mkb", "Mkb");
    assert_live_cvars(&env, "2");
    drop(env);
    assert_live_cvars(&new_cache_env(), "2");
}
