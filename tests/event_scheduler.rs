use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn event_scheduler_defaults_to_hidden_without_events() {
    let env = env();
    let can_show: bool = env
        .eval(
            r#"
            C_EventScheduler._state.canShowEvents = nil
            C_EventScheduler._state.suppressDisplay = false
            C_EventScheduler._state.ongoingEvents = {}
            C_EventScheduler._state.scheduledEvents = {}
            return C_EventScheduler.CanShowEvents()
            "#,
        )
        .unwrap();

    assert!(
        !can_show,
        "scheduler should be hidden when no events are configured"
    );
}

#[test]
fn event_scheduler_starts_with_seeded_events_and_default_visibility() {
    let env = env();
    let (
        can_show,
        ongoing_count,
        scheduled_count,
        first_ongoing_poi,
        first_scheduled_key,
        first_start_time_is_future,
        second_has_reminder,
    ): (bool, i32, i32, i32, String, bool, bool) = env
        .eval(
            r#"
            local canShow = C_EventScheduler.CanShowEvents()
            local ongoing = C_EventScheduler._state.ongoingEvents
            local scheduled = C_EventScheduler._state.scheduledEvents
            return canShow,
                #ongoing,
                #scheduled,
                ongoing[1].areaPoiID,
                scheduled[1].eventKey,
                scheduled[1].startTime > time(),
                scheduled[2].hasReminder
            "#,
        )
        .unwrap();

    assert!(
        can_show,
        "seeded scheduler events should be visible by default"
    );
    assert_eq!(ongoing_count, 2, "expected two seeded ongoing events");
    assert_eq!(scheduled_count, 2, "expected two seeded scheduled events");
    assert_eq!(first_ongoing_poi, 1001);
    assert_eq!(first_scheduled_key, "pvp-brawl-blitz");
    assert!(
        first_start_time_is_future,
        "seeded scheduled events should be upcoming"
    );
    assert!(
        second_has_reminder,
        "seeded scheduled events should include at least one reminder example"
    );
}

#[test]
fn event_scheduler_override_flag_controls_visibility() {
    let env = env();
    let (override_true, override_false): (bool, bool) = env
        .eval(
            r#"
            C_EventScheduler._state.canShowEvents = true
            local whenTrue = C_EventScheduler.CanShowEvents()

            C_EventScheduler._state.canShowEvents = false
            local whenFalse = C_EventScheduler.CanShowEvents()

            return whenTrue, whenFalse
            "#,
        )
        .unwrap();

    assert!(
        override_true,
        "explicit true override should force visibility"
    );
    assert!(
        !override_false,
        "explicit false override should force hidden state"
    );
}

#[test]
fn event_scheduler_derives_visibility_from_event_lists() {
    let env = env();
    let (ongoing_visible, scheduled_visible, suppressed_hidden): (bool, bool, bool) = env
        .eval(
            r#"
            C_EventScheduler._state.canShowEvents = nil
            C_EventScheduler._state.suppressDisplay = false

            C_EventScheduler._state.ongoingEvents = {
                { areaPoiID = 100, rewardsClaimed = false, displayInfo = {} },
            }
            C_EventScheduler._state.scheduledEvents = {}
            local ongoingVisible = C_EventScheduler.CanShowEvents()

            C_EventScheduler._state.ongoingEvents = {}
            C_EventScheduler._state.scheduledEvents = {
                {
                    eventKey = "foo",
                    eventID = 1,
                    areaPoiID = 200,
                    startTime = 10,
                    ["endTime"] = 20,
                    duration = 10,
                    hasReminder = false,
                    rewardsClaimed = false,
                    displayInfo = {},
                },
            }
            local scheduledVisible = C_EventScheduler.CanShowEvents()

            C_EventScheduler._state.suppressDisplay = true
            local suppressedHidden = not C_EventScheduler.CanShowEvents()

            return ongoingVisible, scheduledVisible, suppressedHidden
            "#,
        )
        .unwrap();

    assert!(
        ongoing_visible,
        "ongoing event list should make scheduler visible"
    );
    assert!(
        scheduled_visible,
        "scheduled event list should make scheduler visible"
    );
    assert!(
        suppressed_hidden,
        "suppress flag should hide scheduler when no explicit override is set"
    );
}

#[test]
fn event_scheduler_request_events_repopulates_seeded_state() {
    let env = env();
    let (
        ongoing_count,
        scheduled_count,
        public_ongoing_count,
        first_public_poi,
        public_scheduled_count,
        first_public_event_key,
        has_data,
        can_show,
    ): (i32, i32, i32, i32, i32, String, bool, bool) = env
        .eval(
            r#"
            C_EventScheduler._state.canShowEvents = nil
            C_EventScheduler._state.suppressDisplay = false
            C_EventScheduler._state.ongoingEvents = {}
            C_EventScheduler._state.scheduledEvents = {}

            C_EventScheduler.RequestEvents()

            return #C_EventScheduler._state.ongoingEvents,
                #C_EventScheduler._state.scheduledEvents,
                #C_EventScheduler.GetOngoingEvents(),
                C_EventScheduler.GetOngoingEvents()[1].areaPoiID,
                #C_EventScheduler.GetScheduledEvents(),
                C_EventScheduler.GetScheduledEvents()[1].eventKey,
                C_EventScheduler.HasData(),
                C_EventScheduler.CanShowEvents()
            "#,
        )
        .unwrap();

    assert_eq!(
        ongoing_count, 2,
        "RequestEvents() should repopulate the seeded ongoing event list"
    );
    assert_eq!(
        scheduled_count, 2,
        "RequestEvents() should repopulate the seeded scheduled event list"
    );
    assert_eq!(
        public_ongoing_count, 2,
        "GetOngoingEvents() should expose the repopulated seeded ongoing event list"
    );
    assert_eq!(
        first_public_poi, 1001,
        "GetOngoingEvents() should return the seeded ongoing event records"
    );
    assert_eq!(
        public_scheduled_count, 2,
        "GetScheduledEvents() should expose the repopulated seeded scheduled event list"
    );
    assert_eq!(
        first_public_event_key, "pvp-brawl-blitz",
        "GetScheduledEvents() should return the seeded scheduled event records"
    );
    assert!(
        has_data,
        "HasData() should report true after RequestEvents() restores scheduler state"
    );
    assert!(
        can_show,
        "RequestEvents() should restore visible scheduler data"
    );
}

#[test]
fn event_scheduler_event_zone_names_follow_seeded_area_poi_ids() {
    let env = env();
    let (ongoing_zone, scheduled_zone, missing_zone): (String, String, String) = env
        .eval(
            r#"
            return C_EventScheduler.GetEventZoneName(1001),
                C_EventScheduler.GetEventZoneName(1004),
                C_EventScheduler.GetEventZoneName(999999)
            "#,
        )
        .unwrap();

    assert_eq!(
        ongoing_zone, "Warsong Gulch",
        "GetEventZoneName() should resolve seeded ongoing event POIs"
    );
    assert_eq!(
        scheduled_zone, "Darkmoon Island",
        "GetEventZoneName() should resolve seeded scheduled event POIs"
    );
    assert_eq!(
        missing_zone, "",
        "GetEventZoneName() should keep the empty-string fallback for unknown POIs"
    );
}
