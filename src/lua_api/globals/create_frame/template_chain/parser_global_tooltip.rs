use super::{is_fast_handler_path, is_fast_identifier};

pub(in super::super) fn parse_global_tooltip_set_owner_then_set_text(
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
    let (target_path, method_name, anchor) =
        super::parse_inline_global_method_with_self_string_arg(stmt)?;
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
        super::super::split_top_level_args(text_args)?
            .try_into()
            .ok()?;
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

pub(in super::super) fn parse_global_tooltip_set_owner_then_set_text_literal(
    stmt: &str,
) -> Option<(&str, &str, &str, f64, f64, f64)> {
    let (first, second) = stmt.split_once(';')?;
    let (target_path, method_name, anchor) =
        super::parse_inline_global_method_with_self_string_arg(first.trim())?;
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
    let text = super::super::parse_single_string_literal(parts.next()?)?;
    let red = parts.next()?.parse::<f64>().ok()?;
    let green = parts.next()?.parse::<f64>().ok()?;
    let blue = parts.next()?.parse::<f64>().ok()?;
    if parts.next().is_some() {
        return None;
    }

    Some((target_path, anchor, text, red, green, blue))
}

pub(in super::super) fn parse_conditional_tooltip(
    stmt: &str,
) -> Option<(&str, &str, &str, &str, &str, &str)> {
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
        super::super::split_top_level_args(text_args)?
            .try_into()
            .ok()?;
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
mod tests {
    use super::{parse_conditional_tooltip, parse_global_tooltip_set_owner_then_set_text};

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
