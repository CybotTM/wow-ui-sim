//! Integration tests for the Blizzard_PlayerSpells panel: keybind-driven
//! load + the diagnostic snapshots used to debug template-binding gaps in
//! its OnLoad chain.
//!
//! Companion to `test_showuipanel_lod.rs` (other panel loaders) and
//! `test_showuipanel_lod_fixtures.rs` (harness/fixture coverage). All
//! three share `tests/common/panel_fixtures.rs`.

mod common;

use common::panel_fixtures::{
    clear_recorded_lua_errors, player_spells_panel_debug_snapshot, recorded_lua_errors, setup_env,
};
use wow_ui_sim::startup::prewarm_player_spells_spellbook;

#[test]
fn startup_prewarm_loads_blizzard_player_spells_and_keeps_it_hidden() {
    test_timeout! {
        let env = setup_env();
        common::install_error_collector(&env, "__spellbook_prewarm_errors");
        clear_recorded_lua_errors(&env);

        let unloaded_before: bool = env
            .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_PlayerSpells")"#)
            .expect("initial addon load probe should return");
        assert!(
            !unloaded_before,
            "Test harness should start with Blizzard_PlayerSpells unloaded"
        );

        let warmed = prewarm_player_spells_spellbook(&env);
        assert!(warmed, "SpellBook prewarm should run on the game screen");

        let loaded_after: bool = env
            .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_PlayerSpells")"#)
            .expect("addon load probe after prewarm should return");
        assert!(loaded_after, "SpellBook prewarm should demand-load Blizzard_PlayerSpells");

        let hidden_after: bool = env
            .eval(r#"return PlayerSpellsFrame ~= nil and not PlayerSpellsFrame:IsShown()"#)
            .expect("panel visibility probe after prewarm should return");
        assert!(
            hidden_after,
            "SpellBook prewarm should leave PlayerSpellsFrame hidden"
        );

        let recorded_errors = recorded_lua_errors(&env);
        let handler_errors = common::drain_string_table(&env, "__spellbook_prewarm_errors");
        assert!(
            recorded_errors.is_empty(),
            "SpellBook prewarm produced {} recorded Lua error(s):\n{:#?}\nhandler_errors:\n{}\n{}",
            recorded_errors.len(),
            recorded_errors,
            handler_errors.join("\n"),
            player_spells_panel_debug_snapshot(&env),
        );
        assert!(
            handler_errors.is_empty(),
            "SpellBook prewarm produced {} Lua error(s):\n{}",
            handler_errors.len(),
            handler_errors.join("\n")
        );
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
