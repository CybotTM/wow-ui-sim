//! Integration tests for `src/lua_api/globals/cooldown_probes.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::SpellCooldownState;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {actual} to be close to {expected}"
    );
}

// ── GetSpellCooldown ──────────────────────────────────────────────────────────

#[test]
fn get_spell_cooldown_zero_when_no_cooldown() {
    let env = env();
    let (start, duration, enable, mod_rate): (f64, f64, i32, f64) =
        env.eval("return GetSpellCooldown(12345)").unwrap();
    assert_eq!(start, 0.0);
    assert_eq!(duration, 0.0);
    assert_eq!(enable, 1);
    assert_eq!(mod_rate, 1.0);
}

#[test]
fn get_spell_cooldown_reads_spell_cooldowns_entry() {
    let env = env();
    let now = env.state().borrow().start_time.elapsed().as_secs_f64();
    env.state().borrow_mut().spell_cooldowns.insert(
        12345,
        SpellCooldownState {
            start: now,
            duration: 30.0,
        },
    );
    let (start, duration, _enable, _mod): (f64, f64, i32, f64) =
        env.eval("return GetSpellCooldown(12345)").unwrap();
    assert!(
        (start - now).abs() < 0.5,
        "start should match the seeded cooldown"
    );
    assert_close(duration, 30.0);
}

// ── GetActionCooldown ─────────────────────────────────────────────────────────

#[test]
fn get_action_cooldown_resolves_bar_slot_through_spell() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.action_bars.insert(1, 12345);
        let now = st.start_time.elapsed().as_secs_f64();
        st.spell_cooldowns.insert(
            12345,
            SpellCooldownState {
                start: now,
                duration: 15.0,
            },
        );
    }
    let (_start, duration, enable, mod_rate): (f64, f64, i32, f64) =
        env.eval("return GetActionCooldown(1)").unwrap();
    assert_close(duration, 15.0);
    assert_eq!(enable, 1);
    assert_eq!(mod_rate, 1.0);
}

#[test]
fn get_action_cooldown_empty_slot_returns_zero() {
    let env = env();
    let (start, duration, enable, _mod): (f64, f64, i32, f64) =
        env.eval("return GetActionCooldown(99)").unwrap();
    assert_eq!(start, 0.0);
    assert_eq!(duration, 0.0);
    assert_eq!(enable, 1);
}

// ── GetInventoryItemCooldown ──────────────────────────────────────────────────

#[test]
fn get_inventory_item_cooldown_zero_when_no_cooldown() {
    let env = env();
    let (start, duration, enable): (f64, f64, i32) = env
        .eval(r#"return GetInventoryItemCooldown("player", 13)"#)
        .unwrap();
    assert_eq!(start, 0.0);
    assert_eq!(duration, 0.0);
    assert_eq!(enable, 1);
}

#[test]
fn get_inventory_item_cooldown_reads_state_entry() {
    let env = env();
    let now = env.state().borrow().start_time.elapsed().as_secs_f64();
    env.state().borrow_mut().inventory_item_cooldowns.insert(
        13,
        SpellCooldownState {
            start: now,
            duration: 120.0,
        },
    );
    let (start, duration, enable): (f64, f64, i32) = env
        .eval(r#"return GetInventoryItemCooldown("player", 13)"#)
        .unwrap();
    assert!((start - now).abs() < 0.5);
    assert_eq!(duration, 120.0);
    assert_eq!(enable, 1);
}

// ── GetSpellBonusDamage / GetSpellBonusHealing ────────────────────────────────

#[test]
fn spell_bonus_damage_and_healing_share_intellect_bucket() {
    let env = env();
    env.state().borrow_mut().player.stats.intellect = 2500.0;
    let (damage, healing): (f64, f64) = env
        .eval("return GetSpellBonusDamage(1), GetSpellBonusHealing()")
        .unwrap();
    assert_eq!(damage, 2500.0);
    assert_eq!(healing, 2500.0);
}

// ── GetSpellAutocast ──────────────────────────────────────────────────────────

#[test]
fn get_spell_autocast_always_false() {
    let env = env();
    let (castable, casting): (bool, bool) = env.eval("return GetSpellAutocast(12345)").unwrap();
    assert!(!castable);
    assert!(!casting);
}

// ── GetSpellLevelLearned ──────────────────────────────────────────────────────

#[test]
fn get_spell_level_learned_zero_for_unknown_spell() {
    let env = env();
    let level: i32 = env.eval("return GetSpellLevelLearned(99999)").unwrap();
    assert_eq!(level, 0);
}

#[test]
fn get_spell_level_learned_one_for_known_spell() {
    let env = env();
    env.state().borrow_mut().known_spells.insert(12345);
    let level: i32 = env.eval("return GetSpellLevelLearned(12345)").unwrap();
    assert_eq!(level, 1);
}
