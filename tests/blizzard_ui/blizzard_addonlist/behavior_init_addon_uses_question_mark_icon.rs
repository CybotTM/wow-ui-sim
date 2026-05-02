//! AddonList row icon fallback behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const ICON_FALLBACK_ADDON: &str = "AddonListIconFallbackProbe";

#[test]
fn init_addon_uses_question_mark_icon_when_icon_metadata_is_absent() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_icon_fallback_probe_addon(env);

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

                local addonIndex = FindAddonIndex("AddonListIconFallbackProbe")
                local entry = CreateFrame("Button", "AddonListIconFallbackEntry", UIParent, "AddonListEntryTemplate")
                local treeNode = {
                    GetData = function()
                        return { addonIndex = addonIndex }
                    end,
                }

                AddonList_InitAddon(entry, treeNode)
                return entry.Title:GetText()
                "#,
            )
            .expect("AddonList_InitAddon icon fallback probe must run cleanly");

        assert!(
            title_text.contains("INV_Misc_QuestionMark"),
            "`AddonList_InitAddon` must prefix rows without icon metadata with question-mark markup; got {title_text:?}"
        );
        assert!(
            title_text.contains("Addon List Icon Fallback Probe"),
            "`AddonList_InitAddon` must preserve the addon title after the fallback icon; got {title_text:?}"
        );
    });
}

fn seed_icon_fallback_probe_addon(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.state().borrow_mut().addons.push(AddonInfo {
        folder_name: ICON_FALLBACK_ADDON.into(),
        title: "Addon List Icon Fallback Probe".into(),
        enabled: true,
        loaded: false,
        ..Default::default()
    });
}
