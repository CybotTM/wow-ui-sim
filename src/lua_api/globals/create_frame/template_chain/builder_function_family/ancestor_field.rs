//! Calls forwarding fields and method results from the frame's parent.

use super::FastHandlerRef;

pub(super) fn function_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    let (name, args) = match handler {
        FastHandlerRef::FunctionWithParentFieldArg {
            function_name,
            field,
        } => (function_name, format!("self:GetParent().{field}")),
        FastHandlerRef::FunctionWithSelfAndParentFieldArg {
            function_name,
            field,
        } => (function_name, format!("self, self:GetParent().{field}")),
        FastHandlerRef::FunctionWithParentFieldAndNestedParentFieldMethodResult {
            function_name,
            first_field,
            second_field,
            third_field,
            method_name,
        } => (
            function_name,
            format!(
                "self:GetParent().{first_field}, self:GetParent().{second_field}.{third_field}:{method_name}()"
            ),
        ),
        _ => return None,
    };
    Some(format!("return {name}({args})"))
}
