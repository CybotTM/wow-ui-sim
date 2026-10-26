//! Function-family handlers use Lua expressions so globals are read at invocation.

use super::{FastHandlerRef, load_template};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

mod ancestor;
mod ancestor_field;
mod checked;
mod global_args;
mod literal_args;
mod method_result;
mod plain;

pub(super) fn build_function_family_handler(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    // XML function= binds a value; inline calls instead read their environment.
    if let FastHandlerRef::Function(name) = handler_ref {
        let target =
            crate::lua_api::globals::create_frame::helpers::resolve_global_path(state, name);
        return Ok(Some(target));
    }
    let body = plain::function_body(handler_ref)
        .or_else(|| literal_args::function_body(handler_ref))
        .or_else(|| global_args::function_body(handler_ref))
        .or_else(|| ancestor::function_body(handler_ref))
        .or_else(|| ancestor_field::function_body(handler_ref))
        .or_else(|| method_result::function_body(handler_ref))
        .or_else(|| checked::function_body(handler_ref));
    let Some(body) = body else {
        return Ok(None);
    };
    let params = match handler_ref {
        FastHandlerRef::FunctionWithEventVarargs(_) => "self, event, ...",
        FastHandlerRef::FunctionWithButton(_) => "self, button, ...",
        FastHandlerRef::FunctionWithElapsed(_) => "self, elapsed, ...",
        _ => "self, ...",
    };
    let source = format!("return function({params}) {body} end");
    let builder = load_template(state, &source, "template-inline-function-late-bound")?;
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[])
        .map(Some)
}

fn quote_string(value: &str) -> String {
    format!("\"{}\"", crate::loader::helpers::escape_lua_string(value))
}
