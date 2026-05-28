//! Per-frame debug field table helpers.

use rilua::Val;
use rilua::vm::state::LuaState;

use super::{frame_ref, registry_get, table_get_num};

/// Return the frame table used for normal per-frame Lua fields.
pub fn get_or_create_frame_fields(state: &mut LuaState, frame_id: u64) -> Val {
    frame_ref(state, frame_id).unwrap_or(Val::Nil)
}

/// Return an existing frame fields table without creating a frame ref.
pub fn get_existing_frame_fields(state: &mut LuaState, frame_id: u64) -> Val {
    existing_frame_ref(state, frame_id).unwrap_or(Val::Nil)
}

fn existing_frame_ref(state: &mut LuaState, frame_id: u64) -> Option<Val> {
    let cache = registry_get(state, "__rilua_frame_refs");
    let Val::Table(cache_ref) = cache else {
        return None;
    };
    let existing = table_get_num(state, cache_ref, frame_id as f64);
    if existing == Val::Nil {
        None
    } else {
        Some(existing)
    }
}
