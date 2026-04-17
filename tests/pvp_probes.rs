//! Integration tests for `src/lua_api/globals/pvp_probes.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── IsInActiveWorldPVP ────────────────────────────────────────────────────────

#[test]
fn is_in_active_world_pvp_false_for_contested() {
    let env = env();
    env.state().borrow_mut().world.pvp_type = "contested".into();
    let b: bool = env.eval("return IsInActiveWorldPVP()").unwrap();
    assert!(!b);
}

#[test]
fn is_in_active_world_pvp_true_for_combat_pvp_type() {
    let env = env();
    env.state().borrow_mut().world.pvp_type = "combat".into();
    let b: bool = env.eval("return IsInActiveWorldPVP()").unwrap();
    assert!(b);
}

#[test]
fn is_in_active_world_pvp_true_for_hostile_and_arena() {
    let env = env();
    env.state().borrow_mut().world.pvp_type = "hostile".into();
    assert!(env.eval::<bool>("return IsInActiveWorldPVP()").unwrap());
    env.state().borrow_mut().world.pvp_type = "arena".into();
    assert!(env.eval::<bool>("return IsInActiveWorldPVP()").unwrap());
}

// ── GetPVPDesired ─────────────────────────────────────────────────────────────

#[test]
fn get_pvp_desired_reads_player_pvp_enabled() {
    let env = env();
    let b: bool = env.eval("return GetPVPDesired()").unwrap();
    assert!(!b);
    env.state().borrow_mut().player.pvp_enabled = true;
    let b: bool = env.eval("return GetPVPDesired()").unwrap();
    assert!(b);
}

// ── GetPVPLastHonorGain ───────────────────────────────────────────────────────

#[test]
fn get_pvp_last_honor_gain_default_zero() {
    let env = env();
    let honor: i64 = env.eval("return GetPVPLastHonorGain()").unwrap();
    assert_eq!(honor, 0);
}

#[test]
fn get_pvp_last_honor_gain_reads_state_field() {
    let env = env();
    env.state().borrow_mut().pvp_last_honor_gain = 375;
    let honor: i64 = env.eval("return GetPVPLastHonorGain()").unwrap();
    assert_eq!(honor, 375);
}

// ── IsSubZonePVP ──────────────────────────────────────────────────────────────

#[test]
fn is_sub_zone_pvp_reads_world_flag() {
    let env = env();
    let b: bool = env.eval("return IsSubZonePVP()").unwrap();
    assert!(!b);
    env.state().borrow_mut().world.is_sub_zone_pvp = true;
    let b: bool = env.eval("return IsSubZonePVP()").unwrap();
    assert!(b);
}
