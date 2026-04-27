//! Archaeology state for legacy (non-namespaced) globals consumed by
//! `Blizzard_ArchaeologyUI`.
//!
//! Backs the race-summary, active-artifact, keystone-socket,
//! completion-history, and `CloseResearch` surfaces. Each global module
//! function reads from this struct; tests seed fields directly.

use std::time::Instant;

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

/// Currently-selected artifact backing the active-artifact globals
/// (`GetSelectedArtifactInfo`, `GetArtifactProgress`, `CanSolveArtifact`,
/// `SolveArtifact`). Owned by `ArchaeologyState.selected` and replaced
/// by `SetSelectedArtifact`.
#[derive(Debug, Clone, Default)]
pub struct SelectedArtifact {
    /// 1-based race index this artifact belongs to.
    pub race_id: i32,
    /// 1-based artifact index inside the race when an already-completed
    /// artifact is being viewed (`SetSelectedArtifact(raceID, artifactID)`).
    /// `None` selects the race's currently-pending artifact.
    pub artifact_id: Option<i32>,
    pub name: String,
    pub description: String,
    pub rarity: i32,
    pub icon: u32,
    pub spell_description: String,
    pub num_sockets: i32,
    pub bg_texture: String,
    pub spell_id: u32,
    /// Per-socket keystone presence; `sockets[i]` is true when socket
    /// `i + 1` currently holds a keystone. Length is normalized to
    /// `num_sockets` by the keystone globals before each mutation.
    pub sockets: Vec<bool>,
    /// Fragment progress earned without keystone help.
    pub base_progress: i32,
    /// Fragment progress contributed by socketed keystones.
    pub adjust_progress: i32,
    /// Fragments required to solve this artifact.
    pub total_cost: i32,
    /// Whether the player can solve right now (base + adjust ≥ total_cost
    /// and the artifact is still pending). Authoritative; the simulator
    /// does not derive it so tests can drive any combination.
    pub can_solve: bool,
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
    /// Active artifact slot. `None` means no artifact is selected — the
    /// active-artifact globals return nil / 0 / false until
    /// `SetSelectedArtifact` runs.
    pub selected: Option<SelectedArtifact>,
    /// Fragment value contributed per socketed keystone. `SocketItemToArtifact`
    /// adds this to `selected.adjust_progress`; `RemoveItemFromArtifact`
    /// subtracts it. Tests set this directly; the simulator does not derive
    /// it from item data.
    pub keystone_value: i32,
    /// Whether the server-side completion-history payload has arrived.
    /// `IsArtifactCompletionHistoryAvailable` reads this; the completed-
    /// page paginator hides every row until it is true.
    /// `RequestArtifactCompletionHistory` flips it to true so addons that
    /// gate on the request can proceed.
    pub history_available: bool,
    /// Wall-clock `Instant` of the most recent `CloseResearch()` call.
    /// `ArchaeologyFrame_OnHide` and `ArchaeologyFrame_ShowFailed` both
    /// invoke `CloseResearch()`; recording the timestamp lets tests
    /// assert the call happened without exposing the no-op as a counter.
    pub last_close_request: Option<Instant>,
}

impl Default for ArchaeologyState {
    fn default() -> Self {
        Self {
            profession_name: "Archaeology".to_string(),
            races: Vec::new(),
            selected: None,
            keystone_value: 0,
            history_available: false,
            last_close_request: None,
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

    /// Returns the 1-based artifact entry inside the 1-based race,
    /// or `None` if either index is out of range. Drives
    /// `GetArtifactInfoByRace`, whose caller treats `None` as the
    /// signal to advance to the next race.
    pub fn artifact_at(&self, race_index: i32, project_index: i32) -> Option<&ArchaeologyArtifact> {
        let race = self.race_at(race_index)?;
        if project_index < 1 {
            return None;
        }
        let zero_based = (project_index - 1) as usize;
        race.artifacts.get(zero_based)
    }
}
