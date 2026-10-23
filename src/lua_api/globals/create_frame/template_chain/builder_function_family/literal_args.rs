//! Global calls with quoted string and numeric literals.

use super::{FastHandlerRef, quote_string};

pub(super) fn function_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    string_body(handler).or_else(|| number_body(handler))
}

fn string_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    let (name, args) = match handler {
        FastHandlerRef::FunctionWithStringArg { function_name, arg } => {
            (function_name, quote_string(arg))
        }
        FastHandlerRef::FunctionWithSelfStringArg { function_name, arg } => {
            (function_name, format!("self, {}", quote_string(arg)))
        }
        FastHandlerRef::FunctionWithStringNumberArgs {
            function_name,
            first,
            second,
        } => (function_name, format!("{}, {second}", quote_string(first))),
        FastHandlerRef::FunctionWithStringSelfStringNumberNumberArgs {
            function_name,
            first,
            third,
            fourth,
            fifth,
        } => (
            function_name,
            format!(
                "{}, self, {}, {fourth}, {fifth}",
                quote_string(first),
                quote_string(third)
            ),
        ),
        _ => return None,
    };
    Some(format!("return {name}({args})"))
}

fn number_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    match handler {
        FastHandlerRef::FunctionWithNumberArg {
            function_name,
            value,
        } => Some(format!("return {function_name}({value})")),
        FastHandlerRef::FunctionWithSelfNumberArg {
            function_name,
            value,
        } => Some(format!("return {function_name}(self, {value})")),
        _ => None,
    }
}
