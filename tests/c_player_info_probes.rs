//! Tests for `C_PlayerInfo` probes backed by `SimState.player`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{MythicPlusRatingMapSummary, MythicPlusRatingSummary};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ── GetAlternateFormInfo ──────────────────────────────────────────────────────

#[test]
fn get_alternate_form_info_default_not_in_form() {
    let env = env();
    let (has, in_form): (bool, bool) = env
        .eval("return C_PlayerInfo.GetAlternateFormInfo()")
        .unwrap();
    assert!(!has, "default: player has no alternate form");
    assert!(in_form, "default: alternate_form_is_default=true");
}

#[test]
fn get_alternate_form_info_reflects_mutation() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.player.is_alternate_form = true;
        state.player.alternate_form_is_default = false;
    }
    let (has, in_form): (bool, bool) = env
        .eval("return C_PlayerInfo.GetAlternateFormInfo()")
        .unwrap();
    assert!(has);
    assert!(!in_form);
}

// ── GetContentDifficultyCreatureForPlayer ─────────────────────────────────────

#[test]
fn get_content_difficulty_creature_returns_equal() {
    let env = env();
    // Enum.RelativeContentDifficulty.Equal = 2
    let difficulty: i32 = env
        .eval(r#"return C_PlayerInfo.GetContentDifficultyCreatureForPlayer("player")"#)
        .unwrap();
    assert_eq!(difficulty, 2, "default difficulty is equal (2)");
}

// ── GetPlayerMythicPlusRatingSummary ─────────────────────────────────────────

#[test]
fn get_player_mythic_plus_rating_summary_nil_by_default() {
    let env = env();
    let count: i32 = env
        .eval(r#"return select('#', C_PlayerInfo.GetPlayerMythicPlusRatingSummary("player"))"#)
        .unwrap();
    assert_eq!(count, 0, "no rating data by default — returns nothing");
}

#[test]
fn get_player_mythic_plus_rating_summary_seeded() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.player.mythic_plus_rating_summary = Some(MythicPlusRatingSummary {
            current_season_score: 1234.5,
            runs: vec![MythicPlusRatingMapSummary {
                challenge_mode_id: 399,
                map_score: 200.0,
                best_run_level: 15,
                best_run_duration_ms: 1_800_000,
                finished_success: true,
            }],
        });
    }
    let (score, run_count, map_id, map_score, level, success): (f64, i32, i32, f64, i32, bool) =
        env.eval(
            r#"
            local s = C_PlayerInfo.GetPlayerMythicPlusRatingSummary("player")
            return s.currentSeasonScore,
                   #s.runs,
                   s.runs[1].challengeModeID,
                   s.runs[1].mapScore,
                   s.runs[1].bestRunLevel,
                   s.runs[1].finishedSuccess
            "#,
        )
        .unwrap();
    assert!((score - 1234.5).abs() < 0.01);
    assert_eq!(run_count, 1);
    assert_eq!(map_id, 399);
    assert!((map_score - 200.0).abs() < 0.01);
    assert_eq!(level, 15);
    assert!(success);
}

// ── IsPlayerEligibleForNPE ────────────────────────────────────────────────────

#[test]
fn is_player_eligible_for_npe_default_false() {
    let env = env();
    let (eligible, reason): (bool, String) = env
        .eval("return C_PlayerInfo.IsPlayerEligibleForNPE()")
        .unwrap();
    assert!(!eligible, "default: not eligible for NPE");
    assert!(!reason.is_empty(), "failure reason non-empty when ineligible");
}

#[test]
fn is_player_eligible_for_npe_reflects_mutation() {
    let env = env();
    env.state().borrow_mut().player.is_npe_eligible = true;
    let (eligible, _reason): (bool, String) = env
        .eval("return C_PlayerInfo.IsPlayerEligibleForNPE()")
        .unwrap();
    assert!(eligible);
}

// ── IsPlayerNPERestricted ─────────────────────────────────────────────────────

#[test]
fn is_player_npe_restricted_default_false() {
    let env = env();
    let restricted: bool = env
        .eval("return C_PlayerInfo.IsPlayerNPERestricted()")
        .unwrap();
    assert!(!restricted);
}

#[test]
fn is_player_npe_restricted_reflects_mutation() {
    let env = env();
    env.state().borrow_mut().player.is_npe_restricted = true;
    let restricted: bool = env
        .eval("return C_PlayerInfo.IsPlayerNPERestricted()")
        .unwrap();
    assert!(restricted);
}

// ── IsPlayerInRPE ─────────────────────────────────────────────────────────────

#[test]
fn is_player_in_rpe_default_false() {
    let env = env();
    let in_rpe: bool = env
        .eval("return C_PlayerInfo.IsPlayerInRPE()")
        .unwrap();
    assert!(!in_rpe);
}

#[test]
fn is_player_in_rpe_reflects_mutation() {
    let env = env();
    env.state().borrow_mut().player.is_in_rpe = true;
    let in_rpe: bool = env
        .eval("return C_PlayerInfo.IsPlayerInRPE()")
        .unwrap();
    assert!(in_rpe);
}
