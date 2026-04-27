//! Integration tests for the `C_AzeriteEmpoweredItem` respec-flow surface
//! registered in `src/c_api/c_azerite_empowered_item.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{BagItem, EquippedItem};

#[test]
fn namespace_is_present() {
    let env = WowLuaEnv::new().expect("env");
    let ns_type: String = env.eval("return type(C_AzeriteEmpoweredItem)").unwrap();
    assert_eq!(ns_type, "table");
    let close_type: String = env
        .eval("return type(C_AzeriteEmpoweredItem.CloseAzeriteEmpoweredItemRespec)")
        .unwrap();
    assert_eq!(close_type, "function");
}

#[test]
fn get_respec_cost_defaults_to_zero() {
    let env = WowLuaEnv::new().expect("env");
    let cost: f64 = env
        .eval("return C_AzeriteEmpoweredItem.GetAzeriteEmpoweredItemRespecCost()")
        .unwrap();
    assert!(cost.abs() < 1e-6);
}

#[test]
fn get_respec_cost_reads_state() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().azerite_empowered.respec_cost = 12_345_600;
    let cost: f64 = env
        .eval("return C_AzeriteEmpoweredItem.GetAzeriteEmpoweredItemRespecCost()")
        .unwrap();
    assert!((cost - 12_345_600.0).abs() < 1e-3);
}

#[test]
fn is_empowered_returns_false_without_state() {
    let env = WowLuaEnv::new().expect("env");
    let empowered: bool = env
        .eval("return C_AzeriteEmpoweredItem.IsAzeriteEmpoweredItem({itemID = 158041})")
        .unwrap();
    assert!(!empowered);
}

#[test]
fn is_empowered_returns_false_for_nil_location() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .azerite_empowered
        .empowered_items
        .insert(158041);
    let empowered: bool = env
        .eval("return C_AzeriteEmpoweredItem.IsAzeriteEmpoweredItem(nil)")
        .unwrap();
    assert!(!empowered);
}

#[test]
fn is_empowered_matches_item_id_table_shape() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .azerite_empowered
        .empowered_items
        .insert(158041);
    let empowered: bool = env
        .eval("return C_AzeriteEmpoweredItem.IsAzeriteEmpoweredItem({itemID = 158041})")
        .unwrap();
    assert!(empowered);
    let other: bool = env
        .eval("return C_AzeriteEmpoweredItem.IsAzeriteEmpoweredItem({itemID = 99999})")
        .unwrap();
    assert!(!other);
}

#[test]
fn is_empowered_resolves_bag_and_slot() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut state = env.state().borrow_mut();
        state.azerite_empowered.empowered_items.insert(158041);
        state.bag_items.insert(
            (0, 3),
            BagItem {
                item_id: 158041,
                stack_count: 1,
            },
        );
    }
    let empowered: bool = env
        .eval("return C_AzeriteEmpoweredItem.IsAzeriteEmpoweredItem({bagID = 0, slotIndex = 3})")
        .unwrap();
    assert!(empowered);
}

#[test]
fn is_empowered_resolves_equipment_slot() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut state = env.state().borrow_mut();
        state.azerite_empowered.empowered_items.insert(158041);
        state.player.equipped_items.insert(
            5,
            EquippedItem {
                item_id: 158041,
                enchant_id: 0,
                gem_ids: [0, 0, 0],
            },
        );
    }
    let empowered: bool = env
        .eval("return C_AzeriteEmpoweredItem.IsAzeriteEmpoweredItem({equipmentSlotIndex = 5})")
        .unwrap();
    assert!(empowered);
}

#[test]
fn confirm_respec_records_location() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        "C_AzeriteEmpoweredItem.ConfirmAzeriteEmpoweredItemRespec({bagID = 0, slotIndex = 4})",
    )
    .unwrap();
    let recorded = env
        .state()
        .borrow()
        .azerite_empowered
        .last_confirmed_respec
        .clone();
    let location = recorded.expect("confirm should record the location");
    assert_eq!(location.bag_id, Some(0));
    assert_eq!(location.slot_index, Some(4));
    assert_eq!(location.equipment_slot_index, None);
}

#[test]
fn confirm_respec_is_noop_with_nil_location() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("C_AzeriteEmpoweredItem.ConfirmAzeriteEmpoweredItemRespec(nil)")
        .unwrap();
    let recorded = env
        .state()
        .borrow()
        .azerite_empowered
        .last_confirmed_respec
        .clone();
    assert!(recorded.is_none());
}

#[test]
fn close_respec_fires_event_and_records_location() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        fired = 0
        local frame = CreateFrame("Frame")
        frame:RegisterEvent("AZERITE_EMPOWERED_ITEM_RESPEC_CLOSE")
        frame:SetScript("OnEvent", function() fired = fired + 1 end)
        C_AzeriteEmpoweredItem.CloseAzeriteEmpoweredItemRespec({equipmentSlotIndex = 7})
    "#,
    )
    .unwrap();
    let fired: f64 = env.eval("return fired").unwrap();
    assert!((fired - 1.0).abs() < 1e-6);
    let recorded = env
        .state()
        .borrow()
        .azerite_empowered
        .last_close_request
        .clone();
    let location = recorded.expect("close should record the location");
    assert_eq!(location.equipment_slot_index, Some(7));
}

#[test]
fn close_respec_with_no_arg_still_fires_event() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        fired = 0
        local frame = CreateFrame("Frame")
        frame:RegisterEvent("AZERITE_EMPOWERED_ITEM_RESPEC_CLOSE")
        frame:SetScript("OnEvent", function() fired = fired + 1 end)
        C_AzeriteEmpoweredItem.CloseAzeriteEmpoweredItemRespec()
    "#,
    )
    .unwrap();
    let fired: f64 = env.eval("return fired").unwrap();
    assert!((fired - 1.0).abs() < 1e-6);
    let recorded = env
        .state()
        .borrow()
        .azerite_empowered
        .last_close_request
        .clone();
    assert!(recorded.is_none());
}
