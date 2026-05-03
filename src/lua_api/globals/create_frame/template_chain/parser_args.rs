pub(super) fn split_top_level_args(args: &str) -> Option<Vec<&str>> {
    TopLevelArgSplitter::new(args).split()
}

struct TopLevelArgSplitter<'a> {
    args: &'a str,
    parts: Vec<&'a str>,
    start: usize,
    in_string: bool,
    quote: char,
    escaped: bool,
    paren_depth: usize,
}

impl<'a> TopLevelArgSplitter<'a> {
    fn new(args: &'a str) -> Self {
        Self {
            args,
            parts: Vec::new(),
            start: 0,
            in_string: false,
            quote: '\0',
            escaped: false,
            paren_depth: 0,
        }
    }

    fn split(mut self) -> Option<Vec<&'a str>> {
        for (idx, ch) in self.args.char_indices() {
            self.consume_char(idx, ch)?;
        }
        self.push_trailing_part()?;
        Some(self.parts)
    }

    fn consume_char(&mut self, idx: usize, ch: char) -> Option<()> {
        if self.consume_string_char(ch) {
            return Some(());
        }
        match ch {
            '"' | '\'' => self.start_string(ch),
            '(' => self.paren_depth += 1,
            ')' => self.paren_depth = self.paren_depth.saturating_sub(1),
            ',' if self.paren_depth == 0 => self.split_at_comma(idx, ch)?,
            _ => {}
        }
        Some(())
    }

    fn consume_string_char(&mut self, ch: char) -> bool {
        if !self.in_string {
            return false;
        }
        if self.escaped {
            self.escaped = false;
        } else if ch == '\\' {
            self.escaped = true;
        } else if ch == self.quote {
            self.in_string = false;
        }
        true
    }

    fn start_string(&mut self, quote: char) {
        self.in_string = true;
        self.quote = quote;
    }

    fn split_at_comma(&mut self, idx: usize, ch: char) -> Option<()> {
        self.push_part_until(idx)?;
        self.start = idx + ch.len_utf8();
        Some(())
    }

    fn push_trailing_part(&mut self) -> Option<()> {
        if self.in_string || self.paren_depth != 0 {
            return None;
        }
        let part = self.args[self.start..].trim();
        self.push_part(part)
    }

    fn push_part_until(&mut self, end: usize) -> Option<()> {
        let part = self.args[self.start..end].trim();
        self.push_part(part)
    }

    fn push_part(&mut self, part: &'a str) -> Option<()> {
        if part.is_empty() {
            return None;
        }
        self.parts.push(part);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::split_top_level_args;

    #[test]
    fn splits_simple_args() {
        assert_eq!(split_top_level_args("a, b, c"), Some(vec!["a", "b", "c"]));
    }

    #[test]
    fn keeps_commas_inside_strings() {
        assert_eq!(
            split_top_level_args(r#"self, "a,b", other"#),
            Some(vec!["self", r#""a,b""#, "other"])
        );
    }

    #[test]
    fn keeps_commas_inside_nested_calls() {
        assert_eq!(
            split_top_level_args("foo(a, b), bar"),
            Some(vec!["foo(a, b)", "bar"])
        );
    }

    #[test]
    fn rejects_empty_segments() {
        assert_eq!(split_top_level_args("a,, b"), None);
        assert_eq!(split_top_level_args("a,"), None);
    }

    #[test]
    fn rejects_unclosed_string_or_parens() {
        assert_eq!(split_top_level_args(r#"a, "b"#), None);
        assert_eq!(split_top_level_args("foo(a, b"), None);
    }
}
