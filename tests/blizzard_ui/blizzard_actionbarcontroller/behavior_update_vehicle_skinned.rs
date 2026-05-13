//! Behavior pin: skinned vehicle bars mount the override action bar.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";
const SEEDED_VEHICLE_SKIN: &str = "MechagonShredder";

#[test]
fn update_vehicle_actionbar_mounts_skinned_override_bar() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.has_vehicle_action_bar = true;
            state.player.vehicle_skin = Some(SEEDED_VEHICLE_SKIN.to_string());
        }
        assert_vehicle_preconditions(env);

        env.exec(
            r#"
            OverrideActionBar.updateSkinCallCount = 0
            function OverrideActionBar:UpdateSkin(...)
                self.updateSkinCallCount = self.updateSkinCallCount + 1
                self.lastVehicleSkin = UnitVehicleSkin("player")
            end

            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "UPDATE_VEHICLE_ACTIONBAR"
            )
            "#,
        )
        .expect("ActionBarController UPDATE_VEHICLE_ACTIONBAR dispatch must run cleanly");

        let (current_state, override_state, has_vehicle, update_skin_calls, last_vehicle_skin): (
            i32,
            i32,
            bool,
            i32,
            String,
        ) = env
            .eval(
                r#"
                return ActionBarController_GetCurrentActionBarState(),
                    LE_ACTIONBAR_STATE_OVERRIDE,
                    C_ActionBar.HasVehicleActionBar(),
                    OverrideActionBar.updateSkinCallCount,
                    OverrideActionBar.lastVehicleSkin
                "#,
            )
            .expect("post UPDATE_VEHICLE_ACTIONBAR transition probe must run cleanly");

        assert_eq!(
            current_state, override_state,
            "skinned vehicle state must flip CURRENT_ACTION_BAR_STATE to \
             LE_ACTIONBAR_STATE_OVERRIDE; has_vehicle={has_vehicle}, \
             update_skin_calls={update_skin_calls}, last_vehicle_skin={last_vehicle_skin:?}"
        );
        assert_eq!(
            update_skin_calls, 1,
            "skinned vehicle state must invoke OverrideActionBar:UpdateSkin exactly once"
        );
        assert_eq!(
            last_vehicle_skin, SEEDED_VEHICLE_SKIN,
            "OverrideActionBar:UpdateSkin must see the seeded UnitVehicleSkin"
        );
    });
    }
}

fn assert_vehicle_preconditions(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (has_vehicle, skin): (bool, String) = env
        .eval(
            r#"
            return C_ActionBar.HasVehicleActionBar(),
                UnitVehicleSkin("player")
            "#,
        )
        .expect("vehicle C_ActionBar precondition probe must run cleanly");

    assert!(has_vehicle, "seeded state must expose a vehicle action bar");
    assert_eq!(
        skin, SEEDED_VEHICLE_SKIN,
        "seeded state must expose the vehicle skin"
    );
}
