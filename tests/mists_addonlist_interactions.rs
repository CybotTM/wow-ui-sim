mod common;

#[cfg(feature = "client-mists")]
mod mists_addonlist_interactions {
    use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

    const ROOT: &str = "Blizzard_AddOnList";
    const PROBE_ADDON: &str = "MistsAddonListInteractionProbe";
    const FIND_ADDON_INDEX_LUA: &str = r#"
        local function FindAddonIndex(name)
            for index = 1, C_AddOns.GetNumAddOns() do
                if C_AddOns.GetAddOnName(index) == name then
                    return index
                end
            end
        end
    "#;
    const FIND_VISIBLE_ENTRY_LUA: &str = r#"
        local function FindVisibleEntry(addonIndex)
            for row = 1, MAX_ADDONS_DISPLAYED do
                local entry = _G["AddonListEntry" .. row]
                if entry and entry:IsShown() and entry:GetID() == addonIndex then
                    return entry
                end
            end
        end
    "#;
    const CAPTURE_ENTRY_STATE_LUA: &str = r#"
        local function CaptureEntryState(entry)
            local name = entry:GetName()
            local checkbox = _G[name .. "Enabled"]
            local title = _G[name .. "Title"]
            local reload = entry.Reload
            local status = entry.Status

            return title:GetText(),
                   checkbox:GetChecked() == true,
                   reload:IsShown() == true,
                   status:IsShown() == true
        end
    "#;
    const ROW_TOGGLE_PROBE_LUA: &str = r#"
        local addonIndex = assert(FindAddonIndex(probeName), "probe addon must be registered")
        C_AddOns.EnableAddOn(addonIndex)

        AddonList:Show()
        AddonList.startStatus[addonIndex] = true
        AddonList_Update()

        local entry = assert(FindVisibleEntry(addonIndex), "probe row must be visible")
        local initialTitle, initialChecked, initialReloadShown, initialStatusShown =
            CaptureEntryState(entry)

        _G[entry:GetName() .. "Enabled"]:Click()
        local afterClickTitle, afterClickChecked, afterClickReloadShown =
            CaptureEntryState(entry)
        local afterClickShouldReload = AddonList.shouldReload == true
        local afterClickOkayText = AddonList.OkayButton:GetText()

        AddonList.EnableAllButton:Click()
        local afterEnableAllTitle, afterEnableAllChecked, afterEnableAllReloadShown =
            CaptureEntryState(entry)

        return initialTitle,
               initialChecked,
               initialReloadShown,
               initialStatusShown,
               afterClickChecked,
               afterClickReloadShown,
               afterClickShouldReload,
               afterClickOkayText,
               afterEnableAllChecked,
               afterEnableAllReloadShown
    "#;

    #[test]
    fn mists_addonlist_row_toggle_updates_visible_reload_state() {
        with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
            register_probe_addon(env);

            let (
                initial_title,
                initial_checked,
                initial_reload_shown,
                initial_status_shown,
                after_click_checked,
                after_click_reload_shown,
                after_click_should_reload,
                after_click_okay_text,
                after_enable_all_checked,
                after_enable_all_reload_shown,
            ): (
                String,
                bool,
                bool,
                bool,
                bool,
                bool,
                bool,
                String,
                bool,
                bool,
            ) = env
                .eval(&row_toggle_probe())
                .expect("Mists AddOnList row toggle probe must run cleanly");

            assert_eq!(
                initial_title, PROBE_ADDON,
                "the visible AddOnList row must belong to the registered probe addon"
            );
            assert!(
                initial_checked,
                "the probe row checkbox must start enabled before the click"
            );
            assert!(
                initial_status_shown,
                "the unchanged probe row must show its status text before the click"
            );
            assert!(
                !initial_reload_shown,
                "the unchanged probe row must not show reload text before the click"
            );
            assert!(
                !after_click_checked,
                "clicking the probe row checkbox must visibly uncheck that row"
            );
            assert!(
                after_click_reload_shown,
                "clicking the probe row checkbox must show that row's reload affordance"
            );
            assert!(
                after_click_should_reload,
                "clicking the probe row checkbox must put AddonList into reload-pending state"
            );
            assert_eq!(
                after_click_okay_text, "Reload UI",
                "reload-pending AddOnList state must change the Okay button text"
            );
            assert!(
                after_enable_all_checked,
                "Enable All must visibly restore the probe row checkbox"
            );
            assert!(
                !after_enable_all_reload_shown,
                "Enable All must clear the probe row reload affordance after returning to the start state"
            );
        });
    }

    fn register_probe_addon(env: &wow_ui_sim::lua_api::WowLuaEnv) {
        env.exec(&format!("A_Admin.RegisterTestAddon({PROBE_ADDON:?})"))
            .expect("Mists AddOnList probe addon registration must run cleanly");
    }

    fn row_toggle_probe() -> String {
        [
            format!("local probeName = {PROBE_ADDON:?}"),
            FIND_ADDON_INDEX_LUA.to_string(),
            FIND_VISIBLE_ENTRY_LUA.to_string(),
            CAPTURE_ENTRY_STATE_LUA.to_string(),
            ROW_TOGGLE_PROBE_LUA.to_string(),
        ]
        .join("\n")
    }
}
