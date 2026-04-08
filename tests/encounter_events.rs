use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn encounter_events_exposes_default_event_catalog() {
    let env = env();
    let (has_events, first_exists, info_matches): (bool, bool, bool) = env
        .eval(
            r#"
            local ids = C_EncounterEvents.GetEventList()
            if #ids == 0 then
                return false, false, false
            end
            local first = ids[1]
            local info = C_EncounterEvents.GetEventInfo(first)
            return #ids > 0, C_EncounterEvents.HasEventInfo(first), info and info.encounterEventID == first
            "#,
        )
        .unwrap();

    assert!(
        has_events,
        "encounter event list should include default records"
    );
    assert!(first_exists, "default encounter event IDs should resolve");
    assert!(
        info_matches,
        "GetEventInfo should return matching encounterEventID field"
    );
}

#[test]
fn encounter_events_color_override_round_trip_and_clear() {
    let env = env();
    let (r, g, b, info_has_color, color_cleared): (f64, f64, f64, bool, bool) = env
        .eval(
            r#"
            local eventID = C_EncounterEvents.GetEventList()[1]
            C_EncounterEvents.SetEventColor(eventID, { r = 0.11, g = 0.22, b = 0.33 })
            local color = C_EncounterEvents.GetEventColor(eventID)
            local info = C_EncounterEvents.GetEventInfo(eventID)
            local infoHasColor = info and info.color and info.color.r == 0.11 and info.color.g == 0.22 and info.color.b == 0.33
            C_EncounterEvents.SetEventColor(eventID, nil)
            local colorCleared = C_EncounterEvents.GetEventColor(eventID) == nil and C_EncounterEvents.GetEventInfo(eventID).color == nil
            return color.r, color.g, color.b, infoHasColor, colorCleared
            "#,
        )
        .unwrap();

    assert!((r - 0.11).abs() < 0.0001, "color.r should round-trip");
    assert!((g - 0.22).abs() < 0.0001, "color.g should round-trip");
    assert!((b - 0.33).abs() < 0.0001, "color.b should round-trip");
    assert!(info_has_color, "GetEventInfo should reflect override color");
    assert!(color_cleared, "clearing override should remove color");
}

#[test]
fn encounter_events_sound_override_and_playback_handle() {
    let env = env();
    let (file, channel, volume, handle1, handle2, cleared): (i32, String, f64, i32, i32, bool) = env
        .eval(
            r#"
            local eventID = C_EncounterEvents.GetEventList()[1]
            local trigger = 0
            C_EncounterEvents.SetEventSound(eventID, trigger, { file = 12345, channel = "SFX", volume = 0.75 })
            local sound = C_EncounterEvents.GetEventSound(eventID, trigger)
            local handle1 = C_EncounterEvents.PlayEventSound(eventID, trigger)
            local handle2 = C_EncounterEvents.PlayEventSound(eventID, trigger)
            C_EncounterEvents.SetEventSound(eventID, trigger, nil)
            local cleared = C_EncounterEvents.GetEventSound(eventID, trigger) == nil and C_EncounterEvents.PlayEventSound(eventID, trigger) == nil
            return sound.file, sound.channel, sound.volume, handle1, handle2, cleared
            "#,
        )
        .unwrap();

    assert_eq!(file, 12345, "sound file should round-trip");
    assert_eq!(channel, "SFX", "sound channel should round-trip");
    assert!(
        (volume - 0.75).abs() < 0.0001,
        "sound volume should round-trip"
    );
    assert!(
        handle1 > 0,
        "first PlayEventSound should return fake handle"
    );
    assert_eq!(
        handle2,
        handle1 + 1,
        "PlayEventSound handles should increment"
    );
    assert!(
        cleared,
        "clearing override should remove sound and playback"
    );
}

#[test]
fn encounter_events_invalid_ids_are_ignored() {
    let env = env();
    let (exists, info_nil, color_nil, sound_nil): (bool, bool, bool, bool) = env
        .eval(
            r#"
            C_EncounterEvents.SetEventColor("not-a-number", { r = 1, g = 1, b = 1 })
            C_EncounterEvents.SetEventSound("not-a-number", "0", { file = 42 })
            return C_EncounterEvents.HasEventInfo("not-a-number"),
                   C_EncounterEvents.GetEventInfo("not-a-number") == nil,
                   C_EncounterEvents.GetEventColor("not-a-number") == nil,
                   C_EncounterEvents.GetEventSound("not-a-number", "0") == nil
            "#,
        )
        .unwrap();

    assert!(!exists, "invalid event IDs should not resolve");
    assert!(info_nil, "invalid event IDs should return nil info");
    assert!(color_nil, "invalid event IDs should return nil color");
    assert!(sound_nil, "invalid event IDs should return nil sound");
}
