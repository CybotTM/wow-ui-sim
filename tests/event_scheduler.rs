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
