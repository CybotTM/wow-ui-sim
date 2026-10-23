//! Original TTS persistence consumers; authored only, runtime RED not observed.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary",
))]

use std::path::PathBuf;
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
