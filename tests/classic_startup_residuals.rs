//! Era/Anniversary stock startup consumers, without third-party addons or saved variables.
#![cfg(any(feature = "client-era", feature = "client-anniversary"))]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn load_stock_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    let ui = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    crate::common::load_profile_framexml(&env);
    for (name, toc) in
        wow_ui_sim::loader::discover_blizzard_addons_for_screen(&ui, ScreenKind::Game)
    {
        wow_ui_sim::loader::load_addon(&env.loader_env(), &toc)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
    }
    env.apply_post_load_workarounds();
    env
}

#[test]
fn classic_original_tts_load_marks_character_settings_saved() {
    let env = load_stock_ui();
    env.exec(
        r#"
        TextToSpeechFrame.loaded = false
        TextToSpeechFrame.loadedEvents = {
            PLAYER_ENTERING_WORLD = true,
            VARIABLES_LOADED = true,
            ADDON_LOADED = true,
        }
        TextToSpeechFrame_CheckLoad(TextToSpeechFrame)
        assert(C_TTSSettings.GetCharacterSettingsSaved() == true)
        assert(select('#', C_TTSSettings.MarkCharacterSettingsSaved()) == 0)
        "#,
    )
    .unwrap();
    let fresh = WowLuaEnv::new().unwrap();
    assert!(
        !fresh
            .eval::<bool>("return C_TTSSettings.GetCharacterSettingsSaved()")
            .unwrap()
    );
}

#[test]
fn classic_original_startup_timers_complete_without_lua_errors() {
    let env = load_stock_ui();
    wow_ui_sim::startup::fire_startup_events_for_screen(&env, ScreenKind::Game);
    let callbacks = env
        .eval::<String>(
            r#"
        local labels = {}
        local registry = debug.getregistry()
        for id, callback in pairs(registry.__rilua_timer_callbacks or {}) do
            local info = debug.getinfo(callback, 'S')
            local originals = registry.__wow_timer_original_callbacks
            local original = originals and originals[callback]
            local originalInfo = original and debug.getinfo(original, 'S')
            labels[#labels + 1] = tostring(id) .. ':' .. tostring(info and info.source)
                .. ':' .. tostring(info and info.linedefined)
                .. ' original=' .. tostring(originalInfo and originalInfo.source)
                .. ':' .. tostring(originalInfo and originalInfo.linedefined)
        end
        table.sort(labels)
        return table.concat(labels, '\n')
    "#,
        )
        .unwrap();
    eprintln!("Pending original startup callbacks:\n{callbacks}");
    let before = env.state().borrow().lua_errors.len();
    let fired = env.process_timers().unwrap();
    assert!(fired > 0, "the original startup must schedule callbacks");
    let errors = env.state().borrow().lua_errors[before..].to_vec();
    assert!(
        errors.is_empty(),
        "Original startup timer errors: {errors:#?}"
    );
}
