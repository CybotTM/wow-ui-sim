//! Backing state for the `C_ArrowCalloutManager` namespace consumed by
//! `Blizzard_ArrowCalloutFrame/ArrowCalloutFrame.lua`.
//!
//! - `active` is the live set of callouts currently shown on screen,
//!   keyed by `calloutID`. The shape mirrors the `calloutInfo` table the
//!   Blizzard `Setup` handler reads at lua:99-114 (anchor frame name,
//!   direction, type, offset, text, optional widget set).
//! - `acknowledged` records every callout id the player has dismissed
//!   via the close button. The set is persisted into the
//!   `acknowledgedArrowCallouts` cvar so a reload preserves state.

use std::collections::{BTreeMap, BTreeSet};

/// `calloutInfo` record handed to `C_ArrowCalloutManager.ShowCallout`.
/// Field naming mirrors the Lua keys read by `ArrowCalloutMixin:Setup`
/// and `:AnchorCallout`. `ui_widget_set_id` is `None` unless the
/// callout type is `Enum.ArrowCalloutType.WidgetContainerNoBorder`.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ArrowCalloutInfo {
    pub callout_id: i64,
    pub callout_frame: String,
    pub callout_type: i32,
    pub callout_direction: i32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub callout_text: String,
    pub ui_widget_set_id: Option<u32>,
}

/// `C_ArrowCalloutManager` state. `active` is the live show/hide set,
/// `acknowledged` is the persistent dismissed set.
#[derive(Debug, Default, Clone)]
pub struct ArrowCalloutState {
    pub active: BTreeMap<i64, ArrowCalloutInfo>,
    pub acknowledged: BTreeSet<i64>,
}

impl ArrowCalloutState {
    pub fn sync_acknowledged_cvar_value(&mut self, value: &str) {
        self.acknowledged = parse_acknowledged_callout_ids(value);
    }

    /// Comma-separated list of acknowledged ids in ascending order, used
    /// to round-trip the `acknowledgedArrowCallouts` cvar. An empty set
    /// renders as `"0"` to match the cvar default seeded in
    /// `src/cvars.yaml:2`.
    pub fn acknowledged_cvar_value(&self) -> String {
        if self.acknowledged.is_empty() {
            return "0".to_string();
        }
        self.acknowledged
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn parse_acknowledged_callout_ids(value: &str) -> BTreeSet<i64> {
    value
        .split(',')
        .filter_map(|raw_id| {
            let id = raw_id.trim();
            if id.is_empty() || id == "0" {
                return None;
            }
            id.parse::<i64>().ok()
        })
        .collect()
}
