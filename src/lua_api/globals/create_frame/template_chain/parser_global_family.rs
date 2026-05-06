use super::{FastHandlerRef, FastLiteralValue, is_fast_handler_path, is_fast_identifier};

#[path = "parser_global_dispatch.rs"]
mod parser_global_dispatch;

pub(super) use self::parser_global_dispatch::parse_global_family;

fn parse_local_global_path_conditional_method(stmt: &str) -> Option<(&str, &str)> {
    let stmt = stmt.trim();
    let remainder = stmt.strip_prefix("local ")?;
    let (local_name, remainder) = remainder.split_once('=')?;
    let local_name = local_name.trim();
    if !is_fast_identifier(local_name) {
        return None;
    }
    let target_path = remainder.trim();
    let (target_path, _tail) = target_path
        .split_once('\n')
        .or_else(|| target_path.split_once("if"))?;
    let target_path = target_path.trim();
    let tail = stmt[stmt.find("if")?..].trim();
    let prefix = format!("if ({local_name}) then");
    let remainder = tail.strip_prefix(&prefix)?.trim_start();
    let body = remainder.strip_suffix("end")?.trim();
    let expected_prefix = format!("{local_name}:");
    let method_stmt = body.strip_prefix(&expected_prefix)?;
    let method_name = method_stmt
        .trim()
        .trim_end_matches(';')
        .strip_suffix("()")?
        .trim();
    (is_fast_handler_path(target_path) && is_fast_identifier(method_name))
        .then_some((target_path, method_name))
}

fn parse_get_lfg_mode_branch(stmt: &str) -> Option<(&str, Option<&str>, &str, &str)> {
    let stmt = stmt.trim();
    let prefix = "local mode, subMode = GetLFGMode(";
    let remainder = stmt.strip_prefix(prefix)?;
    let (args, remainder) = remainder.split_once(");")?;
    let mut parts = args.split(',').map(str::trim);
    let category_path = parts.next()?;
    let slot_path = parts.next();
    if parts.next().is_some() || !is_fast_handler_path(category_path) {
        return None;
    }
    if let Some(slot_path) = slot_path
        && !is_fast_handler_path(slot_path)
    {
        return None;
    }

    let remainder = remainder.trim_start();
    let condition_prefix = "if ( mode == \"queued\" or mode == \"listed\" or mode == \"rolecheck\" or mode == \"suspended\" ) then";
    let remainder = remainder.strip_prefix(condition_prefix)?.trim_start();
    let (then_stmt, else_tail) = remainder.split_once("else")?;
    let else_stmt = else_tail.trim().strip_suffix("end")?.trim();
    let (leave_function, leave_args) =
        parse_global_function_call(then_stmt.trim().trim_end_matches(';'))?;
    let (join_function, join_args) =
        parse_global_function_call(else_stmt.trim().trim_end_matches(';'))?;
    if !join_args.trim().is_empty() {
        return None;
    }
    let expected_leave_args = match slot_path {
        Some(slot) => format!("{category_path}, {slot}"),
        None => category_path.to_string(),
    };
    (leave_args.trim() == expected_leave_args
        && is_fast_handler_path(leave_function)
        && is_fast_handler_path(join_function))
    .then_some((category_path, slot_path, leave_function, join_function))
}

fn parse_global_function_call(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?;
    let function_name = function_name.trim();
    is_fast_handler_path(function_name).then_some((function_name, args))
}

fn parse_conditional_global_noarg_then_else<'a>(
    stmt: &'a str,
) -> Option<(&'a str, FastHandlerRef<'a>, FastHandlerRef<'a>)> {
    let remainder = stmt.trim().strip_prefix("if")?.trim_start();
    let (condition, remainder) = if let Some(remainder) = remainder.strip_prefix('(') {
        let (condition, remainder) = remainder.split_once("then")?;
        (condition.trim_end().strip_suffix(')')?.trim(), remainder)
    } else {
        let (condition, remainder) = remainder.split_once("then")?;
        (condition.trim(), remainder)
    };
    let function_name = condition.strip_suffix("()")?.trim();
    if !is_fast_handler_path(function_name) {
        return None;
    }

    let (then_stmt, else_tail) = remainder.split_once("else")?;
    let else_stmt = else_tail.trim().strip_suffix("end")?.trim();
    let then_stmt = then_stmt
        .trim()
        .strip_suffix(';')
        .map(str::trim)
        .unwrap_or(then_stmt.trim());
    let else_stmt = else_stmt
        .strip_suffix(';')
        .map(str::trim)
        .unwrap_or(else_stmt);

    let then_ref = super::parse_inline_fast_handler("OnClick", then_stmt)?;
    let else_ref = super::parse_inline_fast_handler("OnClick", else_stmt)?;
    Some((function_name, then_ref, else_ref))
}

fn parse_conditional_global_function_with_noarg_function_result_then<'a>(
    stmt: &'a str,
) -> Option<(&'a str, &'a str, FastHandlerRef<'a>)> {
    let remainder = stmt.trim().strip_prefix("if")?.trim_start();
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let (condition, remainder) = remainder.split_once("then")?;
    let condition = condition.trim_end().strip_suffix(')')?.trim();
    let (function_name, args) = parse_global_function_call(condition)?;
    let args = args.trim();
    let arg_function_name = args.strip_suffix("()")?.trim();
    if !(is_fast_handler_path(function_name) && is_fast_handler_path(arg_function_name)) {
        return None;
    }

    let then_stmt = remainder.trim().strip_suffix("end")?.trim();
    let then_stmt = then_stmt
        .strip_suffix(';')
        .map(str::trim)
        .unwrap_or(then_stmt);
    let then_ref = super::parse_inline_fast_handler("OnClick", then_stmt)?;
    Some((function_name, arg_function_name, then_ref))
}

fn parse_conditional_global_field_equals_string_then<'a>(
    stmt: &'a str,
) -> Option<(&'a str, &'a str, &'a str, FastHandlerRef<'a>)> {
    let remainder = stmt.trim().strip_prefix("if")?.trim_start();
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let (condition, remainder) = remainder.split_once("then")?;
    let condition = condition.trim_end().strip_suffix(')')?.trim();
    let (lhs, rhs) = condition.split_once("==")?;
    let (target_path, field) = lhs.trim().rsplit_once('.')?;
    let target_path = target_path.trim();
    let field = field.trim();
    let value = super::parse_single_string_literal(rhs.trim())?;
    if !is_fast_handler_path(target_path) || !is_fast_identifier(field) {
        return None;
    }
    let then_stmt = remainder.trim().strip_suffix("end")?.trim();
    let then_stmt = then_stmt
        .strip_suffix(';')
        .map(str::trim)
        .unwrap_or(then_stmt);
    let then_ref = super::parse_inline_fast_handler("OnClick", then_stmt)?;
    Some((target_path, field, value, then_ref))
}

fn parse_inline_global_method(stmt: &str) -> Option<(&str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path)
        && is_fast_identifier(method_name)
        && super::is_fast_passthrough_args(args))
    .then_some((target_path, method_name))
}

fn parse_inline_global_method_with_self_arg(stmt: &str) -> Option<(&str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path) && is_fast_identifier(method_name) && args == "self")
        .then_some((target_path, method_name))
}

fn parse_inline_global_method_with_self_string_arg(stmt: &str) -> Option<(&str, &str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (self_arg, raw_string_arg) = args.split_once(',')?;
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    let arg = super::parse_single_string_literal(raw_string_arg.trim())?;
    (is_fast_handler_path(target_path)
        && is_fast_identifier(method_name)
        && self_arg.trim() == "self")
        .then_some((target_path, method_name, arg))
}

fn parse_inline_global_method_with_self_string_number_number_args(
    stmt: &str,
) -> Option<(&str, &str, &str, f64, f64)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let mut parts = args.split(',').map(str::trim);
    let self_arg = parts.next()?;
    let first = super::parse_single_string_literal(parts.next()?)?;
    let second = parts.next()?.parse::<f64>().ok()?;
    let third = parts.next()?.parse::<f64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path) && is_fast_identifier(method_name) && self_arg == "self")
        .then_some((target_path, method_name, first, second, third))
}

fn parse_inline_global_method_with_string_arg(stmt: &str) -> Option<(&str, &str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let arg = super::parse_single_string_literal(args.strip_suffix(')')?.trim())?;
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path) && is_fast_identifier(method_name)).then_some((
        target_path,
        method_name,
        arg,
    ))
}

fn parse_inline_global_method_with_global_arg(stmt: &str) -> Option<(&str, &str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let arg_path = args.strip_suffix(')')?.trim();
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path)
        && is_fast_identifier(method_name)
        && is_fast_handler_path(arg_path)
        && arg_path.split('.').next() != Some("self"))
    .then_some((target_path, method_name, arg_path))
}

fn parse_inline_global_method_with_four_global_args(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, &str, &str)> {
    let (target_path, method_name, arg_paths) =
        parse_global_method_with_four_global_arg_paths(stmt)?;
    let [
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
    ] = arg_paths;
    Some((
        target_path,
        method_name,
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
    ))
}

fn parse_inline_global_method_with_string_global_bool_args(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, bool)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let mut parts = args.split(',').map(str::trim);
    let first = super::parse_single_string_literal(parts.next()?)?;
    let second_arg_path = parts.next()?;
    let third = super::parse_single_bool_literal(parts.next()?)?;
    if parts.next().is_some() {
        return None;
    }
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    // Reject `self.*` for target — `self` is the per-frame OnLoad arg, not a
    // global resolvable at parse time. Reject Lua keyword literals
    // (`true`/`false`/`nil`) for second_arg_path — `is_fast_handler_path`
    // accepts them as identifiers but `_G.true` etc. resolve to nil and
    // silently substitute nil for the intended literal value.
    (is_fast_handler_path(target_path)
        && target_path.split('.').next() != Some("self")
        && is_fast_identifier(method_name)
        && is_fast_handler_path(second_arg_path)
        && second_arg_path.split('.').next() != Some("self")
        && !matches!(second_arg_path, "true" | "false" | "nil"))
    .then_some((target_path, method_name, first, second_arg_path, third))
}

fn parse_inline_global_method_with_global_three_global_bool_args(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, &str, &str, bool)> {
    let (target_path, method_name, arg_paths, fifth) =
        parse_global_method_with_four_global_arg_paths_and_bool(stmt)?;
    let [
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
    ] = arg_paths;
    Some((
        target_path,
        method_name,
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
        fifth,
    ))
}

fn parse_global_method_with_four_global_arg_paths_and_bool<'a>(
    stmt: &'a str,
) -> Option<(&'a str, &'a str, [&'a str; 4], bool)> {
    let (target_path, method_name, arg_paths, fifth_arg) =
        parse_global_method_with_four_global_arg_paths_and_tail(stmt)?;
    let fifth = super::parse_single_bool_literal(fifth_arg?)?;
    validate_global_method_with_global_arg_paths(target_path, method_name, arg_paths)
        .map(|(target_path, method_name, arg_paths)| (target_path, method_name, arg_paths, fifth))
}

fn parse_global_method_with_four_global_arg_paths<'a>(
    stmt: &'a str,
) -> Option<(&'a str, &'a str, [&'a str; 4])> {
    let (target_path, method_name, arg_paths, tail_arg) =
        parse_global_method_with_four_global_arg_paths_and_tail(stmt)?;
    if tail_arg.is_some() {
        return None;
    }
    validate_global_method_with_global_arg_paths(target_path, method_name, arg_paths)
}

fn parse_global_method_with_four_global_arg_paths_and_tail<'a>(
    stmt: &'a str,
) -> Option<(&'a str, &'a str, [&'a str; 4], Option<&'a str>)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let mut parts = args.split(',').map(str::trim);
    let first_arg_path = parts.next()?;
    let second_arg_path = parts.next()?;
    let third_arg_path = parts.next()?;
    let fourth_arg_path = parts.next()?;
    let tail_arg = parts.next();
    if parts.next().is_some() {
        return None;
    }
    let arg_paths = [
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
    ];
    Some((target_path, method_name, arg_paths, tail_arg))
}

fn validate_global_method_with_global_arg_paths<'a>(
    target_path: &'a str,
    method_name: &'a str,
    arg_paths: [&'a str; 4],
) -> Option<(&'a str, &'a str, [&'a str; 4])> {
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    let all_args_are_global = arg_paths.iter().all(|arg_path| {
        is_fast_handler_path(arg_path) && arg_path.split('.').next() != Some("self")
    });
    (is_fast_handler_path(target_path) && is_fast_identifier(method_name) && all_args_are_global)
        .then_some((target_path, method_name, arg_paths))
}

fn parse_inline_global_method_with_global_nil_nil_nil_nil_bool_args(
    stmt: &str,
) -> Option<(&str, &str, &str, bool)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let mut parts = args.split(',').map(str::trim);
    let first_arg_path = parts.next()?;
    if parts.next()? != "nil" {
        return None;
    }
    if parts.next()? != "nil" {
        return None;
    }
    if parts.next()? != "nil" {
        return None;
    }
    if parts.next()? != "nil" {
        return None;
    }
    let sixth = super::parse_single_bool_literal(parts.next()?)?;
    if parts.next().is_some() {
        return None;
    }
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path)
        && is_fast_identifier(method_name)
        && is_fast_handler_path(first_arg_path)
        && first_arg_path.split('.').next() != Some("self"))
    .then_some((target_path, method_name, first_arg_path, sixth))
}

fn parse_inline_global_method_with_global_self_method_self_method_bool_args(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, &str, bool)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
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
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path)
        && is_fast_identifier(method_name)
        && is_fast_handler_path(first_arg_path)
        && is_fast_identifier(second_self_method)
        && is_fast_identifier(third_self_method))
    .then_some((
        target_path,
        method_name,
        first_arg_path,
        second_self_method,
        third_self_method,
        fourth,
    ))
}

type GlobalMethodStringStringFunctionResultNumbers<'a> =
    (&'a str, &'a str, &'a str, &'a str, &'a str, f64, f64, f64);

fn parse_inline_global_method_with_string_string_function_result_and_three_number_args(
    stmt: &str,
) -> Option<GlobalMethodStringStringFunctionResultNumbers<'_>> {
    let parsed = parse_global_method_with_function_result_and_three_numbers(stmt)?;
    let (function_name, first, second) =
        parse_string_string_function_result_call(parsed.first_arg)?;
    validate_global_method_with_function_result(
        parsed.target_path,
        parsed.method_name,
        function_name,
    )?;
    Some((
        parsed.target_path.trim(),
        parsed.method_name.trim(),
        function_name.trim(),
        first,
        second,
        parsed.third,
        parsed.fourth,
        parsed.fifth,
    ))
}

struct GlobalMethodFunctionResultNumbers<'a> {
    target_path: &'a str,
    method_name: &'a str,
    first_arg: &'a str,
    third: f64,
    fourth: f64,
    fifth: f64,
}

fn parse_global_method_with_function_result_and_three_numbers(
    stmt: &str,
) -> Option<GlobalMethodFunctionResultNumbers<'_>> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let parts = super::split_top_level_args(args)?;
    if parts.len() != 4 {
        return None;
    }
    let third = parts[1].parse::<f64>().ok()?;
    let fourth = parts[2].parse::<f64>().ok()?;
    let fifth = parts[3].parse::<f64>().ok()?;
    Some(GlobalMethodFunctionResultNumbers {
        target_path,
        method_name,
        first_arg: parts[0],
        third,
        fourth,
        fifth,
    })
}

fn parse_string_string_function_result_call(stmt: &str) -> Option<(&str, &str, &str)> {
    let (function_name, raw_first, raw_second) = parse_two_arg_function_call(stmt)?;
    let first = super::parse_single_string_literal(raw_first.trim())?;
    let second = super::parse_single_string_literal(raw_second.trim())?;
    Some((function_name, first, second))
}

fn parse_global_string_function_result_call(stmt: &str) -> Option<(&str, &str, &str)> {
    let (function_name, raw_first, raw_second) = parse_two_arg_function_call(stmt)?;
    let first_arg_path = raw_first.trim();
    let second = super::parse_single_string_literal(raw_second.trim())?;
    (is_fast_handler_path(first_arg_path) && first_arg_path.split('.').next() != Some("self"))
        .then_some((function_name, first_arg_path, second))
}

fn parse_two_arg_function_call(stmt: &str) -> Option<(&str, &str, &str)> {
    let (function_name, call_args) = stmt.split_once('(')?;
    let call_args = call_args.strip_suffix(')')?.trim();
    let call_args = super::split_top_level_args(call_args)?;
    if call_args.len() != 2 {
        return None;
    }
    Some((function_name, call_args[0], call_args[1]))
}

fn validate_global_method_with_function_result(
    target_path: &str,
    method_name: &str,
    function_name: &str,
) -> Option<()> {
    (is_fast_handler_path(target_path.trim())
        && is_fast_identifier(method_name.trim())
        && is_fast_handler_path(function_name.trim()))
    .then_some(())
}

type GlobalMethodGlobalStringFunctionResultNumbers<'a> =
    (&'a str, &'a str, &'a str, &'a str, &'a str, f64, f64, f64);

fn parse_inline_global_method_with_global_string_function_result_and_three_number_args(
    stmt: &str,
) -> Option<GlobalMethodGlobalStringFunctionResultNumbers<'_>> {
    let parsed = parse_global_method_with_function_result_and_three_numbers(stmt)?;
    let (function_name, first_arg_path, second) =
        parse_global_string_function_result_call(parsed.first_arg)?;
    validate_global_method_with_function_result(
        parsed.target_path,
        parsed.method_name,
        function_name,
    )?;
    Some((
        parsed.target_path.trim(),
        parsed.method_name.trim(),
        function_name.trim(),
        first_arg_path,
        second,
        parsed.third,
        parsed.fourth,
        parsed.fifth,
    ))
}

fn parse_inline_global_method_with_self_id_arg(stmt: &str) -> Option<(&str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path) && is_fast_identifier(method_name) && args == "self:GetID()")
        .then_some((target_path, method_name))
}

fn parse_inline_global_method_with_self_field_arg(stmt: &str) -> Option<(&str, &str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let field = args.strip_suffix(')')?.trim().strip_prefix("self.")?.trim();
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path)
        && is_fast_identifier(method_name)
        && is_fast_identifier(field))
    .then_some((target_path, method_name, field))
}

pub(super) fn parse_global_tooltip_set_owner_then_set_text(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, &str, &str, bool)> {
    let (first, second) = stmt.split_once(';')?;
    let owner = parse_tooltip_set_owner(first.trim())?;
    let text = parse_tooltip_set_text_paths(second.trim(), owner.target_path)?;
    Some((
        owner.target_path,
        owner.anchor,
        text.text_path,
        text.red_path,
        text.green_path,
        text.blue_path,
        text.wrap,
    ))
}

struct TooltipOwner<'a> {
    target_path: &'a str,
    anchor: &'a str,
}

struct TooltipTextPaths<'a> {
    text_path: &'a str,
    red_path: &'a str,
    green_path: &'a str,
    blue_path: &'a str,
    wrap: bool,
}

fn parse_tooltip_set_owner(stmt: &str) -> Option<TooltipOwner<'_>> {
    let (target_path, method_name, anchor) = parse_inline_global_method_with_self_string_arg(stmt)?;
    (method_name == "SetOwner").then_some(TooltipOwner {
        target_path,
        anchor,
    })
}

fn parse_tooltip_set_text_paths<'a>(
    stmt: &'a str,
    expected_target_path: &str,
) -> Option<TooltipTextPaths<'a>> {
    let text_args = parse_matching_global_set_text_args(stmt, expected_target_path)?;
    let [text_path, red_path, green_path, blue_path, maybe_nil, wrap] =
        super::split_top_level_args(text_args)?.try_into().ok()?;
    let wrap = parse_tooltip_wrap_arg(wrap, maybe_nil)?;
    (is_fast_handler_path(text_path)
        && is_fast_handler_path(red_path)
        && is_fast_handler_path(green_path)
        && is_fast_handler_path(blue_path))
    .then_some(TooltipTextPaths {
        text_path,
        red_path,
        green_path,
        blue_path,
        wrap,
    })
}

fn parse_matching_global_set_text_args<'a>(
    stmt: &'a str,
    expected_target_path: &str,
) -> Option<&'a str> {
    let (target_path, text_remainder) = stmt.rsplit_once(':')?;
    let (method_name, text_args) = text_remainder.split_once('(')?;
    let same_target = target_path.trim() == expected_target_path;
    let is_set_text = method_name.trim() == "SetText";
    if !(same_target && is_set_text) {
        return None;
    }
    Some(text_args.strip_suffix(')')?.trim())
}

fn parse_tooltip_wrap_arg(wrap: &str, maybe_nil: &str) -> Option<bool> {
    (maybe_nil == "nil")
        .then_some(wrap)
        .and_then(|wrap| match wrap {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        })
}

fn parse_global_tooltip_set_owner_then_set_text_literal(
    stmt: &str,
) -> Option<(&str, &str, &str, f64, f64, f64)> {
    let (first, second) = stmt.split_once(';')?;
    let (target_path, method_name, anchor) =
        parse_inline_global_method_with_self_string_arg(first.trim())?;
    if method_name != "SetOwner" {
        return None;
    }

    let (text_target_path, text_remainder) = second.trim().rsplit_once(':')?;
    let (text_method_name, text_args) = text_remainder.split_once('(')?;
    let text_args = text_args.strip_suffix(')')?.trim();
    if text_target_path.trim() != target_path || text_method_name.trim() != "SetText" {
        return None;
    }

    let mut parts = text_args.split(',').map(str::trim);
    let text = super::parse_single_string_literal(parts.next()?)?;
    let red = parts.next()?.parse::<f64>().ok()?;
    let green = parts.next()?.parse::<f64>().ok()?;
    let blue = parts.next()?.parse::<f64>().ok()?;
    if parts.next().is_some() {
        return None;
    }

    Some((target_path, anchor, text, red, green, blue))
}

fn parse_conditional_tooltip(stmt: &str) -> Option<(&str, &str, &str, &str, &str, &str)> {
    let body = parse_conditional_tooltip_body(stmt)?;
    let owner = parse_tooltip_set_owner(body.owner_stmt)?;
    let text = parse_conditional_tooltip_text(body.text_stmt, owner.target_path, body.field)?;
    Some((
        owner.target_path,
        body.field,
        owner.anchor,
        text.red_path,
        text.green_path,
        text.blue_path,
    ))
}

struct ConditionalTooltipBody<'a> {
    field: &'a str,
    owner_stmt: &'a str,
    text_stmt: &'a str,
}

struct ConditionalTooltipText<'a> {
    red_path: &'a str,
    green_path: &'a str,
    blue_path: &'a str,
}

fn parse_conditional_tooltip_body(stmt: &str) -> Option<ConditionalTooltipBody<'_>> {
    let remainder = stmt.trim().strip_prefix("if")?.trim_start();
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let remainder = remainder.strip_prefix("self.")?;
    let (field, remainder) = remainder.split_once(')')?;
    let field = field.trim();
    if !is_fast_identifier(field) {
        return None;
    }

    let remainder = remainder.trim_start().strip_prefix("then")?.trim_start();
    let (owner_stmt, text_with_end) = remainder.split_once(";")?;
    let text_stmt = text_with_end.trim().strip_suffix("end")?.trim();
    Some(ConditionalTooltipBody {
        field,
        owner_stmt: owner_stmt.trim(),
        text_stmt,
    })
}

fn parse_conditional_tooltip_text<'a>(
    stmt: &'a str,
    expected_target_path: &str,
    expected_field: &str,
) -> Option<ConditionalTooltipText<'a>> {
    let text_args = parse_matching_global_set_text_args(stmt, expected_target_path)?;
    let [text_path, red_path, green_path, blue_path] =
        super::split_top_level_args(text_args)?.try_into().ok()?;
    let text_field = text_path.strip_prefix("self.")?;
    if text_field != expected_field {
        return None;
    }

    let color_paths = [red_path, green_path, blue_path];
    color_paths
        .iter()
        .all(|path| is_fast_handler_path(path))
        .then_some(ConditionalTooltipText {
            red_path,
            green_path,
            blue_path,
        })
}

#[cfg(test)]
mod conditional_tooltip_tests {
    use super::parse_conditional_tooltip;

    #[test]
    fn parses_conditional_tooltip_color_paths() {
        let stmt = concat!(
            "if (self.tooltip) then ",
            "GameTooltip:SetOwner(self, \"ANCHOR_RIGHT\"); ",
            "GameTooltip:SetText(self.tooltip, TEST_FAST_TOOLTIP_COLOR.r, ",
            "TEST_FAST_TOOLTIP_COLOR.g, TEST_FAST_TOOLTIP_COLOR.b) end"
        );

        let parsed = parse_conditional_tooltip(stmt);

        assert_eq!(
            parsed,
            Some((
                "GameTooltip",
                "tooltip",
                "ANCHOR_RIGHT",
                "TEST_FAST_TOOLTIP_COLOR.r",
                "TEST_FAST_TOOLTIP_COLOR.g",
                "TEST_FAST_TOOLTIP_COLOR.b",
            ))
        );
    }

    #[test]
    fn rejects_conditional_tooltip_text_from_different_field() {
        let stmt = concat!(
            "if (self.tooltip) then ",
            "GameTooltip:SetOwner(self, \"ANCHOR_RIGHT\"); ",
            "GameTooltip:SetText(self.otherTooltip, TEST_FAST_TOOLTIP_COLOR.r, ",
            "TEST_FAST_TOOLTIP_COLOR.g, TEST_FAST_TOOLTIP_COLOR.b) end"
        );

        let parsed = parse_conditional_tooltip(stmt);

        assert_eq!(parsed, None);
    }
}

fn parse_toggle_global_visibility(stmt: &str) -> Option<&str> {
    let remainder = stmt.trim().strip_prefix("if")?.trim_start();
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let (condition, remainder) = remainder.split_once("then")?;
    let condition = condition.trim_end().strip_suffix(')')?.trim();
    let (target_path, method_name) = condition.trim().rsplit_once(':')?;
    if method_name.trim() != "IsShown()" {
        return None;
    }
    let (then_stmt, else_tail) = remainder.split_once("else")?;
    let else_stmt = else_tail.trim().strip_suffix("end")?.trim();
    let target_path = target_path.trim();
    let then_stmt = then_stmt
        .trim()
        .strip_suffix(';')
        .map(str::trim)
        .unwrap_or(then_stmt.trim());
    let else_stmt = else_stmt
        .trim()
        .strip_suffix(';')
        .map(str::trim)
        .unwrap_or(else_stmt);
    let hide_stmt = format!("{target_path}:Hide()");
    let show_stmt = format!("{target_path}:Show()");
    (is_fast_handler_path(target_path) && then_stmt == hide_stmt && else_stmt == show_stmt)
        .then_some(target_path)
}

fn parse_inline_named_global_method_with_global_arg(stmt: &str) -> Option<(&str, &str, &str)> {
    let remainder = stmt.strip_prefix("_G[self:GetName()..")?;
    let (raw_suffix, remainder) = remainder.split_once("]:")?;
    let suffix = super::parse_single_string_literal(raw_suffix.trim())?;
    let (method_name, args) = remainder.split_once('(')?;
    let arg_path = args.strip_suffix(')')?.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(method_name) && is_fast_handler_path(arg_path)).then_some((
        suffix,
        method_name,
        arg_path,
    ))
}

fn parse_inline_global_method_then_assign(
    stmt: &str,
) -> Option<(&str, &str, &str, FastLiteralValue<'_>)> {
    let (first, second) = stmt.split_once(';')?;
    let (target_path, method_name) = parse_inline_global_method(first.trim())?;
    let FastHandlerRef::AssignLiteral { field, value } =
        super::parse_inline_assignment(second.trim())?
    else {
        return None;
    };
    Some((target_path, method_name, field, value))
}

#[cfg(test)]
mod tests {
    use super::{
        parse_global_tooltip_set_owner_then_set_text,
        parse_inline_global_method_with_four_global_args,
        parse_inline_global_method_with_global_string_function_result_and_three_number_args,
        parse_inline_global_method_with_global_three_global_bool_args,
        parse_inline_global_method_with_string_string_function_result_and_three_number_args,
    };

    #[test]
    fn parses_global_method_with_four_global_args() {
        let stmt = "GameTooltip:SetText(Tooltip.Title, Tooltip.Red, Tooltip.Green, Tooltip.Blue)";

        let parsed = parse_inline_global_method_with_four_global_args(stmt);

        assert_eq!(
            parsed,
            Some((
                "GameTooltip",
                "SetText",
                "Tooltip.Title",
                "Tooltip.Red",
                "Tooltip.Green",
                "Tooltip.Blue",
            ))
        );
    }

    #[test]
    fn parses_global_method_with_global_three_global_bool_args() {
        let stmt =
            "GameTooltip:AddLine(Tooltip.Text, Tooltip.Red, Tooltip.Green, Tooltip.Blue, true)";

        let parsed = parse_inline_global_method_with_global_three_global_bool_args(stmt);

        assert_eq!(
            parsed,
            Some((
                "GameTooltip",
                "AddLine",
                "Tooltip.Text",
                "Tooltip.Red",
                "Tooltip.Green",
                "Tooltip.Blue",
                true,
            ))
        );
    }

    #[test]
    fn parses_global_method_with_string_string_function_result_and_three_number_args() {
        let stmt = "GameTooltip:SetText(GetColoredText(\"name\", \"rank\"), 1, 0.5, 0)";

        let parsed =
            parse_inline_global_method_with_string_string_function_result_and_three_number_args(
                stmt,
            );

        assert_eq!(
            parsed,
            Some((
                "GameTooltip",
                "SetText",
                "GetColoredText",
                "name",
                "rank",
                1.0,
                0.5,
                0.0,
            ))
        );
    }

    #[test]
    fn parses_global_method_with_global_string_function_result_and_three_number_args() {
        let stmt = "GameTooltip:SetText(GetColoredText(Item.Name, \"rank\"), 1, 0.5, 0)";

        let parsed =
            parse_inline_global_method_with_global_string_function_result_and_three_number_args(
                stmt,
            );

        assert_eq!(
            parsed,
            Some((
                "GameTooltip",
                "SetText",
                "GetColoredText",
                "Item.Name",
                "rank",
                1.0,
                0.5,
                0.0,
            ))
        );
    }

    #[test]
    fn parses_global_tooltip_set_owner_then_set_text_paths() {
        let stmt = concat!(
            "GameTooltip:SetOwner(self, \"ANCHOR_RIGHT\"); ",
            "GameTooltip:SetText(self.Title, self.Red, self.Green, self.Blue, nil, true)"
        );

        let parsed = parse_global_tooltip_set_owner_then_set_text(stmt);

        assert_eq!(
            parsed,
            Some((
                "GameTooltip",
                "ANCHOR_RIGHT",
                "self.Title",
                "self.Red",
                "self.Green",
                "self.Blue",
                true,
            ))
        );
    }

    #[test]
    fn rejects_global_tooltip_set_text_on_different_target() {
        let stmt = concat!(
            "GameTooltip:SetOwner(self, \"ANCHOR_RIGHT\"); ",
            "OtherTooltip:SetText(self.Title, self.Red, self.Green, self.Blue, nil, true)"
        );

        let parsed = parse_global_tooltip_set_owner_then_set_text(stmt);

        assert_eq!(parsed, None);
    }
}
