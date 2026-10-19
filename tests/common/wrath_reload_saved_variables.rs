//! Persisted-file execution observes unchanged original addon initialization.
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::SavedVariablesManager;

pub fn assert_original_restore_after_code() {
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1600.0, 1200.0);
    env.set_screen_mode(wow_ui_sim::screen::ScreenKind::Game);
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().unwrap();
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    assert!(env.eval::<bool>("return StopwatchFrame == nil").unwrap());
    let storage = tempfile::tempdir().unwrap();
    let character_dir = storage.path().join("SimRealm/SimPlayer");
    std::fs::create_dir_all(&character_dir).unwrap();
    std::fs::write(
        character_dir.join("Blizzard_TimeManager.lua"),
        r#"
        SavedReadCount = (SavedReadCount or 0) + 1
        SavedSawOriginal = StopwatchFrame ~= nil and type(StopwatchFrame_OnEvent) == 'function'
        BlizzardStopwatchOptions = { position = { x = 387, y = 249 }, supplied = 713 }
    "#,
    )
    .unwrap();
    let mut manager = SavedVariablesManager::with_storage_dir(storage.path().to_path_buf());
    let result = wow_ui_sim::loader::load_addon_with_saved_vars(
        &env.loader_env(),
        &ui.join("Blizzard_TimeManager/Blizzard_TimeManager.toc"),
        &mut manager,
    )
    .unwrap();
    assert!(result.warnings.is_empty(), "{:?}", result.warnings);
    assert!(
        env.eval::<bool>("return SavedSawOriginal == true").unwrap(),
        "12340 executes original addon code before the saved character file"
    );
    env.exec(
        r#"
        assert(SavedReadCount == 1, 'saved file executes once per scope, not per variable')
        assert(BlizzardStopwatchOptions.supplied == 713)
        StopwatchFrame:HookScript('OnEvent', function(self, event, ...)
            if event == 'ADDON_LOADED' then
                SavedEventArgc = select('#', ...)
                SavedEventPosition = BlizzardStopwatchOptions.position.x
            end
        end)
    "#,
    )
    .unwrap();
    env.fire_event_with_args("ADDON_LOADED", &[env.lua_string("Blizzard_TimeManager")])
        .unwrap();
    env.exec(
        r#"
        assert(SavedEventArgc == 1 and SavedEventPosition == 387)
        local x, y = StopwatchFrame:GetCenter()
        assert(x == 387 and y == 249, 'original ADDON_LOADED consumer restores saved geometry')
    "#,
    )
    .unwrap();
}
