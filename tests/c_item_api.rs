//! Tests for C_Item, C_Container, C_EncodingUtil, and related global functions (c_item_api.rs).

#[path = "c_item_api/c_container.rs"]
mod c_container;
#[path = "c_item_api/c_item.rs"]
mod c_item;
#[path = "c_item_api/globals_and_inventory.rs"]
mod globals_and_inventory;
#[path = "c_item_api/support.rs"]
mod support;

#[test]
fn c_equipment_set_count_defaults_to_zero() {
    let env = support::env();
    let count: i32 = env
        .eval("return C_EquipmentSet.GetNumEquipmentSets()")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn use_equipment_set_updates_equipped_inventory_items() {
    let env = support::env();
    let equipped_id: i64 = env
        .eval(
            "C_EquipmentSet.CreateEquipmentSet('Default Gear', ''); \
             local setID = C_EquipmentSet.GetEquipmentSetID('Default Gear'); \
             A_Admin.EquipItem(1, 229181); \
             C_EquipmentSet.UseEquipmentSet(setID); \
             return GetInventoryItemID('player', 1)",
        )
        .unwrap();
    assert_eq!(equipped_id, 211993);
}
