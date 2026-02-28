//! Tests for A_Admin combat API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetInCombat
// ============================================================================

#[test]
fn test_set_in_combat_true() {
    let env = env();
    let in_combat: bool = env
        .eval(
            r#"
            A_Admin.SetInCombat(true)
            return InCombatLockdown()
            "#,
        )
        .unwrap();
    assert!(in_combat);
}

#[test]
fn test_set_in_combat_false() {
    let env = env();
    let in_combat: bool = env
        .eval(
            r#"
            A_Admin.SetInCombat(true)
            A_Admin.SetInCombat(false)
            return InCombatLockdown()
            "#,
        )
        .unwrap();
    assert!(!in_combat);
}

#[test]
fn test_set_in_combat_unit_affecting_combat() {
    let env = env();
    let in_combat: bool = env
        .eval(
            r#"
            A_Admin.SetInCombat(true)
            return UnitAffectingCombat("player")
            "#,
        )
        .unwrap();
    assert!(in_combat);
}

#[test]
fn test_set_in_combat_false_clears_unit_affecting_combat() {
    let env = env();
    let in_combat: bool = env
        .eval(
            r#"
            A_Admin.SetInCombat(true)
            A_Admin.SetInCombat(false)
            return UnitAffectingCombat("player")
            "#,
        )
        .unwrap();
    assert!(!in_combat);
}

// ============================================================================
// SetResting
// ============================================================================

#[test]
fn test_set_resting_true() {
    let env = env();
    let resting: bool = env
        .eval(
            r#"
            A_Admin.SetResting(true)
            return IsResting()
            "#,
        )
        .unwrap();
    assert!(resting);
}

#[test]
fn test_set_resting_false() {
    let env = env();
    let resting: bool = env
        .eval(
            r#"
            A_Admin.SetResting(true)
            A_Admin.SetResting(false)
            return IsResting()
            "#,
        )
        .unwrap();
    assert!(!resting);
}

// ============================================================================
// SetCasting
// ============================================================================

#[test]
fn test_set_casting_shows_spell_name() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.SetCasting(12345, "Fireball", "Interface\\Icons\\spell_fire_fireball", 2.5)
            local n = UnitCastingInfo("player")
            return n
            "#,
        )
        .unwrap();
    assert_eq!(name, "Fireball");
}

#[test]
fn test_set_casting_spell_id_in_ninth_return() {
    let env = env();
    let spell_id: i64 = env
        .eval(
            r#"
            A_Admin.SetCasting(12345, "Fireball", "Interface\\Icons\\spell_fire_fireball", 2.5)
            local n, t, i, s, e, ts, ci, ni, sid = UnitCastingInfo("player")
            return sid
            "#,
        )
        .unwrap();
    assert_eq!(spell_id, 12345);
}

#[test]
fn test_set_casting_icon_path_in_third_return() {
    let env = env();
    let icon: String = env
        .eval(
            r#"
            A_Admin.SetCasting(12345, "Fireball", "Interface\\Icons\\spell_fire_fireball", 2.5)
            local n, t, i = UnitCastingInfo("player")
            return i
            "#,
        )
        .unwrap();
    assert_eq!(icon, "Interface\\Icons\\spell_fire_fireball");
}

#[test]
fn test_set_casting_times_in_milliseconds() {
    let env = env();
    let duration_ms: f64 = env
        .eval(
            r#"
            A_Admin.SetCasting(9001, "Holy Light", "Interface\\Icons\\holy", 1.5)
            local _, _, _, start_ms, end_ms = UnitCastingInfo("player")
            return end_ms - start_ms
            "#,
        )
        .unwrap();
    assert!(
        (duration_ms - 1500.0).abs() < 10.0,
        "cast duration should be ~1500ms, got {duration_ms}"
    );
}

// ============================================================================
// StopCasting
// ============================================================================

#[test]
fn test_stop_casting_clears_info() {
    let env = env();
    let is_nil: bool = env
        .eval(
            r#"
            A_Admin.SetCasting(12345, "Fireball", "Interface\\Icons\\spell_fire_fireball", 2.5)
            A_Admin.StopCasting()
            return UnitCastingInfo("player") == nil
            "#,
        )
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_stop_casting_without_cast_is_noop() {
    let env = env();
    let is_nil: bool = env
        .eval(
            r#"
            A_Admin.StopCasting()
            return UnitCastingInfo("player") == nil
            "#,
        )
        .unwrap();
    assert!(is_nil);
}

// ============================================================================
// SetGCD
// ============================================================================

#[test]
fn test_set_gcd_shows_on_spell_cooldown() {
    let env = env();
    let duration: f64 = env
        .eval(
            r#"
            A_Admin.SetGCD(1.5)
            local info = C_Spell.GetSpellCooldown(12345)
            return info.duration
            "#,
        )
        .unwrap();
    assert!(
        (duration - 1.5).abs() < 0.01,
        "GCD duration should be ~1.5, got {duration}"
    );
}

#[test]
fn test_set_gcd_is_enabled() {
    let env = env();
    let enabled: bool = env
        .eval(
            r#"
            A_Admin.SetGCD(1.5)
            local info = C_Spell.GetSpellCooldown(12345)
            return info.isEnabled
            "#,
        )
        .unwrap();
    assert!(enabled);
}

// ============================================================================
// SetSpellCooldown
// ============================================================================

#[test]
fn test_set_spell_cooldown_duration() {
    let env = env();
    let duration: f64 = env
        .eval(
            r#"
            A_Admin.SetSpellCooldown(12345, 8.0)
            local info = C_Spell.GetSpellCooldown(12345)
            return info.duration
            "#,
        )
        .unwrap();
    assert!(
        (duration - 8.0).abs() < 0.01,
        "spell cooldown duration should be ~8.0, got {duration}"
    );
}

#[test]
fn test_set_spell_cooldown_different_spells_independent() {
    let env = env();
    let (dur_a, dur_b): (f64, f64) = env
        .eval(
            r#"
            A_Admin.SetSpellCooldown(11111, 3.0)
            A_Admin.SetSpellCooldown(22222, 15.0)
            local a = C_Spell.GetSpellCooldown(11111)
            local b = C_Spell.GetSpellCooldown(22222)
            return a.duration, b.duration
            "#,
        )
        .unwrap();
    assert!((dur_a - 3.0).abs() < 0.01, "spell 11111 duration ~3.0, got {dur_a}");
    assert!((dur_b - 15.0).abs() < 0.01, "spell 22222 duration ~15.0, got {dur_b}");
}

#[test]
fn test_set_spell_cooldown_mod_rate_is_one() {
    let env = env();
    let mod_rate: f64 = env
        .eval(
            r#"
            A_Admin.SetSpellCooldown(12345, 8.0)
            local info = C_Spell.GetSpellCooldown(12345)
            return info.modRate
            "#,
        )
        .unwrap();
    assert!((mod_rate - 1.0).abs() < 0.001);
}
