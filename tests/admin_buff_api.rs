//! Tests for A_Admin buff/aura simulation API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// AddBuff / GetPlayerAuraBySpellID
// ============================================================================

#[test]
fn test_add_buff_is_findable_by_spell_id() {
    let env = env();
    let found: bool = env
        .eval(
            r#"
            A_Admin.AddBuff(99001, "Power Word: Shield", "134973", 30, 1)
            local aura = C_UnitAuras.GetPlayerAuraBySpellID(99001)
            return aura ~= nil
            "#,
        )
        .unwrap();
    assert!(
        found,
        "AddBuff should make the aura findable via GetPlayerAuraBySpellID"
    );
}

#[test]
fn test_add_buff_name_is_correct() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.AddBuff(99001, "Power Word: Shield", "134973", 30, 1)
            local aura = C_UnitAuras.GetPlayerAuraBySpellID(99001)
            return aura.name
            "#,
        )
        .unwrap();
    assert_eq!(name, "Power Word: Shield");
}

#[test]
fn test_add_buff_spell_id_is_correct() {
    let env = env();
    let spell_id: i32 = env
        .eval(
            r#"
            A_Admin.AddBuff(99002, "Blessing of Kings", "134920", 3600, 1)
            local aura = C_UnitAuras.GetPlayerAuraBySpellID(99002)
            return aura.spellId
            "#,
        )
        .unwrap();
    assert_eq!(spell_id, 99002);
}

#[test]
fn test_add_buff_applications_stack_count() {
    let env = env();
    let stacks: i32 = env
        .eval(
            r#"
            A_Admin.AddBuff(99003, "Serenity", "135907", 20, 5)
            local aura = C_UnitAuras.GetPlayerAuraBySpellID(99003)
            return aura.applications
            "#,
        )
        .unwrap();
    assert_eq!(stacks, 5);
}

#[test]
fn test_add_buff_duration_is_set() {
    let env = env();
    let duration: f64 = env
        .eval(
            r#"
            A_Admin.AddBuff(99004, "Shield", "135940", 45, 1)
            local aura = C_UnitAuras.GetPlayerAuraBySpellID(99004)
            return aura.duration
            "#,
        )
        .unwrap();
    assert!(
        (duration - 45.0).abs() < 0.001,
        "duration should be ~45.0, got {}",
        duration
    );
}

#[test]
fn test_add_buff_is_helpful() {
    let env = env();
    let is_helpful: bool = env
        .eval(
            r#"
            A_Admin.AddBuff(99005, "Aura", "134973", 30, 1)
            local aura = C_UnitAuras.GetPlayerAuraBySpellID(99005)
            return aura.isHelpful
            "#,
        )
        .unwrap();
    assert!(is_helpful, "AddBuff aura should be marked isHelpful = true");
}

#[test]
fn test_add_buff_icon_stored_as_numeric_id() {
    let env = env();
    // The icon field stores a numeric file data ID parsed from the icon argument.
    // Passing "134973" (a numeric string) should round-trip correctly.
    let icon: i32 = env
        .eval(
            r#"
            A_Admin.AddBuff(99006, "Shield", "134973", 30, 1)
            local aura = C_UnitAuras.GetPlayerAuraBySpellID(99006)
            return aura.icon
            "#,
        )
        .unwrap();
    assert_eq!(icon, 134973);
}

#[test]
fn test_add_buff_unknown_spell_id_returns_nil() {
    let env = env();
    let is_nil: bool = env
        .eval(
            r#"
            A_Admin.AddBuff(99007, "Shield", "134973", 30, 1)
            return C_UnitAuras.GetPlayerAuraBySpellID(99999) == nil
            "#,
        )
        .unwrap();
    assert!(is_nil, "A different spell ID should return nil");
}

#[test]
fn test_add_multiple_buffs() {
    let env = env();
    let (found1, found2): (bool, bool) = env
        .eval(
            r#"
            A_Admin.AddBuff(80001, "Buff One", "134973", 30, 1)
            A_Admin.AddBuff(80002, "Buff Two", "134973", 60, 1)
            local a1 = C_UnitAuras.GetPlayerAuraBySpellID(80001)
            local a2 = C_UnitAuras.GetPlayerAuraBySpellID(80002)
            return a1 ~= nil, a2 ~= nil
            "#,
        )
        .unwrap();
    assert!(found1, "First buff should be findable");
    assert!(found2, "Second buff should be findable");
}

// ============================================================================
// RemoveBuff
// ============================================================================

#[test]
fn test_remove_buff_makes_aura_gone() {
    let env = env();
    let is_nil: bool = env
        .eval(
            r#"
            A_Admin.AddBuff(77001, "Shield", "134973", 30, 1)
            A_Admin.RemoveBuff(77001)
            return C_UnitAuras.GetPlayerAuraBySpellID(77001) == nil
            "#,
        )
        .unwrap();
    assert!(
        is_nil,
        "RemoveBuff should make the aura unreachable by spell ID"
    );
}

#[test]
fn test_remove_buff_leaves_other_buffs_intact() {
    let env = env();
    let (removed, still_present): (bool, bool) = env
        .eval(
            r#"
            A_Admin.AddBuff(77002, "Buff A", "134973", 30, 1)
            A_Admin.AddBuff(77003, "Buff B", "134973", 30, 1)
            A_Admin.RemoveBuff(77002)
            return C_UnitAuras.GetPlayerAuraBySpellID(77002) == nil,
                   C_UnitAuras.GetPlayerAuraBySpellID(77003) ~= nil
            "#,
        )
        .unwrap();
    assert!(removed, "Removed buff should be nil");
    assert!(still_present, "Other buff should still be present");
}

#[test]
fn test_remove_buff_nonexistent_does_not_error() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.RemoveBuff(99999)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

// ============================================================================
// ClearBuffs
// ============================================================================

#[test]
fn test_clear_buffs_removes_added_buffs() {
    let env = env();
    let (gone1, gone2): (bool, bool) = env
        .eval(
            r#"
            A_Admin.AddBuff(88001, "Buff One", "134973", 30, 1)
            A_Admin.AddBuff(88002, "Buff Two", "134973", 60, 2)
            A_Admin.ClearBuffs()
            return C_UnitAuras.GetPlayerAuraBySpellID(88001) == nil,
                   C_UnitAuras.GetPlayerAuraBySpellID(88002) == nil
            "#,
        )
        .unwrap();
    assert!(gone1, "First buff should be gone after ClearBuffs");
    assert!(gone2, "Second buff should be gone after ClearBuffs");
}

#[test]
fn test_clear_buffs_results_in_no_helpful_auras() {
    let env = env();
    // After ClearBuffs, GetAuraSlots for HELPFUL should return no slots.
    let no_slots: bool = env
        .eval(
            r#"
            A_Admin.AddBuff(88003, "Buff", "134973", 30, 1)
            A_Admin.ClearBuffs()
            local token, s1 = C_UnitAuras.GetAuraSlots("player", "HELPFUL")
            return s1 == nil
            "#,
        )
        .unwrap();
    assert!(
        no_slots,
        "After ClearBuffs, there should be no HELPFUL aura slots"
    );
}

#[test]
fn test_clear_buffs_allows_adding_new_buffs_after() {
    let env = env();
    let found: bool = env
        .eval(
            r#"
            A_Admin.AddBuff(88004, "Old Buff", "134973", 30, 1)
            A_Admin.ClearBuffs()
            A_Admin.AddBuff(88005, "New Buff", "134973", 30, 1)
            return C_UnitAuras.GetPlayerAuraBySpellID(88005) ~= nil
            "#,
        )
        .unwrap();
    assert!(found, "Should be able to add new buffs after ClearBuffs");
}

#[test]
fn test_clear_buffs_does_not_error_on_empty() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.ClearBuffs()
            A_Admin.ClearBuffs()
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

// ============================================================================
// GetPlayerAuraBySpellID via global and C_UnitAuras namespace
// ============================================================================

#[test]
fn test_global_get_player_aura_by_spell_id_finds_added_buff() {
    let env = env();
    let (found, name): (bool, String) = env
        .eval(
            r#"
            A_Admin.AddBuff(55001, "Power Word: Shield", "134973", 30, 1)
            local aura = GetPlayerAuraBySpellID(55001)
            return aura ~= nil, aura and aura.name or ""
            "#,
        )
        .unwrap();
    assert!(found);
    assert_eq!(name, "Power Word: Shield");
}

#[test]
fn test_aura_data_has_expected_fields() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.AddBuff(55002, "Divine Shield", "524354", 8, 1)
            local aura = C_UnitAuras.GetPlayerAuraBySpellID(55002)
            return aura ~= nil
                and aura.name == "Divine Shield"
                and aura.spellId == 55002
                and aura.isHelpful == true
                and aura.auraInstanceID ~= nil
                and type(aura.points) == "table"
            "#,
        )
        .unwrap();
    assert!(ok, "AuraData table should have all expected fields");
}
