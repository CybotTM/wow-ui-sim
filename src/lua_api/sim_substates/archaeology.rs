//! Archaeology state for legacy (non-namespaced) globals consumed by
//! `Blizzard_ArchaeologyUI`.
//!
//! Drives `GetArchaeologyInfo`, `GetNumArchaeologyRaces`,
//! `GetArchaeologyRaceInfo`, and `GetNumArtifactsByRace`. The richer
//! active-artifact / keystone / completion-history surfaces hang off the
//! same struct in follow-up patches; this file currently only carries
//! the race-summary subset.

/// Single archaeology race entry surfaced by `GetArchaeologyRaceInfo`.
/// `texture` is a FileDataID-shaped number per Wowless's API yaml.
/// `currency_amount` is the count of fragments currently held;
/// `project_amount` is the per-project fragment cost.
#[derive(Debug, Clone)]
pub struct ArchaeologyRace {
    pub name: String,
    pub texture: u32,
    pub race_item_id: u32,
    pub currency_amount: i32,
    pub project_amount: i32,
    pub artifacts: Vec<ArchaeologyArtifact>,
}

/// Per-race artifact entry. Race-summary callers only need the
/// per-race count; the full schema is filled in once the active-artifact
/// and completion-history surfaces land.
#[derive(Debug, Clone, Default)]
pub struct ArchaeologyArtifact {
    pub name: String,
    pub description: String,
    pub rarity: i32,
    pub icon: u32,
    pub spell_description: String,
    pub spell_id: u32,
    pub first_completion_time: i64,
    pub completion_count: i32,
}

/// Backing state for the legacy archaeology globals.
///
/// `profession_name` is the localized "Archaeology" string returned by
/// `GetArchaeologyInfo`. The default uses the en-US literal because the
/// simulator does not localize secondary-profession names.
#[derive(Debug, Clone)]
pub struct ArchaeologyState {
    pub profession_name: String,
    pub races: Vec<ArchaeologyRace>,
}

impl Default for ArchaeologyState {
    fn default() -> Self {
        Self {
            profession_name: "Archaeology".to_string(),
            races: Vec::new(),
        }
    }
}

impl ArchaeologyState {
    /// Returns a 1-based race entry, or `None` if `race_index` is
    /// outside `1..=races.len()`. Callers map `None` to a Lua nil
    /// return.
    pub fn race_at(&self, race_index: i32) -> Option<&ArchaeologyRace> {
        if race_index < 1 {
            return None;
        }
        let zero_based = (race_index - 1) as usize;
        self.races.get(zero_based)
    }
}
