//! Integration tests for `src/lua_api/globals/missing_surface/c_spell.rs`.
//!
//! Covers all 10 handlers:
//!   GetSpellInfo, GetSpellCooldown, GetMountFromSpell,
//!   GetVisibilityInfo, IsPriorityAura, IsSelfBuff, IsSpellUsable,
//!   TargetSpellIsEnchanting, TargetSpellJumpsUpgradeTrack,
//!   TargetSpellReplacesBonusTree.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── GetSpellInfo ─────────────────────────────────────────────────────────────

#[test]
fn test_get_spell_info_known_returns_table() {
    let env = env();
    // Spell 116 = Frostbolt (exists in data/spells.rs)
    let name: String = env
        .eval("local info = C_Spell.GetSpellInfo(116); return info.name")
        .unwrap();
    assert_eq!(name, "Frostbolt");
}

#[test]
fn test_get_spell_info_has_required_fields() {
    let env = env();
    let (icon, spell_id, cast_time): (i64, i64, f64) = env
        .eval(
            "local info = C_Spell.GetSpellInfo(116)
             return info.iconID, info.spellID, info.castTime",
        )
        .unwrap();
    assert!(icon > 0, "iconID should be nonzero, got {icon}");
    assert_eq!(spell_id, 116);
    assert!((cast_time - 0.0).abs() < 0.001);
}

#[test]
fn test_get_spell_info_unknown_returns_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Spell.GetSpellInfo(999999999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_get_spell_info_original_icon_matches_icon() {
    let env = env();
    let matches: bool = env
        .eval(
            "local info = C_Spell.GetSpellInfo(116)
             return info.iconID == info.originalIconID",
        )
        .unwrap();
    assert!(matches);
}

#[test]
fn test_get_spell_description_resolves_spell_placeholders() {
    let env = env();
    let desc: String = env
        .eval("return C_Spell.GetSpellDescription(31935)")
        .unwrap();

    assert!(
        desc.contains("25000 Holy damage"),
        "spell description should resolve damage placeholders, got: {desc}"
    );
    assert!(
        !desc.contains('$'),
        "spell description should not expose raw Blizzard placeholders, got: {desc}"
    );
}

// ── GetSpellCooldown ─────────────────────────────────────────────────────────

#[test]
fn test_get_spell_cooldown_returns_table() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Spell.GetSpellCooldown(12345)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_get_spell_cooldown_no_cooldown_has_zero_duration() {
    let env = env();
    let duration: f64 = env
        .eval(
            "local info = C_Spell.GetSpellCooldown(12345)
             return info.duration",
        )
        .unwrap();
    assert!(
        (duration - 0.0).abs() < 0.01,
        "fresh spell should have duration 0, got {duration}"
    );
}

#[test]
fn test_get_spell_cooldown_is_enabled_true() {
    let env = env();
    let enabled: bool = env
        .eval(
            "local info = C_Spell.GetSpellCooldown(12345)
             return info.isEnabled",
        )
        .unwrap();
    assert!(enabled);
}

#[test]
fn test_get_spell_cooldown_mod_rate_is_one() {
    let env = env();
    let mod_rate: f64 = env
        .eval(
            "local info = C_Spell.GetSpellCooldown(12345)
             return info.modRate",
        )
        .unwrap();
    assert!((mod_rate - 1.0).abs() < 0.001);
}

#[test]
fn test_get_spell_cooldown_active_after_set() {
    let env = env();
    let (duration, is_active): (f64, bool) = env
        .eval(
            "A_Admin.SetSpellCooldown(42, 8.0)
             local info = C_Spell.GetSpellCooldown(42)
             return info.duration, info.isActive",
        )
        .unwrap();
    assert!(
        (duration - 8.0).abs() < 0.01,
        "cooldown duration should be 8.0, got {duration}"
    );
    assert!(is_active, "isActive should be true when duration > 0");
}

// ── GetMountFromSpell ─────────────────────────────────────────────────────────

#[test]
fn test_get_mount_from_spell_known_mount_spell() {
    let env = env();
    // Mount ID 6 ("Brown Horse") uses spell_id 458 in state_defaults.rs
    let mount_id: i64 = env.eval("return C_Spell.GetMountFromSpell(458)").unwrap();
    assert_eq!(mount_id, 6);
}

#[test]
fn test_get_mount_from_spell_unknown_returns_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Spell.GetMountFromSpell(999999) == nil")
        .unwrap();
    assert!(is_nil);
}

// ── GetVisibilityInfo ─────────────────────────────────────────────────────────

#[test]
fn test_get_visibility_info_returns_three_booleans() {
    let env = env();
    let (has_custom, always_show_mine, show_for_spec): (bool, bool, bool) = env
        .eval(
            "local a, b, c = C_Spell.GetVisibilityInfo(116, 1)
             return a, b, c",
        )
        .unwrap();
    assert!(!has_custom, "hasCustom should be false");
    assert!(always_show_mine, "alwaysShowMine should be true");
    assert!(!show_for_spec, "showForMySpec should be false");
}

// ── IsPriorityAura ────────────────────────────────────────────────────────────

#[test]
fn test_is_priority_aura_returns_false() {
    let env = env();
    let result: bool = env.eval("return C_Spell.IsPriorityAura(116)").unwrap();
    assert!(!result);
}

// ── IsSelfBuff ────────────────────────────────────────────────────────────────

#[test]
fn test_is_self_buff_true_for_self_target_spell() {
    let env = env();
    // Spell 1272138 "Keen Edge" has implicit_target=1 (self)
    let result: bool = env.eval("return C_Spell.IsSelfBuff(1272138)").unwrap();
    assert!(result, "spell with implicit_target=1 should be a self-buff");
}

#[test]
fn test_is_self_buff_false_for_enemy_spell() {
    let env = env();
    // Spell 116 "Frostbolt" has implicit_target=6 (enemy)
    let result: bool = env.eval("return C_Spell.IsSelfBuff(116)").unwrap();
    assert!(!result, "enemy-targeted spell should not be a self-buff");
}

#[test]
fn test_is_self_buff_false_for_unknown_spell() {
    let env = env();
    let result: bool = env.eval("return C_Spell.IsSelfBuff(999999999)").unwrap();
    assert!(!result);
}

// ── IsSpellUsable ─────────────────────────────────────────────────────────────

#[test]
fn test_is_spell_usable_false_for_unknown_spell() {
    let env = env();
    let (usable, insufficient): (bool, bool) = env
        .eval(
            "local u, ip = C_Spell.IsSpellUsable(99999)
             return u, ip",
        )
        .unwrap();
    assert!(!usable, "unknown spell should not be usable");
    assert!(!insufficient);
}

#[test]
fn test_is_spell_usable_true_for_known_spell() {
    let env = env();
    env.state().borrow_mut().known_spells.insert(12345);
    let (usable, insufficient): (bool, bool) = env
        .eval(
            "local u, ip = C_Spell.IsSpellUsable(12345)
             return u, ip",
        )
        .unwrap();
    assert!(usable, "known spell should be usable");
    assert!(!insufficient);
}

// ── TargetSpell stubs ─────────────────────────────────────────────────────────

#[test]
fn test_target_spell_is_enchanting_false() {
    let env = env();
    let result: bool = env
        .eval("return C_Spell.TargetSpellIsEnchanting()")
        .unwrap();
    assert!(!result);
}

#[test]
fn test_target_spell_jumps_upgrade_track_false() {
    let env = env();
    let result: bool = env
        .eval("return C_Spell.TargetSpellJumpsUpgradeTrack()")
        .unwrap();
    assert!(!result);
}

#[test]
fn test_target_spell_replaces_bonus_tree_false() {
    let env = env();
    let result: bool = env
        .eval("return C_Spell.TargetSpellReplacesBonusTree()")
        .unwrap();
    assert!(!result);
}
