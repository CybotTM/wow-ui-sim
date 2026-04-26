//! Integration tests for the `C_AzeriteItem` Heart-of-Azeroth surface
//! registered in `src/c_api/c_azerite_item.rs`.

use wow_ui_sim::lua_api::{AzeriteItemState, ItemLocationData, WowLuaEnv};

fn sample_item() -> AzeriteItemState {
    AzeriteItemState {
        item_location: ItemLocationData {
            bag_id: None,
            slot_index: None,
            equipment_slot_index: Some(2),
        },
        current_xp: 4_500,
        max_xp: 9_000,
        power_level: 47,
        unlimited_power_level: 51,
        unlimited_unlocked: true,
        at_max_level: false,
        enabled: true,
    }
}

#[test]
fn find_active_azerite_item_is_nil_when_no_item() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_AzeriteItem.FindActiveAzeriteItem() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn find_active_azerite_item_returns_table_with_location_fields() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().azerite_item = Some(sample_item());
    env.exec(
        r#"
        local loc = C_AzeriteItem.FindActiveAzeriteItem()
        bag = loc.bagID
        slot = loc.slotIndex
        equip = loc.equipmentSlotIndex
    "#,
    )
    .unwrap();
    let bag_nil: bool = env.eval("return bag == nil").unwrap();
    let slot_nil: bool = env.eval("return slot == nil").unwrap();
    let equip: f64 = env.eval("return equip").unwrap();
    assert!(bag_nil);
    assert!(slot_nil);
    assert!((equip - 2.0).abs() < 1e-6);
}

#[test]
fn find_active_azerite_item_exposes_bag_and_slot_when_set() {
    let env = WowLuaEnv::new().expect("env");
    let mut item = sample_item();
    item.item_location = ItemLocationData {
        bag_id: Some(0),
        slot_index: Some(7),
        equipment_slot_index: None,
    };
    env.state().borrow_mut().azerite_item = Some(item);
    env.exec(
        r#"
        local loc = C_AzeriteItem.FindActiveAzeriteItem()
        bag = loc.bagID
        slot = loc.slotIndex
        equip = loc.equipmentSlotIndex
    "#,
    )
    .unwrap();
    let bag: f64 = env.eval("return bag").unwrap();
    let slot: f64 = env.eval("return slot").unwrap();
    let equip_nil: bool = env.eval("return equip == nil").unwrap();
    assert!(bag.abs() < 1e-6);
    assert!((slot - 7.0).abs() < 1e-6);
    assert!(equip_nil);
}

#[test]
fn get_azerite_item_xp_info_returns_no_values_when_unset() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_AzeriteItem.GetAzeriteItemXPInfo(nil) == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_azerite_item_xp_info_returns_xp_pair() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().azerite_item = Some(sample_item());
    env.exec(
        "current, maximum = C_AzeriteItem.GetAzeriteItemXPInfo(C_AzeriteItem.FindActiveAzeriteItem())",
    )
    .unwrap();
    let current: f64 = env.eval("return current").unwrap();
    let maximum: f64 = env.eval("return maximum").unwrap();
    assert!((current - 4_500.0).abs() < 1e-6);
    assert!((maximum - 9_000.0).abs() < 1e-6);
}

#[test]
fn get_power_level_returns_zero_without_item() {
    let env = WowLuaEnv::new().expect("env");
    let level: f64 = env.eval("return C_AzeriteItem.GetPowerLevel(nil)").unwrap();
    assert!(level.abs() < 1e-6);
}

#[test]
fn get_power_level_reads_state() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().azerite_item = Some(sample_item());
    let level: f64 = env.eval("return C_AzeriteItem.GetPowerLevel(nil)").unwrap();
    assert!((level - 47.0).abs() < 1e-6);
}

#[test]
fn get_unlimited_power_level_reads_state() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().azerite_item = Some(sample_item());
    let level: f64 = env
        .eval("return C_AzeriteItem.GetUnlimitedPowerLevel(nil)")
        .unwrap();
    assert!((level - 51.0).abs() < 1e-6);
}

#[test]
fn is_unlimited_leveling_unlocked_reads_flag_no_args() {
    let env = WowLuaEnv::new().expect("env");
    let none_default: bool = env
        .eval("return C_AzeriteItem.IsUnlimitedLevelingUnlocked()")
        .unwrap();
    assert!(!none_default);

    env.state().borrow_mut().azerite_item = Some(sample_item());
    let unlocked: bool = env
        .eval("return C_AzeriteItem.IsUnlimitedLevelingUnlocked()")
        .unwrap();
    assert!(unlocked);
}

#[test]
fn is_azerite_item_at_max_level_reads_flag() {
    let env = WowLuaEnv::new().expect("env");
    let none_default: bool = env
        .eval("return C_AzeriteItem.IsAzeriteItemAtMaxLevel()")
        .unwrap();
    assert!(!none_default);

    let mut item = sample_item();
    item.at_max_level = true;
    env.state().borrow_mut().azerite_item = Some(item);
    let maxed: bool = env
        .eval("return C_AzeriteItem.IsAzeriteItemAtMaxLevel()")
        .unwrap();
    assert!(maxed);
}

#[test]
fn is_azerite_item_enabled_reads_flag() {
    let env = WowLuaEnv::new().expect("env");
    let none_default: bool = env
        .eval("return C_AzeriteItem.IsAzeriteItemEnabled(nil)")
        .unwrap();
    assert!(!none_default);

    env.state().borrow_mut().azerite_item = Some(sample_item());
    let enabled: bool = env
        .eval("return C_AzeriteItem.IsAzeriteItemEnabled(nil)")
        .unwrap();
    assert!(enabled);
}

#[test]
fn azerite_bar_helper_consumes_xp_info() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().azerite_item = Some(sample_item());
    env.exec(
        r#"
        local function ratio()
            local loc = C_AzeriteItem.FindActiveAzeriteItem()
            if not loc then return 0 end
            local current, maximum = C_AzeriteItem.GetAzeriteItemXPInfo(loc)
            if maximum == 0 then return 0 end
            return current / maximum
        end
        result = ratio()
    "#,
    )
    .unwrap();
    let result: f64 = env.eval("return result").unwrap();
    assert!((result - 0.5).abs() < 1e-6);
}
