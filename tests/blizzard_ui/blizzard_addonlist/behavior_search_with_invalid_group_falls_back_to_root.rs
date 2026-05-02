//! AddonList invalid-group fallback behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const INVALID_GROUP_ADDON: &str = "AddonListInvalidGroupProbe";

#[test]
fn addon_list_update_keeps_search_match_with_invalid_group_at_root() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_invalid_group_addon(env);

        let probe: InvalidGroupProbe = env
            .eval(
                r#"
                local originalGetAddOnMetadata = C_AddOns.GetAddOnMetadata
                C_AddOns.GetAddOnMetadata = function(addonIndex, field)
                    local addonName = C_AddOns.GetAddOnName(addonIndex)
                    if field == "Group" and addonName == "AddonListInvalidGroupProbe" then
                        return "AddonListMissingParentGroup"
                    end
                    return originalGetAddOnMetadata(addonIndex, field)
                end

                local function IsInvalidGroupAddonNode(node)
                    local data = node:GetData()
                    return data.addonIndex
                       and C_AddOns.GetAddOnName(data.addonIndex) == "AddonListInvalidGroupProbe"
                end

                AddonList:Show()
                AddonList.SearchBox:SetText("AddonListMissingParentGroup")
                AddonList_Update()

                local provider = AddonList.ScrollBox:GetDataProvider()
                local _, node = provider:FindByPredicate(
                    IsInvalidGroupAddonNode,
                    TreeDataProviderConstants.ExcludeCollapsed
                )

                C_AddOns.GetAddOnMetadata = originalGetAddOnMetadata

                return node ~= nil,
                       provider:GetSize(TreeDataProviderConstants.ExcludeCollapsed),
                       node and node:GetDepth() or -1,
                       node and node:GetParent():GetData() == nil
                "#,
            )
            .expect("AddonList invalid-group fallback probe must run cleanly");

        assert_invalid_group_probe(probe);
    });
}

type InvalidGroupProbe = (bool, i64, i64, bool);

fn seed_invalid_group_addon(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.clear();
    state.addons.push(AddonInfo {
        folder_name: INVALID_GROUP_ADDON.into(),
        title: INVALID_GROUP_ADDON.into(),
        enabled: true,
        loaded: false,
        ..Default::default()
    });
}

fn assert_invalid_group_probe(probe: InvalidGroupProbe) {
    let (has_addon_node, visible_row_count, node_depth, parent_is_tree_root) = probe;

    assert!(
        has_addon_node,
        "`AddonList_Update` must not drop search matches whose group parent is missing"
    );
    assert_eq!(
        visible_row_count, 1,
        "the invalid-group addon must remain visible as the only search result"
    );
    assert_eq!(
        node_depth, 1,
        "the invalid-group addon must be inserted directly under the tree root"
    );
    assert!(
        parent_is_tree_root,
        "the invalid-group addon must not be attached below a synthetic parent"
    );
}
