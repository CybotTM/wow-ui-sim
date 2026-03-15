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

fn uses_parent_alpha_fallback(frame: &crate::widget::Frame) -> bool {
    matches!(
        frame.parent_key.as_deref(),
        Some("NormalTexture" | "PushedTexture" | "HighlightTexture" | "DisabledTexture")
    )
}

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
    /// Synced animation group start times (key → elapsed Duration when first PlaySynced was called).
    pub anim_sync_times: HashMap<String, std::time::Duration>,

    /// Player character state (identity, combat, power, buffs, spec).
    pub player: PlayerState,
    /// World state (zone, instance, guild, collections, vault, loot).
    pub world: WorldState,
    /// Bags/Inventory: (bag_index, slot_index) → BagItem.
    pub bag_items: HashMap<(i32, i32), BagItem>,
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
        Self {
            widgets: WidgetRegistry::default(),
            events: EventQueue::default(),
            scripts: ScriptRegistry::default(),
            cvars: CVarStorage::new(),
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
            create_frame_initial_hidden: None,
            spell_cooldowns: HashMap::new(),
            action_ui_buttons: Vec::new(),
            party_members: Vec::new(),
            bag_items: HashMap::new(),
            focused_frame_id: None,
            visible_on_update_cache: None,
            strata_buckets: None,
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
            app_frame_metrics: AppFrameMetrics::default(),
            talents: super::talent_state::TalentState::new(),
            player: PlayerState::default(),
            world: WorldState::default(),
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

impl SimState {
    /// Initialize derived render state that must be propagated once after startup.
    pub fn initialize_render_state(&mut self) {
        self.widgets.propagate_all_effective_alpha();
        self.widgets.propagate_all_effective_scale();
    }

    /// Return the per-strata buckets, building lazily if needed.
    pub fn get_strata_buckets(&mut self) -> Option<&Vec<Vec<u64>>> {
        if self.strata_buckets.is_none() {
            self.strata_buckets = Some(self.build_strata_buckets());
        }
        self.strata_buckets.as_ref()
    }

    /// Build per-strata ID buckets for visible frames only, sorted by render order.
    ///
    /// A frame is included if its "render alpha" > 0: either its own
    /// `effective_alpha > 0`, or (for button state textures with `visible=false`
    /// but `alpha > 0`) its parent's `effective_alpha > 0`. Frames with
    /// explicit `alpha=0` (glow/anim textures) are always excluded.
    fn build_strata_buckets(&mut self) -> Vec<Vec<u64>> {
        use crate::iced_app::frame_collect::intra_strata_sort_key;
        use crate::widget::WidgetType;
        let mut buckets = vec![Vec::new(); crate::widget::FrameStrata::COUNT];
        for id in self.widgets.iter_ids() {
            let Some(f) = self.widgets.get(id) else {
                continue;
            };
            // Visibility filter: skip frames with no render alpha.
            // Fall back to parent alpha only for frames hidden via visible=false
            // (button state textures), NOT for frames with explicit alpha=0.
            let render_alpha = if f.effective_alpha > 0.0 {
                f.effective_alpha
            } else if f.alpha > 0.0 && uses_parent_alpha_fallback(f) {
                f.parent_id
                    .and_then(|pid| self.widgets.get(pid))
                    .map(|p| p.effective_alpha)
                    .unwrap_or(0.0)
            } else {
                0.0
            };
            if render_alpha <= 0.0 {
                continue;
            }
            let strata = if matches!(
                f.widget_type,
                WidgetType::Texture | WidgetType::FontString | WidgetType::Line
            ) {
                f.parent_id
                    .and_then(|pid| self.widgets.get(pid))
                    .map(|p| p.frame_strata)
                    .unwrap_or(f.frame_strata)
            } else {
                f.frame_strata
            };
            buckets[strata.as_index()].push(id);
        }
        for bucket in &mut buckets {
            bucket.sort_by(|&a, &b| match (self.widgets.get(a), self.widgets.get(b)) {
                (Some(fa), Some(fb)) => intra_strata_sort_key(fa, a, &self.widgets)
                    .cmp(&intra_strata_sort_key(fb, b, &self.widgets)),
                _ => a.cmp(&b),
            });
        }
        buckets
    }

    /// Eagerly recompute layout rect for a frame and all its descendants.
    /// Called when layout-affecting properties change (anchors, size, scale, parent).
    /// Stores the computed rect on each Frame so the renderer can use it directly.
    pub fn invalidate_layout(&mut self, id: u64) {
        let sw = self.screen_width;
        let sh = self.screen_height;
        let mut cache = crate::iced_app::layout::LayoutCache::default();
        Self::recompute_layout_subtree(&mut self.widgets, id, sw, sh, &mut cache);
        // Frame positions may have changed — schedule hit grid re-insertion
        // so apply_hit_grid_changes updates stale rectangles.
        self.pending_hit_grid_changes.push((id, true));
    }

    /// Like `invalidate_layout` but also recomputes sibling frames anchored to
    /// `id`. Uses the reverse anchor index for O(k) lookup where k = number of
    /// dependents. Called by SetWidth/SetHeight/SetSize/SetScale/SetAtlas so
    /// that cross-frame-anchored siblings (e.g. three-slice Center) update.
    pub fn invalidate_layout_with_dependents(&mut self, id: u64) {
        let sw = self.screen_width;
        let sh = self.screen_height;
        let mut cache = crate::iced_app::layout::LayoutCache::default();
        Self::recompute_layout_subtree(&mut self.widgets, id, sw, sh, &mut cache);
        Self::recompute_anchor_dependents(&mut self.widgets, id, sw, sh, &mut cache, 0);
        self.pending_hit_grid_changes.push((id, true));
    }

    fn recompute_layout_subtree(
        widgets: &mut crate::widget::WidgetRegistry,
        id: u64,
        screen_width: f32,
        screen_height: f32,
        cache: &mut crate::iced_app::layout::LayoutCache,
    ) {
        // Remove stale entry so compute_frame_rect_cached recomputes.
        cache.remove(&id);
        let rect = crate::iced_app::compute_frame_rect_cached(
            widgets,
            id,
            screen_width,
            screen_height,
            cache,
        )
        .rect;
        let children: Vec<u64> = widgets
            .get(id)
            .map(|f| f.children.clone())
            .unwrap_or_default();
        if let Some(f) = widgets.get_mut(id) {
            f.layout_rect = Some(rect);
        }
        widgets.mark_layout_resolved(id);
        for child_id in children {
            Self::recompute_layout_subtree(widgets, child_id, screen_width, screen_height, cache);
        }
    }

    /// Recompute frames anchored to `target_id` using the reverse index.
    ///
    /// O(k) where k = number of frames anchored to target_id.
    fn recompute_anchor_dependents(
        widgets: &mut crate::widget::WidgetRegistry,
        target_id: u64,
        sw: f32,
        sh: f32,
        cache: &mut crate::iced_app::layout::LayoutCache,
        _depth: u32,
    ) {
        let deps: Vec<u64> = widgets
            .get_anchor_dependents(target_id)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        for dep_id in deps {
            Self::recompute_layout_subtree(widgets, dep_id, sw, sh, cache);
        }
    }
}

impl SimState {
    /// Ensure every frame has a layout_rect and resolve dirty roots.
    /// Called before quad rebuilds (acts as the "next frame" layout resolution).
    pub fn ensure_layout_rects(&mut self) {
        // Phase 1: frames that never had layout computed
        let pending = self.widgets.drain_pending_layout();
        if !pending.is_empty() {
            let sw = self.screen_width;
            let sh = self.screen_height;
            let mut cache = crate::iced_app::layout::LayoutCache::default();
            let pending_root_ids: Vec<u64> = pending
                .iter()
                .copied()
                .filter(|id| {
                    self.widgets
                        .get(*id)
                        .and_then(|f| f.parent_id)
                        .is_none_or(|parent_id| !pending.contains(&parent_id))
                })
                .collect();
            for id in pending_root_ids {
                if self
                    .widgets
                    .get(id)
                    .is_some_and(|f| f.layout_rect.is_none())
                {
                    Self::recompute_layout_subtree(&mut self.widgets, id, sw, sh, &mut cache);
                    self.widgets.clear_rect_dirty_subtree(id);
                }
            }
        }
        // Phase 2: dirty roots — recompute subtree + anchor dependents
        let dirty = self.widgets.drain_rect_dirty();
        if !dirty.is_empty() {
            let sw = self.screen_width;
            let sh = self.screen_height;
            let mut cache = crate::iced_app::layout::LayoutCache::default();
            for id in &dirty {
                Self::recompute_layout_subtree(&mut self.widgets, *id, sw, sh, &mut cache);
                Self::recompute_anchor_dependents(&mut self.widgets, *id, sw, sh, &mut cache, 0);
            }
        }
    }

    /// Force layout resolution for a single frame, clearing its rect_dirty flag.
    /// Called by GetSize/GetWidth/GetHeight, rect query methods, and IsRectValid
    /// to match WoW behavior where layout resolves immediately.
    pub fn resolve_rect_if_dirty(&mut self, id: u64) {
        if !self.widgets.is_rect_dirty(id) {
            return;
        }
        self.resolve_dirty_ancestors(id);
        self.invalidate_layout(id);
        self.widgets.clear_rect_dirty(id);
    }

    /// Resolve dirty ancestor roots that cause `id` to appear dirty via the
    /// `is_rect_dirty` ancestor walk. Computes their layout rects and clears
    /// their dirty flags so descendants become clean.
    fn resolve_dirty_ancestors(&mut self, id: u64) {
        let dirty_roots = self.widgets.collect_dirty_ancestor_roots(id);
        if dirty_roots.is_empty() {
            return;
        }
        let sw = self.screen_width;
        let sh = self.screen_height;
        let mut cache = crate::iced_app::layout::LayoutCache::default();
        // Process topmost first (reverse of bottom-up collection order).
        // Recompute the full subtree so siblings of `id` also get updated
        // layout_rects before we clear the dirty flag.
        for &root_id in dirty_roots.iter().rev() {
            Self::recompute_layout_subtree(&mut self.widgets, root_id, sw, sh, &mut cache);
            self.widgets.clear_rect_dirty(root_id);
        }
    }

    /// Set a frame's visibility and eagerly propagate effective_alpha.
    /// Surgically updates strata_buckets: inserts on show, removes on hide.
    pub fn set_frame_visible(&mut self, id: u64, visible: bool) {
        let was_visible = self.widgets.get(id).map(|f| f.visible).unwrap_or(false);
        self.widgets.set_visible(id, visible);
        if was_visible == visible {
            return;
        }
        // Toplevel frames are raised above siblings when shown (WoW behavior).
        if visible {
            let is_toplevel = self.widgets.get(id).map(|f| f.toplevel).unwrap_or(false);
            if is_toplevel {
                self.raise_frame(id);
            }
        }
        self.update_on_update_cache(id, visible);
        // Propagate effective_alpha: look up parent's effective_alpha.
        let parent_eff = self
            .widgets
            .get(id)
            .and_then(|f| f.parent_id)
            .and_then(|pid| self.widgets.get(pid))
            .map(|p| p.effective_alpha)
            .unwrap_or(1.0);
        if !visible {
            // Hide: remove subtree from buckets BEFORE propagating alpha to 0.
            self.remove_subtree_from_buckets(id);
        }
        self.widgets.propagate_effective_alpha(id, parent_eff);
        if visible {
            // Show: insert newly-visible frames AFTER propagating alpha.
            self.insert_subtree_into_buckets(id);
        }
        // Record for incremental HitGrid update (applied by App after Lua runs).
        self.pending_hit_grid_changes.push((id, visible));
    }

    /// Remove a frame and all its descendants from strata_buckets.
    fn remove_subtree_from_buckets(&mut self, root_id: u64) {
        let Some(buckets) = self.strata_buckets.as_mut() else {
            return;
        };
        // Collect all IDs in the subtree.
        let mut subtree = std::collections::HashSet::new();
        let mut queue = vec![root_id];
        while let Some(fid) = queue.pop() {
            subtree.insert(fid);
            if let Some(f) = self.widgets.get(fid) {
                queue.extend(f.children.iter().copied());
            }
        }
        for bucket in buckets.iter_mut() {
            bucket.retain(|id| !subtree.contains(id));
        }
    }

    /// Insert newly-visible frames from a subtree into strata_buckets.
    ///
    /// Walks all descendants and inserts those with render_alpha > 0
    /// (own effective_alpha, or parent's for button state textures with alpha > 0).
    fn insert_subtree_into_buckets(&mut self, root_id: u64) {
        let Some(buckets) = self.strata_buckets.as_mut() else {
            return;
        };
        use crate::iced_app::frame_collect::intra_strata_sort_key;
        use crate::widget::WidgetType;
        // Walk all descendants.
        let mut queue = vec![root_id];
        while let Some(fid) = queue.pop() {
            let Some(f) = self.widgets.get(fid) else {
                continue;
            };
            queue.extend(f.children.iter().copied());
            let render_alpha = if f.effective_alpha > 0.0 {
                f.effective_alpha
            } else if f.alpha > 0.0 && uses_parent_alpha_fallback(f) {
                f.parent_id
                    .and_then(|pid| self.widgets.get(pid))
                    .map(|p| p.effective_alpha)
                    .unwrap_or(0.0)
            } else {
                0.0
            };
            if render_alpha <= 0.0 {
                continue;
            }
            let strata = if matches!(
                f.widget_type,
                WidgetType::Texture | WidgetType::FontString | WidgetType::Line
            ) {
                f.parent_id
                    .and_then(|pid| self.widgets.get(pid))
                    .map(|p| p.frame_strata)
                    .unwrap_or(f.frame_strata)
            } else {
                f.frame_strata
            };
            let key = intra_strata_sort_key(f, fid, &self.widgets);
            let bucket = &mut buckets[strata.as_index()];
            let pos = bucket.partition_point(|&existing_id| {
                self.widgets
                    .get(existing_id)
                    .map(|ef| intra_strata_sort_key(ef, existing_id, &self.widgets))
                    .unwrap_or_default()
                    < key
            });
            bucket.insert(pos, fid);
        }
    }

    /// Raise a frame above all siblings in the same strata+level.
    ///
    /// Sets `raise_order` to max sibling raise_order + 1 without modifying
    /// `frame_level`. Resorts the affected subtree in strata buckets.
    pub fn raise_frame(&mut self, id: u64) {
        let (parent_id, strata, level) = match self.widgets.get(id) {
            Some(f) => (f.parent_id, f.frame_strata, f.frame_level),
            None => return,
        };
        let max_raise_order = self
            .sibling_raise_order_range(id, parent_id, strata, level)
            .1;
        let current_raise_order = self.widgets.get(id).map(|f| f.raise_order).unwrap_or(0);
        if current_raise_order > max_raise_order {
            return; // Already on top
        }
        if let Some(f) = self.widgets.get_mut_visual(id) {
            f.raise_order = max_raise_order + 1;
        }
        // Re-sort the affected subtree in strata buckets.
        // Avoid setting strata_buckets = None: Show/Hide calls later in the
        // same handler chain rely on buckets being Some for surgical insert/remove.
        if self.strata_buckets.is_some() {
            self.remove_subtree_from_buckets(id);
            self.insert_subtree_into_buckets(id);
        }
    }

    /// Lower a frame below all siblings in the same strata+level.
    ///
    /// Sets `raise_order` to min sibling raise_order - 1 without modifying
    /// `frame_level`. Resorts the affected subtree in strata buckets.
    pub fn lower_frame(&mut self, id: u64) {
        let (parent_id, strata, level) = match self.widgets.get(id) {
            Some(f) => (f.parent_id, f.frame_strata, f.frame_level),
            None => return,
        };
        let min_raise_order = self
            .sibling_raise_order_range(id, parent_id, strata, level)
            .0;
        let current_raise_order = self.widgets.get(id).map(|f| f.raise_order).unwrap_or(0);
        if current_raise_order < min_raise_order {
            return; // Already at bottom
        }
        if let Some(f) = self.widgets.get_mut_visual(id) {
            f.raise_order = min_raise_order - 1;
        }
        if self.strata_buckets.is_some() {
            self.remove_subtree_from_buckets(id);
            self.insert_subtree_into_buckets(id);
        }
    }

    /// Return (min, max) raise_order among siblings of `id` in the given strata+level.
    fn sibling_raise_order_range(
        &self,
        id: u64,
        parent_id: Option<u64>,
        strata: crate::widget::FrameStrata,
        level: i32,
    ) -> (i32, i32) {
        let sibling_ids: Vec<u64> = if let Some(pid) = parent_id {
            self.widgets
                .get(pid)
                .map(|p| p.children.clone())
                .unwrap_or_default()
        } else {
            // Root frames: all frames with no parent
            self.widgets
                .iter_ids()
                .filter(|&fid| {
                    self.widgets
                        .get(fid)
                        .map(|f| f.parent_id.is_none())
                        .unwrap_or(false)
                })
                .collect()
        };
        let orders: Vec<i32> = sibling_ids
            .iter()
            .filter(|&&sid| sid != id)
            .filter_map(|&sid| self.widgets.get(sid))
            .filter(|f| f.frame_strata == strata && f.frame_level == level)
            .map(|f| f.raise_order)
            .collect();
        let min = orders.iter().copied().min().unwrap_or(0);
        let max = orders.iter().copied().max().unwrap_or(0);
        (min, max)
    }

    fn update_on_update_cache(&mut self, id: u64, visible: bool) {
        let Some(mut cache) = self.visible_on_update_cache.take() else {
            return;
        };
        if visible {
            self.add_on_update_descendants(id, &mut cache);
        } else {
            self.remove_on_update_descendants(id, &mut cache);
        }
        self.visible_on_update_cache = Some(cache);
    }

    /// Add `id` and its descendants to cache if they have OnUpdate and are ancestor-visible.
    fn add_on_update_descendants(&self, id: u64, cache: &mut Vec<u64>) {
        if self.on_update_frames.contains(&id) && self.widgets.is_ancestor_visible(id) {
            if !cache.contains(&id) {
                cache.push(id);
            }
        }
        let children: Vec<u64> = self
            .widgets
            .get(id)
            .map(|f| f.children.clone())
            .unwrap_or_default();
        for child_id in children {
            if self.widgets.get(child_id).is_some_and(|f| f.visible) {
                self.add_on_update_descendants(child_id, cache);
            }
        }
    }

    /// Remove `id` and all its descendants from cache (hidden ancestor = all hidden).
    fn remove_on_update_descendants(&self, id: u64, cache: &mut Vec<u64>) {
        cache.retain(|&cached_id| cached_id != id);
        let children: Vec<u64> = self
            .widgets
            .get(id)
            .map(|f| f.children.clone())
            .unwrap_or_default();
        for child_id in children {
            self.remove_on_update_descendants(child_id, cache);
        }
    }

    /// Keep only OnUpdate handlers owned by the named addon. Invalidates cache.
    pub fn retain_on_update_for_addon(&mut self, addon_name: &str) {
        let idx = self.addons.iter().position(|a| a.folder_name == addon_name);
        let addon_idx = idx.map(|i| i as u16);
        let before = self.on_update_frames.len();
        self.on_update_frames
            .retain(|&id| self.widgets.get(id).and_then(|f| f.owner_addon) == addon_idx);
        self.visible_on_update_cache = None;
        let after = self.on_update_frames.len();
        eprintln!("[self-test] stripped OnUpdate: {before} → {after} (keeping {addon_name})");
    }
}
