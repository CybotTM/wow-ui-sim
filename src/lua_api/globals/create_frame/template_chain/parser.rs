use super::{FastHandlerRef, FastLiteralValue};

#[path = "parser_function_family.rs"]
mod parser_function_family;
#[path = "parser_global_family.rs"]
mod parser_global_family;
#[path = "parser_method_family.rs"]
mod parser_method_family;

use self::parser_function_family::{
    parse_copy_club_ticket_to_clipboard_from_parent, parse_function_family,
};
use self::parser_global_family::parse_global_family;
use self::parser_method_family::parse_method_family;

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
    if let Some(handler) = parse_pre_split_special_handler(stmt) {
        return Some(handler);
    }
    if let Some(sequence) = parse_inline_sequence(stmt) {
        return Some(sequence);
    }
    parse_inline_single_fast_handler(stmt)
}

fn parse_pre_split_special_handler<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_play_sound_then_copy_club_ticket(stmt)
        .or_else(|| parse_parent_field_local_click_if_enabled(stmt))
}

fn parse_play_sound_then_copy_club_ticket<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let stmt = stmt.trim();
    let (first_stmt, rest) = stmt.split_once(';')?;
    let (function_name, sound_path) = parse_global_function_call(first_stmt.trim())?;
    if function_name != "PlaySound" || !is_fast_handler_path(sound_path) {
        return None;
    }
    parse_copy_club_ticket_to_clipboard_from_parent(rest.trim()).map(
        |_| FastHandlerRef::PlaySoundThenCopyClubTicketToClipboardFromParent { sound_path },
    )
}

fn parse_parent_field_local_click_if_enabled<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    let stmt = stmt.trim();
    let remainder = stmt.strip_prefix("local ")?;
    let (local_name, remainder) = remainder.split_once('=')?;
    let local_name = local_name.trim();
    if !is_fast_identifier(local_name) {
        return None;
    }
    let (target_expr, remainder) = remainder.split_once(';')?;
    let field = target_expr
        .trim()
        .strip_prefix("self:GetParent().")?
        .trim();
    if !is_fast_identifier(field) {
        return None;
    }
    let remainder = remainder.trim();
    let prefix = format!("if {local_name}:IsEnabled() then");
    let remainder = remainder.strip_prefix(&prefix)?.trim_start();
    let body = remainder.strip_suffix("end")?.trim();
    let expected = format!("{local_name}:GetScript(\"OnClick\")({local_name});");
    (body == expected || body == expected.trim_end_matches(';'))
        .then_some(FastHandlerRef::ParentFieldLocalClickIfEnabled { field })
}

fn parse_inline_single_fast_handler<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_method_family(stmt)
        .or_else(|| parse_global_family(stmt))
        .or_else(|| parse_registration_family(stmt))
        .or_else(|| parse_assignment_family(stmt))
        .or_else(|| parse_function_family(stmt))
}

fn parse_registration_family<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
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
    parse_inline_set_frame_level_from_parent(stmt).map(FastHandlerRef::SetFrameLevelFromParent)
}

fn parse_assignment_family<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    parse_inline_ancestor_assignment(stmt)
        .or_else(|| parse_inline_global_assignment(stmt))
        .or_else(|| parse_inline_assignment(stmt))
        .or_else(|| parse_inline_nested_global_pair_table_assignment(stmt))
        .or_else(|| parse_inline_nested_assignment(stmt))
        .or_else(|| parse_inline_parent_assignment(stmt))
}

fn parse_inline_sequence(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let parts = split_inline_sequence_parts(stmt);
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

fn split_inline_sequence_parts(stmt: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut chars = stmt.char_indices().peekable();
    let mut in_string = false;
    let mut quote = '\0';
    let mut escaped = false;
    let mut in_comment = false;
    let mut block_depth = 0usize;
    let mut paren_depth = 0usize;

    while let Some((idx, ch)) = chars.next() {
        if in_comment {
            if ch == '\n' {
                in_comment = false;
                start = idx + ch.len_utf8();
            }
            continue;
        }

        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == quote {
                in_string = false;
            }
            continue;
        }

        if ch == '"' || ch == '\'' {
            in_string = true;
            quote = ch;
            continue;
        }

        match ch {
            '(' => {
                paren_depth += 1;
                continue;
            }
            ')' => {
                paren_depth = paren_depth.saturating_sub(1);
                continue;
            }
            '\n' if block_depth == 0 && paren_depth == 0 => {
                let part = stmt[start..idx].trim();
                let rest = stmt[idx + ch.len_utf8()..].trim_start();
                if !part.is_empty() && should_keep_local_prelude_with_following_block(part, rest) {
                    continue;
                }
                if !part.is_empty() {
                    parts.push(part);
                }
                start = idx + ch.len_utf8();
                continue;
            }
            _ => {}
        }

        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut end = idx + ch.len_utf8();
            while let Some((next_idx, next_ch)) = chars.peek().copied() {
                if next_ch.is_ascii_alphanumeric() || next_ch == '_' {
                    end = next_idx + next_ch.len_utf8();
                    let _ = chars.next();
                } else {
                    break;
                }
            }
            match &stmt[idx..end] {
                "if" => block_depth += 1,
                "end" if block_depth > 0 => {
                    block_depth -= 1;
                    if block_depth == 0 {
                        let rest = stmt[end..].trim_start();
                        if !rest.is_empty() && !rest.starts_with(';') {
                            let part = stmt[start..end].trim();
                            if !part.is_empty() {
                                parts.push(part);
                            }
                            start = end;
                        }
                    }
                }
                _ => {}
            }
            continue;
        }

        if ch == '-' && matches!(chars.peek(), Some((_, '-'))) {
            let part = stmt[start..idx].trim();
            if !part.is_empty() {
                parts.push(part);
            }
            let _ = chars.next();
            in_comment = true;
            continue;
        }

        if ch == ';' && block_depth == 0 && paren_depth == 0 {
            let part = stmt[start..idx].trim();
            if !part.is_empty() {
                parts.push(part);
            }
            start = idx + ch.len_utf8();
        }
    }

    if !in_comment {
        let part = stmt[start..].trim();
        if !part.is_empty() {
            parts.push(part);
        }
    }

    parts
}

fn should_keep_local_prelude_with_following_block(part: &str, rest: &str) -> bool {
    let part = part.trim_start();
    let rest = rest.trim_start();
    part.starts_with("local ")
        && (rest.starts_with("local ")
            || rest.starts_with("if ")
            || rest.starts_with("if(")
            || rest.starts_with("if\t")
            || rest.starts_with("if\n"))
}

fn parse_global_function_call(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?;
    let function_name = function_name.trim();
    is_fast_handler_path(function_name).then_some((function_name, args.trim()))
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

fn parse_inline_global_assignment(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let (lhs, raw_value) = stmt.split_once('=')?;
    let lhs = lhs.trim();
    let raw_value = raw_value.trim();
    if lhs.starts_with("self.") || lhs.starts_with("self:GetParent().") {
        return None;
    }
    let (target_path, field) = lhs.rsplit_once('.')?;
    let target_path = target_path.trim();
    let field = field.trim();
    if !(is_fast_handler_path(target_path) && is_fast_identifier(field)) {
        return None;
    }
    let value = parse_fast_literal_value(raw_value)?;
    Some(FastHandlerRef::AssignGlobalFieldLiteral {
        target_path,
        field,
        value,
    })
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

fn parse_inline_nested_global_pair_table_assignment(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let (lhs, rhs) = stmt.split_once('=')?;
    let lhs = lhs.trim().strip_prefix("self.")?;
    let (parent_field, field) = lhs.split_once('.')?;
    if !(is_fast_identifier(parent_field) && is_fast_identifier(field)) {
        return None;
    }

    let rhs = rhs.trim().strip_prefix('{')?.strip_suffix('}')?.trim();
    let mut parts = rhs.split(',').map(str::trim);
    let first_path = parts.next()?;
    let second_path = parts.next()?;
    if parts.next().is_some() {
        return None;
    }

    (is_fast_handler_path(first_path) && is_fast_handler_path(second_path)).then_some(
        FastHandlerRef::AssignNestedGlobalPairTable {
            parent_field,
            field,
            first_path,
            second_path,
        },
    )
}
