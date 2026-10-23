//! Plain global calls, with paths validated by the inline parser.

use super::FastHandlerRef;

pub(super) fn function_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    let (name, args) = match handler {
        FastHandlerRef::FunctionWithSelfArg(name) => (name, "self"),
        FastHandlerRef::FunctionNoArgs(name) => (name, ""),
        FastHandlerRef::FunctionWithSelfGetTextResult(name) => (name, "self:GetText()"),
        FastHandlerRef::FunctionWithSelfIdArg(name) => (name, "self:GetID()"),
        FastHandlerRef::FunctionWithEventVarargs(name) => (name, "self, event, ..."),
        FastHandlerRef::FunctionWithButton(name) => (name, "self, button"),
        FastHandlerRef::FunctionWithElapsed(name) => (name, "self, elapsed"),
        _ => return None,
    };
    Some(format!("return {name}({args})"))
}
