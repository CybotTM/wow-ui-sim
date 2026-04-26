//! `C_TransmogOutfitInfo` lock probes. `ActionBarButtonMixin:UpdateUsable`
//! reads both to gray out outfit-action buttons:
//!
//! - `IsLockedOutfit(outfitID)` → membership in `state.transmog_outfit_locks`.
//! - `IsEquippedGearOutfitLocked()` → `state.equipped_outfit_locked`.
//!
//! `C_TransmogOutfitInfo` namespace table is provided by the Lua bootstrap
//! `__wow_namespace` so other unimplemented members still fall through to
//! the no-op metamethod.

use crate::lua_api::methods::{borrow_state, create_table};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::closure::RustFn;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const TRANSMOG_OUTFIT_METHODS: &[(&str, RustFn)] = &[
    ("IsLockedOutfit", is_locked_outfit),
    ("IsEquippedGearOutfitLocked", is_equipped_gear_outfit_locked),
];

pub fn is_locked_outfit(state: &mut LuaState) -> LuaResult<u32> {
    let is_locked = match stack_val(state, 1) {
        Val::Num(id) => borrow_state(state)?
            .transmog_outfit_locks
            .contains(&(id as i64)),
        _ => false,
    };
    state.push(Val::Bool(is_locked));
    Ok(1)
}

pub fn is_equipped_gear_outfit_locked(state: &mut LuaState) -> LuaResult<u32> {
    let is_locked = borrow_state(state)?.equipped_outfit_locked;
    state.push(Val::Bool(is_locked));
    Ok(1)
}

fn ensure_namespace_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_TransmogOutfitInfo");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(r)) = existing {
        return r;
    }
    let new_val = create_table(state);
    let Val::Table(new_ref) = new_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_ref
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let table_ref = ensure_namespace_table(state);
    for &(name, func) in TRANSMOG_OUTFIT_METHODS {
        table_set_rust_fn_static(state, table_ref, name, func)?;
    }
    Ok(())
}
