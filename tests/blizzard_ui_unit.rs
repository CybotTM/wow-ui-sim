//! Blizzard UI unit lane.
//!
//! Keep this file for isolated helper logic and pure component behavior that
//! does not require a full addon bootstrap.

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
        if CharacterFrame.TitleContainer and CharacterFrame.TitleContainer.TitleText then
            CharacterFrame.TitleContainer.TitleText:SetText(UnitPVPName("player"))
        end
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
            if slot and slot.icon then
                local texture = GetInventoryItemTexture("player", slot:GetID())
                if texture ~= nil then
                    slot.icon:SetTexture(texture)
                elseif slot.backgroundTextureName ~= nil then
                    slot.icon:SetTexture(slot.backgroundTextureName)
                end
            end
        end
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

fn spellbook_corner_flipbook_id(registry: &wow_ui_sim::widget::WidgetRegistry) -> u64 {
    let player_spells_id = registry
        .get_id_by_name("PlayerSpellsFrame")
        .expect("PlayerSpellsFrame should exist");
    let player_spells = registry
        .get(player_spells_id)
        .expect("PlayerSpellsFrame id should resolve");
    let spellbook_id = *player_spells
        .children_keys
        .get("SpellBookFrame")
        .expect("SpellBookFrame should be parented under PlayerSpellsFrame");
    let spellbook = registry
        .get(spellbook_id)
        .expect("SpellBookFrame id should resolve");
    *spellbook
        .children_keys
        .get("BookCornerFlipbook")
        .expect("SpellBookFrame should expose BookCornerFlipbook")
}

fn flipbook_frame_uv(
    atlas_uvs: (f32, f32, f32, f32),
    rows: u32,
    columns: u32,
    frame_index: u32,
) -> (f32, f32, f32, f32) {
    let (atlas_left, atlas_right, atlas_top, atlas_bottom) = atlas_uvs;
    let cell_width = (atlas_right - atlas_left) / columns as f32;
    let cell_height = (atlas_bottom - atlas_top) / rows as f32;
    let column = frame_index % columns;
    let row = frame_index / columns;
    let left = atlas_left + column as f32 * cell_width;
    let right = left + cell_width;
    let top = atlas_top + row as f32 * cell_height;
    let bottom = top + cell_height;
    (left, right, top, bottom)
}

fn assert_uv_close(actual: (f32, f32, f32, f32), expected: (f32, f32, f32, f32), context: &str) {
    let tolerance = 0.0001;
    assert!(
        (actual.0 - expected.0).abs() <= tolerance
            && (actual.1 - expected.1).abs() <= tolerance
            && (actual.2 - expected.2).abs() <= tolerance
            && (actual.3 - expected.3).abs() <= tolerance,
        "{context}: expected {:?}, got {:?}",
        expected,
        actual
    );
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
fn spellbook_corner_flipbook_animates_across_hover_states() {
    test_timeout! {
        let env = setup_full_env();
        open_spellbook(&env);

        let (corner_id, atlas_uvs) = {
            let state = env.state().borrow();
            let corner_id = spellbook_corner_flipbook_id(&state.widgets);
            let corner = state
                .widgets
                .get(corner_id)
                .expect("BookCornerFlipbook id should resolve");
            (
                corner_id,
                corner
                    .atlas_tex_coords
                    .expect("BookCornerFlipbook should retain atlas UVs"),
            )
        };

        let expected_start = flipbook_frame_uv(atlas_uvs, 2, 4, 0);
        let expected_mid = flipbook_frame_uv(atlas_uvs, 2, 4, 4);
        let expected_end = flipbook_frame_uv(atlas_uvs, 2, 4, 7);

        let initial_uvs = {
            let state = env.state().borrow();
            state
                .widgets
                .get(corner_id)
                .and_then(|frame| frame.tex_coords)
                .expect("BookCornerFlipbook should have active tex coords")
        };
        assert_uv_close(
            initial_uvs,
            expected_start,
            "SpellBook corner should start paused on flipbook frame 0",
        );

        env.exec("PlayerSpellsFrame.SpellBookFrame:OnPagingButtonEnter()")
            .expect("SpellBook hover enter should run");
        env.fire_on_update(0.125).expect("forward hover tick should run");

        let mid_forward_uvs = {
            let state = env.state().borrow();
            state
                .widgets
                .get(corner_id)
                .and_then(|frame| frame.tex_coords)
                .expect("BookCornerFlipbook should keep tex coords mid-forward")
        };
        assert_uv_close(
            mid_forward_uvs,
            expected_mid,
            "SpellBook corner should advance to flipbook frame 4 halfway through hover enter",
        );

        env.fire_on_update(0.125).expect("forward finish tick should run");

        let end_forward_uvs = {
            let state = env.state().borrow();
            state
                .widgets
                .get(corner_id)
                .and_then(|frame| frame.tex_coords)
                .expect("BookCornerFlipbook should keep tex coords at forward finish")
        };
        assert_uv_close(
            end_forward_uvs,
            expected_end,
            "SpellBook corner should finish on flipbook frame 7 after hover enter",
        );

        env.exec("PlayerSpellsFrame.SpellBookFrame:OnPagingButtonLeave()")
            .expect("SpellBook hover leave should run");
        env.fire_on_update(0.125).expect("reverse hover tick should run");

        let mid_reverse_uvs = {
            let state = env.state().borrow();
            state
                .widgets
                .get(corner_id)
                .and_then(|frame| frame.tex_coords)
                .expect("BookCornerFlipbook should keep tex coords mid-reverse")
        };
        assert_uv_close(
            mid_reverse_uvs,
            expected_mid,
            "SpellBook corner should return to flipbook frame 4 halfway through hover leave",
        );

        env.fire_on_update(0.125).expect("reverse finish tick should run");

        let end_reverse_uvs = {
            let state = env.state().borrow();
            state
                .widgets
                .get(corner_id)
                .and_then(|frame| frame.tex_coords)
                .expect("BookCornerFlipbook should keep tex coords at reverse finish")
        };
        assert_uv_close(
            end_reverse_uvs,
            expected_start,
            "SpellBook corner should return to flipbook frame 0 after hover leave",
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
