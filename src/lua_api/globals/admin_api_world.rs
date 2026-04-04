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
    super::admin_api::set_fn(lua, t, "SetZone", {
        let s = Rc::clone(&state);
        move |_, (name, id): (String, i32)| {
            let mut st = s.borrow_mut();
            st.world.zone_name = name;
            st.world.zone_id = id;
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "SetSubZone", {
        let s = Rc::clone(&state);
        move |_, name: String| {
            s.borrow_mut().world.sub_zone_name = name;
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "SetInstanceInfo", {
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
    super::admin_api::set_fn(lua, t, "SetInInstance", {
        let s = Rc::clone(&state);
        move |_, v: bool| {
            s.borrow_mut().world.in_instance = v;
            Ok(())
        }
    })?;
    Ok(())
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
    super::admin_api::set_fn(lua, t, "SetMountCollected", {
        let s = Rc::clone(&state);
        move |_, (id, collected): (i32, bool)| {
            let mut st = s.borrow_mut();
            if collected {
                st.world.collected_mounts.insert(id);
            } else {
                st.world.collected_mounts.remove(&id);
            }
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "SetPetCollected", {
        let s = Rc::clone(&state);
        move |_, (id, collected): (i32, bool)| {
            let mut st = s.borrow_mut();
            if collected {
                st.world.collected_pets.insert(id);
            } else {
                st.world.collected_pets.remove(&id);
            }
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "SetToyCollected", {
        let s = Rc::clone(&state);
        move |_, (id, collected): (i32, bool)| {
            let mut st = s.borrow_mut();
            if collected {
                st.world.collected_toys.insert(id);
            } else {
                st.world.collected_toys.remove(&id);
            }
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "SetAchievementEarned", {
        let s = Rc::clone(&state);
        move |_, (id, earned): (i32, bool)| {
            let mut st = s.borrow_mut();
            if earned {
                st.world.earned_achievements.insert(id);
            } else {
                st.world.earned_achievements.remove(&id);
            }
            Ok(())
        }
    })?;
    Ok(())
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
    super::admin_api::set_fn(lua, t, "SetGuildInfo", {
        let s = Rc::clone(&state);
        move |_, (name, rank, num_members): (String, String, i32)| {
            let mut st = s.borrow_mut();
            st.world.guild_name = Some(name);
            st.world.guild_rank = Some(rank);
            st.world.guild_num_members = num_members;
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "ClearGuild", {
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
