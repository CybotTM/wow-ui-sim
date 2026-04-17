use super::{FastHandlerRef, FastLiteralValue, is_fast_handler_path, is_fast_identifier};

pub(super) fn parse_global_family<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
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
    if let Some((target_path, method_name)) = parse_inline_global_method(stmt) {
        return Some(FastHandlerRef::GlobalMethod {
            target_path,
            method_name,
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
    if let Some((target_path, method_name)) = parse_inline_global_method_with_self_id_arg(stmt) {
        return Some(FastHandlerRef::GlobalMethodWithSelfIdArg {
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

fn parse_global_tooltip_set_owner_then_set_text(
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
