use super::set_global;
use crate::lua_api::methods::{create_string, table_set, table_set_num};
use rilua::Val;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;

pub(super) fn parse_saved_variables_file(state: &mut LuaState, source: &str) -> Result<(), String> {
    Parser::new(state, source).parse_file()
}

struct Parser<'a, 's> {
    state: &'a mut LuaState,
    bytes: &'s [u8],
    pos: usize,
}

impl<'a, 's> Parser<'a, 's> {
    fn new(state: &'a mut LuaState, source: &'s str) -> Self {
        Self {
            state,
            bytes: source.as_bytes(),
            pos: 0,
        }
    }

    fn parse_file(&mut self) -> Result<(), String> {
        self.skip_ws();
        while !self.is_eof() {
            let name = self.parse_identifier()?;
            self.skip_ws();
            self.expect_byte(b'=')?;
            let value = self.parse_value()?;
            set_global(self.state, &name, value);
            self.skip_field_separator();
            self.skip_ws();
        }
        Ok(())
    }

    fn parse_value(&mut self) -> Result<Val, String> {
        self.skip_ws();
        match self.peek_byte() {
            Some(b'{') => self.parse_table(),
            Some(b'"') | Some(b'\'') => {
                let value = self.parse_string()?;
                Ok(create_string(self.state, &value))
            }
            Some(b'-' | b'0'..=b'9') => self.parse_number().map(Val::Num),
            Some(b't') => {
                self.expect_keyword("true")?;
                Ok(Val::Bool(true))
            }
            Some(b'f') => {
                self.expect_keyword("false")?;
                Ok(Val::Bool(false))
            }
            Some(b'n') => {
                self.expect_keyword("nil")?;
                Ok(Val::Nil)
            }
            Some(_) => Err(self.error("unsupported SavedVariables value")),
            None => Err(self.error("unexpected end of file")),
        }
    }

    fn parse_table(&mut self) -> Result<Val, String> {
        self.expect_byte(b'{')?;
        let table = Val::Table(self.state.gc.alloc_table(Table::new()));
        let Val::Table(table_ref) = table else {
            unreachable!("fresh table is always a table")
        };
        let mut next_array_index = 1.0;

        loop {
            self.skip_ws();
            if self.consume_byte(b'}') {
                break;
            }

            if self.consume_byte(b'[') {
                let key = self.parse_key()?;
                self.skip_ws();
                self.expect_byte(b']')?;
                self.skip_ws();
                self.expect_byte(b'=')?;
                let value = self.parse_value()?;
                self.set_table_key(table, table_ref, key, value);
            } else if self.next_field_is_named_assignment() {
                let key = self.parse_identifier()?;
                self.skip_ws();
                self.expect_byte(b'=')?;
                let value = self.parse_value()?;
                table_set(self.state, table, &key, value);
            } else {
                let value = self.parse_value()?;
                table_set_num(self.state, table_ref, next_array_index, value);
                next_array_index += 1.0;
            }

            self.skip_field_separator();
        }

        Ok(table)
    }

    fn parse_key(&mut self) -> Result<SavedVarKey, String> {
        self.skip_ws();
        match self.peek_byte() {
            Some(b'"') | Some(b'\'') => self.parse_string().map(SavedVarKey::String),
            Some(b'-' | b'0'..=b'9') => self.parse_number().map(SavedVarKey::Number),
            _ => Err(self.error("unsupported table key")),
        }
    }

    fn set_table_key(
        &mut self,
        table: Val,
        table_ref: rilua::vm::gc::arena::GcRef<Table>,
        key: SavedVarKey,
        value: Val,
    ) {
        match key {
            SavedVarKey::String(key) => table_set(self.state, table, &key, value),
            SavedVarKey::Number(key) => table_set_num(self.state, table_ref, key, value),
        }
    }

    fn next_field_is_named_assignment(&self) -> bool {
        let mut pos = self.pos;
        if !is_ident_start(self.bytes.get(pos).copied()) {
            return false;
        }
        pos += 1;
        while is_ident_continue(self.bytes.get(pos).copied()) {
            pos += 1;
        }
        while matches!(self.bytes.get(pos), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            pos += 1;
        }
        self.bytes.get(pos) == Some(&b'=')
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        let start = self.pos;
        if !is_ident_start(self.peek_byte()) {
            return Err(self.error("expected identifier"));
        }
        self.pos += 1;
        while is_ident_continue(self.peek_byte()) {
            self.pos += 1;
        }
        std::str::from_utf8(&self.bytes[start..self.pos])
            .map(|s| s.to_string())
            .map_err(|_| self.error("identifier is not UTF-8"))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        let quote = self
            .next_byte()
            .ok_or_else(|| self.error("expected string"))?;
        let mut out = Vec::new();
        while let Some(byte) = self.next_byte() {
            if byte == quote {
                return String::from_utf8(out).map_err(|_| self.error("string is not UTF-8"));
            }
            if byte != b'\\' {
                out.push(byte);
                continue;
            }
            self.parse_escape(&mut out)?;
        }
        Err(self.error("unterminated string"))
    }

    fn parse_escape(&mut self, out: &mut Vec<u8>) -> Result<(), String> {
        let byte = self
            .next_byte()
            .ok_or_else(|| self.error("unterminated escape"))?;
        match byte {
            b'\\' => out.push(b'\\'),
            b'"' => out.push(b'"'),
            b'\'' => out.push(b'\''),
            b'n' => out.push(b'\n'),
            b'r' => out.push(b'\r'),
            b't' => out.push(b'\t'),
            b'0'..=b'9' => self.parse_decimal_escape(out, byte)?,
            other => out.push(other),
        }
        Ok(())
    }

    fn parse_decimal_escape(&mut self, out: &mut Vec<u8>, first: u8) -> Result<(), String> {
        let mut value = first - b'0';
        for _ in 0..2 {
            let Some(next) = self.peek_byte() else {
                break;
            };
            if !next.is_ascii_digit() {
                break;
            }
            value = value
                .checked_mul(10)
                .and_then(|value| value.checked_add(next - b'0'))
                .ok_or_else(|| self.error("invalid decimal escape"))?;
            self.pos += 1;
        }
        out.push(value);
        Ok(())
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.pos;
        self.consume_byte(b'-');
        self.consume_digits();
        if self.consume_byte(b'.') {
            self.consume_digits();
        }
        if self.consume_byte(b'e') || self.consume_byte(b'E') {
            let _ = self.consume_byte(b'-') || self.consume_byte(b'+');
            self.consume_digits();
        }
        let text = std::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|_| self.error("number is not UTF-8"))?;
        text.parse::<f64>()
            .map_err(|_| self.error("invalid number"))
    }

    fn consume_digits(&mut self) {
        while matches!(self.peek_byte(), Some(b'0'..=b'9')) {
            self.pos += 1;
        }
    }

    fn expect_keyword(&mut self, keyword: &str) -> Result<(), String> {
        if self.bytes[self.pos..].starts_with(keyword.as_bytes()) {
            self.pos += keyword.len();
            return Ok(());
        }
        Err(self.error(&format!("expected {keyword}")))
    }

    fn skip_ws(&mut self) {
        loop {
            while matches!(self.peek_byte(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
                self.pos += 1;
            }
            if self.bytes[self.pos..].starts_with(b"--") {
                while !matches!(self.peek_byte(), None | Some(b'\n')) {
                    self.pos += 1;
                }
                continue;
            }
            break;
        }
    }

    fn skip_field_separator(&mut self) {
        self.skip_ws();
        let _ = self.consume_byte(b',') || self.consume_byte(b';');
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), String> {
        if self.consume_byte(expected) {
            return Ok(());
        }
        Err(self.error(&format!("expected '{}'", expected as char)))
    }

    fn consume_byte(&mut self, expected: u8) -> bool {
        if self.peek_byte() == Some(expected) {
            self.pos += 1;
            return true;
        }
        false
    }

    fn next_byte(&mut self) -> Option<u8> {
        let byte = self.peek_byte()?;
        self.pos += 1;
        Some(byte)
    }

    fn peek_byte(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    fn error(&self, message: &str) -> String {
        format!("{message} at byte {}", self.pos)
    }
}

enum SavedVarKey {
    String(String),
    Number(f64),
}

fn is_ident_start(byte: Option<u8>) -> bool {
    matches!(byte, Some(b'a'..=b'z' | b'A'..=b'Z' | b'_'))
}

fn is_ident_continue(byte: Option<u8>) -> bool {
    matches!(byte, Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
}

#[cfg(test)]
mod tests {
    use super::super::get_global;
    use super::*;
    use crate::lua_api::WowLuaEnv;
    use rilua::LuaApiMut;

    #[test]
    fn parses_saved_variable_assignments_without_lua_compilation() {
        let env = WowLuaEnv::new().unwrap();
        let mut lua = env.rilua_mut();
        let state = lua.state_mut();

        parse_saved_variables_file(
            state,
            r#"
                TEST_SV = {
                    ["name"] = "hello\nworld",
                    count = 3,
                    true,
                    [42] = false,
                }
                "#,
        )
        .unwrap();

        assert!(matches!(get_global(state, "TEST_SV"), Val::Table(_)));
    }

    #[test]
    fn preserves_utf8_strings() {
        let env = WowLuaEnv::new().unwrap();
        {
            let mut lua = env.rilua_mut();
            parse_saved_variables_file(
                lua.state_mut(),
                r#"
                    TEST_SV = {
                        ["localized"] = "Café",
                    }
                    "#,
            )
            .unwrap();
        }

        let localized: String = env.eval("return TEST_SV.localized").unwrap();
        assert_eq!(localized, "Café");
    }
}
