//! AddonList category collapse persistence for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const PROBE_ADDON: &str = "AddonListCategoryCollapseProbe";

#[test]
fn category_collapse_persists_to_saved_variable_and_rebuilt_tree() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_category_probe_addon(env);

        let (
            initially_visible,
            saved_after_collapse,
            collapsed_visible,
            rebuilt_collapsed,
            rebuilt_child_hidden,
            saved_after_expand,
            rebuilt_expanded,
            rebuilt_child_visible,
        ): CategoryCollapseProbe = env
            .eval(
                r#"
                local getAddOnMetadata = C_AddOns.GetAddOnMetadata
                C_AddOns.GetAddOnMetadata = function(addonIndex, field)
                    local addonName = C_AddOns.GetAddOnName(addonIndex)
                    if addonName == "AddonListCategoryCollapseProbe" and field == "Category" then
                        return "Codex Collapse Category"
                    end
                    return getAddOnMetadata(addonIndex, field)
                end

                local function IsProbeAddonNode(node)
                    local data = node:GetData()
                    return data.addonIndex
                       and C_AddOns.GetAddOnName(data.addonIndex) == "AddonListCategoryCollapseProbe"
                end

                local function IsProbeCategoryNode(node)
                    return node:GetData().category == "Codex Collapse Category"
                end

                local function CurrentProvider()
                    return AddonList.ScrollBox:GetDataProvider()
                end

                local function FindProbeCategoryNode()
                    local _, node = CurrentProvider():FindByPredicate(
                        IsProbeCategoryNode,
                        TreeDataProviderConstants.IncludeCollapsed
                    )
                    return node
                end

                local function ProbeAddonVisible()
                    return CurrentProvider():ContainsByPredicate(
                        IsProbeAddonNode,
                        TreeDataProviderConstants.ExcludeCollapsed
                    )
                end

                local function ClickCategoryNode(node, frameName)
                    local entry = CreateFrame("Button", frameName, UIParent, "AddonListCategoryTemplate")
                    entry.CollapseExpand:SetTreeNode(node)
                    entry.CollapseExpand:Click()
                end

                AddonList:Show()
                AddonList_Update()
                local initiallyVisible = ProbeAddonVisible()

                ClickCategoryNode(FindProbeCategoryNode(), "AddonListCategoryCollapseEntry")
                local savedAfterCollapse = g_addonCategoriesCollapsed["Codex Collapse Category"] == true
                local collapsedVisible = ProbeAddonVisible()

                AddonList_Update()
                local rebuiltCollapsed = FindProbeCategoryNode():IsCollapsed() == true
                local rebuiltChildHidden = not ProbeAddonVisible()

                ClickCategoryNode(FindProbeCategoryNode(), "AddonListCategoryExpandEntry")
                local savedAfterExpand = g_addonCategoriesCollapsed["Codex Collapse Category"] == nil

                AddonList_Update()
                local rebuiltExpanded = FindProbeCategoryNode():IsCollapsed() ~= true
                local rebuiltChildVisible = ProbeAddonVisible()

                return initiallyVisible,
                       savedAfterCollapse,
                       collapsedVisible,
                       rebuiltCollapsed,
                       rebuiltChildHidden,
                       savedAfterExpand,
                       rebuiltExpanded,
                       rebuiltChildVisible
                "#,
            )
            .expect("AddonList category collapse persistence probe must run cleanly");

        assert!(
            initially_visible,
            "probe addon must start visible before its category is collapsed"
        );
        assert!(
            saved_after_collapse,
            "collapsing the category must store true in `g_addonCategoriesCollapsed`"
        );
        assert!(
            !collapsed_visible,
            "collapsing the category must hide child addon nodes immediately"
        );
        assert!(
            rebuilt_collapsed,
            "`AddonList_Update` must rebuild the category node as collapsed"
        );
        assert!(
            rebuilt_child_hidden,
            "`AddonList_Update` must keep child addon nodes hidden while saved collapsed"
        );
        assert!(
            saved_after_expand,
            "expanding the category must clear the saved collapsed entry"
        );
        assert!(
            rebuilt_expanded,
            "`AddonList_Update` must rebuild the category node expanded after clearing the saved var"
        );
        assert!(
            rebuilt_child_visible,
            "`AddonList_Update` must show child addon nodes after expansion"
        );
    });
}

type CategoryCollapseProbe = (bool, bool, bool, bool, bool, bool, bool, bool);

fn seed_category_probe_addon(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.state().borrow_mut().addons.push(AddonInfo {
        folder_name: PROBE_ADDON.into(),
        title: "Addon List Category Collapse Probe".into(),
        enabled: true,
        loaded: false,
        ..Default::default()
    });
}
