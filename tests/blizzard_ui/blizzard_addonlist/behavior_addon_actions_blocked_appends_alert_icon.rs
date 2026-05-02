//! AddonList blocked-action alert icon behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const BLOCKED_ADDON: &str = "AddonListBlockedActionProbe";

#[test]
fn addon_actions_blocked_appends_alert_icon_to_initialized_row_title() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_blocked_action_probe_addon(env);

        let title_text: String = env
            .eval(
                r#"
                local function FindAddonIndex(addonName)
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if C_AddOns.GetAddOnName(index) == addonName then
                            return index
                        end
                    end
                end

                AddonTooltip_ActionBlocked("AddonListBlockedActionProbe")

                local addonIndex = FindAddonIndex("AddonListBlockedActionProbe")
                local entry = CreateFrame("Button", "AddonListBlockedActionEntry", UIParent, "AddonListEntryTemplate")
                local treeNode = {
                    GetData = function()
                        return { addonIndex = addonIndex }
                    end,
                }

                AddonList_InitAddon(entry, treeNode)
                return entry.Title:GetText()
                "#,
            )
            .expect("AddonList_InitAddon blocked-action probe must run cleanly");

        assert!(
            title_text.contains("DialogIcon-AlertNew-16"),
            "`AddonList_InitAddon` must append blocked-action alert markup; got {title_text:?}"
        );
        assert!(
            title_text.contains("Addon List Blocked Action Probe"),
            "`AddonList_InitAddon` must preserve the addon title before the alert icon; got {title_text:?}"
        );
    });
}

fn seed_blocked_action_probe_addon(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.state().borrow_mut().addons.push(AddonInfo {
        folder_name: BLOCKED_ADDON.into(),
        title: "Addon List Blocked Action Probe".into(),
        enabled: true,
        loaded: false,
        ..Default::default()
    });
}
