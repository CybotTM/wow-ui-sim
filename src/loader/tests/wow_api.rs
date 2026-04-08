//! Tests for WoW API coverage — verifying that the simulator implements
//! the globals, enums, and C_* namespaces that addons depend on.

use super::*;

// ---------------------------------------------------------------------------
// Enum.BagIndex
// ---------------------------------------------------------------------------

#[test]
fn test_enum_bag_index_backpack_slots() {
    let env = WowLuaEnv::new().unwrap();
    let v: i32 = env.eval("return Enum.BagIndex.Backpack").unwrap();
    assert_eq!(v, 0, "Backpack should be 0");

    let _: i32 = env.eval("return Enum.BagIndex.Bag_1").unwrap();
    let _: i32 = env.eval("return Enum.BagIndex.Bag_2").unwrap();
    let _: i32 = env.eval("return Enum.BagIndex.Bag_3").unwrap();
    let _: i32 = env.eval("return Enum.BagIndex.Bag_4").unwrap();
    let _: i32 = env.eval("return Enum.BagIndex.ReagentBag").unwrap();
}

#[test]
fn test_enum_bag_index_retail_bank_slots() {
    let env = WowLuaEnv::new().unwrap();
    let _: i32 = env.eval("return Enum.BagIndex.Characterbanktab").unwrap();
    let _: i32 = env.eval("return Enum.BagIndex.CharacterBankTab_1").unwrap();
    let _: i32 = env.eval("return Enum.BagIndex.CharacterBankTab_6").unwrap();
    let _: i32 = env.eval("return Enum.BagIndex.AccountBankTab_1").unwrap();
    let _: i32 = env.eval("return Enum.BagIndex.AccountBankTab_5").unwrap();
}

// Bank, BankBag_1–7, Reagentbank: Classic-only enum values, not in retail BagIndex.
// The sim runs as retail (WOW_PROJECT_MAINLINE). BetterBags only uses these
// in its `else` (non-retail) code path.

// ---------------------------------------------------------------------------
// Enum.ItemQuality
// ---------------------------------------------------------------------------

#[test]
fn test_enum_item_quality_values_and_ordering() {
    let env = WowLuaEnv::new().unwrap();
    let poor: i32 = env.eval("return Enum.ItemQuality.Poor").unwrap();
    let common: i32 = env.eval("return Enum.ItemQuality.Common").unwrap();
    let uncommon: i32 = env.eval("return Enum.ItemQuality.Uncommon").unwrap();
    let rare: i32 = env.eval("return Enum.ItemQuality.Rare").unwrap();
    let epic: i32 = env.eval("return Enum.ItemQuality.Epic").unwrap();
    let legendary: i32 = env.eval("return Enum.ItemQuality.Legendary").unwrap();
    let _: i32 = env.eval("return Enum.ItemQuality.Artifact").unwrap();
    let _: i32 = env.eval("return Enum.ItemQuality.Heirloom").unwrap();
    let _: i32 = env.eval("return Enum.ItemQuality.WoWToken").unwrap();

    assert!(poor < common);
    assert!(common < uncommon);
    assert!(uncommon < rare);
    assert!(rare < epic);
    assert!(epic < legendary);
}

// ---------------------------------------------------------------------------
// Enum.ItemClass
// ---------------------------------------------------------------------------

#[test]
fn test_enum_item_class_values() {
    let env = WowLuaEnv::new().unwrap();
    let _: i32 = env.eval("return Enum.ItemClass.Consumable").unwrap();
    let _: i32 = env.eval("return Enum.ItemClass.Container").unwrap();
    let _: i32 = env.eval("return Enum.ItemClass.Weapon").unwrap();
    let _: i32 = env.eval("return Enum.ItemClass.Armor").unwrap();
    let _: i32 = env.eval("return Enum.ItemClass.Reagent").unwrap();
    let _: i32 = env.eval("return Enum.ItemClass.Tradegoods").unwrap();
    let _: i32 = env.eval("return Enum.ItemClass.Questitem").unwrap();
    let _: i32 = env.eval("return Enum.ItemClass.Miscellaneous").unwrap();
}

// ---------------------------------------------------------------------------
// Enum.InventoryType
// ---------------------------------------------------------------------------

#[test]
fn test_enum_inventory_type_values() {
    let env = WowLuaEnv::new().unwrap();
    let _: i32 = env.eval("return Enum.InventoryType.IndexHeadType").unwrap();
    let _: i32 = env.eval("return Enum.InventoryType.IndexNeckType").unwrap();
    let _: i32 = env
        .eval("return Enum.InventoryType.IndexShoulderType")
        .unwrap();
    let _: i32 = env
        .eval("return Enum.InventoryType.IndexChestType")
        .unwrap();
    let _: i32 = env
        .eval("return Enum.InventoryType.IndexWeaponType")
        .unwrap();
    let _: i32 = env
        .eval("return Enum.InventoryType.IndexShieldType")
        .unwrap();
    let _: i32 = env
        .eval("return Enum.InventoryType.Index2HweaponType")
        .unwrap();
}

// ---------------------------------------------------------------------------
// Enum.BankType
// ---------------------------------------------------------------------------

#[test]
fn test_enum_bank_type_values() {
    let env = WowLuaEnv::new().unwrap();
    let _: i32 = env.eval("return Enum.BankType.Character").unwrap();
    let _: i32 = env.eval("return Enum.BankType.Account").unwrap();
}

// ---------------------------------------------------------------------------
// INVSLOT constants
// ---------------------------------------------------------------------------

#[test]
fn test_invslot_constants() {
    let env = WowLuaEnv::new().unwrap();
    let head: i32 = env.eval("return INVSLOT_HEAD").unwrap();
    assert!(head > 0);
    let _: i32 = env.eval("return INVSLOT_NECK").unwrap();
    let _: i32 = env.eval("return INVSLOT_SHOULDER").unwrap();
    let _: i32 = env.eval("return INVSLOT_CHEST").unwrap();
    let _: i32 = env.eval("return INVSLOT_MAINHAND").unwrap();
    let _: i32 = env.eval("return INVSLOT_OFFHAND").unwrap();
    let _: i32 = env.eval("return INVSLOT_BACK").unwrap();
    let _: i32 = env.eval("return INVSLOT_TABARD").unwrap();
    let _: i32 = env.eval("return INVSLOT_TRINKET1").unwrap();
    let _: i32 = env.eval("return INVSLOT_TRINKET2").unwrap();
    let _: i32 = env.eval("return INVSLOT_FINGER1").unwrap();
    let _: i32 = env.eval("return INVSLOT_FINGER2").unwrap();
}

// ---------------------------------------------------------------------------
// Color globals
// ---------------------------------------------------------------------------

#[test]
fn test_color_globals_expose_expected_rgb_values() {
    let env = WowLuaEnv::new().unwrap();

    let faction_ok: bool = env
        .eval(
            r#"
            local r, g, b = PLAYER_FACTION_COLOR_HORDE:GetRGB()
            return math.abs(r - 0.90196) < 0.0001
                and math.abs(g - 0.05098) < 0.0001
                and math.abs(b - 0.07059) < 0.0001
            "#,
        )
        .unwrap();
    assert!(faction_ok);

    let raid_ok: bool = env
        .eval(
            r#"
            local r, g, b = RAID_CLASS_COLORS.WARRIOR:GetRGB()
            return math.abs(r - 0.78) < 0.0001
                and math.abs(g - 0.61) < 0.0001
                and math.abs(b - 0.43) < 0.0001
            "#,
        )
        .unwrap();
    assert!(raid_ok);
}

// ---------------------------------------------------------------------------
// Item quality description globals
// ---------------------------------------------------------------------------

#[test]
fn test_item_quality_desc_globals() {
    let env = WowLuaEnv::new().unwrap();
    for i in 0..=8 {
        let expr = format!("return type(ITEM_QUALITY{}_DESC)", i);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "string", "ITEM_QUALITY{}_DESC should be string", i);
    }
}

// ---------------------------------------------------------------------------
// Expansion globals
// ---------------------------------------------------------------------------

#[test]
fn test_le_expansion_constants() {
    let env = WowLuaEnv::new().unwrap();
    let _: i32 = env.eval("return LE_EXPANSION_CLASSIC").unwrap();
    let _: i32 = env.eval("return LE_EXPANSION_BURNING_CRUSADE").unwrap();
    let _: i32 = env
        .eval("return LE_EXPANSION_WRATH_OF_THE_LICH_KING")
        .unwrap();
    let _: i32 = env.eval("return LE_EXPANSION_CATACLYSM").unwrap();
    let _: i32 = env.eval("return LE_EXPANSION_MISTS_OF_PANDARIA").unwrap();
    let _: i32 = env.eval("return LE_EXPANSION_WARLORDS_OF_DRAENOR").unwrap();
    let _: i32 = env.eval("return LE_EXPANSION_LEGION").unwrap();
    let _: i32 = env.eval("return LE_EXPANSION_BATTLE_FOR_AZEROTH").unwrap();
    let _: i32 = env.eval("return LE_EXPANSION_SHADOWLANDS").unwrap();
    let _: i32 = env.eval("return LE_EXPANSION_DRAGONFLIGHT").unwrap();
    let _: i32 = env.eval("return LE_EXPANSION_WAR_WITHIN").unwrap();
}

#[test]
fn test_expansion_name_globals() {
    let env = WowLuaEnv::new().unwrap();
    for i in 0..=10 {
        let expr = format!("return type(EXPANSION_NAME{})", i);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "string", "EXPANSION_NAME{} should be string", i);
    }
}

#[test]
fn test_character_select_map_scene_highlight_globals_exist() {
    let env = WowLuaEnv::new().unwrap();

    let start_ty: String = env
        .eval("return type(MapSceneCharacterHighlightStart)")
        .unwrap();
    assert_eq!(start_ty, "function");

    let end_ty: String = env
        .eval("return type(MapSceneCharacterHighlightEnd)")
        .unwrap();
    assert_eq!(end_ty, "function");

    env.exec(
        r#"
        MapSceneCharacterHighlightStart("Player-1-00000001")
        MapSceneCharacterHighlightEnd("Player-1-00000001")
        "#,
    )
    .unwrap();
}

// WOW_PROJECT_ID, WOW_PROJECT_MAINLINE: from Blizzard_SharedXML/ProjectConstants.lua
// Tested via Lua addon tests (run-tests), not available in bare WowLuaEnv.

// ---------------------------------------------------------------------------
// C_Container API
// ---------------------------------------------------------------------------

#[test]
fn test_c_container_get_num_slots() {
    let env = WowLuaEnv::new().unwrap();
    let slots: i32 = env
        .eval("return C_Container.GetContainerNumSlots(0)")
        .unwrap();
    assert!(slots >= 0);
}

#[test]
fn test_c_container_get_num_free_slots() {
    let env = WowLuaEnv::new().unwrap();
    let free: i32 = env
        .eval("return C_Container.GetContainerNumFreeSlots(0)")
        .unwrap();
    assert!(free >= 0);
}

#[test]
fn test_c_container_functions_exist() {
    let env = WowLuaEnv::new().unwrap();
    for f in &[
        "ContainerIDToInventoryID",
        "GetBagName",
        "GetContainerItemPurchaseInfo",
        "IsBattlePayItem",
        "UseContainerItem",
        "PickupContainerItem",
        "SplitContainerItem",
        "GetContainerItemInfo",
        "GetContainerItemQuestInfo",
        "GetContainerItemID",
        "GetContainerItemLink",
    ] {
        let expr = format!("return type(C_Container.{})", f);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function", "C_Container.{} should be function", f);
    }
}

#[test]
fn test_c_container_empty_slot_returns_nil() {
    let env = WowLuaEnv::new().unwrap();
    let is_nil: bool = env
        .eval("return C_Container.GetContainerItemInfo(0, 999) == nil")
        .unwrap();
    assert!(is_nil);
}

// ---------------------------------------------------------------------------
// C_Item API
// ---------------------------------------------------------------------------

#[test]
fn test_c_item_get_sub_class_info() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval("return C_Item.GetItemSubClassInfo(Enum.ItemClass.Tradegoods, 4)")
        .unwrap();
    assert!(!name.is_empty());
}

#[test]
fn test_c_item_functions_exist() {
    let env = WowLuaEnv::new().unwrap();
    for f in &[
        "GetItemIconByID",
        "GetDetailedItemLevelInfo",
        "GetItemInfoInstant",
        "GetItemInfo",
    ] {
        let expr = format!("return type(C_Item.{})", f);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function", "C_Item.{} should be function", f);
    }
}

#[test]
fn test_c_item_get_item_info_returns_multi_value() {
    let env = WowLuaEnv::new().unwrap();
    // GetItemInfo returns 17 values: itemName, itemLink, itemQuality, itemLevel,
    // itemMinLevel, itemType, itemSubType, itemStackCount, itemEquipLoc,
    // itemTexture, sellPrice, classID, subclassID, bindType, expacID, setID, isCraftingReagent
    // Item 6948 = Hearthstone (quality 1, Common)
    let name: String = env
        .eval("local n = C_Item.GetItemInfo(6948); return n")
        .unwrap();
    assert_eq!(name, "Hearthstone");

    let quality: i32 = env
        .eval("local _,_,q = C_Item.GetItemInfo(6948); return q")
        .unwrap();
    assert_eq!(quality, 1, "Hearthstone quality should be 1 (Common)");

    // select(14, ...) is bindType — used by BetterBags
    let bind_type: i32 = env
        .eval("return select(14, C_Item.GetItemInfo(6948))")
        .unwrap();
    assert!(bind_type >= 0, "bindType should be a valid number");
}

// ---------------------------------------------------------------------------
// C_NewItems API
// ---------------------------------------------------------------------------

#[test]
fn test_c_new_items_api() {
    let env = WowLuaEnv::new().unwrap();
    let result: bool = env.eval("return C_NewItems.IsNewItem(0, 1)").unwrap();
    assert!(!result);

    for f in &["RemoveNewItem", "ClearAll"] {
        let expr = format!("return type(C_NewItems.{})", f);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function");
    }
}

// ---------------------------------------------------------------------------
// C_CurrencyInfo API
// ---------------------------------------------------------------------------

#[test]
fn test_c_currency_info_api() {
    let env = WowLuaEnv::new().unwrap();
    let size: i32 = env
        .eval("return C_CurrencyInfo.GetCurrencyListSize()")
        .unwrap();
    assert!(size >= 0);

    for f in &["GetCurrencyListInfo", "GetCoinTextureString"] {
        let expr = format!("return type(C_CurrencyInfo.{})", f);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function");
    }
}

// ---------------------------------------------------------------------------
// C_EquipmentSet API
// ---------------------------------------------------------------------------

#[test]
fn test_c_equipment_set_api() {
    let env = WowLuaEnv::new().unwrap();
    let ids_ty: String = env
        .eval("return type(C_EquipmentSet.GetEquipmentSetIDs())")
        .unwrap();
    assert_eq!(ids_ty, "table");

    let fn_ty: String = env
        .eval("return type(C_EquipmentSet.GetEquipmentSetInfo)")
        .unwrap();
    assert_eq!(fn_ty, "function");
}

// ---------------------------------------------------------------------------
// C_Bank API
// ---------------------------------------------------------------------------

#[test]
fn test_c_bank_api() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(C_Bank)").unwrap();
    assert_eq!(ty, "table");
    let fn_ty: String = env.eval("return type(C_Bank.FetchDepositedMoney)").unwrap();
    assert_eq!(fn_ty, "function");
}

// ---------------------------------------------------------------------------
// C_Timer API
// ---------------------------------------------------------------------------

#[test]
fn test_c_timer_functions_exist() {
    let env = WowLuaEnv::new().unwrap();
    let after_ty: String = env.eval("return type(C_Timer.After)").unwrap();
    assert_eq!(after_ty, "function");
    let new_ty: String = env.eval("return type(C_Timer.NewTimer)").unwrap();
    assert_eq!(new_ty, "function");
}

// ---------------------------------------------------------------------------
// C_CVar, C_AddOns, C_TooltipInfo
// ---------------------------------------------------------------------------

#[test]
fn test_c_cvar_set_cvar_exists() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(C_CVar.SetCVar)").unwrap();
    assert_eq!(ty, "function");
}

#[test]
fn test_c_addons_api() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(C_AddOns)").unwrap();
    assert_eq!(ty, "table");
    let fn_ty: String = env.eval("return type(C_AddOns.IsAddOnLoaded)").unwrap();
    assert_eq!(fn_ty, "function");
}

#[test]
fn test_c_tooltip_info_exists() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(C_TooltipInfo)").unwrap();
    assert_eq!(ty, "table");
}

fn flash_of_light_aura() -> crate::lua_api::game_data::AuraInfo {
    crate::lua_api::game_data::AuraInfo {
        name: "Flash of Light".to_string(),
        spell_id: 19750,
        icon: 135987,
        duration: 3600.0,
        expiration_time: 3600.0,
        applications: 0,
        source_unit: "player".to_string(),
        is_helpful: true,
        is_stealable: false,
        can_apply_aura: true,
        is_from_player_or_player_pet: true,
        aura_instance_id: 1,
    }
}

fn lady_liadrin_target() -> crate::lua_api::game_data::TargetInfo {
    crate::lua_api::game_data::TargetInfo {
        unit_id: "target".to_string(),
        name: "Lady Liadrin".to_string(),
        class_index: 2,
        level: 80,
        health: 100_000,
        health_max: 100_000,
        power: 50_000,
        power_max: 100_000,
        power_type: 0,
        power_type_name: "MANA".to_string(),
        is_player: true,
        is_enemy: false,
        guid: "Player-0000-0000BEEF".to_string(),
        classification: "normal".to_string(),
        creature_type: "Blood Elf".to_string(),
        reaction: 5,
    }
}

fn seed_c_tooltip_info_test_state(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.player.buffs = vec![flash_of_light_aura()];
    state.current_target = Some(lady_liadrin_target());
}

#[test]
fn test_c_tooltip_info_shapes_match_tooltip_data_handler_expectations() {
    let env = WowLuaEnv::new().unwrap();
    seed_c_tooltip_info_test_state(&env);
    let all_shapes_match: bool = env
        .eval(
            r#"
            local function colorShapeOk(color)
                return color == nil or (type(color) == "table" and type(color.GetRGB) == "function")
            end

            local function lineShapeOk(line)
                return type(line) == "table"
                    and type(line.type) == "number"
                    and (line.leftText == nil or type(line.leftText) == "string")
                    and colorShapeOk(line.leftColor)
                    and (line.wrapText == nil or type(line.wrapText) == "boolean")
                    and (line.rightText == nil or type(line.rightText) == "string")
                    and colorShapeOk(line.rightColor)
                    and (line.leftOffset == nil or type(line.leftOffset) == "number")
            end

            local function tooltipShapeOk(tooltip, expectedType, allowEmpty)
                if type(tooltip) ~= "table" or tooltip.type ~= expectedType or type(tooltip.lines) ~= "table" then
                    return false
                end

                local lineCount = 0
                for index, line in ipairs(tooltip.lines) do
                    lineCount = index
                    if not lineShapeOk(line) then
                        return false
                    end
                end

                return allowEmpty and lineCount == 0 or lineCount > 0
            end

            local nodeIDs = C_Traits.GetTreeNodes(790)
            local entryID
            for _, nodeID in ipairs(nodeIDs) do
                local nodeInfo = C_Traits.GetNodeInfo(1, nodeID)
                if nodeInfo and nodeInfo.entryIDs and nodeInfo.entryIDs[1] then
                    entryID = nodeInfo.entryIDs[1]
                    break
                end
            end

            if not entryID then
                return false
            end

            local checks = {
                tooltipShapeOk(C_TooltipInfo.GetTraitEntry(entryID, 1), Enum.TooltipDataType.Spell, false),
                tooltipShapeOk(C_TooltipInfo.GetAction(1), Enum.TooltipDataType.Spell, false),
                tooltipShapeOk(C_TooltipInfo.GetItemByID(211992), Enum.TooltipDataType.Item, false),
                tooltipShapeOk(C_TooltipInfo.GetInventoryItem("player", 1), Enum.TooltipDataType.Item, false),
                tooltipShapeOk(C_TooltipInfo.GetSpellByID(19750), Enum.TooltipDataType.Spell, false),
                tooltipShapeOk(C_TooltipInfo.GetUnitBuff("player", 1, "HELPFUL"), Enum.TooltipDataType.UnitAura, false),
                tooltipShapeOk(C_TooltipInfo.GetUnitDebuff("player", 1, "HARMFUL"), Enum.TooltipDataType.UnitAura, true),
                tooltipShapeOk(C_TooltipInfo.GetUnitAura("player", 1, "HELPFUL"), Enum.TooltipDataType.UnitAura, false),
                tooltipShapeOk(C_TooltipInfo.GetHyperlink("|cff0070dd|Hitem:211992:0:0:0:0:0:0:0:0:0|h[Entombed Seraph's Greaves]|h|r"), Enum.TooltipDataType.Item, false),
                tooltipShapeOk(C_TooltipInfo.GetHyperlink(GetSpellLink(19750)), Enum.TooltipDataType.Spell, false),
                tooltipShapeOk(C_TooltipInfo.GetUnit("player"), Enum.TooltipDataType.Unit, false),
            }

            for _, check in ipairs(checks) do
                if not check then
                    return false
                end
            end

            return true
            "#,
        )
        .unwrap();
    assert!(
        all_shapes_match,
        "every implemented C_TooltipInfo getter should return tooltip data tables that TooltipDataHandlerMixin can consume",
    );
}

#[test]
fn test_c_traits_get_node_info_exposes_non_nil_rank_delta_fields() {
    let env = WowLuaEnv::new().unwrap();
    let all_nodes_have_fields: bool = env
        .eval(
            r#"
            local nodeIDs = C_Traits.GetTreeNodes(790)
            for _, nodeID in ipairs(nodeIDs) do
                local info = C_Traits.GetNodeInfo(1, nodeID)
                if not info
                    or info.ranksIncreased == nil
                    or info.entryIDToRanksIncreased == nil
                    or info.totalMaxRanks == nil
                then
                    return false
                end
            end

            local missingNode = C_Traits.GetNodeInfo(1, -1)
            return missingNode
                and missingNode.ranksIncreased ~= nil
                and missingNode.entryIDToRanksIncreased ~= nil
                and missingNode.totalMaxRanks ~= nil
            "#,
        )
        .unwrap();
    assert!(
        all_nodes_have_fields,
        "C_Traits.GetNodeInfo should always provide non-nil rank delta fields",
    );
}

#[test]
fn test_c_tooltip_info_get_trait_entry_returns_real_tooltip_lines() {
    let env = WowLuaEnv::new().unwrap();
    let has_real_tooltip: bool = env
        .eval(
            r#"
            local nodeIDs = C_Traits.GetTreeNodes(790)
            local entryID
            for _, nodeID in ipairs(nodeIDs) do
                local nodeInfo = C_Traits.GetNodeInfo(1, nodeID)
                if nodeInfo and nodeInfo.entryIDs and nodeInfo.entryIDs[1] then
                    entryID = nodeInfo.entryIDs[1]
                    break
                end
            end

            if not entryID then
                return false
            end

            local tooltip = C_TooltipInfo.GetTraitEntry(entryID, 1)
            if not tooltip or tooltip.type ~= Enum.TooltipDataType.Spell or not tooltip.lines then
                return false
            end

            local firstLine = tooltip.lines[1]
            return firstLine
                and firstLine.type == Enum.TooltipDataLineType.SpellName
                and type(firstLine.leftText) == "string"
                and firstLine.leftText ~= ""
            "#,
        )
        .unwrap();
    assert!(
        has_real_tooltip,
        "C_TooltipInfo.GetTraitEntry should expose at least a real spell-name line",
    );
}

#[test]
fn test_c_tooltip_info_get_item_by_id_exposes_colored_name_and_binding_line() {
    let env = WowLuaEnv::new().unwrap();
    let has_full_item_tooltip: bool = env
        .eval(
            r#"
            local tooltip = C_TooltipInfo.GetItemByID(229181)
            if not tooltip or tooltip.type ~= Enum.TooltipDataType.Item or not tooltip.lines then
                return false
            end

            local nameLine = tooltip.lines[1]
            if not nameLine
                or nameLine.type ~= Enum.TooltipDataLineType.ItemName
                or nameLine.leftText ~= "Ordained Forge Maul"
                or type(nameLine.leftColor) ~= "table"
            then
                return false
            end

            local r, g, b = nameLine.leftColor:GetRGB()
            if math.abs(r - 0.64) > 0.01 or math.abs(g - 0.21) > 0.01 or math.abs(b - 0.93) > 0.01 then
                return false
            end

            local bindingLine = tooltip.lines[4]
            return bindingLine
                and bindingLine.type == Enum.TooltipDataLineType.ItemBinding
                and bindingLine.leftText == ITEM_BIND_ON_PICKUP
            "#,
        )
        .unwrap();
    assert!(
        has_full_item_tooltip,
        "C_TooltipInfo.GetItemByID should expose item name color and binding text",
    );
}

#[test]
fn test_c_tooltip_info_get_item_by_id_returns_item_tooltip_lines() {
    let env = WowLuaEnv::new().unwrap();
    let has_real_tooltip: bool = env
        .eval(
            r#"
            local tooltip = C_TooltipInfo.GetItemByID(211992)
            if not tooltip or tooltip.type ~= Enum.TooltipDataType.Item or not tooltip.lines then
                return false
            end

            local nameLine = tooltip.lines[1]
            local itemLevelLine = tooltip.lines[2]
            local equipSlotLine = tooltip.lines[3]

            return nameLine
                and nameLine.type == Enum.TooltipDataLineType.ItemName
                and nameLine.leftText == "Entombed Seraph's Greaves"
                and itemLevelLine
                and itemLevelLine.type == Enum.TooltipDataLineType.ItemLevel
                and itemLevelLine.leftText == "Item Level 571"
                and equipSlotLine
                and equipSlotLine.type == Enum.TooltipDataLineType.EquipSlot
                and equipSlotLine.leftText == "Legs"
            "#,
        )
        .unwrap();
    assert!(
        has_real_tooltip,
        "C_TooltipInfo.GetItemByID should expose item, item-level, and equip-slot lines",
    );
}

#[test]
fn test_c_tooltip_info_get_inventory_item_returns_equipped_item_tooltip() {
    let env = WowLuaEnv::new().unwrap();
    let has_real_tooltip: bool = env
        .eval(
            r#"
            local tooltip = C_TooltipInfo.GetInventoryItem("player", 1)
            if not tooltip or tooltip.type ~= Enum.TooltipDataType.Item or not tooltip.lines then
                return false
            end

            local nameLine = tooltip.lines[1]
            local itemLevelLine = tooltip.lines[2]
            local equipSlotLine = tooltip.lines[3]

            return nameLine
                and nameLine.type == Enum.TooltipDataLineType.ItemName
                and nameLine.leftText == "Entombed Seraph's Casque"
                and itemLevelLine
                and itemLevelLine.type == Enum.TooltipDataLineType.ItemLevel
                and itemLevelLine.leftText == "Item Level 571"
                and equipSlotLine
                and equipSlotLine.type == Enum.TooltipDataLineType.EquipSlot
                and equipSlotLine.leftText == "Head"
            "#,
        )
        .unwrap();
    assert!(
        has_real_tooltip,
        "C_TooltipInfo.GetInventoryItem should expose equipped item tooltip lines",
    );
}

#[test]
fn test_c_tooltip_info_get_hyperlink_returns_item_and_spell_tooltips() {
    let env = WowLuaEnv::new().unwrap();
    let has_real_tooltip: bool = env
        .eval(
            r#"
            local itemTooltip = C_TooltipInfo.GetHyperlink("|cff0070dd|Hitem:211992:0:0:0:0:0:0:0:0:0|h[Entombed Seraph's Greaves]|h|r")
            local spellTooltip = C_TooltipInfo.GetHyperlink(GetSpellLink(19750))
            if not itemTooltip or not spellTooltip then
                return false
            end

            local itemNameLine = itemTooltip.lines[1]
            local itemLevelLine = itemTooltip.lines[2]
            local spellNameLine = spellTooltip.lines[1]
            local spellCostLine = spellTooltip.lines[2]
            local spellCastLine = spellTooltip.lines[3]

            return itemTooltip.type == Enum.TooltipDataType.Item
                and spellTooltip.type == Enum.TooltipDataType.Spell
                and itemNameLine
                and itemNameLine.type == Enum.TooltipDataLineType.ItemName
                and itemNameLine.leftText == "Entombed Seraph's Greaves"
                and itemLevelLine
                and itemLevelLine.type == Enum.TooltipDataLineType.ItemLevel
                and itemLevelLine.leftText == "Item Level 571"
                and spellNameLine
                and spellNameLine.type == Enum.TooltipDataLineType.SpellName
                and spellNameLine.leftText == "Flash of Light"
                and spellCostLine
                and spellCostLine.leftText == "10% of Base MANA"
                and spellCastLine
                and spellCastLine.leftText == "1.5 sec cast"
            "#,
        )
        .unwrap();
    assert!(
        has_real_tooltip,
        "C_TooltipInfo.GetHyperlink should dispatch item and spell hyperlinks to tooltip data",
    );
}

#[test]
fn test_c_tooltip_info_get_spell_by_id_returns_spell_tooltip_lines() {
    let env = WowLuaEnv::new().unwrap();
    let has_real_tooltip: bool = env
        .eval(
            r#"
            local tooltip = C_TooltipInfo.GetSpellByID(19750)
            if not tooltip or tooltip.type ~= Enum.TooltipDataType.Spell or not tooltip.lines then
                return false
            end

            local nameLine = tooltip.lines[1]
            local costLine = tooltip.lines[2]
            local castLine = tooltip.lines[3]
            local descriptionLine = tooltip.lines[4]

            return nameLine
                and nameLine.type == Enum.TooltipDataLineType.SpellName
                and nameLine.leftText == "Flash of Light"
                and costLine
                and costLine.leftText == "10% of Base MANA"
                and castLine
                and castLine.leftText == "1.5 sec cast"
                and descriptionLine
                and descriptionLine.type == Enum.TooltipDataLineType.SpellDescription
                and type(descriptionLine.leftText) == "string"
                and descriptionLine.leftText ~= ""
                and descriptionLine.wrapText == true
            "#,
        )
        .unwrap();
    assert!(
        has_real_tooltip,
        "C_TooltipInfo.GetSpellByID should expose spell name, cost, cast time, and description lines",
    );
}

#[test]
fn test_c_tooltip_info_get_unit_buff_returns_aura_tooltip_lines() {
    let env = WowLuaEnv::new().unwrap();
    seed_c_tooltip_info_test_state(&env);

    let has_real_tooltip: bool = env
        .eval(
            r#"
            local tooltip = C_TooltipInfo.GetUnitBuff("player", 1, "HELPFUL")
            local auraTooltip = C_TooltipInfo.GetUnitAura("player", 1, "HELPFUL")
            if not tooltip or not auraTooltip or tooltip.type ~= Enum.TooltipDataType.UnitAura or not tooltip.lines then
                return false
            end

            local nameLine = tooltip.lines[1]
            local durationLine = tooltip.lines[2]
            local descriptionLine = tooltip.lines[3]
            local auraNameLine = auraTooltip.lines[1]
            local auraDurationLine = auraTooltip.lines[2]
            local auraDescriptionLine = auraTooltip.lines[3]

            return nameLine
                and nameLine.type == Enum.TooltipDataLineType.SpellName
                and nameLine.leftText == "Flash of Light"
                and durationLine
                and durationLine.leftText == "1 hr"
                and descriptionLine
                and descriptionLine.type == Enum.TooltipDataLineType.SpellDescription
                and type(descriptionLine.leftText) == "string"
                and descriptionLine.leftText ~= ""
                and descriptionLine.wrapText == true
                and auraTooltip.type == Enum.TooltipDataType.UnitAura
                and auraNameLine
                and auraNameLine.leftText == "Flash of Light"
                and auraDurationLine
                and auraDurationLine.leftText == "1 hr"
                and auraDescriptionLine
                and auraDescriptionLine.type == Enum.TooltipDataLineType.SpellDescription
                and type(auraDescriptionLine.leftText) == "string"
                and auraDescriptionLine.leftText ~= ""
                and auraDescriptionLine.wrapText == true
            "#,
        )
        .unwrap();
    assert!(
        has_real_tooltip,
        "C_TooltipInfo.GetUnitBuff and GetUnitAura should expose aura name, duration, and description lines",
    );
}

#[test]
fn test_c_tooltip_info_get_unit_debuff_returns_empty_without_simulated_debuffs() {
    let env = WowLuaEnv::new().unwrap();
    let is_empty_tooltip: bool = env
        .eval(
            r#"
            local tooltip = C_TooltipInfo.GetUnitDebuff("player", 1, "HARMFUL")
            return tooltip
                and tooltip.type == Enum.TooltipDataType.UnitAura
                and tooltip.lines
                and next(tooltip.lines) == nil
            "#,
        )
        .unwrap();
    assert!(
        is_empty_tooltip,
        "C_TooltipInfo.GetUnitDebuff should return an empty UnitAura tooltip when no debuffs exist",
    );
}

#[test]
fn test_c_tooltip_info_get_unit_aura_returns_helpful_and_harmful_tooltips() {
    let env = WowLuaEnv::new().unwrap();
    seed_c_tooltip_info_test_state(&env);

    let has_expected_tooltips: bool = env
        .eval(
            r#"
            local helpfulTooltip = C_TooltipInfo.GetUnitAura("player", 1, "HELPFUL")
            local harmfulTooltip = C_TooltipInfo.GetUnitAura("player", 1, "HARMFUL")

            local helpfulNameLine = helpfulTooltip and helpfulTooltip.lines and helpfulTooltip.lines[1]
            local helpfulDurationLine = helpfulTooltip and helpfulTooltip.lines and helpfulTooltip.lines[2]
            local harmfulIsEmpty = harmfulTooltip
                and harmfulTooltip.type == Enum.TooltipDataType.UnitAura
                and harmfulTooltip.lines
                and next(harmfulTooltip.lines) == nil

            return helpfulTooltip
                and helpfulTooltip.type == Enum.TooltipDataType.UnitAura
                and helpfulNameLine
                and helpfulNameLine.type == Enum.TooltipDataLineType.SpellName
                and helpfulNameLine.leftText == "Flash of Light"
                and helpfulDurationLine
                and helpfulDurationLine.leftText == "1 hr"
                and harmfulIsEmpty
            "#,
        )
        .unwrap();
    assert!(
        has_expected_tooltips,
        "C_TooltipInfo.GetUnitAura should route HELPFUL to buffs and HARMFUL to debuffs",
    );
}

#[test]
fn test_c_tooltip_info_get_unit_returns_player_and_target_tooltips() {
    let env = WowLuaEnv::new().unwrap();
    seed_c_tooltip_info_test_state(&env);
    let (player_name, player_level, player_race, player_class, player_color) = {
        let state = env.state().borrow();
        let player = &state.player;
        let player_race = crate::lua_api::state::RACE_DATA
            .get(player.race_index)
            .map(|(name, _, _)| (*name).to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        let player_class = crate::lua_api::game_data::CLASS_LABELS
            .get((player.class_index - 1).max(0) as usize)
            .copied()
            .unwrap_or("Unknown")
            .to_string();
        let player_color = match player.class_index {
            1 => (0.78, 0.61, 0.43),
            2 => (0.96, 0.55, 0.73),
            3 => (0.67, 0.83, 0.45),
            4 => (1.0, 0.96, 0.41),
            5 => (1.0, 1.0, 1.0),
            6 => (0.77, 0.12, 0.23),
            7 => (0.0, 0.44, 0.87),
            8 => (0.25, 0.78, 0.92),
            9 => (0.53, 0.53, 0.93),
            10 => (0.0, 1.0, 0.6),
            11 => (1.0, 0.49, 0.04),
            12 => (0.64, 0.19, 0.79),
            13 => (0.2, 0.58, 0.5),
            _ => (1.0, 1.0, 1.0),
        };
        (
            player.name.clone(),
            player.level,
            player_race,
            player_class,
            player_color,
        )
    };
    let has_expected_tooltips: bool = env
        .eval(&format!(
            r#"
            local playerTooltip = C_TooltipInfo.GetUnit("player")
            local targetTooltip = C_TooltipInfo.GetUnit("target")
            local missingTooltip = C_TooltipInfo.GetUnit("mouseover")

            local playerNameLine = playerTooltip and playerTooltip.lines and playerTooltip.lines[1]
            local playerLevelLine = playerTooltip and playerTooltip.lines and playerTooltip.lines[2]
            local playerRaceLine = playerTooltip and playerTooltip.lines and playerTooltip.lines[3]
            local playerClassLine = playerTooltip and playerTooltip.lines and playerTooltip.lines[4]
            local targetNameLine = targetTooltip and targetTooltip.lines and targetTooltip.lines[1]
            local targetLevelLine = targetTooltip and targetTooltip.lines and targetTooltip.lines[2]
            local targetRaceLine = targetTooltip and targetTooltip.lines and targetTooltip.lines[3]
            local targetClassLine = targetTooltip and targetTooltip.lines and targetTooltip.lines[4]

            local function approx(a, b)
                return math.abs(a - b) <= 0.01
            end

            local pr, pg, pb = playerNameLine.leftColor:GetRGB()
            local tr, tg, tb = targetNameLine.leftColor:GetRGB()

            return playerTooltip
                and playerTooltip.type == Enum.TooltipDataType.Unit
                and playerNameLine
                and playerNameLine.type == Enum.TooltipDataLineType.UnitName
                and playerNameLine.leftText == "{player_name}"
                and playerLevelLine
                and playerLevelLine.leftText == "Level {player_level}"
                and playerRaceLine
                and playerRaceLine.leftText == "{player_race}"
                and playerClassLine
                and playerClassLine.leftText == "{player_class}"
                and approx(pr, {player_r:.2})
                and approx(pg, {player_g:.2})
                and approx(pb, {player_b:.2})
                and targetTooltip
                and targetTooltip.type == Enum.TooltipDataType.Unit
                and targetNameLine
                and targetNameLine.type == Enum.TooltipDataLineType.UnitName
                and targetNameLine.leftText == "Lady Liadrin"
                and targetLevelLine
                and targetLevelLine.leftText == "Level 80"
                and targetRaceLine
                and targetRaceLine.leftText == "Blood Elf"
                and targetClassLine
                and targetClassLine.leftText == "Paladin"
                and approx(tr, 0.96)
                and approx(tg, 0.55)
                and approx(tb, 0.73)
                and missingTooltip
                and missingTooltip.type == Enum.TooltipDataType.Unit
                and missingTooltip.lines
                and next(missingTooltip.lines) == nil
            "#,
            player_r = player_color.0,
            player_g = player_color.1,
            player_b = player_color.2,
        ))
        .unwrap();
    assert!(
        has_expected_tooltips,
        "C_TooltipInfo.GetUnit should expose unit name, level, race, and class lines",
    );
}

#[test]
fn test_c_tooltip_info_get_action_returns_spell_tooltip_for_spell_slots() {
    let env = WowLuaEnv::new().unwrap();
    let has_expected_tooltips: bool = env
        .eval(
            r#"
            local slotTooltip = C_TooltipInfo.GetAction(1)
            local emptyTooltip = C_TooltipInfo.GetAction(999)

            local nameLine = slotTooltip and slotTooltip.lines and slotTooltip.lines[1]
            local castTimeLine = slotTooltip and slotTooltip.lines and slotTooltip.lines[3]

            return slotTooltip
                and slotTooltip.type == Enum.TooltipDataType.Spell
                and nameLine
                and nameLine.type == Enum.TooltipDataLineType.SpellName
                and nameLine.leftText == "Flash of Light"
                and castTimeLine
                and castTimeLine.leftText == "1.5 sec cast"
                and emptyTooltip == nil
            "#,
        )
        .unwrap();
    assert!(
        has_expected_tooltips,
        "C_TooltipInfo.GetAction should delegate spell slots to spell tooltip data and return nil for empty slots",
    );
}

#[test]
fn test_c_spell_get_spell_description_returns_compact_trait_text() {
    let env = WowLuaEnv::new().unwrap();
    let has_description: bool = env
        .eval(
            r#"
            local desc = C_Spell.GetSpellDescription(116)
            return type(desc) == "string" and desc ~= ""
            "#,
        )
        .unwrap();
    assert!(
        has_description,
        "C_Spell.GetSpellDescription should return compact spell text for trait-linked spells",
    );
}

#[test]
fn test_c_spell_get_spell_texture_covers_high_id_paladin_talent_spells() {
    let env = WowLuaEnv::new().unwrap();
    let no_fallback_icons: bool = env
        .eval(
            r#"
            local fallback = 136243
            local ids = {
                1272143,
                1232418,
                1279510,
                1242031,
                1247534,
                1230084,
                1232421,
                1234430,
            }
            for _, spellID in ipairs(ids) do
                local fileDataID = select(2, C_Spell.GetSpellTexture(spellID))
                if fileDataID == fallback then
                    return false
                end
            end
            return true
            "#,
        )
        .unwrap();
    assert!(
        no_fallback_icons,
        "high-id paladin talent spells should not fall back to Trade_Engineering icons",
    );
}
