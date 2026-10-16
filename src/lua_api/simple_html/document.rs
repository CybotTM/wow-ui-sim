//! Bounded p/h1/h2/h3/a tokenizer. Not an HTML browser or validator.
use regex::Regex;
use std::ops::Range;
use std::sync::LazyLock;

static TOKENS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<[^>]*>|[^<]+").unwrap());
static HREF: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)\bhref\s*=\s*(?:"([^"]*)"|'([^']*)')"#).unwrap());

#[derive(Debug)]
pub struct Anchor {
    pub range: Range<usize>,
    pub target: String,
    pub label: String,
}

#[derive(Debug)]
pub struct Block {
    pub role: String,
    pub text: String,
    pub anchors: Vec<Anchor>,
}

impl Block {
    fn new(role: &str) -> Self {
        Self {
            role: role.into(),
            text: String::new(),
            anchors: Vec::new(),
        }
    }
}

struct DocumentParser {
    blocks: Vec<Block>,
    current: Block,
    anchor: Option<(usize, String)>,
}

impl DocumentParser {
    fn finish_anchor(&mut self) {
        let Some((start, target)) = self.anchor.take() else {
            return;
        };
        let end = self.current.text.len();
        self.current.anchors.push(Anchor {
            range: start..end,
            target,
            label: self.current.text[start..end].to_string(),
        });
    }

    fn finish_block(&mut self, next_role: &str) {
        self.finish_anchor();
        let block = std::mem::replace(&mut self.current, Block::new(next_role));
        if !block.text.trim().is_empty() {
            self.blocks.push(block);
        }
    }

    fn read_tag(&mut self, token: &str) {
        let tag = token.trim_start_matches('<').trim_end_matches('>').trim();
        let name = tag
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        match name.as_str() {
            "p" | "h1" | "h2" | "h3" => self.finish_block(&name),
            "/p" | "/h1" | "/h2" | "/h3" => self.finish_block("p"),
            "br" | "br/" => self.current.text.push('\n'),
            "a" => {
                self.finish_anchor();
                if let Some(capture) = HREF.captures(tag) {
                    let value = capture.get(1).or_else(|| capture.get(2)).unwrap().as_str();
                    self.anchor = Some((self.current.text.len(), decode_entities(value)));
                }
            }
            "/a" => self.finish_anchor(),
            _ => {} // No styling, fetching or execution semantics for other tags.
        }
    }
}

fn decode_entities(text: &str) -> String {
    // Only well-formed entity tokens are decoded; unknown entities remain literal.
    static ENTITIES: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"&(?:#x[0-9a-fA-F]+|#[0-9]+|amp|lt|gt|quot|apos|nbsp);").unwrap()
    });
    ENTITIES
        .replace_all(text, |capture: &regex::Captures<'_>| {
            if &capture[0] == "&nbsp;" {
                return "\u{a0}".to_string();
            }
            match quick_xml::escape::unescape(&capture[0]) {
                Ok(decoded) => decoded.into_owned(),
                Err(_) => capture[0].to_string(),
            }
        })
        .into_owned()
}

pub fn parse_document(source: &str) -> Vec<Block> {
    let mut parser = DocumentParser {
        blocks: Vec::new(),
        current: Block::new("p"),
        anchor: None,
    };
    for token in TOKENS.find_iter(source) {
        let token = token.as_str();
        if token.starts_with('<') {
            parser.read_tag(token);
        } else {
            let text = crate::render::strip_wow_markup(&decode_entities(token));
            // Ignore formatting whitespace between top-level blocks, not content whitespace.
            if !parser.current.text.is_empty() || !text.trim().is_empty() {
                parser.current.text.push_str(&text);
            }
        }
    }
    parser.finish_block("p");
    parser.blocks
}
