//! A_Admin namespace for simulator state control from Lua.
//!
//! Provides functions to set player stats, combat state, targeting, party,
//! buffs, zones, economy, collections, PvP, guild, and fire events.
//! Intended for addon test scripts and UI development.

use crate::lua_api::game_data::AuraInfo;
use crate::lua_api::state::SimState;
use mlua::{Lua, Result};
use std::cell::RefCell;
use std::rc::Rc;

mod units;

/// Register the A_Admin namespace on Lua globals.
pub fn register_admin_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let admin = lua.create_table()?;

    register_identity_api(lua, &admin, Rc::clone(&state))?;
    register_combat_api(lua, &admin, Rc::clone(&state))?;
    register_health_power_api(lua, &admin, Rc::clone(&state))?;
    units::register_targeting_api(lua, &admin, Rc::clone(&state))?;
    units::register_party_api(lua, &admin, Rc::clone(&state))?;
    units::register_movement_api(lua, &admin, Rc::clone(&state))?;
    register_spec_talent_api(lua, &admin, Rc::clone(&state))?;
    register_buff_api(lua, &admin, Rc::clone(&state))?;
    super::admin_api_world::register_world_admin_api(lua, &admin, Rc::clone(&state))?;
    super::admin_encounter::register_encounter_api(lua, &admin, Rc::clone(&state))?;
    register_equipment_api(lua, &admin, Rc::clone(&state))?;

    lua.globals().set("A_Admin", admin)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Player identity
// ---------------------------------------------------------------------------

fn register_identity_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetPlayerName", {
        let s = Rc::clone(&state);
        move |_, name: String| {
            s.borrow_mut().player.name = name;
            Ok(())
        }
    })?;
    register_identity_scalar_setters(lua, t, state)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Combat
// ---------------------------------------------------------------------------

fn register_combat_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetInCombat", {
        let s = Rc::clone(&state);
        move |_, v: bool| {
            s.borrow_mut().player.in_combat = v;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetResting", {
        let s = Rc::clone(&state);
        move |_, v: bool| {
            s.borrow_mut().player.is_resting = v;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetFrameProtected", {
        let s = Rc::clone(&state);
        move |_, (name, v): (String, bool)| {
            let mut st = s.borrow_mut();
            if let Some(id) = st.widgets.get_id_by_name(&name) {
                if let Some(frame) = st.widgets.get_mut(id) {
                    frame.is_protected = v;
                }
            }
            Ok(())
        }
    })?;
    super::admin_combat::register_casting_api(lua, t, state)
}

// ---------------------------------------------------------------------------
// Health & Power
// ---------------------------------------------------------------------------

fn register_health_power_api(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
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
            if let Some(pt) = power_type {
                st.player.power_type = pt;
            }
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
// Spec & Talents
// ---------------------------------------------------------------------------

fn register_spec_talent_api(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    set_fn(lua, t, "SetSpec", {
        let s = Rc::clone(&state);
        move |_, spec_index: i32| {
            s.borrow_mut().player.active_spec_index = spec_index;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetTalentRank", {
        let s = Rc::clone(&state);
        move |_, (node_id, rank): (u32, u32)| {
            s.borrow_mut().talents.set_node_rank(node_id, rank);
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetTalentSelection", {
        let s = Rc::clone(&state);
        move |_, (node_id, entry_id): (u32, u32)| {
            s.borrow_mut()
                .talents
                .set_node_selection(node_id, Some(entry_id));
            Ok(())
        }
    })?;
    set_fn(lua, t, "ResetTalents", {
        let s = Rc::clone(&state);
        move |_, ()| {
            let mut st = s.borrow_mut();
            st.talents.clear_ranks();
            st.talents.node_selections.clear();
            st.talents.active_hero_subtree_id = None;
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
            let buff = build_admin_buff(&st, spell_id, name, icon, duration, stacks);
            st.player.buffs.push(buff);
            Ok(())
        }
    })?;
    set_fn(lua, t, "RemoveBuff", {
        let s = Rc::clone(&state);
        move |_, spell_id: i32| {
            s.borrow_mut()
                .player
                .buffs
                .retain(|a| a.spell_id != spell_id);
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearBuffs", {
        let s = Rc::clone(&state);
        move |_, ()| {
            s.borrow_mut().player.buffs.clear();
            Ok(())
        }
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Equipment
// ---------------------------------------------------------------------------

fn register_equipment_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    use crate::lua_api::state_types::{CharacterStats, EquippedItem};

    set_fn(lua, t, "EquipItem", {
        let s = Rc::clone(&state);
        move |_, (slot, item_id): (i32, u32)| {
            let mut state = s.borrow_mut();
            state.player.equipped_items.insert(
                slot,
                EquippedItem {
                    item_id,
                    enchant_id: 0,
                    gem_ids: [0, 0, 0],
                },
            );
            state.player.stats =
                CharacterStats::compute(&state.player.equipped_items, state.player.class_index);
            Ok(())
        }
    })?;
    set_fn(lua, t, "UnequipItem", {
        let s = Rc::clone(&state);
        move |_, slot: i32| {
            let mut state = s.borrow_mut();
            state.player.equipped_items.remove(&slot);
            state.player.stats =
                CharacterStats::compute(&state.player.equipped_items, state.player.class_index);
            Ok(())
        }
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

fn add_player_i32_setter<F>(
    lua: &Lua,
    t: &mlua::Table,
    name: &str,
    state: Rc<RefCell<SimState>>,
    apply: F,
) -> Result<()>
where
    F: Fn(&mut crate::lua_api::state::PlayerState, i32) + 'static,
{
    set_fn(lua, t, name, move |_, value: i32| {
        apply(&mut state.borrow_mut().player, value);
        Ok(())
    })
}

fn register_identity_scalar_setters(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    add_player_i32_setter(
        lua,
        t,
        "SetPlayerClass",
        Rc::clone(&state),
        |player, class_index| {
            player.class_index = class_index;
        },
    )?;
    add_player_i32_setter(
        lua,
        t,
        "SetPlayerRace",
        Rc::clone(&state),
        |player, race_index| {
            player.race_index = race_index as usize;
        },
    )?;
    add_player_i32_setter(
        lua,
        t,
        "SetPlayerLevel",
        Rc::clone(&state),
        |player, level| {
            player.level = level;
        },
    )?;
    add_player_i32_setter(lua, t, "SetPlayerSex", state, |player, sex| {
        player.sex = sex;
    })
}

fn add_rot_damage_setter(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetRotDamage", move |_, level: i32| {
        state.borrow_mut().rot_damage_level = level as usize;
        Ok(())
    })
}

fn build_admin_buff(
    state: &SimState,
    spell_id: i32,
    name: String,
    icon: String,
    duration: f64,
    stacks: i32,
) -> AuraInfo {
    AuraInfo {
        name,
        spell_id,
        icon: parse_admin_buff_icon(&icon),
        duration,
        expiration_time: admin_buff_expiration_time(state, duration),
        applications: stacks,
        source_unit: "player".to_string(),
        is_helpful: true,
        is_stealable: false,
        can_apply_aura: true,
        is_from_player_or_player_pet: true,
        aura_instance_id: next_admin_buff_instance_id(state),
    }
}

fn parse_admin_buff_icon(icon: &str) -> i32 {
    icon.parse::<i32>().unwrap_or(0)
}

fn admin_buff_expiration_time(state: &SimState, duration: f64) -> f64 {
    if duration > 0.0 {
        state.start_time.elapsed().as_secs_f64() + duration
    } else {
        0.0
    }
}

fn next_admin_buff_instance_id(state: &SimState) -> i32 {
    (state.player.buffs.len() + 1) as i32
}
