use super::{FastHandlerRef, is_fast_handler_path, is_fast_identifier};

pub(super) fn parse_function_family<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_direct_function_shapes(stmt)
        .or_else(|| parse_inline_function_arg_shapes(stmt))
        .or_else(|| parse_ancestor_function_shapes(stmt))
        .or_else(|| parse_event_function_shapes(stmt))
}

fn parse_direct_function_shapes<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    if let Some(function_name) = parse_global_function_suffix(stmt, "(self)") {
        return Some(FastHandlerRef::Function(function_name));
    }
    if let Some(function_name) = parse_global_function_suffix(stmt, "()") {
        return Some(FastHandlerRef::FunctionNoArgs(function_name));
    }
    if let Some(function_name) = parse_global_function_suffix(stmt, "(self:GetText())") {
        return Some(FastHandlerRef::FunctionWithSelfGetTextResult(function_name));
    }
    parse_global_function_suffix(stmt, "(self:GetID())").map(FastHandlerRef::FunctionWithSelfIdArg)
}

fn parse_inline_function_arg_shapes<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    if let Some((target_path, field, value, on_change_function, on_sound_function)) =
        parse_checked_number_assignment_then_callbacks(stmt)
    {
        return Some(FastHandlerRef::CheckedNumberAssignmentThenCallbacks {
            target_path,
            field,
            value,
            on_change_function,
            on_sound_function,
        });
    }
    if let Some((
        first_target_path,
        first_field,
        second_target_path,
        second_field,
        third_target_path,
        third_field,
        on_change_function,
        on_sound_function,
    )) = parse_checked_assignments3_then_callbacks(stmt)
    {
        return Some(FastHandlerRef::CheckedAssignments3ThenCallbacks {
            first_target_path,
            first_field,
            second_target_path,
            second_field,
            third_target_path,
            third_field,
            on_change_function,
            on_sound_function,
        });
    }
    if let Some((target_path, field, first_callback, second_callback, on_sound_function)) =
        parse_checked_assignment_then_two_callbacks(stmt)
    {
        return Some(FastHandlerRef::CheckedAssignmentThenTwoCallbacks {
            target_path,
            field,
            first_callback,
            second_callback,
            on_sound_function,
        });
    }
    if let Some((target_path, field, on_change_function, on_sound_function)) =
        parse_checked_assignment_then_callbacks(stmt)
    {
        return Some(FastHandlerRef::CheckedAssignmentThenCallbacks {
            target_path,
            field,
            on_change_function,
            on_sound_function,
        });
    }
    if let Some((function_name, first, fourth)) =
        parse_inline_function_with_string_nil_nil_global_args(stmt)
    {
        return Some(FastHandlerRef::FunctionWithStringNilNilGlobalArgs {
            function_name,
            first,
            fourth,
        });
    }
    if let Some((function_name, first, second)) =
        parse_inline_function_with_string_number_args(stmt)
    {
        return Some(FastHandlerRef::FunctionWithStringNumberArgs {
            function_name,
            first,
            second,
        });
    }
    if let Some((function_name, first_arg_path, second_arg_path, third)) =
        parse_inline_function_with_two_global_number_args(stmt)
    {
        return Some(FastHandlerRef::FunctionWithTwoGlobalNumberArgs {
            function_name,
            first_arg_path,
            second_arg_path,
            third,
        });
    }
    if let Some((function_name, first_arg_path, second_arg_path, third_arg_path)) =
        parse_inline_function_with_three_global_args(stmt)
    {
        return Some(FastHandlerRef::FunctionWithThreeGlobalArgs {
            function_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
        });
    }
    if let Some((function_name, first_arg_path, second_self_method, third_self_method, fourth)) =
        parse_inline_function_with_global_self_method_self_method_bool_args(stmt)
    {
        return Some(
            FastHandlerRef::FunctionWithGlobalSelfMethodSelfMethodBoolArgs {
                function_name,
                first_arg_path,
                second_self_method,
                third_self_method,
                fourth,
            },
        );
    }
    if let Some((function_name, first, second_arg_path, third)) =
        parse_inline_function_with_string_global_bool_arg(stmt)
    {
        return Some(FastHandlerRef::FunctionWithStringGlobalBoolArg {
            function_name,
            first,
            second_arg_path,
            third,
        });
    }
    if let Some((function_name, arg)) = parse_inline_function_with_string_arg(stmt) {
        return Some(FastHandlerRef::FunctionWithStringArg { function_name, arg });
    }
    if let Some((function_name, first_arg_path, second_arg_path)) =
        parse_inline_function_with_two_global_args(stmt)
    {
        return Some(FastHandlerRef::FunctionWithTwoGlobalArgs {
            function_name,
            first_arg_path,
            second_arg_path,
        });
    }
    if let Some((function_name, arg_function_name)) =
        parse_inline_function_with_noarg_function_result(stmt)
    {
        return Some(FastHandlerRef::FunctionWithNoArgFunctionResult {
            function_name,
            arg_function_name,
        });
    }
    if let Some((function_name, method_name)) =
        parse_inline_function_with_self_noarg_method_result(stmt)
    {
        return Some(FastHandlerRef::FunctionWithSelfNoArgsMethodResult {
            function_name,
            method_name,
        });
    }
    if let Some((function_name, target_path, method_name)) =
        parse_inline_function_with_global_method_noargs_result(stmt)
    {
        return Some(FastHandlerRef::FunctionWithGlobalMethodNoArgsResult {
            function_name,
            target_path,
            method_name,
        });
    }
    if let Some((function_name, arg)) = parse_inline_function_with_self_string_arg(stmt) {
        return Some(FastHandlerRef::FunctionWithSelfStringArg { function_name, arg });
    }
    if let Some((function_name, first, third, fourth, fifth)) =
        parse_inline_function_with_string_self_string_number_number_args(stmt)
    {
        return Some(
            FastHandlerRef::FunctionWithStringSelfStringNumberNumberArgs {
                function_name,
                first,
                third,
                fourth,
                fifth,
            },
        );
    }
    if let Some((function_name, value)) = parse_inline_function_with_self_number_arg(stmt) {
        return Some(FastHandlerRef::FunctionWithSelfNumberArg {
            function_name,
            value,
        });
    }
    if let Some((function_name, value)) = parse_inline_function_with_number_arg(stmt) {
        return Some(FastHandlerRef::FunctionWithNumberArg {
            function_name,
            value,
        });
    }
    if let Some((function_name, arg_path)) = parse_inline_function_with_global_arg(stmt) {
        return Some(FastHandlerRef::FunctionWithGlobalArg {
            function_name,
            arg_path,
        });
    }
    if let Some((function_name, global_arg_path)) =
        parse_inline_function_with_global_and_self_id_arg(stmt)
    {
        return Some(FastHandlerRef::FunctionWithGlobalAndSelfIdArg {
            function_name,
            global_arg_path,
        });
    }
    if let Some((function_name, global_arg_path)) =
        parse_inline_function_with_global_and_self_arg(stmt)
    {
        return Some(FastHandlerRef::FunctionWithGlobalAndSelfArg {
            function_name,
            global_arg_path,
        });
    }
    if let Some((function_name, field)) = parse_inline_function_with_parent_field_arg(stmt) {
        return Some(FastHandlerRef::FunctionWithParentFieldArg {
            function_name,
            field,
        });
    }
    if let Some((function_name, first_field, second_field, third_field, method_name)) =
        parse_inline_function_with_parent_field_and_nested_parent_field_method_result(stmt)
    {
        return Some(
            FastHandlerRef::FunctionWithParentFieldAndNestedParentFieldMethodResult {
                function_name,
                first_field,
                second_field,
                third_field,
                method_name,
            },
        );
    }
    parse_inline_function_with_self_and_parent_field_arg(stmt).map(|(function_name, field)| {
        FastHandlerRef::FunctionWithSelfAndParentFieldArg {
            function_name,
            field,
        }
    })
}

fn parse_ancestor_function_shapes<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    if let Some(function_name) = parse_global_function_suffix(stmt, "(self:GetParent())") {
        return Some(FastHandlerRef::FunctionWithParentArg(function_name));
    }
    if let Some(function_name) =
        parse_global_function_suffix(stmt, "(self:GetParent():GetParent())")
    {
        return Some(FastHandlerRef::FunctionWithGrandparentArg(function_name));
    }
    parse_global_function_suffix(stmt, "(self:GetParent():GetID())")
        .map(FastHandlerRef::FunctionWithParentIdArg)
}

fn parse_event_function_shapes<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    if let Some(function_name) = parse_global_function_suffix(stmt, "(self, event, ...)") {
        return Some(FastHandlerRef::FunctionWithEventVarargs(function_name));
    }
    if let Some(function_name) = parse_global_function_suffix(stmt, "(self, button)") {
        return Some(FastHandlerRef::FunctionWithButton(function_name));
    }
    parse_global_function_suffix(stmt, "(self, elapsed)").map(FastHandlerRef::FunctionWithElapsed)
}

fn parse_global_function_suffix<'a>(stmt: &'a str, suffix: &str) -> Option<&'a str> {
    stmt.strip_suffix(suffix)
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
}

fn parse_inline_function_with_self_string_arg(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (self_arg, raw_string_arg) = args.split_once(',')?;
    let function_name = function_name.trim();
    let arg = super::parse_single_string_literal(raw_string_arg.trim())?;
    (is_fast_handler_path(function_name) && self_arg.trim() == "self")
        .then_some((function_name, arg))
}

fn parse_inline_function_with_string_self_string_number_number_args(
    stmt: &str,
) -> Option<(&str, &str, &str, f64, f64)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let args = super::split_top_level_args(args)?;
    if args.len() != 5 {
        return None;
    }
    let first = super::parse_single_string_literal(args[0].trim())?;
    let self_arg = args[1].trim();
    let third = super::parse_single_string_literal(args[2].trim())?;
    let fourth = args[3].trim().parse::<f64>().ok()?;
    let fifth = args[4].trim().parse::<f64>().ok()?;
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name) && self_arg == "self").then_some((
        function_name,
        first,
        third,
        fourth,
        fifth,
    ))
}

fn parse_inline_function_with_string_arg(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let arg = super::parse_single_string_literal(args.strip_suffix(')')?.trim())?;
    let function_name = function_name.trim();
    is_fast_handler_path(function_name).then_some((function_name, arg))
}

fn parse_inline_function_with_string_number_args(stmt: &str) -> Option<(&str, &str, f64)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let args = super::split_top_level_args(args)?;
    if args.len() != 2 {
        return None;
    }
    let first = super::parse_single_string_literal(args[0].trim())?;
    let second = args[1].trim().parse::<f64>().ok()?;
    let function_name = function_name.trim();
    is_fast_handler_path(function_name).then_some((function_name, first, second))
}

fn parse_inline_function_with_two_global_number_args(
    stmt: &str,
) -> Option<(&str, &str, &str, f64)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let args = super::split_top_level_args(args)?;
    if args.len() != 3 {
        return None;
    }
    let first_arg_path = args[0].trim();
    let second_arg_path = args[1].trim();
    let third = args[2].trim().parse::<f64>().ok()?;
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name)
        && is_fast_handler_path(first_arg_path)
        && first_arg_path.split('.').next() != Some("self")
        && is_fast_handler_path(second_arg_path)
        && second_arg_path.split('.').next() != Some("self"))
    .then_some((function_name, first_arg_path, second_arg_path, third))
}

fn parse_inline_function_with_two_global_args(stmt: &str) -> Option<(&str, &str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let args = super::split_top_level_args(args)?;
    if args.len() != 2 {
        return None;
    }
    let function_name = function_name.trim();
    let first_arg_path = args[0].trim();
    let second_arg_path = args[1].trim();
    (is_fast_handler_path(function_name)
        && is_fast_handler_path(first_arg_path)
        && first_arg_path.split('.').next() != Some("self")
        && is_fast_handler_path(second_arg_path)
        && second_arg_path.split('.').next() != Some("self"))
    .then_some((function_name, first_arg_path, second_arg_path))
}

fn parse_inline_function_with_three_global_args(stmt: &str) -> Option<(&str, &str, &str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let args = super::split_top_level_args(args)?;
    if args.len() != 3 {
        return None;
    }
    let first_arg_path = args[0].trim();
    let second_arg_path = args[1].trim();
    let third_arg_path = args[2].trim();
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name)
        && is_fast_handler_path(first_arg_path)
        && is_fast_handler_path(second_arg_path)
        && is_fast_handler_path(third_arg_path)
        && first_arg_path.split('.').next() != Some("self")
        && second_arg_path.split('.').next() != Some("self")
        && third_arg_path.split('.').next() != Some("self"))
    .then_some((
        function_name,
        first_arg_path,
        second_arg_path,
        third_arg_path,
    ))
}

fn parse_inline_function_with_string_global_bool_arg(
    stmt: &str,
) -> Option<(&str, &str, &str, bool)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let mut parts = args.split(',').map(str::trim);
    let first = super::parse_single_string_literal(parts.next()?)?;
    let second_arg_path = parts.next()?;
    let third = super::parse_single_bool_literal(parts.next()?)?;
    if parts.next().is_some() {
        return None;
    }
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name)
        && is_fast_handler_path(second_arg_path)
        && second_arg_path.split('.').next() != Some("self"))
    .then_some((function_name, first, second_arg_path, third))
}

fn parse_inline_function_with_global_self_method_self_method_bool_args(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, bool)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let mut parts = args.split(',').map(str::trim);
    let first_arg_path = parts.next()?;
    let second = parts.next()?;
    let third = parts.next()?;
    let fourth = super::parse_single_bool_literal(parts.next()?)?;
    if parts.next().is_some() {
        return None;
    }
    let second_self_method = second.strip_prefix("self:")?.strip_suffix("()")?.trim();
    let third_self_method = third.strip_prefix("self:")?.strip_suffix("()")?.trim();
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name)
        && is_fast_handler_path(first_arg_path)
        && is_fast_identifier(second_self_method)
        && is_fast_identifier(third_self_method))
    .then_some((
        function_name,
        first_arg_path,
        second_self_method,
        third_self_method,
        fourth,
    ))
}

fn parse_inline_function_with_string_nil_nil_global_args(stmt: &str) -> Option<(&str, &str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let mut parts = args.split(',').map(str::trim);
    let first = super::parse_single_string_literal(parts.next()?)?;
    (parts.next()? == "nil").then_some(())?;
    (parts.next()? == "nil").then_some(())?;
    let fourth = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name) && is_fast_handler_path(fourth)).then_some((
        function_name,
        first,
        fourth,
    ))
}

fn parse_inline_function_with_noarg_function_result(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let arg_function_name = args.strip_suffix("())")?.trim();
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name) && is_fast_handler_path(arg_function_name))
        .then_some((function_name, arg_function_name))
}

fn parse_inline_function_with_self_noarg_method_result(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let method_name = args
        .strip_suffix("())")?
        .trim()
        .strip_prefix("self:")?
        .trim();
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name) && is_fast_identifier(method_name))
        .then_some((function_name, method_name))
}

fn parse_inline_function_with_global_method_noargs_result(
    stmt: &str,
) -> Option<(&str, &str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let arg_expr = args.strip_suffix("())")?.trim();
    let (target_path, method_name) = arg_expr.rsplit_once(':')?;
    let function_name = function_name.trim();
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(function_name)
        && is_fast_handler_path(target_path)
        && target_path != "self"
        && is_fast_identifier(method_name))
    .then_some((function_name, target_path, method_name))
}

fn parse_inline_function_with_number_arg(stmt: &str) -> Option<(&str, f64)> {
    let (function_name, args) = stmt.split_once('(')?;
    let value = args.strip_suffix(')')?.trim().parse::<f64>().ok()?;
    let function_name = function_name.trim();
    is_fast_handler_path(function_name).then_some((function_name, value))
}

fn parse_inline_function_with_self_number_arg(stmt: &str) -> Option<(&str, f64)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (self_arg, raw_number_arg) = args.split_once(',')?;
    let value = raw_number_arg.trim().parse::<f64>().ok()?;
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name) && self_arg.trim() == "self")
        .then_some((function_name, value))
}

fn parse_inline_function_with_global_arg(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let arg_path = args.strip_suffix(')')?.trim();
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name) && is_fast_handler_path(arg_path))
        .then_some((function_name, arg_path))
}

fn parse_inline_function_with_global_and_self_arg(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (global_arg_path, self_arg) = args.split_once(',')?;
    let function_name = function_name.trim();
    let global_arg_path = global_arg_path.trim();
    (is_fast_handler_path(function_name)
        && is_fast_handler_path(global_arg_path)
        && self_arg.trim() == "self")
        .then_some((function_name, global_arg_path))
}

fn parse_inline_function_with_global_and_self_id_arg(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (global_arg_path, self_arg) = args.split_once(',')?;
    let function_name = function_name.trim();
    let global_arg_path = global_arg_path.trim();
    (is_fast_handler_path(function_name)
        && is_fast_handler_path(global_arg_path)
        && self_arg.trim() == "self:GetID()")
        .then_some((function_name, global_arg_path))
}

fn parse_inline_function_with_parent_field_arg(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let field = args
        .strip_suffix(')')?
        .trim()
        .strip_prefix("self:GetParent().")?
        .trim();
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name) && is_fast_identifier(field))
        .then_some((function_name, field))
}

fn parse_inline_function_with_parent_field_and_nested_parent_field_method_result(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (first_arg, second_arg) = args.split_once(',')?;
    let first_field = first_arg.trim().strip_prefix("self:GetParent().")?.trim();
    if !is_fast_identifier(first_field) {
        return None;
    }

    let second_arg = second_arg.trim();
    let second_arg = second_arg.strip_prefix("self:GetParent().")?.trim();
    let (target_path, method_name) = second_arg.rsplit_once(':')?;
    let method_name = method_name.strip_suffix("()")?.trim();
    let mut fields = target_path.split('.').map(str::trim);
    let second_field = fields.next()?;
    let third_field = fields.next()?;
    if fields.next().is_some() {
        return None;
    }

    let function_name = function_name.trim();
    (is_fast_handler_path(function_name)
        && is_fast_identifier(second_field)
        && is_fast_identifier(third_field)
        && is_fast_identifier(method_name))
    .then_some((
        function_name,
        first_field,
        second_field,
        third_field,
        method_name,
    ))
}

fn parse_inline_function_with_self_and_parent_field_arg(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (self_arg, parent_field) = args.split_once(',')?;
    let field = self_arg
        .trim()
        .eq("self")
        .then_some(parent_field.trim())?
        .strip_prefix("self:GetParent().")?
        .trim();
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name) && is_fast_identifier(field))
        .then_some((function_name, field))
}

fn parse_checked_assignment_then_callbacks(stmt: &str) -> Option<(&str, &str, &str, &str)> {
    let stmt = stmt.trim();
    let prefix = "local checked = self:GetChecked()";
    let remainder = stmt.strip_prefix(prefix)?.trim_start();
    let remainder = remainder.strip_prefix("if")?.trim_start();
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let (condition, remainder) = remainder.split_once("then")?;
    if condition.trim_end().strip_suffix(')')?.trim() != "checked" {
        return None;
    }
    let remainder = remainder.trim_start();
    let (then_stmt, else_tail) = remainder.split_once("else")?;
    let else_tail = else_tail.trim_start();
    let (else_stmt, after_end) = else_tail.split_once("end")?;
    let then_stmt = then_stmt.trim().strip_suffix(';')?.trim();
    let else_stmt = else_stmt.trim().strip_suffix(';')?.trim();

    let (then_path, then_field, then_value) = parse_global_bool_assignment(then_stmt)?;
    let (else_path, else_field, else_value) = parse_global_bool_assignment(else_stmt)?;
    if then_path != else_path
        || then_field != else_field
        || then_value != true
        || else_value != false
    {
        return None;
    }

    let after_end = after_end.trim();
    let parts = super::split_inline_sequence_parts(after_end);
    let [on_change_stmt, on_sound_stmt] = parts.as_slice() else {
        return None;
    };
    let on_change_function = parse_global_function_suffix(on_change_stmt.trim(), "()")?;
    let (on_sound_function, args) = on_sound_stmt.trim().split_once('(')?;
    if args.strip_suffix(')')?.trim() != "checked" {
        return None;
    }
    (is_fast_handler_path(then_path)
        && is_fast_identifier(then_field)
        && is_fast_handler_path(on_change_function)
        && is_fast_handler_path(on_sound_function.trim()))
    .then_some((
        then_path,
        then_field,
        on_change_function,
        on_sound_function.trim(),
    ))
}

fn parse_checked_number_assignment_then_callbacks(
    stmt: &str,
) -> Option<(&str, &str, f64, &str, &str)> {
    let stmt = stmt.trim();
    let prefix = "local checked = self:GetChecked();";
    let remainder = stmt.strip_prefix(prefix)?.trim_start();
    let parts = super::split_inline_sequence_parts(remainder);
    let [assign_stmt, on_change_stmt, on_sound_stmt] = parts.as_slice() else {
        return None;
    };
    let (target_path, field, value) = parse_global_number_assignment(assign_stmt.trim())?;
    let on_change_function = parse_global_function_suffix(on_change_stmt.trim(), "()")?;
    let (on_sound_function, args) = on_sound_stmt.trim().split_once('(')?;
    if args.strip_suffix(')')?.trim() != "checked" {
        return None;
    }
    Some((
        target_path,
        field,
        value,
        on_change_function,
        on_sound_function.trim(),
    ))
}

fn parse_checked_assignments3_then_callbacks(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, &str, &str, &str, &str)> {
    let (then_parts, else_parts, after_end) = parse_checked_then_else(stmt)?;
    let [then_first, then_second, then_third] = then_parts.as_slice() else {
        return None;
    };
    let [else_first, else_second, else_third] = else_parts.as_slice() else {
        return None;
    };
    let (first_target_path, first_field, first_then_value) =
        parse_global_bool_assignment(then_first)?;
    let (second_target_path, second_field, second_then_value) =
        parse_global_bool_assignment(then_second)?;
    let (third_target_path, third_field, third_then_value) =
        parse_global_bool_assignment(then_third)?;
    let (first_else_path, first_else_field, first_else_value) =
        parse_global_bool_assignment(else_first)?;
    let (second_else_path, second_else_field, second_else_value) =
        parse_global_bool_assignment(else_second)?;
    let (third_else_path, third_else_field, third_else_value) =
        parse_global_bool_assignment(else_third)?;
    if first_then_value != true
        || second_then_value != true
        || third_then_value != true
        || first_else_value != false
        || second_else_value != false
        || third_else_value != false
        || first_target_path != first_else_path
        || first_field != first_else_field
        || second_target_path != second_else_path
        || second_field != second_else_field
        || third_target_path != third_else_path
        || third_field != third_else_field
    {
        return None;
    }
    let parts = super::split_inline_sequence_parts(after_end);
    let [on_change_stmt, on_sound_stmt] = parts.as_slice() else {
        return None;
    };
    let on_change_function = parse_global_function_suffix(on_change_stmt.trim(), "()")?;
    let (on_sound_function, args) = on_sound_stmt.trim().split_once('(')?;
    if args.strip_suffix(')')?.trim() != "checked" {
        return None;
    }
    Some((
        first_target_path,
        first_field,
        second_target_path,
        second_field,
        third_target_path,
        third_field,
        on_change_function,
        on_sound_function.trim(),
    ))
}

fn parse_checked_assignment_then_two_callbacks(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, &str)> {
    let (then_parts, else_parts, after_end) = parse_checked_then_else(stmt)?;
    let [then_stmt] = then_parts.as_slice() else {
        return None;
    };
    let [else_stmt] = else_parts.as_slice() else {
        return None;
    };
    let (then_path, then_field, then_value) = parse_global_bool_assignment(then_stmt)?;
    let (else_path, else_field, else_value) = parse_global_bool_assignment(else_stmt)?;
    if then_path != else_path
        || then_field != else_field
        || then_value != true
        || else_value != false
    {
        return None;
    }
    let parts = super::split_inline_sequence_parts(after_end);
    let [first_callback_stmt, second_callback_stmt, on_sound_stmt] = parts.as_slice() else {
        return None;
    };
    let first_callback = parse_global_function_suffix(first_callback_stmt.trim(), "()")?;
    let second_callback = parse_global_function_suffix(second_callback_stmt.trim(), "()")?;
    let (on_sound_function, args) = on_sound_stmt.trim().split_once('(')?;
    if args.strip_suffix(')')?.trim() != "checked" {
        return None;
    }
    Some((
        then_path,
        then_field,
        first_callback,
        second_callback,
        on_sound_function.trim(),
    ))
}

fn parse_checked_then_else(stmt: &str) -> Option<(Vec<&str>, Vec<&str>, &str)> {
    let stmt = stmt.trim();
    let prefix = "local checked = self:GetChecked()";
    let remainder = stmt.strip_prefix(prefix)?.trim_start();
    let remainder = remainder.strip_prefix("if")?.trim_start();
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let (condition, remainder) = remainder.split_once("then")?;
    if condition.trim_end().strip_suffix(')')?.trim() != "checked" {
        return None;
    }
    let remainder = remainder.trim_start();
    let (then_stmt, else_tail) = remainder.split_once("else")?;
    let else_tail = else_tail.trim_start();
    let (else_stmt, after_end) = else_tail.split_once("end")?;
    let then_parts = super::split_inline_sequence_parts(then_stmt.trim());
    let else_parts = super::split_inline_sequence_parts(else_stmt.trim());
    Some((then_parts, else_parts, after_end.trim()))
}

fn parse_global_bool_assignment(stmt: &str) -> Option<(&str, &str, bool)> {
    let (lhs, rhs) = stmt.split_once('=')?;
    let value = super::parse_single_bool_literal(rhs.trim())?;
    let (target_path, field) = lhs.trim().rsplit_once('.')?;
    Some((target_path.trim(), field.trim(), value))
}

fn parse_global_number_assignment(stmt: &str) -> Option<(&str, &str, f64)> {
    let (lhs, rhs) = stmt.split_once('=')?;
    let value = rhs
        .trim()
        .trim_end_matches(';')
        .trim()
        .parse::<f64>()
        .ok()?;
    let (target_path, field) = lhs.trim().rsplit_once('.')?;
    Some((target_path.trim(), field.trim(), value))
}
