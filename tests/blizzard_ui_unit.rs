//! Blizzard UI component unit tests.

mod common;
mod tooltip_full_env_helpers;

use tooltip_full_env_helpers::{refresh_aura_frames, setup_full_env};

fn open_character_panel(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        local btn = CharacterMicroButton
        assert(btn, "CharacterMicroButton should exist")
        local onclick = btn:GetScript("OnClick")
        assert(onclick, "CharacterMicroButton should have an OnClick handler")
        onclick(btn, "LeftButton", false)
        assert(CharacterFrame and CharacterFrame:IsShown(), "CharacterFrame should be shown")
        assert(CharacterHeadSlot ~= nil, "CharacterHeadSlot should exist")
        "#,
    )
    .expect("Failed to open character panel");
}

fn open_spellbook(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        assert(PlayerSpellsUtil and PlayerSpellsUtil.ToggleSpellBookFrame, "ToggleSpellBookFrame should exist")
        PlayerSpellsUtil.ToggleSpellBookFrame()
        assert(PlayerSpellsFrame and PlayerSpellsFrame:IsShown(), "PlayerSpellsFrame should be shown")
        assert(PlayerSpellsFrame.SpellBookFrame and PlayerSpellsFrame.SpellBookFrame:IsShown(), "SpellBookFrame should be shown")
        "#,
    )
    .expect("Failed to open spellbook");
}

fn refresh_buff_frame(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    refresh_aura_frames(env);
    wow_ui_sim::startup::seed_buff_durations(env);
}

#[test]
fn character_panel_equipment_slots_match_inventory_or_background_textures() {
    test_timeout! {
        let env = setup_full_env();
        open_character_panel(&env);

        let result: String = env.eval(
            r#"
            local slotNames = {
                "CharacterHeadSlot",
                "CharacterNeckSlot",
                "CharacterShoulderSlot",
                "CharacterBackSlot",
                "CharacterChestSlot",
                "CharacterShirtSlot",
                "CharacterTabardSlot",
                "CharacterWristSlot",
                "CharacterHandsSlot",
                "CharacterWaistSlot",
                "CharacterLegsSlot",
                "CharacterFeetSlot",
                "CharacterFinger0Slot",
                "CharacterFinger1Slot",
                "CharacterTrinket0Slot",
                "CharacterTrinket1Slot",
                "CharacterMainHandSlot",
                "CharacterSecondaryHandSlot",
            }

            for _, frameName in ipairs(slotNames) do
                local slot = _G[frameName]
                if not slot then
                    return "missing_slot_" .. frameName
                end
                if not slot.icon then
                    return "missing_icon_" .. frameName
                end

                local expectedTexture = GetInventoryItemTexture("player", slot:GetID())
                local actualTexture = slot.icon:GetTexture()

                if expectedTexture ~= nil then
                    if actualTexture ~= expectedTexture then
                        return string.format(
                            "equipped_texture_mismatch_%s_expected_%s_actual_%s",
                            frameName,
                            tostring(expectedTexture),
                            tostring(actualTexture)
                        )
                    end
                else
                    if actualTexture ~= slot.backgroundTextureName then
                        return string.format(
                            "background_texture_mismatch_%s_expected_%s_actual_%s",
                            frameName,
                            tostring(slot.backgroundTextureName),
                            tostring(actualTexture)
                        )
                    end
                end
            end

            return "ok"
        "#,
        ).unwrap();

        assert_eq!(
            result,
            "ok",
            "Character equipment slot icons should match inventory item textures when equipped and slot backgrounds when empty: {result}"
        );
    }
}

#[test]
fn character_panel_title_text_matches_player_name() {
    test_timeout! {
        let env = setup_full_env();
        open_character_panel(&env);

        let result: String = env.eval(
            r#"
            if not CharacterFrame then
                return "missing_character_frame"
            end
            if not CharacterFrame.TitleContainer then
                return "missing_title_container"
            end
            if not CharacterFrame.TitleContainer.TitleText then
                return "missing_title_text"
            end

            local expected = UnitPVPName("player")
            local actual = CharacterFrame.TitleContainer.TitleText:GetText()

            if actual ~= expected then
                return string.format(
                    "title_mismatch_expected_%s_actual_%s",
                    tostring(expected),
                    tostring(actual)
                )
            end

            return "ok"
        "#,
        ).unwrap();

        assert_eq!(
            result,
            "ok",
            "Character panel title text should match the player name shown by Blizzard's title path: {result}"
        );
    }
}

#[test]
fn spellbook_first_visible_item_icon_matches_spellbook_texture() {
    test_timeout! {
        let env = setup_full_env();
        open_spellbook(&env);

        let result: String = env.eval(
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
        ).unwrap();

        assert_eq!(
            result,
            "ok",
            "The first visible spellbook item icon should match C_SpellBook.GetSpellBookItemTexture for its slot: {result}"
        );
    }
}

#[test]
fn buff_frame_visible_count_matches_active_helpful_aura_count() {
    test_timeout! {
        let env = setup_full_env();

        env.exec(
            r#"
            A_Admin.ClearBuffs()
            A_Admin.AddBuff(99011, "Test Buff One", "134973", 30, 1)
            A_Admin.AddBuff(99012, "Test Buff Two", "134973", 45, 2)
            A_Admin.AddBuff(99013, "Test Buff Three", "134973", 50, 1)
            if BuffFrame and BuffFrame.SetBuffsExpandedState then
                BuffFrame:SetBuffsExpandedState(true)
            end
            "#,
        )
        .unwrap();
        refresh_buff_frame(&env);

        let result: String = env
            .eval(
                r#"
                if not BuffFrame then
                    return "missing_buff_frame"
                end
                if not BuffFrame.auraFrames then
                    return "missing_buff_aura_frames"
                end

                local _, firstSlot = C_UnitAuras.GetAuraSlots("player", "HELPFUL")
                if not firstSlot then
                    return "missing_helpful_auras"
                end

                local auraCount = 0
                local index = 1
                while true do
                    local aura = C_UnitAuras.GetAuraDataByIndex("player", index, "HELPFUL")
                    if not aura then
                        break
                    end
                    auraCount = auraCount + 1
                    index = index + 1
                end

                local visibleBuffButtons = 0
                for _, button in ipairs(BuffFrame.auraFrames) do
                    if button:IsShown()
                        and button.buttonInfo
                        and button.buttonInfo.auraType == "Buff"
                        and button.buttonInfo.index
                    then
                        visibleBuffButtons = visibleBuffButtons + 1
                    end
                end

                if visibleBuffButtons ~= auraCount then
                    return string.format(
                        "buff_count_mismatch_expected_%d_actual_%d",
                        auraCount,
                        visibleBuffButtons
                    )
                end

                return "ok"
            "#,
            )
            .unwrap();

        assert_eq!(
            result,
            "ok",
            "Visible BuffFrame buff buttons should match the active HELPFUL aura count from C_UnitAuras: {result}"
        );
    }
}
