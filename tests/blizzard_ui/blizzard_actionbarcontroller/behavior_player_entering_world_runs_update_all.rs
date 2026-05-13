//! Behavior pin: `PLAYER_ENTERING_WORLD` runs the controller refresh path.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn player_entering_world_refreshes_main_bar_page_and_visibility() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_stale_main_bar_state(env);

        env.exec(
            r#"
            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "PLAYER_ENTERING_WORLD"
            )
            "#,
        )
        .expect("ActionBarController PLAYER_ENTERING_WORLD dispatch must run cleanly");

        let (action_page, expected_page, main_bar_shown): (i32, i32, bool) = env
            .eval(
                r#"
                return MainActionBar:GetAttribute("actionpage"),
                    C_ActionBar.GetActionBarPage(),
                    MainActionBar:IsShown()
                "#,
            )
            .expect("post PLAYER_ENTERING_WORLD main-bar probe must run cleanly");

        assert_eq!(
            action_page, expected_page,
            "PLAYER_ENTERING_WORLD must route through ActionBarController_UpdateAll, \
             which resets MainActionBar actionpage from C_ActionBar.GetActionBarPage()"
        );
        assert!(
            main_bar_shown,
            "PLAYER_ENTERING_WORLD must leave the main action bar shown when no \
             override, vehicle, or possess state is seeded"
        );
    });
    }
}

fn seed_stale_main_bar_state(env: &WowLuaEnv) {
    env.state().borrow_mut().action_bar_page = 3;
    env.exec(
        r#"
        MainActionBar:SetAttribute("actionpage", 99)
        "#,
    )
    .expect("stale main-bar fixture must run cleanly");
}
