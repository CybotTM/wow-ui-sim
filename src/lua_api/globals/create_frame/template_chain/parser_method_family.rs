use super::{FastHandlerRef, is_fast_handler_path, is_fast_identifier, is_fast_passthrough_args};

pub(super) fn parse_method_family<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    if let Some((method_name, field)) =
        parse_method_then_unchecked_parent_field_clear_and_show_text(stmt)
    {
        return Some(
            FastHandlerRef::MethodThenUncheckedParentFieldClearAndShowText { method_name, field },
        );
    }
    if let Some(field) = parse_parent_field_local_toggle_shown(stmt) {
        return Some(FastHandlerRef::ParentFieldLocalToggleShown { field });
    }
    if let Some((method_name, then_ref, else_ref)) =
        parse_conditional_self_noarg_method_then_else(stmt)
    {
        return Some(FastHandlerRef::ConditionalSelfNoArgsMethod {
            method_name,
            then_ref: Box::new(then_ref),
            else_ref: Box::new(else_ref),
        });
    }
    if let Some((field, then_ref, else_ref)) = parse_conditional_self_field_then_else(stmt) {
        return Some(FastHandlerRef::ConditionalSelfFieldTruthy {
            field,
            then_ref: Box::new(then_ref),
            else_ref: Box::new(else_ref),
        });
    }
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
    if let Some((field, method_name, self_method_name)) =
        parse_inline_parent_field_method_with_self_noarg_method_result(stmt)
    {
        return Some(FastHandlerRef::ParentFieldMethodWithSelfNoArgMethodResult {
            field,
            method_name,
            self_method_name,
        });
    }
    if let Some(method_name) = parse_inline_parent_method(stmt) {
        return Some(FastHandlerRef::ParentMethod(method_name));
    }
    parse_inline_grandparent_method(stmt).map(FastHandlerRef::GrandparentMethod)
}

fn parse_parent_field_local_toggle_shown(stmt: &str) -> Option<&str> {
    let stmt = stmt.trim();
    let remainder = stmt.strip_prefix("local infoFrame = self:GetParent().")?;
    let (field, tail) = remainder.split_once(';')?;
    let field = field.trim();
    if !is_fast_identifier(field) {
        return None;
    }
    let tail = tail.trim();
    let prefix = "infoFrame:SetShown(not infoFrame:IsShown())";
    (tail == prefix).then_some(field)
}

fn parse_method_then_unchecked_parent_field_clear_and_show_text(
    stmt: &str,
) -> Option<(&str, &str)> {
    let stmt = stmt.trim();
    let (first_stmt, rest) = stmt.split_once(';')?;
    let method_name = parse_inline_self_method(first_stmt.trim())?;
    let rest = rest.trim();
    let prefix = "if (not self:GetChecked()) then";
    let remainder = rest.strip_prefix(prefix)?.trim_start();
    let (then_body, tail) = remainder.split_once("end")?;
    if !tail.trim().is_empty() {
        return None;
    }
    let parts = super::split_inline_sequence_parts(then_body.trim());
    let [clear_stmt, show_stmt] = parts.as_slice() else {
        return None;
    };
    let clear_field = clear_stmt
        .trim()
        .strip_prefix("self:GetParent().")?
        .strip_suffix(":SetText(\"\")")?
        .trim();
    let show_field = show_stmt
        .trim()
        .strip_prefix("self:GetParent().")?
        .strip_suffix(".Text:Show()")?
        .trim();
    (clear_field == show_field && is_fast_identifier(clear_field))
        .then_some((method_name, clear_field))
}

fn parse_conditional_self_noarg_method_then_else<'a>(
    stmt: &'a str,
) -> Option<(&'a str, FastHandlerRef<'a>, FastHandlerRef<'a>)> {
    let remainder = stmt.trim().strip_prefix("if")?.trim_start();
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let (condition, remainder) = remainder.split_once("then")?;
    let condition = condition.trim_end().strip_suffix(')')?.trim();
    let remainder = remainder.trim_start();
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
    let remainder = condition.strip_prefix("self:")?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let method_name = method_name.trim();
    if !(is_fast_identifier(method_name) && args.is_empty()) {
        return None;
    }
    let then_ref = super::parse_inline_fast_handler("OnClick", then_stmt)?;
    let else_ref = super::parse_inline_fast_handler("OnClick", else_stmt)?;
    Some((method_name, then_ref, else_ref))
}

fn parse_conditional_self_field_then_else<'a>(
    stmt: &'a str,
) -> Option<(&'a str, FastHandlerRef<'a>, FastHandlerRef<'a>)> {
    let remainder = stmt.trim().strip_prefix("if")?.trim_start();
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let (condition, remainder) = remainder.split_once("then")?;
    let condition = condition.trim_end().strip_suffix(')')?.trim();
    let field = condition.strip_prefix("self.")?.trim();
    if !is_fast_identifier(field) {
        return None;
    }
    let remainder = remainder.trim_start();
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
    Some((field, then_ref, else_ref))
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

fn parse_inline_parent_field_method_with_self_noarg_method_result(
    stmt: &str,
) -> Option<(&str, &str, &str)> {
    let (field, remainder) = stmt.strip_prefix("self:GetParent().")?.split_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let self_method_name = args
        .strip_suffix("())")?
        .trim()
        .strip_prefix("self:")?
        .trim();
    let field = field.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(field)
        && is_fast_identifier(method_name)
        && is_fast_identifier(self_method_name))
    .then_some((field, method_name, self_method_name))
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
