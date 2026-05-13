//! Behavior pin: rested XP uses the blue rested status-bar atlas.
//!
//! `IsResting()` is the player-world flag, but retail
//! `ExhaustionTickMixin:UpdateExhaustionColor` selects the texture from
//! `GetRestState()` (`1` = rested, `2` = normal).

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";
const EXP_BAR_LUA: &str = "StatusTrackingBarManager.MainStatusTrackingBarContainer\
    .bars[StatusTrackingBarInfo.BarsEnum.Experience]";
const NORMAL_REST_STATE: i32 = 2;
const RESTED_REST_STATE: i32 = 1;
const UNRESTED_ATLAS: &str = "UI-HUD-ExperienceBar-Fill-Experience";
const RESTED_ATLAS: &str = "UI-HUD-ExperienceBar-Fill-Rested";

#[test]
fn resting_player_uses_rested_xp_bar_atlas() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_normal_rest_state(env);
        update_exhaustion_color(env);
        assert_status_bar_atlas(env, UNRESTED_ATLAS);

        seed_rested_player(env);
        update_exhaustion_color(env);

        assert_rested_globals(env);
        assert_status_bar_atlas(env, RESTED_ATLAS);
    });
    }
}

fn seed_normal_rest_state(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.player.is_resting = false;
    state.player_xp.rest_state = NORMAL_REST_STATE;
    state.player_xp.rest_state_name = "Normal".to_string();
    state.player_xp.rest_multiplier = 1.0;
}

fn seed_rested_player(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.player.is_resting = true;
    state.player_xp.rest_state = RESTED_REST_STATE;
    state.player_xp.rest_state_name = "Rested".to_string();
    state.player_xp.rest_multiplier = 1.5;
}

fn update_exhaustion_color(env: &WowLuaEnv) {
    env.exec(&format!(
        "{EXP_BAR_LUA}.ExhaustionTick:UpdateExhaustionColor()"
    ))
    .expect("UpdateExhaustionColor must run cleanly");
}

fn assert_rested_globals(env: &WowLuaEnv) {
    let (is_resting, rest_state, rest_multiplier): (bool, i64, f64) = env
        .eval("local state, _, multiplier = GetRestState(); return IsResting(), state, multiplier")
        .expect("rested globals probe must run cleanly");
    assert!(
        is_resting,
        "IsResting() must reflect the seeded player flag"
    );
    assert_eq!(
        rest_state, RESTED_REST_STATE as i64,
        "GetRestState() must return Rested before texture selection"
    );
    assert_eq!(
        rest_multiplier, 1.5,
        "GetRestState() must expose the rested XP multiplier"
    );
}

fn assert_status_bar_atlas(env: &WowLuaEnv, expected_atlas: &str) {
    let atlas: String = env
        .eval(&format!(
            "return {EXP_BAR_LUA}.StatusBar:GetStatusBarTexture():GetAtlas()"
        ))
        .expect("XP status-bar atlas probe must run cleanly");
    assert_eq!(
        atlas, expected_atlas,
        "UpdateExhaustionColor must select {expected_atlas}"
    );
}
