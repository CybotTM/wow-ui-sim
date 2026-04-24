use wow_ui_sim::lua_api::WowLuaEnv;

fn update_tooltip_sizes(env: &WowLuaEnv) {
    use std::path::PathBuf;
    use wow_ui_sim::render::font::WowFontSystem;

    let mut font_sys = WowFontSystem::new(&PathBuf::from("./fonts"));
    let mut state = env.state().borrow_mut();
    wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut font_sys);
}

fn assert_color_close(actual: (f32, f32, f32), expected: (f32, f32, f32), label: &str) {
    assert!(
        (actual.0 - expected.0).abs() < 0.01
            && (actual.1 - expected.1).abs() < 0.01
            && (actual.2 - expected.2).abs() < 0.01,
        "{label} color mismatch; expected rgb=({:.3},{:.3},{:.3}), got rgb=({:.3},{:.3},{:.3})",
        expected.0,
        expected.1,
        expected.2,
        actual.0,
        actual.1,
        actual.2
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
fn test_set_spell_by_id_uses_wrapped_description_for_tooltip_width() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(375576)").unwrap();
    update_tooltip_sizes(&env);

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let frame = state.widgets.get(gt_id).unwrap();

    assert!(
        frame.width >= 240.0,
        "Divine Toll tooltip should not shrink to the short title width, got {}",
        frame.width
    );
}

#[test]
fn test_set_spell_by_id_colors_title_and_metadata_lines() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(19750)").unwrap();

    let highlight: (f32, f32, f32) = env
        .eval("local r,g,b = HIGHLIGHT_FONT_COLOR:GetRGB(); return r,g,b")
        .unwrap();
    let normal: (f32, f32, f32) = env
        .eval("local r,g,b = NORMAL_FONT_COLOR:GetRGB(); return r,g,b")
        .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    let title = &td.lines[0];
    let cost = td
        .lines
        .iter()
        .find(|line| line.left_text.contains("MANA"))
        .expect("Flash of Light tooltip should include resource cost");
    let cast = td
        .lines
        .iter()
        .find(|line| line.left_text.contains("sec cast"))
        .expect("Flash of Light tooltip should include cast time");
    let description = td
        .lines
        .last()
        .expect("Flash of Light tooltip should include a description");

    for line in [title, cost, cast] {
        assert_color_close(line.left_color, highlight, "spell metadata line");
    }
    assert_color_close(description.left_color, normal, "spell description line");
}

#[test]
fn test_set_spell_by_id_colors_cooldown_line() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(642)").unwrap();
    let highlight: (f32, f32, f32) = env
        .eval("local r,g,b = HIGHLIGHT_FONT_COLOR:GetRGB(); return r,g,b")
        .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    let cooldown = td
        .lines
        .iter()
        .find(|line| line.left_text.contains("cooldown"))
        .expect("Divine Shield tooltip should include cooldown line");

    assert_color_close(cooldown.left_color, highlight, "cooldown line");
}

#[test]
fn test_get_action_adds_colored_binding_line() {
    let env = WowLuaEnv::new().unwrap();
    let (binding_text, r, g, b): (String, f32, f32, f32) = env
        .eval(
            r#"
            local info = C_TooltipInfo.GetAction(5)
            for _, line in ipairs(info.lines) do
                if line.leftText and string.find(line.leftText, "Key bound") then
                    return line.leftText, line.leftColor:GetRGB()
                end
            end
            return "", 0, 0, 0
            "#,
        )
        .unwrap();
    let instruction: (f32, f32, f32) = env
        .eval("local r,g,b = GREEN_FONT_COLOR:GetRGB(); return r,g,b")
        .unwrap();

    assert_eq!(binding_text, "Key bound: 5");
    assert_color_close((r, g, b), instruction, "action binding line");
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
    let description_line = td
        .lines
        .iter()
        .find(|line| line.left_text.contains("Armor by"))
        .expect("Shield of the Righteous tooltip should include armor description");
    assert!(
        description_line.left_segments.is_empty(),
        "resolved numeric values should not invent inline color segments"
    );
}

#[test]
fn test_set_spell_by_id_get_left_line_uses_tooltip_line_color() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(53600)").unwrap();

    let (line_index, expected_color) = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        let td = state.tooltips.get(&gt_id).unwrap();
        let description_index = td
            .lines
            .iter()
            .position(|line| line.left_text.contains("Armor by"))
            .expect("Shield of the Righteous tooltip should include armor description")
            + 1;
        (
            description_index,
            td.lines[description_index - 1].left_color,
        )
    };
    let actual_color: (f32, f32, f32) = env
        .eval(&format!(
            "local line = GameTooltip:GetLeftLine({line_index}); return line:GetTextColor()"
        ))
        .unwrap();

    assert!(
        (actual_color.0 - expected_color.0).abs() < 0.01
            && (actual_color.1 - expected_color.1).abs() < 0.01
            && (actual_color.2 - expected_color.2).abs() < 0.01,
        "GetLeftLine should apply tooltip line color; expected rgb=({:.3},{:.3},{:.3}), got rgb=({:.3},{:.3},{:.3})",
        expected_color.0,
        expected_color.1,
        expected_color.2,
        actual_color.0,
        actual_color.1,
        actual_color.2
    );
}

#[test]
fn test_set_spell_by_id_get_left_line_does_not_invent_value_color_segments() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(53600)").unwrap();

    let line_index = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        let td = state.tooltips.get(&gt_id).unwrap();
        let line_index = td
            .lines
            .iter()
            .position(|line| line.left_text.contains("Armor by"))
            .expect("Shield of the Righteous tooltip should include armor description");
        line_index + 1
    };
    env.exec(&format!("GameTooltip:GetLeftLine({line_index})"))
        .unwrap();
    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    let left_line_id = td.left_line_ids[line_index - 1];
    let line_frame = state.widgets.get(left_line_id).unwrap();
    assert!(
        line_frame.text_segments.is_empty(),
        "rendered numeric values should not invent inline color segments"
    );
}

#[test]
fn test_add_line_does_not_invent_processing_info_value_color_segments() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local info = { tooltipData = C_TooltipInfo.GetAction(5) }
        GameTooltip.processingInfo = info
        GameTooltip:ClearLines()
        for _, lineData in ipairs(info.tooltipData.lines) do
            local color = lineData.leftColor or NORMAL_FONT_COLOR
            local r, g, b = color:GetRGB()
            GameTooltip:AddLine(lineData.leftText, r, g, b, lineData.wrapText)
        end
        GameTooltip.processingInfo = nil
        "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    let description_line = td
        .lines
        .iter()
        .find(|line| line.left_text.contains("Armor by"))
        .expect("action tooltip should include Shield of the Righteous description");
    assert!(
        description_line.left_segments.is_empty(),
        "data-handler tooltip should not invent color segments for resolved numeric values"
    );
}

#[test]
fn test_add_line_preserves_explicit_processing_info_inline_color_segments() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local info = {
            tooltipData = {
                lines = {
                    {
                        leftText = "Requires Clearcasting",
                        leftColor = NORMAL_FONT_COLOR,
                        leftColorSegments = {
                            { text = "Requires ", color = NORMAL_FONT_COLOR },
                            { text = "Clearcasting", color = HIGHLIGHT_FONT_COLOR },
                        },
                    },
                },
            },
        }
        GameTooltip.processingInfo = info
        GameTooltip:ClearLines()
        local lineData = info.tooltipData.lines[1]
        local r, g, b = lineData.leftColor:GetRGB()
        GameTooltip:AddLine(lineData.leftText, r, g, b, lineData.wrapText)
        GameTooltip.processingInfo = nil
        "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    let line = td.lines.first().unwrap();
    let highlighted = line
        .left_segments
        .iter()
        .find(|segment| segment.text == "Clearcasting")
        .expect("AddLine should preserve explicit processingInfo color segments");

    assert!(
        (highlighted.color.0 - 1.0).abs() < 0.01
            && (highlighted.color.1 - 1.0).abs() < 0.01
            && (highlighted.color.2 - 1.0).abs() < 0.01,
        "explicit highlighted segment should stay white, got rgb=({:.3},{:.3},{:.3})",
        highlighted.color.0,
        highlighted.color.1,
        highlighted.color.2
    );
}

#[test]
fn test_set_spell_by_id_applies_full_line_inline_color_markup() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(5143)").unwrap();

    let (description_index, description_color, description_text) = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        let td = state.tooltips.get(&gt_id).unwrap();
        let index = td
            .lines
            .iter()
            .position(|line| line.left_text.contains("Requires Clearcasting"))
            .expect("Arcane Missiles tooltip should include its colored requirement")
            + 1;
        let line = &td.lines[index - 1];
        (index, line.left_color, line.left_text.clone())
    };
    let line_frame_color: (f32, f32, f32) = env
        .eval(&format!(
            "local line = GameTooltip:GetLeftLine({description_index}); return line:GetTextColor()"
        ))
        .unwrap();

    assert_eq!(description_text, "Requires Clearcasting");
    assert!(
        (description_color.0 - 1.0).abs() < 0.01
            && (description_color.1 - 1.0).abs() < 0.01
            && (description_color.2 - 1.0).abs() < 0.01,
        "full-line |cFFFFFFFF markup should make tooltip data white, got rgb=({:.3},{:.3},{:.3})",
        description_color.0,
        description_color.1,
        description_color.2
    );
    assert!(
        (line_frame_color.0 - 1.0).abs() < 0.01
            && (line_frame_color.1 - 1.0).abs() < 0.01
            && (line_frame_color.2 - 1.0).abs() < 0.01,
        "full-line |cFFFFFFFF markup should make tooltip FontString white, got rgb=({:.3},{:.3},{:.3})",
        line_frame_color.0,
        line_frame_color.1,
        line_frame_color.2
    );
}

#[test]
fn test_set_spell_by_id_applies_named_inline_color_markup() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(448287)").unwrap();

    let (description_color, description_text) = {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        let td = state.tooltips.get(&gt_id).unwrap();
        let line = td
            .lines
            .iter()
            .find(|line| line.left_text.contains("Season 1"))
            .expect("tooltip should include the named-color description line");
        (line.left_color, line.left_text.clone())
    };
    let expected: (f32, f32, f32) = env
        .eval("local r,g,b = GREEN_FONT_COLOR:GetRGB(); return r,g,b")
        .unwrap();

    assert_eq!(description_text, "Season 1");
    assert!(
        (description_color.0 - expected.0).abs() < 0.01
            && (description_color.1 - expected.1).abs() < 0.01
            && (description_color.2 - expected.2).abs() < 0.01,
        "named |cnGREEN_FONT_COLOR markup should make tooltip data green; expected rgb=({:.3},{:.3},{:.3}), got rgb=({:.3},{:.3},{:.3})",
        expected.0,
        expected.1,
        expected.2,
        description_color.0,
        description_color.1,
        description_color.2
    );
}

#[test]
fn test_set_spell_by_id_preserves_partial_inline_color_segments() {
    let env = WowLuaEnv::new().unwrap();

    env.exec("GameTooltip:SetSpellByID(1223268)").unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state.tooltips.get(&gt_id).unwrap();
    let line = td
        .lines
        .iter()
        .find(|line| line.left_text.contains("Scale of the Earth-Warder"))
        .expect("tooltip should include the partially colored description line");
    let highlighted = line
        .left_segments
        .iter()
        .find(|segment| segment.text == "Scale of the Earth-Warder")
        .expect("partial inline color markup should create a segment for the highlighted text");

    assert!(
        line.left_segments.len() > 1,
        "partial inline color markup should split the line into colored runs, got {}",
        line.left_segments.len()
    );
    assert!(
        (highlighted.color.0 - 1.0).abs() < 0.01
            && (highlighted.color.1 - 0.8).abs() < 0.01
            && (highlighted.color.2 - 0.6).abs() < 0.01,
        "highlighted segment should use |cFFFFCC99 color, got rgb=({:.3},{:.3},{:.3})",
        highlighted.color.0,
        highlighted.color.1,
        highlighted.color.2
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
