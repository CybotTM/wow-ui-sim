use super::FastHandlerRef;

pub(crate) fn parse_global_family<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_mode_or_conditional_global(stmt)
        .or_else(|| parse_literal_global_method(stmt))
        .or_else(|| parse_complex_global_method(stmt))
        .or_else(|| parse_simple_global_method(stmt))
        .or_else(|| parse_tooltip_global(stmt))
        .or_else(|| parse_misc_global(stmt))
}

fn parse_mode_or_conditional_global<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    lfg_mode_branch(stmt)
        .or_else(|| local_path_conditional_method(stmt))
        .or_else(|| conditional_global_noarg(stmt))
        .or_else(|| conditional_global_function_result(stmt))
        .or_else(|| conditional_global_field_equals_string(stmt))
}

fn lfg_mode_branch<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (category_path, slot_path, leave_function, join_function) =
        super::parse_get_lfg_mode_branch(stmt)?;
    Some(FastHandlerRef::GetLfgModeBranch {
        category_path,
        slot_path,
        leave_function,
        join_function,
    })
}

fn local_path_conditional_method<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name) = super::parse_local_global_path_conditional_method(stmt)?;
    Some(FastHandlerRef::LocalGlobalPathConditionalMethod {
        target_path,
        method_name,
    })
}

fn conditional_global_noarg<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, then_ref, else_ref) =
        super::parse_conditional_global_noarg_then_else(stmt)?;
    Some(FastHandlerRef::ConditionalGlobalNoArgs {
        function_name,
        then_ref: Box::new(then_ref),
        else_ref: Box::new(else_ref),
    })
}

fn conditional_global_function_result<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (function_name, arg_function_name, then_ref) =
        super::parse_conditional_global_function_with_noarg_function_result_then(stmt)?;
    Some(
        FastHandlerRef::ConditionalGlobalFunctionWithNoArgFunctionResultThen {
            function_name,
            arg_function_name,
            then_ref: Box::new(then_ref),
        },
    )
}

fn conditional_global_field_equals_string<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, field, value, then_ref) =
        super::parse_conditional_global_field_equals_string_then(stmt)?;
    Some(FastHandlerRef::ConditionalGlobalFieldEqualsStringThen {
        target_path,
        field,
        value,
        then_ref: Box::new(then_ref),
    })
}

fn parse_literal_global_method<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    global_method_then_assign(stmt)
        .or_else(|| global_method_self_string_arg(stmt))
        .or_else(|| global_method_self_string_number_number_args(stmt))
        .or_else(|| global_method_string_arg(stmt))
        .or_else(|| global_method_global_arg(stmt))
        .or_else(|| global_method_string_global_bool_args(stmt))
}

fn global_method_then_assign<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, field, value) =
        super::parse_inline_global_method_then_assign(stmt)?;
    Some(FastHandlerRef::GlobalMethodThenAssignLiteral {
        target_path,
        method_name,
        field,
        value,
    })
}

fn global_method_self_string_arg<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, arg) =
        super::parse_inline_global_method_with_self_string_arg(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithSelfStringArg {
        target_path,
        method_name,
        arg,
    })
}

fn global_method_self_string_number_number_args<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, first, second, third) =
        super::parse_inline_global_method_with_self_string_number_number_args(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithSelfStringNumberNumberArgs {
        target_path,
        method_name,
        first,
        second,
        third,
    })
}

fn global_method_string_arg<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, arg) = super::parse_inline_global_method_with_string_arg(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithStringArg {
        target_path,
        method_name,
        arg,
    })
}

fn global_method_global_arg<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, arg_path) =
        super::parse_inline_global_method_with_global_arg(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithGlobalArg {
        target_path,
        method_name,
        arg_path,
    })
}

fn global_method_string_global_bool_args<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, first, second_arg_path, third) =
        super::parse_inline_global_method_with_string_global_bool_args(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithStringGlobalBoolArgs {
        target_path,
        method_name,
        first,
        second_arg_path,
        third,
    })
}

fn parse_complex_global_method<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    global_method_global_three_global_bool_args(stmt)
        .or_else(|| global_method_global_nil_nil_nil_nil_bool_args(stmt))
        .or_else(|| global_method_global_self_method_self_method_bool_args(stmt))
        .or_else(|| global_method_four_global_args(stmt))
        .or_else(|| global_method_string_string_function_result_three_numbers(stmt))
        .or_else(|| global_method_global_string_function_result_three_numbers(stmt))
}

fn global_method_global_three_global_bool_args<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (
        target_path,
        method_name,
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
        fifth,
    ) = super::parse_inline_global_method_with_global_three_global_bool_args(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithGlobalThreeGlobalBoolArgs {
        target_path,
        method_name,
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
        fifth,
    })
}

fn global_method_global_nil_nil_nil_nil_bool_args<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, first_arg_path, sixth) =
        super::parse_inline_global_method_with_global_nil_nil_nil_nil_bool_args(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithGlobalNilNilNilNilBoolArgs {
        target_path,
        method_name,
        first_arg_path,
        sixth,
    })
}

fn global_method_global_self_method_self_method_bool_args<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, first_arg_path, second_self_method, third_self_method, fourth) =
        super::parse_inline_global_method_with_global_self_method_self_method_bool_args(stmt)?;
    Some(
        FastHandlerRef::GlobalMethodWithGlobalSelfMethodSelfMethodBoolArgs {
            target_path,
            method_name,
            first_arg_path,
            second_self_method,
            third_self_method,
            fourth,
        },
    )
}

fn global_method_four_global_args<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (
        target_path,
        method_name,
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
    ) = super::parse_inline_global_method_with_four_global_args(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithFourGlobalArgs {
        target_path,
        method_name,
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
    })
}

fn global_method_string_string_function_result_three_numbers<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, function_name, first, second, third, fourth, fifth) =
        super::parse_inline_global_method_with_string_string_function_result_and_three_number_args(
            stmt,
        )?;
    Some(
        FastHandlerRef::GlobalMethodWithStringStringFunctionResultAndThreeNumberArgs {
            target_path,
            method_name,
            function_name,
            first,
            second,
            third,
            fourth,
            fifth,
        },
    )
}

fn global_method_global_string_function_result_three_numbers<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, function_name, first_arg_path, second, third, fourth, fifth) =
        super::parse_inline_global_method_with_global_string_function_result_and_three_number_args(
            stmt,
        )?;
    Some(
        FastHandlerRef::GlobalMethodWithGlobalStringFunctionResultAndThreeNumberArgs {
            target_path,
            method_name,
            function_name,
            first_arg_path,
            second,
            third,
            fourth,
            fifth,
        },
    )
}

fn parse_simple_global_method<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    global_method_self_arg(stmt)
        .or_else(|| global_method_self_id_arg(stmt))
        .or_else(|| global_method_no_args(stmt))
}

fn global_method_self_arg<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name) = super::parse_inline_global_method_with_self_arg(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithSelfArg {
        target_path,
        method_name,
    })
}

fn global_method_self_id_arg<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name) = super::parse_inline_global_method_with_self_id_arg(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithSelfIdArg {
        target_path,
        method_name,
    })
}

fn global_method_no_args<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name) = super::parse_inline_global_method(stmt)?;
    Some(FastHandlerRef::GlobalMethod {
        target_path,
        method_name,
    })
}

fn parse_tooltip_global<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    tooltip_literal(stmt)
        .or_else(|| tooltip_set_text(stmt))
        .or_else(|| conditional_tooltip(stmt))
}

fn tooltip_literal<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, anchor, text, red, green, blue) =
        super::parse_global_tooltip_set_owner_then_set_text_literal(stmt)?;
    Some(FastHandlerRef::GlobalTooltipSetOwnerThenSetTextLiteral {
        target_path,
        anchor,
        text,
        red,
        green,
        blue,
    })
}

fn tooltip_set_text<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, anchor, text_path, red_path, green_path, blue_path, wrap) =
        super::parse_global_tooltip_set_owner_then_set_text(stmt)?;
    Some(FastHandlerRef::GlobalTooltipSetOwnerThenSetText {
        target_path,
        anchor,
        text_path,
        red_path,
        green_path,
        blue_path,
        wrap,
    })
}

fn conditional_tooltip<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, field, anchor, red_path, green_path, blue_path) =
        super::parse_conditional_tooltip(stmt)?;
    Some(FastHandlerRef::ConditionalTooltip {
        target_path,
        field,
        anchor,
        red_path,
        green_path,
        blue_path,
    })
}

fn parse_misc_global<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    toggle_global_visibility(stmt)
        .or_else(|| named_global_method_global_arg(stmt))
        .or_else(|| global_method_self_field_arg(stmt))
}

fn toggle_global_visibility<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let target_path = super::parse_toggle_global_visibility(stmt)?;
    Some(FastHandlerRef::ToggleGlobalVisibility { target_path })
}

fn named_global_method_global_arg<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (suffix, method_name, arg_path) =
        super::parse_inline_named_global_method_with_global_arg(stmt)?;
    Some(FastHandlerRef::NamedGlobalMethodWithGlobalArg {
        suffix,
        method_name,
        arg_path,
    })
}

fn global_method_self_field_arg<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let (target_path, method_name, field) =
        super::parse_inline_global_method_with_self_field_arg(stmt)?;
    Some(FastHandlerRef::GlobalMethodWithSelfFieldArg {
        target_path,
        method_name,
        field,
    })
}

#[cfg(test)]
mod tests {
    use super::{FastHandlerRef, parse_global_family};

    #[test]
    fn parses_lfg_mode_branch_before_other_global_shapes() {
        let handler = parse_global_family(
            "local mode, subMode = GetLFGMode(categoryPath, slotPath); if ( mode == \"queued\" or mode == \"listed\" or mode == \"rolecheck\" or mode == \"suspended\" ) then LeaveQueue(categoryPath, slotPath); else JoinQueue() end",
        );

        let Some(FastHandlerRef::GetLfgModeBranch {
            category_path,
            slot_path,
            leave_function,
            join_function,
        }) = handler
        else {
            panic!("expected LFG mode branch");
        };

        assert_eq!(category_path, "categoryPath");
        assert_eq!(slot_path, Some("slotPath"));
        assert_eq!(leave_function, "LeaveQueue");
        assert_eq!(join_function, "JoinQueue");
    }

    #[test]
    fn parses_basic_global_method_shape() {
        let handler = parse_global_family("SettingsPanel:Refresh()");

        let Some(FastHandlerRef::GlobalMethod {
            target_path,
            method_name,
        }) = handler
        else {
            panic!("expected global method");
        };

        assert_eq!(target_path, "SettingsPanel");
        assert_eq!(method_name, "Refresh");
    }

    #[test]
    fn parses_tooltip_literal_shape() {
        let handler = parse_global_family(
            "GameTooltip:SetOwner(self, \"ANCHOR_RIGHT\"); GameTooltip:SetText(\"Title\", 1, 0.5, 0)",
        );

        let Some(FastHandlerRef::GlobalTooltipSetOwnerThenSetTextLiteral {
            target_path,
            anchor,
            text,
            red,
            green,
            blue,
        }) = handler
        else {
            panic!("expected tooltip literal");
        };

        assert_eq!(target_path, "GameTooltip");
        assert_eq!(anchor, "ANCHOR_RIGHT");
        assert_eq!(text, "Title");
        assert_eq!((red, green, blue), (1.0, 0.5, 0.0));
    }
}
