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

fn global_type(env: &WowLuaEnv, global_name: &str) -> String {
    env.eval(&format!("return type(_G[{global_name:?}])"))
        .unwrap_or_else(|err| panic!("failed to probe global type for `{global_name}`: {err}"))
}
