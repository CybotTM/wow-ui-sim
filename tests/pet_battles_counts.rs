//! `C_PetBattles.GetNumPets` + `C_PetBattles.GetBattleState` — the two
//! probes the PLAN calls out, SimState-backed.
//!
//! NOTE: `tests/pet_battles.rs` is a separate aspirational test file for
//! the full C_PetBattles engine (IsInBattle, GetActivePet, UseAbility, ...)
//! — those stubs don't exist yet and the tests were failing before this
//! PLAN item. This file scopes to the two probes we do implement.

use wow_ui_sim::lua_api::WowLuaEnv;

fn state(env: &WowLuaEnv) -> (i64, i64, i64) {
    env.eval(
        r#"
        return C_PetBattles.GetNumPets(1),
               C_PetBattles.GetNumPets(2),
               C_PetBattles.GetBattleState()
        "#,
    )
    .unwrap()
}

#[test]
fn defaults_all_zero() {
    let env = WowLuaEnv::new().unwrap();
    assert_eq!(state(&env), (0, 0, 0));
}

#[test]
fn admin_set_pet_battle_counts_drives_both_owners() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPetBattleCounts(3, 4)").unwrap();
    let (p, e, _) = state(&env);
    assert_eq!(p, 3);
    assert_eq!(e, 4);
}

#[test]
fn admin_set_pet_battle_counts_missing_args_default_to_zero() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPetBattleCounts(5)").unwrap();
    let (p, e, _) = state(&env);
    assert_eq!(p, 5);
    assert_eq!(e, 0, "missing enemy arg should default to 0");
}

#[test]
fn admin_set_pet_battle_state_drives_getter() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPetBattleState(3)").unwrap();
    let (_, _, s) = state(&env);
    assert_eq!(s, 3);
}

#[test]
fn get_num_pets_for_unknown_owner_is_zero() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPetBattleCounts(3, 4)").unwrap();
    let other: i64 = env.eval("return C_PetBattles.GetNumPets(0)").unwrap();
    let weird: i64 = env.eval("return C_PetBattles.GetNumPets(99)").unwrap();
    assert_eq!(other, 0, "owner 0 is not a valid side → 0");
    assert_eq!(weird, 0, "unknown owner → 0");
}

#[test]
fn negative_counts_clamp_to_zero() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPetBattleCounts(-1, -5)").unwrap();
    let (p, e, _) = state(&env);
    assert_eq!(p, 0);
    assert_eq!(e, 0);
}

#[test]
fn get_active_pet_stub_still_resolves() {
    // Sanity: the earlier __wow_merge_namespace pass stubbed a handful of
    // other C_PetBattles members. Our Rust registration only overrides
    // GetNumPets + GetBattleState, so the pre-existing GetActivePet stub
    // should still be callable.
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            if type(C_PetBattles.GetActivePet) ~= "function" then
                return "missing_get_active_pet"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}
