//! One layout shared by content height, glyph emission and anchor hit testing.
use super::{
    TextStyle,
    document::{Block, parse_document},
};
use crate::render::font::{WowFontSystem, line_height_for_font_size};
use crate::widget::{Frame, TextJustify};

pub struct LinkRegion {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub target: String,
    pub label: String,
}

impl LinkRegion {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

pub struct LaidOutBlock {
    pub block: Block,
    pub style: TextStyle,
    pub y: f32,
    pub height: f32,
}

pub struct DocumentLayout {
    pub blocks: Vec<LaidOutBlock>,
    pub links: Vec<LinkRegion>,
    pub height: f32,
}

pub fn block_style(frame: &Frame, role: &str) -> TextStyle {
    if let Some(style) = frame
        .simple_html
        .as_ref()
        .and_then(|data| data.text_styles.get(role))
    {
        return style.clone();
    }
    TextStyle {
        font: frame.font.clone(),
        font_size: frame.font_size,
        text_color: (
            frame.text_color.r,
            frame.text_color.g,
            frame.text_color.b,
            frame.text_color.a,
        ),
        spacing: frame.text_line_spacing,
        justify_h: frame.justify_h.as_h_str().to_string(),
        ..TextStyle::default()
    }
}

pub fn layout_document(frame: &Frame, width: f32, fonts: &mut WowFontSystem) -> DocumentLayout {
    let mut layout = DocumentLayout {
        blocks: Vec::new(),
        links: Vec::new(),
        height: 0.0,
    };
    for block in parse_document(frame.text.as_deref().unwrap_or_default()) {
        let style = block_style(frame, &block.role);
        let (height, links) = shape_block(&block, &style, width, fonts);
        for mut link in links {
            link.y += layout.height;
            layout.links.push(link);
        }
        layout.blocks.push(LaidOutBlock {
            block,
            style,
            y: layout.height,
            height,
        });
        layout.height += height;
    }
    layout
}

fn shape_block(
    block: &Block,
    style: &TextStyle,
    width: f32,
    fonts: &mut WowFontSystem,
) -> (f32, Vec<LinkRegion>) {
    let Some(line_height) = line_height_for_font_size(style.font_size, false) else {
        return (0.0, Vec::new());
    };
    let attrs = fonts.attrs_owned(style.font.as_deref());
    let metrics = cosmic_text::Metrics::new(style.font_size, line_height);
    let mut buffer = cosmic_text::Buffer::new(&mut fonts.font_system, metrics);
    buffer.set_size(&mut fonts.font_system, Some(width.max(1.0)), None);
    buffer.set_text(
        &mut fonts.font_system,
        &block.text,
        &attrs.as_attrs(),
        cosmic_text::Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(&mut fonts.font_system, true);
    let mut links = Vec::new();
    let mut height = 0.0;
    for (row, run) in buffer.layout_runs().enumerate() {
        let y = row as f32 * (line_height + style.spacing);
        height = y + line_height;
        append_link_regions(&mut links, block, style, &run, (width, y, line_height));
    }
    (height, links)
}

fn append_link_regions(
    links: &mut Vec<LinkRegion>,
    block: &Block,
    style: &TextStyle,
    run: &cosmic_text::LayoutRun<'_>,
    geometry: (f32, f32, f32),
) {
    let (width, y, line_height) = geometry;
    let line_offset: usize = block
        .text
        .split_inclusive('\n')
        .take(run.line_i)
        .map(str::len)
        .sum();
    let x_offset = match TextJustify::from_wow_str(&style.justify_h) {
        TextJustify::Center => (width - run.line_w) / 2.0,
        TextJustify::Right => width - run.line_w,
        _ => 0.0,
    };
    for glyph in run.glyphs {
        let start = line_offset + glyph.start;
        let end = line_offset + glyph.end;
        let Some(anchor) = block
            .anchors
            .iter()
            .find(|anchor| anchor.range.start < end && start < anchor.range.end)
        else {
            continue;
        };
        links.push(LinkRegion {
            x: x_offset + glyph.x,
            y,
            width: glyph.w,
            height: line_height,
            target: anchor.target.clone(),
            label: anchor.label.clone(),
        });
    }
}
