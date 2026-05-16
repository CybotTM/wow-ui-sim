use crate::support::env;

#[test]
fn test_c_item_surface_registers_expected_methods() {
    let env = env();
    let all_present: bool = env
        .eval(
            r#"
            local methods = {
                "DoesItemExist",
                "DoesItemExistByID",
                "GetItemID",
                "IsItemDataCached",
                "IsItemDataCachedByID",
                "GetItemIDForItemInfo",
                "GetItemIcon",
                "GetItemName",
                "GetItemIconByID",
                "GetItemNameByID",
                "GetItemQualityByID",
                "GetItemInfoInstant",
                "GetItemInfo",
                "GetDetailedItemLevelInfo",
                "GetItemSubClassInfo",
                "GetItemClassInfo",
                "GetItemCount",
                "GetItemLink",
                "GetItemCooldown",
                "GetItemGUID",
                "IsHelpfulItem",
                "IsHarmfulItem",
                "GetItemInventorySlotInfo",
            }
            for _, method_name in ipairs(methods) do
                if type(C_Item[method_name]) ~= "function" then
                    return false
                end
            end
            return true
            "#,
        )
        .unwrap();
    assert!(
        all_present,
        "C_Item should expose the full method surface after registration refactors"
    );
}

#[test]
fn test_c_item_get_item_info_returns_nil() {
    let env = env();
    let is_nil: bool = env.eval("return C_Item.GetItemInfo(42) == nil").unwrap();
    assert!(is_nil);
}

#[test]
fn test_c_item_get_item_info_returns_data() {
    let env = env();
    let name: String = env.eval("return C_Item.GetItemInfo(6948)").unwrap();
    assert_eq!(name, "Hearthstone");
}

#[test]
fn test_c_item_get_item_info_instant_by_id() {
    let env = env();
    let (item_id, item_type, item_sub_type): (i64, String, String) =
        env.eval("return C_Item.GetItemInfoInstant(12345)").unwrap();
    assert_eq!(item_id, 12345);
    assert_eq!(item_type, "Miscellaneous");
    assert_eq!(item_sub_type, "Junk");
}

#[test]
fn test_c_item_unknown_positive_item_ids_have_synthetic_cached_data() {
    let env = env();
    let (exists, cached): (bool, bool) = env
        .eval("return C_Item.DoesItemExistByID(224116), C_Item.IsItemDataCachedByID(224116)")
        .unwrap();

    assert!(exists, "synthetic item data should make positive item IDs exist");
    assert!(cached, "synthetic item data should already be cached");
}

#[test]
fn test_c_item_get_item_info_instant_by_link() {
    let env = env();
    let item_id: i64 = env
        .eval(r#"return C_Item.GetItemInfoInstant("|cffffffff|Hitem:54321::::::::80:::::|h[Test]|h|r")"#)
        .unwrap();
    assert_eq!(item_id, 54321);
}

#[test]
fn test_c_item_get_item_info_instant_invalid() {
    let env = env();
    let count: i32 = env
        .eval("return select('#', C_Item.GetItemInfoInstant(nil))")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_c_item_get_item_id_for_item_info_integer() {
    let env = env();
    let id: i64 = env.eval("return C_Item.GetItemIDForItemInfo(999)").unwrap();
    assert_eq!(id, 999);
}

#[test]
fn test_c_item_get_item_id_for_item_info_link() {
    let env = env();
    let id: i64 = env
        .eval(
            r#"return C_Item.GetItemIDForItemInfo("|cffffffff|Hitem:42::::::::80:::::|h[X]|h|r")"#,
        )
        .unwrap();
    assert_eq!(id, 42);
}

#[test]
fn test_c_item_get_item_id_for_item_info_invalid() {
    let env = env();
    let is_nil: bool = env
        .eval(r#"return C_Item.GetItemIDForItemInfo("not a link") == nil"#)
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_c_item_get_item_icon_by_id() {
    let env = env();
    let icon: i32 = env.eval("return C_Item.GetItemIconByID(1)").unwrap();
    assert_eq!(icon, 134400);
}

#[test]
fn test_c_item_get_item_quality_by_id() {
    let env = env();
    let quality: i32 = env.eval("return C_Item.GetItemQualityByID(1)").unwrap();
    assert_eq!(quality, 1);
}

#[test]
fn test_c_item_get_item_link() {
    let env = env();
    let link: String = env.eval("return C_Item.GetItemLink(6948)").unwrap();
    assert!(link.contains("Hitem:6948"));
    assert!(link.contains("[Hearthstone]"));
}

#[test]
fn test_c_item_item_location_queries_return_seeded_backpack_metadata() {
    let env = env();
    let (item_id, icon, icon_by_id, name, link_ok, link_or_err): (
        Option<i64>,
        Option<i64>,
        i64,
        String,
        bool,
        String,
    ) = env
        .eval(
            r#"
            local itemLocation = { bagID = 0, slotIndex = 1 }
            local ok, linkOrErr = pcall(function()
                return C_Item.GetItemLink(itemLocation)
            end)

            return C_Item.GetItemID(itemLocation),
                C_Item.GetItemIcon(itemLocation),
                C_Item.GetItemIconByID(6948),
                C_Item.GetItemName(itemLocation),
                ok,
                tostring(linkOrErr)
            "#,
        )
        .unwrap();

    assert_eq!(
        item_id,
        Some(6948),
        "The seeded backpack hearthstone should resolve through C_Item.GetItemID(ItemLocation)"
    );
    assert_eq!(
        icon,
        Some(icon_by_id),
        "The seeded backpack hearthstone should resolve through C_Item.GetItemIcon(ItemLocation)"
    );
    assert_eq!(
        name, "Hearthstone",
        "The seeded backpack hearthstone should resolve through C_Item.GetItemName(ItemLocation)"
    );
    assert!(
        link_ok,
        "C_Item.GetItemLink(ItemLocation) should not error: {link_or_err}"
    );
    assert!(
        link_or_err.contains("Hitem:6948") && link_or_err.contains("[Hearthstone]"),
        "C_Item.GetItemLink(ItemLocation) should return the seeded backpack hearthstone link: {link_or_err}"
    );
}

#[test]
fn test_c_item_get_item_name_by_id() {
    let env = env();
    let name: String = env.eval("return C_Item.GetItemNameByID(6948)").unwrap();
    assert_eq!(name, "Hearthstone");
}

#[test]
fn test_c_item_get_item_name_by_id_unknown() {
    let env = env();
    let name: String = env.eval("return C_Item.GetItemNameByID(42)").unwrap();
    assert_eq!(name, "Unknown");
}

#[test]
fn test_c_item_get_item_sub_class_info_weapon() {
    let env = env();
    let name: String = env.eval("return C_Item.GetItemSubClassInfo(2, 7)").unwrap();
    assert_eq!(name, "One-Handed Swords");
}

#[test]
fn test_c_item_get_item_sub_class_info_armor() {
    let env = env();
    let name: String = env.eval("return C_Item.GetItemSubClassInfo(4, 4)").unwrap();
    assert_eq!(name, "Plate");
}

#[test]
fn test_c_item_get_item_sub_class_info_unknown() {
    let env = env();
    let name: String = env
        .eval("return C_Item.GetItemSubClassInfo(99, 99)")
        .unwrap();
    assert_eq!(name, "Unknown");
}

#[test]
fn test_c_item_get_item_class_info() {
    let env = env();
    let name: String = env.eval("return C_Item.GetItemClassInfo(2)").unwrap();
    assert_eq!(name, "Weapon");
}

#[test]
fn test_c_item_get_item_class_info_retail_tradeskill() {
    let env = env();
    let name: String = env.eval("return C_Item.GetItemClassInfo(7)").unwrap();
    assert_eq!(name, "Tradeskill");
}

#[test]
fn test_c_item_get_item_class_info_retail_battle_pets() {
    let env = env();
    let name: String = env.eval("return C_Item.GetItemClassInfo(17)").unwrap();
    assert_eq!(name, "Battle Pets");
}

#[test]
fn test_c_item_get_detailed_item_level_info() {
    let env = env();
    let (a, b, c): (i32, i32, i32) = env
        .eval("return C_Item.GetDetailedItemLevelInfo(42)")
        .unwrap();
    assert_eq!((a, b, c), (0, 0, 0));
}

#[test]
fn test_c_item_get_detailed_item_level_info_real() {
    let env = env();
    let (level, _, _): (i32, i32, i32) = env
        .eval("return C_Item.GetDetailedItemLevelInfo(6948)")
        .unwrap();
    assert!(level > 0);
}

#[test]
fn test_c_item_get_current_item_level_from_item_location() {
    let env = env();
    let level: i32 = env
        .eval("return C_Item.GetCurrentItemLevel({ bagID = 0, slotIndex = 1 })")
        .unwrap();
    assert_eq!(level, 1);
}

#[test]
fn test_c_item_get_quality_and_stack_count_from_item_location() {
    let env = env();
    env.exec("A_Admin.AddBagItem(0, 5, 159, 5)").unwrap();
    let (quality, count): (i32, i32) = env
        .eval("return C_Item.GetItemQuality({ bagID = 0, slotIndex = 5 }), C_Item.GetStackCount({ bagID = 0, slotIndex = 5 })")
        .unwrap();
    assert_eq!(quality, 1);
    assert_eq!(count, 5);
}

#[test]
fn test_c_item_binding_probes_return_booleans_for_item_location() {
    let env = env();
    let (is_bound, is_bound_until_equip): (bool, bool) = env
        .eval("return C_Item.IsBound({ bagID = 0, slotIndex = 1 }), C_Item.IsBoundToAccountUntilEquip({ bagID = 0, slotIndex = 1 })")
        .unwrap();
    assert!(!is_bound);
    assert!(!is_bound_until_equip);
}

#[test]
fn test_c_item_get_item_count() {
    let env = env();
    let count: i32 = env.eval("return C_Item.GetItemCount(12345)").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_c_item_does_item_exist_by_id_resolves_profession_overrides() {
    let env = env();
    let (exists, name): (bool, String) = env
        .eval("return C_Item.DoesItemExistByID(2852), C_Item.GetItemNameByID(2852)")
        .unwrap();
    assert!(
        exists,
        "C_Item.DoesItemExistByID(2852) must resolve via profession_item_overrides — \
         otherwise NonEmptyItem:ContinueWithCancelOnItemLoad raises and \
         ProfessionsRecipeSchematicForm:Init aborts before initializing reagentSlots"
    );
    assert_eq!(name, "Copper Chain Pants");
}
