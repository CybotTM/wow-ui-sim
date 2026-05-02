//! Public mixin methods for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";
const ADDON_LIST_MIXIN_METHODS: &[&str] = &[
    "OnLoad",
    "OnShow",
    "OnHide",
    "OnUpdate",
    "GetAddonMetricPercent",
    "GetOverallMetric",
    "UpdateOverallMetric",
    "UpdatePerformance",
    "UpdateAddOnMemoryUsage",
];

#[test]
fn addon_list_mixin_exposes_plan_methods() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for method_name in ADDON_LIST_MIXIN_METHODS {
            let actual_type: String = env
                .eval(&format!("return type(AddonListMixin[{method_name:?}])"))
                .unwrap_or_else(|err| {
                    panic!("failed to probe `AddonListMixin.{method_name}`: {err}")
                });

            assert_eq!(
                actual_type, "function",
                "`AddonListMixin.{method_name}` must be exposed as a function"
            );
        }
    });
}
