//! Tests for `C_SummonInfo` and `C_IncomingSummon` probes backed by
//! `SimState.summon_request`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::SummonRequestState;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ── Inactive defaults ────────────────────────────────────────────────────────

#[test]
fn get_summon_reason_returns_nil_when_inactive() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_SummonInfo.GetSummonReason() == nil")
        .unwrap();
    assert!(
        is_nil,
        "GetSummonReason() should be nil when no summon is active"
    );
}

#[test]
fn get_summon_confirm_time_left_returns_zero_when_inactive() {
    let env = env();
    let time_left: i32 = env
        .eval("return C_SummonInfo.GetSummonConfirmTimeLeft()")
        .unwrap();
    assert_eq!(time_left, 0);
}

#[test]
fn is_summon_skipping_start_experience_false_when_inactive() {
    let env = env();
    let flag: bool = env
        .eval("return C_SummonInfo.IsSummonSkippingStartExperience()")
        .unwrap();
    assert!(!flag);
}

#[test]
fn has_incoming_summon_false_for_player_when_inactive() {
    let env = env();
    let result: bool = env
        .eval("return C_IncomingSummon.HasIncomingSummon('player')")
        .unwrap();
    assert!(!result);
}

#[test]
fn incoming_summon_status_zero_when_inactive() {
    let env = env();
    let status: i32 = env
        .eval("return C_IncomingSummon.IncomingSummonStatus('player')")
        .unwrap();
    assert_eq!(status, 0);
}

// ── Active summon ─────────────────────────────────────────────────────────────

#[test]
fn get_summon_reason_returns_seeded_reason() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.summon_request = SummonRequestState {
            active: true,
            reason: 3,
            time_left_ms: 60000,
            skips_start_experience: false,
            target_name: "Thrall".into(),
        };
    }
    let reason: i32 = env.eval("return C_SummonInfo.GetSummonReason()").unwrap();
    assert_eq!(reason, 3);
}

#[test]
fn get_summon_confirm_time_left_reflects_mutation() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.summon_request.active = true;
        sim.summon_request.time_left_ms = 45000;
    }
    let ms: i32 = env
        .eval("return C_SummonInfo.GetSummonConfirmTimeLeft()")
        .unwrap();
    assert_eq!(ms, 45000);
}

#[test]
fn is_summon_skipping_start_experience_reflects_flag() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.summon_request.active = true;
        sim.summon_request.skips_start_experience = true;
    }
    let flag: bool = env
        .eval("return C_SummonInfo.IsSummonSkippingStartExperience()")
        .unwrap();
    assert!(flag);
}

#[test]
fn has_incoming_summon_true_for_player_when_active() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.summon_request.active = true;
    }
    let result: bool = env
        .eval("return C_IncomingSummon.HasIncomingSummon('player')")
        .unwrap();
    assert!(result);
}

#[test]
fn has_incoming_summon_false_for_non_player_unit_when_active() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.summon_request.active = true;
    }
    let result: bool = env
        .eval("return C_IncomingSummon.HasIncomingSummon('target')")
        .unwrap();
    assert!(!result, "HasIncomingSummon only returns true for 'player'");
}

#[test]
fn incoming_summon_status_pending_for_player_when_active() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.summon_request.active = true;
    }
    let status: i32 = env
        .eval("return C_IncomingSummon.IncomingSummonStatus('player')")
        .unwrap();
    assert_eq!(status, 1, "status 1 = pending");
}

#[test]
fn incoming_summon_status_zero_for_non_player_even_when_active() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.summon_request.active = true;
    }
    let status: i32 = env
        .eval("return C_IncomingSummon.IncomingSummonStatus('focus')")
        .unwrap();
    assert_eq!(status, 0);
}
