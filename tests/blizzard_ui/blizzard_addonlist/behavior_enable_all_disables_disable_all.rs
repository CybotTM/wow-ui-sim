//! Enable-all and disable-all button behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";

#[test]
fn enable_all_and_disable_all_buttons_flip_addon_enable_states() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        register_probe_addons(env);

        let (disabled_before_enable, disabled_after_enable, enabled_after_disable): (
            i64,
            i64,
            i64,
        ) = env
            .eval(
                r#"
                local function IsUserAddon(index)
                    return C_AddOns.GetAddOnName(index) ~= "__BuiltIn"
                end

                local function CountUserAddonsWhere(predicate)
                    local count = 0
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if IsUserAddon(index) and predicate(index) then
                            count = count + 1
                        end
                    end
                    return count
                end

                local function IsEnabled(index)
                    return C_AddOns.GetAddOnEnableState(index, nil) > Enum.AddOnEnableState.None
                end

                C_AddOns.DisableAllAddOns(nil)
                local disabledBeforeEnable = CountUserAddonsWhere(function(index)
                    return not IsEnabled(index)
                end)

                AddonList.EnableAllButton:Click()
                local disabledAfterEnable = CountUserAddonsWhere(function(index)
                    return not IsEnabled(index)
                end)

                AddonList.DisableAllButton:Click()
                local enabledAfterDisable = CountUserAddonsWhere(IsEnabled)

                return disabledBeforeEnable, disabledAfterEnable, enabledAfterDisable
                "#,
            )
            .expect("AddonList enable/disable-all probe must run cleanly");

        assert!(
            disabled_before_enable > 0,
            "test setup must start with at least one disabled AddOnList entry"
        );
        assert_eq!(
            disabled_after_enable, 0,
            "`EnableAllButton:Click()` must enable every AddOnList entry"
        );
        assert_eq!(
            enabled_after_disable, 0,
            "`DisableAllButton:Click()` must disable every AddOnList entry"
        );
    });
}

fn register_probe_addons(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        A_Admin.RegisterTestAddon("EnableAllProbeOne")
        A_Admin.RegisterTestAddon("EnableAllProbeTwo")
        "#,
    )
    .expect("enable/disable-all probe addon registration must run cleanly");
}
