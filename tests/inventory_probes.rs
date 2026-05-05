//! Integration tests for `src/lua_api/globals/inventory_probes.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{CursorInfo, CursorItemOrigin};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── IsInventoryItemLocked ─────────────────────────────────────────────────────

#[test]
fn is_inventory_item_locked_false_without_cursor_item() {
    let env = env();
    let b: bool = env.eval("return IsInventoryItemLocked(16)").unwrap();
    assert!(!b);
}

#[test]
fn is_inventory_item_locked_true_when_cursor_holds_that_slot() {
    let env = env();
    env.state().borrow_mut().cursor_item = Some(CursorInfo::Item {
        item_id: 1,
        stack_count: 1,
        origin: CursorItemOrigin::Equipped { slot: 16 },
    });
    let b: bool = env.eval("return IsInventoryItemLocked(16)").unwrap();
    assert!(b);
    // Different slot should still be false.
    let b: bool = env.eval("return IsInventoryItemLocked(5)").unwrap();
    assert!(!b);
}

// ── IsEquippableItem / IsConsumableItem ───────────────────────────────────────

#[test]
fn is_equippable_item_reads_state_set() {
    let env = env();
    env.state().borrow_mut().equippable_items.insert(19019);
    let b: bool = env.eval("return IsEquippableItem(19019)").unwrap();
    assert!(b);
    let b: bool = env.eval("return IsEquippableItem(6948)").unwrap();
    assert!(!b);
}

#[test]
fn is_consumable_item_reads_state_set() {
    let env = env();
    env.state().borrow_mut().consumable_items.insert(6948);
    let b: bool = env.eval("return IsConsumableItem(6948)").unwrap();
    assert!(b);
    let b: bool = env.eval("return IsConsumableItem(19019)").unwrap();
    assert!(!b);
}

// ── CanLootUnit ───────────────────────────────────────────────────────────────

#[test]
fn can_loot_unit_requires_dead_enemy_target() {
    let env = env();
    let b: bool = env.eval(r#"return CanLootUnit("target")"#).unwrap();
    assert!(!b, "with no target the probe is false");

    // Admin test helper: seed an enemy target at 0 HP.
    use wow_ui_sim::lua_api::state::TargetInfo;
    env.state().borrow_mut().current_target = Some(TargetInfo {
        unit_id: "target".into(),
        name: "Quilboar".into(),
        health: 0,
        health_max: 100,
        power: 0,
        power_max: 0,
        power_type: 0,
        power_type_name: "MANA".into(),
        is_player: false,
        is_enemy: true,
        guid: "Creature-0-0".into(),
        level: 10,
        class_index: 1,
        classification: "normal".into(),
        creature_type: "Humanoid".into(),
        reaction: 2,
    });
    let b: bool = env.eval(r#"return CanLootUnit("target")"#).unwrap();
    assert!(b);
}

// ── CanMerchant ───────────────────────────────────────────────────────────────

#[test]
fn can_merchant_tracks_merchant_frame_flag() {
    let env = env();
    let b: bool = env.eval("return CanMerchant()").unwrap();
    assert!(!b);
    env.state().borrow_mut().merchant_frame_open = true;
    let b: bool = env.eval("return CanMerchant()").unwrap();
    assert!(b);
}

// ── CanInspect ────────────────────────────────────────────────────────────────

#[test]
fn can_inspect_true_for_player_and_party_members() {
    let env = env();
    env.state().borrow_mut().party_group_active = true;
    let b_player: bool = env.eval(r#"return CanInspect("player")"#).unwrap();
    let b_party: bool = env.eval(r#"return CanInspect("party1")"#).unwrap();
    assert!(b_player);
    assert!(b_party);
}

#[test]
fn can_inspect_false_for_empty_target() {
    let env = env();
    env.state().borrow_mut().current_target = None;
    let b: bool = env.eval(r#"return CanInspect("target")"#).unwrap();
    assert!(!b);
}
