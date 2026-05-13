//! Behavior pin: pet battles route the main bar to the current action page.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";
const SEEDED_ACTION_BAR_PAGE: u32 = 4;
const SEEDED_PET_BATTLE_STATE: i32 = 1;

#[test]
fn update_pet_battle_uses_current_action_bar_page() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_pet_battle(env);
        dispatch_action_bar_update(env);
        assert_main_bar_actionpage(env, SEEDED_ACTION_BAR_PAGE as i32);
    });
    }
}

fn seed_pet_battle(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.action_bar_page = SEEDED_ACTION_BAR_PAGE;
    state.pet_battles.battle_state = SEEDED_PET_BATTLE_STATE;
}

fn dispatch_action_bar_update(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        MainActionBar:SetAttribute("actionpage", 99)
        ActionBarController_UpdateAll()
        "#,
    )
    .expect("ActionBarController_UpdateAll must run cleanly");
}

fn assert_main_bar_actionpage(env: &wow_ui_sim::lua_api::WowLuaEnv, expected_page: i32) {
    let (in_pet_battle, action_page, current_page): (bool, i32, i32) = env
        .eval(
            r#"
            return C_PetBattles.IsInBattle(),
                MainActionBar:GetAttribute("actionpage"),
                C_ActionBar.GetActionBarPage()
            "#,
        )
        .expect("post pet-battle actionpage probe must run cleanly");

    assert!(
        in_pet_battle,
        "seeded pet-battle state must make IsInBattle true"
    );
    assert_eq!(
        action_page, expected_page,
        "pet-battle branch chose wrong actionpage; current_page={current_page}"
    );
}
