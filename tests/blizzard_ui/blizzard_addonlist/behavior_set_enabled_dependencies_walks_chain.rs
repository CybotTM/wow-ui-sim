//! AddonList enable-dependencies menu behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const TARGET_ADDON: &str = "AddonListDependencyTargetProbe";
const DIRECT_DEP: &str = "AddonListDependencyDirectProbe";
const TRANSITIVE_DEP: &str = "AddonListDependencyTransitiveProbe";

#[test]
fn enable_dependencies_menu_enables_target_and_transitive_dependencies() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_disabled_dependency_chain(env);

        let probe: EnableDependenciesProbe = env
            .eval(
                r#"
                local function FindAddonIndex(addonName)
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if C_AddOns.GetAddOnName(index) == addonName then
                            return index
                        end
                    end
                end

                local targetIndex = FindAddonIndex("AddonListDependencyTargetProbe")
                local directIndex = FindAddonIndex("AddonListDependencyDirectProbe")
                local transitiveIndex = FindAddonIndex("AddonListDependencyTransitiveProbe")
                local capturedEnableDependencies

                local createContextMenu = MenuUtil.CreateContextMenu
                MenuUtil.CreateContextMenu = function(owner, generator)
                    local rootDescription = {
                        SetTag = function() end,
                        CreateTitle = function() end,
                        CreateButton = function(_, text, callback)
                            if text == ADDON_LIST_ENABLE_DEPENDENCIES then
                                capturedEnableDependencies = callback
                            end
                        end,
                    }

                    generator(owner, rootDescription)
                end

                local treeNode = {
                    nodes = {},
                    GetData = function()
                        return { addonIndex = targetIndex }
                    end,
                }

                local entry = CreateFrame("Button", "AddonListEnableDependenciesEntry", UIParent, "AddonListEntryTemplate")
                AddonList_InitAddon(entry, treeNode)
                entry:OnClick("RightButton")
                capturedEnableDependencies()

                MenuUtil.CreateContextMenu = createContextMenu

                return capturedEnableDependencies ~= nil,
                       C_AddOns.GetAddOnEnableState(targetIndex, nil),
                       C_AddOns.GetAddOnEnableState(directIndex, nil),
                       C_AddOns.GetAddOnEnableState(transitiveIndex, nil)
                "#,
            )
            .expect("AddonList enable-dependencies probe must run cleanly");

        assert_enable_dependencies_probe(probe);
    });
}

type EnableDependenciesProbe = (bool, i64, i64, i64);

fn seed_disabled_dependency_chain(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.clear();
    state.addons.push(addon_info(TRANSITIVE_DEP, &[]));
    state.addons.push(addon_info(DIRECT_DEP, &[TRANSITIVE_DEP]));
    state.addons.push(addon_info(TARGET_ADDON, &[DIRECT_DEP]));
}

fn addon_info(folder_name: &str, dependencies: &[&str]) -> AddonInfo {
    AddonInfo {
        folder_name: folder_name.into(),
        title: folder_name.into(),
        enabled: false,
        loaded: false,
        dependencies: dependencies
            .iter()
            .map(|dependency| (*dependency).into())
            .collect(),
        ..Default::default()
    }
}

fn assert_enable_dependencies_probe(probe: EnableDependenciesProbe) {
    let (captured_menu_callback, target_state, direct_state, transitive_state) = probe;

    assert!(
        captured_menu_callback,
        "right-clicking an addon row must expose the enable-dependencies menu callback"
    );
    assert_eq!(
        target_state, 2,
        "`Enable Dependencies` must enable the target addon"
    );
    assert_eq!(
        direct_state, 2,
        "`Enable Dependencies` must enable the target's direct dependency"
    );
    assert_eq!(
        transitive_state, 2,
        "`Enable Dependencies` must enable transitive dependencies, not just direct dependencies"
    );
}
