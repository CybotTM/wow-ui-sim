//! Load-on-demand dependency readiness behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const LOADED_DEP: &str = "AddonListLoadedDep";
const UNLOADED_DEP: &str = "AddonListUnloadedDep";
const READY_LOD_ADDON: &str = "AddonListReadyLod";
const BLOCKED_LOD_ADDON: &str = "AddonListBlockedLod";
const NON_LOD_ADDON: &str = "AddonListNotLod";

#[test]
fn is_addon_load_on_demand_requires_lod_flag_and_loaded_dependencies() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_dependency_probe_addons(env);

        let (non_lod_ready, ready_lod, blocked_lod): (bool, bool, bool) = env
            .eval(
                r#"
                local function FindAddonIndex(addonName)
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if C_AddOns.GetAddOnName(index) == addonName then
                            return index
                        end
                    end
                end

                return AddonList_IsAddOnLoadOnDemand(FindAddonIndex("AddonListNotLod")),
                       AddonList_IsAddOnLoadOnDemand(FindAddonIndex("AddonListReadyLod")),
                       AddonList_IsAddOnLoadOnDemand(FindAddonIndex("AddonListBlockedLod"))
                "#,
            )
            .expect("AddonList LOD dependency readiness probe must run cleanly");

        assert!(
            !non_lod_ready,
            "`AddonList_IsAddOnLoadOnDemand` must return false when the addon is not LOD"
        );
        assert!(
            ready_lod,
            "`AddonList_IsAddOnLoadOnDemand` must return true when all declared deps are loaded"
        );
        assert!(
            !blocked_lod,
            "`AddonList_IsAddOnLoadOnDemand` must return false when any declared dep is unloaded"
        );
    });
}

fn seed_dependency_probe_addons(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.push(addon_info(LOADED_DEP, false, true, &[]));
    state
        .addons
        .push(addon_info(UNLOADED_DEP, false, false, &[]));
    state
        .addons
        .push(addon_info(READY_LOD_ADDON, true, false, &[LOADED_DEP]));
    state.addons.push(addon_info(
        BLOCKED_LOD_ADDON,
        true,
        false,
        &[UNLOADED_DEP, LOADED_DEP],
    ));
    state
        .addons
        .push(addon_info(NON_LOD_ADDON, false, false, &[LOADED_DEP]));
}

fn addon_info(
    folder_name: &str,
    load_on_demand: bool,
    loaded: bool,
    dependencies: &[&str],
) -> AddonInfo {
    AddonInfo {
        folder_name: folder_name.into(),
        title: folder_name.into(),
        enabled: true,
        loaded,
        load_on_demand,
        dependencies: dependencies
            .iter()
            .map(|dependency| (*dependency).into())
            .collect(),
        ..Default::default()
    }
}
