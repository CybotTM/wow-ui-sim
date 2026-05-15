use super::*;

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
fn test_set_bag_item_populates_tooltip() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            A_Admin.ClearBags()
            A_Admin.AddBagItem(0, 1, 6948, 1)
            GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
            local hasCooldown, repairCost = GameTooltip:SetBagItem(0, 1)
            if hasCooldown ~= false then
                return "hasCooldown=" .. tostring(hasCooldown)
            end
            if repairCost ~= nil then
                return "repairCost=" .. tostring(repairCost)
            end
            local tooltipLine = GameTooltip:GetLeftLine(1)
            local tooltipText = tooltipLine and tooltipLine:GetText()
            if tooltipText ~= "Hearthstone" then
                return "tooltipText=" .. tostring(tooltipText)
            end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "SetBagItem should populate bag item tooltips: {result}"
    );
}

#[test]
fn test_set_inventory_item_tooltip_content() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
        GameTooltip:SetInventoryItem("player", 1)
        "#,
    )
    .unwrap();

    let tooltip_id = {
        let state = env.state().borrow();
        state
            .widgets
            .get_id_by_name("GameTooltip")
            .expect("GameTooltip not found")
    };
    let state = env.state().borrow();
    let tooltip = state.tooltips.get(&tooltip_id).expect("No tooltip data");

    assert_eq!(tooltip.lines[0].left_text, "Entombed Seraph's Casque");
    let (red, _green, _blue) = tooltip.lines[0].left_color;
    assert!(
        red > 0.5,
        "Epic quality title should have purple/red color component"
    );
    assert!(
        tooltip.lines[1].left_text.contains("571"),
        "Second line should contain ilvl 571, got: {}",
        tooltip.lines[1].left_text
    );
    assert!(
        tooltip.lines.len() >= 3,
        "Should have at least 3 lines (name, ilvl, slot)"
    );
}
