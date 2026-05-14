//! Runtime / profiler types: cursor state, per-addon metrics, error records.

use std::collections::{HashMap, VecDeque};

/// What is currently held on the cursor (drag-and-drop state).
#[derive(Debug, Clone)]
pub enum CursorInfo {
    /// An action bar spell: PickupAction(slot) removes it from the bar.
    Action { slot: u32, spell_id: u32 },
    /// A spell from the spellbook (doesn't remove from spellbook).
    Spell { spell_id: u32 },
    /// A talent picked from the talent frame. `pvp=true` when sourced
    /// from the PvP talent pane.
    Talent { talent_id: u32, pvp: bool },
    /// A pet-action spell picked from the pet action bar.
    PetAction { slot: u32, spell_id: u32 },
    /// A macro picked up by slot index.
    Macro { macro_index: u32 },
    /// An item picked up from a bag slot, equipment slot, or merchant.
    Item {
        item_id: u32,
        stack_count: i32,
        origin: CursorItemOrigin,
    },
    /// Money in copper held on the cursor (PickupPlayerMoney → DropCursorMoney).
    Money { copper: u64 },
}

/// Where a cursor-carried item came from — used to route drops back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorItemOrigin {
    Bag { bag: i32, slot: i32 },
    Equipped { slot: i32 },
    Merchant { index: u32 },
    Unknown,
}

/// Per-addon runtime profiler metrics, updated each frame.
#[derive(Debug, Clone)]
pub struct AddonRuntimeMetrics {
    /// Time spent in this addon's handlers during the current frame (accumulator).
    pub current_frame_ms: f64,
    /// Rolling window of per-frame times (last 60 frames) for RecentAverageTime.
    pub recent_frames: VecDeque<f64>,
    /// Peak time ever recorded in a single frame.
    pub peak_ms: f64,
    /// Session total time (ms) across all frames.
    pub session_total_ms: f64,
    /// Number of frames where this addon had handlers fire.
    pub session_frame_count: u64,
    /// Threshold counters: frames where addon time exceeded N ms.
    pub count_over_1ms: u32,
    pub count_over_5ms: u32,
    pub count_over_10ms: u32,
    pub count_over_50ms: u32,
    pub count_over_100ms: u32,
    pub count_over_500ms: u32,
    pub count_over_1000ms: u32,
}

impl Default for AddonRuntimeMetrics {
    fn default() -> Self {
        Self {
            current_frame_ms: 0.0,
            recent_frames: VecDeque::with_capacity(60),
            peak_ms: 0.0,
            session_total_ms: 0.0,
            session_frame_count: 0,
            count_over_1ms: 0,
            count_over_5ms: 0,
            count_over_10ms: 0,
            count_over_50ms: 0,
            count_over_100ms: 0,
            count_over_500ms: 0,
            count_over_1000ms: 0,
        }
    }
}

/// Application-level frame timing for profiler (total frame time, not just addon time).
#[derive(Debug, Clone, Default)]
pub struct AppFrameMetrics {
    /// Rolling window of total frame times in ms (last 60 frames).
    pub recent_frame_ms: VecDeque<f64>,
    /// Peak frame time ever recorded.
    pub peak_ms: f64,
    /// Session total frame time in ms.
    pub session_total_ms: f64,
    /// Number of frames recorded.
    pub session_frame_count: u64,
}

/// Information about a loaded addon.
#[derive(Debug, Clone)]
pub struct AddonInfo {
    /// Folder name (used as addon identifier).
    pub folder_name: String,
    /// Display title from TOC metadata.
    pub title: String,
    /// Notes/description from TOC metadata.
    pub notes: String,
    /// Whether the addon is currently enabled.
    pub enabled: bool,
    /// Whether the addon was successfully loaded.
    pub loaded: bool,
    /// Load on demand flag.
    pub load_on_demand: bool,
    /// Whether the addon loads Lua/XML chunks in the secure environment.
    pub use_secure_env: bool,
    /// Optional security status reported by `C_AddOns.GetAddOnSecurity`.
    pub security: Option<String>,
    /// Total load time in seconds (for profiler metrics).
    pub load_time_secs: f64,
    /// Runtime profiler metrics (updated per frame).
    pub runtime: AddonRuntimeMetrics,
    /// Required dependencies declared in TOC (`Dependencies` / `RequiredDep` / `RequiredDeps`).
    /// Surfaced to Lua via `C_AddOns.GetAddOnDependencies` as a variadic of strings.
    pub dependencies: Vec<String>,
    /// Raw TOC metadata exposed through `C_AddOns.GetAddOnMetadata`.
    pub metadata: HashMap<String, String>,
    /// Factory default enabled state, derived from `## DefaultState: disabled`
    /// (`false` only when the TOC opts out; otherwise `true`). Surfaced via
    /// `C_AddOns.IsAddOnDefaultEnabled` and used by the addon list's
    /// reset-to-default action.
    pub default_enabled: bool,
}

impl Default for AddonInfo {
    fn default() -> Self {
        Self {
            folder_name: String::new(),
            title: String::new(),
            notes: String::new(),
            enabled: false,
            loaded: false,
            load_on_demand: false,
            use_secure_env: false,
            security: None,
            load_time_secs: 0.0,
            runtime: AddonRuntimeMetrics::default(),
            dependencies: Vec::new(),
            metadata: HashMap::new(),
            default_enabled: true,
        }
    }
}

/// A collected Lua error with optional addon attribution.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LuaErrorRecord {
    /// Raw collected error message.
    pub message: String,
    /// Addon name inferred from the loading/executing context or Lua stack.
    pub addon_name: Option<String>,
}

/// A missing symbol access captured through `_G` or `C_*` namespace `__index` hooks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NilSymbolAccess {
    /// Addon name inferred from the loading/executing context.
    pub addon_name: Option<String>,
    /// Container table where the miss happened (`_G` or `C_*` namespace name).
    pub container: String,
    /// Missing key that resolved to nil.
    pub key: String,
    /// Raw Lua chunk source reported by `debug.getinfo`, if available.
    pub source: Option<String>,
    /// 1-based source line where the nil access happened, if available.
    pub line: Option<i32>,
}
