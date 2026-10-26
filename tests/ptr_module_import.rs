#![cfg(feature = "client-ptr")]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
use wow_ui_sim::screen::ScreenKind;

#[test]
fn original_smart_navigation_and_debug_share_completed_utility_and_find_buttons() {
    let ui = blizzard_ui_dir();
    assert!(
        discover_blizzard_addons_for_screen(&ui, ScreenKind::Game)
            .iter()
            .any(|(name, _)| name == "Blizzard_GamepadSmartNavigation")
    );
    let env = new_blizzard_addon_env(&ui);
    let loaded =
        load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_GamepadSmartNavigation"], &[]);
    assert!(
        loaded
            .iter()
            .any(|name| name == "Blizzard_GamepadSmartNavigation")
    );
    let state = env.state().borrow();
    let import_errors: Vec<_> = state
        .lua_errors
        .iter()
        .filter(|error| {
            error.contains("Blizzard_GamepadSmartNavigation")
                && (error.contains("SmartNavigation.lua:1:") || error.contains("Debug.lua:1:"))
        })
        .collect();
    assert!(
        import_errors.is_empty(),
        "original import failed: {import_errors:?}"
    );
    drop(state);
    env.exec(
        r#"
        local function utilityUpvalue(fn)
            for index = 1, 50 do
                local name, value = debug.getupvalue(fn, index)
                if name == "Utility" then return value end
                if not name then break end
            end
            error("original consumer must capture Utility")
        end
        local Utility = utilityUpvalue(SmartNavigationMixin.RefreshButtonGroups)
        assert(rawequal(Utility, utilityUpvalue(SmartNavigationDebug.OnUpdate)))
        assert(rawequal(Utility, import("Blizzard_GamepadSmartNavigation.Utility")))
        collectgarbage("collect")
        assert(rawequal(Utility, import("Blizzard_GamepadSmartNavigation.Utility")))
        local source = debug.getinfo(Utility.FindButtonGroups, "S").source:gsub("\\", "/")
        assert(source:find("Blizzard_GamepadSmartNavigation/Utility.lua", 1, true))

        local panel = CreateFrame("Frame")
        panel:SetSize(200, 100)
        local first = CreateFrame("Button", nil, panel)
        first:SetSize(30, 20)
        local second = CreateFrame("Button", nil, panel)
        second:SetSize(40, 20)
        local ignored = CreateFrame("Frame", nil, panel)
        ignored:SetSize(50, 20)
        local nav = CreateFromMixins(SmartNavigationMixin)
        nav.activeInfo = { frame = panel }
        nav:RefreshButtonGroups()
        local group = nav.activeInfo.buttonGroups.mainGroup
        assert(#group.buttons == 2)
        assert(group.buttons[1] == first and group.buttons[2] == second)
        local visited = {}
        Utility.ForEachButtonInGroup(group, function(button) visited[button] = true end)
        assert(visited[first] and visited[second] and not visited[ignored])
        second:SetParent(nil)
        nav:RefreshButtonGroups()
        assert(#nav.activeInfo.buttonGroups.mainGroup.buttons == 1)
        assert(nav.activeInfo.buttonGroups.mainGroup.buttons[1] == first)
        "#,
    )
    .expect("original PTR consumers must import the completed original Utility table");
}

#[test]
fn original_smart_navigation_rejects_unfinished_utility_without_autoloading() {
    let ui = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui);
    load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_SharedXML"], &[]);
    let root = tempfile::tempdir().unwrap();
    let addon = root.path().join("Blizzard_GamepadSmartNavigation");
    std::fs::create_dir(&addon).unwrap();
    std::fs::copy(
        ui.join("Blizzard_GamepadSmartNavigation/SmartNavigation.lua"),
        addon.join("SmartNavigation.lua"),
    )
    .unwrap();
    // Utility exists on disk but is not completed by the TOC. Import must not load it.
    std::fs::copy(
        ui.join("Blizzard_GamepadSmartNavigation/Utility.lua"),
        addon.join("Utility.lua"),
    )
    .unwrap();
    let toc = addon.join("Blizzard_GamepadSmartNavigation.toc");
    std::fs::write(
        &toc,
        "## Title: Unfinished module probe\nSmartNavigation.lua\n",
    )
    .unwrap();
    let result = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("Invalid import: No module with that name exists")),
        "{:?}",
        result.warnings
    );
    assert!(env.eval::<bool>("SmartNavigationMixin == nil").unwrap());
}
