#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use std::time::{Duration, Instant};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn calendar_invite_original_4644_pending_and_throttle_state() {
    let env = WowLuaEnv::new().unwrap();
    let source = original_lines("Blizzard_Calendar", 4644, 4644);
    env.exec(&format!("{source}\nassert(canSendInvite == true)"))
        .unwrap();
    env.exec(
        r#"
        inviteEvents = 0
        local watcher = CreateFrame('Frame')
        watcher:RegisterEvent('CALENDAR_ACTION_PENDING')
        watcher:SetScript('OnEvent', function() inviteEvents = inviteEvents + 1 end)
        assert(select('#', CalendarCanSendInvite(false, {})) == 1)
        assert(type(CalendarCanSendInvite()) == 'boolean')
    "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .wrath_legacy
        .calendar
        .action_pending = true;
    env.exec(&format!("{source}\nassert(canSendInvite == false)"))
        .unwrap();
    {
        let mut sim = env.state().borrow_mut();
        let context = &mut sim.wrath_legacy.calendar;
        context.invite_throttle_bypass = true;
        assert!(!context.can_send_invite_at(5000)); // pending wins over bypass
        context.action_pending = false;
        context.last_invite_time_ms = 4000;
        assert!(context.can_send_invite_at(4000));
        context.invite_throttle_bypass = false;
        assert!(!context.can_send_invite_at(4000));
        assert!(!context.can_send_invite_at(5999));
        assert!(context.can_send_invite_at(6000));
        assert!(context.can_send_invite_at(6001));
        context.last_invite_time_ms = 0;
        assert!(context.can_send_invite_at(0));
        context.player_present = false;
        assert!(context.can_send_invite_at(100)); // no player gate in native wrapper
    }
    // Public getter uses the existing GetTime monotonic clock; far from the boundary.
    {
        let mut sim = env.state().borrow_mut();
        sim.start_time = Instant::now() - Duration::from_secs(10);
        sim.wrath_legacy.calendar.last_invite_time_ms = 1;
    }
    env.exec(&format!("{source}\nassert(canSendInvite == true)"))
        .unwrap();
    env.exec("assert(inviteEvents == 0)").unwrap();
    let sim = env.state().borrow();
    let context = &sim.wrath_legacy.calendar;
    assert_eq!(
        (
            context.action_pending,
            context.invite_throttle_bypass,
            context.last_invite_time_ms
        ),
        (false, false, 1)
    );
}
