//! Tests for GameTooltip implementation.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::widget::AnchorPoint;

#[test]
fn test_gametooltip_exists_and_has_correct_type() {
    let env = WowLuaEnv::new().unwrap();

    let exists: bool = env.eval("return GameTooltip ~= nil").unwrap();
    assert!(exists);

    let obj_type: String = env.eval("return GameTooltip:GetObjectType()").unwrap();
    assert_eq!(obj_type, "GameTooltip");
}

#[test]
fn test_gametooltip_strata_is_tooltip() {
    let env = WowLuaEnv::new().unwrap();

    let strata: String = env.eval("return GameTooltip:GetFrameStrata()").unwrap();
    assert_eq!(strata, "TOOLTIP");
}

#[test]
fn test_addline_and_numlines() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddLine("First line")
        GameTooltip:AddLine("Second line", 1, 0, 0)
        GameTooltip:AddLine("Third line", 0, 1, 0, true)
    "#,
    )
    .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(count, 3);
}

#[test]
fn test_adddoubleline_and_numlines() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddDoubleLine("Left", "Right")
        GameTooltip:AddDoubleLine("Name", "Value", 1, 1, 1, 0.5, 0.5, 0.5)
    "#,
    )
    .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(count, 2);
}

#[test]
fn test_clearlines_resets_count() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddLine("Line 1")
        GameTooltip:AddLine("Line 2")
        GameTooltip:ClearLines()
    "#,
    )
    .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_settext_clears_and_sets_first_line() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddLine("Old line 1")
        GameTooltip:AddLine("Old line 2")
        GameTooltip:SetText("New text")
    "#,
    )
    .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(count, 1, "SetText should clear existing lines and add one");
}

#[test]
fn test_setowner_and_isowned_and_getowner() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "TooltipOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_RIGHT")
    "#,
    )
    .unwrap();

    let is_owned: bool = env
        .eval("return GameTooltip:IsOwned(TooltipOwner)")
        .unwrap();
    assert!(is_owned, "GameTooltip should be owned by TooltipOwner");

    let owner_name: String = env.eval("return GameTooltip:GetOwner():GetName()").unwrap();
    assert_eq!(owner_name, "TooltipOwner");

    // Check that non-owner returns false
    env.exec(r#"local other = CreateFrame("Frame", "OtherFrame", UIParent)"#)
        .unwrap();
    let not_owned: bool = env.eval("return GameTooltip:IsOwned(OtherFrame)").unwrap();
    assert!(!not_owned, "GameTooltip should not be owned by OtherFrame");
}

#[test]
fn test_getanchortype_after_setowner() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "AnchorTestOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_BOTTOMRIGHT")
    "#,
    )
    .unwrap();

    let anchor: String = env.eval("return GameTooltip:GetAnchorType()").unwrap();
    assert_eq!(anchor, "ANCHOR_BOTTOMRIGHT");
}

#[test]
fn test_on_tooltip_cleared_fires_on_clearlines() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        _G.tooltip_cleared_count = 0
        GameTooltip:SetScript("OnTooltipCleared", function()
            _G.tooltip_cleared_count = _G.tooltip_cleared_count + 1
        end)
        GameTooltip:AddLine("Some line")
        GameTooltip:ClearLines()
    "#,
    )
    .unwrap();

    let count: i32 = env.eval("return _G.tooltip_cleared_count").unwrap();
    assert_eq!(count, 1, "OnTooltipCleared should fire once on ClearLines");
}

#[test]
fn test_on_tooltip_cleared_fires_on_setowner() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        _G.cleared_count = 0
        GameTooltip:SetScript("OnTooltipCleared", function()
            _G.cleared_count = _G.cleared_count + 1
        end)
        local owner = CreateFrame("Frame", "ClearedTestOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_NONE")
    "#,
    )
    .unwrap();

    let count: i32 = env.eval("return _G.cleared_count").unwrap();
    assert_eq!(count, 1, "OnTooltipCleared should fire on SetOwner");
}

#[test]
fn test_isobjecttype_frame_returns_true_for_gametooltip() {
    let env = WowLuaEnv::new().unwrap();

    let is_frame: bool = env
        .eval("return GameTooltip:IsObjectType('Frame')")
        .unwrap();
    assert!(
        is_frame,
        "GameTooltip:IsObjectType('Frame') should return true"
    );

    let is_region: bool = env
        .eval("return GameTooltip:IsObjectType('Region')")
        .unwrap();
    assert!(
        is_region,
        "GameTooltip:IsObjectType('Region') should return true"
    );

    let is_tooltip: bool = env
        .eval("return GameTooltip:IsObjectType('GameTooltip')")
        .unwrap();
    assert!(
        is_tooltip,
        "GameTooltip:IsObjectType('GameTooltip') should return true"
    );

    let is_button: bool = env
        .eval("return GameTooltip:IsObjectType('Button')")
        .unwrap();
    assert!(
        !is_button,
        "GameTooltip:IsObjectType('Button') should return false"
    );
}

#[test]
fn test_isobjecttype_for_other_types() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TypeTestButton", UIParent)
        local cb = CreateFrame("CheckButton", "TypeTestCheckButton", UIParent)
        local frame = CreateFrame("Frame", "TypeTestFrame", UIParent)
    "#,
    )
    .unwrap();

    // Button is a Frame
    let btn_is_frame: bool = env
        .eval("return TypeTestButton:IsObjectType('Frame')")
        .unwrap();
    assert!(btn_is_frame);

    // CheckButton is a Button
    let cb_is_button: bool = env
        .eval("return TypeTestCheckButton:IsObjectType('Button')")
        .unwrap();
    assert!(cb_is_button);

    // CheckButton is a Frame
    let cb_is_frame: bool = env
        .eval("return TypeTestCheckButton:IsObjectType('Frame')")
        .unwrap();
    assert!(cb_is_frame);

    // Frame is NOT a Button
    let frame_is_button: bool = env
        .eval("return TypeTestFrame:IsObjectType('Button')")
        .unwrap();
    assert!(!frame_is_button);
}

#[test]
fn test_setminimumwidth_and_getminimumwidth() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetMinimumWidth(150)").unwrap();

    let width: f32 = env.eval("return GameTooltip:GetMinimumWidth()").unwrap();
    assert_eq!(width, 150.0);
}

#[test]
fn test_setpadding_and_getpadding() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetPadding(8)").unwrap();

    let padding: f32 = env.eval("return GameTooltip:GetPadding()").unwrap();
    assert_eq!(padding, 8.0);
}

#[test]
fn test_fadeout_hides_and_clears_owner() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "FadeOutOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_RIGHT")
        GameTooltip:FadeOut()
    "#,
    )
    .unwrap();

    let visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    assert!(!visible, "FadeOut should hide the tooltip");

    let has_owner: bool = env.eval("return GameTooltip:GetOwner() ~= nil").unwrap();
    assert!(!has_owner, "FadeOut should clear the owner");
}

#[test]
fn test_appendtext() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddLine("Hello")
        GameTooltip:AppendText(" World")
    "#,
    )
    .unwrap();

    // Verify through tooltip data
    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert_eq!(td.lines.len(), 1);
    assert_eq!(td.lines[0].left_text, "Hello World");
}

#[test]
fn test_setowner_makes_tooltip_visible() {
    let env = WowLuaEnv::new().unwrap();

    // GameTooltip starts hidden
    let initially_visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    assert!(!initially_visible, "GameTooltip should start hidden");

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "VisOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_RIGHT")
    "#,
    )
    .unwrap();

    let now_visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    assert!(now_visible, "SetOwner should make tooltip visible");
}

#[test]
fn test_createframe_gametooltip_type() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local tt = CreateFrame("GameTooltip", "CustomTooltip", UIParent)
        tt:AddLine("Test")
    "#,
    )
    .unwrap();

    let obj_type: String = env.eval("return CustomTooltip:GetObjectType()").unwrap();
    assert_eq!(obj_type, "GameTooltip");

    let count: i32 = env.eval("return CustomTooltip:NumLines()").unwrap();
    assert_eq!(count, 1);

    let strata: String = env.eval("return CustomTooltip:GetFrameStrata()").unwrap();
    assert_eq!(strata, "TOOLTIP");
}

#[test]
fn test_other_tooltip_frames_exist() {
    let env = WowLuaEnv::new().unwrap();

    let item_ref: bool = env.eval("return ItemRefTooltip ~= nil").unwrap();
    let shopping1: bool = env.eval("return ShoppingTooltip1 ~= nil").unwrap();
    let shopping2: bool = env.eval("return ShoppingTooltip2 ~= nil").unwrap();
    let friends: bool = env.eval("return FriendsTooltip ~= nil").unwrap();

    assert!(item_ref, "ItemRefTooltip should exist");
    assert!(shopping1, "ShoppingTooltip1 should exist");
    assert!(shopping2, "ShoppingTooltip2 should exist");
    assert!(friends, "FriendsTooltip should exist");

    // All should be GameTooltip type
    let item_type: String = env.eval("return ItemRefTooltip:GetObjectType()").unwrap();
    assert_eq!(item_type, "GameTooltip");
}

#[test]
fn test_tooltip_anchor_right_sets_anchors() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "AnchorRightOwner", UIParent)
        owner:SetSize(100, 30)
        owner:SetPoint("CENTER")
        GameTooltip:SetOwner(owner, "ANCHOR_RIGHT")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let frame = state.widgets.get(gt_id).unwrap();

    assert_eq!(frame.anchors.len(), 1, "ANCHOR_RIGHT should set one anchor");
    let anchor = &frame.anchors[0];
    assert_eq!(
        anchor.point,
        AnchorPoint::TopLeft,
        "tooltip point should be TopLeft"
    );
    assert_eq!(
        anchor.relative_point,
        AnchorPoint::TopRight,
        "owner point should be TopRight"
    );

    let owner_id = state.widgets.get_id_by_name("AnchorRightOwner").unwrap();
    assert_eq!(anchor.relative_to_id, Some(owner_id as usize));
}

#[test]
fn test_tooltip_anchor_none_no_anchors() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "AnchorNoneOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_NONE")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let frame = state.widgets.get(gt_id).unwrap();

    assert!(
        frame.anchors.is_empty(),
        "ANCHOR_NONE should not set anchors"
    );
}

#[test]
fn test_tooltip_anchor_cursor_uses_absolute_position() {
    let env = WowLuaEnv::new().unwrap();

    // Set mouse position before SetOwner
    env.state().borrow_mut().mouse_position = Some((200.0, 300.0));

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "AnchorCursorOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_CURSOR")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let frame = state.widgets.get(gt_id).unwrap();

    assert_eq!(
        frame.anchors.len(),
        1,
        "ANCHOR_CURSOR should set one anchor"
    );
    let anchor = &frame.anchors[0];
    assert_eq!(anchor.point, AnchorPoint::TopLeft);
    assert!(
        anchor.relative_to_id.is_none(),
        "ANCHOR_CURSOR should not reference owner"
    );
    assert!(
        (anchor.x_offset - 200.0).abs() < 0.1,
        "x_offset should be mouse x"
    );
    assert!(
        (anchor.y_offset - 320.0).abs() < 0.1,
        "y_offset should be mouse y + 20"
    );
}

#[test]
fn test_set_item_by_id_populates_lines() {
    let env = WowLuaEnv::new().unwrap();

    // Item 229181: Ordained Forge Maul, epic (quality 4), ilvl 610, Two-Hand
    env.exec("GameTooltip:SetItemByID(229181)").unwrap();

    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(num_lines > 0, "SetItemByID should populate tooltip lines");

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert_eq!(td.lines[0].left_text, "Ordained Forge Maul");
    // Epic quality is purple (0.64, 0.21, 0.93)
    assert!((td.lines[0].left_color.0 - 0.64).abs() < 0.01);
    assert_eq!(td.lines[1].left_text, "Item Level 610");
    assert_eq!(td.lines[2].left_text, "Two-Hand");
}

#[test]
fn test_set_item_by_id_makes_tooltip_visible() {
    let env = WowLuaEnv::new().unwrap();

    let initially_visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    assert!(!initially_visible);

    env.exec("GameTooltip:SetItemByID(229181)").unwrap();

    let visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    assert!(visible, "SetItemByID should make tooltip visible");
}

#[test]
fn test_set_hyperlink_populates_lines() {
    let env = WowLuaEnv::new().unwrap();

    // Standard WoW hyperlink format
    env.exec(
        r#"GameTooltip:SetHyperlink("|Hitem:229181:0:0:0:0:0:0:0:0:0|h[Ordained Forge Maul]|h")"#,
    )
    .unwrap();

    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(num_lines > 0, "SetHyperlink should populate tooltip lines");

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert_eq!(td.lines[0].left_text, "Ordained Forge Maul");
}

#[test]
fn test_set_hyperlink_short_format() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"GameTooltip:SetHyperlink("item:229181")"#)
        .unwrap();

    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(
        num_lines > 0,
        "SetHyperlink with short format should populate lines"
    );
}

#[test]
fn test_get_num_lines_returns_actual_count() {
    let env = WowLuaEnv::new().unwrap();

    let zero: i32 = env.eval("return GameTooltip:GetNumLines()").unwrap();
    assert_eq!(zero, 0, "GetNumLines should return 0 when no lines");

    env.exec("GameTooltip:SetItemByID(229181)").unwrap();

    let count: i32 = env.eval("return GameTooltip:GetNumLines()").unwrap();
    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(
        count, num_lines,
        "GetNumLines should match NumLines: got {count} vs {num_lines}"
    );
    assert!(count > 0, "GetNumLines should be > 0 after SetItemByID");
}

#[test]
fn test_set_unit_aura_populates_lines() {
    let env = WowLuaEnv::new().unwrap();
    // Player has default buffs; buff 1 should exist
    let has_buff: bool = env.eval("return UnitBuff('player', 1) ~= nil").unwrap();
    assert!(has_buff, "Player should have at least one buff");

    env.exec(r#"GameTooltip:SetUnitAura("player", 1, "HELPFUL")"#)
        .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(count > 0, "SetUnitAura should populate tooltip lines");

    let visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    assert!(visible, "SetUnitAura should make tooltip visible");

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert!(
        !td.lines[0].left_text.is_empty(),
        "First line should be the buff name"
    );
}

#[test]
fn test_set_unit_buff_populates_lines() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"GameTooltip:SetUnitBuff("player", 1)"#).unwrap();
    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(count > 0, "SetUnitBuff should populate tooltip lines");
}

#[test]
fn test_set_unit_aura_invalid_index_no_crash() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"GameTooltip:SetUnitAura("player", 999, "HELPFUL")"#)
        .unwrap();
    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(count, 0, "Invalid index should leave tooltip empty");
}

#[test]
fn test_set_unit_player_populates_tooltip() {
    let env = WowLuaEnv::new().unwrap();

    let result: bool = env
        .eval(r#"return GameTooltip:SetUnit("player") == true"#)
        .unwrap();
    assert!(result, "SetUnit('player') should return true");

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(count >= 2, "Should have at least name + level lines");

    let visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    assert!(visible, "SetUnit should make tooltip visible");

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert_eq!(td.lines[0].left_text, state.player.name);
    assert!(
        td.lines[1].left_text.contains("Level"),
        "Second line should contain level info"
    );
}

#[test]
fn test_set_unit_invalid_returns_false() {
    let env = WowLuaEnv::new().unwrap();
    let result: bool = env
        .eval(r#"return GameTooltip:SetUnit("nonexistent")"#)
        .unwrap();
    assert!(!result, "SetUnit with invalid unit should return false");
}

#[test]
fn test_set_inventory_item_shows_tooltip() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            -- Slot 1 = Head, has default equipped item (Entombed Seraph's Casque)
            GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
            local hasItem = GameTooltip:SetInventoryItem("player", 1)
            if not hasItem then return "no_item" end
            local lines = GameTooltip:NumLines()
            if lines < 2 then return "lines=" .. tostring(lines) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "SetInventoryItem should populate tooltip: {result}"
    );
}

#[test]
fn test_set_inventory_item_empty_slot() {
    let env = WowLuaEnv::new().unwrap();
    let result: bool = env
        .eval(
            r#"
            GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
            -- Slot 4 = shirt, typically empty
            return GameTooltip:SetInventoryItem("player", 4)
            "#,
        )
        .unwrap();
    assert!(!result, "Empty slot should return false");
}

#[test]
fn test_set_inventory_item_tooltip_content() {
    let env = WowLuaEnv::new().unwrap();
    // Populate the tooltip for slot 1 (Head: Entombed Seraph's Casque, ilvl 571)
    env.exec(
        r#"
        GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
        GameTooltip:SetInventoryItem("player", 1)
        "#,
    )
    .unwrap();

    // Read tooltip lines from Rust state
    let tooltip_id = {
        let state = env.state().borrow();
        state
            .widgets
            .get_id_by_name("GameTooltip")
            .expect("GameTooltip not found")
    };
    let state = env.state().borrow();
    let td = state.tooltips.get(&tooltip_id).expect("No tooltip data");

    // Line 1: item name with quality color
    assert_eq!(td.lines[0].left_text, "Entombed Seraph's Casque");
    let (r, _g, _b) = td.lines[0].left_color;
    assert!(
        r > 0.5,
        "Epic quality title should have purple/red color component"
    );

    // Line 2: item level
    assert!(
        td.lines[1].left_text.contains("571"),
        "Second line should contain ilvl 571, got: {}",
        td.lines[1].left_text
    );

    // Line 3: equip slot
    assert!(
        td.lines.len() >= 3,
        "Should have at least 3 lines (name, ilvl, slot)"
    );
}

// --- Spell tooltip tests ---

#[test]
fn test_set_spell_by_id_populates_lines() {
    let env = WowLuaEnv::new().unwrap();

    // Flash of Light (19750): has cast time (1.5s), description, and power cost
    env.exec("GameTooltip:SetSpellByID(19750)").unwrap();

    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(
        num_lines >= 2,
        "SetSpellByID should populate tooltip lines, got {num_lines}"
    );

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert_eq!(td.lines[0].left_text, "Flash of Light");
}

#[test]
fn test_set_spell_by_id_shows_cast_time() {
    let env = WowLuaEnv::new().unwrap();

    // Flash of Light has 1500ms cast time
    env.exec("GameTooltip:SetSpellByID(19750)").unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    let has_cast_line = td.lines.iter().any(|l| l.left_text.contains("sec cast"));
    assert!(has_cast_line, "Should have a cast time line");
}

#[test]
fn test_set_spell_by_id_instant_cast() {
    let env = WowLuaEnv::new().unwrap();

    // Crusader Strike (35395): instant cast
    env.exec("GameTooltip:SetSpellByID(35395)").unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert_eq!(td.lines[0].left_text, "Crusader Strike");
    let has_instant = td.lines.iter().any(|l| l.left_text == "Instant");
    assert!(has_instant, "Instant cast spells should show 'Instant'");
}

#[test]
fn test_set_spell_by_id_makes_tooltip_visible() {
    let env = WowLuaEnv::new().unwrap();

    let initially_visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    assert!(!initially_visible);

    env.exec("GameTooltip:SetSpellByID(19750)").unwrap();

    let visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    assert!(visible, "SetSpellByID should make tooltip visible");
}

#[test]
fn test_set_spell_by_id_fires_on_tooltip_set_spell() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        _G.spell_set_count = 0
        GameTooltip:SetScript("OnTooltipSetSpell", function()
            _G.spell_set_count = _G.spell_set_count + 1
        end)
        GameTooltip:SetSpellByID(19750)
    "#,
    )
    .unwrap();

    let count: i32 = env.eval("return _G.spell_set_count").unwrap();
    assert_eq!(count, 1, "OnTooltipSetSpell should fire once");
}

#[test]
fn test_get_spell_returns_spell_data_after_set() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(19750)").unwrap();

    let name: String = env
        .eval("local name, id = GameTooltip:GetSpell(); return name")
        .unwrap();
    assert_eq!(name, "Flash of Light");

    let id: i32 = env
        .eval("local name, id = GameTooltip:GetSpell(); return id")
        .unwrap();
    assert_eq!(id, 19750);
}

#[test]
fn test_get_spell_returns_nil_when_no_spell() {
    let env = WowLuaEnv::new().unwrap();

    let is_nil: bool = env
        .eval("local name, id = GameTooltip:GetSpell(); return name == nil and id == nil")
        .unwrap();
    assert!(
        is_nil,
        "GetSpell should return nil,nil when no spell is set"
    );
}

#[test]
fn test_clear_lines_clears_spell_id() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:SetSpellByID(19750)
        GameTooltip:ClearLines()
    "#,
    )
    .unwrap();

    let is_nil: bool = env
        .eval("local name, id = GameTooltip:GetSpell(); return name == nil")
        .unwrap();
    assert!(is_nil, "ClearLines should clear spell_id");
}

#[test]
fn test_set_spell_by_id_unknown_spell_is_noop() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(999999999)").unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(count, 0, "Unknown spell should not add lines");
}

// --- GetLeftLine / GetRightLine tests ---

#[test]
fn test_get_left_line_returns_fontstring() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"GameTooltip:AddLine("Hello")"#).unwrap();

    let obj_type: String = env
        .eval("return GameTooltip:GetLeftLine(1):GetObjectType()")
        .unwrap();
    assert_eq!(obj_type, "FontString");
}

#[test]
fn test_get_left_line_has_correct_text() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddLine("First line")
        GameTooltip:AddLine("Second line")
    "#,
    )
    .unwrap();

    let text1: String = env
        .eval("return GameTooltip:GetLeftLine(1):GetText()")
        .unwrap();
    assert_eq!(text1, "First line");

    let text2: String = env
        .eval("return GameTooltip:GetLeftLine(2):GetText()")
        .unwrap();
    assert_eq!(text2, "Second line");
}

#[test]
fn test_get_right_line_has_correct_text() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"GameTooltip:AddDoubleLine("Left", "Right")"#)
        .unwrap();

    let right_text: String = env
        .eval("return GameTooltip:GetRightLine(1):GetText()")
        .unwrap();
    assert_eq!(right_text, "Right");
}

#[test]
fn test_get_left_line_out_of_range_returns_nil() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"GameTooltip:AddLine("Only one")"#).unwrap();

    let is_nil: bool = env
        .eval("return GameTooltip:GetLeftLine(5) == nil")
        .unwrap();
    assert!(is_nil, "Out-of-range index should return nil");

    let zero_nil: bool = env
        .eval("return GameTooltip:GetLeftLine(0) == nil")
        .unwrap();
    assert!(zero_nil, "Index 0 should return nil");
}

#[test]
fn test_tooltip_fontstring_globals_exist() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddLine("Line 1")
        GameTooltip:AddLine("Line 2")
        -- Force FontString creation by accessing them
        local _ = GameTooltip:GetLeftLine(1)
        local _ = GameTooltip:GetLeftLine(2)
    "#,
    )
    .unwrap();

    let exists1: bool = env.eval("return GameTooltipTextLeft1 ~= nil").unwrap();
    assert!(exists1, "GameTooltipTextLeft1 global should exist");

    let exists2: bool = env.eval("return GameTooltipTextLeft2 ~= nil").unwrap();
    assert!(exists2, "GameTooltipTextLeft2 global should exist");

    let text: String = env.eval("return GameTooltipTextLeft1:GetText()").unwrap();
    assert_eq!(text, "Line 1");
}

#[test]
fn test_get_left_line_after_set_item_by_id() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetItemByID(229181)").unwrap();

    let text: String = env
        .eval("return GameTooltip:GetLeftLine(1):GetText()")
        .unwrap();
    assert_eq!(text, "Ordained Forge Maul");
}

#[test]
fn test_get_right_line_no_right_text_returns_nil_text() {
    let env = WowLuaEnv::new().unwrap();

    // Single-text line has no right text
    env.exec(r#"GameTooltip:AddLine("Left only")"#).unwrap();

    // GetRightLine still returns a FontString, but with nil text
    let right_text_nil: bool = env
        .eval(
            r#"
        local fs = GameTooltip:GetRightLine(1)
        return fs:GetText() == nil or fs:GetText() == ""
    "#,
        )
        .unwrap();
    assert!(
        right_text_nil,
        "Right line text should be nil/empty for single-text lines"
    );
}

// --- AddTexture / AddAtlas tests ---

#[test]
fn test_add_texture_increments_numlines() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddLine("Header")
        GameTooltip:AddTexture(136243)
    "#,
    )
    .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(count, 2, "AddTexture should add a line");
}

#[test]
fn test_add_atlas_increments_numlines() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddLine("Header")
        GameTooltip:AddAtlas("groupfinder-icon-friend", {})
    "#,
    )
    .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(count, 2, "AddAtlas should add a line");
}

#[test]
fn test_add_texture_stores_file_data_id() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:AddTexture(136243)").unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert_eq!(td.lines.len(), 1);
    match &td.lines[0].texture {
        Some(wow_ui_sim::lua_api::tooltip::TooltipTexture::FileDataId(id)) => {
            assert_eq!(*id, 136243);
        }
        other => panic!("Expected FileDataId, got {:?}", other.is_some()),
    }
}

#[test]
fn test_add_atlas_stores_atlas_name() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"GameTooltip:AddAtlas("groupfinder-icon-friend")"#)
        .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert_eq!(td.lines.len(), 1);
    match &td.lines[0].texture {
        Some(wow_ui_sim::lua_api::tooltip::TooltipTexture::Atlas(name)) => {
            assert_eq!(name, "groupfinder-icon-friend");
        }
        other => panic!("Expected Atlas, got {:?}", other.is_some()),
    }
}

#[test]
fn test_add_texture_with_string_id() {
    let env = WowLuaEnv::new().unwrap();

    // Some addons pass texture path as string
    env.exec(r#"GameTooltip:AddTexture("136243")"#).unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(count, 1, "AddTexture with string ID should add a line");
}

#[test]
fn test_clearlines_clears_texture_lines() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddTexture(136243)
        GameTooltip:AddAtlas("test-atlas")
        GameTooltip:ClearLines()
    "#,
    )
    .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(count, 0, "ClearLines should remove texture lines too");
}

// --- SetCustomLineSpacing / GetCustomLineSpacing tests ---

#[test]
fn test_set_custom_line_spacing_and_get() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetCustomLineSpacing(8)").unwrap();

    let spacing: f64 = env
        .eval("return GameTooltip:GetCustomLineSpacing()")
        .unwrap();
    assert!((spacing - 8.0).abs() < 0.01);
}

#[test]
fn test_get_custom_line_spacing_default_is_zero() {
    let env = WowLuaEnv::new().unwrap();

    let spacing: f64 = env
        .eval("return GameTooltip:GetCustomLineSpacing()")
        .unwrap();
    assert_eq!(
        spacing, 0.0,
        "Default should be 0 (meaning use default 2px)"
    );
}

#[test]
fn test_set_custom_line_spacing_stores_in_tooltip_data() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetCustomLineSpacing(5)").unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert_eq!(td.line_spacing, Some(5.0));
}

#[test]
fn test_set_custom_line_spacing_on_custom_tooltip() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local tt = CreateFrame("GameTooltip", "SpacingTestTooltip", UIParent)
        tt:SetCustomLineSpacing(12)
    "#,
    )
    .unwrap();

    let spacing: f64 = env
        .eval("return SpacingTestTooltip:GetCustomLineSpacing()")
        .unwrap();
    assert!((spacing - 12.0).abs() < 0.01);
}

// --- Text wrapping tests ---

#[test]
fn test_addline_wrap_flag_stored() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        GameTooltip:AddLine("Short title")
        GameTooltip:AddLine("This is a long description that should wrap", 1, 1, 1, true)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert!(!td.lines[0].wrap, "First line should not wrap");
    assert!(td.lines[1].wrap, "Second line (with wrap=true) should wrap");
}

fn update_tooltip_sizes(env: &WowLuaEnv) {
    use std::path::PathBuf;
    use wow_ui_sim::render::font::WowFontSystem;

    let mut font_sys = WowFontSystem::new(&PathBuf::from("./fonts"));
    let mut state = env.state().borrow_mut();
    wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut font_sys);
}

#[test]
fn test_wrapped_line_does_not_expand_width() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "WrapWidthOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_NONE")
        GameTooltip:AddLine("Short")
        GameTooltip:AddLine("This is a very very very very very very very very very long line that should word-wrap within the tooltip width rather than expanding it to be extremely wide", 1, 1, 1, true)
    "#,
    )
    .unwrap();

    update_tooltip_sizes(&env);

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let frame = state.widgets.get(gt_id).unwrap();

    // Width should be determined by the short line, not the long wrapped one.
    // "Short" at 14px ~ 40-60px + 24px padding = ~64-84px total.
    assert!(
        frame.width < 200.0,
        "Tooltip width should be small (non-wrapping lines only), got {}",
        frame.width
    );
}

#[test]
fn test_wrapped_line_increases_height() {
    let env = WowLuaEnv::new().unwrap();

    // Single non-wrapping line
    env.exec(
        r#"
        local owner = CreateFrame("Frame", "WrapHeightOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_NONE")
        GameTooltip:AddLine("Short")
    "#,
    )
    .unwrap();
    update_tooltip_sizes(&env);
    let height_one_line = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        state.widgets.get(gt_id).unwrap().height
    };

    // Clear and add a short line + a long wrapping line
    env.exec(
        r#"
        GameTooltip:ClearLines()
        GameTooltip:AddLine("Short")
        GameTooltip:AddLine("This is a long description that should definitely wrap onto multiple lines when rendered within the narrow tooltip width constraint", 1, 1, 1, true)
    "#,
    )
    .unwrap();
    update_tooltip_sizes(&env);

    let height_with_wrap = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        state.widgets.get(gt_id).unwrap().height
    };

    assert!(
        height_with_wrap > height_one_line,
        "Wrapped text should increase tooltip height: one_line={}, with_wrap={}",
        height_one_line,
        height_with_wrap
    );
}

#[test]
fn test_double_line_width_includes_gap() {
    let env = WowLuaEnv::new().unwrap();

    // A single-text line for baseline width
    env.exec(
        r#"
        local owner = CreateFrame("Frame", "GapTestOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_NONE")
        GameTooltip:AddLine("Left text only")
    "#,
    )
    .unwrap();
    update_tooltip_sizes(&env);
    let width_single = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        state.widgets.get(gt_id).unwrap().width
    };

    // Now a double line — should be wider due to right text + gap
    env.exec(
        r#"
        GameTooltip:ClearLines()
        GameTooltip:AddDoubleLine("Left text only", "Right text")
    "#,
    )
    .unwrap();
    update_tooltip_sizes(&env);
    let width_double = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        state.widgets.get(gt_id).unwrap().width
    };

    assert!(
        width_double > width_single,
        "Double-line tooltip should be wider than single: single={}, double={}",
        width_single,
        width_double
    );
}

// --- Tooltip NineSlice sizing tests ---

#[test]
fn test_tooltip_sizing_includes_padding() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "PadTestOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_NONE")
        GameTooltip:AddLine("X")
    "#,
    )
    .unwrap();
    update_tooltip_sizes(&env);

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let frame = state.widgets.get(gt_id).unwrap();

    // Padding is 12px on each side (24 total per axis).
    // Single line at 14px header: height = ceil(14*1.2) = 17 + 24 padding = 41.
    // Width = text width + 24 padding.
    assert!(
        frame.width > 24.0,
        "Width should be > padding alone, got {}",
        frame.width
    );
    assert!(
        frame.height > 24.0,
        "Height should be > padding alone, got {}",
        frame.height
    );
    // Single header line (14pt * 1.2 = ~17px) + 24px vertical padding = ~41px
    let expected_height = (14.0_f32 * 1.2).ceil() + 24.0;
    assert!(
        (frame.height - expected_height).abs() < 1.0,
        "Expected height ~{}, got {}",
        expected_height,
        frame.height
    );
}

#[test]
fn test_tooltip_height_grows_with_lines() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "GrowOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_NONE")
        GameTooltip:AddLine("Line 1")
    "#,
    )
    .unwrap();
    update_tooltip_sizes(&env);
    let h1 = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        state.widgets.get(gt_id).unwrap().height
    };

    env.exec(r#"GameTooltip:AddLine("Line 2")"#).unwrap();
    update_tooltip_sizes(&env);
    let h2 = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        state.widgets.get(gt_id).unwrap().height
    };

    env.exec(r#"GameTooltip:AddLine("Line 3")"#).unwrap();
    update_tooltip_sizes(&env);
    let h3 = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        state.widgets.get(gt_id).unwrap().height
    };

    assert!(h2 > h1, "2 lines should be taller than 1: h1={h1}, h2={h2}");
    assert!(h3 > h2, "3 lines should be taller than 2: h2={h2}, h3={h3}");
}

#[test]
fn test_tooltip_nineslice_child_accessible() {
    let env = WowLuaEnv::new().unwrap();

    // Verify NineSlice exists in Rust children_keys
    let ns_exists_rust = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        let frame = state.widgets.get(gt_id).unwrap();
        frame.children_keys.contains_key("NineSlice")
    };
    assert!(ns_exists_rust, "NineSlice should be in Rust children_keys");

    let has_ns: bool = env.eval("return GameTooltip.NineSlice ~= nil").unwrap();
    assert!(
        has_ns,
        "GameTooltip.NineSlice should be accessible from Lua"
    );

    let obj_type: String = env
        .eval("return GameTooltip.NineSlice:GetObjectType()")
        .unwrap();
    assert_eq!(obj_type, "Frame", "NineSlice child should be a Frame");
}

#[test]
fn test_tooltip_sizing_skipped_when_hidden() {
    let env = WowLuaEnv::new().unwrap();

    // GameTooltip starts hidden — sizing should not change its dimensions
    let width_before = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        state.widgets.get(gt_id).unwrap().width
    };

    // Add a line but don't show the tooltip (no SetOwner)
    env.exec(r#"GameTooltip:AddLine("Hidden line")"#).unwrap();
    update_tooltip_sizes(&env);

    let width_after = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        state.widgets.get(gt_id).unwrap().width
    };

    assert_eq!(
        width_before, width_after,
        "Sizing should skip hidden tooltips"
    );
}

#[test]
fn test_tooltip_min_width_respected() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "MinWOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_NONE")
        GameTooltip:SetMinimumWidth(300)
        GameTooltip:AddLine("Short")
    "#,
    )
    .unwrap();
    update_tooltip_sizes(&env);

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let frame = state.widgets.get(gt_id).unwrap();

    // min_width=300 + 24px padding = 324
    assert!(
        frame.width >= 324.0,
        "Width should respect min_width (300+padding), got {}",
        frame.width
    );
}

// --- Invalid anchor type warning tests ---

#[test]
fn test_invalid_anchor_type_warns_and_defaults() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "BadAnchorOwner", UIParent)
        GameTooltip:SetOwner(owner, "INVALID_ANCHOR")
    "#,
    )
    .unwrap();

    // Should still work with default ANCHOR_LEFT
    let anchor: String = env.eval("return GameTooltip:GetAnchorType()").unwrap();
    assert_eq!(
        anchor, "ANCHOR_LEFT",
        "Invalid anchor should default to ANCHOR_LEFT"
    );

    // Should have logged a warning
    let state = env.state().borrow();
    let has_warning = state
        .lua_errors
        .iter()
        .any(|e| e.contains("invalid anchor type") && e.contains("INVALID_ANCHOR"));
    assert!(has_warning, "Should warn about invalid anchor type");
}

#[test]
fn test_valid_anchor_type_no_warning() {
    let env = WowLuaEnv::new().unwrap();

    env.state().borrow_mut().lua_errors.clear();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "GoodAnchorOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_RIGHT")
    "#,
    )
    .unwrap();

    let anchor: String = env.eval("return GameTooltip:GetAnchorType()").unwrap();
    assert_eq!(anchor, "ANCHOR_RIGHT");

    let state = env.state().borrow();
    let has_anchor_warning = state
        .lua_errors
        .iter()
        .any(|e| e.contains("invalid anchor type"));
    assert!(
        !has_anchor_warning,
        "Valid anchor should not produce a warning"
    );
}
