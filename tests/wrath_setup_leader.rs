//! Wrath's original target frame reflects the supplied party leader.
#![cfg(feature = "client-wrath")]

#[path = "common/wrath_full_ui.rs"]
mod wrath_full_ui;

#[test]
fn original_target_frame_tracks_party_leadership() {
    let env = wrath_full_ui::load_original_wrath_startup();
    env.exec(
        r#"
        A_Admin.SetPartySize(1)
        A_Admin.SetPartyLeader(0)
        TargetFrame.unit = "player"
        TargetFrame.showLeader = true
        TargetFrame_Update(TargetFrame)
        assert(TargetFrame.leaderIcon:IsShown(), "player leader must show icon")
        A_Admin.SetPartyLeader(1)
        TargetFrame_Update(TargetFrame)
        assert(not TargetFrame.leaderIcon:IsShown(), "party leader must hide player icon")
        "#,
    )
    .expect("original Wrath target update must track party leadership");
}
