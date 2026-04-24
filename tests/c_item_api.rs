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
