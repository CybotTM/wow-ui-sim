use super::{FastHandlerRef, is_fast_handler_path, is_fast_identifier, is_fast_passthrough_args};

type MethodFamilyParser = for<'a> fn(&'a str) -> Option<FastHandlerRef<'a>>;

const METHOD_FAMILY_PARSERS: &[MethodFamilyParser] = &[
    parse_conditional_self_text_empty_show_text_child_ref,
    parse_method_then_unchecked_parent_field_clear_and_show_text_ref,
    parse_conditional_self_get_text_non_empty_then_parent_method_ref,
    parse_parent_field_local_toggle_shown_ref,
    parse_conditional_self_noarg_method_then_else_ref,
    parse_conditional_not_self_noarg_method_then_ref,
    parse_conditional_self_field_then_else_ref,
    parse_inline_self_method_with_bool_arg_ref,
    parse_inline_self_method_with_number_arg_ref,
    parse_inline_self_method_with_two_number_args_ref,
    parse_inline_self_method_with_string_arg_ref,
    parse_inline_self_method_ref,
    parse_inline_self_field_method_with_string_arg_ref,
    parse_inline_self_field_method_with_number_arg_ref,
    parse_inline_self_field_method_with_string_number_number_args_ref,
    parse_inline_self_field_method_with_string_self_string_number_number_args_ref,
    parse_inline_self_field_method_with_self_field_arg_ref,
    parse_inline_self_field_method_with_global_arg_ref,
    parse_inline_self_field_method_ref,
    parse_inline_parent_method_with_string_arg_ref,
    parse_inline_parent_field_method_with_self_noarg_method_result_ref,
    parse_inline_grandparent_field_method_ref,
    parse_inline_grandparent_method_with_not_self_checked_arg_ref,
    parse_inline_parent_method_ref,
    parse_inline_grandparent_method_ref,
];

pub(super) fn parse_method_family<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    METHOD_FAMILY_PARSERS
        .iter()
        .find_map(|parse_method_candidate| parse_method_candidate(stmt))
}

fn parse_conditional_self_text_empty_show_text_child_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_conditional_self_text_empty_show_text_child(stmt)
        .map(|()| FastHandlerRef::ConditionalSelfTextEmptyShowTextChild)
}

fn parse_method_then_unchecked_parent_field_clear_and_show_text_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_method_then_unchecked_parent_field_clear_and_show_text(stmt).map(
        |(method_name, field)| FastHandlerRef::MethodThenUncheckedParentFieldClearAndShowText {
            method_name,
            field,
        },
    )
}

fn parse_conditional_self_get_text_non_empty_then_parent_method_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_conditional_self_get_text_non_empty_then_parent_method(stmt).map(|method_name| {
        FastHandlerRef::ConditionalSelfGetTextNonEmptyThenParentMethodWithSelfGetTextAndClear {
            method_name,
        }
    })
}

fn parse_parent_field_local_toggle_shown_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_parent_field_local_toggle_shown(stmt)
        .map(|field| FastHandlerRef::ParentFieldLocalToggleShown { field })
}

fn parse_conditional_self_noarg_method_then_else_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_conditional_self_noarg_method_then_else(stmt).map(|(method_name, then_ref, else_ref)| {
        FastHandlerRef::ConditionalSelfNoArgsMethod {
            method_name,
            then_ref: Box::new(then_ref),
            else_ref: Box::new(else_ref),
        }
    })
}

fn parse_conditional_not_self_noarg_method_then_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_conditional_not_self_noarg_method_then(stmt).map(|(method_name, then_ref)| {
        FastHandlerRef::ConditionalNotSelfNoArgsMethodThen {
            method_name,
            then_ref: Box::new(then_ref),
        }
    })
}

fn parse_conditional_self_field_then_else_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_conditional_self_field_then_else(stmt).map(|(field, then_ref, else_ref)| {
        FastHandlerRef::ConditionalSelfFieldTruthy {
            field,
            then_ref: Box::new(then_ref),
            else_ref: Box::new(else_ref),
        }
    })
}

fn parse_inline_self_method_with_bool_arg_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_method_with_bool_arg(stmt)
        .map(|(method_name, value)| FastHandlerRef::MethodWithBoolArg { method_name, value })
}

fn parse_inline_self_method_with_number_arg_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_method_with_number_arg(stmt)
        .map(|(method_name, value)| FastHandlerRef::MethodWithNumberArg { method_name, value })
}

fn parse_inline_self_method_with_two_number_args_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_method_with_two_number_args(stmt).map(|(method_name, first, second)| {
        FastHandlerRef::MethodWithTwoNumberArgs {
            method_name,
            first,
            second,
        }
    })
}

fn parse_inline_self_method_with_string_arg_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_method_with_string_arg(stmt)
        .map(|(method_name, arg)| FastHandlerRef::MethodWithStringArg { method_name, arg })
}

fn parse_inline_self_method_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_method(stmt).map(FastHandlerRef::Method)
}

fn parse_inline_self_field_method_with_string_arg_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_field_method_with_string_arg(stmt).map(|(field, method_name, arg)| {
        FastHandlerRef::SelfFieldMethodWithStringArg {
            field,
            method_name,
            arg,
        }
    })
}

fn parse_inline_self_field_method_with_number_arg_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_field_method_with_number_arg(stmt).map(|(field, method_name, value)| {
        FastHandlerRef::SelfFieldMethodWithNumberArg {
            field,
            method_name,
            value,
        }
    })
}

fn parse_inline_self_field_method_with_string_number_number_args_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_field_method_with_string_number_number_args(stmt).map(
        |(field, method_name, first, second, third)| {
            FastHandlerRef::SelfFieldMethodWithStringNumberNumberArgs {
                field,
                method_name,
                first,
                second,
                third,
            }
        },
    )
}

fn parse_inline_self_field_method_with_string_self_string_number_number_args_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_field_method_with_string_self_string_number_number_args(stmt).map(
        |(field, method_name, first, third, fourth, fifth)| {
            FastHandlerRef::SelfFieldMethodWithStringSelfStringNumberNumberArgs {
                field,
                method_name,
                first,
                third,
                fourth,
                fifth,
            }
        },
    )
}

fn parse_inline_self_field_method_with_self_field_arg_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_field_method_with_self_field_arg(stmt).map(
        |(field, method_name, arg_field)| FastHandlerRef::SelfFieldMethodWithSelfFieldArg {
            field,
            method_name,
            arg_field,
        },
    )
}

fn parse_inline_self_field_method_with_global_arg_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_field_method_with_global_arg(stmt).map(|(field, method_name, arg_path)| {
        FastHandlerRef::SelfFieldMethodWithGlobalArg {
            field,
            method_name,
            arg_path,
        }
    })
}

fn parse_inline_self_field_method_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_inline_self_field_method(stmt)
        .map(|(field, method_name)| FastHandlerRef::SelfFieldMethod { field, method_name })
}

fn parse_inline_parent_method_with_string_arg_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_inline_parent_method_with_string_arg(stmt)
        .map(|(method_name, arg)| FastHandlerRef::ParentMethodWithStringArg { method_name, arg })
}

fn parse_inline_parent_field_method_with_self_noarg_method_result_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_inline_parent_field_method_with_self_noarg_method_result(stmt).map(
        |(field, method_name, self_method_name)| {
            FastHandlerRef::ParentFieldMethodWithSelfNoArgMethodResult {
                field,
                method_name,
                self_method_name,
            }
        },
    )
}

fn parse_inline_grandparent_field_method_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_inline_grandparent_field_method(stmt)
        .map(|(field, method_name)| FastHandlerRef::GrandparentFieldMethod { field, method_name })
}

fn parse_inline_grandparent_method_with_not_self_checked_arg_ref<'a>(
    stmt: &'a str,
) -> Option<FastHandlerRef<'a>> {
    parse_inline_grandparent_method_with_not_self_checked_arg(stmt)
        .map(|method_name| FastHandlerRef::GrandparentMethodWithNotSelfCheckedArg { method_name })
}

fn parse_inline_parent_method_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_inline_parent_method(stmt).map(FastHandlerRef::ParentMethod)
}

fn parse_inline_grandparent_method_ref<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
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

fn parse_conditional_self_get_text_non_empty_then_parent_method(stmt: &str) -> Option<&str> {
    let stmt = stmt.trim();
    let prefix = "local text = self:GetText();";
    let remainder = stmt.strip_prefix(prefix)?.trim_start();
    let remainder = remainder
        .strip_prefix("if text and #text > 0 then")?
        .trim_start();
    let (parent_stmt, tail) = remainder.split_once(';')?;
    let method_name = parent_stmt
        .trim()
        .strip_prefix("self:GetParent():")?
        .strip_suffix("(self:GetText())")?
        .trim();
    if !is_fast_identifier(method_name) {
        return None;
    }
    let tail = tail.trim();
    let expected = r#"self:SetText("");"#;
    let expected_no_semi = r#"self:SetText("")"#;
    let end_tail = tail.strip_suffix("end")?.trim();
    (end_tail == expected || end_tail == expected_no_semi).then_some(method_name)
}

fn parse_conditional_self_text_empty_show_text_child(stmt: &str) -> Option<()> {
    let stmt = stmt.trim();
    let prefix = "if ( self:GetText() == \"\" ) then";
    let remainder = stmt.strip_prefix(prefix)?.trim_start();
    let body = remainder.strip_suffix("end")?.trim();
    (body == "self.Text:Show();" || body == "self.Text:Show()").then_some(())
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

fn parse_conditional_not_self_noarg_method_then<'a>(
    stmt: &'a str,
) -> Option<(&'a str, FastHandlerRef<'a>)> {
    let remainder = stmt.trim().strip_prefix("if")?.trim_start();
    let remainder = remainder.strip_prefix('(')?.trim_start();
    let (condition, remainder) = remainder.split_once("then")?;
    let condition = condition.trim_end().strip_suffix(')')?.trim();
    let remainder = remainder.trim_start();
    let then_stmt = remainder.strip_suffix("end")?.trim();
    let remainder = condition.strip_prefix("not self:")?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let method_name = method_name.trim();
    if !(is_fast_identifier(method_name) && args.is_empty()) {
        return None;
    }
    let then_ref = super::parse_inline_fast_handler("OnEnter", then_stmt)?;
    Some((method_name, then_ref))
}

fn parse_inline_self_method(stmt: &str) -> Option<&str> {
    parse_inline_method_call(stmt, "self:")
}

fn parse_inline_grandparent_method_with_not_self_checked_arg(stmt: &str) -> Option<&str> {
    let remainder = stmt.strip_prefix("self:GetParent():GetParent():")?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(method_name) && args == "not self:GetChecked()").then_some(method_name)
}

fn parse_inline_grandparent_field_method(stmt: &str) -> Option<(&str, &str)> {
    let remainder = stmt.strip_prefix("self:GetParent():GetParent().")?;
    let (field, remainder) = remainder.split_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let field = field.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(field) && is_fast_identifier(method_name) && is_fast_passthrough_args(args))
        .then_some((field, method_name))
}

fn parse_inline_self_method_with_bool_arg(stmt: &str) -> Option<(&str, bool)> {
    let remainder = stmt.strip_prefix("self:")?;
    let (method_name, args) = remainder.split_once('(')?;
    let value = super::parse_single_bool_literal(args.strip_suffix(')')?.trim())?;
    let method_name = method_name.trim();
    is_fast_identifier(method_name).then_some((method_name, value))
}

fn parse_inline_self_method_with_number_arg(stmt: &str) -> Option<(&str, f64)> {
    let remainder = stmt.strip_prefix("self:")?;
    let (method_name, args) = remainder.split_once('(')?;
    let value = args.strip_suffix(')')?.trim().parse::<f64>().ok()?;
    let method_name = method_name.trim();
    is_fast_identifier(method_name).then_some((method_name, value))
}

fn parse_inline_self_method_with_two_number_args(stmt: &str) -> Option<(&str, f64, f64)> {
    let remainder = stmt.strip_prefix("self:")?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (first, second) = args.split_once(',')?;
    let first = first.trim().parse::<f64>().ok()?;
    let second = second.trim().parse::<f64>().ok()?;
    let method_name = method_name.trim();
    is_fast_identifier(method_name).then_some((method_name, first, second))
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

fn parse_inline_self_field_method_with_string_self_string_number_number_args(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, f64, f64)> {
    let (field, remainder) = stmt.strip_prefix("self.")?.split_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let mut parts = args.split(',').map(str::trim);
    let first = super::parse_single_string_literal(parts.next()?)?;
    let second = parts.next()?;
    let third = super::parse_single_string_literal(parts.next()?)?;
    let fourth = parts.next()?.parse::<f64>().ok()?;
    let fifth = parts.next()?.parse::<f64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    let field = field.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(field) && is_fast_identifier(method_name) && second == "self").then_some((
        field,
        method_name,
        first,
        third,
        fourth,
        fifth,
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

#[cfg(test)]
mod tests {
    use super::parse_method_family;
    use crate::lua_api::globals::create_frame::template_chain::FastHandlerRef;

    #[test]
    fn dispatches_plain_self_method() {
        let parsed = parse_method_family("self:Hide()");

        match parsed {
            Some(FastHandlerRef::Method("Hide")) => {}
            _ => panic!("expected self method parser to dispatch Hide"),
        }
    }

    #[test]
    fn dispatches_self_field_method_with_string_arg() {
        let parsed = parse_method_family(r#"self.Text:SetText("hello")"#);

        match parsed {
            Some(FastHandlerRef::SelfFieldMethodWithStringArg {
                field: "Text",
                method_name: "SetText",
                arg: "hello",
            }) => {}
            _ => panic!("expected self field string-arg method parser"),
        }
    }

    #[test]
    fn does_not_fuse_two_string_args_into_one() {
        // MountListButtonTemplate's inline OnLoad. The generic single
        // string-arg parser must not swallow this as one fused argument —
        // that registered mount-list rows for an unmatchable click edge and
        // broke real mouse clicks (selection never switched).
        let parsed =
            parse_method_family(r#"self:RegisterForClicks("LeftButtonUp", "RightButtonUp")"#);

        match parsed {
            Some(FastHandlerRef::MethodWithStringArg { arg, .. }) => {
                panic!("two-arg call fused into single string arg: {arg:?}")
            }
            _ => {}
        }
    }

    #[test]
    fn accepts_string_arg_containing_comma() {
        let parsed = parse_method_family(r#"self:SetText("Hello, world")"#);

        match parsed {
            Some(FastHandlerRef::MethodWithStringArg {
                method_name: "SetText",
                arg: "Hello, world",
            }) => {}
            _ => panic!("expected comma inside one string literal to stay a single arg"),
        }
    }
}
