use crate::event::{Event, EventArg};
use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value, Variadic};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn register_world_admin_api(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    register_zone_api(lua, t, Rc::clone(&state))?;
    register_economy_api(lua, t, Rc::clone(&state))?;
    register_collection_api(lua, t, Rc::clone(&state))?;
    register_pvp_guild_api(lua, t, Rc::clone(&state))?;
    register_event_api(lua, t, Rc::clone(&state))?;
    register_vault_api(lua, t, Rc::clone(&state))?;
    register_action_bar_api(lua, t, Rc::clone(&state))?;
    register_bag_api(lua, t, state)?;
    Ok(())
}

fn register_zone_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    add_zone_setter(lua, t, Rc::clone(&state))?;
    add_sub_zone_setter(lua, t, Rc::clone(&state))?;
    add_instance_info_setter(lua, t, Rc::clone(&state))?;
    add_in_instance_setter(lua, t, state)?;
    Ok(())
}

fn apply_instance_info(
    state: &mut SimState,
    name: String,
    inst_type: String,
    difficulty: i32,
    max_players: i32,
) {
    state.world.instance_name = name;
    state.world.instance_type = inst_type;
    state.world.instance_difficulty = difficulty;
    state.world.instance_max_players = max_players;
    state.world.in_instance = true;
}

fn add_zone_setter(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "SetZone", move |_, (name, id): (String, i32)| {
        let mut state = state.borrow_mut();
        state.world.zone_name = name;
        state.world.zone_id = id;
        Ok(())
    })
}

fn add_sub_zone_setter(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "SetSubZone", move |_, name: String| {
        state.borrow_mut().world.sub_zone_name = name;
        Ok(())
    })
}

fn add_instance_info_setter(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    super::admin_api::set_fn(
        lua,
        t,
        "SetInstanceInfo",
        move |_, (name, inst_type, difficulty, max_players): (String, String, i32, i32)| {
            apply_instance_info(
                &mut state.borrow_mut(),
                name,
                inst_type,
                difficulty,
                max_players,
            );
            Ok(())
        },
    )
}

fn add_in_instance_setter(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "SetInInstance", move |_, v: bool| {
        state.borrow_mut().world.in_instance = v;
        Ok(())
    })
}

fn register_economy_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "SetMoney", {
        let s = Rc::clone(&state);
        move |_, copper: i64| {
            s.borrow_mut().player.money = copper;
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "SetItemLevel", {
        let s = Rc::clone(&state);
        move |_, ilvl: f64| {
            s.borrow_mut().player.item_level = ilvl as f32;
            Ok(())
        }
    })?;
    Ok(())
}

fn register_collection_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "AddTransmog", {
        let s = Rc::clone(&state);
        move |_, id: i32| {
            s.borrow_mut().world.collected_transmogs.insert(id);
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "RemoveTransmog", {
        let s = Rc::clone(&state);
        move |_, id: i32| {
            s.borrow_mut().world.collected_transmogs.remove(&id);
            Ok(())
        }
    })?;
    add_world_toggle_setter(lua, t, "SetMountCollected", Rc::clone(&state), |state| {
        &mut state.world.collected_mounts
    })?;
    add_world_toggle_setter(lua, t, "SetPetCollected", Rc::clone(&state), |state| {
        &mut state.world.collected_pets
    })?;
    add_world_toggle_setter(lua, t, "SetToyCollected", Rc::clone(&state), |state| {
        &mut state.world.collected_toys
    })?;
    add_world_toggle_setter(lua, t, "SetAchievementEarned", state, |state| {
        &mut state.world.earned_achievements
    })?;
    Ok(())
}

fn add_world_toggle_setter<F>(
    lua: &Lua,
    t: &mlua::Table,
    name: &str,
    state: Rc<RefCell<SimState>>,
    set_ref: F,
) -> Result<()>
where
    F: Fn(&mut SimState) -> &mut std::collections::HashSet<i32> + 'static,
{
    super::admin_api::set_fn(lua, t, name, move |_, (id, collected): (i32, bool)| {
        let mut state = state.borrow_mut();
        let set = set_ref(&mut state);
        if collected {
            set.insert(id);
        } else {
            set.remove(&id);
        }
        Ok(())
    })
}

fn register_pvp_guild_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "SetPvPEnabled", {
        let s = Rc::clone(&state);
        move |_, v: bool| {
            s.borrow_mut().player.pvp_enabled = v;
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "SetHonorLevel", {
        let s = Rc::clone(&state);
        move |_, level: i32| {
            s.borrow_mut().player.honor_level = level;
            Ok(())
        }
    })?;
    add_guild_info_setter(lua, t, Rc::clone(&state))?;
    add_clear_guild_setter(lua, t, state)?;
    Ok(())
}

fn add_guild_info_setter(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(
        lua,
        t,
        "SetGuildInfo",
        move |_, (name, rank, num_members): (String, String, i32)| {
            let mut state = state.borrow_mut();
            state.world.guild_name = Some(name);
            state.world.guild_rank = Some(rank);
            state.world.guild_num_members = num_members;
            Ok(())
        },
    )
}

fn add_clear_guild_setter(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "ClearGuild", move |_, ()| {
        clear_guild_info(&mut state.borrow_mut());
        Ok(())
    })
}

fn clear_guild_info(state: &mut SimState) {
    state.world.guild_name = None;
    state.world.guild_rank = None;
    state.world.guild_num_members = 0;
}

fn register_event_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "FireEvent", {
        let s = Rc::clone(&state);
        move |_, (event_name, args): (String, Variadic<Value>)| {
            let event_args: Vec<EventArg> = args.iter().map(lua_value_to_event_arg).collect();
            s.borrow_mut().events.push(Event {
                name: event_name,
                args: event_args,
            });
            Ok(())
        }
    })?;
    Ok(())
}

fn lua_value_to_event_arg(v: &Value) -> EventArg {
    match v {
        Value::String(s) => EventArg::String(s.to_string_lossy()),
        Value::Number(n) => EventArg::Number(*n),
        Value::Integer(n) => EventArg::Number(*n as f64),
        Value::Boolean(b) => EventArg::Boolean(*b),
        _ => EventArg::Nil,
    }
}

fn register_action_bar_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "SetActionSlot", {
        let s = Rc::clone(&state);
        move |_, (slot, spell_id): (u32, u32)| {
            s.borrow_mut().action_bars.insert(slot, spell_id);
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "ClearActionSlot", {
        let s = Rc::clone(&state);
        move |_, slot: u32| {
            s.borrow_mut().action_bars.remove(&slot);
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "ClearActionBars", {
        let s = Rc::clone(&state);
        move |_, ()| {
            s.borrow_mut().action_bars.clear();
            Ok(())
        }
    })?;
    Ok(())
}

fn register_vault_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "SetVaultActivity", {
        let s = Rc::clone(&state);
        move |_, (atype, index, threshold, progress, level): (i32, i32, i32, i32, i32)| {
            let mut st = s.borrow_mut();
            let activity = crate::lua_api::state::GreatVaultActivity {
                activity_type: atype,
                index,
                threshold,
                progress,
                level,
            };
            if let Some(existing) = st
                .world
                .great_vault_activities
                .iter_mut()
                .find(|a| a.activity_type == atype && a.index == index)
            {
                *existing = activity;
            } else {
                st.world.great_vault_activities.push(activity);
            }
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "SetVaultRewards", {
        let s = Rc::clone(&state);
        move |_, (has, can_claim): (bool, Option<bool>)| {
            let mut st = s.borrow_mut();
            st.world.great_vault_has_rewards = has;
            st.world.great_vault_can_claim = can_claim.unwrap_or(has);
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "ClearVault", {
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

fn register_bag_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "AddBagItem", {
        let s = Rc::clone(&state);
        move |_, (bag, slot, item_id, stack): (i32, i32, u32, Option<i32>)| {
            let item = crate::lua_api::state::BagItem {
                item_id,
                stack_count: stack.unwrap_or(1),
            };
            s.borrow_mut().bag_items.insert((bag, slot), item);
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "RemoveBagItem", {
        let s = Rc::clone(&state);
        move |_, (bag, slot): (i32, i32)| {
            s.borrow_mut().bag_items.remove(&(bag, slot));
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "ClearBags", {
        let s = Rc::clone(&state);
        move |_, ()| {
            s.borrow_mut().bag_items.clear();
            Ok(())
        }
    })?;
    Ok(())
}
