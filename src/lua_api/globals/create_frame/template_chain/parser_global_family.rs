use super::{FastHandlerRef, FastLiteralValue, is_fast_handler_path, is_fast_identifier};

pub(super) fn parse_global_family<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    if let Some((category_path, slot_path, leave_function, join_function)) =
        parse_get_lfg_mode_branch(stmt)
    {
        return Some(FastHandlerRef::GetLfgModeBranch {
            category_path,
            slot_path,
            leave_function,
            join_function,
        });
    }
    if let Some((target_path, method_name)) = parse_local_global_path_conditional_method(stmt) {
        return Some(FastHandlerRef::LocalGlobalPathConditionalMethod {
            target_path,
            method_name,
        });
    }
    if let Some((function_name, then_ref, else_ref)) =
        parse_conditional_global_noarg_then_else(stmt)
    {
        return Some(FastHandlerRef::ConditionalGlobalNoArgs {
            function_name,
            then_ref: Box::new(then_ref),
            else_ref: Box::new(else_ref),
        });
    }
    if let Some((target_path, method_name, field, value)) =
        parse_inline_global_method_then_assign(stmt)
    {
        return Some(FastHandlerRef::GlobalMethodThenAssignLiteral {
            target_path,
            method_name,
            field,
            value,
        });
    }
    if let Some((target_path, method_name, arg)) =
        parse_inline_global_method_with_self_string_arg(stmt)
    {
        return Some(FastHandlerRef::GlobalMethodWithSelfStringArg {
            target_path,
            method_name,
            arg,
        });
    }
    if let Some((target_path, method_name, arg)) = parse_inline_global_method_with_string_arg(stmt)
    {
        return Some(FastHandlerRef::GlobalMethodWithStringArg {
            target_path,
            method_name,
            arg,
        });
    }
    if let Some((target_path, method_name, arg_path)) =
        parse_inline_global_method_with_global_arg(stmt)
    {
        return Some(FastHandlerRef::GlobalMethodWithGlobalArg {
            target_path,
            method_name,
            arg_path,
        });
    }
    if let Some((
        target_path,
        method_name,
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
    )) = parse_inline_global_method_with_four_global_args(stmt)
    {
        return Some(FastHandlerRef::GlobalMethodWithFourGlobalArgs {
            target_path,
            method_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
            fourth_arg_path,
        });
    }
    if let Some((target_path, method_name)) = parse_inline_global_method_with_self_id_arg(stmt) {
        return Some(FastHandlerRef::GlobalMethodWithSelfIdArg {
            target_path,
            method_name,
        });
    }
    if let Some((target_path, method_name)) = parse_inline_global_method(stmt) {
        return Some(FastHandlerRef::GlobalMethod {
            target_path,
            method_name,
        });
    }
    if let Some((target_path, anchor, text, red, green, blue)) =
        parse_global_tooltip_set_owner_then_set_text_literal(stmt)
    {
        return Some(FastHandlerRef::GlobalTooltipSetOwnerThenSetTextLiteral {
            target_path,
            anchor,
            text,
            red,
            green,
            blue,
        });
    }
    if let Some((target_path, anchor, text_path, red_path, green_path, blue_path, wrap)) =
        parse_global_tooltip_set_owner_then_set_text(stmt)
    {
        return Some(FastHandlerRef::GlobalTooltipSetOwnerThenSetText {
            target_path,
            anchor,
            text_path,
            red_path,
            green_path,
            blue_path,
            wrap,
        });
    }
    if let Some((target_path, field, anchor, red_path, green_path, blue_path)) =
        parse_conditional_tooltip(stmt)
    {
        return Some(FastHandlerRef::ConditionalTooltip {
            target_path,
            field,
            anchor,
            red_path,
            green_path,
            blue_path,
        });
    }
    if let Some(target_path) = parse_toggle_global_visibility(stmt) {
        return Some(FastHandlerRef::ToggleGlobalVisibility { target_path });
    }
    if let Some((suffix, method_name, arg_path)) =
        parse_inline_named_global_method_with_global_arg(stmt)
    {
        return Some(FastHandlerRef::NamedGlobalMethodWithGlobalArg {
            suffix,
            method_name,
            arg_path,
        });
    }
    parse_inline_global_method_with_self_field_arg(stmt).map(|(target_path, method_name, field)| {
        FastHandlerRef::GlobalMethodWithSelfFieldArg {
            target_path,
            method_name,
            field,
        }
    })
}

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
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let (condition, remainder) = remainder.split_once("then")?;
    let condition = condition.trim_end().strip_suffix(')')?.trim();
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
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let mut parts = args.split(',').map(str::trim);
    let first_arg_path = parts.next()?;
    let second_arg_path = parts.next()?;
    let third_arg_path = parts.next()?;
    let fourth_arg_path = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path)
        && is_fast_identifier(method_name)
        && is_fast_handler_path(first_arg_path)
        && is_fast_handler_path(second_arg_path)
        && is_fast_handler_path(third_arg_path)
        && is_fast_handler_path(fourth_arg_path)
        && first_arg_path.split('.').next() != Some("self")
        && second_arg_path.split('.').next() != Some("self")
        && third_arg_path.split('.').next() != Some("self")
        && fourth_arg_path.split('.').next() != Some("self"))
    .then_some((
        target_path,
        method_name,
        first_arg_path,
        second_arg_path,
        third_arg_path,
        fourth_arg_path,
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
    let text_path = parts.next()?;
    let red_path = parts.next()?;
    let green_path = parts.next()?;
    let blue_path = parts.next()?;
    let maybe_nil = parts.next()?;
    let wrap = parts.next()?;
    if parts.next().is_some() {
        return None;
    }

    (is_fast_handler_path(text_path)
        && is_fast_handler_path(red_path)
        && is_fast_handler_path(green_path)
        && is_fast_handler_path(blue_path)
        && maybe_nil == "nil"
        && matches!(wrap, "true" | "false"))
    .then_some((
        target_path,
        anchor,
        text_path,
        red_path,
        green_path,
        blue_path,
        wrap == "true",
    ))
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
    let remainder = stmt.trim().strip_prefix("if")?.trim_start();
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let remainder = remainder.strip_prefix("self.")?;
    let (field, remainder) = remainder.split_once(')')?;
    if !is_fast_identifier(field.trim()) {
        return None;
    }
    let remainder = remainder.trim_start().strip_prefix("then")?.trim_start();
    let (first, second_with_end) = remainder.split_once(";")?;
    let second = second_with_end.trim().strip_suffix("end")?.trim();

    let (target_path, method_name, anchor) =
        parse_inline_global_method_with_self_string_arg(first.trim())?;
    if method_name != "SetOwner" {
        return None;
    }

    let (text_target_path, text_remainder) = second.rsplit_once(':')?;
    let (text_method_name, text_args) = text_remainder.split_once('(')?;
    let text_args = text_args.strip_suffix(')')?.trim();
    if text_target_path.trim() != target_path || text_method_name.trim() != "SetText" {
        return None;
    }
    let mut parts = text_args.split(',').map(str::trim);
    let text_field = parts.next()?.strip_prefix("self.")?;
    let red_path = parts.next()?;
    let green_path = parts.next()?;
    let blue_path = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    (text_field == field.trim()
        && is_fast_handler_path(red_path)
        && is_fast_handler_path(green_path)
        && is_fast_handler_path(blue_path))
    .then_some((
        target_path,
        field.trim(),
        anchor,
        red_path,
        green_path,
        blue_path,
    ))
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
