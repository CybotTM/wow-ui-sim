//! Integration tests for `AchievementDisplayMixin:SetAchievements`.
//!
//! Drives `Blizzard_AlliedRacesFrameUI:UpdateModel`
//! (`Blizzard_AlliedRacesFrameUI.lua:73`):
//!
//! ```lua
//! self.RaceInfoFrame.ScrollFrame.Child.ObjectivesFrame
//!     :SetAchievements(raceInfo.achievementIds);
//! ```
//!
//! Real WoW renders one bullet row per achievement criterion via a
//! frame pool keyed off `GetAchievementInfo`. The simulator skips the
//! bullet rendering — the panel is already cosmetic without 3D models —
//! and only records the ID list so addons that round-trip
//! `self.achievementIds` see what they wrote.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn achievement_display_mixin_table_exists() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env.eval("return type(AchievementDisplayMixin)").unwrap();
    assert_eq!(
        kind, "table",
        "AchievementDisplayMixin must be defined so AlliedRacesUI can mix it into ObjectivesFrame"
    );
}

#[test]
fn set_achievements_method_exists_on_mixin() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(AchievementDisplayMixin.SetAchievements)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn set_achievements_records_id_list_on_mixed_in_frame() {
    let env = WowLuaEnv::new().expect("env");
    let id_one: f64 = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "TestAchievementDisplayRecords")
            Mixin(frame, AchievementDisplayMixin)
            frame:SetAchievements({ 12345, 67890, 11111 })
            ACHIEVEMENT_RECORDS_FRAME = frame
            return frame.achievementIds[1]
            "#,
        )
        .unwrap();
    let id_two: f64 = env
        .eval("return ACHIEVEMENT_RECORDS_FRAME.achievementIds[2]")
        .unwrap();
    let id_three: f64 = env
        .eval("return ACHIEVEMENT_RECORDS_FRAME.achievementIds[3]")
        .unwrap();
    assert_eq!(
        (id_one as i64, id_two as i64, id_three as i64),
        (12345, 67890, 11111),
        "SetAchievements must store the list verbatim so callers can read it back"
    );
}

#[test]
fn set_achievements_accepts_empty_list_without_error() {
    let env = WowLuaEnv::new().expect("env");
    let stored_count: f64 = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "TestAchievementDisplayEmpty")
            Mixin(frame, AchievementDisplayMixin)
            frame:SetAchievements({})
            return #frame.achievementIds
            "#,
        )
        .unwrap();
    assert_eq!(
        stored_count as i64, 0,
        "an empty list is a valid achievement set and must not error"
    );
}

#[test]
fn set_achievements_overwrites_previous_list() {
    let env = WowLuaEnv::new().expect("env");
    let head_id: f64 = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "TestAchievementDisplayOverwrite")
            Mixin(frame, AchievementDisplayMixin)
            frame:SetAchievements({ 1, 2, 3 })
            frame:SetAchievements({ 99 })
            ACHIEVEMENT_OVERWRITE_FRAME = frame
            return frame.achievementIds[1]
            "#,
        )
        .unwrap();
    assert_eq!(
        head_id as i64, 99,
        "consecutive calls must overwrite — UpdateModel rebinds achievements per race"
    );
    let stale_index_two: bool = env
        .eval("return ACHIEVEMENT_OVERWRITE_FRAME.achievementIds[2] == nil")
        .unwrap();
    assert!(
        stale_index_two,
        "the new list has one entry; index 2 must be nil rather than retaining the stale value"
    );
}

#[test]
fn allied_races_objectives_frame_call_pattern_runs_without_error() {
    // Mirrors AlliedRacesFrameMixin:UpdateModel's call site:
    // self.RaceInfoFrame.ScrollFrame.Child.ObjectivesFrame:SetAchievements(ids).
    // The ObjectivesFrame is only set up once Blizzard_AlliedRacesUI
    // loads; this test stands in for the bare-mixin invocation.
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval(
            r#"
            local objectivesFrame = CreateFrame("Frame", "TestAchievementDisplayObjectivesFrame")
            Mixin(objectivesFrame, AchievementDisplayMixin)
            local raceInfo = { achievementIds = { 14012, 14013 } }
            objectivesFrame:SetAchievements(raceInfo.achievementIds)
            return #objectivesFrame.achievementIds
            "#,
        )
        .unwrap();
    assert_eq!(
        count as i64, 2,
        "the AlliedRaces UpdateModel call site must complete and the list must round-trip"
    );
}
