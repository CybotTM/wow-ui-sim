use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn reagent_bag_slot_uses_bag_file_data_id() {
    let (id, texture): (i32, i32) = env()
        .eval(r#"return GetInventorySlotInfo("ReagentBag0Slot")"#)
        .unwrap();
    assert_eq!(id, 25);
    assert_eq!(texture, 136511);
}

#[test]
fn wrist_slot_maps_to_wrists_file_data_id() {
    let (_, texture): (i32, i32) = env()
        .eval(r#"return GetInventorySlotInfo("WristSlot")"#)
        .unwrap();
    assert_eq!(texture, 136530);
}

#[test]
fn main_hand_slot_uses_main_hand_file_data_id() {
    let (_, texture): (i32, i32) = env()
        .eval(r#"return GetInventorySlotInfo("MainHandSlot")"#)
        .unwrap();
    assert_eq!(texture, 136518);
}

#[test]
fn bag_slots_share_single_bag_file_data_id() {
    let env = env();
    for slot in ["Bag0Slot", "Bag1Slot", "Bag2Slot", "Bag3Slot", "Bag4Slot"] {
        let code = format!(r#"return GetInventorySlotInfo("{slot}")"#);
        let (_, texture): (i32, i32) = env.eval(&code).unwrap();
        assert_eq!(texture, 136511, "slot {slot} should share Bag fileDataID");
    }
}

#[test]
fn main_hand_slot_id_is_16() {
    let id: i32 = env()
        .eval(r#"return (GetInventorySlotInfo("MainHandSlot"))"#)
        .unwrap();
    assert_eq!(id, 16);
}

#[test]
fn get_inventory_slot_info_returns_two_values() {
    let count: i32 = env()
        .eval(r#"return select('#', GetInventorySlotInfo("HeadSlot"))"#)
        .unwrap();
    assert_eq!(count, 2);
}
