//! Call arguments containing global paths remain Lua expressions, not captures.

use super::{FastHandlerRef, quote_string};

pub(super) fn function_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    global_body(handler)
        .or_else(|| mixed_body(handler))
        .or_else(|| global_self_body(handler))
}

fn global_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    let (name, args) = match handler {
        FastHandlerRef::FunctionWithGlobalArg {
            function_name,
            arg_path,
        } => (function_name, arg_path.to_string()),
        FastHandlerRef::FunctionWithTwoGlobalArgs {
            function_name,
            first_arg_path,
            second_arg_path,
        } => (
            function_name,
            format!("{first_arg_path}, {second_arg_path}"),
        ),
        FastHandlerRef::FunctionWithTwoGlobalNumberArgs {
            function_name,
            first_arg_path,
            second_arg_path,
            third,
        } => (
            function_name,
            format!("{first_arg_path}, {second_arg_path}, {third}"),
        ),
        FastHandlerRef::FunctionWithThreeGlobalArgs {
            function_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
        } => (
            function_name,
            format!("{first_arg_path}, {second_arg_path}, {third_arg_path}"),
        ),
        _ => return None,
    };
    Some(format!("return {name}({args})"))
}

fn mixed_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    let (name, args) = match handler {
        FastHandlerRef::FunctionWithStringGlobalBoolArg {
            function_name,
            first,
            second_arg_path,
            third,
        } => (
            function_name,
            format!("{}, {second_arg_path}, {third}", quote_string(first)),
        ),
        FastHandlerRef::FunctionWithStringNilNilGlobalArgs {
            function_name,
            first,
            fourth,
        } => (
            function_name,
            format!("{}, nil, nil, {fourth}", quote_string(first)),
        ),
        FastHandlerRef::FunctionWithGlobalSelfMethodSelfMethodBoolArgs {
            function_name,
            first_arg_path,
            second_self_method,
            third_self_method,
            fourth,
        } => (
            function_name,
            format!(
                "{first_arg_path}, self:{second_self_method}(), self:{third_self_method}(), {fourth}"
            ),
        ),
        _ => return None,
    };
    Some(format!("return {name}({args})"))
}

fn global_self_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    match handler {
        FastHandlerRef::FunctionWithGlobalAndSelfArg {
            function_name,
            global_arg_path,
        } => Some(format!("return {function_name}({global_arg_path}, self)")),
        FastHandlerRef::FunctionWithGlobalAndSelfIdArg {
            function_name,
            global_arg_path,
        } => Some(format!(
            "return {function_name}({global_arg_path}, self:GetID())"
        )),
        _ => None,
    }
}
