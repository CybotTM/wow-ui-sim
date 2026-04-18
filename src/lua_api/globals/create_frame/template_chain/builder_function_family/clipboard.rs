//! Clipboard-copy specials are currently forced through the generic chunk path.

use super::super::FastHandlerRef;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_clipboard_variants(
    _state: &mut LuaState,
    _handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    Ok(None)
}
