//! Public globals for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ArchaeologyUI";
const GLOBAL_FUNCTIONS: &[&str] = &[
    "ArchaeologyFrame_Show",
    "ArchaeologyFrame_Hide",
    "ArchaeologyFrame_OnLoad",
    "ArchaeologyFrame_OnEvent",
    "ArchaeologyFrame_ShowFailed",
    "ArchaeologyFrame_ShowArtifact",
    "ArchaeologyFrame_UpdateSummary",
    "ArchaeologyFrame_UpdateComplete",
    "ArchaeologyFrame_CurrentArtifactUpdate",
];
const MODULE_CONSTANTS: &[(&str, i32)] = &[
    ("ARCHAEOLOGY_BUTTON_HEIGHT", 59),
    ("ARCHAEOLOGY_MAX_RACES", 12),
    ("ARCHAEOLOGY_MAX_STONES", 4),
    ("ARCHAEOLOGY_MAX_COMPLETED_SHOWN", 12),
    ("ARCHAEOLOGY_HELP_TAB", 0),
    ("ARCHAEOLOGY_SUMMARY_TAB", 1),
    ("ARCHAEOLOGY_COMPLETED_TAB", 2),
    ("ARCHAEOLOGY_SUMMARY_PAGE", 1),
    ("ARCHAEOLOGY_COMPLETED_PAGE", 2),
    ("ARCHAEOLOGY_CURRENT_PAGE", 3),
];

#[test]
fn archaeology_ui_publishes_expected_global_functions() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for global_name in GLOBAL_FUNCTIONS {
            let actual_type = global_type(env, global_name);

            assert_eq!(
                actual_type, "function",
                "`{global_name}` must be published as a global function after `{ROOT}` loads"
            );
        }
    });
}

#[test]
fn archaeology_ui_publishes_module_constants() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for (global_name, expected_value) in MODULE_CONSTANTS {
            let actual_value = global_i32(env, global_name);

            assert_eq!(
                actual_value, *expected_value,
                "`{global_name}` must match Blizzard_ArchaeologyUI.lua's module constant"
            );
        }
    });
}

fn global_type(env: &WowLuaEnv, global_name: &str) -> String {
    env.eval(&format!("return type(_G[{global_name:?}])"))
        .unwrap_or_else(|err| panic!("failed to probe global type for `{global_name}`: {err}"))
}

fn global_i32(env: &WowLuaEnv, global_name: &str) -> i32 {
    env.eval(&format!("return _G[{global_name:?}]"))
        .unwrap_or_else(|err| panic!("failed to probe integer global `{global_name}`: {err}"))
}
