//! AddonList tri-state checkbox visuals for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const DISABLED_ADDON: &str = "AddonListTriStateDisabledProbe";
const ENABLED_ADDON: &str = "AddonListTriStateEnabledProbe";
const PARTIAL_ADDON: &str = "AddonListTriStatePartialProbe";

#[test]
fn init_addon_applies_tristate_checkbox_visuals() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_tristate_probe_addons(env);

        let (
            disabled_checked,
            disabled_desaturated,
            disabled_state,
            enabled_checked,
            enabled_desaturated,
            enabled_state,
            partial_checked,
            partial_desaturated,
            partial_state,
        ): TriStateVisuals = env
            .eval(
                r#"
                local addonStates = {
                    AddonListTriStateDisabledProbe = Enum.AddOnEnableState.None,
                    AddonListTriStateEnabledProbe = Enum.AddOnEnableState.All,
                    AddonListTriStatePartialProbe = Enum.AddOnEnableState.Some,
                }
                local getAddOnEnableState = C_AddOns.GetAddOnEnableState
                C_AddOns.GetAddOnEnableState = function(addonIndex, character)
                    local addonName = C_AddOns.GetAddOnName(addonIndex)
                    return addonStates[addonName] or getAddOnEnableState(addonIndex, character)
                end

                local function FindAddonIndex(addonName)
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if C_AddOns.GetAddOnName(index) == addonName then
                            return index
                        end
                    end
                end

                local function InitProbeEntry(addonName, frameName)
                    local addonIndex = FindAddonIndex(addonName)
                    local entry = CreateFrame("Button", frameName, UIParent, "AddonListEntryTemplate")
                    local treeNode = {
                        GetData = function()
                            return { addonIndex = addonIndex }
                        end,
                    }

                    AddonList_InitAddon(entry, treeNode)
                    return entry.Enabled:GetChecked(),
                           entry.Enabled.CheckedTexture:IsDesaturated(),
                           entry.Enabled.state
                end

                local disabledChecked, disabledDesaturated, disabledState = InitProbeEntry(
                    "AddonListTriStateDisabledProbe",
                    "AddonListTriStateDisabledEntry"
                )
                local enabledChecked, enabledDesaturated, enabledState = InitProbeEntry(
                    "AddonListTriStateEnabledProbe",
                    "AddonListTriStateEnabledEntry"
                )
                local partialChecked, partialDesaturated, partialState = InitProbeEntry(
                    "AddonListTriStatePartialProbe",
                    "AddonListTriStatePartialEntry"
                )

                return disabledChecked,
                       disabledDesaturated,
                       disabledState,
                       enabledChecked,
                       enabledDesaturated,
                       enabledState,
                       partialChecked,
                       partialDesaturated,
                       partialState
                "#,
            )
            .expect("AddonList_InitAddon tri-state checkbox probe must run cleanly");

        assert_eq!(
            disabled_checked,
            Some(false),
            "None must clear the checkbox"
        );
        assert!(
            !disabled_desaturated,
            "None should not leave a desaturated checked texture visible"
        );
        assert_eq!(disabled_state, 0);

        assert_eq!(
            enabled_checked,
            Some(true),
            "All must show a normal checked state"
        );
        assert!(
            !enabled_desaturated,
            "All must leave the checked texture saturated"
        );
        assert_eq!(enabled_state, 2);

        assert_eq!(
            partial_checked,
            Some(true),
            "Some must still show a checked state"
        );
        assert!(
            partial_desaturated,
            "Some must desaturate the checked texture"
        );
        assert_eq!(partial_state, 1);
    });
}

type TriStateVisuals = (
    Option<bool>,
    bool,
    i64,
    Option<bool>,
    bool,
    i64,
    Option<bool>,
    bool,
    i64,
);

fn seed_tristate_probe_addons(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.push(probe_addon(DISABLED_ADDON, false));
    state.addons.push(probe_addon(ENABLED_ADDON, true));
    state.addons.push(probe_addon(PARTIAL_ADDON, true));
}

fn probe_addon(folder_name: &str, enabled: bool) -> AddonInfo {
    AddonInfo {
        folder_name: folder_name.into(),
        title: folder_name.into(),
        enabled,
        loaded: false,
        ..Default::default()
    }
}
