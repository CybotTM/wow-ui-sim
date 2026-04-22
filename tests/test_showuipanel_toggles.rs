//! Integration tests for ShowUIPanel toggle and interaction behaviors.
//!
//! Tests panel displacement, coexistence, and toggling via Blizzard global functions.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Extra addons needed for spellbook tests (loaded on demand in real WoW,
/// but we load them explicitly here for deterministic testing).
const SPELLBOOK_ADDONS: &[(&str, &str)] = &[("Blizzard_PlayerSpells", "Blizzard_PlayerSpells.toc")];

/// Blizzard addons needed for the panel system (dependency order).
const PANEL_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    (
        "Blizzard_SharedXMLGame",
        "Blizzard_SharedXMLGame_Mainline.toc",
    ),
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
    (
        "Blizzard_FrameXMLUtil",
        "Blizzard_FrameXMLUtil_Mainline.toc",
    ),
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

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

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

    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
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

#[test]
fn show_ui_panel_displaces_previous_occupant() {
    test_timeout! {
        let env = setup_env();
        // UIParentPanelManager manages left/center/right slots. When two panels both
        // have pushable=0 and a left slot is occupied, the new panel replaces the old.
        // CharacterFrame (pushable=3) gets pushed to center instead of replaced.
        //
        // Test 1: Pushable panel gets pushed to center (not closed)
        // CharacterFrame (pushable=3) in left, then FriendsFrame (pushable=0) opens
        // → CharacterFrame pushed to center, both visible
        let result: String = env.eval(r#"
            if not CharacterFrame or not FriendsFrame then
                return "missing_frames"
            end
            ShowUIPanel(CharacterFrame)
            if not CharacterFrame:IsShown() then return "char_not_shown" end
            ShowUIPanel(FriendsFrame)
            if not FriendsFrame:IsShown() then return "friends_not_shown" end
            -- CharacterFrame should be pushed to center, still visible
            if not CharacterFrame:IsShown() then return "char_should_be_pushed_not_closed" end
            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "Pushable panel should be pushed to center, not closed: {result}");

        // Test 2: Non-pushable panel replaces another non-pushable panel
        // When both panels have pushable=0, the old one is replaced (hidden).
        let result: String = env.eval(r#"
            -- Close everything first
            CloseAllWindows()
            -- Register two test panels with pushable=0
            local a = CreateFrame("Frame", "TestPanelA", UIParent)
            a:SetSize(300, 400)
            a:Hide()
            UIPanelWindows["TestPanelA"] = { area = "left", pushable = 0, whileDead = 1 }
            local b = CreateFrame("Frame", "TestPanelB", UIParent)
            b:SetSize(300, 400)
            b:Hide()
            UIPanelWindows["TestPanelB"] = { area = "left", pushable = 0, whileDead = 1 }

            ShowUIPanel(a)
            if not a:IsShown() then return "a_not_shown" end
            ShowUIPanel(b)
            if not b:IsShown() then return "b_not_shown" end
            if a:IsShown() then return "a_not_replaced" end
            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "Non-pushable panel should replace previous non-pushable occupant: {result}");
    }
}

#[test]
fn character_and_spellbook_coexist() {
    test_timeout! {
        let env = setup_env();

        // Load PlayerSpells addon (normally LoD, loaded on demand)
        let ui = blizzard_ui_dir();
        for (name, toc) in SPELLBOOK_ADDONS {
            let toc_path = ui.join(name).join(toc);
            if toc_path.exists() {
                if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
                    eprintln!("[load {name}] FAILED: {e}");
                }
            }
        }

        // CharacterFrame: area="left", pushable=3
        // PlayerSpellsFrame: area="centerOrLeft", pushable=3, allowOtherPanels=1
        // Both are pushable and allow other panels, so they coexist:
        // Character stays in left slot, Spellbook goes to center.
        let result: String = env.eval(r#"
            if not CharacterFrame then return "no_char_frame" end
            if not PlayerSpellsFrame then return "no_spellbook_frame" end

            ShowUIPanel(CharacterFrame)
            if not CharacterFrame:IsShown() then return "char_not_shown" end

            ShowUIPanel(PlayerSpellsFrame)
            if not PlayerSpellsFrame:IsShown() then return "spellbook_not_shown" end

            -- Both should be visible: CharacterFrame in left, PlayerSpellsFrame in center
            if not CharacterFrame:IsShown() then return "char_closed_unexpectedly" end
            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "Character and Spellbook panels should coexist (left + center): {result}");
    }
}

#[test]
fn toggle_spellbook_legacy_global_opens_and_closes_spellbook_panel() {
    test_timeout! {
        let env = setup_env();

        let ui = blizzard_ui_dir();
        for (name, toc) in SPELLBOOK_ADDONS {
            let toc_path = ui.join(name).join(toc);
            if toc_path.exists() {
                if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
                    eprintln!("[load {name}] FAILED: {e}");
                }
            }
        }

        let result: String = env.eval(r#"
            if not ToggleSpellBook then
                return "missing_toggle_spellbook"
            end

            ToggleSpellBook(BOOKTYPE_SPELL)
            if not PlayerSpellsFrame or not PlayerSpellsFrame:IsShown() then
                return "spellbook_not_shown"
            end
            if not PlayerSpellsFrame.SpellBookFrame or not PlayerSpellsFrame.SpellBookFrame:IsShown() then
                return "spellbook_tab_not_shown"
            end

            ToggleSpellBook(BOOKTYPE_SPELL)
            if PlayerSpellsFrame:IsShown() then
                return "spellbook_not_hidden"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "ToggleSpellBook(BOOKTYPE_SPELL) should toggle the spellbook panel: {result}"
        );
    }
}

#[test]
fn toggle_player_spells_frame_opens_and_closes_talent_panel() {
    test_timeout! {
        let env = setup_env();

        let ui = blizzard_ui_dir();
        for (name, toc) in SPELLBOOK_ADDONS {
            let toc_path = ui.join(name).join(toc);
            if toc_path.exists() {
                if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
                    eprintln!("[load {name}] FAILED: {e}");
                }
            }
        }

        let result: String = env.eval(r#"
            if not PlayerSpellsUtil or not PlayerSpellsUtil.TogglePlayerSpellsFrame then
                return "missing_toggle_player_spells_frame"
            end

            PlayerSpellsUtil.TogglePlayerSpellsFrame()
            if not PlayerSpellsFrame or not PlayerSpellsFrame:IsShown() then
                return "player_spells_not_shown"
            end

            PlayerSpellsUtil.TogglePlayerSpellsFrame()
            if PlayerSpellsFrame:IsShown() then
                return "player_spells_not_hidden"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "PlayerSpellsUtil.TogglePlayerSpellsFrame() should toggle the talent panel: {result}"
        );
    }
}

#[test]
fn toggle_collections_journal_opens_mounts_pets_and_toys_tabs_and_accepts_search_text() {
    test_timeout! {
        let env = setup_env();

        let result: String = env.eval(r#"
            if not ToggleCollectionsJournal then
                return "missing_toggle_collections_journal"
            end

            local cases = {
                { COLLECTIONS_JOURNAL_TAB_INDEX_MOUNTS, "MountJournal", "searchBox", "gryphon" },
                { COLLECTIONS_JOURNAL_TAB_INDEX_PETS, "PetJournal", "searchBox", "cat" },
                { COLLECTIONS_JOURNAL_TAB_INDEX_TOYS, "ToyBox", "searchBox", "ball" },
            }

            for _, case in ipairs(cases) do
                local tabIndex, childName, searchKey, searchText = case[1], case[2], case[3], case[4]
                ToggleCollectionsJournal(tabIndex)

                if not CollectionsJournal or not CollectionsJournal:IsShown() then
                    return "journal_not_shown_" .. tostring(tabIndex)
                end
                if CollectionsJournal_GetTab(CollectionsJournal) ~= tabIndex then
                    return "wrong_tab_" .. tostring(tabIndex) .. "_" .. tostring(CollectionsJournal_GetTab(CollectionsJournal))
                end

                local child = _G[childName]
                if not child or not child:IsShown() then
                    return "child_not_shown_" .. childName
                end
                local searchBox = child[searchKey]
                if not searchBox then
                    return "search_box_missing_" .. childName
                end
                searchBox:SetText(searchText)
                if searchBox:GetText() ~= searchText then
                    return "search_text_not_set_" .. childName
                end

                ToggleCollectionsJournal(tabIndex)
                if CollectionsJournal:IsShown() then
                    return "journal_not_hidden_" .. tostring(tabIndex)
                end
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "ToggleCollectionsJournal(tab) should open, switch tabs, accept search text, and close for mounts, pets, and toys: {result}"
        );
    }
}

#[test]
fn collections_mount_list_count_matches_displayed_mount_count() {
    test_timeout! {
        let env = setup_env();

        let result: String = env.eval(r#"
            if not ToggleCollectionsJournal then
                return "missing_toggle_collections_journal"
            end

            ToggleCollectionsJournal(COLLECTIONS_JOURNAL_TAB_INDEX_MOUNTS)

            if not (CollectionsJournal and CollectionsJournal:IsShown()) then
                return "journal_not_shown"
            end
            if not (MountJournal and MountJournal:IsShown()) then
                return "mount_journal_not_shown"
            end
            if not MountJournal.ScrollBox then
                return "missing_mount_scroll_box"
            end

            local dataProvider = MountJournal.ScrollBox:GetDataProvider()
            if not dataProvider then
                return "missing_mount_data_provider"
            end

            local expected = C_MountJournal.GetNumDisplayedMounts()
            local actual = dataProvider:GetSize()
            if actual ~= expected then
                return string.format(
                    "mount_list_count_mismatch_expected_%s_actual_%s",
                    tostring(expected),
                    tostring(actual)
                )
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "MountJournal scrollbox data-provider size should match C_MountJournal.GetNumDisplayedMounts(): {result}"
        );
    }
}

#[test]
fn toggle_achievement_frame_opens_and_closes_achievement_panel() {
    test_timeout! {
        let env = setup_env();

        let result: String = env.eval(r#"
            if not ToggleAchievementFrame then
                return "missing_toggle_achievement_frame"
            end

            ToggleAchievementFrame()
            if not AchievementFrame or not AchievementFrame:IsShown() then
                return "achievement_not_shown"
            end

            ToggleAchievementFrame()
            if AchievementFrame:IsShown() then
                return "achievement_not_hidden"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "ToggleAchievementFrame() should open and close the achievement panel: {result}"
        );
    }
}

#[test]
fn achievement_frame_layout_stays_locked() {
    test_timeout! {
        let env = setup_env();

        let result: String = env
            .eval(
                r#"
                local EPS = 0.75

                local function approx(actual, expected, eps)
                    if type(actual) ~= "number" or type(expected) ~= "number" then
                        return false
                    end
                    return math.abs(actual - expected) <= (eps or EPS)
                end

                local function rect(frame, name)
                    if type(frame) ~= "table" then
                        return nil, name .. "_missing"
                    end
                    local l, b, w, h = frame:GetRect()
                    if not (l and b and w and h) then
                        return nil, name .. "_missing_rect"
                    end
                    return { l = l, b = b, w = w, h = h, r = l + w, t = b + h }, nil
                end

                local function has_point(frame, point, rel, rel_point, x, y, eps)
                    for i = 1, frame:GetNumPoints() do
                        local p, r, rp, ox, oy = frame:GetPoint(i)
                        local rel_matches = (r == rel) or (r == nil and rel ~= nil and frame.GetParent and frame:GetParent() == rel)
                        if p == point and rel_matches and rp == rel_point and approx(ox or 0, x, eps) and approx(oy or 0, y, eps) then
                            return true
                        end
                    end
                    return false
                end

                local function expect_rect(frame, name, left, bottom, width, height)
                    local r, e = rect(frame, name)
                    if not r then
                        return nil, e
                    end
                    if not approx(r.l, left) then
                        return nil, name .. "_left=" .. tostring(r.l)
                    end
                    if not approx(r.b, bottom) then
                        return nil, name .. "_bottom=" .. tostring(r.b)
                    end
                    if not approx(r.w, width, 0.1) then
                        return nil, name .. "_width=" .. tostring(r.w)
                    end
                    if not approx(r.h, height, 0.1) then
                        return nil, name .. "_height=" .. tostring(r.h)
                    end
                    return r, nil
                end

                if not ToggleAchievementFrame then
                    return "missing_toggle_achievement_frame"
                end

                ToggleAchievementFrame()
                if not AchievementFrame or not AchievementFrame:IsShown() then
                    return "achievement_not_shown"
                end

                local frame_rect, frame_err = expect_rect(AchievementFrame, "achievement_frame", 0, 164, 768, 500)
                if not frame_rect then return frame_err end
                if not has_point(AchievementFrame, "TOPLEFT", UIParent, "TOPLEFT", 0, -104, 0.1) then
                    return "achievement_frame_anchor_mismatch"
                end

                local close_rect, close_err = expect_rect(AchievementFrameCloseButton, "achievement_close_button", 744, 640, 24, 24)
                if not close_rect then return close_err end
                if not has_point(AchievementFrameCloseButton, "TOPRIGHT", AchievementFrame, "TOPRIGHT", 0, 0, 0.1) then
                    return "achievement_close_button_anchor_mismatch"
                end

                if not AchievementFrameCategories or not AchievementFrameCategories:IsShown() then
                    return "achievement_categories_missing_or_hidden"
                end
                local categories_rect, categories_err = expect_rect(
                    AchievementFrameCategories,
                    "achievement_categories",
                    21,
                    184,
                    175,
                    461
                )
                if not categories_rect then return categories_err end
                if not has_point(AchievementFrameCategories, "TOPLEFT", AchievementFrame, "TOPLEFT", 21, -19, 0.1) then
                    return "achievement_categories_top_anchor_mismatch"
                end
                if not has_point(AchievementFrameCategories, "BOTTOMLEFT", AchievementFrame, "BOTTOMLEFT", 21, 20, 0.1) then
                    return "achievement_categories_bottom_anchor_mismatch"
                end

                local categories_bg_rect, categories_bg_err = expect_rect(
                    AchievementFrameCategoriesBG,
                    "achievement_categories_bg",
                    25,
                    187,
                    195,
                    454
                )
                if not categories_bg_rect then return categories_bg_err end
                if not has_point(AchievementFrameCategoriesBG, "TOPLEFT", AchievementFrame, "TOPLEFT", 25, -23, 0.1) then
                    return "achievement_categories_bg_top_anchor_mismatch"
                end
                if not has_point(AchievementFrameCategoriesBG, "BOTTOMLEFT", AchievementFrame, "BOTTOMLEFT", 25, 23, 0.1) then
                    return "achievement_categories_bg_bottom_anchor_mismatch"
                end

                local background = AchievementFrame.Background
                local bg_rect, bg_err = expect_rect(background, "achievement_background", 16, 180, 736, 468)
                if not bg_rect then return bg_err end
                if not has_point(background, "TOPLEFT", AchievementFrame, "TOPLEFT", 16, -16, 0.1) then
                    return "achievement_background_top_anchor_mismatch"
                end
                if not has_point(background, "BOTTOMRIGHT", AchievementFrame, "BOTTOMRIGHT", -16, 16, 0.1) then
                    return "achievement_background_bottom_anchor_mismatch"
                end

                local header_rect, header_err = expect_rect(AchievementFrame.Header, "achievement_header", 26, 626, 726, 106)
                if not header_rect then return header_err end
                if not has_point(AchievementFrame.Header, "BOTTOMLEFT", AchievementFrame, "TOPLEFT", 26, -38, 0.1) then
                    return "achievement_header_anchor_mismatch"
                end

                local scrollbox_rect, scrollbox_err = expect_rect(
                    AchievementFrameCategories.ScrollBox,
                    "achievement_categories_scrollbox",
                    21,
                    189,
                    175,
                    451
                )
                if not scrollbox_rect then return scrollbox_err end
                if not AchievementFrameCategories.ScrollBox:IsShown() then
                    return "achievement_categories_scrollbox_hidden"
                end

                local summary_rect, summary_err = expect_rect(AchievementFrameSummary, "achievement_summary", 218, 184, 530, 461)
                if not summary_rect then return summary_err end
                if not AchievementFrameSummary:IsShown() then
                    return "achievement_summary_hidden"
                end
                if AchievementFrameAchievements and AchievementFrameAchievements:IsShown() then
                    return "achievement_list_should_start_hidden"
                end
                if AchievementFrameStats and AchievementFrameStats:IsShown() then
                    return "achievement_stats_should_start_hidden"
                end
                if AchievementFrameComparison and AchievementFrameComparison:IsShown() then
                    return "achievement_comparison_should_start_hidden"
                end

                local status_rect, status_err = expect_rect(
                    AchievementFrameSummaryCategoriesStatusBar,
                    "achievement_summary_categories_status_bar",
                    239,
                    384,
                    488,
                    21
                )
                if not status_rect then return status_err end
                if not AchievementFrameSummaryCategoriesStatusBar:IsShown() then
                    return "achievement_summary_categories_status_bar_hidden"
                end

                if PanelTemplates_GetSelectedTab and PanelTemplates_GetSelectedTab(AchievementFrame) ~= 1 then
                    return "achievement_selected_tab=" .. tostring(PanelTemplates_GetSelectedTab(AchievementFrame))
                end

                return "ok"
            "#,
            )
            .unwrap();

        assert_eq!(
            result, "ok",
            "AchievementFrame layout should remain locked after ToggleAchievementFrame(): {result}"
        );
    }
}

#[test]
fn toggle_encounter_journal_opens_and_closes_panel() {
    test_timeout! {
        let env = setup_env();

        let result: String = env.eval(r#"
            if not ToggleEncounterJournal then
                return "missing_toggle_encounter_journal"
            end

            ToggleEncounterJournal()
            if not EncounterJournal or not EncounterJournal:IsShown() then
                return "encounter_journal_not_shown"
            end

            ToggleEncounterJournal()
            if EncounterJournal:IsShown() then
                return "encounter_journal_not_hidden"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "ToggleEncounterJournal() should open and close the encounter journal panel: {result}"
        );
    }
}

#[test]
fn open_trade_skill_opens_blacksmithing_panel() {
    test_timeout! {
        let env = setup_env();

        let result: String = env.eval(r#"
            if not C_TradeSkillUI or not C_TradeSkillUI.OpenTradeSkill then
                return "missing_open_trade_skill"
            end

            local opened = C_TradeSkillUI.OpenTradeSkill(164)
            if opened ~= true then
                return "opened=" .. tostring(opened)
            end

            if not ProfessionsFrame or not ProfessionsFrame:IsShown() then
                return "professions_frame_not_shown"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "C_TradeSkillUI.OpenTradeSkill(164) should open the professions frame: {result}"
        );
    }
}

#[test]
fn toggle_guild_frame_opens_and_closes_communities_panel() {
    test_timeout! {
        let env = setup_env();

        let result: String = env.eval(r#"
            if not ToggleGuildFrame then
                return "missing_toggle_guild_frame"
            end

            local opened = ToggleGuildFrame()
            if CommunitiesFrame and not CommunitiesFrame:IsShown() then
                return "communities_frame_not_shown"
            end

            ToggleGuildFrame()
            if CommunitiesFrame and CommunitiesFrame:IsShown() then
                return "communities_frame_not_hidden"
            end

            return tostring(opened == nil or opened == true) == "true" and "ok" or ("opened=" .. tostring(opened))
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "ToggleGuildFrame() should open and close the guild/communities panel: {result}"
        );
    }
}

#[test]
fn toggle_lfd_parent_frame_opens_and_closes_group_finder_panel() {
    test_timeout! {
        let env = setup_env();

        let result: String = env.eval(r#"
            if not ToggleLFDParentFrame then
                return "missing_toggle_lfd_parent_frame"
            end

            ToggleLFDParentFrame()
            if not PVEFrame or not PVEFrame:IsShown() then
                return "pve_frame_not_shown"
            end

            ToggleLFDParentFrame()
            if PVEFrame and PVEFrame:IsShown() then
                return "pve_frame_not_hidden"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "ToggleLFDParentFrame() should open and close the group finder panel: {result}"
        );
    }
}

#[test]
fn toggle_character_reputation_frame_selects_and_toggles_reputation_panel() {
    test_timeout! {
        let env = setup_env();

        let result: String = env.eval(r#"
            if not ToggleCharacter then
                return "missing_toggle_character"
            end
            if not ReputationFrame then
                return "missing_reputation_frame"
            end

            ToggleCharacter("ReputationFrame")
            if not CharacterFrame or not CharacterFrame:IsShown() then
                return "character_frame_not_shown"
            end
            if not ReputationFrame:IsShown() then
                return "reputation_frame_not_shown"
            end
            if PaperDollFrame and PaperDollFrame:IsShown() then
                return "paperdoll_should_be_hidden"
            end

            ToggleCharacter("ReputationFrame")
            if CharacterFrame and CharacterFrame:IsShown() then
                return "character_frame_not_hidden"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "ToggleCharacter(\"ReputationFrame\") should select and toggle the reputation panel: {result}"
        );
    }
}

#[test]
fn toggle_mail_frame_opens_and_closes_mail_panel() {
    test_timeout! {
        let env = setup_env();

        let result: String = env.eval(r#"
            A_Admin.ClearInbox()
            A_Admin.AddMail("Thrall", "Unread Orders", "Meet me in Orgrimmar.")

            if not ToggleMailFrame then
                return "missing_toggle_mail_frame"
            end

            ToggleMailFrame()
            if not MailFrame or not MailFrame:IsShown() then
                return "mail_frame_not_shown"
            end
            if not InboxFrame or not InboxFrame:IsShown() then
                return "inbox_frame_not_shown"
            end

            ToggleMailFrame()
            if MailFrame and MailFrame:IsShown() then
                return "mail_frame_not_hidden"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "ToggleMailFrame() should open and close the mail panel: {result}"
        );
    }
}

#[test]
fn open_all_bags_opens_a_bag_frame() {
    test_timeout! {
        let env = setup_env();

        let result: String = env.eval(r#"
            if not OpenAllBags then
                return "missing_open_all_bags"
            end

            OpenAllBags()

            if ContainerFrameCombinedBags and ContainerFrameCombinedBags:IsShown() then
                return "ok"
            end

            for i = 1, 6 do
                local frame = _G["ContainerFrame" .. i]
                if frame and frame:IsShown() then
                    return "ok"
                end
            end

            return "no_bag_frame_shown"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "OpenAllBags() should show a combined or individual bag frame: {result}"
        );
    }
}

#[test]
fn housing_dashboard_loads_and_opens_panel() {
    test_timeout! {
        let env = setup_env();
        let result: String = env
            .eval(
                r#"
                    local loaded, reason = C_AddOns.LoadAddOn("Blizzard_HousingDashboard")
                    if not loaded then
                        return "load_failed:" .. tostring(reason)
                    end
                    if not HousingDashboardFrame then
                        return "missing_frame"
                    end

                    local panelEntry = UIPanelWindows["HousingDashboardFrame"]
                    if not panelEntry then
                        return "missing_panel_registration"
                    end

                    local ok, err = pcall(function()
                        ShowUIPanel(HousingDashboardFrame)
                    end)
                    if not ok then
                        return "show_failed:" .. tostring(err)
                    end
                    if not HousingDashboardFrame:IsShown() then
                        return "panel_not_shown"
                    end
                    if HousingDashboardFrame.HouseInfoContent.LoadingSpinner:IsShown() then
                        return "spinner_still_shown"
                    end
                    if not HousingDashboardFrame.HouseInfoContent.DashboardNoHousesFrame:IsShown() then
                        return "no_houses_dashboard_not_shown"
                    end

                    return "ok"
                "#,
            )
            .unwrap();
        assert_eq!(
            result, "ok",
            "Housing dashboard should load and open via ShowUIPanel: {result}"
        );
    }
}
