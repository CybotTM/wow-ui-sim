//! Original TTS persistence consumers; authored only, runtime RED not observed.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary",
))]

use std::collections::BTreeMap;
use wow_ui_sim::lua_api::WowLuaEnv;

#[path = "common/cf2_cvar_storage.rs"]
mod storage;

fn new_cache_env() -> WowLuaEnv {
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
    // Observe, do not prescribe a native initial/default marker value.
    let saved_before: bool = env
        .eval("return C_TTSSettings.GetCharacterSettingsSaved()")
        .expect("observe saved marker before original CheckLoad");
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
    assert_saved_marker_after_load(env, saved_before);
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

fn assert_saved_marker_after_load(env: &WowLuaEnv, saved_before: bool) {
    let saved_after: bool = env
        .eval("return C_TTSSettings.GetCharacterSettingsSaved()")
        .expect("observe saved marker after original CheckLoad");
    assert!(saved_after, "original mark must leave settings saved");
    if !saved_before {
        assert_ne!(
            saved_before, saved_after,
            "observed unsaved-to-saved transition"
        );
    }
    // An already-true marker proves preservation only. Initial/reset policy and
    // standalone Mark producer acceptance remain PENDING-NATIVE-ORACLE.
    eprintln!("CF-2 saved marker: {saved_before} -> {saved_after}");
}

#[test]
fn original_tts_slider_settings_survive_mark_saved_and_second_environment() {
    storage::run_isolated(
        "cf2_tts::original_tts_slider_settings_survive_mark_saved_and_second_environment",
        || {
            let mut env = new_cache_env();
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
            // Explicit inputs, not copied host preferences or prescribed native defaults.
            env.exec(
        "assert(TextToSpeech_SetRate(TextToSpeechFrame, 0));          assert(TextToSpeech_SetVolume(TextToSpeechFrame, 0))",
    )
    .expect("seed live preferences through original slider callbacks");
            assert_eq!(
                env.eval::<i32>("return C_TTSSettings.GetSpeechRate()")
                    .unwrap(),
                0
            );
            assert_eq!(
                env.eval::<i32>("return C_TTSSettings.GetSpeechVolume()")
                    .unwrap(),
                0
            );
            let rates = [4, 7];
            let volumes = [37, 63];
            for (rate, volume) in rates.into_iter().zip(volumes) {
                let before = read_live_cvars(&env);
                env.exec(&format!(
                    "assert(TextToSpeech_SetRate(TextToSpeechFrame, {rate})); \
             assert(TextToSpeech_SetVolume(TextToSpeechFrame, {volume}))"
                ))
                .expect("unchanged slider callbacks accept explicit preferences");
                complete_original_tts_load(&env);
                assert_tts_values_and_controls(&env, rate, volume);
                let after = read_live_cvars(&env);
                let rate_keys = changed_preference_keys(&before, &after, rate);
                let volume_keys = changed_preference_keys(&before, &after, volume);
                assert!(
                    !rate_keys.is_empty(),
                    "rate must change the live CVar store"
                );
                assert!(
                    !volume_keys.is_empty(),
                    "volume must change the live CVar store"
                );
                drop(env);

                env = new_cache_env();
                assert_reopened_tts_preferences(&env, rate, volume);
                let reopened = read_live_cvars(&env);
                for key in rate_keys.iter().chain(&volume_keys) {
                    assert_eq!(
                        reopened.get(key),
                        after.get(key),
                        "persisted preference {key}"
                    );
                }
                load_original_addon(&env, "Blizzard_ChatFrame");
                assert_tts_values_and_controls(&env, rate, volume);
            }
            complete_original_tts_load(&env);
            assert_tts_values_and_controls(&env, rates[1], volumes[1]);
        },
    );
}

fn assert_reopened_tts_preferences(reloaded: &WowLuaEnv, rate: i32, volume: i32) {
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
    reloaded
        .exec(&format!(
            "assert(C_TTSSettings.GetCharacterSettingsSaved() == true); \
         assert(C_TTSSettings.GetSpeechRate() == {rate}); \
         assert(C_TTSSettings.GetSpeechVolume() == {volume})"
        ))
        .expect("saved marker and each preference pair survive environment destruction");
}

fn assert_tts_values_and_controls(env: &WowLuaEnv, rate: i32, volume: i32) {
    env.exec(&format!(
        r#"
        assert(C_TTSSettings.GetSpeechRate() == {rate})
        assert(C_TTSSettings.GetSpeechVolume() == {volume})
        local controls = TextToSpeechFrame.PanelContainer
        assert(controls.AdjustRateSlider.Slider:GetValue() == {rate})
        assert(controls.AdjustVolumeSlider.Slider:GetValue() == {volume})
        "#,
    ))
    .expect("original TTS controls and preference getters agree on each explicit pair");
}

fn read_live_cvars(env: &WowLuaEnv) -> BTreeMap<String, String> {
    let state = env.state().borrow();
    state
        .cvars
        .all_keys()
        .into_iter()
        .map(|key| {
            let value = state.cvars.get(&key).expect("listed live CVar has a value");
            (key, value)
        })
        .collect()
}

fn changed_preference_keys(
    before: &BTreeMap<String, String>,
    after: &BTreeMap<String, String>,
    expected: i32,
) -> Vec<String> {
    // Discover observable storage keys instead of inventing native CVar names.
    after
        .iter()
        .filter(|(key, value)| {
            before.get(*key) != Some(*value)
                && value.parse::<f64>().ok() == Some(f64::from(expected))
        })
        .map(|(key, _)| key.clone())
        .collect()
}
