//! Tests for spell damage/healing application.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// Damage spells
// ============================================================================

#[test]
fn test_damage_spell_reduces_enemy_health() {
    let env = env();
    let (before, after): (i32, i32) = env
        .eval(
            r#"
            A_Admin.SetTarget("Boss", 63, 1, true)
            local before = UnitHealth("target")
            CastSpellByID(35395) -- Crusader Strike
            local after = UnitHealth("target")
            return before, after
            "#,
        )
        .unwrap();
    assert!(after < before, "health should decrease: {} -> {}", before, after);
    assert_eq!(before - after, 15_000);
}

#[test]
fn test_damage_spell_blocked_on_friendly() {
    let env = env();
    let (before, after): (i32, i32) = env
        .eval(
            r#"
            A_Admin.SetTarget("Ally", 63, 1, false)
            local before = UnitHealth("target")
            CastSpellByID(35395) -- Crusader Strike on friendly
            local after = UnitHealth("target")
            return before, after
            "#,
        )
        .unwrap();
    assert_eq!(before, after, "friendly target health should not change");
}

#[test]
fn test_damage_spell_blocked_no_target() {
    let env = env();
    // Should not error when no target is set
    let hp: i32 = env
        .eval(
            r#"
            CastSpellByID(35395) -- Crusader Strike, no target
            return UnitHealth("player")
            "#,
        )
        .unwrap();
    assert!(hp > 0);
}

#[test]
fn test_damage_clamps_to_zero() {
    let env = env();
    let hp: i32 = env
        .eval(
            r#"
            A_Admin.SetTarget("Boss", 63, 1, true)
            A_Admin.SetTargetHealth(100, 100000)
            CastSpellByID(35395) -- 15000 damage on 100 HP
            return UnitHealth("target")
            "#,
        )
        .unwrap();
    assert_eq!(hp, 0, "health should clamp to 0");
}

#[test]
fn test_damage_fires_unit_health_event() {
    let env = env();
    let fired: bool = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("UNIT_HEALTH")
            f:SetScript("OnEvent", function(self, event, unit)
                if event == "UNIT_HEALTH" then fired = true end
            end)
            A_Admin.SetTarget("Boss", 63, 1, true)
            CastSpellByID(35395)
            return fired
            "#,
        )
        .unwrap();
    assert!(fired, "UNIT_HEALTH should fire on damage");
}

// ============================================================================
// Healing spells
// ============================================================================

#[test]
fn test_heal_spell_increases_friendly_health() {
    let env = env();
    let (before, after): (i32, i32) = env
        .eval(
            r#"
            A_Admin.SetTarget("Ally", 63, 1, false)
            A_Admin.SetTargetHealth(50000, 200000)
            local before = UnitHealth("target")
            CastSpellByID(85673) -- Word of Glory (instant heal)
            local after = UnitHealth("target")
            return before, after
            "#,
        )
        .unwrap();
    assert!(after > before, "health should increase: {} -> {}", before, after);
    assert_eq!(after - before, 20_000);
}

#[test]
fn test_heal_spell_no_target_heals_self() {
    let env = env();
    let (before, after): (i32, i32) = env
        .eval(
            r#"
            A_Admin.SetPlayerHealth(50000, 200000)
            local before = UnitHealth("player")
            CastSpellByID(85673) -- Word of Glory, no target
            local after = UnitHealth("player")
            return before, after
            "#,
        )
        .unwrap();
    assert!(after > before, "player health should increase: {} -> {}", before, after);
    assert_eq!(after - before, 20_000);
}

#[test]
fn test_heal_spell_enemy_target_heals_self() {
    let env = env();
    let (before, after): (i32, i32) = env
        .eval(
            r#"
            A_Admin.SetPlayerHealth(50000, 200000)
            A_Admin.SetTarget("Boss", 63, 1, true)
            local before = UnitHealth("player")
            CastSpellByID(85673) -- Word of Glory on enemy → heals self
            local after = UnitHealth("player")
            return before, after
            "#,
        )
        .unwrap();
    assert!(after > before, "player health should increase: {} -> {}", before, after);
}

#[test]
fn test_heal_clamps_to_max() {
    let env = env();
    let (hp, max): (i32, i32) = env
        .eval(
            r#"
            A_Admin.SetPlayerHealth(195000, 200000)
            CastSpellByID(85673) -- Word of Glory heals 20000
            return UnitHealth("player"), UnitHealthMax("player")
            "#,
        )
        .unwrap();
    assert_eq!(hp, max, "health should clamp to max");
}

#[test]
fn test_heal_fires_unit_health_event() {
    let env = env();
    let fired: bool = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("UNIT_HEALTH")
            f:SetScript("OnEvent", function(self, event, unit)
                if event == "UNIT_HEALTH" then fired = true end
            end)
            A_Admin.SetPlayerHealth(50000, 200000)
            CastSpellByID(85673)
            return fired
            "#,
        )
        .unwrap();
    assert!(fired, "UNIT_HEALTH should fire on heal");
}
