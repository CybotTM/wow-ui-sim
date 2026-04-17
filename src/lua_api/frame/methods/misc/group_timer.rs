//! Group creation, timer updates, frame/font-string registration, and quest blob methods.

use crate::lua_api::methods::{
    borrow_state_mut, call_function_state, create_string, create_table, extract_frame_id,
    frame_id_from_stack, get_or_create_frame_fields, table_get, table_get_static, table_set,
    val_to_string,
};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn register(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, mt, "GetOrCreateGroup", get_or_create_group)?;
    table_set_rust_fn(state, mt, "ForceUpdateTimers", force_update_timers)?;
    table_set_rust_fn(state, mt, "RegisterFontStrings", register_font_strings)?;
    table_set_rust_fn(
        state,
        mt,
        "RegisterBackgroundTexture",
        register_background_texture,
    )?;
    table_set_rust_fn(state, mt, "RegisterFrames", register_frames)?;
    table_set_rust_fn(state, mt, "SetBorderAlpha", set_border_alpha)?;
    table_set_rust_fn(state, mt, "SetBorderScalar", set_border_scalar)?;
    table_set_rust_fn(state, mt, "SetBorderTexture", set_border_texture)?;
    table_set_rust_fn(state, mt, "SetFillAlpha", set_fill_alpha)?;
    table_set_rust_fn(state, mt, "SetOwningDialog", set_owning_dialog)?;
    table_set_rust_fn(state, mt, "SetFillTexture", set_fill_texture)?;
    table_set_rust_fn(state, mt, "SetToDefaults", set_to_defaults)?;
    table_set_rust_fn(state, mt, "DrawNone", draw_none)?;
    Ok(())
}

// ── GetOrCreateGroup ──────────────────────────────────────────────────────────

pub fn get_or_create_group(state: &mut LuaState) -> LuaResult<u32> {
    let self_table = stack_val(state, 1);
    let group_text = String::from_stack(state, 2)?;
    let order = read_order(state);
    let groups = get_or_create_groups_table(state, self_table);
    let result = find_or_insert_group(state, groups, &group_text, order);
    state.push(result);
    Ok(1)
}

fn read_order(state: &mut LuaState) -> f64 {
    match stack_val(state, 3) {
        Val::Num(v) => v,
        _ => 10.0,
    }
}

fn get_or_create_groups_table(state: &mut LuaState, self_table: Val) -> Val {
    match table_get(state, self_table, "groups") {
        t @ Val::Table(_) => t,
        _ => {
            let t = create_table(state);
            table_set(state, self_table, "groups", t);
            t
        }
    }
}

fn find_or_insert_group(state: &mut LuaState, groups: Val, group_text: &str, order: f64) -> Val {
    let Val::Table(groups_ref) = groups else {
        return Val::Nil;
    };
    let existing = state
        .gc
        .tables
        .get(groups_ref)
        .map(|t| t.array_slice().to_vec())
        .unwrap_or_default();

    if let Some(found) = find_existing_group(state, &existing, group_text) {
        return found;
    }
    insert_new_group(state, groups_ref, &existing, group_text, order)
}

fn find_existing_group(state: &mut LuaState, existing: &[Val], group_text: &str) -> Option<Val> {
    for &entry in existing {
        let name = table_get(state, entry, "groupText");
        if val_to_string(state, name).as_deref() == Some(group_text) {
            return Some(entry);
        }
    }
    None
}

fn insert_new_group(
    state: &mut LuaState,
    groups_ref: GcRef<Table>,
    existing: &[Val],
    group_text: &str,
    order: f64,
) -> Val {
    let group = create_table(state);
    let name_val = create_string(state, group_text);
    let categories = create_table(state);
    table_set(state, group, "groupText", name_val);
    table_set(state, group, "order", Val::Num(order));
    table_set(state, group, "categories", categories);
    if let Some(table) = state.gc.tables.get_mut(groups_ref) {
        let _ = table.raw_set(
            Val::Num((existing.len() + 1) as f64),
            group,
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(groups_ref);
    group
}

// ── ForceUpdateTimers ─────────────────────────────────────────────────────────

pub fn force_update_timers(state: &mut LuaState) -> LuaResult<u32> {
    let self_table = stack_val(state, 1);
    let active_timers = table_get(state, self_table, "activeTimers");
    let Val::Table(active_timers_ref) = active_timers else {
        return Ok(0);
    };
    let timers = state
        .gc
        .tables
        .get(active_timers_ref)
        .map(|t| t.hash_entries())
        .unwrap_or_default();
    for (_, timer) in timers {
        let update_fn = table_get_static(state, timer, "OnUpdate");
        if matches!(update_fn, Val::Function(_)) {
            let _ = call_function_state(state, update_fn, &[timer]);
        }
    }
    Ok(0)
}

// ── Frame/FontString registration ─────────────────────────────────────────────

pub fn register_font_strings(state: &mut LuaState) -> LuaResult<u32> {
    let fields = frame_fields_from_self(state)?;
    let font_strings = collect_varargs_table(state, 2);
    table_set(state, fields, "fontStrings", font_strings);
    table_set(state, fields, "__registeredFontStrings", font_strings);
    Ok(0)
}

pub fn register_background_texture(state: &mut LuaState) -> LuaResult<u32> {
    let fields = frame_fields_from_self(state)?;
    let background = stack_val(state, 2);
    let texture_kit = stack_val(state, 3);
    table_set(state, fields, "backgroundTexture", background);
    table_set(state, fields, "textureKit", texture_kit);
    Ok(0)
}

pub fn register_frames(state: &mut LuaState) -> LuaResult<u32> {
    let fields = frame_fields_from_self(state)?;
    let frames = collect_varargs_table(state, 2);
    table_set(state, fields, "frames", frames);
    Ok(0)
}

pub fn set_owning_dialog(state: &mut LuaState) -> LuaResult<u32> {
    let fields = frame_fields_from_self(state)?;
    let dialog = stack_val(state, 2);
    table_set(state, fields, "owningDialog", dialog);
    table_set(state, fields, "OwningDialog", dialog);
    Ok(0)
}

fn frame_fields_from_self(state: &mut LuaState) -> LuaResult<Val> {
    let frame = stack_val(state, 1);
    let Some(id) = extract_frame_id(state, frame) else {
        return Ok(Val::Nil);
    };
    Ok(get_or_create_frame_fields(state, id))
}

fn collect_varargs_table(state: &mut LuaState, start: i32) -> Val {
    let table = create_table(state);
    let mut out_index = 1i64;
    let mut input_index = start;
    loop {
        let value = stack_val(state, input_index);
        if value == Val::Nil {
            break;
        }
        set_array_entry(state, table, out_index, value);
        out_index += 1;
        input_index += 1;
    }
    table
}

fn set_array_entry(state: &mut LuaState, table: Val, index: i64, value: Val) {
    let Val::Table(table_ref) = table else { return };
    if let Some(t) = state.gc.tables.get_mut(table_ref) {
        let _ = t.raw_set(Val::Num(index as f64), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(table_ref);
}

// ── Quest blob fields ─────────────────────────────────────────────────────────

pub fn set_fill_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = val_to_string(state, stack_val(state, 2));
    borrow_state_mut(state)?
        .quest_blobs
        .entry(id)
        .or_default()
        .fill_texture = texture;
    Ok(0)
}

pub fn set_fill_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let alpha = match stack_val(state, 2) {
        Val::Num(value) => Some(value),
        _ => None,
    };
    borrow_state_mut(state)?
        .quest_blobs
        .entry(id)
        .or_default()
        .fill_alpha = alpha;
    Ok(0)
}

pub fn set_border_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = val_to_string(state, stack_val(state, 2));
    borrow_state_mut(state)?
        .quest_blobs
        .entry(id)
        .or_default()
        .border_texture = texture;
    Ok(0)
}

pub fn set_border_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let alpha = match stack_val(state, 2) {
        Val::Num(value) => Some(value),
        _ => None,
    };
    borrow_state_mut(state)?
        .quest_blobs
        .entry(id)
        .or_default()
        .border_alpha = alpha;
    Ok(0)
}

pub fn set_border_scalar(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scalar = match stack_val(state, 2) {
        Val::Num(value) => Some(value),
        _ => None,
    };
    borrow_state_mut(state)?
        .quest_blobs
        .entry(id)
        .or_default()
        .border_scalar = scalar;
    Ok(0)
}

pub fn set_to_defaults(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.minimap_player_texture = None;
            frame.minimap_ping_position = None;
        }
    }
    let fields = get_or_create_frame_fields(state, id);
    table_set(state, fields, "__minimap_zoom", Val::Num(0.0));
    Ok(0)
}

pub fn draw_none(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    borrow_state_mut(state)?
        .quest_blobs
        .entry(id)
        .or_default()
        .active_quests
        .clear();
    Ok(0)
}
