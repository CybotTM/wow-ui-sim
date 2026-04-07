//! Shared state types for the WoW Lua API.

use crate::cvars::CVarStorage;
use crate::event::{EventQueue, ScriptRegistry};
use crate::lua_api::animation::AnimGroupState;
use crate::lua_api::message_frame::MessageFrameData;
use crate::lua_api::simple_html::SimpleHtmlData;
use crate::lua_api::tooltip::TooltipData;
use crate::screen::ScreenKind;
use crate::sound::SoundManager;
use crate::widget::WidgetRegistry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::time::Instant;

// Re-export game data types so existing `crate::lua_api::state::X` imports keep working.
pub use super::game_data::SpellCooldownState;
pub use super::game_data::{
    AuraInfo, CLASS_LABELS, CastingState, PartyMember, RACE_DATA, ROT_DAMAGE_LEVELS, TargetInfo,
    XP_LEVELS, build_target_info, tick_party_health,
};
use super::game_data::{
    default_action_bars, default_party, default_player_buffs, random_player_name,
};
pub use super::state_types::{
    AddonInfo, AddonRuntimeMetrics, AppFrameMetrics, BagItem, CursorInfo, GreatVaultActivity,
    LootRollInfo, MovementState, PendingTimer, PlayerState, WorldState,
};

/// Shared simulator state accessible from Lua.
pub struct SimState {
    pub widgets: WidgetRegistry,
    pub events: EventQueue,
    pub scripts: ScriptRegistry,
    /// Console output from Lua print() calls.
    pub console_output: Vec<String>,
    /// Pending timer callbacks.
    pub timers: VecDeque<PendingTimer>,
    /// Currently focused frame ID (for keyboard input).
    pub focused_frame_id: Option<u64>,
    /// Registered addons (includes all scanned addons, not just loaded ones).
    pub addons: Vec<AddonInfo>,
    /// Console variables (CVars).
    pub cvars: CVarStorage,
    /// Tooltip state for GameTooltip frames (keyed by frame ID).
    pub tooltips: HashMap<u64, TooltipData>,
    /// SimpleHTML state (keyed by frame ID).
    pub simple_htmls: HashMap<u64, SimpleHtmlData>,
    /// MessageFrame state (keyed by frame ID).
    pub message_frames: HashMap<u64, MessageFrameData>,
    /// Frame IDs with active OnUpdate script handlers.
    pub on_update_frames: HashSet<u64>,
    /// Cached subset of `on_update_frames` whose ancestors are all visible.
    /// Invalidated when `WidgetRegistry::visibility_dirty` is set.
    pub visible_on_update_cache: Option<Vec<u64>>,
    /// Per-strata buckets of visible frame IDs. Index = FrameStrata as usize.
    /// Contains only frames with render_alpha > 0 (visible or button state
    /// textures with visible parent). Built lazily, maintained surgically
    /// by `set_frame_visible`.
    pub strata_buckets: Option<Vec<Vec<u64>>>,
    /// Pending HitGrid updates from `set_frame_visible`. Each entry is the root
    /// frame ID that changed visibility and whether it became visible.
    /// Drained and applied by the App after Lua handlers run.
    pub pending_hit_grid_changes: Vec<(u64, bool)>,
    /// Animation groups keyed by unique group ID.
    pub animation_groups: HashMap<u64, AnimGroupState>,
    /// Counter for generating unique animation group IDs.
    pub next_anim_group_id: u64,
    /// Map: animation-group frame_id → group_id in `animation_groups`.
    pub anim_frame_to_group: HashMap<u64, u64>,
    /// Map: animation frame_id → (group_id, anim_index).
    pub anim_frame_to_anim: HashMap<u64, (u64, usize)>,
    /// Screen dimensions in UI coordinates.
    pub screen_width: f32,
    pub screen_height: f32,
    /// Requested UI surface (in-game vs glue screen).
    pub screen_kind: ScreenKind,
    /// Whether the simulated player is logged into the world.
    pub is_logged_in: bool,
    /// Whether the current glue screen has been displayed at least once.
    pub screen_first_displayed: bool,
    /// Remembered account name for glue login UI helpers.
    pub saved_account_name: String,
    /// Remembered account list string for glue login UI helpers.
    pub saved_account_list: String,
    /// Whether the saved account uses token login.
    pub uses_token: bool,
    /// Action bar slots: slot (1-120) → spell ID.
    pub action_bars: HashMap<u32, u32>,
    /// Addon base paths for runtime on-demand loading (Blizzard UI + AddOns directories).
    pub addon_base_paths: Vec<PathBuf>,
    /// One-shot override for XML frame creation: whether the next CreateFrame
    /// should start hidden before registration/render eligibility.
    pub create_frame_initial_hidden: Option<bool>,
    /// Current mouse position in UI coordinates (for ANCHOR_CURSOR tooltip positioning).
    pub mouse_position: Option<(f32, f32)>,
    /// Currently hovered frame ID (for IsMouseMotionFocus / GetMouseFocus).
    pub hovered_frame: Option<u64>,
    /// Simulated party members (empty = not in group).
    pub party_members: Vec<PartyMember>,
    /// Current target (None = no target).
    pub current_target: Option<TargetInfo>,
    /// Current focus target (None = no focus).
    pub current_focus: Option<TargetInfo>,
    /// Audio playback manager (None when no audio device or WOW_SIM_NO_SOUND=1).
    pub sound_manager: Option<SoundManager>,
    /// Rot damage intensity (index into ROT_DAMAGE_LEVELS).
    pub rot_damage_level: usize,
    /// Current framerate (FPS), updated by the app's FPS counter.
    pub fps: f32,
    /// Instant at which the UI started (used by GetTime and message timestamps).
    pub start_time: Instant,
    /// Active spell cast (None = not casting).
    pub casting: Option<CastingState>,
    /// Counter for generating unique cast IDs.
    pub next_cast_id: u32,
    /// Global Cooldown: (start_time, duration) in GetTime() seconds.
    pub gcd: Option<(f64, f64)>,
    /// Per-spell cooldowns: spell_id → SpellCooldownState.
    pub spell_cooldowns: HashMap<u32, SpellCooldownState>,
    /// Buttons registered via SetActionUIButton(button, action, cooldownFrame).
    pub action_ui_buttons: Vec<(u64, u32)>,
    /// What is currently held on the cursor (drag-and-drop).
    pub cursor_item: Option<CursorInfo>,
    /// Index of the addon currently being loaded (into `addons` vec).
    pub loading_addon_index: Option<u16>,
    /// Index of the addon whose code is currently executing (event/timer/script handlers).
    pub executing_addon_index: Option<u16>,
    /// Whether loading inside a ScopedModifier with forbidden="true".
    pub loading_forbidden: bool,
    /// Application-level frame metrics (total frame time for profiler ratios).
    pub app_frame_metrics: AppFrameMetrics,
    /// Talent tree interactive state (ranks, selections, currency mappings).
    pub talents: super::talent_state::TalentState,
    /// Collected Lua errors (from call_error_handler and addframetext).
    pub lua_errors: Vec<String>,
    /// Global cross-frame Show/Hide dispatch depth (prevents Lua stack overflow
    /// when OnShow handlers trigger Show on other frames recursively).
    pub global_show_hide_depth: u32,
    /// Synced animation group start times (key → elapsed Duration when first PlaySynced was called).
    pub anim_sync_times: HashMap<String, std::time::Duration>,

    /// Player character state (identity, combat, power, buffs, spec).
    pub player: PlayerState,
    /// World state (zone, instance, guild, collections, vault, loot).
    pub world: WorldState,
    /// Bags/Inventory: (bag_index, slot_index) → BagItem.
    pub bag_items: HashMap<(i32, i32), BagItem>,
    /// Debug visualization: red borders around elements.
    pub debug_borders: bool,
    /// Debug visualization: green dots at anchor points.
    pub debug_anchors: bool,
}

struct EmptyStateCollections {
    console_output: Vec<String>,
    timers: VecDeque<PendingTimer>,
    addons: Vec<AddonInfo>,
    lua_errors: Vec<String>,
    tooltips: HashMap<u64, TooltipData>,
    simple_htmls: HashMap<u64, SimpleHtmlData>,
    message_frames: HashMap<u64, MessageFrameData>,
    animation_groups: HashMap<u64, AnimGroupState>,
    anim_sync_times: HashMap<String, std::time::Duration>,
    anim_frame_to_group: HashMap<u64, u64>,
    anim_frame_to_anim: HashMap<u64, (u64, usize)>,
    on_update_frames: HashSet<u64>,
    pending_hit_grid_changes: Vec<(u64, bool)>,
    action_bars: HashMap<u32, u32>,
    addon_base_paths: Vec<PathBuf>,
    spell_cooldowns: HashMap<u32, SpellCooldownState>,
    action_ui_buttons: Vec<(u64, u32)>,
    party_members: Vec<PartyMember>,
    bag_items: HashMap<(i32, i32), BagItem>,
}

impl EmptyStateCollections {
    fn new() -> Self {
        Self {
            console_output: Vec::new(),
            timers: VecDeque::new(),
            addons: Vec::new(),
            lua_errors: Vec::new(),
            tooltips: HashMap::new(),
            simple_htmls: HashMap::new(),
            message_frames: HashMap::new(),
            animation_groups: HashMap::new(),
            anim_sync_times: HashMap::new(),
            anim_frame_to_group: HashMap::new(),
            anim_frame_to_anim: HashMap::new(),
            on_update_frames: HashSet::new(),
            pending_hit_grid_changes: Vec::new(),
            action_bars: HashMap::new(),
            addon_base_paths: Vec::new(),
            spell_cooldowns: HashMap::new(),
            action_ui_buttons: Vec::new(),
            party_members: Vec::new(),
            bag_items: default_backpack_items(),
        }
    }
}

/// Default items in bag 0 (backpack) at startup. Slots are 1-based (WoW convention).
fn default_backpack_items() -> HashMap<(i32, i32), BagItem> {
    [
        (
            1,
            BagItem {
                item_id: 6948,
                stack_count: 1,
            },
        ), // Hearthstone
        (
            2,
            BagItem {
                item_id: 159,
                stack_count: 5,
            },
        ), // Refreshing Spring Water
        (
            3,
            BagItem {
                item_id: 4540,
                stack_count: 5,
            },
        ), // Tough Hunk of Bread
        (
            4,
            BagItem {
                item_id: 7005,
                stack_count: 1,
            },
        ), // Skinning Knife
    ]
    .into_iter()
    .map(|(slot, item)| ((0, slot), item))
    .collect()
}

struct EmptyRuntimeState {
    focused_frame_id: Option<u64>,
    visible_on_update_cache: Option<Vec<u64>>,
    strata_buckets: Option<Vec<Vec<u64>>>,
    create_frame_initial_hidden: Option<bool>,
    mouse_position: Option<(f32, f32)>,
    hovered_frame: Option<u64>,
    current_target: Option<TargetInfo>,
    current_focus: Option<TargetInfo>,
    sound_manager: Option<SoundManager>,
    casting: Option<CastingState>,
    gcd: Option<(f64, f64)>,
    cursor_item: Option<CursorInfo>,
    loading_addon_index: Option<u16>,
    executing_addon_index: Option<u16>,
    loading_forbidden: bool,
    next_anim_group_id: u64,
    next_cast_id: u32,
    screen_width: f32,
    screen_height: f32,
    screen_kind: ScreenKind,
    is_logged_in: bool,
    screen_first_displayed: bool,
    saved_account_name: String,
    saved_account_list: String,
    uses_token: bool,
    fps: f32,
    rot_damage_level: usize,
    start_time: Instant,
}

impl EmptyRuntimeState {
    fn new() -> Self {
        Self {
            focused_frame_id: None,
            visible_on_update_cache: None,
            strata_buckets: None,
            create_frame_initial_hidden: None,
            mouse_position: None,
            hovered_frame: None,
            current_target: None,
            current_focus: None,
            sound_manager: None,
            casting: None,
            gcd: None,
            cursor_item: None,
            loading_addon_index: None,
            executing_addon_index: None,
            loading_forbidden: false,
            next_anim_group_id: 1,
            next_cast_id: 1,
            screen_width: 1600.0,
            screen_height: 1200.0,
            screen_kind: ScreenKind::Game,
            is_logged_in: false,
            screen_first_displayed: false,
            saved_account_name: String::new(),
            saved_account_list: String::new(),
            uses_token: false,
            fps: 0.0,
            rot_damage_level: 0,
            start_time: Instant::now(),
        }
    }
}

impl Default for SimState {
    fn default() -> Self {
        let mut state = Self::new_empty();
        state.action_bars = default_action_bars();
        state.party_members = default_party();
        state.player.name = random_player_name();
        state.player.buffs = default_player_buffs();
        state
    }
}

impl SimState {
    fn new_empty() -> Self {
        let c = EmptyStateCollections::new();
        let r = EmptyRuntimeState::new();

        Self {
            widgets: WidgetRegistry::default(),
            events: EventQueue::default(),
            scripts: ScriptRegistry::default(),
            cvars: CVarStorage::new(),
            console_output: c.console_output,
            timers: c.timers,
            addons: c.addons,
            lua_errors: c.lua_errors,
            tooltips: c.tooltips,
            simple_htmls: c.simple_htmls,
            message_frames: c.message_frames,
            animation_groups: c.animation_groups,
            anim_sync_times: c.anim_sync_times,
            anim_frame_to_group: c.anim_frame_to_group,
            anim_frame_to_anim: c.anim_frame_to_anim,
            on_update_frames: c.on_update_frames,
            pending_hit_grid_changes: c.pending_hit_grid_changes,
            action_bars: c.action_bars,
            addon_base_paths: c.addon_base_paths,
            create_frame_initial_hidden: r.create_frame_initial_hidden,
            spell_cooldowns: c.spell_cooldowns,
            action_ui_buttons: c.action_ui_buttons,
            party_members: c.party_members,
            bag_items: c.bag_items,
            focused_frame_id: r.focused_frame_id,
            visible_on_update_cache: r.visible_on_update_cache,
            strata_buckets: r.strata_buckets,
            mouse_position: r.mouse_position,
            hovered_frame: r.hovered_frame,
            current_target: r.current_target,
            current_focus: r.current_focus,
            sound_manager: r.sound_manager,
            casting: r.casting,
            gcd: r.gcd,
            cursor_item: r.cursor_item,
            loading_addon_index: r.loading_addon_index,
            executing_addon_index: r.executing_addon_index,
            loading_forbidden: r.loading_forbidden,
            next_anim_group_id: r.next_anim_group_id,
            next_cast_id: r.next_cast_id,
            screen_width: r.screen_width,
            screen_height: r.screen_height,
            screen_kind: r.screen_kind,
            is_logged_in: r.is_logged_in,
            screen_first_displayed: r.screen_first_displayed,
            saved_account_name: r.saved_account_name,
            saved_account_list: r.saved_account_list,
            uses_token: r.uses_token,
            fps: r.fps,
            rot_damage_level: r.rot_damage_level,
            start_time: r.start_time,
            app_frame_metrics: AppFrameMetrics::default(),
            talents: super::talent_state::TalentState::new(),
            player: PlayerState::default(),
            world: WorldState::default(),
            global_show_hide_depth: 0,
            debug_borders: false,
            debug_anchors: false,
        }
    }

    /// Look up bag item at (bag, slot). Returns (item_id, stack_count).
    pub fn get_bag_item(&self, bag: i32, slot: i32) -> Option<(u32, i32)> {
        self.bag_items
            .get(&(bag, slot))
            .map(|i| (i.item_id, i.stack_count))
    }

    /// Count occupied slots in a bag.
    pub fn bag_occupied_slots(&self, bag: i32) -> i32 {
        self.bag_items.keys().filter(|(b, _)| *b == bag).count() as i32
    }

    pub fn set_screen_kind(&mut self, screen_kind: ScreenKind) {
        self.screen_kind = screen_kind;
        self.screen_first_displayed = false;
        if screen_kind.is_glue() {
            self.is_logged_in = false;
        }
    }
}
