//! Integration tests for Load-on-Demand panel loading via ShowUIPanel.
//!
//! Tests that LoD addons load correctly and populate their UI state when triggered
//! through the panel system or explicit LoadAddOn calls.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Blizzard addons needed for the panel system (dependency order).
const PANEL_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    ("Blizzard_SharedXMLGame", "Blizzard_SharedXMLGame.toc"),
    (
        "Blizzard_UIPanelTemplates",
        "Blizzard_UIPanelTemplates_Mainline.toc",
    ),
    (
        "Blizzard_FrameXMLBase",
        "Blizzard_FrameXMLBase_Mainline.toc",
    ),
    ("Blizzard_FrameEffects", "Blizzard_FrameEffects.toc"),
    ("Blizzard_LoadLocale", "Blizzard_LoadLocale.toc"),
    ("Blizzard_Fonts_Shared", "Blizzard_Fonts_Shared.toc"),
    ("Blizzard_HelpPlate", "Blizzard_HelpPlate.toc"),
    (
        "Blizzard_AccessibilityTemplates",
        "Blizzard_AccessibilityTemplates.toc",
    ),
    ("Blizzard_ObjectAPI", "Blizzard_ObjectAPI_Mainline.toc"),
    ("Blizzard_UIParent", "Blizzard_UIParent_Mainline.toc"),
    ("Blizzard_TextStatusBar", "Blizzard_TextStatusBar.toc"),
    ("Blizzard_MoneyFrame", "Blizzard_MoneyFrame_Mainline.toc"),
    ("Blizzard_POIButton", "Blizzard_POIButton.toc"),
    ("Blizzard_Flyout", "Blizzard_Flyout.toc"),
    ("Blizzard_StoreUI", "Blizzard_StoreUI_Mainline.toc"),
    ("Blizzard_MicroMenu", "Blizzard_MicroMenu_Mainline.toc"),
    ("Blizzard_EditMode", "Blizzard_EditMode.toc"),
    ("Blizzard_GarrisonBase", "Blizzard_GarrisonBase.toc"),
    ("Blizzard_GameTooltip", "Blizzard_GameTooltip_Mainline.toc"),
    (
        "Blizzard_UIParentPanelManager",
        "Blizzard_UIParentPanelManager_Mainline.toc",
    ),
    (
        "Blizzard_Settings_Shared",
        "Blizzard_Settings_Shared_Mainline.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Shared",
        "Blizzard_SettingsDefinitions_Shared.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Frame",
        "Blizzard_SettingsDefinitions_Frame_Mainline.toc",
    ),
    ("Blizzard_FrameXMLUtil", "Blizzard_FrameXMLUtil.toc"),
    ("Blizzard_Menu", "Blizzard_Menu.toc"),
    ("Blizzard_Minimap", "Blizzard_Minimap_Mainline.toc"),
    ("Blizzard_StaticPopup", "Blizzard_StaticPopup.toc"),
    ("Blizzard_TimeManager", "Blizzard_TimeManager_Mainline.toc"),
    ("Blizzard_ItemButton", "Blizzard_ItemButton_Mainline.toc"),
    ("Blizzard_QuickKeybind", "Blizzard_QuickKeybind.toc"),
    ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
    (
        "Blizzard_UIPanels_Game",
        "Blizzard_UIPanels_Game_Mainline.toc",
    ),
];

fn setup_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    seed_addon_search_paths(&env);
    load_panel_addons(&env);
    install_lua_harness_stubs(&env);

    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
}

fn seed_addon_search_paths(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addon_base_paths = vec![blizzard_ui_dir()];
}

fn load_panel_addons(env: &WowLuaEnv) {
    let ui = blizzard_ui_dir();
    for (name, toc) in PANEL_ADDONS {
        let toc_path = ui.join(name).join(toc);
        if !toc_path.exists() {
            continue;
        }
        if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }
}

fn install_lua_harness_stubs(env: &WowLuaEnv) {
    install_uiparent_load_addon_seam(env);
    install_action_button_util_stub(env);
}

/// Wrap `UIParentLoadAddOn` so the LoD panel tests don't drag in
/// `Blizzard_CooldownBroadcaster` by default (the addon's `ADDON_LOADED` /
/// `PLAYER_ENTERING_WORLD` handlers reach for `C_ChatInfo` / `C_Spell`
/// surface the panel tests don't bring up). The previous version returned
/// `false` straight from the wrapper, which silently bypassed the failure
/// bookkeeping the real Blizzard impl performs (`FailedAddOnLoad[name] =
/// true` plus the dialog message), and removed any dedicated seam for
/// future tests that *do* want to exercise `CooldownBroadcaster_LoadUI`
/// end-to-end.
///
/// The new seam:
///
/// * keeps the default skip behaviour for the broadcaster, so every
///   currently-passing test stays passing;
/// * routes ALL non-broadcaster `UIParentLoadAddOn` calls through the
///   original implementation untouched (no other addon is affected);
/// * mirrors the failure bookkeeping the real `UIParentLoadAddOn`
///   performs by populating both a global `FailedAddOnLoad` table and a
///   harness-visible `__test_uiparent_load_addon_failures` log so tests
///   can assert on the failure record;
/// * exposes a `__test_skip_cooldown_broadcaster_load` opt-out flag so a
///   dedicated coverage test (see
///   `cooldown_broadcaster_loadui_can_be_exercised_when_opt_in`) can flip
///   it to `false` and run `CooldownBroadcaster_LoadUI()` against the
///   real addon load path.
fn install_uiparent_load_addon_seam(env: &WowLuaEnv) {
    env.exec(
        r#"
        __test_uiparent_load_addon_failures = __test_uiparent_load_addon_failures or {}
        if __test_skip_cooldown_broadcaster_load == nil then
            __test_skip_cooldown_broadcaster_load = true
        end

        if type(UIParentLoadAddOn) == "function" and not __test_original_uiparent_load_addon then
            __test_original_uiparent_load_addon = UIParentLoadAddOn
            UIParentLoadAddOn = function(name)
                if name == "Blizzard_CooldownBroadcaster" and __test_skip_cooldown_broadcaster_load then
                    -- Mirror the real Blizzard_UIParent failure bookkeeping:
                    -- record the failure into FailedAddOnLoad (the global
                    -- name the test seam can observe — Blizzard's local-of-
                    -- the-same-name is not reachable from outside) and log
                    -- the {name, reason} into a harness-visible array so
                    -- tests can assert the broadcaster was indeed skipped.
                    FailedAddOnLoad = FailedAddOnLoad or {}
                    if not FailedAddOnLoad[name] then
                        FailedAddOnLoad[name] = true
                        __test_uiparent_load_addon_failures[
                            #__test_uiparent_load_addon_failures + 1
                        ] = { name = name, reason = "DISABLED_FOR_TESTS" }
                    end
                    return false
                end
                return __test_original_uiparent_load_addon(name)
            end
        end
        "#,
    )
    .expect("failed to install UIParentLoadAddOn harness seam");
}

/// Install the `ActionButtonUtil` namespace used by `Blizzard_SpellSearch`
/// (`SpellSearchUtil.GetActionBarStatusFor*`) and the talent / spellbook
/// item templates. Loading the real `Blizzard_ActionBar` addon would drag in
/// MainActionBar, the MultiBars, StanceBar, PetActionBar, and the C_ActionBar
/// API surface — none of which the LoD panel tests need. Instead we install a
/// narrower fixture that:
///
/// * publishes the `ActionBarActionStatus` enum literal Blizzard addons key
///   off (NotMissing / MissingFromAllBars / OnInactiveBonusBar /
///   OnDisabledActionBar);
/// * defaults the three `GetActionBarStatusFor{Spell,PetAction,Flyout}`
///   probes to `NotMissing` (matches the previous stub — keeps every
///   currently-passing test passing);
/// * exposes a per-id override table per probe (`__test_action_bar_status_for_spell`
///   / `_pet_action` / `_flyout`) so future tests that *want* to drive the
///   `MissingFromAllBars` / `OnInactiveBonusBar` / `OnDisabledActionBar`
///   branches in `SpellSearchUtil` can do so by writing a single Lua line
///   before invoking the panel.
fn install_action_button_util_stub(env: &WowLuaEnv) {
    env.exec(
        r#"
        if not ActionButtonUtil then
            ActionButtonUtil = {
                ActionBarActionStatus = {
                    NotMissing = 1,
                    MissingFromAllBars = 2,
                    OnInactiveBonusBar = 3,
                    OnDisabledActionBar = 4,
                },
            }

            __test_action_bar_status_for_spell = {}
            __test_action_bar_status_for_pet_action = {}
            __test_action_bar_status_for_flyout = {}

            function ActionButtonUtil.GetActionBarStatusForSpell(spellID)
                local override = spellID and __test_action_bar_status_for_spell[spellID]
                return override or ActionButtonUtil.ActionBarActionStatus.NotMissing
            end

            function ActionButtonUtil.GetActionBarStatusForPetAction(petActionID)
                local override = petActionID and __test_action_bar_status_for_pet_action[petActionID]
                return override or ActionButtonUtil.ActionBarActionStatus.NotMissing
            end

            function ActionButtonUtil.GetActionBarStatusForFlyout(flyoutActionID)
                local override = flyoutActionID and __test_action_bar_status_for_flyout[flyoutActionID]
                return override or ActionButtonUtil.ActionBarActionStatus.NotMissing
            end
        end
        "#,
    )
    .expect("failed to install ActionButtonUtil harness stub");
}

fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }
}

fn clear_recorded_lua_errors(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.lua_errors.clear();
    state.lua_error_records.clear();
    state.lua_error_counts.clear();
}

fn recorded_lua_errors(env: &WowLuaEnv) -> Vec<String> {
    env.state().borrow().lua_errors.clone()
}

fn player_spells_panel_debug_snapshot(env: &WowLuaEnv) -> String {
    if let Some(missing) = player_spells_frame_missing_marker(env) {
        return missing;
    }
    [
        player_spells_panel_settings_section(env),
        player_spells_subframe_names_section(env),
        player_spells_missing_onload_methods_section(env),
        player_spells_callback_invocation_section(env),
    ]
    .join("\n")
}

/// `Some("player_spells_frame=nil")` if the frame doesn't exist (matches
/// the original snapshot's nil-marker line); `None` otherwise so the
/// caller proceeds with the per-aspect probes.
fn player_spells_frame_missing_marker(env: &WowLuaEnv) -> Option<String> {
    let exists: bool = env
        .eval("return PlayerSpellsFrame ~= nil")
        .unwrap_or(false);
    (!exists).then(|| "player_spells_frame=nil".to_string())
}

/// Probes the `UIPanelWindows[PlayerSpellsFrame]` settings + the matching
/// frame attributes / scripts / tab tracker / `GetTab()` results — the
/// data that drives the panel-manager auto-minimize behaviour.
fn player_spells_panel_settings_section(env: &WowLuaEnv) -> String {
    eval_snapshot_section(env, PANEL_SETTINGS_SECTION_LUA)
}

/// Names + minimized/maximized widths of the three child panels —
/// the data we need to confirm subframe wiring loaded correctly.
fn player_spells_subframe_names_section(env: &WowLuaEnv) -> String {
    eval_snapshot_section(env, SUBFRAME_NAMES_SECTION_LUA)
}

/// Walks `PlayerSpellsFrame`'s subtree and lists every frame that has
/// an `OnLoad` script registered but no `OnLoad` method on the frame
/// table. Surfaces template-binding gaps that only manifest at script
/// dispatch time.
fn player_spells_missing_onload_methods_section(env: &WowLuaEnv) -> String {
    eval_snapshot_section(env, MISSING_ONLOAD_METHODS_SECTION_LUA)
}

/// Invokes the stored `autoMinimizeOnCondition` and `setMinimizedFunc`
/// callbacks under `pcall` and reports the (ok, result) pairs — useful
/// when the panel is registered but the callbacks themselves are the
/// thing crashing.
fn player_spells_callback_invocation_section(env: &WowLuaEnv) -> String {
    eval_snapshot_section(env, CALLBACK_INVOCATION_SECTION_LUA)
}

fn eval_snapshot_section(env: &WowLuaEnv, lua: &str) -> String {
    env.eval(lua)
        .unwrap_or_else(|error| format!("snapshot_error={error:?}"))
}

const PANEL_SETTINGS_SECTION_LUA: &str = r#"
    local panelSettings = UIPanelWindows and UIPanelWindows[PlayerSpellsFrame:GetName()]
    local storedAutoMinimize = panelSettings and panelSettings.autoMinimizeOnCondition
    local storedSetMinimized = panelSettings and panelSettings.setMinimizedFunc
    local frameAutoMinimize = PlayerSpellsFrame:GetAttribute("UIPanelLayout-autoMinimizeOnCondition")
    local frameSetMinimized = PlayerSpellsFrame:GetAttribute("UIPanelLayout-setMinimizedFunc")
    local onLoadScript = PlayerSpellsFrame:GetScript("OnLoad")
    local playerGetTabOk, playerGetTabResult = pcall(function()
        return PlayerSpellsFrame:GetTab()
    end)
    return table.concat({
        "PlayerSpellsFrame=" .. tostring(type(PlayerSpellsFrame)),
        "ShouldAutoMinimize=" .. tostring(type(PlayerSpellsFrame.ShouldAutoMinimize)),
        "SetMinimized=" .. tostring(type(PlayerSpellsFrame.SetMinimized)),
        "UIPanelWindows.PlayerSpellsFrame=" .. tostring(type(panelSettings)),
        "stored.autoMinimizeOnCondition=" .. tostring(type(storedAutoMinimize)),
        "stored.setMinimizedFunc=" .. tostring(type(storedSetMinimized)),
        "frame.autoMinimizeOnCondition=" .. tostring(type(frameAutoMinimize)),
        "frame.setMinimizedFunc=" .. tostring(type(frameSetMinimized)),
        "PlayerSpellsFrame.OnLoadScript=" .. tostring(type(onLoadScript)),
        "PlayerSpellsFrame.internalTabTracker=" .. tostring(type(PlayerSpellsFrame.internalTabTracker)),
        "PlayerSpellsFrame.minimizedWidth=" .. tostring(PlayerSpellsFrame.minimizedWidth),
        "PlayerSpellsFrame.maximizedWidth=" .. tostring(PlayerSpellsFrame.maximizedWidth),
        "PlayerSpellsFrame.GetTab()=" .. tostring(playerGetTabOk) .. ":" .. tostring(playerGetTabResult),
    }, "\n")
"#;

const SUBFRAME_NAMES_SECTION_LUA: &str = r#"
    local spellbookFrame = PlayerSpellsFrame.SpellBookFrame
    local spellbookGetTabOk, spellbookGetTabResult = pcall(function()
        if not spellbookFrame then
            return "missing"
        end
        return spellbookFrame:GetTab()
    end)
    local function frameName(frame) return frame and frame:GetName() or "missing" end
    return table.concat({
        "SpecFrame.name=" .. frameName(PlayerSpellsFrame.SpecFrame),
        "TalentsFrame.name=" .. frameName(PlayerSpellsFrame.TalentsFrame),
        "SpellBookFrame.name=" .. frameName(spellbookFrame),
        "SpellBookFrame.internalTabTracker=" .. (spellbookFrame and tostring(type(spellbookFrame.internalTabTracker)) or "missing"),
        "SpellBookFrame.minimizedWidth=" .. tostring(spellbookFrame and spellbookFrame.minimizedWidth or nil),
        "SpellBookFrame.maximizedWidth=" .. tostring(spellbookFrame and spellbookFrame.maximizedWidth or nil),
        "SpellBookFrame.GetTab()=" .. tostring(spellbookGetTabOk) .. ":" .. tostring(spellbookGetTabResult),
    }, "\n")
"#;

const MISSING_ONLOAD_METHODS_SECTION_LUA: &str = r#"
    local missingOnLoadMethods = {}
    local function childAliases(parent, child)
        if not debug or not debug.getfenv then
            return ""
        end
        local env = debug.getfenv(parent)
        local fields = env and env[1]
        if type(fields) ~= "table" then
            return ""
        end
        local aliases = {}
        for key, value in pairs(fields) do
            if value == child and type(key) == "string" then
                table.insert(aliases, key)
            end
        end
        table.sort(aliases)
        return table.concat(aliases, ",")
    end
    local function childSegment(index, child, alias)
        local childName = child.GetName and child:GetName() or nil
        if childName then
            if alias ~= "" then
                return childName .. ":" .. alias
            end
            return childName
        end
        return 'child_' .. tostring(index) .. (alias ~= "" and ':' .. alias or '')
    end
    local function appendMissing(frame, path)
        local frameOnLoadScript = frame.GetScript and frame:GetScript("OnLoad")
        if frameOnLoadScript and type(frame.OnLoad) ~= "function" then
            local objectType = frame.GetObjectType and frame:GetObjectType() or "?"
            table.insert(missingOnLoadMethods, path .. " type=" .. tostring(objectType) .. " OnLoad=" .. tostring(type(frame.OnLoad)))
        end
        local children = { frame:GetChildren() }
        for index, child in ipairs(children) do
            local alias = childAliases(frame, child)
            local segment = childSegment(index, child, alias)
            appendMissing(child, path .. "." .. segment)
        end
    end
    appendMissing(PlayerSpellsFrame, "PlayerSpellsFrame")
    return "missing.OnLoad.methods=" .. table.concat(missingOnLoadMethods, " | ")
"#;

const CALLBACK_INVOCATION_SECTION_LUA: &str = r#"
    local panelSettings = UIPanelWindows and UIPanelWindows[PlayerSpellsFrame:GetName()]
    local storedAutoMinimize = panelSettings and panelSettings.autoMinimizeOnCondition
    local storedSetMinimized = panelSettings and panelSettings.setMinimizedFunc
    local autoCallOk, autoCallResult = pcall(function()
        if type(storedAutoMinimize) ~= "function" then
            return "skip"
        end
        return storedAutoMinimize(PlayerSpellsFrame)
    end)
    local setCallOk, setCallResult = pcall(function()
        if type(storedSetMinimized) ~= "function" then
            return "skip"
        end
        return storedSetMinimized(PlayerSpellsFrame, false)
    end)
    return table.concat({
        "call.autoMinimizeOnCondition=" .. tostring(autoCallOk) .. ":" .. tostring(autoCallResult),
        "call.setMinimizedFunc=" .. tostring(setCallOk) .. ":" .. tostring(setCallResult),
    }, "\n")
"#;

#[test]
fn show_macro_frame_loads_and_populates_selector() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            if not ShowMacroFrame then
                return "missing_show_macro_frame"
            end

            ShowMacroFrame()

            if not MacroFrame or not MacroFrame:IsShown() then
                return "macro_frame_not_shown"
            end
            if PanelTemplates_GetSelectedTab(MacroFrame) ~= 1 then
                return "selected_tab=" .. tostring(PanelTemplates_GetSelectedTab(MacroFrame))
            end
            if MacroFrame.MacroSelector.numMacros ~= 2 then
                return "selector_count=" .. tostring(MacroFrame.MacroSelector.numMacros)
            end
            if MacroFrameSelectedMacroName:GetText() ~= "Raid Beacon" then
                return "selected_name=" .. tostring(MacroFrameSelectedMacroName:GetText())
            end
            if MacroFrameText:GetText() ~= "/rw Stack on star" then
                return "selected_body=" .. tostring(MacroFrameText:GetText())
            end
            if MacroFrame.SelectedMacroButton.Icon:GetTexture() ~= "Interface\\Icons\\INV_Misc_QuestionMark" then
                return "selected_icon=" .. tostring(MacroFrame.SelectedMacroButton.Icon:GetTexture())
            end

            local accountCount, characterCount = GetNumMacros()
            if accountCount ~= 2 or characterCount ~= 1 then
                return "counts=" .. tostring(accountCount) .. "," .. tostring(characterCount)
            end
            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "ShowMacroFrame should load and populate the macro selector: {result}");
    }
}

#[test]
fn keybind_s_loads_blizzard_player_spells_and_shows_spellbook() {
    test_timeout! {
        let env = setup_env();
        common::install_error_collector(&env, "__spellbook_keybind_errors");
        clear_recorded_lua_errors(&env);

        let result: String = env.eval(r#"
            local loadedBefore = C_AddOns.IsAddOnLoaded("Blizzard_PlayerSpells")
            if loadedBefore then
                return "addon_preloaded"
            end

            if not PlayerSpellsUtil or type(PlayerSpellsUtil.ToggleSpellBookFrame) ~= "function" then
                return "missing_toggle_spellbook_frame"
            end

            if GetBindingAction("S") ~= "" then
                return "unexpected_binding_store_seed"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "Test harness should start with Blizzard_PlayerSpells unloaded and the keybinding store unseeded: {result}"
        );

        env.send_key_press("S", None).expect("S keybind failed");

        let recorded_errors = recorded_lua_errors(&env);
        let handler_errors = common::drain_string_table(&env, "__spellbook_keybind_errors");
        assert!(
            recorded_errors.is_empty(),
            "Opening spellbook through S produced {} recorded Lua error(s):\n{:#?}\nhandler_errors:\n{}\n{}",
            recorded_errors.len(),
            recorded_errors,
            handler_errors.join("\n"),
            player_spells_panel_debug_snapshot(&env),
        );

        assert!(handler_errors.is_empty(), "Opening spellbook through S produced {} Lua error(s):\n{}", handler_errors.len(), handler_errors.join("\n"));

        let result: String = env.eval(r#"
            if not C_AddOns.IsAddOnLoaded("Blizzard_PlayerSpells") then
                return "addon_not_loaded"
            end
            if not PlayerSpellsFrame or not PlayerSpellsFrame:IsShown() then
                return "player_spells_not_shown"
            end
            if not PlayerSpellsFrame.SpellBookFrame or not PlayerSpellsFrame.SpellBookFrame:IsShown() then
                return "spellbook_tab_not_shown"
            end
            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "Pressing S should demand-load Blizzard_PlayerSpells and show the SpellBook tab: {result}"
        );
    }
}

#[test]
fn keybind_n_loads_blizzard_player_spells_and_shows_talents() {
    test_timeout! {
        let env = setup_env();
        common::install_error_collector(&env, "__talents_keybind_errors");
        clear_recorded_lua_errors(&env);

        let result: String = env.eval(r#"
            local loadedBefore = C_AddOns.IsAddOnLoaded("Blizzard_PlayerSpells")
            if loadedBefore then
                return "addon_preloaded"
            end

            if not PlayerSpellsUtil or type(PlayerSpellsUtil.ToggleClassTalentFrame) ~= "function" then
                return "missing_toggle_class_talent_frame"
            end

            if GetBindingAction("N") ~= "" then
                return "unexpected_binding_store_seed"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "Test harness should start with Blizzard_PlayerSpells unloaded and the keybinding store unseeded: {result}"
        );

        env.send_key_press("N", None).expect("N keybind failed");

        let recorded_errors = recorded_lua_errors(&env);
        let handler_errors = common::drain_string_table(&env, "__talents_keybind_errors");
        assert!(
            recorded_errors.is_empty(),
            "Opening talents through N produced {} recorded Lua error(s):\n{:#?}\nhandler_errors:\n{}\n{}",
            recorded_errors.len(),
            recorded_errors,
            handler_errors.join("\n"),
            player_spells_panel_debug_snapshot(&env),
        );

        assert!(handler_errors.is_empty(), "Opening talents through N produced {} Lua error(s):\n{}", handler_errors.len(), handler_errors.join("\n"));

        let result: String = env.eval(r#"
            if not C_AddOns.IsAddOnLoaded("Blizzard_PlayerSpells") then
                return "addon_not_loaded"
            end
            if not PlayerSpellsFrame or not PlayerSpellsFrame:IsShown() then
                return "player_spells_not_shown"
            end
            if not PlayerSpellsFrame.TalentsFrame or not PlayerSpellsFrame.TalentsFrame:IsShown() then
                return "talents_tab_not_shown"
            end
            if not (PlayerSpellsUtil and PlayerSpellsUtil.FrameTabs and PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.ClassTalents)) then
                return "talents_tab_not_active"
            end
            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "Pressing N should demand-load Blizzard_PlayerSpells and show the talents tab: {result}"
        );
    }
}

#[test]
#[ignore = "diagnostic"]
fn debug_player_spells_onload_subcalls() {
    let env = setup_env();
    let report: String = env
        .eval(
            r##"
            C_AddOns.LoadAddOn("Blizzard_PlayerSpells")

            local function attempt(label, fn)
                local ok, value = pcall(fn)
                return label .. "=" .. tostring(ok) .. ":" .. tostring(value)
            end

            local lines = {}
            local specFrame = PlayerSpellsFrame and PlayerSpellsFrame.SpecFrame
            local talentsFrame = PlayerSpellsFrame and PlayerSpellsFrame.TalentsFrame
            local spellBookFrame = PlayerSpellsFrame and PlayerSpellsFrame.SpellBookFrame

            table.insert(lines, attempt("spec.ClassSpecFrameMixin.OnLoad", function() return ClassSpecFrameMixin.OnLoad(specFrame) end))
            table.insert(lines, attempt("spec.UpdateSpecContents", function() return specFrame:UpdateSpecContents() end))
            table.insert(lines, attempt("spec.UpdateSpecFrame", function() return specFrame:UpdateSpecFrame() end))

            table.insert(lines, attempt("talents.ClassTalentsFrameMixin.OnLoad", function() return ClassTalentsFrameMixin.OnLoad(talentsFrame) end))
            table.insert(lines, attempt("talents.TalentFrameBaseMixin.OnLoad", function() return TalentFrameBaseMixin.OnLoad(talentsFrame) end))
            table.insert(lines, attempt("talents.UpdateClassVisuals", function() return talentsFrame:UpdateClassVisuals() end))
            table.insert(lines, attempt("talents.InitializeLoadSystem", function() return talentsFrame:InitializeLoadSystem() end))
            table.insert(lines, attempt("talents.InitializeSearch", function() return talentsFrame:InitializeSearch() end))
            table.insert(lines, attempt("talents.RefreshLoadoutOptions", function() return talentsFrame:RefreshLoadoutOptions() end))
            table.insert(lines, attempt("talents.RefreshConfigID", function() return talentsFrame:RefreshConfigID() end))
            table.insert(lines, attempt("talents.ClassTalentsFrameMixin.OnShow", function() return ClassTalentsFrameMixin.OnShow(talentsFrame) end))
            table.insert(lines, attempt("talents.TalentFrameBaseMixin.OnShow", function() return TalentFrameBaseMixin.OnShow(talentsFrame) end))
            table.insert(lines, attempt("talents.UpdateSpecBackground", function() return talentsFrame:UpdateSpecBackground() end))
            table.insert(lines, attempt("talents.CheckSetSelectedConfigID", function() return talentsFrame:CheckSetSelectedConfigID() end))
            table.insert(lines, attempt("talents.UpdateConfigButtonsState", function() return talentsFrame:UpdateConfigButtonsState() end))
            table.insert(lines, attempt("talents.UpdateAllButtons", function() return talentsFrame:UpdateAllButtons() end))
            table.insert(lines, attempt("talents.UpdateStarterBuildHighlights", function() return talentsFrame:UpdateStarterBuildHighlights() end))
            table.insert(lines, attempt("talents.HeroTalentsContainer.UpdateHeroTalentInfo", function() return talentsFrame.HeroTalentsContainer:UpdateHeroTalentInfo() end))
            table.insert(lines, attempt("talents.SetBackgroundAnimationsPlaying", function() return talentsFrame:SetBackgroundAnimationsPlaying(true) end))
            table.insert(lines, attempt("talents.CheckLoadSystemTutorials", function() return talentsFrame:CheckLoadSystemTutorials() end))
            table.insert(lines, attempt("talents.ClassTalentsFrameMixin.OnUpdate", function() return ClassTalentsFrameMixin.OnUpdate(talentsFrame, 0.016) end))
            table.insert(lines, attempt("talents.TalentFrameBaseMixin.OnUpdate", function() return TalentFrameBaseMixin.OnUpdate(talentsFrame, 0.016) end))
            table.insert(lines, attempt("talents.UpdateFullSearchResults", function() return talentsFrame:UpdateFullSearchResults() end))

            table.insert(lines, attempt("talents.LoadSystem.GetDropdown", function() return talentsFrame.LoadSystem:GetDropdown() end))
            table.insert(lines, attempt("talents.LoadSystem.dropdown.SetWidth", function()
                local dropdown = talentsFrame.LoadSystem:GetDropdown()
                return dropdown:SetWidth(200)
            end))
            table.insert(lines, attempt("talents.LoadSystem.SetMenuTag", function() return talentsFrame.LoadSystem:SetMenuTag("MENU_CLASS_TALENT_PROFILE") end))
            table.insert(lines, attempt("talents.LoadSystem.SetDropdownDefaultText", function()
                return talentsFrame.LoadSystem:SetDropdownDefaultText(WrapTextInColor(TALENT_FRAME_DROP_DOWN_DEFAULT, GRAY_FONT_COLOR))
            end))
            table.insert(lines, attempt("talents.LoadSystem.SetSelectionEnabled", function()
                local function SelectionEnabledCallback(selectionID, isUserInput)
                    return true
                end
                return talentsFrame.LoadSystem:SetSelectionEnabled(SelectionEnabledCallback)
            end))
            table.insert(lines, attempt("talents.LoadSystem.SetNewEntryCallbackCustomPopup", function()
                local function NewEntryCallback(entryName)
                    return nil
                end
                local function NewEntryDisabledCallback()
                    return false, "", nil, nil
                end
                return talentsFrame.LoadSystem:SetNewEntryCallbackCustomPopup(NewEntryCallback, TALENT_FRAME_DROP_DOWN_NEW_LOADOUT, ClassTalentLoadoutCreateDialog, NewEntryDisabledCallback)
            end))
            table.insert(lines, attempt("talents.LoadSystem.SetEditEntryCallback", function()
                local function EditLoadoutCallback(configID)
                end
                local function CanEditLoadoutCallback(configID)
                    return true
                end
                return talentsFrame.LoadSystem:SetEditEntryCallback(EditLoadoutCallback, TALENT_FRAME_DROP_DOWN_TOOLTIP_EDIT, CanEditLoadoutCallback)
            end))
            table.insert(lines, attempt("talents.LoadSystem.AddSentinelValue", function()
                return talentsFrame.LoadSystem:AddSentinelValue({ text = TALENT_FRAME_DROP_DOWN_IMPORT, color = WHITE_FONT_COLOR, callback = function() end })
            end))
            table.insert(lines, attempt("talents.LoadSystem.SetLoadCallback", function()
                local function LoadConfiguration(configID, isUserInput)
                end
                return talentsFrame.LoadSystem:SetLoadCallback(LoadConfiguration)
            end))

            table.insert(lines, attempt("spellbook.SpellBookFrameMixin.OnLoad", function() return SpellBookFrameMixin.OnLoad(spellBookFrame) end))
            table.insert(lines, attempt("spellbook.TabSystemOwnerMixin.OnLoad", function() return TabSystemOwnerMixin.OnLoad(spellBookFrame) end))
            table.insert(lines, attempt("spellbook.SetupSettingsDropdown", function() return spellBookFrame:SetupSettingsDropdown() end))
            table.insert(lines, attempt("spellbook.SpellBookFrameTutorialsMixin.OnLoad", function() return SpellBookFrameTutorialsMixin.OnLoad(spellBookFrame) end))
            table.insert(lines, attempt("spellbook.CategoryTabSystem.SetScript", function()
                return spellBookFrame.CategoryTabSystem:SetScript("OnSizeChanged", GenerateClosure(spellBookFrame.ResizeSearchBox, spellBookFrame))
            end))
            table.insert(lines, attempt("spellbook.CreateAndInit.ClassCategory", function()
                return CreateAndInitFromMixin(SpellBookClassCategoryMixin, spellBookFrame)
            end))
            table.insert(lines, attempt("spellbook.CreateAndInit.GeneralCategory", function()
                return CreateAndInitFromMixin(SpellBookGeneralCategoryMixin, spellBookFrame)
            end))
            table.insert(lines, attempt("spellbook.CreateAndInit.PetCategory", function()
                return CreateAndInitFromMixin(SpellBookPetCategoryMixin, spellBookFrame)
            end))
            table.insert(lines, attempt("spellbook.C_SpellBook.GetNumSpellBookSkillLines", function()
                return C_SpellBook.GetNumSpellBookSkillLines()
            end))
            table.insert(lines, attempt("spellbook.C_SpellBook.GetSpellBookSkillLineInfo.Class", function()
                local info = C_SpellBook.GetSpellBookSkillLineInfo(Enum.SpellBookSkillLineIndex.Class)
                return info and info.name or "nil"
            end))
            table.insert(lines, attempt("spellbook.C_SpellBook.GetSpellBookSkillLineInfo.General", function()
                local info = C_SpellBook.GetSpellBookSkillLineInfo(Enum.SpellBookSkillLineIndex.General)
                return info and info.name or "nil"
            end))
            table.insert(lines, attempt("spellbook.C_SpellBook.HasPetSpells", function()
                return C_SpellBook.HasPetSpells()
            end))
            table.insert(lines, attempt("spellbook.ClassCategory.InitDirect", function()
                local category = CreateFromMixins(SpellBookClassCategoryMixin)
                return category:Init(spellBookFrame)
            end))
            table.insert(lines, attempt("spellbook.GeneralCategory.InitDirect", function()
                local category = CreateFromMixins(SpellBookGeneralCategoryMixin)
                return category:Init(spellBookFrame)
            end))
            table.insert(lines, attempt("spellbook.PetCategory.InitDirect", function()
                local category = CreateFromMixins(SpellBookPetCategoryMixin)
                return category:Init(spellBookFrame)
            end))
            table.insert(lines, attempt("spellbook.PagedSpellsFrame.SetElementTemplateData", function()
                return spellBookFrame.PagedSpellsFrame:SetElementTemplateData(Templates)
            end))
            table.insert(lines, attempt("spellbook.PagedSpellsFrame.RegisterCallback", function()
                return spellBookFrame.PagedSpellsFrame:RegisterCallback(PagedContentFrameBaseMixin.Event.OnUpdate, spellBookFrame.OnPagedSpellsUpdate, spellBookFrame)
            end))
            table.insert(lines, attempt("spellbook.EventRegistry.ClickBinding", function()
                return EventRegistry:RegisterCallback("ClickBindingFrame.UpdateFrames", spellBookFrame.OnClickBindingUpdate, spellBookFrame)
            end))
            table.insert(lines, attempt("spellbook.EventRegistry.AssistedCombatActionSpell", function()
                return EventRegistry:RegisterCallback("AssistedCombatManager.OnSetActionSpell", function(o)
                    spellBookFrame:UpdateAttic()
                    spellBookFrame:CheckShowHelpTips()
                end)
            end))
            table.insert(lines, attempt("spellbook.EventRegistry.AssistedCombatHighlight", function()
                return EventRegistry:RegisterCallback("AssistedCombatManager.OnSetCanHighlightSpellbookSpells", spellBookFrame.MarkSpellDataDirty, spellBookFrame)
            end))
            table.insert(lines, attempt("spellbook.SetButtonHoverCallbacks", function()
                local onPagingButtonEnter = GenerateClosure(spellBookFrame.OnPagingButtonEnter, spellBookFrame)
                local onPagingButtonLeave = GenerateClosure(spellBookFrame.OnPagingButtonLeave, spellBookFrame)
                return spellBookFrame.PagedSpellsFrame.PagingControls:SetButtonHoverCallbacks(onPagingButtonEnter, onPagingButtonLeave)
            end))
            table.insert(lines, attempt("spellbook.BookCornerFlipbook.PlayPause", function()
                spellBookFrame.BookCornerFlipbook.Anim:Play()
                return spellBookFrame.BookCornerFlipbook.Anim:Pause()
            end))
            table.insert(lines, attempt("spellbook.InitializeSearch", function() return spellBookFrame:InitializeSearch() end))

            return table.concat(lines, "\n")
            "##,
        )
        .expect("diagnostic evaluation should return");
    panic!("{report}");
}

#[test]
#[ignore = "diagnostic"]
fn debug_keybind_n_nil_width_callsite() {
    let env = setup_env();
    clear_recorded_lua_errors(&env);

    let report: String = env
        .eval(
            r##"
            local ok, result = xpcall(
                function()
                    PlayerSpellsUtil.ToggleClassTalentFrame()
                    return "ok"
                end,
                function(msg)
                    return tostring(msg) .. "\n" .. debug.traceback()
                end
            )

            return "ok=" .. tostring(ok) .. "\nresult=" .. tostring(result)
            "#,
        )
        .expect("diagnostic xpcall should return");
    let recorded_errors = recorded_lua_errors(&env);

    panic!(
        "{report}\nrecorded_errors={recorded_errors:#?}\n{}",
        player_spells_panel_debug_snapshot(&env),
    );
}

#[test]
#[ignore = "diagnostic"]
fn debug_player_spells_visible_onupdate_handlers() {
    let env = setup_env();
    let report: String = env
        .eval(
            r##"
            PlayerSpellsUtil.ToggleClassTalentFrame()

            local lines = {}
            local function visit(frame, path)
                if not frame then
                    return
                end

                local onUpdate = frame.GetScript and frame:GetScript("OnUpdate")
                if type(onUpdate) == "function" and frame.IsVisible and frame:IsVisible() then
                    local ok, value = pcall(onUpdate, frame, 0.016)
                    table.insert(lines, path .. ".OnUpdate=" .. tostring(ok) .. ":" .. tostring(value))
                end

                local children = frame.GetChildren and { frame:GetChildren() } or {}
                for index, child in ipairs(children) do
                    local childName = child.GetName and child:GetName() or ("child_" .. tostring(index))
                    visit(child, path .. "." .. tostring(childName))
                end
            end

            visit(PlayerSpellsFrame, "PlayerSpellsFrame")
            return table.concat(lines, "\n")
            "##,
        )
        .expect("diagnostic OnUpdate scan should return");

    panic!("{report}");
}

#[test]
#[ignore = "diagnostic"]
fn debug_player_spells_nil_numeric_calls() {
    let env = setup_env();
    let report: String = env
        .eval(
            r##"
            local lines = {}

            local function wrap_nil_arg(namespace, fnName)
                local original = namespace and namespace[fnName]
                if type(original) ~= "function" then
                    table.insert(lines, fnName .. "=missing")
                    return
                end

                namespace[fnName] = function(...)
                    local arg1 = select(1, ...)
                    local arg2 = select(2, ...)
                    if arg1 == nil or arg2 == nil then
                        table.insert(
                            lines,
                            fnName
                                .. "(arg1="
                                .. tostring(arg1)
                                .. ",arg2="
                                .. tostring(arg2)
                                .. ",argc="
                                .. tostring(select("#", ...))
                                .. ")"
                        )
                    end
                    return original(...)
                end
            end

            wrap_nil_arg(C_Traits, "GetDefinitionInfo")
            wrap_nil_arg(C_Traits, "GetEntryInfo")
            wrap_nil_arg(C_Traits, "GetSubTreeInfo")
            wrap_nil_arg(C_Traits, "GetConditionInfo")
            wrap_nil_arg(C_Traits, "GetNodeInfo")
            wrap_nil_arg(C_Traits, "GetNodeCost")
            wrap_nil_arg(C_Traits, "GetTraitCurrencyInfo")
            wrap_nil_arg(C_Traits, "GetTreeInfo")
            wrap_nil_arg(C_Traits, "GetTreeCurrencyInfo")
            wrap_nil_arg(C_ClassTalents, "GetHeroTalentSpecsForClassSpec")
            wrap_nil_arg(C_ClassTalents, "GetConfigIDsBySpecID")
            wrap_nil_arg(C_ClassTalents, "GetLastSelectedSavedConfigID")
            wrap_nil_arg(C_ClassTalents, "UpdateLastSelectedSavedConfigID")
            wrap_nil_arg(C_ClassTalents, "GetTraitTreeForSpec")
            wrap_nil_arg(C_CurrencyInfo, "GetCurrencyInfo")
            wrap_nil_arg(C_SpecializationInfo, "GetSpecialization")
            wrap_nil_arg(C_SpecializationInfo, "GetSpecializationInfo")
            wrap_nil_arg(C_SpecializationInfo, "GetClassIDFromSpecID")
            wrap_nil_arg(C_SpecializationInfo, "GetPvpTalentSlotInfo")
            wrap_nil_arg(C_SpecializationInfo, "GetPvpTalentInfo")
            wrap_nil_arg(C_SpecializationInfo, "GetPvpTalentSlotUnlockLevel")
            wrap_nil_arg(C_Texture, "GetAtlasInfo")
            wrap_nil_arg(C_Spell, "IsSpellPassive")
            wrap_nil_arg(C_Spell, "GetSpellLink")
            wrap_nil_arg(C_Spell, "GetSpellTexture")
            wrap_nil_arg(C_SpellBook, "GetSpellBookItemInfo")

            PlayerSpellsUtil.ToggleClassTalentFrame()

            return table.concat(lines, "\n")
            "##,
        )
        .expect("diagnostic nil-arg scan should return");

    panic!("{report}");
}

#[test]
fn show_mail_frame_loads_and_populates_inbox_rows() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            A_Admin.ClearInbox()
            A_Admin.AddMail("Thrall", "Unread Orders", "Meet me in Orgrimmar.")
            A_Admin.AddMail("Jaina", "Arcane Invoice", "The Kirin Tor still expects payment.")

            local loaded, reason = C_AddOns.LoadAddOn("Blizzard_MailFrame")
            if not loaded then
                return "load_failed:" .. tostring(reason)
            end
            if not MailFrame or not MailFrame_Show then
                return "mail_frame_missing"
            end

            MailFrame_Show()

            if not MailFrame:IsShown() then
                return "mail_frame_not_shown"
            end
            if not InboxFrame or not InboxFrame:IsShown() then
                return "inbox_frame_not_shown"
            end
            if C_Mail.GetNumItems() ~= 2 then
                return "c_mail_count=" .. tostring(C_Mail.GetNumItems())
            end

            local firstButton = MailItem1Button
            if not firstButton or not firstButton:IsShown() then
                return "mail_item_1_not_shown"
            end
            if firstButton.index ~= 1 then
                return "mail_item_1_index=" .. tostring(firstButton.index)
            end
            if MailItem1Sender:GetText() ~= "Thrall" then
                return "mail_item_1_sender=" .. tostring(MailItem1Sender:GetText())
            end
            if MailItem1Subject:GetText() ~= "Unread Orders" then
                return "mail_item_1_subject=" .. tostring(MailItem1Subject:GetText())
            end
            if not MailItem2Button or not MailItem2Button:IsShown() then
                return "mail_item_2_not_shown"
            end
            if MailItem2Sender:GetText() ~= "Jaina" then
                return "mail_item_2_sender=" .. tostring(MailItem2Sender:GetText())
            end

            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "ShowMailFrame should load the inbox panel and populate seeded inbox rows: {result}");
    }
}

#[test]
fn item_socketing_frame_loads_and_populates_socket_buttons() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            C_ItemSocketInfo._state.uiType = Enum.ItemSocketInfoUIType.ItemSocketingUI or 0
            C_ItemSocketInfo._state.isOpen = true
            C_ItemSocketInfo._state.numSockets = 2
            C_ItemSocketInfo._state.itemInfo = {
                name = "Socketed Helm",
                icon = 901,
                itemID = 6948,
                link = "item:6948",
                quality = 4,
                isRefundable = false,
                isBoundTradeable = true,
            }
            C_ItemSocketInfo._state.socketTypes = {
                [1] = "Red",
                [2] = "Blue",
            }
            C_ItemSocketInfo._state.existingSockets = {
                [1] = {
                    name = "Ruby",
                    icon = 111,
                    itemID = 6948,
                    gemMatchesSocket = true,
                    link = "item:6948",
                },
            }
            C_ItemSocketInfo._state.newSockets = {
                [2] = {
                    name = "Bound Sapphire",
                    icon = 222,
                    itemID = 6948,
                    gemMatchesSocket = false,
                    link = "item:6948",
                    isBound = true,
                },
            }
            C_ItemSocketInfo._state.hasBoundGemProposed = true

            local loaded, reason = C_AddOns.LoadAddOn("Blizzard_ItemSocketingUI")
            if not loaded then
                return "load_failed:" .. tostring(reason)
            end

            if ItemSocketingFrame:GetScript("OnEvent") == nil then
                return "missing_on_event"
            end

            ItemSocketingFrame:GetScript("OnEvent")(ItemSocketingFrame, "SOCKET_INFO_UPDATE")

            if not ItemSocketingFrame:IsShown() then
                return "frame_not_shown"
            end
            if not ItemSocketingFrame.SocketingContainer.Socket1:IsShown() then
                return "socket1_hidden"
            end
            if not ItemSocketingFrame.SocketingContainer.Socket2:IsShown() then
                return "socket2_hidden"
            end
            if ItemSocketingFrame.SocketingContainer.Socket1.Icon:GetTexture() ~= 111 then
                return "socket1_icon=" .. tostring(ItemSocketingFrame.SocketingContainer.Socket1.Icon:GetTexture())
            end
            if ItemSocketingFrame.SocketingContainer.Socket2.Icon:GetTexture() ~= 222 then
                return "socket2_icon=" .. tostring(ItemSocketingFrame.SocketingContainer.Socket2.Icon:GetTexture())
            end
            if not ItemSocketingFrame.SocketingContainer.Socket1.Shine:IsShown() then
                return "socket1_shine_hidden"
            end
            if not ItemSocketingFrame.SocketingContainer.itemIsBoundTradeable then
                return "bound_tradeable_flag_missing"
            end
            if ItemSocketingFrame.SocketingContainer.ApplySocketsButton:IsEnabled() ~= true then
                return "apply_disabled"
            end

            local socket1 = ItemSocketingFrame.SocketingContainer.Socket1
            socket1:GetScript("OnEnter")(socket1)
            if not GameTooltip:IsShown() then
                return "tooltip_not_shown"
            end
            if not GameTooltip:IsOwned(socket1) then
                return "tooltip_not_owned"
            end
            if GameTooltip:NumLines() == 0 then
                return "tooltip_empty"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result, "ok",
            "ItemSocketingFrame should load and populate from seeded C_ItemSocketInfo state: {result}"
        );
    }
}

#[test]
fn encounter_timeline_loads_and_populates_track_view_event_frames() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            local loaded, reason = C_AddOns.LoadAddOn("Blizzard_EncounterTimeline")
            if not loaded then
                return "load_failed:" .. tostring(reason)
            end
            if not EncounterTimeline or not EncounterTimeline.TrackView then
                return "encounter_timeline_missing"
            end

            EncounterTimeline:UpdateSystemSettingViewType()
            EncounterTimeline:SetExplicitlyShown(true)
            EncounterTimeline.TrackView:ActivateView()
            EncounterTimeline:UpdateVisibility()

            if not EncounterTimeline:IsShown() then
                return "timeline_not_shown"
            end
            if not EncounterTimeline.TrackView:IsShown() then
                return "track_view_not_shown"
            end
            if EncounterTimeline.TrackView:GetTrackCount() < 3 then
                return "track_count=" .. tostring(EncounterTimeline.TrackView:GetTrackCount())
            end
            if EncounterTimeline.TrackView:GetActiveEventFrameCount() ~= 1 then
                return "active_frame_count=" .. tostring(EncounterTimeline.TrackView:GetActiveEventFrameCount())
            end

            local eventFrame = EncounterTimeline.TrackView:GetEventFrame(1)
            if not eventFrame then
                return "missing_event_frame"
            end
            if not eventFrame:IsShown() then
                return "event_frame_not_shown"
            end
            if not eventFrame.IconContainer or not eventFrame.IconContainer.IconTexture then
                return "missing_icon_container"
            end
            if not eventFrame.IconContainer.IconTexture:GetTexture() then
                return "missing_icon_texture"
            end

            local eventInfo = eventFrame:GetEventInfo()
            if not eventInfo or eventInfo.spellID ~= 19750 then
                return "event_info_spell_id=" .. tostring(eventInfo and eventInfo.spellID)
            end

            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "EncounterTimeline should load and populate a visible track event frame: {result}");
    }
}

#[test]
fn damage_meter_loads_and_populates_primary_session_window() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            SetCVar("damageMeterEnabled", "1")

            local loaded, reason = C_AddOns.LoadAddOn("Blizzard_DamageMeter")
            if not loaded then
                return "load_failed:" .. tostring(reason)
            end
            if not DamageMeter then
                return "damage_meter_missing"
            end

            DamageMeter:OnVariablesLoaded()
            DamageMeter:SetShown(true)

            local sessionWindow = DamageMeter:GetPrimarySessionWindow()
            if not sessionWindow then
                return "primary_window_missing"
            end

            sessionWindow:Refresh(ScrollBoxConstants.DiscardScrollPosition)

            if not DamageMeter:IsShown() then
                return "damage_meter_hidden"
            end
            if not sessionWindow:IsShown() then
                return "session_window_hidden"
            end
            if sessionWindow:GetEntryFrameCount() == 0 then
                return "entry_frame_count=0"
            end

            local entryData = sessionWindow:GetScrollBox():GetDataProvider():Find(1)
            if not entryData then
                return "missing_entry_data"
            end
            if entryData.name ~= "Player" then
                return "entry_name=" .. tostring(entryData.name)
            end
            if entryData.totalAmount ~= 52000 then
                return "entry_total=" .. tostring(entryData.totalAmount)
            end
            if entryData.amountPerSecond ~= 1300 then
                return "entry_per_second=" .. tostring(entryData.amountPerSecond)
            end
            if not entryData.isLocalPlayer then
                return "entry_not_local_player"
            end

            sessionWindow:ShowSourceWindow(entryData)
            local sourceWindow = sessionWindow:GetSourceWindow()
            if not sourceWindow:IsShown() then
                return "source_window_hidden"
            end
            if sourceWindow:GetEntryFrameCount() == 0 then
                return "source_entry_frame_count=0"
            end

            local spellData = sourceWindow:GetScrollBox():GetDataProvider():Find(1)
            if not spellData then
                return "missing_spell_data"
            end
            if spellData.spellID ~= 19750 then
                return "spell_id=" .. tostring(spellData.spellID)
            end
            if spellData.totalAmount ~= 52000 then
                return "spell_total=" .. tostring(spellData.totalAmount)
            end

            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "DamageMeter should load and populate the primary session and source windows: {result}");
    }
}

#[test]
fn audio_settings_register_seeded_output_device_dropdown_options() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            if not Settings or not Settings.AUDIO_CATEGORY_ID then
                return "audio_category_missing"
            end

            local category = Settings.GetCategory(Settings.AUDIO_CATEGORY_ID)
            if not category then
                return "audio_category_lookup_failed"
            end

            local layout = SettingsPanel:GetLayout(category)
            if not layout then
                return "audio_layout_missing"
            end

            local outputInitializer = nil
            for _, initializer in ipairs(layout:GetInitializers()) do
                local setting = initializer.GetSetting and initializer:GetSetting()
                if setting and setting:GetVariable() == "Sound_OutputDriverIndex" then
                    outputInitializer = initializer
                    break
                end
            end

            if not outputInitializer then
                return "output_initializer_missing"
            end

            local optionsFunc = outputInitializer:GetOptions()
            if type(optionsFunc) ~= "function" then
                return "options_func_type=" .. tostring(type(optionsFunc))
            end

            local options = optionsFunc()
            if #options ~= 1 then
                return "option_count=" .. tostring(#options)
            end
            if options[1].value ~= 0 then
                return "option_value=" .. tostring(options[1].value)
            end
            if options[1].label ~= "Silent Output Device" then
                return "option_label=" .. tostring(options[1].label)
            end

            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "Audio settings should register a seeded output-device dropdown option: {result}");
    }
}

#[test]
fn settings_open_to_interface_category_opens_settings_panel() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            if not Settings or not Settings.OpenToCategory then
                return "missing_settings_open_to_category"
            end
            if not Settings.INTERFACE_CATEGORY_ID then
                return "missing_interface_category_id"
            end

            Settings.OpenToCategory(Settings.INTERFACE_CATEGORY_ID)
            if not SettingsPanel or not SettingsPanel:IsShown() then
                return "settings_panel_not_shown"
            end

            local currentCategory = SettingsPanel.GetCurrentCategory and SettingsPanel:GetCurrentCategory()
            if not currentCategory then
                return "current_category_missing"
            end
            if currentCategory:GetID() ~= Settings.INTERFACE_CATEGORY_ID then
                return "current_category=" .. tostring(currentCategory:GetID())
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "Settings.OpenToCategory(Settings.INTERFACE_CATEGORY_ID) should open SettingsPanel on the interface category: {result}"
        );
    }
}

#[test]
fn professions_frame_loads_and_populates_specialization_tab() {
    test_timeout! {
        let env = setup_env();
        let ui = blizzard_ui_dir();
        for (name, toc) in [
            ("Blizzard_FrameXMLUtil", "Blizzard_FrameXMLUtil.toc"),
            ("Blizzard_ProfessionsTemplates", "Blizzard_ProfessionsTemplates.toc"),
            ("Blizzard_SharedTalentUI", "Blizzard_SharedTalentUI.toc"),
            ("Blizzard_Professions", "Blizzard_Professions.toc"),
        ] {
            let toc_path = ui.join(name).join(toc);
            if let Err(error) = load_addon(&env.loader_env(), &toc_path) {
                panic!("failed to load {name}: {error}");
            }
        }
        let result: String = env.eval(r#"
            if not ProfessionsFrame or not ProfessionsFrame.SpecPage or not Professions then
                return "professions_spec_page_missing"
            end

            local professionInfo = Professions.GetProfessionInfo()
            if not professionInfo or professionInfo.professionID ~= 164 then
                return "profession_info=" .. tostring(professionInfo and professionInfo.professionID)
            end

            ProfessionsFrame.SpecPage:Refresh(professionInfo)

            local selectedTab = ProfessionsFrame.SpecPage.selectedTab
            if not selectedTab then
                return "selected_spec_tab_missing"
            end
            if not selectedTab.tabInfo or not selectedTab.tabInfo.rootNodeID then
                return "selected_tab_root_missing"
            end

            local rootNodeID = selectedTab.tabInfo.rootNodeID
            if ProfessionsFrame.SpecPage:GetTalentTreeID() ~= selectedTab.traitTreeID then
                return "talent_tree_id=" .. tostring(ProfessionsFrame.SpecPage:GetTalentTreeID())
            end
            local children = C_ProfSpecs.GetChildrenForPath(rootNodeID)
            if #children == 0 then
                return "root_children=0"
            end

            if not ProfessionsFrame.SpecPage:IsTreeDirty() then
                return "tree_not_marked_dirty"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "ProfessionsFrame should show a populated specialization tab: {result}");
    }
}

#[test]
fn loss_of_control_frame_shows_seeded_overlay_on_added_event() {
    test_timeout! {
        let env = setup_env();
        let _ = env.fire_event_with_args(
            "LOSS_OF_CONTROL_ADDED",
            &[
                env.lua_string("player"),
                rilua::Val::Num(1.0),
            ],
        );

        let result: String = env.eval(r#"
            if not LossOfControlFrame then
                return "missing_frame"
            end
            if not LossOfControlFrame:IsShown() then
                return "frame_hidden"
            end
            if LossOfControlFrame.AbilityName:GetText() ~= "Kidney Shot" then
                return "text=" .. tostring(LossOfControlFrame.AbilityName:GetText())
            end
            if LossOfControlFrame.Icon:GetTexture() ~= "Interface\\Icons\\Ability_Rogue_KidneyShot" then
                return "icon=" .. tostring(LossOfControlFrame.Icon:GetTexture())
            end
            if not LossOfControlFrame.TimeLeft:IsShown() then
                return "time_hidden"
            end
            return "ok"
        "#).unwrap();

        assert_eq!(
            result, "ok",
            "LossOfControlFrame should show the seeded loss-of-control overlay: {result}"
        );
    }
}

/// Verifies the `ActionButtonUtil` test fixture: defaults remain `NotMissing`
/// (so existing panel tests don't change behaviour) but per-id override tables
/// can drive `MissingFromAllBars` / `OnInactiveBonusBar` / `OnDisabledActionBar`
/// through `SpellSearchUtil.GetActionbarStatusForSpell`. Without these branches
/// the spell-book / talent search-filter UI cannot be exercised end-to-end.
#[test]
fn action_button_util_fixture_drives_all_action_bar_statuses() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            local enum = ActionButtonUtil and ActionButtonUtil.ActionBarActionStatus
            if not enum then
                return "missing_action_bar_action_status_enum"
            end
            if enum.NotMissing ~= 1 or enum.MissingFromAllBars ~= 2
               or enum.OnInactiveBonusBar ~= 3 or enum.OnDisabledActionBar ~= 4 then
                return "wrong_enum_values"
            end

            -- Defaults: nothing configured → NotMissing for every probe.
            if ActionButtonUtil.GetActionBarStatusForSpell(101) ~= enum.NotMissing then
                return "default_spell_not_NotMissing"
            end
            if ActionButtonUtil.GetActionBarStatusForPetAction(202) ~= enum.NotMissing then
                return "default_pet_not_NotMissing"
            end
            if ActionButtonUtil.GetActionBarStatusForFlyout(303) ~= enum.NotMissing then
                return "default_flyout_not_NotMissing"
            end

            -- Overrides drive each non-default branch SpellSearchUtil cares about.
            __test_action_bar_status_for_spell[1001] = enum.MissingFromAllBars
            __test_action_bar_status_for_spell[1002] = enum.OnInactiveBonusBar
            __test_action_bar_status_for_spell[1003] = enum.OnDisabledActionBar
            __test_action_bar_status_for_pet_action[2001] = enum.OnDisabledActionBar
            __test_action_bar_status_for_flyout[3001] = enum.OnInactiveBonusBar

            if ActionButtonUtil.GetActionBarStatusForSpell(1001) ~= enum.MissingFromAllBars then
                return "spell_override_MissingFromAllBars_failed"
            end
            if ActionButtonUtil.GetActionBarStatusForSpell(1002) ~= enum.OnInactiveBonusBar then
                return "spell_override_OnInactiveBonusBar_failed"
            end
            if ActionButtonUtil.GetActionBarStatusForSpell(1003) ~= enum.OnDisabledActionBar then
                return "spell_override_OnDisabledActionBar_failed"
            end
            if ActionButtonUtil.GetActionBarStatusForPetAction(2001) ~= enum.OnDisabledActionBar then
                return "pet_override_failed"
            end
            if ActionButtonUtil.GetActionBarStatusForFlyout(3001) ~= enum.OnInactiveBonusBar then
                return "flyout_override_failed"
            end

            -- The overrides must flow through SpellSearchUtil's wrappers
            -- (this is the path real Blizzard search-filter UI exercises).
            if SpellSearchUtil then
                if SpellSearchUtil.GetActionbarStatusForSpell(1001) ~= enum.MissingFromAllBars then
                    return "spellsearch_spell_override_failed"
                end

                local talentNode = { entryIDsWithCommittedRanks = { 1 } }
                if SpellSearchUtil.GetActionBarStatusForTraitNode(talentNode, 1002) ~= enum.OnInactiveBonusBar then
                    return "spellsearch_trait_override_failed"
                end
                if SpellSearchUtil.GetActionBarStatusForTraitNodeEntry(1, talentNode, 1003) ~= enum.OnDisabledActionBar then
                    return "spellsearch_trait_entry_override_failed"
                end

                -- And the tooltip lookup table on SpellSearchUtil should index
                -- non-`NotMissing` statuses (sanity: confirms the enum keys
                -- match the ones SpellSearchUtil built its lookup tables with).
                if SpellSearchUtil.ActionBarStatusTooltips[enum.MissingFromAllBars] == nil
                   or SpellSearchUtil.ActionBarStatusMatchTypes[enum.OnDisabledActionBar] == nil then
                    return "spellsearch_lookup_table_missing_entries"
                end
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result, "ok",
            "ActionButtonUtil fixture should drive every ActionBarActionStatus through SpellSearchUtil: {result}"
        );
    }
}

/// Verifies the `UIParentLoadAddOn` harness seam: the default broadcaster
/// skip path mirrors Blizzard's failure bookkeeping (FailedAddOnLoad
/// table + a harness-visible failure log) instead of silently swallowing
/// the call, and unrelated `UIParentLoadAddOn` calls still flow through
/// to the real implementation untouched.
#[test]
fn uiparent_load_addon_seam_records_broadcaster_failure() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            -- Calling CooldownBroadcaster_LoadUI must hit the seam, return
            -- false, and book the failure into BOTH FailedAddOnLoad and the
            -- harness-visible log. The real Blizzard impl marks the addon in
            -- its file-local FailedAddOnLoad to dedupe the dialog message;
            -- the seam mirrors that on a global of the same name so the
            -- test can observe it.
            if type(CooldownBroadcaster_LoadUI) ~= "function" then
                return "missing_cooldown_broadcaster_loadui"
            end
            CooldownBroadcaster_LoadUI()

            if not FailedAddOnLoad or not FailedAddOnLoad["Blizzard_CooldownBroadcaster"] then
                return "FailedAddOnLoad_not_marked"
            end
            if type(__test_uiparent_load_addon_failures) ~= "table"
               or #__test_uiparent_load_addon_failures < 1 then
                return "harness_log_empty"
            end
            local last = __test_uiparent_load_addon_failures[#__test_uiparent_load_addon_failures]
            if last.name ~= "Blizzard_CooldownBroadcaster" or last.reason ~= "DISABLED_FOR_TESTS" then
                return "harness_log_wrong_entry:" .. tostring(last.name) .. "/" .. tostring(last.reason)
            end

            -- Re-firing must NOT push another entry (matches Blizzard's
            -- "show the dialog once per name" behaviour).
            local before = #__test_uiparent_load_addon_failures
            CooldownBroadcaster_LoadUI()
            if #__test_uiparent_load_addon_failures ~= before then
                return "harness_log_duplicated_entry"
            end

            -- Unrelated names must still flow through to the real
            -- UIParentLoadAddOn (here we just verify the wrapper doesn't
            -- short-circuit them — the real call's loaded/failed result
            -- depends on what the sim provides for that addon).
            local already_loaded_addon = "Blizzard_SharedXMLBase"
            local loaded = UIParentLoadAddOn(already_loaded_addon)
            if loaded == nil then
                return "wrapper_swallowed_unrelated_addon"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result, "ok",
            "UIParentLoadAddOn seam should preserve failure bookkeeping and pass-through: {result}"
        );
    }
}

/// Dedicated coverage path for `CooldownBroadcaster_LoadUI` against the
/// real `Blizzard_CooldownBroadcaster` addon load. Off by default
/// (`#[ignore]`) because the broadcaster's `ADDON_LOADED` /
/// `PLAYER_ENTERING_WORLD` handlers reach for `C_ChatInfo` / `C_Spell`
/// surface the panel tests don't bring up — this test exists so the seam
/// has a documented opt-in: flip `__test_skip_cooldown_broadcaster_load`
/// to `false` BEFORE the wrapper is installed (or before the broadcaster
/// loader is invoked) and run the loader against the real path. Run with
/// `cargo test --test test_showuipanel_lod -- --ignored
/// cooldown_broadcaster_loadui_can_be_exercised_when_opt_in`.
#[test]
#[ignore = "opt-in coverage for real CooldownBroadcaster load — needs C_ChatInfo/C_Spell surface"]
fn cooldown_broadcaster_loadui_can_be_exercised_when_opt_in() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            -- Disable the broadcaster skip and clear any prior failure
            -- bookkeeping so we observe a fresh run.
            __test_skip_cooldown_broadcaster_load = false
            if FailedAddOnLoad then
                FailedAddOnLoad["Blizzard_CooldownBroadcaster"] = nil
            end

            if type(CooldownBroadcaster_LoadUI) ~= "function" then
                return "missing_cooldown_broadcaster_loadui"
            end

            CooldownBroadcaster_LoadUI()

            -- Either the addon loaded (CooldownBroadcasterFrame is set) OR
            -- the load failed for a real reason (recorded in
            -- FailedAddOnLoad). What we MUST NOT see is the harness
            -- DISABLED_FOR_TESTS marker — that would mean the opt-in seam
            -- failed to deactivate.
            for _, entry in ipairs(__test_uiparent_load_addon_failures or {}) do
                if entry.name == "Blizzard_CooldownBroadcaster"
                   and entry.reason == "DISABLED_FOR_TESTS" then
                    return "seam_failed_to_deactivate"
                end
            end

            if CooldownBroadcasterFrame then
                return "loaded"
            end
            return "real_load_failed_but_seam_deactivated"
        "#).unwrap();
        // Either outcome is acceptable for the seam itself — the assertion
        // proves the opt-in path is live, not that CB fully starts up.
        assert!(
            result == "loaded" || result == "real_load_failed_but_seam_deactivated",
            "broadcaster opt-in seam should run the real load path (got {result})"
        );
    }
}
