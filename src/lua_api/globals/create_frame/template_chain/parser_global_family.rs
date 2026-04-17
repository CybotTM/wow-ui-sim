use super::{FastHandlerRef, FastLiteralValue, is_fast_handler_path, is_fast_identifier};
use crate::lua_api::globals::create_frame::template_chain::FastValueExpr;

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
    if let Some((target_path, method_name)) = parse_inline_global_method_with_self_id_arg(stmt) {
        return Some(FastHandlerRef::GlobalMethodWithSelfIdArg {
            target_path,
            method_name,
        });
    }
    if let Some((target_path, method_name, field)) =
        parse_inline_global_method_with_self_field_arg(stmt)
    {
        return Some(FastHandlerRef::GlobalMethodWithSelfFieldArg {
            target_path,
            method_name,
            field,
        });
    }
    parse_inline_global_method_with_runtime_args(stmt).map(|(target_path, method_name, args)| {
        FastHandlerRef::GlobalMethodWithRuntimeArgs {
            target_path,
            method_name,
            args,
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

fn parse_inline_global_method_with_runtime_args(
    stmt: &str,
) -> Option<(&str, &str, Vec<FastValueExpr<'_>>)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    if !is_fast_handler_path(target_path) || !is_fast_identifier(method_name) || args.is_empty() {
        return None;
    }

    let parsed_args = args
        .split(',')
        .map(str::trim)
        .map(parse_fast_value_expr)
        .collect::<Option<Vec<_>>>()?;
    Some((target_path, method_name, parsed_args))
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

fn parse_fast_value_expr(raw_value: &str) -> Option<FastValueExpr<'_>> {
    super::parse_single_string_literal(raw_value)
        .map(FastValueExpr::String)
        .or_else(|| super::parse_fast_literal_value(raw_value).map(FastValueExpr::Literal))
}
