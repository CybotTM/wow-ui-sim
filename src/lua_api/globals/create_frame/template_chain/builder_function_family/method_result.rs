//! Calls forwarding another function or method's result.

use super::FastHandlerRef;

pub(super) fn function_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    let (name, arg) = match handler {
        FastHandlerRef::FunctionWithNoArgFunctionResult {
            function_name,
            arg_function_name,
        } => (function_name, format!("{arg_function_name}()")),
        FastHandlerRef::FunctionWithSelfNoArgsMethodResult {
            function_name,
            method_name,
        } => (function_name, format!("self:{method_name}()")),
        FastHandlerRef::FunctionWithGlobalMethodNoArgsResult {
            function_name,
            target_path,
            method_name,
        } => (function_name, format!("{target_path}:{method_name}()")),
        _ => return None,
    };
    Some(format!("return {name}({arg})"))
}
