//! Checked assignments and their callbacks read the handler's environment.

use super::FastHandlerRef;

pub(super) fn function_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    single_assignment_body(handler).or_else(|| triple_assignment_body(handler))
}

fn single_assignment_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    let (assignment, callbacks) = match handler {
        FastHandlerRef::CheckedAssignmentThenCallbacks {
            target_path,
            field,
            on_change_function,
            on_sound_function,
        } => (
            format!("{target_path}.{field} = checked and true or false"),
            format!("{on_change_function}(); {on_sound_function}(checked)"),
        ),
        FastHandlerRef::CheckedAssignmentThenTwoCallbacks {
            target_path,
            field,
            first_callback,
            second_callback,
            on_sound_function,
        } => (
            format!("{target_path}.{field} = checked and true or false"),
            format!("{first_callback}(); {second_callback}(); {on_sound_function}(checked)"),
        ),
        FastHandlerRef::CheckedNumberAssignmentThenCallbacks {
            target_path,
            field,
            value,
            on_change_function,
            on_sound_function,
        } => (
            format!("{target_path}.{field} = {value}"),
            format!("{on_change_function}(); {on_sound_function}(checked)"),
        ),
        _ => return None,
    };
    Some(format!(
        "local checked = self:GetChecked(); {assignment}; {callbacks}"
    ))
}

fn triple_assignment_body(handler: &FastHandlerRef<'_>) -> Option<String> {
    let FastHandlerRef::CheckedAssignments3ThenCallbacks {
        first_target_path,
        first_field,
        second_target_path,
        second_field,
        third_target_path,
        third_field,
        on_change_function,
        on_sound_function,
    } = handler
    else {
        return None;
    };
    Some(format!(
        "local checked = self:GetChecked(); \
         {first_target_path}.{first_field} = checked and true or false; \
         {second_target_path}.{second_field} = checked and true or false; \
         {third_target_path}.{third_field} = checked and true or false; \
         {on_change_function}(); {on_sound_function}(checked)"
    ))
}
