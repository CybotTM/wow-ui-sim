//! Original consumers plus persisted old-VM event observations.
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::SavedVariablesManager;

fn register_originals(env: &WowLuaEnv, manager: &mut SavedVariablesManager) {
    let root = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    for name in [
        "Blizzard_TimeManager",
        "Blizzard_BattlefieldMinimap",
        "Blizzard_CombatLog",
    ] {
        let toc = wow_ui_sim::toc::TocFile::from_file(&root.join(name).join(format!("{name}.toc")))
            .unwrap();
        env.loader_env()
            .with_state(|state| {
                manager.init_for_addon(
                    state,
                    name,
                    &toc.saved_variables(),
                    &toc.saved_variables_per_character(),
                )
            })
            .unwrap();
    }
    env.loader_env()
        .with_state(|state| {
            manager.init_for_addon(state, "ReloadObserver", &["ReloadTrace".into()], &[])
        })
        .unwrap();
}

pub fn assert_shutdown(in_world: bool, combat: bool) {
    let mut env = super::original::load_original_wrath_startup();
    env.exec("LoadAddOn('Blizzard_TimeManager'); LoadAddOn('Blizzard_BattlefieldMinimap'); LoadAddOn('Blizzard_CombatLog')").unwrap();
    let storage = tempfile::tempdir().unwrap();
    let mut manager = SavedVariablesManager::with_storage_dir(storage.path().to_path_buf());
    register_originals(&env, &mut manager);
    {
        let mut state = env.state().borrow_mut();
        state.in_world = in_world;
        state.player.in_combat = combat;
    }
    env.exec(r#"
        StopwatchFrame:ClearAllPoints()
        StopwatchFrame:SetPoint('CENTER', UIParent, 'BOTTOMLEFT', 321, 234)
        StopwatchFrame:SetUserPlaced(true)
        BattlefieldMinimapTab:ClearAllPoints()
        BattlefieldMinimapTab:SetPoint('CENTER', UIParent, 'BOTTOMLEFT', 456, 345)
        BattlefieldMinimapTab:SetUserPlaced(true)
        Blizzard_CombatLog_Filters.ReloadControl = 7321
        PlayerFrame.onHateList = 1
        ReloadTrace = { events = {}, payloads = {}, permissions = {} }
        local observer = CreateFrame('Frame')
        for _, name in ipairs({'PLAYER_REGEN_ENABLED', 'PLAYER_LEAVING_WORLD', 'PLAYER_LOGOUT', 'ADDONS_UNLOADING'}) do
            observer:RegisterEvent(name)
        end
        observer:SetScript('OnEvent', function(self, event, ...)
            table.insert(ReloadTrace.events, event)
            table.insert(ReloadTrace.payloads, select('#', ...))
            table.insert(ReloadTrace.permissions, not InCombatLockdown())
            if event == 'PLAYER_LOGOUT' then
                ReloadTrace.hateCleared = PlayerFrame.onHateList == nil
            end
        end)
        ReloadUI()
    "#).unwrap();
    let mut manager = Some(manager);
    assert!(env.service_reload(&mut manager).unwrap());
    assert_eq!(
        env.state().borrow().player.in_combat,
        combat,
        "temporary allowance must not erase combat"
    );
    for path in [
        storage.path().join("ReloadObserver.lua"),
        storage.path().join("Blizzard_CombatLog.lua"),
        storage
            .path()
            .join("SimRealm/SimPlayer/Blizzard_TimeManager.lua"),
        storage
            .path()
            .join("SimRealm/SimPlayer/Blizzard_BattlefieldMinimap.lua"),
    ] {
        env.exec(&std::fs::read_to_string(path).unwrap()).unwrap();
    }
    let expected = if in_world {
        "PLAYER_REGEN_ENABLED,PLAYER_LEAVING_WORLD,PLAYER_LOGOUT"
    } else {
        "PLAYER_REGEN_ENABLED,PLAYER_LOGOUT"
    };
    assert_eq!(
        env.eval::<String>("return table.concat(ReloadTrace.events, ',')")
            .unwrap(),
        expected,
        "12340 synchronous shutdown order before SV writes"
    );
    env.exec(r#"
        for _, count in ipairs(ReloadTrace.payloads) do assert(count == 0, 'argless shutdown') end
        for _, allowed in ipairs(ReloadTrace.permissions) do assert(allowed, 'temporary protected-action window') end
        assert(ReloadTrace.hateCleared, 'original PlayerFrame regen handler clears hate display')
        assert(BlizzardStopwatchOptions.position.x == 321 and BlizzardStopwatchOptions.position.y == 234, 'original stopwatch logout mutation persisted')
        assert(BattlefieldMinimapOptions.position.x == 456 and BattlefieldMinimapOptions.position.y == 345, 'original minimap logout mutation persisted')
        assert(Blizzard_CombatLog_Filters.ReloadControl == 7321, 'account scope retained')
    "#).unwrap();
}
