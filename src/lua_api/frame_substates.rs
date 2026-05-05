//! Per-frame SimState side-tables keyed by widget ID — split out of
//! `state.rs` so the main `SimState` struct stays the focus of that
//! file. Each map on `SimState` stores one of these structs per frame
//! of the relevant widget kind.

use std::collections::{HashMap, HashSet};

/// Active quest blob state for a QuestPOIFrame.
pub struct QuestBlobState {
    /// Map ID set via `SetMapID`.
    pub map_id: u32,
    /// Quest IDs currently drawn (via `DrawBlob`).
    pub active_quests: Vec<u32>,
    active_quest_ids: HashSet<u32>,
    /// Fill texture configured via `SetFillTexture`.
    pub fill_texture: Option<String>,
    /// Border texture configured via `SetBorderTexture`.
    pub border_texture: Option<String>,
    /// Fill alpha configured via `SetFillAlpha`.
    pub fill_alpha: Option<f64>,
    /// Border alpha configured via `SetBorderAlpha`.
    pub border_alpha: Option<f64>,
    /// Border scalar configured via `SetBorderScalar`.
    pub border_scalar: Option<f64>,
}

impl Default for QuestBlobState {
    fn default() -> Self {
        Self {
            map_id: 0,
            active_quests: Vec::new(),
            active_quest_ids: HashSet::new(),
            fill_texture: None,
            border_texture: None,
            fill_alpha: None,
            border_alpha: None,
            border_scalar: None,
        }
    }
}

impl QuestBlobState {
    pub fn insert_active_quest(&mut self, quest_id: u32) {
        if self.active_quest_ids.insert(quest_id) {
            self.active_quests.push(quest_id);
        }
    }

    pub fn clear_active_quests(&mut self) {
        self.active_quests.clear();
        self.active_quest_ids.clear();
    }
}

/// A unit pin stored by a UnitPositionFrame.
pub struct UnitPositionUnit {
    pub unit: String,
    pub asset: Option<String>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub color: Option<(f64, f64, f64, f64)>,
    pub sublevel: Option<i32>,
    pub show_facing: Option<bool>,
}

/// A player-ping texture configured on a UnitPositionFrame.
pub struct UnitPositionPlayerPingTexture {
    pub asset: Option<String>,
    pub width: f64,
    pub height: f64,
}

/// Runtime state for a FogOfWarFrame.
#[derive(Default)]
pub struct FogOfWarFrameState {
    pub ui_map_id: Option<i32>,
    pub background_atlas: Option<String>,
    pub mask_atlas: Option<String>,
    pub mask_scalar: Option<f64>,
}

/// Runtime state for a UnitPositionFrame.
pub struct UnitPositionFrameState {
    pub ui_map_id: Option<i32>,
    pub units: Vec<UnitPositionUnit>,
    pub unit_colors: HashMap<String, (f64, f64, f64, f64)>,
    pub mouse_over_units: Vec<String>,
    pub player_ping_scale: f64,
    pub player_ping_textures: HashMap<i32, UnitPositionPlayerPingTexture>,
    pub player_ping_active: bool,
    pub player_ping_duration: Option<f64>,
    pub player_ping_fade_duration: Option<f64>,
    pub is_finalized: bool,
}

/// Pending player report initiated through `C_ReportSystem`.
pub struct PendingPlayerReport {
    pub report_type: String,
    pub comment: Option<String>,
}
