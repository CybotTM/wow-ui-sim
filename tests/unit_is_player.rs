//! `UnitIsPlayer` resolved against SimState — exercise every token shape.

use wow_ui_sim::lua_api::WowLuaEnv;

fn is_player(env: &WowLuaEnv, token: &str) -> bool {
    env.eval::<bool>(&format!("return UnitIsPlayer({:?})", token))
        .expect("UnitIsPlayer should return a bool")
}

#[test]
fn player_and_self_are_always_players() {
    let env = WowLuaEnv::new().unwrap();
    assert!(is_player(&env, "player"));
    assert!(is_player(&env, "self"));
    assert!(is_player(&env, "PLAYER"), "resolution is case-insensitive");
}

#[test]
fn default_party_members_are_all_players() {
    // SimState::default() seeds 4 party members. PartyMember records are
    // player-characters by definition.
    let env = WowLuaEnv::new().unwrap();
    assert!(is_player(&env, "party1"));
    assert!(is_player(&env, "party2"));
    assert!(is_player(&env, "party3"));
    assert!(is_player(&env, "party4"));
}

#[test]
fn party_slots_track_populated_roster_after_resize() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPartySize(2)").unwrap();
    assert!(is_player(&env, "party1"));
    assert!(is_player(&env, "party2"));
    assert!(!is_player(&env, "party3"), "slot 3 not populated");
    assert!(!is_player(&env, "party4"));
}

#[test]
fn party_slot_0_and_5_are_not_players() {
    let env = WowLuaEnv::new().unwrap();
    // WoW only has party1..party4 — out-of-range tokens must be false.
    assert!(!is_player(&env, "party0"));
    assert!(!is_player(&env, "party5"));
}

#[test]
fn partypet_is_not_a_player() {
    let env = WowLuaEnv::new().unwrap();
    assert!(!is_player(&env, "partypet"));
    assert!(!is_player(&env, "partypet1"));
}

#[test]
fn raid_tokens_are_not_players_in_current_sim() {
    let env = WowLuaEnv::new().unwrap();
    // Sim has no raid roster — the pre-refactor regex returned true for
    // any raidN match, which was a lie. Now false is honest.
    assert!(!is_player(&env, "raid1"));
    assert!(!is_player(&env, "raid40"));
}

#[test]
fn non_string_args_return_false() {
    let env = WowLuaEnv::new().unwrap();
    assert!(!env.eval::<bool>("return UnitIsPlayer(nil)").unwrap());
    assert!(!env.eval::<bool>("return UnitIsPlayer(42)").unwrap());
    assert!(!env.eval::<bool>("return UnitIsPlayer(true)").unwrap());
    assert!(!env.eval::<bool>("return UnitIsPlayer({})").unwrap());
}

#[test]
fn mouseover_pet_and_random_tokens_are_not_players() {
    let env = WowLuaEnv::new().unwrap();
    assert!(!is_player(&env, "mouseover"));
    assert!(!is_player(&env, "pet"));
    assert!(!is_player(&env, "vehicle"));
    assert!(!is_player(&env, "targettarget"));
    assert!(!is_player(&env, "nameplate1"));
    assert!(!is_player(&env, "not-a-real-unit"));
}

#[test]
fn target_resolves_from_current_target_is_player_flag() {
    let env = WowLuaEnv::new().unwrap();
    // No target by default.
    assert!(!is_player(&env, "target"));

    // A_Admin.SetTarget(name, level, classIndex, isEnemy) — isEnemy=false
    // means the target is a friendly player character.
    env.exec(r#"A_Admin.SetTarget("Thrall", 60, 1, false)"#)
        .expect("SetTarget should succeed");
    assert!(
        is_player(&env, "target"),
        "friendly target (is_enemy=false) should report as player",
    );

    // Hostile NPC target.
    env.exec(r#"A_Admin.SetTarget("Training Dummy", 60, 1, true)"#)
        .unwrap();
    assert!(
        !is_player(&env, "target"),
        "enemy target should not report as player",
    );
}
