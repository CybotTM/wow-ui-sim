#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn calendar_original_3771_types_and_4521_defaults() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        CALENDAR_TYPE_RAID, CALENDAR_TYPE_DUNGEON = 'raid fixture', 'dungeon fixture'
        CALENDAR_TYPE_PVP, CALENDAR_TYPE_MEETING, CALENDAR_TYPE_OTHER = 'pvp fixture', 'meeting fixture', 'other fixture'
        function CalendarCreateEventTypeDropDown_InitEventTypes(self, ...)
            self.types = {...}; self.count = select('#', ...)
        end
    "#).unwrap();
    env.exec(&original_lines("Blizzard_Calendar", 3770, 3772))
        .unwrap();
    env.exec(r#"
        local dropdown = {}
        CalendarCreateEventTypeDropDown_Initialize(dropdown)
        assert(dropdown.count == 5)
        assert(table.concat(dropdown.types, ',') == 'raid fixture,dungeon fixture,pvp fixture,meeting fixture,other fixture')
    "#).unwrap();
    let source = original_lines("Blizzard_Calendar", 4521, 4521);
    env.exec(&format!(
        "{source}\nassert(minLevel == 80 and maxLevel == 80)"
    ))
    .unwrap();
    let ranks = env.state().borrow().world.guild_ranks.len();
    let actual: (i32, i32, i32) = env.eval("return CalendarDefaultGuildFilter()").unwrap();
    assert_eq!(actual, (80, 80, ranks as i32));
    for (expansion, cap) in [(0, 60), (1, 70), (2, 80)] {
        env.state()
            .borrow_mut()
            .wrath_legacy
            .calendar
            .account_expansion = expansion;
        let actual: (i32, i32, i32) = env
            .eval("return CalendarDefaultGuildFilter(false)")
            .unwrap();
        assert_eq!(actual, (cap, cap, ranks as i32));
    }
    // Native wrapper reads m_numRanks directly; do not repair guild-model state here.
    env.state().borrow_mut().world.guild_name = None;
    let actual: (i32, i32, i32) = env.eval("return CalendarDefaultGuildFilter()").unwrap();
    assert_eq!(actual, (80, 80, ranks as i32));
    env.state()
        .borrow_mut()
        .wrath_legacy
        .calendar
        .player_present = false;
    let actual: (i32, i32, i32) = env.eval("return CalendarDefaultGuildFilter()").unwrap();
    assert_eq!(actual, (0, 0, 0));
    env.state()
        .borrow_mut()
        .wrath_legacy
        .calendar
        .player_present = true;
    env.state().borrow_mut().world.guild_ranks.clear();
    let actual: (i32, i32, i32) = env.eval("return CalendarDefaultGuildFilter()").unwrap();
    assert_eq!(actual, (80, 80, 0));
    env.state()
        .borrow_mut()
        .wrath_legacy
        .calendar
        .player_present = false;
    let actual: (i32, i32, i32) = env.eval("return CalendarDefaultGuildFilter()").unwrap();
    assert_eq!(actual, (0, 0, 0));
    env.exec("assert(select('#', CalendarEventGetTypes(42)) == 5); assert(select('#', CalendarDefaultGuildFilter()) == 3)").unwrap();
}
