//! Public globals for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AddOnList";
const ADDON_BUTTON_HEIGHT: i32 = 16;
const MAX_ADDONS_DISPLAYED: i32 = 19;
const GLOBAL_FUNCTIONS: &[&str] = &[
    "AddonList_HasAnyChanged",
    "AddonList_HasOutOfDate",
    "AddonList_IsAddOnLoadOnDemand",
    "AddonList_Update",
    "AddonList_OnKeyDown",
    "AddonList_Enable",
    "AddonList_EnableAll",
    "AddonList_DisableAll",
    "AddonList_LoadAddOn",
    "AddonList_OnOkay",
    "AddonList_OnCancel",
    "AddonList_DisableOutOfDate",
    "AddonList_SetSecurityIcon",
    "AddonList_SetStatus",
    "AddonList_InitAddon",
    "AddonList_ClearCharacterDropdown",
    "AddonTooltip_BuildDeps",
    "AddonTooltip_Update",
    "AddonTooltip_ActionBlocked",
];

#[test]
fn addon_list_publishes_module_constants() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let (button_height, max_displayed, collapsed_type): (i32, i32, String) = env
            .eval(
                r#"
                return ADDON_BUTTON_HEIGHT,
                       MAX_ADDONS_DISPLAYED,
                       type(g_addonCategoriesCollapsed)
                "#,
            )
            .expect("Blizzard_AddOnList module globals must be probeable after load");

        assert_eq!(
            button_height, ADDON_BUTTON_HEIGHT,
            "`ADDON_BUTTON_HEIGHT` must match Blizzard's published constant"
        );
        assert_eq!(
            max_displayed, MAX_ADDONS_DISPLAYED,
            "`MAX_ADDONS_DISPLAYED` must match Blizzard's published constant"
        );
        assert_eq!(
            collapsed_type, "table",
            "`g_addonCategoriesCollapsed` must be available as a table"
        );
    });
}

#[test]
fn addon_list_publishes_global_functions() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for global_name in GLOBAL_FUNCTIONS {
            let actual_type = global_type(env, global_name);

            assert_eq!(
                actual_type, "function",
                "`{global_name}` must be published as a global function"
            );
        }
    });
}

fn global_type(env: &WowLuaEnv, global_name: &str) -> String {
    env.eval(&format!("return type(_G[{global_name:?}])"))
        .unwrap_or_else(|err| panic!("failed to probe global type for `{global_name}`: {err}"))
}
