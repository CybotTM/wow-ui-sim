#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn calendar_repeat_original_3811_returns_four_localized_strings() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        CALENDAR_REPEAT_NEVER = 'never fixture'
        CALENDAR_REPEAT_WEEKLY = 'weekly fixture'
        CALENDAR_REPEAT_BIWEEKLY = 'biweekly fixture'
        CALENDAR_REPEAT_MONTHLY = 'monthly fixture'
        function CalendarCreateEventTypeDropDown_InitRepeatOptions(self, ...)
            self.values = {...}; self.count = select('#', ...)
        end
        local watcher = CreateFrame('Frame')
        repeatEvents = 0
        watcher:RegisterEvent('CALENDAR_ACTION_PENDING')
        watcher:SetScript('OnEvent', function() repeatEvents = repeatEvents + 1 end)
    "#,
    )
    .unwrap();
    env.exec(&original_lines("Blizzard_Calendar", 3810, 3812))
        .unwrap();
    env.exec(
        r#"
        local dropdown = {}
        CalendarCreateEventRepeatOptionDropDown_Initialize(dropdown)
        assert(dropdown.count == 4)
        assert(table.concat(dropdown.values, '|') ==
            'never fixture|weekly fixture|biweekly fixture|monthly fixture')
        assert(select('#', CalendarEventGetRepeatOptions(false, {})) == 4)
        assert(repeatEvents == 0)
    "#,
    )
    .unwrap();
}
