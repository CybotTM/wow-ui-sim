//! Public query shapes after all bootstrap passes; original-consumer tests cover events.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn super_track_targets_round_trip_and_clear() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(C_SuperTrack.GetSuperTrackedQuestID() == nil)
        assert(select('#', C_SuperTrack.GetSuperTrackedContent()) == 0)
        assert(select('#', C_SuperTrack.GetSuperTrackedMapPin()) == 0)
        assert(C_SuperTrack.GetSuperTrackedVignette() == nil)
        assert(C_SuperTrack.GetHighestPrioritySuperTrackingType() == nil)
        assert(not C_SuperTrack.IsSuperTrackingAnything())
        C_SuperTrack.SetSuperTrackedQuestID(42)
        assert(C_SuperTrack.GetSuperTrackedQuestID() == 42)
        assert(C_SuperTrack.IsSuperTrackingAnything())
        C_SuperTrack.SetSuperTrackedQuestID(43)
        assert(C_SuperTrack.GetSuperTrackedQuestID() == 43)
        C_SuperTrack.SetSuperTrackedContent(0, 101)
        local kind, id = C_SuperTrack.GetSuperTrackedContent()
        assert(kind == 0 and id == 101)
        C_SuperTrack.ClearAllSuperTracked()
        assert(C_SuperTrack.GetSuperTrackedQuestID() == nil)
        assert(select('#', C_SuperTrack.GetSuperTrackedContent()) == 0)
        assert(not C_SuperTrack.IsSuperTrackingAnything())
    "#,
    )
    .unwrap();
}

#[test]
fn supplied_map_pin_and_vignette_are_observable_and_clearable() {
    let env = WowLuaEnv::new().unwrap();
    {
        let mut state = env.state().borrow_mut();
        state.super_track.map_pin = Some((0, 201));
        state.super_track.vignette = Some("Vignette-fixture-1".into());
    }
    env.exec(
        r#"
        local kind, id = C_SuperTrack.GetSuperTrackedMapPin()
        assert(kind == 0 and id == 201)
        assert(C_SuperTrack.GetSuperTrackedVignette() == 'Vignette-fixture-1')
        assert(C_SuperTrack.IsSuperTrackingAnything())
        C_SuperTrack.ClearSuperTrackedMapPin()
        assert(select('#', C_SuperTrack.GetSuperTrackedMapPin()) == 0)
        assert(C_SuperTrack.IsSuperTrackingAnything())
        C_SuperTrack.ClearAllSuperTracked()
        assert(C_SuperTrack.GetSuperTrackedVignette() == nil)
        assert(not C_SuperTrack.IsSuperTrackingAnything())
    "#,
    )
    .unwrap();
}

#[test]
fn unsupplied_highlights_fail_explicitly() {
    let env = WowLuaEnv::new().unwrap();
    for quest_id in [0, 999999] {
        let error = env
            .exec(&format!(
                "C_QuestLog.GetQuestAdditionalHighlights({quest_id})"
            ))
            .unwrap_err();
        assert!(error.to_string().contains("no supplied row"), "{error}");
    }
}
