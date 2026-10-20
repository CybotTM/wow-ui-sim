//! Parent and grandparent arguments use the same expressions as inline Lua.

use super::FastHandlerRef;

pub(super) fn function_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    let (name, arg) = match handler {
        FastHandlerRef::FunctionWithParentArg(name) => (name, "self:GetParent()"),
        FastHandlerRef::FunctionWithGrandparentArg(name) => (name, "self:GetParent():GetParent()"),
        FastHandlerRef::FunctionWithParentIdArg(name) => (name, "self:GetParent():GetID()"),
        _ => return None,
    };
    Some(format!("return {name}({arg})"))
}
