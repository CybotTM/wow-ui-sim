//! A_Admin namespace for simulator state control from Lua.
//!
//! Provides functions to set player stats, combat state, targeting, party,
//! buffs, zones, economy, collections, PvP, guild, and fire events.
//! Intended for addon test scripts and UI development.

use crate::event::{Event, EventArg};
use crate::lua_api::game_data::{AuraInfo, CastingState, PartyMember, SpellCooldownState, TargetInfo};
use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value, Variadic};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

/// Register the A_Admin namespace on Lua globals.
pub fn register_admin_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let admin = lua.create_table()?;

    register_identity_api(lua, &admin, Rc::clone(&state))?;
    register_combat_api(lua, &admin, Rc::clone(&state))?;
    register_health_power_api(lua, &admin, Rc::clone(&state))?;
    register_targeting_api(lua, &admin, Rc::clone(&state))?;
    register_party_api(lua, &admin, Rc::clone(&state))?;
    register_movement_api(lua, &admin, Rc::clone(&state))?;
    register_spec_talent_api(lua, &admin, Rc::clone(&state))?;
    register_buff_api(lua, &admin, Rc::clone(&state))?;
    register_zone_api(lua, &admin, Rc::clone(&state))?;
    register_economy_api(lua, &admin, Rc::clone(&state))?;
    register_collection_api(lua, &admin, Rc::clone(&state))?;
    register_pvp_guild_api(lua, &admin, Rc::clone(&state))?;
    register_event_api(lua, &admin, Rc::clone(&state))?;
    register_vault_api(lua, &admin, Rc::clone(&state))?;
    register_action_bar_api(lua, &admin, Rc::clone(&state))?;
    register_bag_api(lua, &admin, Rc::clone(&state))?;
    super::admin_encounter::register_encounter_api(lua, &admin, Rc::clone(&state))?;

    lua.globals().set("A_Admin", admin)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Player identity
// ---------------------------------------------------------------------------

fn register_identity_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetPlayerName", {
        let s = Rc::clone(&state);
        move |_, name: String| { s.borrow_mut().player.name = name; Ok(()) }
    })?;
    set_fn(lua, t, "SetPlayerClass", {
        let s = Rc::clone(&state);
        move |_, class_index: i32| { s.borrow_mut().player.class_index = class_index; Ok(()) }
    })?;
    set_fn(lua, t, "SetPlayerRace", {
        let s = Rc::clone(&state);
        move |_, race_index: i32| { s.borrow_mut().player.race_index = race_index as usize; Ok(()) }
    })?;
    set_fn(lua, t, "SetPlayerLevel", {
        let s = Rc::clone(&state);
        move |_, level: i32| { s.borrow_mut().player.level = level; Ok(()) }
    })?;
    set_fn(lua, t, "SetPlayerSex", {
        let s = Rc::clone(&state);
        move |_, sex: i32| { s.borrow_mut().player.sex = sex; Ok(()) }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Combat
// ---------------------------------------------------------------------------

fn register_combat_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetInCombat", {
        let s = Rc::clone(&state);
        move |_, v: bool| { s.borrow_mut().player.in_combat = v; Ok(()) }
    })?;
    set_fn(lua, t, "SetResting", {
        let s = Rc::clone(&state);
        move |_, v: bool| { s.borrow_mut().player.is_resting = v; Ok(()) }
    })?;
    set_fn(lua, t, "SetCasting", {
        let s = Rc::clone(&state);
        move |_, (spell_id, spell_name, icon_path, duration): (u32, String, String, f64)| {
            let mut st = s.borrow_mut();
            let now = st.start_time.elapsed().as_secs_f64();
            let cast_id = st.next_cast_id;
            st.next_cast_id += 1;
            st.casting = Some(CastingState {
                spell_id,
                spell_name,
                icon_path,
                start_time: now,
                end_time: now + duration,
                cast_id,
            });
            Ok(())
        }
    })?;
    set_fn(lua, t, "StopCasting", {
        let s = Rc::clone(&state);
        move |_, ()| { s.borrow_mut().casting = None; Ok(()) }
    })?;
    set_fn(lua, t, "SetGCD", {
        let s = Rc::clone(&state);
        move |_, duration: f64| {
            let now = s.borrow().start_time.elapsed().as_secs_f64();
            s.borrow_mut().gcd = Some((now, duration));
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetSpellCooldown", {
        let s = Rc::clone(&state);
        move |_, (spell_id, duration): (u32, f64)| {
            let now = s.borrow().start_time.elapsed().as_secs_f64();
            s.borrow_mut().spell_cooldowns.insert(spell_id, SpellCooldownState { start: now, duration });
            Ok(())
        }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Health & Power
// ---------------------------------------------------------------------------

fn register_health_power_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetPlayerHealth", {
        let s = Rc::clone(&state);
        move |_, (cur, max): (i32, i32)| {
            let mut st = s.borrow_mut();
            st.player.health = cur;
            st.player.health_max = max;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetPlayerPower", {
        let s = Rc::clone(&state);
        move |_, (cur, max, power_type): (i32, i32, Option<i32>)| {
            let mut st = s.borrow_mut();
            st.player.power = cur;
            st.player.power_max = max;
            if let Some(pt) = power_type { st.player.power_type = pt; }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetTargetHealth", {
        let s = Rc::clone(&state);
        move |_, (cur, max): (i32, i32)| {
            let mut st = s.borrow_mut();
            if let Some(t) = st.current_target.as_mut() {
                t.health = cur;
                t.health_max = max;
            }
            Ok(())
        }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Targeting
// ---------------------------------------------------------------------------

fn register_targeting_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetTarget", {
        let s = Rc::clone(&state);
        move |_, (name, level, class_index, is_enemy): (String, i32, i32, bool)| {
            s.borrow_mut().current_target = Some(make_target_info("target", &name, level, class_index, is_enemy));
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearTarget", {
        let s = Rc::clone(&state);
        move |_, ()| { s.borrow_mut().current_target = None; Ok(()) }
    })?;
    set_fn(lua, t, "SetFocus", {
        let s = Rc::clone(&state);
        move |_, (name, level, class_index, is_enemy): (String, i32, i32, bool)| {
            s.borrow_mut().current_focus = Some(make_target_info("focus", &name, level, class_index, is_enemy));
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearFocus", {
        let s = Rc::clone(&state);
        move |_, ()| { s.borrow_mut().current_focus = None; Ok(()) }
    })?;
    set_fn(lua, t, "SetTargetPower", {
        let s = Rc::clone(&state);
        move |_, (cur, max, power_type): (i32, i32, Option<i32>)| {
            let mut st = s.borrow_mut();
            if let Some(t) = st.current_target.as_mut() {
                t.power = cur;
                t.power_max = max;
                if let Some(pt) = power_type { t.power_type = pt; }
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetFocusPower", {
        let s = Rc::clone(&state);
        move |_, (cur, max, power_type): (i32, i32, Option<i32>)| {
            let mut st = s.borrow_mut();
            if let Some(f) = st.current_focus.as_mut() {
                f.power = cur;
                f.power_max = max;
                if let Some(pt) = power_type { f.power_type = pt; }
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetTargetType", {
        let s = Rc::clone(&state);
        move |_, (classification, creature_type, reaction): (Option<String>, Option<String>, Option<i32>)| {
            let mut st = s.borrow_mut();
            if let Some(t) = st.current_target.as_mut() {
                if let Some(c) = classification { t.classification = c; }
                if let Some(ct) = creature_type { t.creature_type = ct; }
                if let Some(r) = reaction { t.reaction = r; }
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetFocusType", {
        let s = Rc::clone(&state);
        move |_, (classification, creature_type, reaction): (Option<String>, Option<String>, Option<i32>)| {
            let mut st = s.borrow_mut();
            if let Some(f) = st.current_focus.as_mut() {
                if let Some(c) = classification { f.classification = c; }
                if let Some(ct) = creature_type { f.creature_type = ct; }
                if let Some(r) = reaction { f.reaction = r; }
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetFocusHealth", {
        let s = Rc::clone(&state);
        move |_, (cur, max): (i32, i32)| {
            let mut st = s.borrow_mut();
            if let Some(f) = st.current_focus.as_mut() {
                f.health = cur;
                f.health_max = max;
            }
            Ok(())
        }
    })?;
    Ok(())
}

/// Build a TargetInfo from admin-supplied parameters.
fn make_target_info(unit_id: &str, name: &str, level: i32, class_index: i32, is_enemy: bool) -> TargetInfo {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let guid = if is_enemy {
        format!("Creature-0-0-0-0-0-{}", nanos % 1_000_000)
    } else {
        format!("Player-0000-{:08}", nanos % 100_000_000)
    };
    TargetInfo {
        unit_id: unit_id.to_string(),
        name: name.to_string(),
        class_index,
        level,
        health: 100_000,
        health_max: 100_000,
        power: 50_000,
        power_max: 100_000,
        power_type: 0,
        power_type_name: "MANA".to_string(),
        is_player: !is_enemy,
        is_enemy,
        guid,
        classification: if is_enemy { "normal".to_string() } else { "normal".to_string() },
        creature_type: if is_enemy { "Humanoid".to_string() } else { "Humanoid".to_string() },
        reaction: if is_enemy { 2 } else { 5 },
    }
}

// ---------------------------------------------------------------------------
// Party
// ---------------------------------------------------------------------------

fn register_party_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetPartySize", {
        let s = Rc::clone(&state);
        move |_, n: i32| {
            let mut st = s.borrow_mut();
            let n = n.max(0) as usize;
            while st.party_members.len() < n {
                st.party_members.push(default_party_member());
            }
            st.party_members.truncate(n);
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetPartyMember", {
        let s = Rc::clone(&state);
        move |_, (idx, name, class_index, level): (i32, String, i32, i32)| {
            let mut st = s.borrow_mut();
            let i = (idx - 1) as usize;
            if let Some(m) = st.party_members.get_mut(i) {
                m.name = name;
                m.class_index = class_index;
                m.level = level;
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetPartyMemberHealth", {
        let s = Rc::clone(&state);
        move |_, (idx, cur, max): (i32, i32, i32)| {
            let mut st = s.borrow_mut();
            if let Some(m) = st.party_members.get_mut((idx - 1) as usize) {
                m.health = cur;
                m.health_max = max;
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "KillPartyMember", {
        let s = Rc::clone(&state);
        move |_, idx: i32| {
            let mut st = s.borrow_mut();
            if let Some(m) = st.party_members.get_mut((idx - 1) as usize) {
                m.dead_since = Some(Instant::now());
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "ResPartyMember", {
        let s = Rc::clone(&state);
        move |_, idx: i32| {
            let mut st = s.borrow_mut();
            if let Some(m) = st.party_members.get_mut((idx - 1) as usize) {
                m.dead_since = None;
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetRotDamage", {
        let s = Rc::clone(&state);
        move |_, level: i32| { s.borrow_mut().rot_damage_level = level as usize; Ok(()) }
    })?;
    Ok(())
}

/// Build a default party member for padding.
fn default_party_member() -> PartyMember {
    PartyMember {
        name: "Unknown".to_string(),
        class_index: 1,
        level: 80,
        health: 100_000,
        health_max: 100_000,
        power: 0,
        power_max: 100,
        power_type: 1,
        power_type_name: "RAGE".to_string(),
        is_leader: false,
        dead_since: None,
    }
}

// ---------------------------------------------------------------------------
// Movement
// ---------------------------------------------------------------------------

fn register_movement_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetMoving", {
        let s = Rc::clone(&state);
        move |_, v: bool| { s.borrow_mut().player.movement.moving = v; Ok(()) }
    })?;
    set_fn(lua, t, "SetMounted", {
        let s = Rc::clone(&state);
        move |_, v: bool| { s.borrow_mut().player.movement.mounted = v; Ok(()) }
    })?;
    set_fn(lua, t, "SetFlying", {
        let s = Rc::clone(&state);
        move |_, v: bool| { s.borrow_mut().player.movement.flying = v; Ok(()) }
    })?;
    set_fn(lua, t, "SetFalling", {
        let s = Rc::clone(&state);
        move |_, v: bool| { s.borrow_mut().player.movement.falling = v; Ok(()) }
    })?;
    set_fn(lua, t, "SetSwimming", {
        let s = Rc::clone(&state);
        move |_, v: bool| { s.borrow_mut().player.movement.swimming = v; Ok(()) }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Spec & Talents
// ---------------------------------------------------------------------------

fn register_spec_talent_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetSpec", {
        let s = Rc::clone(&state);
        move |_, spec_index: i32| { s.borrow_mut().player.active_spec_index = spec_index; Ok(()) }
    })?;
    set_fn(lua, t, "SetTalentRank", {
        let s = Rc::clone(&state);
        move |_, (node_id, rank): (u32, u32)| {
            s.borrow_mut().talents.node_ranks.insert(node_id, rank);
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetTalentSelection", {
        let s = Rc::clone(&state);
        move |_, (node_id, entry_id): (u32, u32)| {
            s.borrow_mut().talents.node_selections.insert(node_id, entry_id);
            Ok(())
        }
    })?;
    set_fn(lua, t, "ResetTalents", {
        let s = Rc::clone(&state);
        move |_, ()| {
            let mut st = s.borrow_mut();
            st.talents.node_ranks.clear();
            st.talents.node_selections.clear();
            Ok(())
        }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Buffs
// ---------------------------------------------------------------------------

fn register_buff_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "AddBuff", {
        let s = Rc::clone(&state);
        move |_, (spell_id, name, icon, duration, stacks): (i32, String, String, f64, i32)| {
            let mut st = s.borrow_mut();
            let now = st.start_time.elapsed().as_secs_f64();
            let expiration_time = if duration > 0.0 { now + duration } else { 0.0 };
            let aura_instance_id = (st.player.buffs.len() + 1) as i32;
            st.player.buffs.push(AuraInfo {
                name,
                spell_id,
                icon: icon.parse::<i32>().unwrap_or(0),
                duration,
                expiration_time,
                applications: stacks,
                source_unit: "player".to_string(),
                is_helpful: true,
                is_stealable: false,
                can_apply_aura: true,
                is_from_player_or_player_pet: true,
                aura_instance_id,
            });
            Ok(())
        }
    })?;
    set_fn(lua, t, "RemoveBuff", {
        let s = Rc::clone(&state);
        move |_, spell_id: i32| {
            s.borrow_mut().player.buffs.retain(|a| a.spell_id != spell_id);
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearBuffs", {
        let s = Rc::clone(&state);
        move |_, ()| { s.borrow_mut().player.buffs.clear(); Ok(()) }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Zone & Instance
// ---------------------------------------------------------------------------

fn register_zone_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetZone", {
        let s = Rc::clone(&state);
        move |_, (name, id): (String, i32)| {
            let mut st = s.borrow_mut();
            st.world.zone_name = name;
            st.world.zone_id = id;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetSubZone", {
        let s = Rc::clone(&state);
        move |_, name: String| { s.borrow_mut().world.sub_zone_name = name; Ok(()) }
    })?;
    set_fn(lua, t, "SetInstanceInfo", {
        let s = Rc::clone(&state);
        move |_, (name, inst_type, difficulty, max_players): (String, String, i32, i32)| {
            let mut st = s.borrow_mut();
            st.world.instance_name = name;
            st.world.instance_type = inst_type;
            st.world.instance_difficulty = difficulty;
            st.world.instance_max_players = max_players;
            st.world.in_instance = true;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetInInstance", {
        let s = Rc::clone(&state);
        move |_, v: bool| { s.borrow_mut().world.in_instance = v; Ok(()) }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Economy
// ---------------------------------------------------------------------------

fn register_economy_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetMoney", {
        let s = Rc::clone(&state);
        move |_, copper: i64| { s.borrow_mut().player.money = copper; Ok(()) }
    })?;
    set_fn(lua, t, "SetItemLevel", {
        let s = Rc::clone(&state);
        move |_, ilvl: f64| { s.borrow_mut().player.item_level = ilvl as f32; Ok(()) }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Collections
// ---------------------------------------------------------------------------

fn register_collection_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "AddTransmog", {
        let s = Rc::clone(&state);
        move |_, id: i32| { s.borrow_mut().world.collected_transmogs.insert(id); Ok(()) }
    })?;
    set_fn(lua, t, "RemoveTransmog", {
        let s = Rc::clone(&state);
        move |_, id: i32| { s.borrow_mut().world.collected_transmogs.remove(&id); Ok(()) }
    })?;
    set_fn(lua, t, "SetMountCollected", {
        let s = Rc::clone(&state);
        move |_, (id, collected): (i32, bool)| {
            let mut st = s.borrow_mut();
            if collected { st.world.collected_mounts.insert(id); } else { st.world.collected_mounts.remove(&id); }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetPetCollected", {
        let s = Rc::clone(&state);
        move |_, (id, collected): (i32, bool)| {
            let mut st = s.borrow_mut();
            if collected { st.world.collected_pets.insert(id); } else { st.world.collected_pets.remove(&id); }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetToyCollected", {
        let s = Rc::clone(&state);
        move |_, (id, collected): (i32, bool)| {
            let mut st = s.borrow_mut();
            if collected { st.world.collected_toys.insert(id); } else { st.world.collected_toys.remove(&id); }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetAchievementEarned", {
        let s = Rc::clone(&state);
        move |_, (id, earned): (i32, bool)| {
            let mut st = s.borrow_mut();
            if earned { st.world.earned_achievements.insert(id); } else { st.world.earned_achievements.remove(&id); }
            Ok(())
        }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// PvP & Guild
// ---------------------------------------------------------------------------

fn register_pvp_guild_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetPvPEnabled", {
        let s = Rc::clone(&state);
        move |_, v: bool| { s.borrow_mut().player.pvp_enabled = v; Ok(()) }
    })?;
    set_fn(lua, t, "SetHonorLevel", {
        let s = Rc::clone(&state);
        move |_, level: i32| { s.borrow_mut().player.honor_level = level; Ok(()) }
    })?;
    set_fn(lua, t, "SetGuildInfo", {
        let s = Rc::clone(&state);
        move |_, (name, rank, num_members): (String, String, i32)| {
            let mut st = s.borrow_mut();
            st.world.guild_name = Some(name);
            st.world.guild_rank = Some(rank);
            st.world.guild_num_members = num_members;
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearGuild", {
        let s = Rc::clone(&state);
        move |_, ()| {
            let mut st = s.borrow_mut();
            st.world.guild_name = None;
            st.world.guild_rank = None;
            st.world.guild_num_members = 0;
            Ok(())
        }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

fn register_event_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "FireEvent", {
        let s = Rc::clone(&state);
        move |_, (event_name, args): (String, Variadic<Value>)| {
            let event_args: Vec<EventArg> = args.iter().map(lua_value_to_event_arg).collect();
            s.borrow_mut().events.push(Event { name: event_name, args: event_args });
            Ok(())
        }
    })?;
    Ok(())
}

/// Convert a Lua Value to an EventArg for queuing.
fn lua_value_to_event_arg(v: &Value) -> EventArg {
    match v {
        Value::String(s) => EventArg::String(s.to_string_lossy()),
        Value::Number(n) => EventArg::Number(*n),
        Value::Integer(n) => EventArg::Number(*n as f64),
        Value::Boolean(b) => EventArg::Boolean(*b),
        _ => EventArg::Nil,
    }
}

// ---------------------------------------------------------------------------
// Action Bars
// ---------------------------------------------------------------------------

fn register_action_bar_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetActionSlot", {
        let s = Rc::clone(&state);
        move |_, (slot, spell_id): (u32, u32)| {
            s.borrow_mut().action_bars.insert(slot, spell_id);
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearActionSlot", {
        let s = Rc::clone(&state);
        move |_, slot: u32| {
            s.borrow_mut().action_bars.remove(&slot);
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearActionBars", {
        let s = Rc::clone(&state);
        move |_, ()| { s.borrow_mut().action_bars.clear(); Ok(()) }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Great Vault
// ---------------------------------------------------------------------------

fn register_vault_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetVaultActivity", {
        let s = Rc::clone(&state);
        move |_, (atype, index, threshold, progress, level): (i32, i32, i32, i32, i32)| {
            let mut st = s.borrow_mut();
            let activity = crate::lua_api::state::GreatVaultActivity {
                activity_type: atype, index, threshold, progress, level,
            };
            if let Some(existing) = st.world.great_vault_activities.iter_mut()
                .find(|a| a.activity_type == atype && a.index == index)
            {
                *existing = activity;
            } else {
                st.world.great_vault_activities.push(activity);
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetVaultRewards", {
        let s = Rc::clone(&state);
        move |_, (has, can_claim): (bool, Option<bool>)| {
            let mut st = s.borrow_mut();
            st.world.great_vault_has_rewards = has;
            st.world.great_vault_can_claim = can_claim.unwrap_or(has);
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearVault", {
        let s = Rc::clone(&state);
        move |_, ()| {
            let mut st = s.borrow_mut();
            st.world.great_vault_activities.clear();
            st.world.great_vault_has_rewards = false;
            st.world.great_vault_can_claim = false;
            Ok(())
        }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Bags / Inventory
// ---------------------------------------------------------------------------

fn register_bag_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "AddBagItem", {
        let s = Rc::clone(&state);
        move |_, (bag, slot, item_id, stack): (i32, i32, u32, Option<i32>)| {
            let item = crate::lua_api::state::BagItem {
                item_id, stack_count: stack.unwrap_or(1),
            };
            s.borrow_mut().bag_items.insert((bag, slot), item);
            Ok(())
        }
    })?;
    set_fn(lua, t, "RemoveBagItem", {
        let s = Rc::clone(&state);
        move |_, (bag, slot): (i32, i32)| {
            s.borrow_mut().bag_items.remove(&(bag, slot));
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearBags", {
        let s = Rc::clone(&state);
        move |_, ()| { s.borrow_mut().bag_items.clear(); Ok(()) }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

/// Register a closure as a named method on a table.
pub fn set_fn<F, A, R>(lua: &Lua, t: &mlua::Table, name: &str, f: F) -> Result<()>
where
    F: Fn(&Lua, A) -> Result<R> + 'static,
    A: mlua::FromLuaMulti,
    R: mlua::IntoLuaMulti,
{
    t.set(name, lua.create_function(f)?)
}
