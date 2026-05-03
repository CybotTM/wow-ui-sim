use super::FastHandlerRef;

pub(super) fn parse_inline_function_arg_shapes<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_checked_function_arg_shape(stmt)
        .or_else(|| parse_literal_and_global_function_arg_shape(stmt))
        .or_else(|| parse_function_result_arg_shape(stmt))
        .or_else(|| parse_self_parent_function_arg_shape(stmt))
}

fn parse_checked_function_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    checked_number_assignment_shape(stmt)
        .or_else(|| checked_assignments3_shape(stmt))
        .or_else(|| checked_assignment_two_callbacks_shape(stmt))
        .or_else(|| checked_assignment_callbacks_shape(stmt))
}

fn checked_number_assignment_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, field, value, on_change_function, on_sound_function) =
        super::parse_checked_number_assignment_then_callbacks(stmt)?;
    Some(FastHandlerRef::CheckedNumberAssignmentThenCallbacks {
        target_path,
        field,
        value,
        on_change_function,
        on_sound_function,
    })
}

fn checked_assignments3_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (
        first_target_path,
        first_field,
        second_target_path,
        second_field,
        third_target_path,
        third_field,
        on_change_function,
        on_sound_function,
    ) = super::parse_checked_assignments3_then_callbacks(stmt)?;
    Some(FastHandlerRef::CheckedAssignments3ThenCallbacks {
        first_target_path,
        first_field,
        second_target_path,
        second_field,
        third_target_path,
        third_field,
        on_change_function,
        on_sound_function,
    })
}

fn checked_assignment_two_callbacks_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, field, first_callback, second_callback, on_sound_function) =
        super::parse_checked_assignment_then_two_callbacks(stmt)?;
    Some(FastHandlerRef::CheckedAssignmentThenTwoCallbacks {
        target_path,
        field,
        first_callback,
        second_callback,
        on_sound_function,
    })
}

fn checked_assignment_callbacks_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, field, on_change_function, on_sound_function) =
        super::parse_checked_assignment_then_callbacks(stmt)?;
    Some(FastHandlerRef::CheckedAssignmentThenCallbacks {
        target_path,
        field,
        on_change_function,
        on_sound_function,
    })
}

fn parse_literal_and_global_function_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_string_literal_function_arg_shape(stmt)
        .or_else(|| parse_global_path_function_arg_shape(stmt))
}

fn parse_string_literal_function_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    string_nil_nil_global_args_shape(stmt)
        .or_else(|| string_number_args_shape(stmt))
        .or_else(|| string_global_bool_arg_shape(stmt))
        .or_else(|| string_arg_shape(stmt))
}

fn string_nil_nil_global_args_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, first, fourth) =
        super::parse_inline_function_with_string_nil_nil_global_args(stmt)?;
    Some(FastHandlerRef::FunctionWithStringNilNilGlobalArgs {
        function_name,
        first,
        fourth,
    })
}

fn string_number_args_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, first, second) =
        super::parse_inline_function_with_string_number_args(stmt)?;
    Some(FastHandlerRef::FunctionWithStringNumberArgs {
        function_name,
        first,
        second,
    })
}

fn string_global_bool_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, first, second_arg_path, third) =
        super::parse_inline_function_with_string_global_bool_arg(stmt)?;
    Some(FastHandlerRef::FunctionWithStringGlobalBoolArg {
        function_name,
        first,
        second_arg_path,
        third,
    })
}

fn string_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, arg) = super::parse_inline_function_with_string_arg(stmt)?;
    Some(FastHandlerRef::FunctionWithStringArg { function_name, arg })
}

fn parse_global_path_function_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    two_global_number_args_shape(stmt)
        .or_else(|| three_global_args_shape(stmt))
        .or_else(|| global_self_method_self_method_bool_args_shape(stmt))
        .or_else(|| two_global_args_shape(stmt))
}

fn two_global_number_args_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, first_arg_path, second_arg_path, third) =
        super::parse_inline_function_with_two_global_number_args(stmt)?;
    Some(FastHandlerRef::FunctionWithTwoGlobalNumberArgs {
        function_name,
        first_arg_path,
        second_arg_path,
        third,
    })
}

fn three_global_args_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, first_arg_path, second_arg_path, third_arg_path) =
        super::parse_inline_function_with_three_global_args(stmt)?;
    Some(FastHandlerRef::FunctionWithThreeGlobalArgs {
        function_name,
        first_arg_path,
        second_arg_path,
        third_arg_path,
    })
}

fn global_self_method_self_method_bool_args_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, first_arg_path, second_self_method, third_self_method, fourth) =
        super::parse_inline_function_with_global_self_method_self_method_bool_args(stmt)?;
    Some(
        FastHandlerRef::FunctionWithGlobalSelfMethodSelfMethodBoolArgs {
            function_name,
            first_arg_path,
            second_self_method,
            third_self_method,
            fourth,
        },
    )
}

fn two_global_args_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, first_arg_path, second_arg_path) =
        super::parse_inline_function_with_two_global_args(stmt)?;
    Some(FastHandlerRef::FunctionWithTwoGlobalArgs {
        function_name,
        first_arg_path,
        second_arg_path,
    })
}

fn parse_function_result_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    noarg_function_result_shape(stmt)
        .or_else(|| self_noarg_method_result_shape(stmt))
        .or_else(|| global_method_noargs_result_shape(stmt))
        .or_else(|| self_string_arg_shape(stmt))
}

fn noarg_function_result_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, arg_function_name) =
        super::parse_inline_function_with_noarg_function_result(stmt)?;
    Some(FastHandlerRef::FunctionWithNoArgFunctionResult {
        function_name,
        arg_function_name,
    })
}

fn self_noarg_method_result_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, method_name) =
        super::parse_inline_function_with_self_noarg_method_result(stmt)?;
    Some(FastHandlerRef::FunctionWithSelfNoArgsMethodResult {
        function_name,
        method_name,
    })
}

fn global_method_noargs_result_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, target_path, method_name) =
        super::parse_inline_function_with_global_method_noargs_result(stmt)?;
    Some(FastHandlerRef::FunctionWithGlobalMethodNoArgsResult {
        function_name,
        target_path,
        method_name,
    })
}

fn self_string_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, arg) = super::parse_inline_function_with_self_string_arg(stmt)?;
    Some(FastHandlerRef::FunctionWithSelfStringArg { function_name, arg })
}

fn parse_self_parent_function_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_self_and_number_function_arg_shape(stmt)
        .or_else(|| parse_global_and_self_function_arg_shape(stmt))
        .or_else(|| parse_parent_function_arg_shape(stmt))
}

fn parse_self_and_number_function_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    string_self_string_number_number_args_shape(stmt)
        .or_else(|| self_number_arg_shape(stmt))
        .or_else(|| number_arg_shape(stmt))
        .or_else(|| global_arg_shape(stmt))
}

fn string_self_string_number_number_args_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, first, third, fourth, fifth) =
        super::parse_inline_function_with_string_self_string_number_number_args(stmt)?;
    Some(
        FastHandlerRef::FunctionWithStringSelfStringNumberNumberArgs {
            function_name,
            first,
            third,
            fourth,
            fifth,
        },
    )
}

fn self_number_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, value) = super::parse_inline_function_with_self_number_arg(stmt)?;
    Some(FastHandlerRef::FunctionWithSelfNumberArg {
        function_name,
        value,
    })
}

fn number_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, value) = super::parse_inline_function_with_number_arg(stmt)?;
    Some(FastHandlerRef::FunctionWithNumberArg {
        function_name,
        value,
    })
}

fn global_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, arg_path) = super::parse_inline_function_with_global_arg(stmt)?;
    Some(FastHandlerRef::FunctionWithGlobalArg {
        function_name,
        arg_path,
    })
}

fn parse_global_and_self_function_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    global_and_self_id_arg_shape(stmt).or_else(|| global_and_self_arg_shape(stmt))
}

fn global_and_self_id_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, global_arg_path) =
        super::parse_inline_function_with_global_and_self_id_arg(stmt)?;
    Some(FastHandlerRef::FunctionWithGlobalAndSelfIdArg {
        function_name,
        global_arg_path,
    })
}

fn global_and_self_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, global_arg_path) =
        super::parse_inline_function_with_global_and_self_arg(stmt)?;
    Some(FastHandlerRef::FunctionWithGlobalAndSelfArg {
        function_name,
        global_arg_path,
    })
}

fn parse_parent_function_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parent_field_arg_shape(stmt)
        .or_else(|| parent_nested_method_result_shape(stmt))
        .or_else(|| self_and_parent_field_arg_shape(stmt))
}

fn parent_field_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, field) = super::parse_inline_function_with_parent_field_arg(stmt)?;
    Some(FastHandlerRef::FunctionWithParentFieldArg {
        function_name,
        field,
    })
}

fn parent_nested_method_result_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, first_field, second_field, third_field, method_name) =
        super::parse_inline_function_with_parent_field_and_nested_parent_field_method_result(stmt)?;
    Some(
        FastHandlerRef::FunctionWithParentFieldAndNestedParentFieldMethodResult {
            function_name,
            first_field,
            second_field,
            third_field,
            method_name,
        },
    )
}

fn self_and_parent_field_arg_shape<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, field) = super::parse_inline_function_with_self_and_parent_field_arg(stmt)?;
    Some(FastHandlerRef::FunctionWithSelfAndParentFieldArg {
        function_name,
        field,
    })
}

#[cfg(test)]
mod tests {
    use super::{FastHandlerRef, parse_inline_function_arg_shapes};

    #[test]
    fn parses_checked_number_assignment_callback_shape() {
        let handler = parse_inline_function_arg_shapes(
            "local checked = self:GetChecked(); SettingsPanel.Volume = 0.75; RefreshSettings(); PlaySound(checked)",
        );

        let Some(FastHandlerRef::CheckedNumberAssignmentThenCallbacks {
            target_path,
            field,
            value,
            on_change_function,
            on_sound_function,
        }) = handler
        else {
            panic!("expected checked number assignment handler");
        };

        assert_eq!(target_path, "SettingsPanel");
        assert_eq!(field, "Volume");
        assert_eq!(value, 0.75);
        assert_eq!(on_change_function, "RefreshSettings");
        assert_eq!(on_sound_function, "PlaySound");
    }

    #[test]
    fn parses_literal_and_global_arg_shape() {
        let handler = parse_inline_function_arg_shapes("ShowNamedPanel(\"Collections\")");

        let Some(FastHandlerRef::FunctionWithStringArg { function_name, arg }) = handler else {
            panic!("expected string argument handler");
        };

        assert_eq!(function_name, "ShowNamedPanel");
        assert_eq!(arg, "Collections");
    }

    #[test]
    fn parses_function_result_arg_shape() {
        let handler = parse_inline_function_arg_shapes("UseValue(GetCurrentValue())");

        let Some(FastHandlerRef::FunctionWithNoArgFunctionResult {
            function_name,
            arg_function_name,
        }) = handler
        else {
            panic!("expected no-arg function result handler");
        };

        assert_eq!(function_name, "UseValue");
        assert_eq!(arg_function_name, "GetCurrentValue");
    }

    #[test]
    fn parses_self_parent_arg_shape() {
        let handler =
            parse_inline_function_arg_shapes("HandleSelection(self, self:GetParent().SelectedTab)");

        let Some(FastHandlerRef::FunctionWithSelfAndParentFieldArg {
            function_name,
            field,
        }) = handler
        else {
            panic!("expected self and parent field argument handler");
        };

        assert_eq!(function_name, "HandleSelection");
        assert_eq!(field, "SelectedTab");
    }
}
