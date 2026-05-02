//! Glue-screen event registration behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";
const FRAME_NAME: &str = "ActionStatus";
const GLUE_SCREEN_EVENTS: &[&str] = &[
    "GLUE_SCREENSHOT_STARTED",
    "GLUE_SCREENSHOT_SUCCEEDED",
    "GLUE_SCREENSHOT_FAILED",
];
const GAME_SCREEN_EVENTS: &[&str] = &[
    "SCREENSHOT_STARTED",
    "SCREENSHOT_SUCCEEDED",
    "SCREENSHOT_FAILED",
];

#[test]
fn glue_branch_registers_glue_screenshot_events_only() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        assert_in_glue(env);
        assert_events_registered(env, GLUE_SCREEN_EVENTS);
        assert_events_not_registered(env, GAME_SCREEN_EVENTS);
    });
}

fn assert_in_glue(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let in_glue: bool = env
        .eval("return InGlue()")
        .expect("InGlue probe must run cleanly");

    assert!(
        in_glue,
        "`{ROOT}` glue-branch behavior probe must run with InGlue() == true"
    );
}

fn assert_events_registered(env: &wow_ui_sim::lua_api::WowLuaEnv, events: &[&str]) {
    for event_name in events {
        let registered = action_status_is_event_registered(env, event_name);

        assert!(
            registered,
            "Expected `{FRAME_NAME}:IsEventRegistered({event_name:?})` to be true in glue"
        );
    }
}

fn assert_events_not_registered(env: &wow_ui_sim::lua_api::WowLuaEnv, events: &[&str]) {
    for event_name in events {
        let registered = action_status_is_event_registered(env, event_name);

        assert!(
            !registered,
            "Expected `{FRAME_NAME}:IsEventRegistered({event_name:?})` to be false in glue"
        );
    }
}

fn action_status_is_event_registered(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    event_name: &str,
) -> bool {
    env.eval(&format!(
        "return _G[{FRAME_NAME:?}]:IsEventRegistered({event_name:?})"
    ))
    .unwrap_or_else(|err| {
        panic!("failed to probe `{FRAME_NAME}:IsEventRegistered({event_name:?})`: {err}")
    })
}
