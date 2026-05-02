//! Load-on-demand addon behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const NON_LOD_ADDON: &str = "AddonListNonLodProbe";
const LOD_ADDON: &str = "AddonListLodProbe";

#[test]
fn load_lod_addon_calls_load_and_sets_start_status_on_success() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_probe_addons(env);

        let (
            load_calls_after_non_lod,
            non_lod_start_status,
            load_calls_after_lod,
            loaded_arg_was_lod,
            lod_start_status,
        ): (i64, bool, i64, bool, bool) = env
            .eval(
                r#"
                local function FindAddonIndex(addonName)
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if C_AddOns.GetAddOnName(index) == addonName then
                            return index
                        end
                    end
                end

                local nonLodIndex = FindAddonIndex("AddonListNonLodProbe")
                local lodIndex = FindAddonIndex("AddonListLodProbe")
                AddonList.startStatus[nonLodIndex] = false
                AddonList.startStatus[lodIndex] = false

                local loadCalls = 0
                local loadedAddonArg = nil
                C_AddOns.LoadAddOn = function(addon)
                    loadCalls = loadCalls + 1
                    loadedAddonArg = addon
                    return true
                end

                local originalIsAddOnLoaded = C_AddOns.IsAddOnLoaded
                C_AddOns.IsAddOnLoaded = function(addon)
                    if addon == lodIndex then
                        return loadCalls > 0, loadCalls > 0
                    end
                    return originalIsAddOnLoaded(addon)
                end

                AddonList_LoadAddOn(nonLodIndex)
                local loadCallsAfterNonLod = loadCalls
                local nonLodStartStatus = AddonList.startStatus[nonLodIndex] == true

                AddonList_LoadAddOn(lodIndex)

                return loadCallsAfterNonLod,
                       nonLodStartStatus,
                       loadCalls,
                       loadedAddonArg == lodIndex,
                       AddonList.startStatus[lodIndex] == true
                "#,
            )
            .expect("AddonList LOD load probe must run cleanly");

        assert_eq!(
            load_calls_after_non_lod, 0,
            "`AddonList_LoadAddOn` must return before calling `C_AddOns.LoadAddOn` for non-LOD addons"
        );
        assert!(
            !non_lod_start_status,
            "`AddonList_LoadAddOn` must not set startStatus for non-LOD addons"
        );
        assert_eq!(
            load_calls_after_lod, 1,
            "`AddonList_LoadAddOn` must call `C_AddOns.LoadAddOn` for a load-on-demand addon"
        );
        assert!(
            loaded_arg_was_lod,
            "`AddonList_LoadAddOn` must pass the LOD addon's index to `C_AddOns.LoadAddOn`"
        );
        assert!(
            lod_start_status,
            "`AddonList_LoadAddOn` must set `AddonList.startStatus[index] = true` after a successful load"
        );
    });
}

fn seed_probe_addons(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.push(AddonInfo {
        folder_name: NON_LOD_ADDON.into(),
        title: NON_LOD_ADDON.into(),
        enabled: true,
        loaded: false,
        load_on_demand: false,
        ..Default::default()
    });
    state.addons.push(AddonInfo {
        folder_name: LOD_ADDON.into(),
        title: LOD_ADDON.into(),
        enabled: true,
        loaded: false,
        load_on_demand: true,
        dependencies: vec!["__BuiltIn".into()],
        ..Default::default()
    });
}
