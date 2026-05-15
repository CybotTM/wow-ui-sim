use std::borrow::Cow;

pub(super) fn normalize_unsupported_lua_escapes(source: &str) -> Cow<'_, str> {
    if !has_unsupported_lua_escape_candidate(source) {
        return Cow::Borrowed(source);
    }

    let normalized = normalize_escape_candidates(source);
    if normalized == source {
        Cow::Borrowed(source)
    } else {
        Cow::Owned(normalized)
    }
}

fn normalize_escape_candidates(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            normalize_escape_candidate(&mut output, &mut chars);
        } else {
            output.push(ch);
        }
    }

    output
}

fn normalize_escape_candidate(
    output: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) {
    match chars.peek().copied() {
        Some('x') => normalize_hex_escape(output, chars),
        Some('u') => normalize_unicode_escape(output, chars),
        Some(next) if !next.is_ascii() => {
            chars.next();
            output.push(next);
        }
        _ => output.push('\\'),
    }
}

fn normalize_hex_escape(output: &mut String, chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
    chars.next();
    if let Some(byte) = read_hex_byte(chars) {
        write_decimal_lua_escape(output, byte);
    } else {
        output.push('\\');
        output.push('x');
    }
}

fn normalize_unicode_escape(
    output: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) {
    chars.next();
    if let Some(ch) = read_unicode_escape(chars) {
        output.push(ch);
    } else {
        output.push('\\');
        output.push('u');
    }
}

fn has_unsupported_lua_escape_candidate(source: &str) -> bool {
    if source.contains("\\x") || source.contains("\\u") {
        return true;
    }

    let mut chars = source.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' && chars.next().is_some_and(|next| !next.is_ascii()) {
            return true;
        }
    }

    false
}

fn read_hex_byte(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Option<u8> {
    let high = chars.next()?.to_digit(16)?;
    let low = chars.next()?.to_digit(16)?;
    Some(((high << 4) | low) as u8)
}

fn read_unicode_escape(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Option<char> {
    let mut codepoint = 0;
    for _ in 0..4 {
        codepoint = (codepoint << 4) | chars.next()?.to_digit(16)?;
    }
    char::from_u32(codepoint)
}

fn write_decimal_lua_escape(output: &mut String, byte: u8) {
    output.push('\\');
    output.push_str(&format!("{byte:03}"));
}

#[cfg(test)]
mod tests {
    use super::normalize_unsupported_lua_escapes;

    #[test]
    fn lua_source_normalizes_hex_escapes_for_rilua() {
        assert_eq!(
            normalize_unsupported_lua_escapes(r#"return "\x1F""#).as_ref(),
            r#"return "\031""#
        );
    }

    #[test]
    fn lua_source_normalizes_unicode_escapes_for_rilua() {
        assert_eq!(
            normalize_unsupported_lua_escapes(r#"return "\u2013""#).as_ref(),
            "return \"\u{2013}\""
        );
    }

    #[test]
    fn lua_source_normalizes_escaped_unicode_punctuation_for_rilua() {
        assert_eq!(
            normalize_unsupported_lua_escapes("return \"\\“quoted\\”\"").as_ref(),
            "return \"“quoted”\""
        );
    }
}
