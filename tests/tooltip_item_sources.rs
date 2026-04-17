use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn c_tooltip_info_item_source_aliases_delegate_to_existing_paths() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local baseline = C_TooltipInfo.GetItemByID(6948)
            local bagItem = C_TooltipInfo.GetBagItem(0, 1)
            local viaBagLocation = C_TooltipInfo.GetItem({ bagID = 0, slotIndex = 1 })
            local viaItemId = C_TooltipInfo.GetItem(6948)
            local viaItemLink = C_TooltipInfo.GetTooltipDataForItem("item:6948")

            local equipped = C_TooltipInfo.GetInventoryItem("player", 1)
            local viaEquipmentLocation = C_TooltipInfo.GetItem({ equipmentSlotIndex = 1 })

            local spellBaseline = C_TooltipInfo.GetSpellByID(19750)
            local spellByLink = C_TooltipInfo.GetSpell(GetSpellLink(19750))

            if bagItem.lines[1].leftText ~= baseline.lines[1].leftText then
                return "bag_item_should_match_item_by_id"
            end
            if viaBagLocation.lines[1].leftText ~= baseline.lines[1].leftText then
                return "item_location_should_match_bag_item"
            end
            if viaItemId.lines[1].leftText ~= baseline.lines[1].leftText then
                return "numeric_item_should_match_item_by_id"
            end
            if viaItemLink.lines[1].leftText ~= baseline.lines[1].leftText then
                return "tooltip_data_for_item_should_match_item_by_id"
            end
            if viaEquipmentLocation.lines[1].leftText ~= equipped.lines[1].leftText then
                return "equipment_location_should_match_inventory_item"
            end
            if spellByLink.lines[1].leftText ~= spellBaseline.lines[1].leftText then
                return "spell_alias_should_match_spell_by_id"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "Tooltip item/spell source aliases should reuse the existing item and spell tooltip paths"
    );
}
