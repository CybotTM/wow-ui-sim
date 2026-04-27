//! `C_ActionBar` slot mutation + profession-quality probes consumed by
//! Blizzard_ActionBar.
//!
//! - `PutActionInSlot(slot, targetSlot)` moves `state.action_bars[slot]` to
//!   `state.action_bars[targetSlot]` and fires `ACTIONBAR_SLOT_CHANGED`.
//! - `ForceUpdateAction(slot)` fires `ACTIONBAR_SLOT_CHANGED`.
//! - `GetProfessionQualityInfo(slot)` reads `state.action_profession_quality`.

use wow_ui_sim::lua_api::{ProfessionQualityInfo, WowLuaEnv};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("env")
}

#[test]
fn put_action_in_slot_moves_spell_and_returns_true() {
    let env = env();
    env.state().borrow_mut().action_bars.insert(1, 133);
    let moved: bool = env
        .eval("return C_ActionBar.PutActionInSlot(1, 5)")
        .unwrap();
    assert!(moved, "should return true when there was an action to move");
    let sim = env.state().borrow();
    assert!(
        !sim.action_bars.contains_key(&1),
        "source slot should be cleared"
    );
    assert_eq!(
        sim.action_bars.get(&5).copied(),
        Some(133),
        "target slot should hold the moved spell id"
    );
}

#[test]
fn put_action_in_slot_empty_source_returns_false() {
    let env = env();
    env.state().borrow_mut().action_bars.clear();
    let moved: bool = env
        .eval("return C_ActionBar.PutActionInSlot(50, 51)")
        .unwrap();
    assert!(!moved, "should return false when source slot has no action");
}

#[test]
fn put_action_in_slot_invalid_input_returns_false() {
    let env = env();
    env.state().borrow_mut().action_bars.insert(1, 133);
    let from_string: bool = env
        .eval(r#"return C_ActionBar.PutActionInSlot("one", 5)"#)
        .unwrap();
    let missing_target: bool = env.eval("return C_ActionBar.PutActionInSlot(1)").unwrap();
    assert!(!from_string);
    assert!(!missing_target);
}

#[test]
fn put_action_in_slot_fires_actionbar_slot_changed_for_both() {
    let env = env();
    env.state().borrow_mut().action_bars.insert(1, 133);
    let (call_count, slot_a, slot_b): (i64, i64, i64) = env
        .eval(
            r#"
            _G.__seen = {}
            local frame = CreateFrame("Frame")
            frame:RegisterEvent("ACTIONBAR_SLOT_CHANGED")
            frame:SetScript("OnEvent", function(_, _, slot)
                table.insert(_G.__seen, slot)
            end)
            C_ActionBar.PutActionInSlot(1, 5)
            return #_G.__seen, _G.__seen[1] or -1, _G.__seen[2] or -1
            "#,
        )
        .unwrap();
    assert_eq!(call_count, 2, "expected one event per affected slot");
    assert_eq!(slot_a, 1, "source slot should fire first");
    assert_eq!(slot_b, 5, "target slot should fire second");
}

#[test]
fn force_update_action_fires_event_for_slot() {
    let env = env();
    let (call_count, slot_arg): (i64, i64) = env
        .eval(
            r#"
            _G.__seen = {}
            local frame = CreateFrame("Frame")
            frame:RegisterEvent("ACTIONBAR_SLOT_CHANGED")
            frame:SetScript("OnEvent", function(_, _, slot)
                table.insert(_G.__seen, slot)
            end)
            C_ActionBar.ForceUpdateAction(7)
            return #_G.__seen, _G.__seen[1] or -1
            "#,
        )
        .unwrap();
    assert_eq!(call_count, 1);
    assert_eq!(slot_arg, 7);
}

#[test]
fn force_update_action_invalid_input_is_noop() {
    let env = env();
    let call_count: i64 = env
        .eval(
            r#"
            _G.__seen = {}
            local frame = CreateFrame("Frame")
            frame:RegisterEvent("ACTIONBAR_SLOT_CHANGED")
            frame:SetScript("OnEvent", function(_, _, slot)
                table.insert(_G.__seen, slot)
            end)
            C_ActionBar.ForceUpdateAction("bad")
            return #_G.__seen
            "#,
        )
        .unwrap();
    assert_eq!(call_count, 0);
}

#[test]
fn profession_quality_info_defaults_nil() {
    let env = env();
    let result: Option<bool> = env
        .eval(
            r#"
            local info = C_ActionBar.GetProfessionQualityInfo(1)
            if info == nil then return nil end
            return true
            "#,
        )
        .unwrap();
    assert!(
        result.is_none(),
        "missing slot should return nil so the overlay clears"
    );
}

#[test]
fn profession_quality_info_returns_state_table() {
    let env = env();
    env.state().borrow_mut().action_profession_quality.insert(
        1,
        ProfessionQualityInfo {
            inventory_quality: 3,
            icon_inventory: "Professions-Icon-Quality-Tier3-Inv".into(),
            icon_quality_container: "Professions-Icon-Quality-Tier3".into(),
        },
    );
    let (quality, inv, container): (i64, String, String) = env
        .eval(
            r#"
            local info = C_ActionBar.GetProfessionQualityInfo(1)
            return info.inventoryQuality, info.iconInventory, info.iconQualityContainer
            "#,
        )
        .unwrap();
    assert_eq!(quality, 3);
    assert_eq!(inv, "Professions-Icon-Quality-Tier3-Inv");
    assert_eq!(container, "Professions-Icon-Quality-Tier3");
}

#[test]
fn profession_quality_info_invalid_input_returns_nil() {
    let env = env();
    let result: Option<bool> = env
        .eval(
            r#"
            local info = C_ActionBar.GetProfessionQualityInfo({})
            if info == nil then return nil end
            return true
            "#,
        )
        .unwrap();
    assert!(result.is_none());
}

#[test]
fn update_profession_quality_branches_on_state_presence() {
    // Mirrors ActionBarActionButtonMixin:UpdateProfessionQuality — show
    // the overlay when GetProfessionQualityInfo returns a table, clear
    // otherwise.
    let env = env();
    env.state().borrow_mut().action_profession_quality.insert(
        2,
        ProfessionQualityInfo {
            inventory_quality: 1,
            icon_inventory: "Atlas-Inv".into(),
            icon_quality_container: "Atlas-Container".into(),
        },
    );
    let (slot_with, slot_without): (bool, bool) = env
        .eval(
            r#"
            local function shouldShowOverlay(slot)
                local info = C_ActionBar.GetProfessionQualityInfo(slot)
                return info ~= nil
            end
            return shouldShowOverlay(2), shouldShowOverlay(99)
            "#,
        )
        .unwrap();
    assert!(slot_with, "slot with quality info should show overlay");
    assert!(
        !slot_without,
        "slot without quality info should clear overlay"
    );
}
