//! AddonList reset-to-default menu behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const DEFAULT_ENABLED_ADDON: &str = "AddonListDefaultEnabledProbe";
const DEFAULT_DISABLED_ADDON: &str = "AddonListDefaultDisabledProbe";

#[test]
fn reset_to_default_uses_default_enabled_state() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_reset_to_default_addons(env);

        let probe: ResetToDefaultProbe = env
            .eval(
                r#"
                local function FindAddonIndex(addonName)
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if C_AddOns.GetAddOnName(index) == addonName then
                            return index
                        end
                    end
                end

                local function CaptureResetCallback(addonIndex, frameName)
                    local capturedReset
                    local createContextMenu = MenuUtil.CreateContextMenu
                    MenuUtil.CreateContextMenu = function(owner, generator)
                        local rootDescription = {
                            SetTag = function() end,
                            CreateTitle = function() end,
                            CreateButton = function(_, text, callback)
                                if text == ADDON_LIST_RESET_TO_DEFAULT then
                                    capturedReset = callback
                                end
                            end,
                        }

                        generator(owner, rootDescription)
                    end

                    local treeNode = {
                        nodes = {},
                        GetData = function()
                            return { addonIndex = addonIndex }
                        end,
                    }

                    local entry = CreateFrame("Button", frameName, UIParent, "AddonListEntryTemplate")
                    AddonList_InitAddon(entry, treeNode)
                    entry:OnClick("RightButton")
                    MenuUtil.CreateContextMenu = createContextMenu
                    return capturedReset
                end

                local defaultEnabledIndex = FindAddonIndex("AddonListDefaultEnabledProbe")
                local defaultDisabledIndex = FindAddonIndex("AddonListDefaultDisabledProbe")
                local resetDefaultEnabled = CaptureResetCallback(defaultEnabledIndex, "AddonListResetDefaultEnabledEntry")
                local resetDefaultDisabled = CaptureResetCallback(defaultDisabledIndex, "AddonListResetDefaultDisabledEntry")

                resetDefaultEnabled()
                resetDefaultDisabled()

                return resetDefaultEnabled ~= nil,
                       resetDefaultDisabled ~= nil,
                       C_AddOns.GetAddOnEnableState(defaultEnabledIndex, nil),
                       C_AddOns.GetAddOnEnableState(defaultDisabledIndex, nil)
                "#,
            )
            .expect("AddonList reset-to-default probe must run cleanly");

        assert_reset_to_default_probe(probe);
    });
}

type ResetToDefaultProbe = (bool, bool, i64, i64);

fn seed_reset_to_default_addons(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.clear();
    state
        .addons
        .push(addon_info(DEFAULT_ENABLED_ADDON, false, true));
    state
        .addons
        .push(addon_info(DEFAULT_DISABLED_ADDON, true, false));
}

fn addon_info(folder_name: &str, enabled: bool, default_enabled: bool) -> AddonInfo {
    AddonInfo {
        folder_name: folder_name.into(),
        title: folder_name.into(),
        enabled,
        loaded: false,
        default_enabled,
        ..Default::default()
    }
}

fn assert_reset_to_default_probe(probe: ResetToDefaultProbe) {
    let (
        captured_default_enabled_reset,
        captured_default_disabled_reset,
        default_enabled_state,
        default_disabled_state,
    ) = probe;

    assert!(
        captured_default_enabled_reset,
        "right-clicking a row must expose the reset-to-default callback"
    );
    assert!(
        captured_default_disabled_reset,
        "right-clicking a second row must expose its reset-to-default callback"
    );
    assert_eq!(
        default_enabled_state, 2,
        "`Reset to Default` must enable addons whose default state is enabled"
    );
    assert_eq!(
        default_disabled_state, 0,
        "`Reset to Default` must disable addons whose default state is disabled"
    );
}
