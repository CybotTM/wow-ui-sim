use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AuthChallengeUI";

const GLOBALS: &[&str] = &[
    "AuthChallengeUI_OnLoad",
    "AuthChallengeUI_Submit",
    "AuthChallengeUI_Cancel",
    "AuthChallengeUI_OnTabPressed",
    "AuthChallengeUI_OnKeyDown",
];

fn load_auth_challenge_ui(env: &WowLuaEnv) {
    clear_recorded_lua_errors(env);

    let (loaded, reason): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
        .expect("C_AddOns.LoadAddOn should return");
    assert!(loaded, "`{ROOT}` should load: {reason:?}");
}

fn assert_no_lua_errors(env: &WowLuaEnv, test_name: &str) {
    let errors = recorded_lua_errors(env);
    assert!(
        errors.is_empty(),
        "`{ROOT}` {test_name} emitted Lua errors:\n{}",
        errors.join("\n")
    );
}

#[test]
fn blizzard_auth_challenge_ui_publishes_expected_globals() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_auth_challenge_ui(env);

        for name in GLOBALS {
            let is_function: bool = env
                .eval(&format!("return type(_G['{name}']) == 'function'"))
                .expect("global type probe should succeed");
            assert!(
                is_function,
                "After loading `{ROOT}`, `{name}` must publish as a function in `_G`"
            );
        }

        assert_no_lua_errors(env, "global-surface test");
    });
}

#[test]
fn blizzard_auth_challenge_ui_publishes_expected_frame_tree() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_auth_challenge_ui(env);

        let frame_exists: bool = env
            .eval(
                "local f = _G['AuthChallengeFrame']; \
                 return type(f) == 'table' and type(f.GetName) == 'function' and f:GetName() == 'AuthChallengeFrame'",
            )
            .expect("AuthChallengeFrame probe should succeed");
        assert!(
            frame_exists,
            "AuthChallengeFrame must exist as a named frame"
        );

        let parent_and_flags: (bool, bool, bool) = env
            .eval(
                "local f = _G['AuthChallengeFrame']; \
                 return f:GetParent() == UIParent, \
                        f:GetFrameStrata() == 'BLIZZARD', \
                        not f:IsShown()",
            )
            .expect("AuthChallengeFrame parent/flag probe should succeed");
        assert!(
            parent_and_flags.0,
            "AuthChallengeFrame must be parented to UIParent"
        );
        assert!(
            parent_and_flags.1,
            "AuthChallengeFrame must use BLIZZARD frame strata"
        );
        assert!(parent_and_flags.2, "AuthChallengeFrame must start hidden");

        let keyboard_and_mouse: (bool, bool) = env
            .eval(
                "local f = _G['AuthChallengeFrame']; \
                 return f:IsKeyboardEnabled(), f:IsMouseEnabled()",
            )
            .expect("AuthChallengeFrame input probe should succeed");
        assert!(
            keyboard_and_mouse.0,
            "AuthChallengeFrame must enable keyboard input"
        );
        assert!(
            keyboard_and_mouse.1,
            "AuthChallengeFrame must enable mouse input"
        );

        assert_no_lua_errors(env, "frame-tree test");
    });
}

#[test]
fn blizzard_auth_challenge_ui_publishes_expected_child_frames_and_templates() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_auth_challenge_ui(env);

        for child in ["WaitFrame", "InputFrame", "DeniedFrame", "ErrorFrame"] {
            let exists: bool = env
                .eval(&format!(
                    "local f = AuthChallengeFrame.{child}; return type(f) == 'table' and type(f.GetName) == 'function'"
                ))
                .expect("child frame probe should succeed");
            assert!(exists, "AuthChallengeFrame.{child} must publish as a frame");

            let is_hidden: bool = env
                .eval(&format!("return not AuthChallengeFrame.{child}:IsShown()"))
                .expect("child hidden probe should succeed");
            assert!(is_hidden, "AuthChallengeFrame.{child} must start hidden");
        }

        let input_frame_children: (bool, bool, bool, bool, bool, bool, bool) = env
            .eval(
                "local f = _G['AuthChallengeFrame'].InputFrame; \
                 return type(f.Input1) == 'table', \
                        type(f.Input2) == 'table', \
                        type(f.Input3) == 'table', \
                        type(f.Input4) == 'table', \
                        type(f.Prompt) == 'table', \
                        type(f.Info) == 'table', \
                        type(f.Error) == 'table'",
            )
            .expect("InputFrame child probe should succeed");
        assert!(
            input_frame_children.0
                && input_frame_children.1
                && input_frame_children.2
                && input_frame_children.3
                && input_frame_children.4
                && input_frame_children.5
                && input_frame_children.6,
            "InputFrame must publish the expected edit boxes and prompt/info/error text"
        );

        assert_no_lua_errors(env, "child-frame/template test");
    });
}
