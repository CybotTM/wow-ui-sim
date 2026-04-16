//! Rilua A_Admin handlers (PvP/guild through Encounter).
//!
//! Split out of rilua_admin.rs to keep files under the 750-line cap.
//! Covers PvP/guild, Events, Debug toggles, Vault, Action bars, Bags,
//! Mail, Premade listings, and Encounter sections. The `pub(super)`
//! visibility matches the entry-point TableBuilder chain in
//! rilua_admin::register_all.

use super::rilua_admin::{build_mail, lua_val_to_event_arg, opt_string_stack};
use crate::lua_api::rilua_methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ── PvP & guild ───────────────────────────────────────────────────────────────

pub(super) fn set_pvp_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let v = bool::from_stack(state, 1)?;
    borrow_state_mut(state)?.player.pvp_enabled = v;
    Ok(0)
}

pub(super) fn set_honor_level(state: &mut LuaState) -> LuaResult<u32> {
    let level = i32::from_stack(state, 1)?;
    borrow_state_mut(state)?.player.honor_level = level;
    Ok(0)
}

pub(super) fn set_guild_info(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let rank = String::from_stack(state, 2)?;
    let num_members = i32::from_stack(state, 3)?;
    let mut st = borrow_state_mut(state)?;
    st.world.guild_name = Some(name);
    st.world.guild_rank = Some(rank);
    st.world.guild_num_members = num_members;
    Ok(0)
}

pub(super) fn join_guild(state: &mut LuaState) -> LuaResult<u32> {
    use crate::event::Event;
    let name = String::from_stack(state, 1)?;
    let rank = String::from_stack(state, 2)?;
    let num_members = i32::from_stack(state, 3)?;
    let mut st = borrow_state_mut(state)?;
    st.world.guild_name = Some(name);
    st.world.guild_rank = Some(rank);
    st.world.guild_num_members = num_members;
    st.events.push(Event {
        name: "PLAYER_GUILD_UPDATE".to_string(),
        args: vec![],
    });
    Ok(0)
}

pub(super) fn clear_guild(state: &mut LuaState) -> LuaResult<u32> {
    let mut st = borrow_state_mut(state)?;
    st.world.guild_name = None;
    st.world.guild_rank = None;
    st.world.guild_num_members = 0;
    Ok(0)
}

pub(super) fn leave_guild(state: &mut LuaState) -> LuaResult<u32> {
    use crate::event::Event;
    let mut st = borrow_state_mut(state)?;
    st.world.guild_name = None;
    st.world.guild_rank = None;
    st.world.guild_num_members = 0;
    st.events.push(Event {
        name: "PLAYER_GUILD_UPDATE".to_string(),
        args: vec![],
    });
    Ok(0)
}

// ── Events ────────────────────────────────────────────────────────────────────

pub(super) fn fire_event_admin(state: &mut LuaState) -> LuaResult<u32> {
    use crate::event::Event;
    use crate::lua_bridge::stack_val;

    let event_name = String::from_stack(state, 1)?;
    let nargs = state.top as i32 - state.base as i32;
    let mut event_args = Vec::new();
    for i in 2..=nargs {
        let val = stack_val(state, i);
        event_args.push(lua_val_to_event_arg(state, val));
    }
    borrow_state_mut(state)?.events.push(Event {
        name: event_name,
        args: event_args,
    });
    Ok(0)
}

// ── Debug toggles ─────────────────────────────────────────────────────────────

pub(super) fn toggle_debug_borders(state: &mut LuaState) -> LuaResult<u32> {
    let mut st = borrow_state_mut(state)?;
    st.debug_borders = !st.debug_borders;
    st.invalidate_strata_buckets();
    let result = st.debug_borders;
    drop(st);
    state.push(Val::Bool(result));
    Ok(1)
}

pub(super) fn toggle_debug_anchors(state: &mut LuaState) -> LuaResult<u32> {
    let mut st = borrow_state_mut(state)?;
    st.debug_anchors = !st.debug_anchors;
    st.invalidate_strata_buckets();
    let result = st.debug_anchors;
    drop(st);
    state.push(Val::Bool(result));
    Ok(1)
}

// ── Vault ─────────────────────────────────────────────────────────────────────

pub(super) fn set_vault_activity(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::state::GreatVaultActivity;
    let atype = i32::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    let threshold = i32::from_stack(state, 3)?;
    let progress = i32::from_stack(state, 4)?;
    let level = i32::from_stack(state, 5)?;
    let activity = GreatVaultActivity {
        activity_type: atype,
        index,
        threshold,
        progress,
        level,
    };
    let mut st = borrow_state_mut(state)?;
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
    Ok(0)
}

pub(super) fn set_vault_rewards(state: &mut LuaState) -> LuaResult<u32> {
    let has = bool::from_stack(state, 1)?;
    let can_claim = Option::<bool>::from_stack(state, 2)?;
    let mut st = borrow_state_mut(state)?;
    st.world.great_vault_has_rewards = has;
    st.world.great_vault_can_claim = can_claim.unwrap_or(has);
    Ok(0)
}

pub(super) fn clear_vault(state: &mut LuaState) -> LuaResult<u32> {
    let mut st = borrow_state_mut(state)?;
    st.world.great_vault_activities.clear();
    st.world.great_vault_has_rewards = false;
    st.world.great_vault_can_claim = false;
    Ok(0)
}

// ── Action bars ───────────────────────────────────────────────────────────────

pub(super) fn set_action_slot(state: &mut LuaState) -> LuaResult<u32> {
    let slot = u32::from_stack(state, 1)?;
    let spell_id = u32::from_stack(state, 2)?;
    borrow_state_mut(state)?.action_bars.insert(slot, spell_id);
    Ok(0)
}

pub(super) fn clear_action_slot(state: &mut LuaState) -> LuaResult<u32> {
    let slot = u32::from_stack(state, 1)?;
    borrow_state_mut(state)?.action_bars.remove(&slot);
    Ok(0)
}

pub(super) fn clear_action_bars(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.action_bars.clear();
    Ok(0)
}

// ── Bags ──────────────────────────────────────────────────────────────────────

pub(super) fn add_bag_item(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::state::BagItem;
    let bag = i32::from_stack(state, 1)?;
    let slot = i32::from_stack(state, 2)?;
    let item_id = u32::from_stack(state, 3)?;
    let stack = Option::<i32>::from_stack(state, 4)?;
    borrow_state_mut(state)?.bag_items.insert(
        (bag, slot),
        BagItem {
            item_id,
            stack_count: stack.unwrap_or(1),
        },
    );
    Ok(0)
}

pub(super) fn remove_bag_item(state: &mut LuaState) -> LuaResult<u32> {
    let bag = i32::from_stack(state, 1)?;
    let slot = i32::from_stack(state, 2)?;
    borrow_state_mut(state)?.bag_items.remove(&(bag, slot));
    Ok(0)
}

pub(super) fn clear_bags(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.bag_items.clear();
    Ok(0)
}

// ── Mail ──────────────────────────────────────────────────────────────────────

pub(super) fn add_mail(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_bridge::stack_val;

    let sender = opt_string_stack(state, 1, "Unknown");
    let subject = opt_string_stack(state, 2, "No Subject");
    let body = opt_string_stack(state, 3, "");
    let money = match stack_val(state, 4) {
        Val::Num(n) => n as u64,
        _ => 0,
    };
    // items table at arg 5 — parsed as empty for now (no mlua Table access in rilua path)
    let items = Vec::new();

    let mut st = borrow_state_mut(state)?;
    let id = st.player.next_mail_id;
    st.player.next_mail_id += 1;
    st.player
        .inbox
        .push(build_mail(id, sender, subject, body, money, items));
    Ok(0)
}

pub(super) fn clear_inbox(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.player.inbox.clear();
    Ok(0)
}

pub(super) fn set_inbox_count(state: &mut LuaState) -> LuaResult<u32> {
    let count = i32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.player.inbox.clear();
    for i in 0..count {
        let id = st.player.next_mail_id;
        st.player.next_mail_id += 1;
        let sender = format!("Player{}", i + 1);
        let subject = format!("Test Mail #{}", i + 1);
        let body = format!("This is test mail message {}.", i + 1);
        st.player
            .inbox
            .push(build_mail(id, sender, subject, body, 0, Vec::new()));
    }
    Ok(0)
}

// ── Premade listings ──────────────────────────────────────────────────────────

pub(super) fn add_premade_listing(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::state_types::PremadeListing;
    let name = String::from_stack(state, 1)?;
    let comment = String::from_stack(state, 2)?;
    let activity_id = u32::from_stack(state, 3)?;
    let num = i32::from_stack(state, 4)?;
    let max = i32::from_stack(state, 5)?;
    let mut st = borrow_state_mut(state)?;
    let id = st.world.premade_listings.len() as u32 + 1;
    st.world.premade_listings.push(PremadeListing {
        search_result_id: id,
        name,
        comment,
        leader_name: "Player".to_string(),
        activity_id,
        num_members: num,
        max_members: max,
        voice_chat: false,
        auto_accept: false,
        is_delisted: false,
    });
    drop(st);
    state.push(Val::Num(id as f64));
    Ok(1)
}

pub(super) fn clear_premade_listings(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.world.premade_listings.clear();
    Ok(0)
}

pub(super) fn update_premade_listing(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_bridge::stack_val;
    let result_id = u32::from_stack(state, 1)?;
    let field = String::from_stack(state, 2)?;
    let value = stack_val(state, 3);
    let mut st = borrow_state_mut(state)?;
    let Some(listing) = st
        .world
        .premade_listings
        .iter_mut()
        .find(|l| l.search_result_id == result_id)
    else {
        return Ok(0);
    };
    match field.as_str() {
        "numMembers" => {
            if let Val::Num(n) = value {
                listing.num_members = n as i32;
            }
        }
        "isDelisted" => {
            if let Val::Bool(b) = value {
                listing.is_delisted = b;
            }
        }
        _ => {}
    }
    Ok(0)
}

// ── Encounter ─────────────────────────────────────────────────────────────────

pub(super) fn simulate_boss_kill(state: &mut LuaState) -> LuaResult<u32> {
    use crate::event::{Event, EventArg};
    let encounter_id = i32::from_stack(state, 1)?;
    let name = String::from_stack(state, 2)?;
    let difficulty_id = i32::from_stack(state, 3)?;
    let group_size = i32::from_stack(state, 4)?;
    let mut st = borrow_state_mut(state)?;
    st.events.push(Event {
        name: "ENCOUNTER_END".to_string(),
        args: vec![
            EventArg::Number(encounter_id as f64),
            EventArg::String(name.clone()),
            EventArg::Number(difficulty_id as f64),
            EventArg::Number(group_size as f64),
            EventArg::Number(1.0), // success
        ],
    });
    st.events.push(Event {
        name: "BOSS_KILL".to_string(),
        args: vec![
            EventArg::Number(encounter_id as f64),
            EventArg::String(name),
        ],
    });
    Ok(0)
}

pub(super) fn start_loot_roll(state: &mut LuaState) -> LuaResult<u32> {
    use crate::event::{Event, EventArg};
    use crate::lua_api::state::LootRollInfo;
    use crate::lua_bridge::stack_val;

    let roll_id = i32::from_stack(state, 1)?;
    let roll_time = f64::from_stack(state, 2)?;
    let item_name = opt_string_stack(state, 3, "");
    let item_texture = opt_string_stack(state, 4, "");
    let item_quality = match stack_val(state, 5) {
        Val::Num(n) => n as i32,
        _ => 4,
    };
    let item_level = match stack_val(state, 6) {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    let item_link = opt_string_stack(state, 7, "");

    let info = LootRollInfo {
        roll_id,
        roll_time,
        texture: item_texture,
        name: item_name,
        count: 1,
        quality: item_quality,
        bind_on_pickup: true,
        can_need: true,
        can_greed: true,
        can_disenchant: false,
        disenchant_level: 0,
        item_level,
        item_link,
    };
    let mut st = borrow_state_mut(state)?;
    st.world.loot_rolls.insert(roll_id, info);
    st.events.push(Event {
        name: "START_LOOT_ROLL".to_string(),
        args: vec![
            EventArg::Number(roll_id as f64),
            EventArg::Number(roll_time),
        ],
    });
    Ok(0)
}

pub(super) fn end_loot_roll(state: &mut LuaState) -> LuaResult<u32> {
    use crate::event::{Event, EventArg};
    let roll_id = i32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.world.loot_rolls.remove(&roll_id);
    st.events.push(Event {
        name: "LOOT_ROLLS_COMPLETE".to_string(),
        args: vec![EventArg::Number(roll_id as f64)],
    });
    Ok(0)
}
