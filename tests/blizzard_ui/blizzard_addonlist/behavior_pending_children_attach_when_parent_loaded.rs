//! AddonList pending child grouping behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const CHILD_ADDON: &str = "AddonListPendingChildProbe";
const PARENT_ADDON: &str = "AddonListPendingParentProbe";

#[test]
fn addon_list_update_attaches_pending_children_when_parent_loads_later() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_child_before_parent_addons(env);

        let probe: PendingChildProbe = env
            .eval(
                r#"
                local originalGetAddOnMetadata = C_AddOns.GetAddOnMetadata
                C_AddOns.GetAddOnMetadata = function(addonIndex, field)
                    local addonName = C_AddOns.GetAddOnName(addonIndex)
                    if field == "Group" and addonName == "AddonListPendingChildProbe" then
                        return "AddonListPendingParentProbe"
                    end
                    return originalGetAddOnMetadata(addonIndex, field)
                end

                local function IsAddonNode(addonName)
                    return function(node)
                        local data = node:GetData()
                        return data.addonIndex
                           and C_AddOns.GetAddOnName(data.addonIndex) == addonName
                    end
                end

                AddonList:Show()
                AddonList_Update()

                local provider = AddonList.ScrollBox:GetDataProvider()
                local _, parentNode = provider:FindByPredicate(
                    IsAddonNode("AddonListPendingParentProbe"),
                    TreeDataProviderConstants.IncludeCollapsed
                )
                local _, childNode = provider:FindByPredicate(
                    IsAddonNode("AddonListPendingChildProbe"),
                    TreeDataProviderConstants.IncludeCollapsed
                )

                C_AddOns.GetAddOnMetadata = originalGetAddOnMetadata

                return parentNode ~= nil,
                       childNode ~= nil,
                       childNode and childNode:GetParent() == parentNode,
                       parentNode and parentNode:GetSize() or 0,
                       parentNode and parentNode:GetDepth() or -1,
                       childNode and childNode:GetDepth() or -1
                "#,
            )
            .expect("AddonList pending child grouping probe must run cleanly");

        assert_pending_child_probe(probe);
    });
}

type PendingChildProbe = (bool, bool, bool, i64, i64, i64);

fn seed_child_before_parent_addons(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.clear();
    state.addons.push(addon_info(CHILD_ADDON));
    state.addons.push(addon_info(PARENT_ADDON));
}

fn addon_info(folder_name: &str) -> AddonInfo {
    AddonInfo {
        folder_name: folder_name.into(),
        title: folder_name.into(),
        enabled: true,
        loaded: false,
        ..Default::default()
    }
}

fn assert_pending_child_probe(probe: PendingChildProbe) {
    let (
        has_parent_node,
        has_child_node,
        child_attached_to_parent,
        parent_child_count,
        parent_depth,
        child_depth,
    ) = probe;

    assert!(
        has_parent_node,
        "`AddonList_Update` must create the parent addon row"
    );
    assert!(
        has_child_node,
        "`AddonList_Update` must preserve the child addon row when it appears before its group"
    );
    assert!(
        child_attached_to_parent,
        "pending children must be attached to the parent group node when the parent is reached"
    );
    assert_eq!(
        parent_child_count, 1,
        "the parent addon node must contain the buffered child node"
    );
    assert_eq!(
        child_depth,
        parent_depth + 1,
        "the child node must be one tree level below its parent"
    );
}
