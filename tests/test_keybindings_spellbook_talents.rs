//! Integration tests for spellbook and talent panel keybinding details.

use crate::common;
#[path = "common/keybindings_panels_detail.rs"]
mod keybindings_panels_detail;
#[path = "common/token_ui_fixtures.rs"]
mod token_ui_fixtures;

use keybindings_panels_detail::{
    drain_test_errors, frame_is_shown, install_test_error_handler, setup_env,
};

// ── S → Spellbook panel opens without errors ─────────────────────────────

#[test]
fn keybind_s_opens_spellbook_no_errors() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("S", None).expect("S keybind dispatch failed");

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Opening spellbook produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert!(
            frame_is_shown(&env, "PlayerSpellsFrame"),
            "PlayerSpellsFrame should be shown after pressing S"
        );
    }
}

#[test]
fn keybind_s_opens_spellbook_tab_on_first_press() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("S", None).expect("S keybind dispatch failed");

        let result: String = env
            .eval(
                r#"
                if not (PlayerSpellsFrame and PlayerSpellsFrame:IsShown()) then
                    return "player_spells_not_shown"
                end
                if not (PlayerSpellsFrame.SpellBookFrame and PlayerSpellsFrame.SpellBookFrame:IsShown()) then
                    return "spellbook_tab_not_shown"
                end
                return "ok"
                "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Opening spellbook through S produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Pressing S should show the spellbook tab on the first open: {result}"
        );
    }
}

#[test]
fn keybind_s_is_a_single_thin_dispatch_to_toggle_spellbook_frame() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.exec(
            r#"
            if not (PlayerSpellsUtil and type(PlayerSpellsUtil.ToggleSpellBookFrame) == "function") then
                error("missing_toggle_spellbook")
            end

            original_toggle_spellbook_frame = PlayerSpellsUtil.ToggleSpellBookFrame
            spellbook_toggle_calls = 0

            PlayerSpellsUtil.ToggleSpellBookFrame = function(...)
                spellbook_toggle_calls = spellbook_toggle_calls + 1
                return false
            end
            "#,
        )
        .unwrap();

        env.send_key_press("S", None)
            .expect("S keybind dispatch failed");

        let result: (i32, bool, bool) = env
            .eval(
                r#"
                return
                    spellbook_toggle_calls or 0,
                    PlayerSpellsFrame and PlayerSpellsFrame:IsShown() == true or false,
                    PlayerSpellsFrame and PlayerSpellsFrame.SpellBookFrame and PlayerSpellsFrame.SpellBookFrame:IsShown() == true or false
                "#,
            )
            .unwrap();
        env.exec(
            r#"
            if original_toggle_spellbook_frame ~= nil then
                PlayerSpellsUtil.ToggleSpellBookFrame = original_toggle_spellbook_frame
            end
            "#,
        )
        .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Spellbook keybind fallback regression produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            (1, false, false),
            "Spellbook keybind should be a single dispatch into ToggleSpellBookFrame without force-show fallback"
        );
    }
}

#[test]
fn keybind_s_dispatches_directly_to_playerspellsutil_toggle_spellbook_frame() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.exec(
            r#"
            if not (PlayerSpellsUtil and type(PlayerSpellsUtil.ToggleSpellBookFrame) == "function") then
                error("missing_toggle_spellbook")
            end
            if type(ToggleSpellBook) ~= "function" then
                error("missing_legacy_toggle_spellbook")
            end

            original_toggle_spellbook_frame = PlayerSpellsUtil.ToggleSpellBookFrame
            original_toggle_spellbook = ToggleSpellBook
            spellbook_toggle_frame_calls = 0
            legacy_toggle_spellbook_calls = 0

            PlayerSpellsUtil.ToggleSpellBookFrame = function(...)
                spellbook_toggle_frame_calls = spellbook_toggle_frame_calls + 1
                return false
            end

            ToggleSpellBook = function(...)
                legacy_toggle_spellbook_calls = legacy_toggle_spellbook_calls + 1
                return false
            end
            "#,
        )
        .unwrap();

        env.send_key_press("S", None)
            .expect("S keybind dispatch failed");

        let result: (i32, i32) = env
            .eval(
                r#"
                return
                    spellbook_toggle_frame_calls or 0,
                    legacy_toggle_spellbook_calls or 0
                "#,
            )
            .unwrap();
        env.exec(
            r#"
            if original_toggle_spellbook_frame ~= nil then
                PlayerSpellsUtil.ToggleSpellBookFrame = original_toggle_spellbook_frame
            end
            if original_toggle_spellbook ~= nil then
                ToggleSpellBook = original_toggle_spellbook
            end
            "#,
        )
        .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Direct spellbook keybind dispatch produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            (1, 0),
            "Spellbook keybind should dispatch directly into PlayerSpellsUtil.ToggleSpellBookFrame, not legacy ToggleSpellBook"
        );
    }
}

#[test]
fn keybind_s_toggles_spellbook_closed_on_second_press() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("S", None).expect("first S keybind dispatch failed");
        assert!(
            frame_is_shown(&env, "PlayerSpellsFrame"),
            "PlayerSpellsFrame should be shown after the first S press"
        );

        env.send_key_press("S", None).expect("second S keybind dispatch failed");

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Toggling spellbook through S produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert!(
            !frame_is_shown(&env, "PlayerSpellsFrame"),
            "PlayerSpellsFrame should be hidden after pressing S twice"
        );
    }
}

#[test]
fn spellbook_panel_spell_tooltip_has_lines_after_tab_switch_and_closes_without_errors() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        let result: String = env
            .eval(
                r#"
                if not (PlayerSpellsUtil and type(PlayerSpellsUtil.ToggleSpellBookFrame) == "function") then
                    return "missing_toggle_spellbook"
                end

                if not (PlayerSpellsUtil.FrameTabs and PlayerSpellsUtil.FrameTabs.ClassTalents and PlayerSpellsUtil.FrameTabs.SpellBook) then
                    return "missing_frame_tabs"
                end

                PlayerSpellsUtil.ToggleSpellBookFrame()

                if not (PlayerSpellsFrame and PlayerSpellsFrame:IsShown()) then
                    return "spellbook_not_open"
                end

                if not PlayerSpellsFrame:TrySetTab(PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "talent_tab_unavailable"
                end

                if not PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "talent_tab_not_selected"
                end

                if not PlayerSpellsFrame:TrySetTab(PlayerSpellsUtil.FrameTabs.SpellBook) then
                    return "spellbook_tab_unavailable"
                end

                if not PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.SpellBook) then
                    return "spellbook_tab_not_selected"
                end

                local hasSpell = GameTooltip:SetSpellBookItem(1)
                if not hasSpell then
                    return "no_spellbook_item"
                end

                local info = GameTooltip:GetPrimaryTooltipInfo()
                local tooltipData = GameTooltip:GetPrimaryTooltipData()
                if not info
                    or not tooltipData
                    or not tooltipData.lines
                    or not tooltipData.lines[1]
                then
                    return "tooltip_has_no_lines"
                end

                PlayerSpellsUtil.ToggleSpellBookFrame()

                if PlayerSpellsFrame:IsShown() then
                    return "spellbook_not_closed"
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Spellbook tooltip flow produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Spellbook panel tooltip flow should open, switch tabs, populate tooltip lines, and close: {result}"
        );
    }
}

#[test]
fn spellbook_first_visible_item_icon_matches_spellbook_texture() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("S", None).expect("S keybind dispatch failed");

        let result: String = env
            .eval(
                r#"
                local paged = PlayerSpellsFrame and PlayerSpellsFrame.SpellBookFrame and PlayerSpellsFrame.SpellBookFrame.PagedSpellsFrame
                if not paged then
                    return "missing_paged_spells_frame"
                end

                for _, frame in paged:EnumerateFrames() do
                    if frame
                        and frame:IsShown()
                        and frame.HasValidData
                        and frame:HasValidData()
                        and frame.slotIndex
                        and frame.spellBank
                        and frame.Button
                        and frame.Button.Icon
                    then
                        local expected = C_SpellBook.GetSpellBookItemTexture(frame.slotIndex, frame.spellBank)
                        local actual = frame.Button.Icon:GetTexture()
                        if actual ~= expected then
                            return string.format(
                                "icon_mismatch_slot_%s_expected_%s_actual_%s",
                                tostring(frame.slotIndex),
                                tostring(expected),
                                tostring(actual)
                            )
                        end
                        return "ok"
                    end
                end

                return "no_visible_spellbook_item"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Spellbook icon regression produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "The first visible spellbook item icon should match C_SpellBook.GetSpellBookItemTexture for its slot: {result}"
        );
    }
}

#[test]
fn spellbook_paging_label_is_formatted_on_first_open() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("S", None).expect("S keybind dispatch failed");

        let result: String = env
            .eval(
                r#"
                local pagingControls = PlayerSpellsFrame
                    and PlayerSpellsFrame.SpellBookFrame
                    and PlayerSpellsFrame.SpellBookFrame.PagedSpellsFrame
                    and PlayerSpellsFrame.SpellBookFrame.PagedSpellsFrame.PagingControls
                if not pagingControls then
                    return "missing_paging_controls"
                end

                local text = pagingControls.PageText and pagingControls.PageText:GetText()
                if not text then
                    return "missing_page_text"
                end
                if text:find("%%d") then
                    return "unformatted_page_text_" .. text
                end
                if not text:match("^Page %d+/%d+$") then
                    return "unexpected_page_text_" .. text
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Spellbook paging label regression produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Spellbook paging controls should render a formatted page label, not a literal format string: {result}"
        );
    }
}

#[test]
fn talent_panel_switches_spec_tabs_and_closes_without_errors() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        let result: String = env
            .eval(
                r#"
                if not (PlayerSpellsUtil and type(PlayerSpellsUtil.ToggleClassTalentFrame) == "function") then
                    return "missing_toggle_class_talent_frame"
                end

                if not (PlayerSpellsUtil.FrameTabs and PlayerSpellsUtil.FrameTabs.ClassSpecializations and PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "missing_frame_tabs"
                end

                PlayerSpellsUtil.ToggleClassTalentFrame()

                if not (PlayerSpellsFrame and PlayerSpellsFrame:IsShown()) then
                    return "talent_panel_not_open"
                end

                if not PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "talent_tab_not_initial"
                end

                if not PlayerSpellsFrame:TrySetTab(PlayerSpellsUtil.FrameTabs.ClassSpecializations) then
                    return "spec_tab_unavailable"
                end

                if not PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.ClassSpecializations) then
                    return "spec_tab_not_selected"
                end

                if not PlayerSpellsFrame:TrySetTab(PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "talent_tab_unavailable"
                end

                if not PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "talent_tab_not_reselected"
                end

                PlayerSpellsUtil.ToggleClassTalentFrame()

                if PlayerSpellsFrame:IsShown() then
                    return "talent_panel_not_closed"
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Talent panel tab-switch flow produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Talent panel flow should open, switch to spec tab, switch back, and close: {result}"
        );
    }
}

#[test]
fn talent_panel_has_at_least_one_visible_talent_node_frame() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        let result: String = env
            .eval(
                r#"
                if not (PlayerSpellsUtil and type(PlayerSpellsUtil.ToggleClassTalentFrame) == "function") then
                    return "missing_toggle_class_talent_frame"
                end

                PlayerSpellsUtil.ToggleClassTalentFrame()

                if not (PlayerSpellsFrame and PlayerSpellsFrame:IsShown()) then
                    return "talent_panel_not_open"
                end
                if not (PlayerSpellsFrame.TalentsFrame and PlayerSpellsFrame.TalentsFrame:IsShown()) then
                    return "talents_frame_not_shown"
                end

                local totalButtons = 0
                local visibleButtons = 0
                for talentButton in PlayerSpellsFrame.TalentsFrame:EnumerateAllTalentButtons() do
                    totalButtons = totalButtons + 1
                    if talentButton and talentButton:IsShown() then
                        visibleButtons = visibleButtons + 1
                    end
                end

                if totalButtons == 0 then
                    return "no_talent_buttons"
                end
                if visibleButtons == 0 then
                    return "no_visible_talent_buttons"
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Talent panel flow produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Talent panel should expose at least one visible active talent button frame: {result}"
        );
    }
}
