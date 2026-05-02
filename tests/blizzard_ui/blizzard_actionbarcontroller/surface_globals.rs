//! Surface-global probes for `Blizzard_ActionBarController`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionBarController";
const ACTION_BAR_CONTROLLER_FUNCTIONS: &[&str] = &[
    "ActionBarController_GetCurrentActionBarState",
    "ActionBarController_OnLoad",
    "ActionBarController_OnEvent",
    "ActionBarController_UpdateAll",
    "ActionBarController_UpdateAllSpellHighlights",
    "ActionBarController_ResetToDefault",
];

#[test]
fn action_bar_controller_surface_globals_are_functions() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for function_name in ACTION_BAR_CONTROLLER_FUNCTIONS {
            let is_function: bool = env
                .eval(&format!(r#"return type({function_name}) == "function""#))
                .unwrap_or_else(|_| panic!("{function_name} type probe must run cleanly"));

            assert!(
                is_function,
                "`{function_name}` must be a global function after `{ROOT}` loads"
            );
        }
    });
}
