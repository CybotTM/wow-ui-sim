pub(super) fn split_inline_sequence_parts(stmt: &str) -> Vec<&str> {
    InlineSequenceSplitter::new(stmt).split()
}

struct InlineSequenceSplitter<'a> {
    stmt: &'a str,
    parts: Vec<&'a str>,
    start: usize,
    in_string: bool,
    quote: char,
    escaped: bool,
    in_comment: bool,
    block_depth: usize,
    paren_depth: usize,
}

impl<'a> InlineSequenceSplitter<'a> {
    fn new(stmt: &'a str) -> Self {
        Self {
            stmt,
            parts: Vec::new(),
            start: 0,
            in_string: false,
            quote: '\0',
            escaped: false,
            in_comment: false,
            block_depth: 0,
            paren_depth: 0,
        }
    }

    fn split(mut self) -> Vec<&'a str> {
        let mut chars = self.stmt.char_indices().peekable();
        while let Some((idx, ch)) = chars.next() {
            self.consume_char(idx, ch, &mut chars);
        }
        self.push_trailing_part();
        self.parts
    }

    fn consume_char(
        &mut self,
        idx: usize,
        ch: char,
        chars: &mut std::iter::Peekable<std::str::CharIndices<'a>>,
    ) {
        if self.consume_comment_char(idx, ch)
            || self.consume_string_char(ch)
            || self.start_string(ch)
            || self.consume_grouping_or_newline(idx, ch)
            || self.consume_identifier(idx, ch, chars)
            || self.start_comment(idx, ch, chars)
        {
            return;
        }

        self.split_on_semicolon(idx, ch);
    }

    fn consume_comment_char(&mut self, idx: usize, ch: char) -> bool {
        if !self.in_comment {
            return false;
        }
        if ch == '\n' {
            self.in_comment = false;
            self.start = idx + ch.len_utf8();
        }
        true
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

    fn start_string(&mut self, ch: char) -> bool {
        if ch != '"' && ch != '\'' {
            return false;
        }
        self.in_string = true;
        self.quote = ch;
        true
    }

    fn consume_grouping_or_newline(&mut self, idx: usize, ch: char) -> bool {
        match ch {
            '(' => self.paren_depth += 1,
            ')' => self.paren_depth = self.paren_depth.saturating_sub(1),
            '\n' if self.is_top_level() => self.split_on_newline(idx, ch),
            _ => return false,
        }
        true
    }

    fn consume_identifier(
        &mut self,
        idx: usize,
        ch: char,
        chars: &mut std::iter::Peekable<std::str::CharIndices<'a>>,
    ) -> bool {
        if !is_identifier_start(ch) {
            return false;
        }
        let end = self.identifier_end(idx, ch, chars);
        self.apply_block_keyword(idx, end);
        true
    }

    fn identifier_end(
        &self,
        idx: usize,
        ch: char,
        chars: &mut std::iter::Peekable<std::str::CharIndices<'a>>,
    ) -> usize {
        let mut end = idx + ch.len_utf8();
        while let Some((next_idx, next_ch)) = chars.peek().copied() {
            if !is_identifier_continue(next_ch) {
                break;
            }
            end = next_idx + next_ch.len_utf8();
            let _ = chars.next();
        }
        end
    }

    fn apply_block_keyword(&mut self, idx: usize, end: usize) {
        match &self.stmt[idx..end] {
            "if" => self.block_depth += 1,
            "end" if self.block_depth > 0 => self.close_block(end),
            _ => {}
        }
    }

    fn close_block(&mut self, end: usize) {
        self.block_depth -= 1;
        if self.block_depth != 0 {
            return;
        }
        let rest = self.stmt[end..].trim_start();
        if !rest.is_empty() && !rest.starts_with(';') {
            self.push_part_until(end);
            self.start = end;
        }
    }

    fn start_comment(
        &mut self,
        idx: usize,
        ch: char,
        chars: &mut std::iter::Peekable<std::str::CharIndices<'a>>,
    ) -> bool {
        if ch != '-' || !matches!(chars.peek(), Some((_, '-'))) {
            return false;
        }
        self.push_part_until(idx);
        let _ = chars.next();
        self.in_comment = true;
        true
    }

    fn split_on_newline(&mut self, idx: usize, ch: char) {
        let part = self.stmt[self.start..idx].trim();
        let rest = self.stmt[idx + ch.len_utf8()..].trim_start();
        if !part.is_empty() && should_keep_local_prelude_with_following_block(part, rest) {
            return;
        }
        self.push_trimmed_part(part);
        self.start = idx + ch.len_utf8();
    }

    fn split_on_semicolon(&mut self, idx: usize, ch: char) {
        if ch != ';' || !self.is_top_level() {
            return;
        }
        self.push_part_until(idx);
        self.start = idx + ch.len_utf8();
    }

    fn push_trailing_part(&mut self) {
        if !self.in_comment {
            let part = self.stmt[self.start..].trim();
            self.push_trimmed_part(part);
        }
    }

    fn push_part_until(&mut self, end: usize) {
        let part = self.stmt[self.start..end].trim();
        self.push_trimmed_part(part);
    }

    fn push_trimmed_part(&mut self, part: &'a str) {
        if !part.is_empty() {
            self.parts.push(part);
        }
    }

    fn is_top_level(&self) -> bool {
        self.block_depth == 0 && self.paren_depth == 0
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_identifier_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
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

#[cfg(test)]
mod tests {
    use super::split_inline_sequence_parts;

    #[test]
    fn keeps_string_separators_inside_part() {
        let parts = split_inline_sequence_parts(r#"self:SetText("a;b"); self:Show()"#);
        assert_eq!(parts, vec![r#"self:SetText("a;b")"#, "self:Show()"]);
    }

    #[test]
    fn drops_line_comments() {
        let parts = split_inline_sequence_parts("self:Show() -- ignored\nself:Hide()");
        assert_eq!(parts, vec!["self:Show()", "self:Hide()"]);
    }

    #[test]
    fn keeps_local_prelude_with_following_if_block() {
        let parts = split_inline_sequence_parts(
            "local enabled = self:IsEnabled()\nif enabled then\nself:Show()\nend",
        );
        assert_eq!(
            parts,
            vec!["local enabled = self:IsEnabled()\nif enabled then\nself:Show()\nend"]
        );
    }

    #[test]
    fn splits_after_closed_if_without_semicolon() {
        let parts = split_inline_sequence_parts("if enabled then\nself:Show()\nend self:Hide()");
        assert_eq!(
            parts,
            vec!["if enabled then\nself:Show()\nend", "self:Hide()"]
        );
    }

    #[test]
    fn ignores_separators_inside_call_args() {
        let parts = split_inline_sequence_parts("self:SetPoint(foo;\nbar); self:Show()");
        assert_eq!(parts, vec!["self:SetPoint(foo;\nbar)", "self:Show()"]);
    }
}
