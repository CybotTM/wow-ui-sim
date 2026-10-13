//! Unchanged cached ItemRef.lua and CalendarUtil; no synthetic consumer or Club producer.
use super::fixtures::{KEY, assert_lua, erase, record, supplied_env, supply};
use crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use wow_ui_sim::c_api::c_calendar::CalendarEventIndexInfo;
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_consumers(env: &WowLuaEnv) {
    let cache = wow_ui_sim::paths::default_blizzard_ui_addons_path()
        .expect("active Retail/PTR profile cache required");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(wow_ui_sim::screen::ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![cache.clone()];
    wow_ui_sim::xml::register_intrinsic_templates();
    load_blizzard_addon_closure_into_env(
        env,
        &cache,
        &[
            "Blizzard_SharedXMLGame",
            "Blizzard_Colors",
            "Blizzard_FrameXMLBase",
            "Blizzard_FrameXMLUtil",
        ],
        &[],
    );
    // Load the ENTIRE real file, not extracted/recreated GetCalendarEventLink.
    // Its full UIPanels TOC pulls unrelated panels; callable needs only real LinkUtil.
    let path = cache.join("Blizzard_UIPanels_Game/Mainline/ItemRef.lua");
    let source = std::fs::read_to_string(&path).expect("cached complete ItemRef.lua");
    env.exec(&source)
        .expect("load unchanged complete cached ItemRef.lua");
    assert_lua(
        env,
        r#"
        assert(type(GetCalendarEventLink)=='function', 'real ItemRef consumer available')
        assert(type(LinkUtil)=='table' and type(LinkUtil.FormatLink)=='function')
        assert(LinkTypes.CalendarEvent=='calendarEvent')
    "#,
    );
}

#[test]
fn calendar_day_events_cached_itemref_link_tracks_title_replacement_and_deletion() {
    let env = supplied_env();
    load_consumers(&env);
    assert_lua(
        &env,
        r#"
        CalendarLinkBefore = GetCalendarEventLink(0,15,1)
        assert(CalendarLinkBefore=='|HcalendarEvent:0:15:1|hHost raid|h')
    "#,
    );
    let mut replacement = record();
    replacement.title = "Updated host raid".into();
    supply(&env, KEY, vec![replacement]);
    assert_lua(
        &env,
        r#"
        assert(GetCalendarEventLink(0,15,1)=='|HcalendarEvent:0:15:1|hUpdated host raid|h')
        assert(CalendarLinkBefore=='|HcalendarEvent:0:15:1|hHost raid|h')
    "#,
    );
    erase(&env);
    assert_lua(
        &env,
        r#"
        assert(GetCalendarEventLink(0,15,1)==nil, 'real consumer miss after host erase')
    "#,
    );
}

#[test]
fn calendar_day_events_cached_ongoing_broadcast_uses_separate_caller_numeric_index_fixture() {
    let env = supplied_env();
    // Caller-owned numeric relation is separate from the opaque string day identifiers.
    env.state().borrow_mut().calendar_event_indices.insert(
        910001,
        CalendarEventIndexInfo {
            offset_months: 0,
            month_day: 15,
            event_index: 1,
        },
    );
    load_consumers(&env);
    assert_lua(
        &env,
        r#"
        assert(type(CalendarUtil.GetOngoingEventBroadcastText)=='function')
        assert(type(GameTime_GetFormattedTime)=='function')
        assert(type(COMMUNITIES_CALENDAR_CHAT_EVENT_BROADCAST_FORMAT)=='string')
        assert(type(COMMUNITIES_CALENDAR_CHAT_EVENT_TITLE_FORMAT)=='string')
        assert(type(COMMUNITIES_CALENDAR_ONGOING_EVENT_PREFIX)=='string')
        assert(LINK_FONT_COLOR and YELLOW_FONT_COLOR and CALENDAR_INVITESTATUS_INFO)
        local day = C_Calendar.GetDayEvent(0,15,1)
        assert(day.eventID=='9007199254740997' and day.clubID=='9007199254740995')
        local index = C_Calendar.GetEventIndexInfo(910001,nil,15)
        assert(index.offsetMonths==0 and index.monthDay==15 and index.eventIndex==1)
        -- No string-to-number conversion: this is a separate caller event fixture.
        CalendarCallerEvent = {
            eventID=910001, monthDay=15, startTime=day.startTime,
            inviteStatus=Enum.CalendarStatus.Confirmed,
        }
        local status = CALENDAR_INVITESTATUS_INFO[Enum.CalendarStatus.Confirmed]
        assert(status and type(status.name)=='string' and status.color)
        function CalendarExpectedBroadcast(title)
            local link = '|HcalendarEvent:0:15:1|h'..title..'|h'
            return COMMUNITIES_CALENDAR_CHAT_EVENT_BROADCAST_FORMAT:format(
                YELLOW_FONT_COLOR:WrapTextInColorCode(COMMUNITIES_CALENDAR_ONGOING_EVENT_PREFIX),
                LINK_FONT_COLOR:WrapTextInColorCode(COMMUNITIES_CALENDAR_CHAT_EVENT_TITLE_FORMAT:format(link)),
                status.color:WrapTextInColorCode(status.name))
        end
        assert(CalendarUtil.GetOngoingEventBroadcastText(CalendarCallerEvent)==CalendarExpectedBroadcast('Host raid'))
    "#,
    );
    let mut replacement = record();
    replacement.title = "Broadcast update".into();
    supply(&env, KEY, vec![replacement]);
    assert_lua(
        &env,
        r#"
        assert(CalendarUtil.GetOngoingEventBroadcastText(CalendarCallerEvent)==CalendarExpectedBroadcast('Broadcast update'))
        assert(C_Calendar.GetDayEvent(0,15,1).eventID=='9007199254740997')
    "#,
    );
}
