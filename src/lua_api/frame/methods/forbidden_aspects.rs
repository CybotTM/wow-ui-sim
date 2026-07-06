#[cfg(feature = "retail-12-1-0")]
use crate::lua_api::methods::table_get;
use crate::lua_api::methods::{get_or_create_frame_fields, table_set};
use rilua::Val;
use rilua::vm::state::LuaState;
#[cfg(feature = "retail-12-1-0")]
use rilua::{LuaResult, runtime_error};

pub(crate) const INHERITANCE_PARENT: u64 = 1;
pub(crate) const INHERITANCE_LAYOUT: u64 = 2;

const FORBIDDEN_ASPECTS_KEY: &str = "__forbiddenAspects";
const INHERITABLE_PARENT_KEY: &str = "__forbiddenAspectsParent";
const INHERITABLE_LAYOUT_KEY: &str = "__forbiddenAspectsLayout";

#[cfg(feature = "retail-12-1-0")]
pub(crate) fn stored_forbidden_aspects(state: &mut LuaState, frame_id: u64) -> u64 {
    read_mask_field(state, frame_id, FORBIDDEN_ASPECTS_KEY)
}

pub(crate) fn set_forbidden_aspects(state: &mut LuaState, frame_id: u64, mask: u64) {
    write_mask_field(state, frame_id, FORBIDDEN_ASPECTS_KEY, mask);
    write_mask_field(state, frame_id, INHERITABLE_PARENT_KEY, mask);
    write_mask_field(state, frame_id, INHERITABLE_LAYOUT_KEY, mask);
}

pub(crate) fn set_inheritable_forbidden_aspects(
    state: &mut LuaState,
    frame_id: u64,
    parent_mask: u64,
    layout_mask: u64,
) {
    write_mask_field(state, frame_id, INHERITABLE_PARENT_KEY, parent_mask);
    write_mask_field(state, frame_id, INHERITABLE_LAYOUT_KEY, layout_mask);
}

#[cfg(feature = "retail-12-1-0")]
pub(crate) fn stored_inheritable_forbidden_aspects(
    state: &mut LuaState,
    frame_id: u64,
    inheritance: u64,
) -> u64 {
    let mut mask = 0;
    if inheritance & INHERITANCE_PARENT != 0 {
        mask |= read_inheritable_mask(state, frame_id, INHERITABLE_PARENT_KEY);
    }
    if inheritance & INHERITANCE_LAYOUT != 0 {
        mask |= read_inheritable_mask(state, frame_id, INHERITABLE_LAYOUT_KEY);
    }
    mask
}

#[cfg(feature = "retail-12-1-0")]
pub(crate) fn ensure_forbidden_aspects_already_owned(
    state: &mut LuaState,
    frame_id: u64,
    source_frame_id: u64,
    inheritance: u64,
    method_name: &str,
) -> LuaResult<()> {
    let source_aspects = stored_inheritable_forbidden_aspects(state, source_frame_id, inheritance);
    let frame_aspects = stored_forbidden_aspects(state, frame_id);
    let missing_aspects = source_aspects & !frame_aspects;
    if missing_aspects == 0 {
        return Ok(());
    }

    Err(runtime_error(format!(
        "Action[{method_name}] failed because[Cannot implicitly gain forbidden aspects]"
    )))
}

#[cfg(feature = "retail-12-1-0")]
fn read_inheritable_mask(state: &mut LuaState, frame_id: u64, key: &str) -> u64 {
    read_optional_mask_field(state, frame_id, key)
        .unwrap_or_else(|| stored_forbidden_aspects(state, frame_id))
}

#[cfg(feature = "retail-12-1-0")]
fn read_mask_field(state: &mut LuaState, frame_id: u64, key: &str) -> u64 {
    read_optional_mask_field(state, frame_id, key).unwrap_or(0)
}

#[cfg(feature = "retail-12-1-0")]
fn read_optional_mask_field(state: &mut LuaState, frame_id: u64, key: &str) -> Option<u64> {
    let fields = get_or_create_frame_fields(state, frame_id);
    match table_get(state, fields, key) {
        Val::Num(value) if value >= 0.0 => Some(value as u64),
        _ => None,
    }
}

fn write_mask_field(state: &mut LuaState, frame_id: u64, key: &str, mask: u64) {
    let fields = get_or_create_frame_fields(state, frame_id);
    table_set(state, fields, key, Val::Num(mask as f64));
}
