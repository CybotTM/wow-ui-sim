#![cfg(feature = "retail-12-0-5")]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::game_data::TargetInfo;

fn seed_vehicle_ui_env(player_vehicle_ui: bool, target_vehicle_ui: bool) -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("create vehicle macro environment");
    {
        let mut sim = env.state().borrow_mut();
        sim.player.has_vehicle_ui = player_vehicle_ui;
        sim.current_target = Some(TargetInfo {
            unit_id: "target".into(),
            name: "Independent Vehicle Target".into(),
            class_index: 1,
            level: 60,
            health: 100,
            health_max: 100,
            power: 0,
            power_max: 100,
            power_type: 1,
            power_type_name: "RAGE".into(),
            is_player: false,
            is_enemy: true,
            has_vehicle_ui: target_vehicle_ui,
            guid: "Creature-0-0-0-0-448-000042".into(),
            classification: "normal".into(),
            creature_type: "Humanoid".into(),
            reaction: 2,
            interaction: Default::default(),
        });
    }
    env
}

fn assert_vehicle_ui_options(env: &WowLuaEnv, expected_player: &str, expected_target: &str) {
    let actual: (String, String) = env
        .eval(
            r#"
            local player = SecureCmdOptionParse('[vehicleui]yes;no')
            local target = SecureCmdOptionParse('[@target,unithasvehicleui]yes;no')
            return player, target
            "#,
        )
        .expect("evaluate both public vehicle macro clauses");
    assert_eq!(
        actual,
        (expected_player.to_owned(), expected_target.to_owned()),
        "player vehicleui and explicit target unithasvehicleui must remain independent"
    );
}

#[test]
fn player_true_target_false_selects_yes_no() {
    let env = seed_vehicle_ui_env(true, false);
    assert_vehicle_ui_options(&env, "yes", "no");
}

#[test]
fn player_false_target_true_selects_no_yes() {
    let env = seed_vehicle_ui_env(false, true);
    assert_vehicle_ui_options(&env, "no", "yes");
}

#[test]
fn player_true_target_true_selects_yes_yes() {
    let env = seed_vehicle_ui_env(true, true);
    assert_vehicle_ui_options(&env, "yes", "yes");
}

#[test]
fn player_false_target_false_selects_no_no() {
    let env = seed_vehicle_ui_env(false, false);
    assert_vehicle_ui_options(&env, "no", "no");
}
