//! AddonList row right-click menu behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const PARENT_ADDON: &str = "AddonListRightClickParentProbe";
const CHILD_ADDON: &str = "AddonListRightClickChildProbe";

#[test]
fn right_clicking_addon_row_builds_context_menu() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_right_click_probe_addons(env);

        let (menu_tag, has_deps, has_enable_group, has_disable_group, has_reset): MenuProbe =
            env.eval(
                r#"
                local capturedTag
                local capturedButtons = {}
                local createContextMenu = MenuUtil.CreateContextMenu
                MenuUtil.CreateContextMenu = function(owner, generator)
                    local rootDescription = {
                        SetTag = function(_, tag)
                            capturedTag = tag
                        end,
                        CreateTitle = function() end,
                        CreateButton = function(_, text)
                            capturedButtons[text] = true
                        end,
                    }

                    generator(owner, rootDescription)
                end

                local function FindAddonIndex(addonName)
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if C_AddOns.GetAddOnName(index) == addonName then
                            return index
                        end
                    end
                end

                local parentIndex = FindAddonIndex("AddonListRightClickParentProbe")
                local childIndex = FindAddonIndex("AddonListRightClickChildProbe")
                local treeNode = {
                    nodes = {
                        {
                            GetData = function()
                                return { addonIndex = childIndex }
                            end,
                        },
                    },
                    GetData = function()
                        return { addonIndex = parentIndex }
                    end,
                }

                local entry = CreateFrame("Button", "AddonListRightClickEntry", UIParent, "AddonListEntryTemplate")
                AddonList_InitAddon(entry, treeNode)
                entry:OnClick("RightButton")

                MenuUtil.CreateContextMenu = createContextMenu

                return capturedTag,
                       capturedButtons[ADDON_LIST_ENABLE_DEPENDENCIES] == true,
                       capturedButtons[ADDON_LIST_ENABLE_GROUP] == true,
                       capturedButtons[ADDON_LIST_DISABLE_GROUP] == true,
                       capturedButtons[ADDON_LIST_RESET_ALL_TO_DEFAULT] == true
                "#,
            )
            .expect("AddonList right-click context menu probe must run cleanly");

        assert_eq!(menu_tag, "MENU_ADDON_LIST_ENTRY");
        assert!(
            has_deps,
            "right-click menu must include the enable-dependencies action"
        );
        assert!(
            has_enable_group,
            "right-click menu must include the enable-group action for rows with children"
        );
        assert!(
            has_disable_group,
            "right-click menu must include the disable-group action for rows with children"
        );
        assert!(
            has_reset,
            "right-click menu must include the reset-to-default action"
        );
    });
}

type MenuProbe = (String, bool, bool, bool, bool);

fn seed_right_click_probe_addons(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.push(probe_addon(PARENT_ADDON));
    state.addons.push(probe_addon(CHILD_ADDON));
}

fn probe_addon(folder_name: &str) -> AddonInfo {
    AddonInfo {
        folder_name: folder_name.into(),
        title: folder_name.into(),
        enabled: true,
        loaded: false,
        ..Default::default()
    }
}
