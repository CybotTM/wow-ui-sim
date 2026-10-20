#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn arena_original_2340_and_native_no_team_validation() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(&original_lines("Blizzard_Calendar", 2334, 2344))
        .unwrap();
    env.exec(
        r#"
        TOOLTIP_DEFAULT_COLOR = {r=1,g=1,b=1}
        TOOLTIP_DEFAULT_BACKGROUND_COLOR = {r=0,g=0,b=0}
        MAX_ARENA_TEAMS = 3
        local updates = 0
        local watcher = CreateFrame('Frame')
        watcher:RegisterEvent('ARENA_TEAM_ROSTER_UPDATE')
        watcher:SetScript('OnEvent', function() updates = updates + 1 end)
        CalendarArenaTeamContextMenu_OnLoad(CreateFrame('Frame'))
        for _, value in ipairs({0, 1, 2, 3, 4, -1, '1', 1.9}) do
            assert(select('#', ArenaTeamRoster(value)) == 0)
        end
        for _, value in ipairs({false, true, {}, 'bad'}) do
            local ok, err = pcall(ArenaTeamRoster, value)
            assert(not ok and string.find(err, 'Usage: ArenaTeamRoster(team)', 1, true))
        end
        assert(not pcall(ArenaTeamRoster))
        assert(updates == 0)
    "#,
    )
    .unwrap();
}
