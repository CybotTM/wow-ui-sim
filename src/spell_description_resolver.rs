//! Resolve Blizzard spell-description tokens into simulator tooltip text.

use crate::lua_api::game_data;
use crate::lua_api::state::SimState;

pub fn resolve_spell_description(sim: &SimState, spell_id: u32) -> String {
    let Some(raw) = crate::spell_descriptions::get_spell_description(spell_id) else {
        return "No description available.".to_string();
    };
    resolve_text(sim, spell_id, raw, 0)
}

pub fn resolve_spell_description_or_empty(sim: &SimState, spell_id: u32) -> String {
    let Some(raw) = crate::spell_descriptions::get_spell_description(spell_id) else {
        return String::new();
    };
    resolve_text(sim, spell_id, raw, 0)
}

fn resolve_text(sim: &SimState, spell_id: u32, text: &str, depth: usize) -> String {
    if depth > 4 {
        return cleanup_control_tokens(text);
    }

    let expanded = expand_named_references(sim, text, depth);
    let expanded = replace_expressions(sim, spell_id, &expanded);
    let expanded = replace_angle_tokens(sim, spell_id, &expanded);
    let expanded = replace_dollar_tokens(sim, spell_id, &expanded);
    cleanup_control_tokens(&expanded)
}

fn expand_named_references(sim: &SimState, text: &str, depth: usize) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(index) = rest.find("$@") {
        out.push_str(&rest[..index]);
        let token_start = index + 2;
        let token = &rest[token_start..];
        if let Some(id_text) = token.strip_prefix("spellname") {
            let (id, consumed) = parse_number_prefix(id_text);
            if let Some(name) = id.and_then(|id| crate::spells::get_spell(id).map(|s| s.name)) {
                out.push_str(name);
            }
            rest = &token["spellname".len() + consumed..];
        } else if let Some(id_text) = token.strip_prefix("spelldesc") {
            let (id, consumed) = parse_number_prefix(id_text);
            if let Some(id) = id {
                out.push_str(&resolve_spell_description(sim, id));
            }
            rest = &token["spelldesc".len() + consumed..];
        } else {
            out.push_str("$@");
            rest = token;
        }
    }
    out.push_str(rest);
    if depth == 0 {
        out
    } else {
        resolve_text(sim, 0, &out, depth + 1)
    }
}

fn replace_expressions(sim: &SimState, spell_id: u32, text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let expression_start = start + 2;
        let Some(end) = rest[expression_start..].find('}') else {
            out.push_str(&rest[start..]);
            return out;
        };
        let expression = &rest[expression_start..expression_start + end];
        let numeric = replace_expression_variables(sim, spell_id, expression);
        match evaluate_number_expression(&numeric) {
            Some(value) => out.push_str(&format_number(value)),
            None => out.push_str(&cleanup_control_tokens(expression)),
        }
        rest = &rest[expression_start + end + 1..];
    }
    out.push_str(rest);
    out
}

fn replace_expression_variables(sim: &SimState, spell_id: u32, expression: &str) -> String {
    let mut out = String::with_capacity(expression.len());
    let mut chars = expression.char_indices().peekable();
    while let Some((index, ch)) = chars.next() {
        if ch != '$' {
            out.push(ch);
            continue;
        }

        let tail = &expression[index + 1..];
        if let Some((token, consumed)) = parse_value_token(sim, spell_id, tail) {
            out.push_str(&token.to_string());
            while chars.peek().is_some_and(|(i, _)| *i < index + 1 + consumed) {
                chars.next();
            }
        } else {
            out.push('0');
        }
    }
    out
}

fn replace_angle_tokens(sim: &SimState, spell_id: u32, text: &str) -> String {
    let mut out = text.replace("$<damage>", &spell_amount(sim, spell_id, 1).to_string());
    out = out.replace(
        "$<damageValue>",
        &spell_amount(sim, spell_id, 1).to_string(),
    );
    out.replace("$<shield>", &shield_amount(sim, spell_id).to_string())
}

fn replace_dollar_tokens(sim: &SimState, spell_id: u32, text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.char_indices().peekable();
    while let Some((index, ch)) = chars.next() {
        if ch != '$' {
            out.push(ch);
            continue;
        }

        let tail = &text[index + 1..];
        if tail.starts_with('?') {
            out.push_str("$?");
            chars.next();
            continue;
        }
        if let Some((value, consumed)) = parse_value_token(sim, spell_id, tail) {
            out.push_str(&format_number(value));
            while chars.peek().is_some_and(|(i, _)| *i < index + 1 + consumed) {
                chars.next();
            }
            continue;
        }

        out.push('$');
    }
    out
}

fn parse_value_token(sim: &SimState, current_spell_id: u32, token: &str) -> Option<(f64, usize)> {
    if token.starts_with("STR") {
        return Some((sim.player.stats.strength, 3));
    }
    if token.starts_with("INT") {
        return Some((sim.player.stats.intellect, 3));
    }
    if token.starts_with("AP") {
        return Some((player_attack_power(sim), 2));
    }

    let (referenced_id, digits) = parse_number_prefix(token);
    let spell_id = referenced_id.unwrap_or(current_spell_id);
    let suffix = &token[digits..];
    let Some(kind) = suffix.chars().next() else {
        return None;
    };
    if !matches!(kind, 's' | 'm' | 'x' | 'd' | 't' | 'A' | 'u' | 'h') {
        return None;
    }

    let (effect_index, effect_digits) = parse_number_prefix(&suffix[kind.len_utf8()..]);
    let effect_index = effect_index.unwrap_or(1);
    let consumed = digits + kind.len_utf8() + effect_digits;
    let value = match kind {
        'd' => spell_duration(spell_id),
        'A' => spell_radius(spell_id, effect_index),
        'x' => spell_count(spell_id, effect_index),
        't' => spell_tick_time(spell_id, effect_index),
        'u' => spell_stack_count(spell_id),
        'h' => spell_proc_chance(spell_id),
        _ => spell_amount(sim, spell_id, effect_index),
    };
    Some((value, consumed))
}

fn parse_number_prefix(text: &str) -> (Option<u32>, usize) {
    let count = text
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .map(char::len_utf8)
        .sum();
    if count == 0 {
        return (None, 0);
    }
    (text[..count].parse().ok(), count)
}

fn spell_amount(sim: &SimState, spell_id: u32, effect_index: u32) -> f64 {
    match (spell_id, effect_index) {
        (184662, _) => shield_amount(sim, spell_id) as f64,
        (633, 2) => 100.0,
        (31935, 1) => 25_000.0,
        (35395, _) => 15_000.0,
        (53600, 1) => 20_000.0,
        (19750, _) | (85673, _) | (130551, _) => 20_000.0,
        (82326, _) => 35_000.0,
        (25912, _) | (25914, _) | (20473, _) => 10_000.0,
        (132403, 1) => 100.0,
        _ => game_data::spell_effect_amount(spell_id) as f64,
    }
}

fn shield_amount(sim: &SimState, spell_id: u32) -> i32 {
    match spell_id {
        184662 => (sim.player.health_max as f64 * 0.30).round() as i32,
        _ => game_data::spell_effect_amount(spell_id),
    }
}

fn player_attack_power(sim: &SimState) -> f64 {
    sim.player.stats.strength + sim.player.stats.agility + sim.player.level as f64 * 10.0
}

fn spell_duration(spell_id: u32) -> f64 {
    match spell_id {
        31935 => 3.0,
        184662 => 15.0,
        _ => 0.0,
    }
}

fn spell_radius(_spell_id: u32, _effect_index: u32) -> f64 {
    0.0
}

fn spell_count(spell_id: u32, effect_index: u32) -> f64 {
    match (spell_id, effect_index) {
        (31935, 1) => 3.0,
        _ => spell_amount_for_count(spell_id),
    }
}

fn spell_amount_for_count(_spell_id: u32) -> f64 {
    1.0
}

fn spell_tick_time(_spell_id: u32, _effect_index: u32) -> f64 {
    1.0
}

fn spell_stack_count(_spell_id: u32) -> f64 {
    1.0
}

fn spell_proc_chance(_spell_id: u32) -> f64 {
    0.0
}

fn evaluate_number_expression(expression: &str) -> Option<f64> {
    let tokens = tokenize_expression(expression)?;
    let mut parser = NumberExpressionParser { tokens, index: 0 };
    let value = parser.parse_sum()?;
    (parser.index == parser.tokens.len()).then_some(value)
}

#[derive(Clone, Copy)]
enum NumberToken {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
}

fn tokenize_expression(expression: &str) -> Option<Vec<NumberToken>> {
    let mut tokens = Vec::new();
    let mut index = 0;
    let bytes = expression.as_bytes();
    while index < bytes.len() {
        let ch = bytes[index] as char;
        match ch {
            ' ' => index += 1,
            '+' => {
                tokens.push(NumberToken::Plus);
                index += 1;
            }
            '-' => {
                tokens.push(NumberToken::Minus);
                index += 1;
            }
            '*' => {
                tokens.push(NumberToken::Star);
                index += 1;
            }
            '/' => {
                tokens.push(NumberToken::Slash);
                index += 1;
            }
            '0'..='9' | '.' => {
                let start = index;
                index += 1;
                while index < bytes.len()
                    && ((bytes[index] as char).is_ascii_digit() || bytes[index] == b'.')
                {
                    index += 1;
                }
                tokens.push(NumberToken::Number(expression[start..index].parse().ok()?));
            }
            _ => return None,
        }
    }
    Some(tokens)
}

struct NumberExpressionParser {
    tokens: Vec<NumberToken>,
    index: usize,
}

impl NumberExpressionParser {
    fn parse_sum(&mut self) -> Option<f64> {
        let mut value = self.parse_product()?;
        while let Some(token) = self.current() {
            match token {
                NumberToken::Plus => {
                    self.index += 1;
                    value += self.parse_product()?;
                }
                NumberToken::Minus => {
                    self.index += 1;
                    value -= self.parse_product()?;
                }
                _ => break,
            }
        }
        Some(value)
    }

    fn parse_product(&mut self) -> Option<f64> {
        let mut value = self.parse_factor()?;
        while let Some(token) = self.current() {
            match token {
                NumberToken::Star => {
                    self.index += 1;
                    value *= self.parse_factor()?;
                }
                NumberToken::Slash => {
                    self.index += 1;
                    let denominator = self.parse_factor()?;
                    if denominator == 0.0 {
                        return None;
                    }
                    value /= denominator;
                }
                _ => break,
            }
        }
        Some(value)
    }

    fn parse_factor(&mut self) -> Option<f64> {
        match self.current()? {
            NumberToken::Number(value) => {
                self.index += 1;
                Some(value)
            }
            NumberToken::Minus => {
                self.index += 1;
                self.parse_factor().map(|value| -value)
            }
            _ => None,
        }
    }

    fn current(&self) -> Option<NumberToken> {
        self.tokens.get(self.index).copied()
    }
}

fn cleanup_control_tokens(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' && chars.peek().is_some_and(|next| *next == '?') {
            chars.next();
            while chars.peek().is_some_and(|next| *next != '[') {
                chars.next();
            }
            if chars.peek().is_some_and(|next| *next == '[') {
                chars.next();
            }
            continue;
        }
        if ch == '$' {
            continue;
        }
        if ch != '[' && ch != ']' {
            out.push(ch);
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn format_number(value: f64) -> String {
    if (value.fract()).abs() < 0.001 {
        (value.round() as i64).to_string()
    } else {
        format!("{value:.1}")
    }
}
