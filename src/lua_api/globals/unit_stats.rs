//! Unit-stat probe globals.
//!
//! Migrates 18 entries off `GLOBAL_ZERO_STUBS`:
//!
//! - `UnitArmor`, `UnitAttackPower`, `UnitCriticalStrike`, `UnitDamage`,
//!   `UnitDefense`, `UnitDodge`, `UnitParry`, `UnitSpellHaste`, `UnitStat`,
//!   `UnitResistance`, `UnitRangedAttackPower`, `UnitRangedCriticalStrike`,
//!   `UnitRangedDamage`, `UnitReaction`, `UnitHealthMax`, `UnitPowerMax`,
//!   `UnitXP`, `UnitXPMax`.
//!
//! Stats come from one of three sources depending on the unit token:
//!
//! - `"player"` / `"pet"`: `SimState.player.stats` + level + equipped
//!   item-level, wrapped into a `UnitStats` snapshot.
//! - `"target"` / `"focus"` / `"mouseover"`: the corresponding
//!   `SimState.current_target` / `focus_target` / `mouseover_target`
//!   entry, with stats derived from level (no stat model on
//!   `TargetInfo`).
//! - `"party1"`..`"party4"`: `PartyMember` entries, stats derived from
//!   level.
//!
//! Units the sim doesn't model return zero-valued stats (the retail
//! convention when `UnitExists(unit)` is false).

use crate::lua_api::game_data::{PartyMember, TargetInfo};
use crate::lua_api::globals::unit_api::parse_party_index;
use crate::lua_api::methods::{borrow_state, val_to_string};
use crate::lua_api::state::{PlayerState, SimState};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

/// Snapshot of the stats we expose for a unit. Every field is derivable
/// from `PlayerState` / `TargetInfo` / `PartyMember`; callers pick out
/// whichever subset each global needs.
#[derive(Clone, Copy)]
struct UnitStats {
    level: i32,
    health_max: i32,
    power_max: i32,
    power_type: i32,
    armor: i32,
    attack_power: i32,
    crit_rating: i32,
    haste_rating: i32,
    strength: f64,
    agility: f64,
    stamina: f64,
    intellect: f64,
    damage_min: f64,
    damage_max: f64,
    reaction: i32,
    xp: i64,
    xp_max: i64,
}

/// The Default impl is the "unit doesn't exist" fallback: zero for
/// numerics, friendly (5) for reaction. Retail returns 0 from stat
/// probes when the unit token is unknown.
impl Default for UnitStats {
    fn default() -> Self {
        Self {
            level: 0,
            health_max: 0,
            power_max: 0,
            power_type: 0,
            armor: 0,
            attack_power: 0,
            crit_rating: 0,
            haste_rating: 0,
            strength: 0.0,
            agility: 0.0,
            stamina: 0.0,
            intellect: 0.0,
            damage_min: 0.0,
            damage_max: 0.0,
            reaction: 4, // neutral
            xp: 0,
            xp_max: 0,
        }
    }
}

fn player_stats(player: &PlayerState) -> UnitStats {
    let level_f = player.level as f64;
    let ap = (player.stats.strength + player.stats.agility) + level_f * 10.0;
    let weapon_low = 50.0 + level_f * 5.0;
    let weapon_high = 80.0 + level_f * 7.0;
    UnitStats {
        level: player.level,
        health_max: player.health_max,
        power_max: player.power_max,
        power_type: player.power_type,
        armor: player.stats.armor,
        attack_power: ap as i32,
        crit_rating: player.stats.crit_rating,
        haste_rating: player.stats.haste_rating,
        strength: player.stats.strength,
        agility: player.stats.agility,
        stamina: player.stats.stamina,
        intellect: player.stats.intellect,
        damage_min: weapon_low + ap * 0.1,
        damage_max: weapon_high + ap * 0.1,
        reaction: 5, // friendly to self
        xp: player.xp,
        xp_max: player.xp_max,
    }
}

fn target_stats(target: &TargetInfo) -> UnitStats {
    let level_f = target.level.max(1) as f64;
    let ap = level_f * 80.0;
    UnitStats {
        level: target.level,
        health_max: target.health_max,
        power_max: target.power_max,
        power_type: target.power_type,
        armor: (level_f * 150.0) as i32,
        attack_power: ap as i32,
        crit_rating: (level_f * 4.0) as i32,
        haste_rating: (level_f * 3.0) as i32,
        strength: level_f * 8.0,
        agility: level_f * 8.0,
        stamina: level_f * 15.0,
        intellect: level_f * 8.0,
        damage_min: level_f * 20.0,
        damage_max: level_f * 35.0,
        reaction: target.reaction,
        xp: 0,
        xp_max: 0,
    }
}

fn party_stats(member: &PartyMember) -> UnitStats {
    let level_f = member.level.max(1) as f64;
    let ap = level_f * 120.0;
    UnitStats {
        level: member.level,
        health_max: member.health_max,
        power_max: member.power_max,
        power_type: member.power_type,
        armor: (level_f * 180.0) as i32,
        attack_power: ap as i32,
        crit_rating: (level_f * 5.0) as i32,
        haste_rating: (level_f * 4.0) as i32,
        strength: level_f * 10.0,
        agility: level_f * 10.0,
        stamina: level_f * 20.0,
        intellect: level_f * 10.0,
        damage_min: level_f * 25.0,
        damage_max: level_f * 45.0,
        reaction: 5, // friendly
        xp: 0,
        xp_max: 0,
    }
}

fn lookup_unit_stats(sim: &SimState, unit: &str) -> UnitStats {
    match unit {
        "player" | "pet" => player_stats(&sim.player),
        "target" => sim
            .current_target
            .as_ref()
            .map(target_stats)
            .unwrap_or_default(),
        "focus" => sim
            .current_focus
            .as_ref()
            .map(target_stats)
            .unwrap_or_default(),
        // The sim has no mouseover target model; fall back to zero stats
        // to match retail's "unit doesn't exist" behaviour.
        "mouseover" => UnitStats::default(),
        other => parse_party_index(other)
            .and_then(|index| sim.party_members.get(index))
            .map(party_stats)
            .unwrap_or_default(),
    }
}

fn unit_token(state: &LuaState) -> String {
    val_to_string(state, stack_val(state, 1)).unwrap_or_else(|| "player".to_string())
}

fn stats_for(state: &LuaState) -> UnitStats {
    let unit = unit_token(state);
    let sim = borrow_state(state).expect("sim state should exist");
    lookup_unit_stats(&sim, &unit)
}

// ── Stat probes ──────────────────────────────────────────────────────────────

/// `UnitArmor(unit)` — retail: `(base, armor, posBuff, negBuff)`.
/// We model armor as a single value and report it as both `base` and
/// `armor`, with zero buffs.
fn unit_armor(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(stats.armor as f64));
    state.push(Val::Num(stats.armor as f64));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(4)
}

/// `UnitAttackPower(unit)` — retail: `(base, posBuff, negBuff)`.
fn unit_attack_power(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(stats.attack_power as f64));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(3)
}

/// `UnitRangedAttackPower(unit)` — same retail shape as `UnitAttackPower`.
fn unit_ranged_attack_power(state: &mut LuaState) -> LuaResult<u32> {
    unit_attack_power(state)
}

/// `UnitCriticalStrike(unit)` — retail returns a single percent-chance
/// number. Approximates crit % from the rating.
fn unit_critical_strike(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(rating_to_percent(stats.crit_rating)));
    Ok(1)
}

/// `UnitRangedCriticalStrike(unit)` — alias for melee crit in this sim.
fn unit_ranged_critical_strike(state: &mut LuaState) -> LuaResult<u32> {
    unit_critical_strike(state)
}

/// `UnitSpellHaste(unit)` — retail returns a percent-chance number from
/// the player's haste rating.
fn unit_spell_haste(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(rating_to_percent(stats.haste_rating)));
    Ok(1)
}

/// `UnitDamage(unit)` — retail: `(minDmg, maxDmg, minOffHand, maxOffHand,
/// physicalBonusPos, physicalBonusNeg, percent)`. Off-hand and bonuses
/// are zero in this sim.
fn unit_damage(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(stats.damage_min));
    state.push(Val::Num(stats.damage_max));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(1.0)); // percent (multiplier)
    Ok(7)
}

/// `UnitRangedDamage(unit)` — same shape as `UnitDamage`.
fn unit_ranged_damage(state: &mut LuaState) -> LuaResult<u32> {
    unit_damage(state)
}

/// `UnitDefense(unit)` — retail's old API, returns skill rating. We
/// approximate as `level * 5` (the pre-MoP formula).
fn unit_defense(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num((stats.level * 5).max(0) as f64));
    Ok(1)
}

/// `UnitDodge(unit)` — retail returns a percent-chance number.
fn unit_dodge(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(base_avoidance_percent(stats.agility)));
    Ok(1)
}

/// `UnitParry(unit)` — retail returns a percent-chance number.
fn unit_parry(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(base_avoidance_percent(stats.strength)));
    Ok(1)
}

/// `UnitReaction(unit, otherUnit)` — retail returns 1-8 reaction. We
/// always compare relative to the player, so `otherUnit` is informational
/// only (sim has no per-pair reaction table).
fn unit_reaction(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(stats.reaction as f64));
    Ok(1)
}

/// `UnitHealthMax(unit)` — single-value shortcut.
fn unit_health_max(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(stats.health_max as f64));
    Ok(1)
}

/// `UnitPowerMax(unit)` — retail: `(maxPower, powerType)`.
fn unit_power_max(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(stats.power_max as f64));
    state.push(Val::Num(stats.power_type as f64));
    Ok(2)
}

/// `UnitXP(unit)` — player-only in retail; returns 0 for other units.
fn unit_xp(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(stats.xp as f64));
    Ok(1)
}

/// `UnitXPMax(unit)` — player-only in retail; returns 0 for other units.
fn unit_xp_max(state: &mut LuaState) -> LuaResult<u32> {
    let stats = stats_for(state);
    state.push(Val::Num(stats.xp_max as f64));
    Ok(1)
}

/// `UnitStat(unit, statIndex)` — retail: `(stat, base, positive, negative)`.
/// `statIndex` 1 = Strength, 2 = Agility, 3 = Stamina, 4 = Intellect,
/// 5 = Spirit (unused in modern WoW — falls through to 0).
fn unit_stat(state: &mut LuaState) -> LuaResult<u32> {
    let stat_index = match stack_val(state, 2) {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    let stats = stats_for(state);
    let value = match stat_index {
        1 => stats.strength,
        2 => stats.agility,
        3 => stats.stamina,
        4 => stats.intellect,
        _ => 0.0,
    };
    state.push(Val::Num(value));
    state.push(Val::Num(value));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(4)
}

/// `UnitResistance(unit, resistanceIndex)` — retail: `(base, resistance,
/// positive, negative)`. The sim doesn't model resistances; always 0.
fn unit_resistance(state: &mut LuaState) -> LuaResult<u32> {
    let _ = state;
    for _ in 0..4 {
        state.push(Val::Num(0.0));
    }
    Ok(4)
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Approximate rating-to-percent conversion used by the sim's stat
/// model. Retail uses per-expansion divisors; 180 is an acceptable
/// middle-ground for The War Within crit/haste without hard-coding a
/// DR curve.
fn rating_to_percent(rating: i32) -> f64 {
    (rating as f64) / 180.0
}

fn base_avoidance_percent(primary_stat: f64) -> f64 {
    5.0 + primary_stat / 100.0
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "UnitArmor", unit_armor)?;
    LuaApiMut::register_function(lua, "UnitAttackPower", unit_attack_power)?;
    LuaApiMut::register_function(lua, "UnitRangedAttackPower", unit_ranged_attack_power)?;
    LuaApiMut::register_function(lua, "UnitCriticalStrike", unit_critical_strike)?;
    LuaApiMut::register_function(lua, "UnitRangedCriticalStrike", unit_ranged_critical_strike)?;
    LuaApiMut::register_function(lua, "UnitSpellHaste", unit_spell_haste)?;
    LuaApiMut::register_function(lua, "UnitDamage", unit_damage)?;
    LuaApiMut::register_function(lua, "UnitRangedDamage", unit_ranged_damage)?;
    LuaApiMut::register_function(lua, "UnitDefense", unit_defense)?;
    LuaApiMut::register_function(lua, "UnitDodge", unit_dodge)?;
    LuaApiMut::register_function(lua, "UnitParry", unit_parry)?;
    LuaApiMut::register_function(lua, "UnitReaction", unit_reaction)?;
    LuaApiMut::register_function(lua, "UnitHealthMax", unit_health_max)?;
    LuaApiMut::register_function(lua, "UnitPowerMax", unit_power_max)?;
    LuaApiMut::register_function(lua, "UnitXP", unit_xp)?;
    LuaApiMut::register_function(lua, "UnitXPMax", unit_xp_max)?;
    LuaApiMut::register_function(lua, "UnitStat", unit_stat)?;
    LuaApiMut::register_function(lua, "UnitResistance", unit_resistance)?;
    Ok(())
}
