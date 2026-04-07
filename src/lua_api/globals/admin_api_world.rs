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
    register_bag_api(lua, t, Rc::clone(&state))?;
    super::admin_api_mail_premade::register_mail_admin_api(lua, t, Rc::clone(&state))?;
    super::admin_api_mail_premade::register_premade_admin_api(lua, t, Rc::clone(&state))?;
    register_debug_toggle_api(lua, t, state)?;
    Ok(())
}

fn register_debug_toggle_api(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    super::admin_api::set_fn(lua, t, "ToggleDebugBorders", {
        let s = Rc::clone(&state);
        move |_, ()| {
            let mut st = s.borrow_mut();
            st.debug_borders = !st.debug_borders;
            st.invalidate_strata_buckets();
            Ok(st.debug_borders)
        }
    })?;
    super::admin_api::set_fn(lua, t, "ToggleDebugAnchors", {
        let s = Rc::clone(&state);
        move |_, ()| {
            let mut st = s.borrow_mut();
            st.debug_anchors = !st.debug_anchors;
            st.invalidate_strata_buckets();
            Ok(st.debug_anchors)
        }
    })?;
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
    register_transmog_admin(lua, t, Rc::clone(&state))?;
    register_toggle_setters(lua, t, Rc::clone(&state))?;
    register_achievement_admin(lua, t, Rc::clone(&state))?;
    register_collect_helpers(lua, t, state)
}

fn register_transmog_admin(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
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
    register_add_transmog_appearance(lua, t, Rc::clone(&state))?;
    register_heirloom_admin(lua, t, Rc::clone(&state))?;
    super::admin_api::set_fn(lua, t, "SetTransmogForSlot", {
        let s = state;
        move |_, (slot_id, source_id): (i32, i32)| {
            s.borrow_mut()
                .world
                .applied_transmog_slots
                .insert(slot_id, source_id);
            Ok(())
        }
    })?;
    Ok(())
}

fn register_add_transmog_appearance(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    super::admin_api::set_fn(lua, t, "AddTransmogAppearance", {
        move |_, (source_id, category_id, item_id): (i32, i32, i32)| {
            let mut st = state.borrow_mut();
            let visual_id = st
                .world
                .transmog_appearances
                .iter()
                .map(|a| a.visual_id)
                .max()
                .unwrap_or(0)
                + 1;
            st.world
                .transmog_appearances
                .push(crate::lua_api::state_types::TransmogAppearance {
                    source_id,
                    visual_id,
                    category_id,
                    item_id,
                    is_collected: true,
                    source_type: 0,
                    item_mod_id: 0,
                });
            Ok(())
        }
    })
}

fn register_heirloom_admin(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "CollectHeirloom", {
        let s = Rc::clone(&state);
        move |_, item_id: i32| {
            s.borrow_mut()
                .world
                .collected_heirlooms
                .insert(item_id as u32);
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "UncollectHeirloom", {
        let s = state;
        move |_, item_id: i32| {
            s.borrow_mut()
                .world
                .collected_heirlooms
                .remove(&(item_id as u32));
            Ok(())
        }
    })
}

fn register_toggle_setters(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    add_world_toggle_setter(lua, t, "SetMountCollected", Rc::clone(&state), |s| {
        &mut s.world.collected_mounts
    })?;
    add_world_toggle_setter(lua, t, "SetPetCollected", Rc::clone(&state), |s| {
        &mut s.world.collected_pets
    })?;
    add_world_toggle_setter(lua, t, "SetToyCollected", Rc::clone(&state), |s| {
        &mut s.world.collected_toys
    })?;
    add_world_toggle_setter(lua, t, "SetAchievementEarned", state, |s| {
        &mut s.world.earned_achievements
    })
}

fn register_achievement_admin(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    super::admin_api::set_fn(lua, t, "HasAchievement", {
        let s = Rc::clone(&state);
        move |_, id: i32| Ok(s.borrow().world.earned_achievements.contains(&id))
    })?;
    super::admin_api::set_fn(lua, t, "EarnAchievement", {
        move |lua, id: i32| {
            state.borrow_mut().world.earned_achievements.insert(id);
            let fire: mlua::Function = lua.globals().get("FireEvent")?;
            fire.call::<()>(mlua::MultiValue::from_vec(vec![
                Value::String(lua.create_string("ACHIEVEMENT_EARNED")?),
                Value::Integer(id as i64),
            ]))?;
            Ok(())
        }
    })
}

fn register_collect_helpers(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    register_mount_collect(lua, t, Rc::clone(&state))?;
    register_pet_collect(lua, t, Rc::clone(&state))?;
    register_toy_collect(lua, t, state)
}

fn register_mount_collect(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "CollectMount", {
        let s = Rc::clone(&state);
        move |_, mount_id: u32| {
            let mut st = s.borrow_mut();
            st.world.collected_mounts.insert(mount_id as i32);
            if let Some(m) = st.world.mounts.iter_mut().find(|m| m.mount_id == mount_id) {
                m.is_collected = true;
                m.is_usable = true;
            }
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "UncollectMount", {
        move |_, mount_id: u32| {
            let mut st = state.borrow_mut();
            st.world.collected_mounts.remove(&(mount_id as i32));
            if let Some(m) = st.world.mounts.iter_mut().find(|m| m.mount_id == mount_id) {
                m.is_collected = false;
                m.is_usable = false;
            }
            Ok(())
        }
    })
}

fn register_pet_collect(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "CollectPet", {
        let s = Rc::clone(&state);
        move |_, species_id: u32| {
            let mut st = s.borrow_mut();
            st.world.collected_pets.insert(species_id as i32);
            if let Some(p) = st
                .world
                .pets
                .iter_mut()
                .find(|p| p.species_id == species_id)
            {
                p.is_collected = true;
            }
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "UncollectPet", {
        move |_, species_id: u32| {
            let mut st = state.borrow_mut();
            st.world.collected_pets.remove(&(species_id as i32));
            if let Some(p) = st
                .world
                .pets
                .iter_mut()
                .find(|p| p.species_id == species_id)
            {
                p.is_collected = false;
            }
            Ok(())
        }
    })
}

fn register_toy_collect(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "CollectToy", {
        let s = Rc::clone(&state);
        move |_, item_id: u32| {
            let mut st = s.borrow_mut();
            st.world.collected_toys.insert(item_id as i32);
            if let Some(toy) = st.world.toys.iter_mut().find(|t| t.item_id == item_id) {
                toy.is_collected = true;
                toy.is_usable = true;
            }
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "UncollectToy", {
        move |_, item_id: u32| {
            let mut st = state.borrow_mut();
            st.world.collected_toys.remove(&(item_id as i32));
            if let Some(toy) = st.world.toys.iter_mut().find(|t| t.item_id == item_id) {
                toy.is_collected = false;
                toy.is_usable = false;
            }
            Ok(())
        }
    })
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
    add_join_guild(lua, t, Rc::clone(&state))?;
    add_clear_guild_setter(lua, t, Rc::clone(&state))?;
    add_leave_guild(lua, t, state)?;
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

fn add_join_guild(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(
        lua,
        t,
        "JoinGuild",
        move |lua, (name, rank, num_members): (String, String, i32)| {
            let mut st = state.borrow_mut();
            st.world.guild_name = Some(name);
            st.world.guild_rank = Some(rank);
            st.world.guild_num_members = num_members;
            drop(st);
            let fire: mlua::Function = lua.globals().get("FireEvent")?;
            fire.call::<()>(mlua::MultiValue::from_vec(vec![Value::String(
                lua.create_string("PLAYER_GUILD_UPDATE")?,
            )]))?;
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

fn add_leave_guild(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "LeaveGuild", move |lua, ()| {
        clear_guild_info(&mut state.borrow_mut());
        let fire: mlua::Function = lua.globals().get("FireEvent")?;
        fire.call::<()>(mlua::MultiValue::from_vec(vec![Value::String(
            lua.create_string("PLAYER_GUILD_UPDATE")?,
        )]))?;
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
            upsert_vault_activity(
                &mut s.borrow_mut(),
                atype,
                index,
                threshold,
                progress,
                level,
            );
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "SetVaultRewards", {
        let s = Rc::clone(&state);
        move |_, (has, can_claim): (bool, Option<bool>)| {
            set_vault_rewards(&mut s.borrow_mut(), has, can_claim);
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "ClearVault", {
        let s = Rc::clone(&state);
        move |_, ()| {
            clear_vault(&mut s.borrow_mut());
            Ok(())
        }
    })?;
    Ok(())
}

fn upsert_vault_activity(
    state: &mut SimState,
    atype: i32,
    index: i32,
    threshold: i32,
    progress: i32,
    level: i32,
) {
    let activity = crate::lua_api::state::GreatVaultActivity {
        activity_type: atype,
        index,
        threshold,
        progress,
        level,
    };
    if let Some(existing) = state
        .world
        .great_vault_activities
        .iter_mut()
        .find(|a| a.activity_type == atype && a.index == index)
    {
        *existing = activity;
    } else {
        state.world.great_vault_activities.push(activity);
    }
}

fn set_vault_rewards(state: &mut SimState, has: bool, can_claim: Option<bool>) {
    state.world.great_vault_has_rewards = has;
    state.world.great_vault_can_claim = can_claim.unwrap_or(has);
}

fn clear_vault(state: &mut SimState) {
    state.world.great_vault_activities.clear();
    state.world.great_vault_has_rewards = false;
    state.world.great_vault_can_claim = false;
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
