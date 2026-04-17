use super::{FastHandlerRef, FastLiteralValue};

pub(super) fn parse_inline_fast_handler<'a>(
    _handler_name: &'static str,
    body: &'a str,
) -> Option<FastHandlerRef<'a>> {
    let trimmed = strip_leading_comment_lines(body.trim());
    if trimmed.is_empty() {
        return Some(FastHandlerRef::NoOp);
    }
    let stmt = trimmed.strip_suffix(';').unwrap_or(trimmed).trim();
    if stmt.is_empty() {
        return Some(FastHandlerRef::NoOp);
    }
    if let Some(sequence) = parse_inline_sequence(stmt) {
        return Some(sequence);
    }
    parse_inline_single_fast_handler(stmt)
}

fn parse_inline_single_fast_handler<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
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
    if let Some(method_name) = parse_inline_grandparent_method(stmt) {
        return Some(FastHandlerRef::GrandparentMethod(method_name));
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
    if let Some((first, second, third)) = parse_inline_register_for_clicks(stmt) {
        return Some(FastHandlerRef::RegisterForClicks {
            first,
            second,
            third,
        });
    }
    if let Some(button) = parse_inline_register_for_drag(stmt) {
        return Some(FastHandlerRef::RegisterForDrag(button));
    }
    if let Some(alpha) = parse_inline_set_alpha(stmt) {
        return Some(FastHandlerRef::SetAlpha(alpha));
    }
    if let Some(delta) = parse_inline_set_frame_level_from_parent(stmt) {
        return Some(FastHandlerRef::SetFrameLevelFromParent(delta));
    }
    if let Some(assign) = parse_inline_ancestor_assignment(stmt) {
        return Some(assign);
    }
    if let Some(assign) = parse_inline_assignment(stmt) {
        return Some(assign);
    }
    if let Some(assign) = parse_inline_nested_assignment(stmt) {
        return Some(assign);
    }
    if let Some(assign) = parse_inline_parent_assignment(stmt) {
        return Some(assign);
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self)")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::Function(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("()")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionNoArgs(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self:GetID())")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithSelfIdArg(function_name));
    }
    if let Some((function_name, arg)) = parse_inline_function_with_self_string_arg(stmt) {
        return Some(FastHandlerRef::FunctionWithSelfStringArg { function_name, arg });
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
        parse_inline_function_with_global_and_self_arg(stmt)
    {
        return Some(FastHandlerRef::FunctionWithGlobalAndSelfArg {
            function_name,
            global_arg_path,
        });
    }
    if let Some((function_name, field)) = parse_inline_function_with_self_and_parent_field_arg(stmt)
    {
        return Some(FastHandlerRef::FunctionWithSelfAndParentFieldArg {
            function_name,
            field,
        });
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self:GetParent())")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithParentArg(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self:GetParent():GetParent())")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithGrandparentArg(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self:GetParent():GetID())")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithParentIdArg(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self, event, ...)")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithEventVarargs(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self, button)")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithButton(function_name));
    }
    stmt.strip_suffix("(self, elapsed)")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
        .map(FastHandlerRef::FunctionWithElapsed)
}

fn parse_inline_sequence(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let parts = stmt
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    match parts.as_slice() {
        [first, second] => Some(FastHandlerRef::Sequence2(Box::new((
            parse_inline_single_fast_handler(first)?,
            parse_inline_single_fast_handler(second)?,
        )))),
        [first, second, third] => Some(FastHandlerRef::Sequence3(Box::new((
            parse_inline_single_fast_handler(first)?,
            parse_inline_single_fast_handler(second)?,
            parse_inline_single_fast_handler(third)?,
        )))),
        _ => None,
    }
}

fn parse_inline_self_method(stmt: &str) -> Option<&str> {
    parse_inline_method_call(stmt, "self:")
}

fn parse_inline_self_method_with_bool_arg(stmt: &str) -> Option<(&str, bool)> {
    let remainder = stmt.strip_prefix("self:")?;
    let (method_name, args) = remainder.split_once('(')?;
    let value = parse_single_bool_literal(args.strip_suffix(')')?.trim())?;
    let method_name = method_name.trim();
    is_fast_identifier(method_name).then_some((method_name, value))
}

fn parse_inline_self_method_with_string_arg(stmt: &str) -> Option<(&str, &str)> {
    let remainder = stmt.strip_prefix("self:")?;
    let (method_name, args) = remainder.split_once('(')?;
    let arg = parse_single_string_literal(args.strip_suffix(')')?.trim())?;
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
    let arg = parse_single_string_literal(args.strip_suffix(')')?.trim())?;
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
    let first = parse_single_string_literal(parts.next()?)?;
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
    let arg = parse_single_string_literal(args.strip_suffix(')')?.trim())?;
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

fn parse_inline_global_method(stmt: &str) -> Option<(&str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path)
        && is_fast_identifier(method_name)
        && is_fast_passthrough_args(args))
    .then_some((target_path, method_name))
}

fn parse_inline_global_method_with_self_string_arg(stmt: &str) -> Option<(&str, &str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (self_arg, raw_string_arg) = args.split_once(',')?;
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    let arg = parse_single_string_literal(raw_string_arg.trim())?;
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

fn parse_inline_global_method_then_assign(
    stmt: &str,
) -> Option<(&str, &str, &str, FastLiteralValue<'_>)> {
    let (first, second) = stmt.split_once(';')?;
    let (target_path, method_name) = parse_inline_global_method(first.trim())?;
    let FastHandlerRef::AssignLiteral { field, value } = parse_inline_assignment(second.trim())?
    else {
        return None;
    };
    Some((target_path, method_name, field, value))
}

fn parse_inline_function_with_self_string_arg(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (self_arg, raw_string_arg) = args.split_once(',')?;
    let function_name = function_name.trim();
    let arg = parse_single_string_literal(raw_string_arg.trim())?;
    (is_fast_handler_path(function_name) && self_arg.trim() == "self")
        .then_some((function_name, arg))
}

fn parse_inline_function_with_number_arg(stmt: &str) -> Option<(&str, f64)> {
    let (function_name, args) = stmt.split_once('(')?;
    let value = args.strip_suffix(')')?.trim().parse::<f64>().ok()?;
    let function_name = function_name.trim();
    is_fast_handler_path(function_name).then_some((function_name, value))
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

fn parse_inline_register_for_clicks(stmt: &str) -> Option<(&str, Option<&str>, Option<&str>)> {
    let args = stmt
        .strip_prefix("self:RegisterForClicks(")?
        .strip_suffix(')')?
        .trim();
    let args = parse_string_literal_args(args)?;
    match args.as_slice() {
        [first] => Some((first, None, None)),
        [first, second] => Some((first, Some(second), None)),
        [first, second, third] => Some((first, Some(second), Some(third))),
        _ => None,
    }
}

fn parse_inline_register_for_drag(stmt: &str) -> Option<&str> {
    let args = stmt
        .strip_prefix("self:RegisterForDrag(")?
        .strip_suffix(')')?
        .trim();
    let args = parse_string_literal_args(args)?;
    match args.as_slice() {
        [button] => Some(button),
        _ => None,
    }
}

fn parse_inline_set_alpha(stmt: &str) -> Option<f64> {
    stmt.strip_prefix("self:SetAlpha(")?
        .strip_suffix(')')?
        .trim()
        .parse::<f64>()
        .ok()
}

fn parse_inline_set_frame_level_from_parent(stmt: &str) -> Option<i32> {
    let remainder = stmt
        .strip_prefix("self:SetFrameLevel(self:GetParent():GetFrameLevel()")?
        .trim();
    if let Some(remainder) = remainder.strip_suffix(')') {
        let remainder = remainder.trim();
        if remainder.is_empty() {
            return Some(0);
        }
        if let Some(delta) = remainder.strip_prefix('+') {
            return delta.trim().parse::<i32>().ok();
        }
        if let Some(delta) = remainder.strip_prefix('-') {
            return delta.trim().parse::<i32>().ok().map(|delta| -delta);
        }
    }
    None
}

fn parse_inline_ancestor_assignment(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let (field, depth) =
        if let Some((field, _)) = stmt.strip_prefix("self.")?.split_once("= self:GetParent()") {
            let field = field.trim();
            if let Some(suffix) = stmt.trim().strip_prefix(&format!("self.{field} = ")) {
                let depth = if suffix.trim() == "self:GetParent()" {
                    1
                } else if suffix.trim() == "self:GetParent():GetParent()" {
                    2
                } else {
                    return None;
                };
                (field, depth)
            } else {
                return None;
            }
        } else {
            return None;
        };
    if !is_fast_identifier(field) {
        return None;
    }
    Some(FastHandlerRef::AssignAncestorRef { field, depth })
}

fn parse_inline_assignment(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let (field, raw_value) = stmt.strip_prefix("self.")?.split_once('=')?;
    let field = field.trim();
    let raw_value = raw_value.trim();
    if !is_fast_identifier(field) {
        return None;
    }
    let value = parse_fast_literal_value(raw_value)?;
    Some(FastHandlerRef::AssignLiteral { field, value })
}

fn parse_inline_parent_assignment(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let (field, raw_value) = stmt.strip_prefix("self:GetParent().")?.split_once('=')?;
    let field = field.trim();
    let raw_value = raw_value.trim();
    if !is_fast_identifier(field) {
        return None;
    }
    let value = parse_fast_literal_value(raw_value)?;
    Some(FastHandlerRef::AssignParentField { field, value })
}

fn parse_string_literal_args(args: &str) -> Option<Vec<&str>> {
    if args.is_empty() {
        return Some(Vec::new());
    }
    let mut values = Vec::new();
    for part in args.split(',') {
        let part = part.trim();
        let value = part.strip_prefix('"')?.strip_suffix('"')?;
        values.push(value);
    }
    Some(values)
}

fn parse_single_string_literal(arg: &str) -> Option<&str> {
    arg.strip_prefix('"')?.strip_suffix('"')
}

fn parse_single_bool_literal(arg: &str) -> Option<bool> {
    match arg {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn strip_leading_comment_lines(mut stmt: &str) -> &str {
    loop {
        let trimmed = stmt.trim_start();
        let Some(comment) = trimmed.strip_prefix("--") else {
            return trimmed;
        };
        let Some((_, rest)) = comment.split_once('\n') else {
            return "";
        };
        stmt = rest;
    }
}

fn is_fast_handler_path(path: &str) -> bool {
    path.split('.').all(is_fast_identifier)
}

fn is_fast_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(ch) if ch == '_' || ch.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn is_fast_passthrough_args(args: &str) -> bool {
    args.split(',')
        .map(str::trim)
        .all(|arg| arg.is_empty() || arg == "..." || is_fast_identifier(arg))
}

fn parse_fast_literal_value(raw_value: &str) -> Option<FastLiteralValue<'_>> {
    if raw_value.eq("nil") {
        Some(FastLiteralValue::Nil)
    } else if raw_value.eq("true") {
        Some(FastLiteralValue::Bool(true))
    } else if raw_value.eq("false") {
        Some(FastLiteralValue::Bool(false))
    } else if let Ok(number) = raw_value.parse::<f64>() {
        Some(FastLiteralValue::Number(number))
    } else if is_fast_handler_path(raw_value) {
        Some(FastLiteralValue::Global(raw_value))
    } else {
        None
    }
}

fn parse_inline_nested_assignment(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let (lhs, rhs) = stmt.split_once('=')?;
    let lhs = lhs.trim();
    let rhs = rhs.trim();
    let lhs = lhs.strip_prefix("self.")?;
    let (parent_field, field) = lhs.split_once('.')?;
    let value = parse_fast_literal_value(rhs)?;
    (is_fast_identifier(parent_field) && is_fast_identifier(field)).then_some(
        FastHandlerRef::AssignNestedLiteral {
            parent_field,
            field,
            value,
        },
    )
}
