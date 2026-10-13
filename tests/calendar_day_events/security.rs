//! Actual VM secret inputs, following calendar_event_index_info helpers.
//! Unlocked host output only; no SecretInChatMessagingLockdown claim.
use super::fixtures::{assert_lua, supplied_env};

#[test]
fn calendar_day_events_untainted_secret_selectors_preserve_original_wrappers() {
    let env = supplied_env();
    assert_lua(
        &env,
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint()==nil, 'untainted entry')
        local offset, day, position = secretwrap(0), secretwrap(15), secretwrap(1)
        local get, count = C_Calendar.GetDayEvent, C_Calendar.GetNumDayEvents
        local expected = CalendarDayExpected('Host raid',136116)
        CalendarDayOne(expected,get(offset,15,1))
        CalendarDayOne(expected,get(0,day,1))
        CalendarDayOne(expected,get(0,15,position))
        CalendarDayOne(expected,get(offset,day,position))
        CalendarDayCount(1,count(offset,15)); CalendarDayCount(1,count(0,day))
        CalendarDayCount(1,count(offset,day))
        CalendarDayCount(0,count(secretwrap(99),day))
        CalendarDayMissing(get(offset,day,secretwrap(2)))
        for _, value in ipairs({offset,day,position}) do
            assert(issecretvalue(value), 'original wrapper retained')
        end
        assert(secretunwrap(offset)==0 and secretunwrap(day)==15 and secretunwrap(position)==1)
        assert(debug.getstacktaint()==nil, 'caller context preserved')
    "#,
    );
}

#[test]
fn calendar_day_events_tainted_secret_authenticates_every_argument_before_type_or_lookup() {
    let env = supplied_env();
    assert_lua(
        &env,
        r#"
        debug.settaintmode(true)
        local offset, day, position = secretwrap(0), secretwrap(15), secretwrap(1)
        local get, count = C_Calendar.GetDayEvent, C_Calendar.GetNumDayEvents
        local function denied(fn,...)
            local ok, message = pcall(fn,...)
            assert(not ok and type(message)=='string', 'secret denial')
            assert(string.find(message,'untainted caller',1,true),
                'authentication before type/lookup: '..tostring(message))
            assert(debug.getstacktaint()=='CalendarDayProbe', 'caller taint retained')
        end
        local function addon()
            assert(debug.getstacktaint()=='CalendarDayProbe')
            CalendarDayOne(CalendarDayExpected('Host raid',136116),get(0,15,1))
            CalendarDayCount(1,count(0,15))
            denied(get,offset,15,1); denied(get,0,day,1); denied(get,0,15,position)
            denied(get,offset,day,position)
            denied(count,offset,15); denied(count,0,day); denied(count,offset,day)
            -- Malformed earlier arguments must not hide secrets in later slots.
            denied(get,false,day,1); denied(get,nil,15,position)
            denied(get,0,false,position); denied(count,false,day); denied(count,nil,day)
            denied(get,99,day,1); denied(count,99,day)
            for _, value in ipairs({offset,day,position}) do
                assert(issecretvalue(value), 'denial retains wrapper')
                assert(not pcall(secretunwrap,value), 'no declassification')
            end
        end
        debug.setobjecttaint(addon,'CalendarDayProbe')
        addon()
        assert(debug.getstacktaint()==nil, 'untainted caller restored')
        CalendarDayOne(CalendarDayExpected('Host raid',136116),get(offset,day,position))
        CalendarDayCount(1,count(offset,day))
    "#,
    );
}
