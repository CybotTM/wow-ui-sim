//! C_Texture: atlas info and existence queries.

use crate::atlas::AtlasLookup;
use crate::lua_api::methods::{create_string, create_table, val_to_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

use super::helpers::set_global_val;

pub fn register_c_texture(state: &mut LuaState) -> LuaResult<()> {
    let c_texture = create_table(state);
    let Val::Table(c_texture_ref) = c_texture else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn_static(
        state,
        c_texture_ref,
        "GetAtlasInfo",
        c_texture_get_atlas_info,
    )?;
    table_set_rust_fn_static(
        state,
        c_texture_ref,
        "GetAtlasExists",
        c_texture_get_atlas_exists,
    )?;
    set_global_val(state, "C_Texture", c_texture);
    Ok(())
}

pub fn c_texture_get_atlas_exists(state: &mut LuaState) -> LuaResult<u32> {
    let atlas_name = val_to_string(state, stack_val(state, 1));
    state.push(Val::Bool(
        atlas_name
            .as_deref()
            .and_then(crate::atlas::get_atlas_info)
            .is_some(),
    ));
    Ok(1)
}

pub fn c_texture_get_atlas_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(atlas_name) = val_to_string(state, stack_val(state, 1)) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let Some(lookup) = crate::atlas::get_atlas_info(&atlas_name) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    let Val::Table(info_ref) = info else {
        unreachable!("create_table must return a table");
    };
    let raw_size = build_raw_size(state, lookup.width() as f64, lookup.height() as f64);
    fill_atlas_info_table(state, info_ref, &atlas_name, &lookup);
    attach_raw_size(state, info_ref, raw_size);
    state.push(info);
    Ok(1)
}

fn build_raw_size(state: &mut LuaState, width: f64, height: f64) -> Val {
    let raw_size = create_table(state);
    let Val::Table(raw_size_ref) = raw_size else {
        unreachable!("create_table must return a table");
    };
    if let Some(table) = state.gc.tables.get_mut(raw_size_ref) {
        let _ = table.raw_set(Val::Num(1.0), Val::Num(width), &state.gc.string_arena);
        let _ = table.raw_set(Val::Num(2.0), Val::Num(height), &state.gc.string_arena);
    }
    state.gc.barrier_back(raw_size_ref);
    raw_size
}

fn fill_atlas_info_table(
    state: &mut LuaState,
    info_ref: GcRef<Table>,
    atlas_name: &str,
    lookup: &AtlasLookup,
) {
    set_str_static(state, info_ref, "elementName", atlas_name);
    set_num_static(state, info_ref, "width", lookup.width() as f64);
    set_num_static(state, info_ref, "height", lookup.height() as f64);
    fill_atlas_tex_coords(state, info_ref, lookup);
    fill_atlas_tile_flags(state, info_ref, lookup);
    set_str_static(state, info_ref, "filename", lookup.info.file);
}

fn fill_atlas_tex_coords(state: &mut LuaState, info_ref: GcRef<Table>, lookup: &AtlasLookup) {
    let coords: [(&'static str, f32); 4] = [
        ("leftTexCoord", lookup.info.left_tex_coord),
        ("rightTexCoord", lookup.info.right_tex_coord),
        ("topTexCoord", lookup.info.top_tex_coord),
        ("bottomTexCoord", lookup.info.bottom_tex_coord),
    ];
    for (key, value) in coords {
        set_num_static(state, info_ref, key, value as f64);
    }
}

fn fill_atlas_tile_flags(state: &mut LuaState, info_ref: GcRef<Table>, lookup: &AtlasLookup) {
    set_bool_static(
        state,
        info_ref,
        "tilesHorizontally",
        lookup.info.tiles_horizontally,
    );
    set_bool_static(
        state,
        info_ref,
        "tilesVertically",
        lookup.info.tiles_vertically,
    );
}

fn attach_raw_size(state: &mut LuaState, info_ref: GcRef<Table>, raw_size: Val) {
    let raw_size_key = state.gc.intern_string_static(b"rawSize");
    if let Some(table) = state.gc.tables.get_mut(info_ref) {
        let _ = table.raw_set(Val::Str(raw_size_key), raw_size, &state.gc.string_arena);
    }
    state.gc.barrier_back(info_ref);
}

fn set_str_static(state: &mut LuaState, info_ref: GcRef<Table>, key: &'static str, value: &str) {
    let k = state.gc.intern_string_static(key.as_bytes());
    let v = create_string(state, value);
    if let Some(table) = state.gc.tables.get_mut(info_ref) {
        let _ = table.raw_set(Val::Str(k), v, &state.gc.string_arena);
    }
    state.gc.barrier_back(info_ref);
}

fn set_num_static(state: &mut LuaState, info_ref: GcRef<Table>, key: &'static str, value: f64) {
    let k = state.gc.intern_string_static(key.as_bytes());
    if let Some(table) = state.gc.tables.get_mut(info_ref) {
        let _ = table.raw_set(Val::Str(k), Val::Num(value), &state.gc.string_arena);
    }
    state.gc.barrier_back(info_ref);
}

fn set_bool_static(state: &mut LuaState, info_ref: GcRef<Table>, key: &'static str, value: bool) {
    let k = state.gc.intern_string_static(key.as_bytes());
    if let Some(table) = state.gc.tables.get_mut(info_ref) {
        let _ = table.raw_set(Val::Str(k), Val::Bool(value), &state.gc.string_arena);
    }
    state.gc.barrier_back(info_ref);
}
