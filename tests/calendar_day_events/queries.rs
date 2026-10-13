use super::fixtures::{KEY, assert_lua, erase, record, supplied_env, supply};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn calendar_day_events_complete_snapshots_survive_replacement_and_erase() {
    let env = supplied_env();
    assert_lua(
        &env,
        r#"
        DayBefore = C_Calendar.GetDayEvent(0,15,1)
        local other = C_Calendar.GetDayEvent(0,15,1)
        assert(not rawequal(DayBefore, other), 'fresh outer table')
        assert(not rawequal(DayBefore.startTime, other.startTime), 'fresh startTime')
        assert(not rawequal(DayBefore.endTime, other.endTime), 'fresh endTime')
        assert(not rawequal(DayBefore.startTime, DayBefore.endTime), 'independent time tables')
        other.title, other.eventID, other.clubID = 'caller', '1', '2'
        other.startTime.hour, other.endTime.monthDay = 0, 1
        other.extra = true
        CalendarDayEqual(DayBefore, CalendarDayExpected('Host raid',136116))
        CalendarDayOne(CalendarDayExpected('Host raid',136116), C_Calendar.GetDayEvent(0,15,1))
    "#,
    );
    let mut replacement = record();
    replacement.title = "Replacement raid".into();
    replacement.icon_texture = None;
    replacement.start_time.hour = 20.0;
    replacement.end_time.month_day = 17;
    supply(&env, KEY, vec![replacement]);
    assert_lua(
        &env,
        r#"
        DayAfter = C_Calendar.GetDayEvent(0,15,1)
        DayReplacementExpected = CalendarDayExpected('Replacement raid',nil)
        DayReplacementExpected.startTime.hour = 20
        DayReplacementExpected.endTime.monthDay = 17
        CalendarDayEqual(DayAfter, DayReplacementExpected)
        assert(DayAfter.iconTexture == nil, 'optional absent: exactly 20 outer fields')
        assert(not rawequal(DayAfter, DayBefore))
        assert(not rawequal(DayAfter.startTime, DayBefore.startTime))
        assert(not rawequal(DayAfter.endTime, DayBefore.endTime))
        CalendarDayEqual(DayBefore, CalendarDayExpected('Host raid',136116))
    "#,
    );
    erase(&env);
    assert_lua(
        &env,
        r#"
        CalendarDayMissing(C_Calendar.GetDayEvent(0,15,1))
        CalendarDayCount(0,C_Calendar.GetNumDayEvents(0,15))
        CalendarDayEqual(DayBefore, CalendarDayExpected('Host raid',136116))
        CalendarDayEqual(DayAfter, DayReplacementExpected)
    "#,
    );
}

#[test]
fn calendar_day_events_counts_exact_keys_and_dense_positions_without_default_rows() {
    let env = WowLuaEnv::new().expect("bare count environment");
    assert!(
        env.state().borrow().calendar_day_events.is_empty(),
        "no default day rows"
    );
    let mut second = record();
    second.title = "Second raid".into();
    supply(&env, KEY, vec![record(), second]);
    // First API result is positive count: constant-zero baseline must fail here.
    assert_lua(
        &env,
        r#"
        CalendarDayCount(2,C_Calendar.GetNumDayEvents(0,15))
        CalendarDayOne(CalendarDayExpected('Host raid',136116), C_Calendar.GetDayEvent(0,15,1))
        CalendarDayOne(CalendarDayExpected('Second raid',136116), C_Calendar.GetDayEvent(0,15,2))
        CalendarDayMissing(C_Calendar.GetDayEvent(0,15,3))
        CalendarDayCount(0,C_Calendar.GetNumDayEvents(1,15))
        CalendarDayCount(0,C_Calendar.GetNumDayEvents(0,16))
        CalendarDayMissing(C_Calendar.GetDayEvent(1,15,1))
        CalendarDayMissing(C_Calendar.GetDayEvent(0,16,1))
    "#,
    );
    let mut adjacent = record();
    adjacent.title = "Previous month".into();
    supply(&env, (-1, 15), vec![adjacent]);
    supply(&env, (0, 16), vec![]);
    assert_lua(
        &env,
        r#"
        CalendarDayCount(1,C_Calendar.GetNumDayEvents(-1,15))
        CalendarDayOne(CalendarDayExpected('Previous month',136116), C_Calendar.GetDayEvent(-1,15,1))
        CalendarDayCount(2,C_Calendar.GetNumDayEvents(0,15))
        CalendarDayCount(0,C_Calendar.GetNumDayEvents(0,16))
        CalendarDayMissing(C_Calendar.GetDayEvent(0,16,1))
    "#,
    );
}

#[test]
fn calendar_day_events_same_key_is_isolated_between_environments() {
    let first = supplied_env();
    let second = supplied_env();
    let mut other = record();
    other.title = "Other environment".into();
    supply(&second, KEY, vec![other]);
    erase(&first);
    assert_lua(
        &first,
        r#"
        CalendarDayMissing(C_Calendar.GetDayEvent(0,15,1))
        CalendarDayCount(0,C_Calendar.GetNumDayEvents(0,15))
    "#,
    );
    assert_lua(
        &second,
        r#"
        CalendarDayOne(CalendarDayExpected('Other environment',136116), C_Calendar.GetDayEvent(0,15,1))
        CalendarDayCount(1,C_Calendar.GetNumDayEvents(0,15))
    "#,
    );
}

#[test]
fn calendar_day_events_all_required_selectors_reject_non_numeric_types() {
    let env = supplied_env();
    assert_lua(
        &env,
        r#"
        local get, count = C_Calendar.GetDayEvent, C_Calendar.GetNumDayEvents
        local function rejected(fn, ...)
            local ok, message = pcall(fn,...)
            assert(not ok and type(message)=='string' and #message>0, 'required numeric input')
        end
        rejected(get); rejected(get,0); rejected(get,0,15)
        rejected(count); rejected(count,0)
        for _, value in ipairs({'0', '15', false, true, {}, function() end}) do
            rejected(get,value,15,1); rejected(get,0,value,1); rejected(get,0,15,value)
            rejected(count,value,15); rejected(count,0,value)
            rejected(get,99,value,1); rejected(count,99,value)
        end
        rejected(get,nil,15,1); rejected(get,0,nil,1); rejected(get,0,15,nil)
        rejected(count,nil,15); rejected(count,0,nil)
    "#,
    );
}

#[test]
fn calendar_day_events_nonrepresented_numbers_miss_without_aliasing_or_native_bounds_claim() {
    let env = supplied_env();
    assert_lua(
        &env,
        r#"
        local get, count = C_Calendar.GetDayEvent, C_Calendar.GetNumDayEvents
        -- Representation/miss policy, NOT calendar-date/native range validation.
        for _, value in ipairs({0/0, math.huge, -math.huge, 1e308, -1e308}) do
            CalendarDayCount(0,count(value,15)); CalendarDayCount(0,count(0,value))
            CalendarDayMissing(get(value,15,1)); CalendarDayMissing(get(0,value,1))
            CalendarDayMissing(get(0,15,value))
        end
        CalendarDayCount(0,count(0.5,15)); CalendarDayCount(0,count(0,15.5))
        CalendarDayMissing(get(0.5,15,1)); CalendarDayMissing(get(0,15.5,1))
        for _, position in ipairs({-1,0,1.5,2,4294967297}) do
            CalendarDayMissing(get(0,15,position))
        end
        CalendarDayCount(0,count(0,-1)); CalendarDayCount(0,count(0,4294967311))
        CalendarDayMissing(get(0,-1,1)); CalendarDayMissing(get(0,4294967311,1))
        CalendarDayCount(0,count(4294967296,15))
        CalendarDayMissing(get(4294967296,15,1))
        CalendarDayCount(1,count(0,15))
        CalendarDayOne(CalendarDayExpected('Host raid',136116),get(0,15,1))
    "#,
    );
}
