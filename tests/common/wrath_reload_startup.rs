//! Original UIParent consumers with event payload/order observations.
use wow_ui_sim::lua_api::WowLuaEnv;

pub fn assert_startup_readiness(ready: [bool; 2]) {
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1600.0, 1200.0);
    super::original::load_original_wrath_ui(&env);
    env.state().borrow_mut().account_cvar_chunks_ready = ready;
    env.exec(r#"
        SetCVar('timeMgrAlarmEnabled', '1')
        ReloadStartupEvents = {}
        ReloadSyntheticAddons = 0
        local observer = CreateFrame('Frame')
        for _, name in ipairs({'VARIABLES_LOADED', 'PLAYER_LOGIN', 'PLAYER_ENTERING_WORLD', 'ADDON_LOADED'}) do observer:RegisterEvent(name) end
        observer:SetScript('OnEvent', function(self, event, ...)
            if event == 'ADDON_LOADED' then
                if (...) == 'WoWUISim' then ReloadSyntheticAddons = ReloadSyntheticAddons + 1 end
            else
                table.insert(ReloadStartupEvents, { name = event, argc = select('#', ...) })
            end
        end)
    "#).unwrap();
    wow_ui_sim::startup::fire_startup_events(&env);
    let all_ready = ready.iter().all(|value| *value);
    let expected = if all_ready {
        "VARIABLES_LOADED,PLAYER_LOGIN,PLAYER_ENTERING_WORLD"
    } else {
        "PLAYER_LOGIN,PLAYER_ENTERING_WORLD"
    };
    let actual: String = env.eval("local result = {}; for _, e in ipairs(ReloadStartupEvents) do result[#result+1] = e.name end; return table.concat(result, ',')").unwrap();
    assert_eq!(actual, expected, "12340 readiness-dependent startup order");
    env.exec(r#"
        assert(Blizzard_CombatLog_Filters ~= nil, 'original UIParent LOGIN consumer loads combat log')
        assert(ReloadSyntheticAddons == 0, 'WoWUISim is not a native Wrath addon completion')
        for _, e in ipairs(ReloadStartupEvents) do assert(e.argc == 0, 'Wrath login/variables/PEW are argless') end
    "#).unwrap();
    if !all_ready {
        assert!(
            env.eval::<bool>("return TimeManagerFrame == nil").unwrap(),
            "original variables consumer waits for account readiness"
        );
        for (chunk, was_ready) in ready.iter().enumerate() {
            if !was_ready {
                env.set_account_cvar_chunk_ready(chunk, true).unwrap();
            }
        }
        env.exec("assert(#ReloadStartupEvents == 3 and ReloadStartupEvents[3].name == 'VARIABLES_LOADED', 'completion dispatches inline after entry')").unwrap();
    }
    assert!(
        env.eval::<bool>("return TimeManagerFrame ~= nil").unwrap(),
        "original UIParent variables consumer loads time manager"
    );
    env.set_account_cvar_chunk_ready(0, true).unwrap();
    env.set_account_cvar_chunk_ready(1, true).unwrap();
    assert_eq!(
        env.eval::<i32>("return #ReloadStartupEvents").unwrap(),
        3,
        "completion is exactly once"
    );
}
