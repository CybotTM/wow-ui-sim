//! Integration tests for the unit-liveness probes added to
//! `src/lua_api/globals/group_queries.rs`.

use std::time::Instant;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::MirrorTimer;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── UnitIsDeadOrGhost / UnitIsCorpse ──────────────────────────────────────────

#[test]
fn unit_is_dead_false_for_alive_player() {
    let env = env();
    let b: bool = env.eval(r#"return UnitIsDeadOrGhost("player")"#).unwrap();
    assert!(!b);
}

#[test]
fn unit_is_dead_true_when_player_health_zero() {
    let env = env();
    env.state().borrow_mut().player.health = 0;
    let dead: bool = env.eval(r#"return UnitIsDeadOrGhost("player")"#).unwrap();
    let corpse: bool = env.eval(r#"return UnitIsCorpse("player")"#).unwrap();
    assert!(dead);
    assert!(corpse);
}

#[test]
fn unit_is_civilian_is_available_and_false_for_modeled_units() {
    let env = env();
    let player_civilian: bool = env.eval(r#"return UnitIsCivilian("player")"#).unwrap();
    let target_civilian: bool = env.eval(r#"return UnitIsCivilian("target")"#).unwrap();
    assert!(!player_civilian);
    assert!(!target_civilian);
}

#[test]
fn unit_is_dead_tracks_party_member_dead_since() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.party_group_active = true;
        st.party_members[0].dead_since = Some(Instant::now());
    }
    let b: bool = env.eval(r#"return UnitIsDeadOrGhost("party1")"#).unwrap();
    assert!(b);
}

#[test]
fn unit_is_feign_death_tracks_player_mirror_timer() {
    let env = env();
    env.state().borrow_mut().world.mirror_timers.push(MirrorTimer {
        name: "FEIGNDEATH".into(),
        ..MirrorTimer::default()
    });

    let player_feigning: bool = env.eval(r#"return UnitIsFeignDeath("player")"#).unwrap();
    let target_feigning: bool = env.eval(r#"return UnitIsFeignDeath("target")"#).unwrap();
    assert!(player_feigning);
    assert!(!target_feigning);
}

// ── UnitIsUnconscious ─────────────────────────────────────────────────────────

#[test]
fn unit_is_unconscious_always_false() {
    let env = env();
    let b: bool = env.eval(r#"return UnitIsUnconscious("player")"#).unwrap();
    assert!(!b);
}

// ── UnitHasIncomingResurrection ───────────────────────────────────────────────

#[test]
fn unit_has_incoming_resurrection_tracks_player_pending_offer() {
    let env = env();
    env.state().borrow_mut().pending_resurrect = Some("Priest".into());
    let b: bool = env
        .eval(r#"return UnitHasIncomingResurrection("player")"#)
        .unwrap();
    assert!(b);
}

#[test]
fn unit_has_incoming_resurrection_false_without_offer() {
    let env = env();
    let b: bool = env
        .eval(r#"return UnitHasIncomingResurrection("player")"#)
        .unwrap();
    assert!(!b);
}

#[test]
fn unit_has_incoming_resurrection_false_for_party_member() {
    // Per-unit resurrect tracking is not modelled for party; always false.
    let env = env();
    env.state().borrow_mut().pending_resurrect = Some("Priest".into());
    let b: bool = env
        .eval(r#"return UnitHasIncomingResurrection("party1")"#)
        .unwrap();
    assert!(!b);
}

// ── UnitIsVisible ─────────────────────────────────────────────────────────────

#[test]
fn unit_is_visible_true_for_player() {
    let env = env();
    let b: bool = env.eval(r#"return UnitIsVisible("player")"#).unwrap();
    assert!(b);
}

#[test]
fn unit_is_visible_false_for_empty_target_slot() {
    let env = env();
    env.state().borrow_mut().current_target = None;
    let b: bool = env.eval(r#"return UnitIsVisible("target")"#).unwrap();
    assert!(!b);
}

#[test]
fn unit_is_visible_false_for_empty_string() {
    let env = env();
    let b: bool = env.eval(r#"return UnitIsVisible("")"#).unwrap();
    assert!(!b);
}

#[test]
fn unit_is_visible_true_for_active_party_member() {
    let env = env();
    env.state().borrow_mut().party_group_active = true;
    let b: bool = env.eval(r#"return UnitIsVisible("party1")"#).unwrap();
    assert!(b);
}
