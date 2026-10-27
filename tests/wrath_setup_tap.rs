//! Wrath's original faction consumer reflects supplied unit tap facts.
#![cfg(feature = "client-wrath")]

#[path = "common/wrath_full_ui.rs"]
mod wrath_full_ui;

use wow_ui_sim::lua_api::state::WrathUnitTapState;

#[test]
fn original_target_frame_tracks_tap_ownership() {
    let env = wrath_full_ui::load_original_wrath_startup();
    env.exec("A_Admin.SetTarget('Creature', 20, 1, true); TargetFrame.unit = 'target'")
        .unwrap();
    let guid = env
        .state()
        .borrow()
        .current_target
        .as_ref()
        .unwrap()
        .guid
        .clone();
    let cases = [
        (
            WrathUnitTapState {
                tapped: true,
                by_player: false,
                by_all_threat_list: false,
            },
            0.5,
        ),
        (
            WrathUnitTapState {
                tapped: false,
                by_player: false,
                by_all_threat_list: false,
            },
            1.0,
        ),
        (
            WrathUnitTapState {
                tapped: true,
                by_player: true,
                by_all_threat_list: false,
            },
            1.0,
        ),
        (
            WrathUnitTapState {
                tapped: true,
                by_player: false,
                by_all_threat_list: true,
            },
            1.0,
        ),
        (
            WrathUnitTapState {
                tapped: true,
                by_player: false,
                by_all_threat_list: false,
            },
            0.5,
        ),
    ];
    for (tap, red) in cases {
        env.state()
            .borrow_mut()
            .wrath_unit_tap_states
            .insert(guid.clone(), tap);
        env.exec(&format!(
            r#"
            TargetFrame_CheckFaction(TargetFrame)
            local r, g, b = TargetFrame.portrait:GetVertexColor()
            assert(r == {red} and g == {red} and b == {red}, 'tap portrait color')
        "#
        ))
        .expect("original Wrath faction consumer must project tap state");
    }
}
