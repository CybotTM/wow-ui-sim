//! Nine-slice quad rendering for panel borders and frames.

use super::quad::{BlendMode, QuadBatch};
use iced::Rectangle;

/// Texture indices for 9-slice rendering.
#[derive(Debug, Clone, Copy, Default)]
pub struct NineSliceTextures {
    pub top_left: Option<i32>,
    pub top: Option<i32>,
    pub top_right: Option<i32>,
    pub left: Option<i32>,
    pub center: Option<i32>,
    pub right: Option<i32>,
    pub bottom_left: Option<i32>,
    pub bottom: Option<i32>,
    pub bottom_right: Option<i32>,
}

impl QuadBatch {
    /// Push a 9-slice texture (corners fixed, edges stretched, center stretched).
    pub fn push_nine_slice(
        &mut self,
        bounds: Rectangle,
        corner_size: f32,
        edge_size: f32,
        textures: &NineSliceTextures,
        color: [f32; 4],
    ) {
        if bounds.width < corner_size * 2.0 || bounds.height < corner_size * 2.0 {
            if let Some(center) = textures.center {
                self.push_textured(bounds, center, color, BlendMode::Alpha);
            }
            return;
        }

        let inner_width = bounds.width - corner_size * 2.0;
        let inner_height = bounds.height - corner_size * 2.0;
        let full_uv = Rectangle::new(iced::Point::ORIGIN, iced::Size::new(1.0, 1.0));

        push_center(self, bounds, edge_size, textures, color, full_uv);
        push_corners(self, bounds, corner_size, textures, color, full_uv);
        push_edges(
            self,
            NineSliceEdgeStrip {
                bounds,
                corner_size,
                edge_size,
                inner_width,
                inner_height,
                textures,
                color,
                full_uv,
            },
        );
    }
}

fn push_center(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    edge_size: f32,
    textures: &NineSliceTextures,
    color: [f32; 4],
    full_uv: Rectangle,
) {
    if let Some(tex) = textures.center {
        let center_bounds = Rectangle::new(
            iced::Point::new(bounds.x + edge_size, bounds.y + edge_size),
            iced::Size::new(
                bounds.width - edge_size * 2.0,
                bounds.height - edge_size * 2.0,
            ),
        );
        batch.push_quad(center_bounds, full_uv, color, tex, BlendMode::Alpha);
    }
}

fn push_corners(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    corner_size: f32,
    textures: &NineSliceTextures,
    color: [f32; 4],
    full_uv: Rectangle,
) {
    if let Some(tex) = textures.top_left {
        let corner = Rectangle::new(
            iced::Point::new(bounds.x, bounds.y),
            iced::Size::new(corner_size, corner_size),
        );
        batch.push_quad(corner, full_uv, color, tex, BlendMode::Alpha);
    }
    if let Some(tex) = textures.top_right {
        let corner = Rectangle::new(
            iced::Point::new(bounds.x + bounds.width - corner_size, bounds.y),
            iced::Size::new(corner_size, corner_size),
        );
        batch.push_quad(corner, full_uv, color, tex, BlendMode::Alpha);
    }
    if let Some(tex) = textures.bottom_left {
        let corner = Rectangle::new(
            iced::Point::new(bounds.x, bounds.y + bounds.height - corner_size),
            iced::Size::new(corner_size, corner_size),
        );
        batch.push_quad(corner, full_uv, color, tex, BlendMode::Alpha);
    }
    if let Some(tex) = textures.bottom_right {
        let corner = Rectangle::new(
            iced::Point::new(
                bounds.x + bounds.width - corner_size,
                bounds.y + bounds.height - corner_size,
            ),
            iced::Size::new(corner_size, corner_size),
        );
        batch.push_quad(corner, full_uv, color, tex, BlendMode::Alpha);
    }
}

fn push_edges(batch: &mut QuadBatch, strip: NineSliceEdgeStrip<'_>) {
    if let Some(tex) = strip.textures.top {
        let edge = Rectangle::new(
            iced::Point::new(strip.bounds.x + strip.corner_size, strip.bounds.y),
            iced::Size::new(strip.inner_width, strip.edge_size),
        );
        batch.push_quad(edge, strip.full_uv, strip.color, tex, BlendMode::Alpha);
    }
    if let Some(tex) = strip.textures.bottom {
        let edge = Rectangle::new(
            iced::Point::new(
                strip.bounds.x + strip.corner_size,
                strip.bounds.y + strip.bounds.height - strip.edge_size,
            ),
            iced::Size::new(strip.inner_width, strip.edge_size),
        );
        batch.push_quad(edge, strip.full_uv, strip.color, tex, BlendMode::Alpha);
    }
    if let Some(tex) = strip.textures.left {
        let edge = Rectangle::new(
            iced::Point::new(strip.bounds.x, strip.bounds.y + strip.corner_size),
            iced::Size::new(strip.edge_size, strip.inner_height),
        );
        batch.push_quad(edge, strip.full_uv, strip.color, tex, BlendMode::Alpha);
    }
    if let Some(tex) = strip.textures.right {
        let edge = Rectangle::new(
            iced::Point::new(
                strip.bounds.x + strip.bounds.width - strip.edge_size,
                strip.bounds.y + strip.corner_size,
            ),
            iced::Size::new(strip.edge_size, strip.inner_height),
        );
        batch.push_quad(edge, strip.full_uv, strip.color, tex, BlendMode::Alpha);
    }
}

struct NineSliceEdgeStrip<'a> {
    bounds: Rectangle,
    corner_size: f32,
    edge_size: f32,
    inner_width: f32,
    inner_height: f32,
    textures: &'a NineSliceTextures,
    color: [f32; 4],
    full_uv: Rectangle,
}
