//! Restored rilua API surface for item, spell, tooltip, and small legacy globals.

mod item_spell;
mod professions;
mod tooltip_info;
mod traits;

use crate::lua_api::rilua_methods::{borrow_state_mut, create_string, val_to_string};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val};

const TOOLTIP_TYPE_ITEM: f64 = 0.0;
const TOOLTIP_TYPE_SPELL: f64 = 1.0;
const TOOLTIP_TYPE_UNIT: f64 = 2.0;
const TOOLTIP_TYPE_UNIT_AURA: f64 = 7.0;
const TOOLTIP_TYPE_MINIMAP_MOUSEOVER: f64 = 21.0;

const LINE_TYPE_UNIT_NAME: f64 = 2.0;
const LINE_TYPE_SPELL_NAME: f64 = 13.0;
const LINE_TYPE_ITEM_BINDING: f64 = 20.0;
const LINE_TYPE_EQUIP_SLOT: f64 = 21.0;
const LINE_TYPE_ITEM_NAME: f64 = 22.0;
const LINE_TYPE_ITEM_LEVEL: f64 = 31.0;
const LINE_TYPE_SPELL_DESCRIPTION: f64 = 34.0;
const WORLD_LOOT_TOOLTIP_SPELL_ID: u32 = 19750;
const WORLD_LOOT_TOOLTIP_INVENTORY_TYPE: f64 = 13.0;
const WORLD_CURSOR_GUID: &str = "WorldLootObject-0000-0000C0DE";

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "PlaySound", noop)?;
    LuaApiMut::register_function(lua, "PlaySoundFile", noop)?;
    LuaApiMut::register_function(lua, "StopSound", noop)?;
    LuaApiMut::register_function(lua, "GetSpellLink", get_spell_link_global)?;
    LuaApiMut::register_function(lua, "GetRepairAllCost", get_repair_all_cost)?;
    LuaApiMut::register_function(lua, "SetActionUIButton", set_action_ui_button)?;
    LuaApiMut::register_function(lua, "MapSceneCharacterHighlightStart", noop)?;
    LuaApiMut::register_function(lua, "MapSceneCharacterHighlightEnd", noop)?;
    LuaApiMut::register_function(lua, "CreateAtlasMarkup", create_atlas_markup)?;
    LuaApiMut::register_function(lua, "InGlue", in_glue)?;
    LuaApiMut::register_function(lua, "strsub", strsub)?;

    let state = lua.state_mut();
    ensure_global_table(state, "UISpecialFrames");
    ensure_global_table(state, "StaticPopupDialogs");
    ensure_global_table(state, "UIPanelWindows");
    ensure_global_table(state, "SOUNDKIT");
    item_spell::register_item_and_spell_surfaces(state)?;
    professions::register_profession_surface(state)?;
    traits::register_trait_surfaces(state)?;
    tooltip_info::register_tooltip_surface(state)?;
    Ok(())
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn get_spell_link_global(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    match item_spell::spell_link_for_id(spell_id) {
        Some(link) => {
            let link = create_string(state, &link);
            state.push(link);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_repair_all_cost(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    Ok(2)
}

fn set_action_ui_button(state: &mut LuaState) -> LuaResult<u32> {
    let button = stack_val(state, 1);
    let action = u32::from_stack(state, 2)?;
    let Some(button_id) = crate::lua_api::rilua_methods::extract_frame_id(state, button) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    sim.action_ui_buttons.retain(|(id, _)| *id != button_id);
    sim.action_ui_buttons.push((button_id, action));
    Ok(0)
}

fn create_atlas_markup(state: &mut LuaState) -> LuaResult<u32> {
    let atlas_name = match stack_val(state, 1) {
        Val::Str(_) => val_to_string(state, stack_val(state, 1)).unwrap_or_default(),
        _ => String::new(),
    };
    let text = if atlas_name.is_empty() {
        String::new()
    } else {
        format!("|A:{atlas_name}:0:0|a")
    };
    let value = create_string(state, &text);
    state.push(value);
    Ok(1)
}

fn in_glue(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn strsub(state: &mut LuaState) -> LuaResult<u32> {
    let Some(text) = val_to_string(state, stack_val(state, 1)) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let start = i32::from_stack(state, 2).unwrap_or(1);
    let end = i32::from_stack(state, 3).unwrap_or(-1);
    let len = text.chars().count() as i32;
    let normalize = |index: i32| {
        if index < 0 {
            (len + index + 1).max(1)
        } else {
            index.max(1)
        }
    };
    let start = normalize(start);
    let end = normalize(end).min(len);
    let result = if start > end || start > len {
        String::new()
    } else {
        text.chars()
            .skip((start - 1) as usize)
            .take((end - start + 1) as usize)
            .collect::<String>()
    };
    let value = create_string(state, &result);
    state.push(value);
    Ok(1)
}

fn ensure_global_table(state: &mut LuaState, name: &str) {
    let _ = ensure_namespace(state, name);
}

pub(super) fn ensure_namespace(state: &mut LuaState, name: &str) -> LuaResult<GcRef<Table>> {
    let key_ref = state.gc.intern_string(name.as_bytes());
    let current = state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    let table_ref = match current {
        Val::Table(table_ref) => table_ref,
        _ => {
            let table_ref = state.gc.alloc_table(Table::new());
            let global = state.global;
            if let Some(globals) = state.gc.tables.get_mut(global) {
                let _ = globals.raw_set(
                    Val::Str(key_ref),
                    Val::Table(table_ref),
                    &state.gc.string_arena,
                );
            }
            state.gc.barrier_back(global);
            table_ref
        }
    };
    Ok(table_ref)
}

pub(super) fn set_table_array(state: &mut LuaState, table: Val, index: i64, value: Val) {
    let Val::Table(table_ref) = table else { return };
    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        let _ = table.raw_set(Val::Num(index as f64), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(table_ref);
}
