//! Fast-path builders for XML inline function handlers. Each submodule owns
//! one shape-family (plain, literal args, global args, ancestor fields,
//! method results, checked assignments, clipboard, ancestor walks) and
//! exports its own `pub(super) fn build_*_variants` dispatcher.

use super::{FastHandlerRef, load_template};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

mod ancestor;
mod ancestor_field;
mod checked;
mod clipboard;
mod global_args;
mod literal_args;
mod method_result;
mod plain;

pub(super) fn build_function_family_handler(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = plain::build_plain_function_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = build_function_with_arg_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = ancestor::build_ancestor_function_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    Ok(None)
}

/// Per-argument shapes: string / number / global / self+field / global+self combinations.
///
/// Dispatches through six themed sub-helpers. Each returns `Ok(None)` for
/// arms it doesn't handle so the chain falls through to the next.
fn build_function_with_arg_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = literal_args::build_literal_arg_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = global_args::build_global_arg_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = ancestor_field::build_ancestor_field_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = method_result::build_method_result_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = checked::build_checked_assignment_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    clipboard::build_clipboard_variants(state, handler_ref)
}

/// Compile `template` (tagged with `tag`), marshal `args` into the new VM
/// stack, and invoke the builder closure to produce the runtime handler.
/// Shared by the checked-assignment family.
pub(super) fn instantiate_template_with_args(
    state: &mut LuaState,
    template: &str,
    tag: &str,
    args: &[Val],
) -> LuaResult<Val> {
    let builder = load_template(state, template, tag)?;
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), args)
}
