//! Integration tests for `src/lua_api/globals/spell_state_probes.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── IsCurrentSpell / IsCurrentAction ──────────────────────────────────────────

#[test]
fn is_current_spell_matches_active_cast() {
    let env = env();
    env.exec("CastSpellByID(42)").unwrap();
    let b: bool = env.eval("return IsCurrentSpell(42)").unwrap();
    assert!(b);
}

#[test]
fn is_current_spell_false_for_different_id() {
    let env = env();
    env.exec("CastSpellByID(42)").unwrap();
    let b: bool = env.eval("return IsCurrentSpell(7)").unwrap();
    assert!(!b);
}

#[test]
fn is_current_action_matches_action_bar_to_cast() {
    let env = env();
    env.state().borrow_mut().action_bars.insert(99, 42);
    env.exec("CastSpellByID(42)").unwrap();
    let b: bool = env.eval("return IsCurrentAction(99)").unwrap();
    assert!(b);
}

// ── IsSpellKnown ──────────────────────────────────────────────────────────────

#[test]
fn is_spell_known_reads_state_set() {
    let env = env();
    env.state().borrow_mut().known_spells.insert(12345);
    let b: bool = env.eval("return IsSpellKnown(12345)").unwrap();
    assert!(b);
}

#[test]
fn is_spell_known_false_for_unknown_id() {
    let env = env();
    let b: bool = env.eval("return IsSpellKnown(99999)").unwrap();
    assert!(!b);
}

#[test]
fn is_spell_known_or_overrides_alias() {
    let env = env();
    env.state().borrow_mut().known_spells.insert(1);
    let b: bool = env.eval("return IsSpellKnownOrOverridesKnown(1)").unwrap();
    assert!(b);
}

// ── IsSpellInRange / IsItemInRange ────────────────────────────────────────────

#[test]
fn is_spell_in_range_requires_known_spell_and_visible_unit() {
    let env = env();
    env.state().borrow_mut().known_spells.insert(7);
    // No target → false.
    let b: bool = env.eval(r#"return IsSpellInRange(7, "target")"#).unwrap();
    assert!(!b);
    // Player is always reachable; spell is known → true.
    let b: bool = env.eval(r#"return IsSpellInRange(7, "player")"#).unwrap();
    assert!(b);
}

#[test]
fn is_item_in_range_follows_unit_reachability() {
    let env = env();
    let b: bool = env.eval(r#"return IsItemInRange(6948, "player")"#).unwrap();
    assert!(b);
    let b: bool = env.eval(r#"return IsItemInRange(6948, "")"#).unwrap();
    assert!(!b);
}

// ── IsUsableSpell ─────────────────────────────────────────────────────────────

#[test]
fn is_usable_spell_true_when_known_and_no_cooldown() {
    let env = env();
    env.state().borrow_mut().known_spells.insert(99);
    let (usable, no_mana): (bool, bool) = env.eval("return IsUsableSpell(99)").unwrap();
    assert!(usable);
    assert!(!no_mana);
}

#[test]
fn is_usable_spell_false_on_cooldown() {
    let env = env();
    {
        use wow_ui_sim::lua_api::state::SpellCooldownState;
        let mut st = env.state().borrow_mut();
        st.known_spells.insert(10);
        st.spell_cooldowns.insert(
            10,
            SpellCooldownState {
                start: 0.0,
                duration: 3.0,
            },
        );
    }
    let (usable, _): (bool, bool) = env.eval("return IsUsableSpell(10)").unwrap();
    assert!(!usable);
}

// ── IsHarmfulSpell / IsHelpfulSpell ───────────────────────────────────────────

#[test]
fn is_harmful_spell_reads_state_set() {
    let env = env();
    env.state().borrow_mut().harmful_spells.insert(100);
    let b: bool = env.eval("return IsHarmfulSpell(100)").unwrap();
    assert!(b);
}

#[test]
fn is_helpful_spell_reads_state_set() {
    let env = env();
    env.state().borrow_mut().helpful_spells.insert(200);
    let b: bool = env.eval("return IsHelpfulSpell(200)").unwrap();
    assert!(b);
}

// ── HasPetSpells ──────────────────────────────────────────────────────────────

#[test]
fn has_pet_spells_nil_when_empty() {
    let env = env();
    let is_nil: bool = env.eval("return HasPetSpells() == nil").unwrap();
    assert!(is_nil);
}

#[test]
fn has_pet_spells_returns_count_when_populated() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.pet_spells.insert(1);
        st.pet_spells.insert(2);
        st.pet_spells.insert(3);
    }
    let n: i64 = env.eval("return HasPetSpells()").unwrap();
    assert_eq!(n, 3);
}
