use wow_ui_sim::lua_api::WowLuaEnv;

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
fn test_set_item_by_id_6948_contains_hearthstone_line() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetItemByID(6948)").unwrap();

    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(
        num_lines > 0,
        "SetItemByID(6948) should populate tooltip lines"
    );

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert!(
        td.lines
            .iter()
            .any(|line| line.left_text.contains("Hearthstone")),
        "Tooltip lines for item 6948 should contain Hearthstone, got: {:?}",
        td.lines
            .iter()
            .map(|line| line.left_text.clone())
            .collect::<Vec<_>>()
    );
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
fn test_set_hyperlink_spell_link_populates_spell_tooltip() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"GameTooltip:SetHyperlink(GetSpellLink(19750))"#)
        .unwrap();

    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(
        num_lines >= 2,
        "SetHyperlink with a spell link should populate spell tooltip lines"
    );

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    assert_eq!(td.lines[0].left_text, "Flash of Light");
    assert_eq!(
        state.tooltips.get(&gt_id).and_then(|td| td.spell_id),
        Some(19750)
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
fn test_set_unit_aura_by_aura_instance_id_populates_lines() {
    let env = WowLuaEnv::new().unwrap();
    let aura_instance_id = {
        let state = env.state().borrow();
        state
            .player
            .buffs
            .first()
            .map(|aura| aura.aura_instance_id)
            .expect("Player should have at least one buff")
    };

    env.exec(&format!(
        r#"GameTooltip:SetUnitAuraByAuraInstanceID("player", {aura_instance_id})"#
    ))
    .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(
        count > 0,
        "SetUnitAuraByAuraInstanceID should populate tooltip lines for a valid player aura"
    );
}

#[test]
fn test_set_unit_debuff_by_aura_instance_id_does_not_show_helpful_buffs() {
    let env = WowLuaEnv::new().unwrap();
    let aura_instance_id = {
        let state = env.state().borrow();
        state
            .player
            .buffs
            .first()
            .map(|aura| aura.aura_instance_id)
            .expect("Player should have at least one buff")
    };

    env.exec(&format!(
        r#"GameTooltip:SetUnitDebuffByAuraInstanceID("player", {aura_instance_id}, "HARMFUL")"#
    ))
    .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(
        count, 0,
        "SetUnitDebuffByAuraInstanceID should leave the tooltip empty for helpful-only player buffs"
    );
}

#[test]
fn test_set_unit_buff_by_aura_instance_id_respects_unit() {
    let env = WowLuaEnv::new().unwrap();
    let aura_instance_id = {
        let state = env.state().borrow();
        state
            .player
            .buffs
            .first()
            .map(|aura| aura.aura_instance_id)
            .expect("Player should have at least one buff")
    };

    env.exec(&format!(
        r#"GameTooltip:SetUnitBuffByAuraInstanceID("target", {aura_instance_id})"#
    ))
    .unwrap();

    let count: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert_eq!(
        count, 0,
        "SetUnitBuffByAuraInstanceID should not read player buffs when asked for another unit"
    );
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
fn test_set_unit_party_member_populates_tooltip_and_fires_event() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetPartySize(1)").unwrap();

    let result: String = env
        .eval(
            r#"
            local eventCount = 0
            local eventName, eventUnit, eventGuid
            GameTooltip:SetScript("OnTooltipSetUnit", function(self)
                eventCount = eventCount + 1
                eventName, eventUnit, eventGuid = self:GetUnit()
            end)

            local hasUnit = GameTooltip:SetUnit("party1")
            local name, unit, guid = GameTooltip:GetUnit()

            return table.concat({
                tostring(hasUnit),
                tostring(GameTooltip:NumLines()),
                tostring(name),
                tostring(unit),
                tostring(guid),
                tostring(eventCount),
                tostring(eventName),
                tostring(eventUnit),
                tostring(eventGuid),
            }, "|")
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "true|4|Thrynn|party1|Player-0000-00000002|1|Thrynn|party1|Player-0000-00000002",
        "SetUnit('party1') should populate unit lines, preserve displayed unit data, and fire OnTooltipSetUnit once"
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
    let expected_name: String = env.eval("return C_Spell.GetSpellName(19750)").unwrap();
    assert_eq!(td.lines[0].left_text, expected_name);
}

#[test]
fn test_set_spell_by_id_defaults_uncolored_lines_to_normal_font_color() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(19750)").unwrap();

    let expected: (f32, f32, f32) = env
        .eval("local r,g,b = NORMAL_FONT_COLOR:GetRGB(); return r,g,b")
        .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    let title = td
        .lines
        .first()
        .expect("spell tooltip should contain at least one line");

    assert!(
        (title.left_color.0 - expected.0).abs() < 0.01
            && (title.left_color.1 - expected.1).abs() < 0.01
            && (title.left_color.2 - expected.2).abs() < 0.01,
        "uncolored spell-tooltip lines should inherit NORMAL_FONT_COLOR; expected rgb=({:.3},{:.3},{:.3}), got rgb=({:.3},{:.3},{:.3})",
        expected.0,
        expected.1,
        expected.2,
        title.left_color.0,
        title.left_color.1,
        title.left_color.2
    );
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
fn test_set_spell_by_id_replaces_damage_placeholders_in_description() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(31935)").unwrap();
    let avengers_shield_damage: String = env
        .eval("local function fmt(v) if math.abs(v) >= 100 or math.abs(v - math.floor(v + 0.5)) < 0.001 then return tostring(math.floor(v + 0.5)) else return string.format('%.1f', v) end end local ap = UnitAttackPower('player'); return fmt(ap * 1.55)")
        .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();

    assert!(
        td.lines.iter().any(|line| line
            .left_text
            .contains(&format!("{avengers_shield_damage} Holy damage"))),
        "SetSpellByID should replace spell damage placeholders with AP-scaled values, got: {:?}",
        td.lines
            .iter()
            .map(|line| line.left_text.clone())
            .collect::<Vec<_>>()
    );
    assert!(
        td.lines.iter().all(|line| !line.left_text.contains('$')),
        "SetSpellByID should not leave raw parameter placeholders in tooltip text, got: {:?}",
        td.lines
            .iter()
            .map(|line| line.left_text.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_set_spell_by_id_replaces_shield_placeholders_from_player_health() {
    let env = WowLuaEnv::new().unwrap();

    env.state().borrow_mut().player.health_max = 120_000;
    env.exec("GameTooltip:SetSpellByID(184662)").unwrap();
    let shield_amount: i32 = env
        .eval("return math.floor(120000 * 0.30 * (1 + GetVersatilityBonus() / 100) + 0.5)")
        .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();

    assert!(
        td.lines.iter().any(|line| line
            .left_text
            .contains(&format!("absorbs {shield_amount} damage"))),
        "SetSpellByID should calculate shield values from player health, got: {:?}",
        td.lines
            .iter()
            .map(|line| line.left_text.clone())
            .collect::<Vec<_>>()
    );
    assert!(
        td.lines.iter().all(|line| !line.left_text.contains('$')),
        "SetSpellByID should not leave raw parameter placeholders in shield text, got: {:?}",
        td.lines
            .iter()
            .map(|line| line.left_text.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_set_spell_by_id_replaces_armor_placeholder_for_shield_of_the_righteous() {
    let env = WowLuaEnv::new().unwrap();
    let expected_armor: i32 = env
        .eval("local str = UnitStat('player', 1); return math.floor(str * 1.60 + 0.5)")
        .unwrap();

    env.exec("GameTooltip:SetSpellByID(53600)").unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();

    assert!(
        td.lines.iter().any(|line| line
            .left_text
            .contains(&format!("Armor by {expected_armor}"))),
        "SetSpellByID should calculate Shield of the Righteous armor from 160% primary stat, got: {:?}",
        td.lines
            .iter()
            .map(|line| line.left_text.clone())
            .collect::<Vec<_>>()
    );
    assert!(
        td.lines
            .iter()
            .any(|line| line.left_text.contains("for 4.5")),
        "SetSpellByID should resolve Shield of the Righteous armor duration from spell 132403, got: {:?}",
        td.lines
            .iter()
            .map(|line| line.left_text.clone())
            .collect::<Vec<_>>()
    );
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
