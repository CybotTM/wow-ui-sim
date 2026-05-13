//! Behavior pin: `ACTIONBAR_PAGE_CHANGED` runs the controller refresh path.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBarController";
const SEEDED_ACTION_BAR_PAGE: i32 = 3;

#[test]
fn actionbar_page_changed_refreshes_main_bar_action_page() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_stale_main_bar_page(env);

        env.exec(
            r#"
            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "ACTIONBAR_PAGE_CHANGED"
            )
            "#,
        )
        .expect("ActionBarController ACTIONBAR_PAGE_CHANGED dispatch must run cleanly");

        let action_page: i32 = env
            .eval(r#"return MainActionBar:GetAttribute("actionpage")"#)
            .expect("post ACTIONBAR_PAGE_CHANGED actionpage probe must run cleanly");

        assert_eq!(
            action_page, SEEDED_ACTION_BAR_PAGE,
            "ACTIONBAR_PAGE_CHANGED must route through ActionBarController_UpdateAll, \
             which refreshes MainActionBar actionpage from C_ActionBar.GetActionBarPage()"
        );
    });
    }
}

fn seed_stale_main_bar_page(env: &WowLuaEnv) {
    env.state().borrow_mut().action_bar_page = SEEDED_ACTION_BAR_PAGE as u32;
    env.exec(
        r#"
        MainActionBar:SetAttribute("actionpage", 99)
        "#,
    )
    .expect("stale actionpage fixture must run cleanly");
}
