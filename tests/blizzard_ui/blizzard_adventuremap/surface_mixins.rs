//! Public mixin methods for `Blizzard_AdventureMap`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const ADVENTURE_MAP_MIXIN_METHODS: &[&str] = &[
    "OnLoad",
    "OnShow",
    "OnHide",
    "OnEvent",
    "RefreshInsets",
    "IsMapInsetExpanded",
    "SetupTitle",
    "AddStandardDataProviders",
    "ClearAreaTableIDAvailableForInsets",
    "SetAreaTableIDAvailableForInsets",
];

#[test]
fn adventure_map_mixin_exposes_plan_methods() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        assert_mixin_methods(env, "AdventureMapMixin", ADVENTURE_MAP_MIXIN_METHODS);
    });
}

fn assert_mixin_methods(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    mixin_name: &str,
    method_names: &[&str],
) {
    for method_name in method_names {
        let actual_type = probe_mixin_method_type(env, mixin_name, method_name);

        assert_eq!(
            actual_type, "function",
            "`{mixin_name}.{method_name}` must be exposed as a function"
        );
    }
}

fn probe_mixin_method_type(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    mixin_name: &str,
    method_name: &str,
) -> String {
    env.eval(&format!("return type({mixin_name}[{method_name:?}])"))
        .unwrap_or_else(|err| panic!("failed to probe `{mixin_name}.{method_name}`: {err}"))
}
