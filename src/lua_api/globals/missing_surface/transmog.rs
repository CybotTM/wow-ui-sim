use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, table_get, val_to_string};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const TRANSMOGRIFY_FRAME: &str = "TransmogrifyFrame";

pub(super) fn register_transmog_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Transmog")?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAppliedAlteredAppearance",
        c_transmog_get_applied_altered_appearance,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetCreatureDisplayIDForSource",
        c_transmog_get_creature_display_id_for_source,
    )?;
    table_set_rust_fn_static(state, table_ref, "IsAtTransmogNPC", c_transmog_is_at_npc)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "PlayerHasTransmogByItemInfo",
        c_transmog_player_has_transmog_by_item_info,
    )?;
    Ok(())
}

fn c_transmog_get_applied_altered_appearance(state: &mut LuaState) -> LuaResult<u32> {
    let slot_id = i32::from_stack(state, 1)?;
    let source_id = borrow_state(state)?
        .world
        .applied_transmog_slots
        .get(&slot_id)
        .copied();
    match source_id {
        Some(source_id) => state.push(Val::Num(source_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_transmog_get_creature_display_id_for_source(state: &mut LuaState) -> LuaResult<u32> {
    let source_id = i32::from_stack(state, 1)?;
    let visual_id = borrow_state(state)?
        .world
        .transmog_appearances
        .iter()
        .find(|appearance| appearance.source_id == source_id)
        .map(|appearance| appearance.visual_id);
    match visual_id {
        Some(visual_id) => state.push(Val::Num(visual_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_transmog_is_at_npc(state: &mut LuaState) -> LuaResult<u32> {
    let at_npc = table_get(state, Val::Table(state.global), TRANSMOGRIFY_FRAME) != Val::Nil;
    state.push(Val::Bool(at_npc));
    Ok(1)
}

fn c_transmog_player_has_transmog_by_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(item_id) = parse_item_id_from_val(state, stack_val(state, 1)) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };

    let item_id = item_id as i32;
    let has_transmog = borrow_state(state)
        .map(|sim| {
            sim.world.collected_transmogs.contains(&item_id)
                || sim
                    .world
                    .transmog_appearances
                    .iter()
                    .any(|appearance| appearance.item_id == item_id && appearance.is_collected)
        })
        .unwrap_or(false);
    state.push(Val::Bool(has_transmog));
    Ok(1)
}

fn parse_item_id_from_val(state: &LuaState, value: Val) -> Option<u32> {
    match value {
        Val::Num(number) if number > 0.0 => Some(number as u32),
        Val::Str(_) => {
            let text = val_to_string(state, value)?;
            parse_prefixed_id(&text, "item").or_else(|| text.parse().ok())
        }
        _ => None,
    }
}

fn parse_prefixed_id(value: &str, prefix: &str) -> Option<u32> {
    let prefixed = format!("|H{prefix}:");
    if let Some(start) = value.find(&prefixed) {
        let digits: String = value[start + prefixed.len()..]
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect();
        return digits.parse().ok();
    }

    let bare = format!("{prefix}:");
    if let Some(start) = value.find(&bare) {
        let digits: String = value[start + bare.len()..]
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect();
        return digits.parse().ok();
    }

    None
}
