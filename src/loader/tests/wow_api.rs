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
    let _: i32 = env.eval("return Enum.InventoryType.IndexShoulderType").unwrap();
    let _: i32 = env.eval("return Enum.InventoryType.IndexChestType").unwrap();
    let _: i32 = env.eval("return Enum.InventoryType.IndexWeaponType").unwrap();
    let _: i32 = env.eval("return Enum.InventoryType.IndexShieldType").unwrap();
    let _: i32 = env.eval("return Enum.InventoryType.Index2HweaponType").unwrap();
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
    let _: i32 = env.eval("return LE_EXPANSION_WRATH_OF_THE_LICH_KING").unwrap();
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

// WOW_PROJECT_ID, WOW_PROJECT_MAINLINE: from Blizzard_SharedXML/ProjectConstants.lua
// Tested via Lua addon tests (run-tests), not available in bare WowLuaEnv.

// ---------------------------------------------------------------------------
// C_Container API
// ---------------------------------------------------------------------------

#[test]
fn test_c_container_get_num_slots() {
    let env = WowLuaEnv::new().unwrap();
    let slots: i32 = env.eval("return C_Container.GetContainerNumSlots(0)").unwrap();
    assert!(slots >= 0);
}

#[test]
fn test_c_container_get_num_free_slots() {
    let env = WowLuaEnv::new().unwrap();
    let free: i32 = env.eval("return C_Container.GetContainerNumFreeSlots(0)").unwrap();
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
    let size: i32 = env.eval("return C_CurrencyInfo.GetCurrencyListSize()").unwrap();
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
    let fn_ty: String = env
        .eval("return type(C_Bank.FetchDepositedMoney)")
        .unwrap();
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

// ---------------------------------------------------------------------------
// Global functions
// ---------------------------------------------------------------------------

#[test]
fn test_get_build_info() {
    let env = WowLuaEnv::new().unwrap();
    let (version, toc): (String, i32) =
        env.eval("local v,_,_,t = GetBuildInfo(); return v, t").unwrap();
    assert!(!version.is_empty());
    assert!(toc > 0);
}

#[test]
fn test_get_locale() {
    let env = WowLuaEnv::new().unwrap();
    let locale: String = env.eval("return GetLocale()").unwrap();
    assert!(!locale.is_empty());
}

#[test]
fn test_unit_name_player() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env.eval("return UnitName('player')").unwrap();
    assert!(!name.is_empty());
}

#[test]
fn test_get_money() {
    let env = WowLuaEnv::new().unwrap();
    let money: i64 = env.eval("return GetMoney()").unwrap();
    assert!(money >= 0);
}

#[test]
fn test_in_combat_lockdown_false() {
    let env = WowLuaEnv::new().unwrap();
    let in_combat: bool = env.eval("return InCombatLockdown()").unwrap();
    assert!(!in_combat);
}

#[test]
fn test_wipe_function() {
    let (t, _) = load_test_lua(
        "test-wipe",
        r#"
        local t = {1, 2, 3, a = "b"}
        wipe(t)
        WIPE_LEN = #t
        WIPE_A_NIL = (t.a == nil)
    "#,
    );
    let len: i32 = t.env.eval("return WIPE_LEN").unwrap();
    assert_eq!(len, 0);
    t.assert_lua_true("return WIPE_A_NIL", "wipe should clear named keys");
}

#[test]
fn test_copy_table_deep() {
    let (t, _) = load_test_lua(
        "test-copytable",
        r#"
        local orig = {a = 1, b = {c = 2}}
        local copy = CopyTable(orig)
        COPY_A = copy.a
        COPY_BC = copy.b.c
        copy.a = 99
        ORIG_A = orig.a
    "#,
    );
    let copy_a: i32 = t.env.eval("return COPY_A").unwrap();
    assert_eq!(copy_a, 1);
    let copy_bc: i32 = t.env.eval("return COPY_BC").unwrap();
    assert_eq!(copy_bc, 2);
    let orig_a: i32 = t.env.eval("return ORIG_A").unwrap();
    assert_eq!(orig_a, 1, "original should be unmodified");
}

#[test]
fn test_strsplit() {
    let (t, _) = load_test_lua(
        "test-strsplit",
        r#"
        local a, b, c = strsplit(",", "one,two,three")
        SS_A, SS_B, SS_C = a, b, c
    "#,
    );
    t.assert_lua_str("return SS_A", "one");
    t.assert_lua_str("return SS_B", "two");
    t.assert_lua_str("return SS_C", "three");
}

#[test]
fn test_strtrim() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env.eval(r#"return strtrim("  hello  ")"#).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_geterrorhandler() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(geterrorhandler())").unwrap();
    assert_eq!(ty, "function");
}

#[test]
fn test_hooksecurefunc() {
    let (t, _) = load_test_lua(
        "test-hooksecure",
        r#"
        local obj = { MyMethod = function() end }
        HOOK_CALLED = false
        hooksecurefunc(obj, "MyMethod", function() HOOK_CALLED = true end)
        obj:MyMethod()
    "#,
    );
    t.assert_lua_true("return HOOK_CALLED", "hook should fire");
}

#[test]
fn test_hooksecurefunc_on_frame_userdata() {
    let (t, _) = load_test_lua(
        "test-hooksecure-ud",
        r#"
        local f = CreateFrame("Frame", "HookSecureUDTest", UIParent)
        HOOK_CALLED = false
        hooksecurefunc(f, "SetAlpha", function() HOOK_CALLED = true end)
        f:SetAlpha(0.5)
    "#,
    );
    t.assert_lua_true("return HOOK_CALLED", "hook should fire on userdata frame");
}

#[test]
fn test_mixin() {
    let (t, _) = load_test_lua(
        "test-mixin",
        r#"
        local target = {}
        Mixin(target, {foo = 1, bar = "hello"})
        MIX_FOO = target.foo
        MIX_BAR = target.bar
    "#,
    );
    let foo: i32 = t.env.eval("return MIX_FOO").unwrap();
    assert_eq!(foo, 1);
    t.assert_lua_str("return MIX_BAR", "hello");
}

#[test]
fn test_global_functions_callable() {
    let env = WowLuaEnv::new().unwrap();
    for f in &[
        "BreakUpLargeNumbers",
        "PlaySound",
        "ReloadUI",
        "GetBindingKey",
        "SetOverrideBinding",
        "ClearOverrideBindings",
        "GetInventoryItemLink",
        "GetInventoryItemTexture",
        "GetInventorySlotInfo",
        "GetFramerate",
        "format",
        "strjoin",
    ] {
        let expr = format!("return type({})", f);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function", "{} should be function", f);
    }
}

// ---------------------------------------------------------------------------
// Global frames and tables
// ---------------------------------------------------------------------------

#[test]
fn test_uiparent_exists() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return UIParent:GetObjectType()").unwrap();
    assert_eq!(ty, "Frame");
}

#[test]
fn test_ui_special_frames_table() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(UISpecialFrames)").unwrap();
    assert_eq!(ty, "table");
}

// SOUNDKIT: from Blizzard_SharedXML/SoundKitConstants.lua
// Tested via Lua addon tests (run-tests).

#[test]
fn test_game_tooltip_methods() {
    let env = WowLuaEnv::new().unwrap();
    for m in &["SetOwner", "Show", "Hide"] {
        let expr = format!("return type(GameTooltip.{})", m);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function", "GameTooltip.{} should be function", m);
    }
}

#[test]
fn test_static_popup() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(StaticPopup_Show)").unwrap();
    assert_eq!(ty, "function");
    let ty2: String = env.eval("return type(StaticPopupDialogs)").unwrap();
    assert_eq!(ty2, "table");
}

// ContinuableContainer, ItemButtonUtil, ScrollUtil, CreateScrollBoxLinearView,
// MainMenuBarBackpackButton: all from Blizzard addon Lua/XML.
// Tested via Lua addon tests (run-tests).
