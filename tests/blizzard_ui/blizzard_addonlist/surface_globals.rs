//! Public globals for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";
const ADDON_BUTTON_HEIGHT: i32 = 16;
const MAX_ADDONS_DISPLAYED: i32 = 19;

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
