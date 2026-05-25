#[test]
fn pet_battle_runtime_state_is_not_runtime_bootstrap_fallback() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "__wow_pet_battle_seed_sample",
        "__wow_pet_battle_ensure_active",
        "__wow_pet_battle_get_pet",
        "C_PetBattles.GetAbilityInfo = function",
        "C_PetBattles.GetPVPMatchmakingInfo = function",
        "C_PetBattles.StartPVPDuel = function",
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} must live in the explicit temporary pet-battle runtime workaround boundary, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} must live in the explicit temporary pet-battle runtime workaround boundary, not shared bootstrap"
        );
    }
}
