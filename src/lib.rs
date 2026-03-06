//! WoW UI Simulator
//!
//! A standalone environment for testing World of Warcraft addons outside the game.
//! Embeds Lua 5.1 and implements the WoW widget API.

#[path = "../data/atlas.rs"]
mod atlas_data;
#[path = "../data/atlas_elements.rs"]
mod atlas_elements;
pub mod atlas;
#[path = "../data/global_strings.rs"]
pub mod global_strings;
#[path = "../data/items.rs"]
pub mod items;
#[path = "../data/manifest_interface_data.rs"]
pub mod manifest_interface_data;
#[path = "../data/specializations.rs"]
pub mod specializations;
#[path = "../data/spec_display_spells.rs"]
pub mod spec_display_spells;
#[path = "../data/spells.rs"]
pub mod spells;
#[path = "../data/spell_power.rs"]
pub mod spell_power;
#[path = "../data/traits.rs"]
pub mod traits;
#[path = "../data/zones.rs"]
pub mod zones;
pub mod config;
pub mod cvars;
pub mod dump;
#[cfg(feature = "gui")]
pub mod dump_texture;
pub mod error;
pub mod event;
pub mod extract_textures;
pub mod iced_app;
pub mod loader;
pub mod lua_api;
pub mod debug_helpers;
pub mod lua_errors;
pub mod self_test;
pub mod addon_tests;
pub mod lua_server;
pub mod render;
pub mod saved_variables;
pub mod sound;
pub mod stack;
pub mod startup;
pub mod texture;
pub mod toc;
pub mod widget;
pub mod xml;

pub use error::{Error, Result};
#[cfg(feature = "gui")]
pub use iced_app::{run_iced_ui, run_iced_ui_with_textures, DebugOptions};

/// Blend mode for quad rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum BlendMode {
    /// Standard alpha blending: src * alpha + dst * (1 - alpha)
    #[default]
    Alpha = 0,
    /// Additive blending: src + dst (for highlight textures)
    Additive = 1,
}

/// Computed layout position for a frame.
#[derive(Debug, Clone, Copy, Default)]
pub struct LayoutRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
