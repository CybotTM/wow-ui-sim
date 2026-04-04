//! A_Admin namespace for simulator state control from Lua.
//!
//! Provides functions to set player stats, combat state, targeting, party,
//! buffs, zones, economy, collections, PvP, guild, and fire events.
//! Intended for addon test scripts and UI development.

use crate::lua_api::game_data::{AuraInfo, PartyMember, TargetInfo};
use crate::lua_api::state::SimState;
use mlua::{Lua, Result};
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
    super::admin_api_world::register_world_admin_api(lua, &admin, Rc::clone(&state))?;
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
        move |_, name: String| {
            s.borrow_mut().player.name = name;
            Ok(())
        }
    })?;
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
    })?;
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
// Targeting
// ---------------------------------------------------------------------------

fn register_targeting_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    set_fn(lua, t, "SetTarget", {
        let s = Rc::clone(&state);
        move |_, (name, level, class_index, is_enemy): (String, i32, i32, bool)| {
            s.borrow_mut().current_target = Some(make_target_info(
                "target",
                &name,
                level,
                class_index,
                is_enemy,
            ));
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearTarget", {
        let s = Rc::clone(&state);
        move |_, ()| {
            s.borrow_mut().current_target = None;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetFocus", {
        let s = Rc::clone(&state);
        move |_, (name, level, class_index, is_enemy): (String, i32, i32, bool)| {
            s.borrow_mut().current_focus = Some(make_target_info(
                "focus",
                &name,
                level,
                class_index,
                is_enemy,
            ));
            Ok(())
        }
    })?;
    set_fn(lua, t, "ClearFocus", {
        let s = Rc::clone(&state);
        move |_, ()| {
            s.borrow_mut().current_focus = None;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetTargetPower", {
        let s = Rc::clone(&state);
        move |_, (cur, max, power_type): (i32, i32, Option<i32>)| {
            let mut st = s.borrow_mut();
            if let Some(t) = st.current_target.as_mut() {
                t.power = cur;
                t.power_max = max;
                if let Some(pt) = power_type {
                    t.power_type = pt;
                }
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
                if let Some(pt) = power_type {
                    f.power_type = pt;
                }
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetTargetType", {
        let s = Rc::clone(&state);
        move |_,
              (classification, creature_type, reaction): (
            Option<String>,
            Option<String>,
            Option<i32>,
        )| {
            let mut st = s.borrow_mut();
            if let Some(t) = st.current_target.as_mut() {
                if let Some(c) = classification {
                    t.classification = c;
                }
                if let Some(ct) = creature_type {
                    t.creature_type = ct;
                }
                if let Some(r) = reaction {
                    t.reaction = r;
                }
            }
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetFocusType", {
        let s = Rc::clone(&state);
        move |_,
              (classification, creature_type, reaction): (
            Option<String>,
            Option<String>,
            Option<i32>,
        )| {
            let mut st = s.borrow_mut();
            if let Some(f) = st.current_focus.as_mut() {
                if let Some(c) = classification {
                    f.classification = c;
                }
                if let Some(ct) = creature_type {
                    f.creature_type = ct;
                }
                if let Some(r) = reaction {
                    f.reaction = r;
                }
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
fn make_target_info(
    unit_id: &str,
    name: &str,
    level: i32,
    class_index: i32,
    is_enemy: bool,
) -> TargetInfo {
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
        classification: if is_enemy {
            "normal".to_string()
        } else {
            "normal".to_string()
        },
        creature_type: if is_enemy {
            "Humanoid".to_string()
        } else {
            "Humanoid".to_string()
        },
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
        move |_, level: i32| {
            s.borrow_mut().rot_damage_level = level as usize;
            Ok(())
        }
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
        move |_, v: bool| {
            s.borrow_mut().player.movement.moving = v;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetMounted", {
        let s = Rc::clone(&state);
        move |_, v: bool| {
            s.borrow_mut().player.movement.mounted = v;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetFlying", {
        let s = Rc::clone(&state);
        move |_, v: bool| {
            s.borrow_mut().player.movement.flying = v;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetFalling", {
        let s = Rc::clone(&state);
        move |_, v: bool| {
            s.borrow_mut().player.movement.falling = v;
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetSwimming", {
        let s = Rc::clone(&state);
        move |_, v: bool| {
            s.borrow_mut().player.movement.swimming = v;
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
