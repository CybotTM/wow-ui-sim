use super::{FastHandlerRef, is_fast_handler_path, is_fast_identifier, is_fast_passthrough_args};

pub(super) fn parse_method_family<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    if let Some((method_name, value)) = parse_inline_self_method_with_bool_arg(stmt) {
        return Some(FastHandlerRef::MethodWithBoolArg { method_name, value });
    }
    if let Some((method_name, arg)) = parse_inline_self_method_with_string_arg(stmt) {
        return Some(FastHandlerRef::MethodWithStringArg { method_name, arg });
    }
    if let Some(method_name) = parse_inline_self_method(stmt) {
        return Some(FastHandlerRef::Method(method_name));
    }
    if let Some((field, method_name, arg)) = parse_inline_self_field_method_with_string_arg(stmt) {
        return Some(FastHandlerRef::SelfFieldMethodWithStringArg {
            field,
            method_name,
            arg,
        });
    }
    if let Some((field, method_name, value)) = parse_inline_self_field_method_with_number_arg(stmt)
    {
        return Some(FastHandlerRef::SelfFieldMethodWithNumberArg {
            field,
            method_name,
            value,
        });
    }
    if let Some((field, method_name, first, second, third)) =
        parse_inline_self_field_method_with_string_number_number_args(stmt)
    {
        return Some(FastHandlerRef::SelfFieldMethodWithStringNumberNumberArgs {
            field,
            method_name,
            first,
            second,
            third,
        });
    }
    if let Some((field, method_name, arg_field)) =
        parse_inline_self_field_method_with_self_field_arg(stmt)
    {
        return Some(FastHandlerRef::SelfFieldMethodWithSelfFieldArg {
            field,
            method_name,
            arg_field,
        });
    }
    if let Some((field, method_name, arg_path)) =
        parse_inline_self_field_method_with_global_arg(stmt)
    {
        return Some(FastHandlerRef::SelfFieldMethodWithGlobalArg {
            field,
            method_name,
            arg_path,
        });
    }
    if let Some((field, method_name)) = parse_inline_self_field_method(stmt) {
        return Some(FastHandlerRef::SelfFieldMethod { field, method_name });
    }
    if let Some((method_name, arg)) = parse_inline_parent_method_with_string_arg(stmt) {
        return Some(FastHandlerRef::ParentMethodWithStringArg { method_name, arg });
    }
    if let Some(method_name) = parse_inline_parent_method(stmt) {
        return Some(FastHandlerRef::ParentMethod(method_name));
    }
    parse_inline_grandparent_method(stmt).map(FastHandlerRef::GrandparentMethod)
}

fn parse_inline_self_method(stmt: &str) -> Option<&str> {
    parse_inline_method_call(stmt, "self:")
}

fn parse_inline_self_method_with_bool_arg(stmt: &str) -> Option<(&str, bool)> {
    let remainder = stmt.strip_prefix("self:")?;
    let (method_name, args) = remainder.split_once('(')?;
    let value = super::parse_single_bool_literal(args.strip_suffix(')')?.trim())?;
    let method_name = method_name.trim();
    is_fast_identifier(method_name).then_some((method_name, value))
}

fn parse_inline_self_method_with_string_arg(stmt: &str) -> Option<(&str, &str)> {
    let remainder = stmt.strip_prefix("self:")?;
    let (method_name, args) = remainder.split_once('(')?;
    let arg = super::parse_single_string_literal(args.strip_suffix(')')?.trim())?;
    let method_name = method_name.trim();
    is_fast_identifier(method_name).then_some((method_name, arg))
}

fn parse_inline_self_field_method(stmt: &str) -> Option<(&str, &str)> {
    let (field, remainder) = stmt.strip_prefix("self.")?.split_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let field = field.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(field) && is_fast_identifier(method_name) && is_fast_passthrough_args(args))
        .then_some((field, method_name))
}

fn parse_inline_self_field_method_with_string_arg(stmt: &str) -> Option<(&str, &str, &str)> {
    let (field, remainder) = stmt.strip_prefix("self.")?.split_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let arg = super::parse_single_string_literal(args.strip_suffix(')')?.trim())?;
    let field = field.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(field) && is_fast_identifier(method_name)).then_some((
        field,
        method_name,
        arg,
    ))
}

fn parse_inline_self_field_method_with_number_arg(stmt: &str) -> Option<(&str, &str, f64)> {
    let (field, remainder) = stmt.strip_prefix("self.")?.split_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let value = args.strip_suffix(')')?.trim().parse::<f64>().ok()?;
    let field = field.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(field) && is_fast_identifier(method_name)).then_some((
        field,
        method_name,
        value,
    ))
}

fn parse_inline_self_field_method_with_string_number_number_args(
    stmt: &str,
) -> Option<(&str, &str, &str, f64, f64)> {
    let (field, remainder) = stmt.strip_prefix("self.")?.split_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?;
    let mut parts = args.split(',').map(str::trim);
    let first = super::parse_single_string_literal(parts.next()?)?;
    let second = parts.next()?.parse::<f64>().ok()?;
    let third = parts.next()?.parse::<f64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    let field = field.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(field) && is_fast_identifier(method_name)).then_some((
        field,
        method_name,
        first,
        second,
        third,
    ))
}

fn parse_inline_self_field_method_with_self_field_arg(stmt: &str) -> Option<(&str, &str, &str)> {
    let (field, remainder) = stmt.strip_prefix("self.")?.split_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let arg_field = args.strip_suffix(')')?.trim().strip_prefix("self.")?.trim();
    let field = field.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(field) && is_fast_identifier(method_name) && is_fast_identifier(arg_field))
        .then_some((field, method_name, arg_field))
}

fn parse_inline_self_field_method_with_global_arg(stmt: &str) -> Option<(&str, &str, &str)> {
    let (field, remainder) = stmt.strip_prefix("self.")?.split_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let arg_path = args.strip_suffix(')')?.trim();
    let field = field.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(field) && is_fast_identifier(method_name) && is_fast_handler_path(arg_path))
        .then_some((field, method_name, arg_path))
}

fn parse_inline_parent_method(stmt: &str) -> Option<&str> {
    parse_inline_method_call(stmt, "self:GetParent():")
}

fn parse_inline_parent_method_with_string_arg(stmt: &str) -> Option<(&str, &str)> {
    let remainder = stmt.strip_prefix("self:GetParent():")?;
    let (method_name, args) = remainder.split_once('(')?;
    let arg = super::parse_single_string_literal(args.strip_suffix(')')?.trim())?;
    let method_name = method_name.trim();
    is_fast_identifier(method_name).then_some((method_name, arg))
}

fn parse_inline_grandparent_method(stmt: &str) -> Option<&str> {
    parse_inline_method_call(stmt, "self:GetParent():GetParent():")
}

fn parse_inline_method_call<'a>(stmt: &'a str, prefix: &str) -> Option<&'a str> {
    let remainder = stmt.strip_prefix(prefix)?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(method_name) && is_fast_passthrough_args(args)).then_some(method_name)
}
