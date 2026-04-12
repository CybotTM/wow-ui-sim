//! Global widget registry for tracking all widgets.

use super::Frame;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderDirtySource {
    pub frame_id: u64,
    pub method: String,
}

#[derive(Debug, Default)]
pub struct RenderDirtyBatch {
    pub strata_mask: u16,
    pub frame_ids: Option<HashSet<u64>>,
    pub sources: HashMap<u64, HashSet<RenderDirtySource>>,
}

/// Registry of all widgets in the UI.
#[derive(Debug, Default)]
pub struct WidgetRegistry {
    /// Widgets by ID.
    widgets: HashMap<u64, Frame>,
    /// Widget IDs by name.
    names: HashMap<String, u64>,
    /// Widget IDs in creation order (monotonically increasing, always sorted).
    ordered_ids: Vec<u64>,
    /// Frame IDs whose visual properties changed since last render.
    /// Checked and drained by the render loop.
    render_dirty_ids: RefCell<HashSet<u64>>,
    /// Dirty provenance captured while a specific script/method is running.
    render_dirty_sources: RefCell<HashMap<u64, HashSet<RenderDirtySource>>>,
    /// Current source context for dirty attribution.
    current_dirty_source: RefCell<Option<RenderDirtySource>>,
    /// Reverse index: target_id → set of frame IDs anchored to it.
    anchor_dependents: HashMap<u64, HashSet<u64>>,
    /// Frames with `rect_dirty = true`, for fast lookup in `ensure_layout_rects`.
    rect_dirty_ids: HashSet<u64>,
    /// Frames with `layout_rect = None` that need layout computation.
    pending_layout_ids: HashSet<u64>,
}

impl WidgetRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new widget.
    pub fn register(&mut self, widget: Frame) -> u64 {
        let id = widget.id;
        let is_new = !self.widgets.contains_key(&id);
        // Debug: check for re-registration that would lose children
        if let Some(existing) = self.widgets.get(&id)
            && !existing.children.is_empty()
        {
            eprintln!(
                "[WARN] Re-registering widget id={} name={:?} which has {} children!",
                id,
                existing.name,
                existing.children.len()
            );
        }
        if let Some(ref name) = widget.name {
            self.names.insert(name.clone(), id);
        }
        if widget.layout_rect.is_none() {
            self.pending_layout_ids.insert(id);
        }
        self.widgets.insert(id, widget);
        if is_new {
            self.ordered_ids.push(id);
        }
        id
    }

    /// Get a widget by ID.
    pub fn get(&self, id: u64) -> Option<&Frame> {
        self.widgets.get(&id)
    }

    /// Get a mutable widget by ID. Does not mark dirty.
    ///
    /// Use for non-visual mutations (event registration, attributes, input
    /// config, animation offsets, layout cache, parent-child bookkeeping).
    /// For visual mutations, use `get_mut_visual()` instead.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Frame> {
        self.widgets.get_mut(&id)
    }

    /// Get a mutable widget by ID and mark it visually dirty.
    ///
    /// Use when changing visual properties: texture, text, alpha, color,
    /// visibility, size, anchors, draw_layer, frame_strata, backdrop, etc.
    pub fn get_mut_visual(&mut self, id: u64) -> Option<&mut Frame> {
        self.record_visual_dirty(id);
        self.widgets.get_mut(&id)
    }

    /// Mark a frame as visually dirty (needs re-render).
    ///
    /// Call after changing visual properties: texture, text, alpha, color,
    /// visibility, size, anchors, tex_coords, atlas, blend_mode, vertex_color,
    /// nine_slice, backdrop, rotation, desaturated.
    pub fn mark_visual_dirty(&self, id: u64) {
        self.record_visual_dirty(id);
    }

    /// Mark all frames as visually dirty (e.g. after screen resize).
    pub fn mark_all_visual_dirty(&self) {
        // Insert a sentinel value that consumers check via has_dirty_frames().
        // Avoids iterating all 50K frames just to insert their IDs.
        self.render_dirty_ids.borrow_mut().insert(u64::MAX);
    }

    pub fn set_render_dirty_source(&self, source: Option<RenderDirtySource>) {
        *self.current_dirty_source.borrow_mut() = source;
    }

    /// Set (or update) the name of a widget, updating the names index.
    pub fn set_name(&mut self, id: u64, name: String) {
        self.names.insert(name.clone(), id);
        if let Some(frame) = self.widgets.get_mut(&id) {
            frame.name = Some(name);
        }
    }

    /// Get a widget by name.
    pub fn get_by_name(&self, name: &str) -> Option<&Frame> {
        self.names.get(name).and_then(|id| self.widgets.get(id))
    }

    /// Get a widget ID by name.
    pub fn get_id_by_name(&self, name: &str) -> Option<u64> {
        self.names.get(name).copied()
    }

    /// Iterate all (id, name) pairs in the registry.
    pub fn named_frames(&self) -> impl Iterator<Item = (u64, &String)> {
        self.names.iter().map(|(name, &id)| (id, name))
    }

    /// Get all widgets registered for a specific event.
    /// Individual RegisterEvent listeners fire before RegisterAllEvents listeners.
    /// Within each group, frames fire in creation order (ascending ID).
    pub fn get_event_listeners(&self, event: &str) -> Vec<u64> {
        let mut individual = Vec::new();
        let mut all_events = Vec::new();
        for frame in self.widgets.values() {
            if frame.registered_events.contains(event) {
                individual.push(frame.id);
            } else if frame.register_all_events {
                all_events.push(frame.id);
            }
        }
        individual.sort_unstable();
        all_events.sort_unstable();
        individual.extend(all_events);
        individual
    }

    /// Add a child to a parent widget.
    pub fn add_child(&mut self, parent_id: u64, child_id: u64) {
        if let Some(parent) = self.widgets.get_mut(&parent_id) {
            parent.children.push(child_id);
        }
    }

    /// Iterate over all widget IDs.
    pub fn iter_ids(&self) -> impl Iterator<Item = u64> + '_ {
        self.widgets.keys().copied()
    }

    pub fn storage_estimate_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.widgets.capacity() * std::mem::size_of::<(u64, Frame)>()
            + self
                .widgets
                .values()
                .map(Frame::storage_estimate_bytes)
                .sum::<usize>()
            + self.names.capacity() * std::mem::size_of::<(String, u64)>()
            + self.names.keys().map(String::capacity).sum::<usize>()
            + self.ordered_ids.capacity() * std::mem::size_of::<u64>()
            + hash_set_u64_bytes(&self.render_dirty_ids.borrow())
            + render_dirty_sources_bytes(&self.render_dirty_sources.borrow())
            + dirty_source_bytes(&self.current_dirty_source.borrow())
            + hash_map_u64_hash_set_u64_bytes(&self.anchor_dependents)
            + hash_set_u64_bytes(&self.rect_dirty_ids)
            + hash_set_u64_bytes(&self.pending_layout_ids)
    }

    /// Collect unique texture paths from all visible frames.
    pub fn visible_texture_paths(&self) -> Vec<String> {
        let mut paths = std::collections::HashSet::new();
        for frame in self.widgets.values() {
            if !frame.visible {
                continue;
            }
            for path in [
                &frame.texture,
                &frame.normal_texture,
                &frame.pushed_texture,
                &frame.highlight_texture,
                &frame.disabled_texture,
            ] {
                if let Some(t) = path {
                    paths.insert(t.clone());
                }
            }
        }
        paths.into_iter().collect()
    }

    /// Get the next widget ID after `after_id` in creation order.
    /// Returns `None` when enumeration is complete.
    pub fn next_id_after(&self, after_id: u64) -> Option<u64> {
        let idx = self.ordered_ids.partition_point(|&id| id <= after_id);
        self.ordered_ids.get(idx).copied()
    }

    /// Clear all cached layout rects (e.g. after screen resize).
    pub fn clear_all_layout_rects(&mut self) {
        self.pending_layout_ids.clear();
        for (&id, frame) in self.widgets.iter_mut() {
            frame.layout_rect = None;
            if frame.parent_id.is_none() {
                self.pending_layout_ids.insert(id);
            }
        }
        self.mark_all_visual_dirty();
    }

    /// Check whether any frames have been visually dirtied since last drain.
    pub fn has_dirty_frames(&self) -> bool {
        !self.render_dirty_ids.borrow().is_empty()
    }

    /// Drain the set of visually dirty frame IDs and return a per-strata
    /// bitmask indicating which strata contain dirty frames.
    ///
    /// Bit `i` is set when at least one dirty frame lives in strata index `i`.
    /// The sentinel `u64::MAX` (from `mark_all_visual_dirty`) produces the
    /// all-strata mask `(1 << COUNT) - 1`.
    pub fn take_render_dirty(&self) -> u16 {
        self.take_render_dirty_batch().strata_mask
    }

    /// Drain the dirty set, returning both the strata bitmask and the set of
    /// dirty frame IDs. Returns `None` for the ID set when the sentinel
    /// (`u64::MAX`) was present, signalling that a full rebuild is needed.
    pub fn take_render_dirty_with_ids(&self) -> (u16, Option<HashSet<u64>>) {
        let batch = self.take_render_dirty_batch();
        (batch.strata_mask, batch.frame_ids)
    }

    pub fn take_render_dirty_batch(&self) -> RenderDirtyBatch {
        let mut ids = self.render_dirty_ids.borrow_mut();
        if ids.is_empty() {
            return self.empty_render_dirty_batch();
        }
        let (strata_mask, has_sentinel) = self.render_dirty_mask(&ids);
        let frame_ids = Self::drain_render_dirty_ids(&mut ids, has_sentinel);
        RenderDirtyBatch {
            strata_mask,
            frame_ids,
            sources: self.take_render_dirty_sources(),
        }
    }

    fn empty_render_dirty_batch(&self) -> RenderDirtyBatch {
        RenderDirtyBatch {
            strata_mask: 0,
            frame_ids: Some(HashSet::new()),
            sources: HashMap::new(),
        }
    }

    fn render_dirty_mask(&self, ids: &HashSet<u64>) -> (u16, bool) {
        let all_mask = (1u16 << super::FrameStrata::COUNT) - 1;
        let has_sentinel = ids.contains(&u64::MAX);
        if has_sentinel {
            return (all_mask, true);
        }

        let mut mask: u16 = 0;
        for &id in ids {
            mask |= self.strata_bit_for(id);
            if mask == all_mask {
                break;
            }
        }
        (mask, false)
    }

    fn drain_render_dirty_ids(ids: &mut HashSet<u64>, has_sentinel: bool) -> Option<HashSet<u64>> {
        if has_sentinel {
            ids.clear();
            None
        } else {
            Some(std::mem::take(ids))
        }
    }

    fn take_render_dirty_sources(&self) -> HashMap<u64, HashSet<RenderDirtySource>> {
        std::mem::take(&mut *self.render_dirty_sources.borrow_mut())
    }

    /// Return the strata bitmask for a single frame ID.
    ///
    /// Regions (Texture, FontString, Line) use their parent's strata.
    fn strata_bit_for(&self, id: u64) -> u16 {
        let Some(f) = self.widgets.get(&id) else {
            return 0;
        };
        let strata = match f.widget_type {
            super::WidgetType::Texture
            | super::WidgetType::FontString
            | super::WidgetType::Line => f
                .parent_id
                .and_then(|pid| self.widgets.get(&pid))
                .map(|p| p.frame_strata)
                .unwrap_or(f.frame_strata),
            _ => f.frame_strata,
        };
        1u16 << strata.as_index()
    }

    /// Set a widget's visibility flag and mark it visually dirty.
    ///
    /// Prefer `SimState::set_frame_visible` which also updates the OnUpdate cache.
    pub fn set_visible(&mut self, id: u64, visible: bool) {
        if let Some(f) = self.widgets.get_mut(&id) {
            if f.visible != visible {
                f.visible = visible;
                self.mark_visual_dirty(id);
            }
        }
    }

    /// Check if a frame and all its ancestors are visible (shown).
    ///
    /// Matches WoW's `IsVisible()` semantics: a frame is visible when its
    /// own `visible` flag is true AND all ancestors are visible. Alpha does
    /// NOT affect visibility — a frame with alpha=0 is still "visible" and
    /// receives OnUpdate, events, etc.
    pub fn is_ancestor_visible(&self, id: u64) -> bool {
        let mut current_id = id;
        loop {
            let Some(f) = self.widgets.get(&current_id) else {
                return false;
            };
            if !f.visible {
                return false;
            }
            match f.parent_id {
                Some(parent_id) => current_id = parent_id,
                None => return true,
            }
        }
    }

    /// Recompute `effective_alpha` for a frame and propagate to all descendants.
    ///
    /// Visible frames inherit their parent's effective alpha unless they
    /// explicitly ignore it. Hidden frames always resolve to 0.0.
    /// Also marks frames as visually dirty when their effective_alpha changes,
    /// so cached quad snapshots with baked-in alpha are invalidated.
    pub fn propagate_effective_alpha(&mut self, id: u64, parent_effective_alpha: f32) {
        let Some(f) = self.widgets.get_mut(&id) else {
            return;
        };
        let eff = if f.visible {
            if f.ignore_parent_alpha {
                f.alpha
            } else {
                parent_effective_alpha * f.alpha
            }
        } else {
            0.0
        };
        let became_dirty = (eff - f.effective_alpha).abs() > f32::EPSILON;
        if became_dirty {
            f.effective_alpha = eff;
        } else {
            f.effective_alpha = eff;
        }
        let children: Vec<u64> = f.children.clone();
        let _ = f;
        if became_dirty {
            self.record_visual_dirty(id);
        }
        for child_id in children {
            self.propagate_effective_alpha(child_id, eff);
        }
    }

    /// Propagate effective_alpha for ALL frames from root. Called once at startup
    /// to initialize effective_alpha after all frames are created and parented.
    pub fn propagate_all_effective_alpha(&mut self) {
        let root_ids: Vec<u64> = self
            .widgets
            .keys()
            .copied()
            .filter(|&id| self.widgets.get(&id).is_some_and(|f| f.parent_id.is_none()))
            .collect();
        for id in root_ids {
            self.propagate_effective_alpha(id, 1.0);
        }
    }

    /// Propagate effective_scale for ALL frames from root. Called once at startup.
    pub fn propagate_all_effective_scale(&mut self) {
        let root_ids: Vec<u64> = self
            .widgets
            .keys()
            .copied()
            .filter(|&id| self.widgets.get(&id).is_some_and(|f| f.parent_id.is_none()))
            .collect();
        for id in root_ids {
            self.propagate_effective_scale(id, 1.0);
        }
    }

    /// Recompute `effective_scale` for a frame and propagate to all descendants.
    ///
    /// Frames that ignore parent scale use only their own scale. All other
    /// frames inherit the parent's effective scale.
    pub fn propagate_effective_scale(&mut self, id: u64, parent_effective_scale: f32) {
        let Some(f) = self.widgets.get_mut(&id) else {
            return;
        };
        let eff = if f.ignore_parent_scale {
            f.scale
        } else {
            parent_effective_scale * f.scale
        };
        let became_dirty = (eff - f.effective_scale).abs() > f32::EPSILON;
        f.effective_scale = eff;
        let children: Vec<u64> = f.children.clone();
        let _ = f;
        if became_dirty {
            self.record_visual_dirty(id);
        }
        for child_id in children {
            self.propagate_effective_scale(child_id, eff);
        }
    }

    /// Mark a frame as rect-dirty root. O(1) — no subtree walk.
    /// Descendants discover dirtiness lazily via `is_rect_dirty` ancestor walk.
    pub fn mark_rect_dirty(&mut self, id: u64) {
        if self.widgets.contains_key(&id) {
            self.rect_dirty_ids.insert(id);
        }
    }

    /// Check if a frame or any ancestor is rect-dirty.
    /// Pure ancestor walk using `rect_dirty_ids` as single source of truth.
    pub fn is_rect_dirty(&self, id: u64) -> bool {
        let mut current = Some(id);
        while let Some(cid) = current {
            if self.rect_dirty_ids.contains(&cid) {
                return true;
            }
            current = self.widgets.get(&cid).and_then(|f| f.parent_id);
        }
        false
    }

    /// Walk up the parent chain and collect all frame IDs (including `id`)
    /// that are in `rect_dirty_ids`. Returns in bottom-up order.
    pub fn collect_dirty_ancestor_roots(&self, id: u64) -> Vec<u64> {
        let mut roots = Vec::new();
        let mut current = Some(id);
        while let Some(cid) = current {
            if self.rect_dirty_ids.contains(&cid) {
                roots.push(cid);
            }
            current = self.widgets.get(&cid).and_then(|f| f.parent_id);
        }
        roots
    }

    /// Clear rect-dirty on a single frame (after layout recomputation).
    pub fn clear_rect_dirty(&mut self, id: u64) {
        self.rect_dirty_ids.remove(&id);
    }

    /// Clear rect-dirty for a frame and all descendants.
    pub fn clear_rect_dirty_subtree(&mut self, id: u64) {
        self.rect_dirty_ids.remove(&id);
        let children = self
            .widgets
            .get(&id)
            .map(|f| f.children.clone())
            .unwrap_or_default();
        for child_id in children {
            self.clear_rect_dirty_subtree(child_id);
        }
    }

    /// Drain rect_dirty_ids. Returns the set for callers that need it.
    pub fn drain_rect_dirty(&mut self) -> HashSet<u64> {
        std::mem::take(&mut self.rect_dirty_ids)
    }

    /// Drain pending_layout_ids (frames missing layout_rect).
    pub fn drain_pending_layout(&mut self) -> HashSet<u64> {
        std::mem::take(&mut self.pending_layout_ids)
    }

    /// Mark a frame's layout as resolved: remove from pending set.
    pub fn mark_layout_resolved(&mut self, id: u64) {
        self.pending_layout_ids.remove(&id);
    }

    /// Check if setting a point from `frame_id` to `relative_to_id` would create a cycle.
    /// A cycle exists if relative_to (or any of its anchor dependencies) already
    /// depends on frame_id.
    pub fn would_create_anchor_cycle(&self, frame_id: u64, relative_to_id: u64) -> bool {
        frame_id == relative_to_id || self.anchor_dependency_reaches_frame(relative_to_id, frame_id)
    }

    fn anchor_dependency_reaches_frame(&self, start_id: u64, target_id: u64) -> bool {
        let mut queue = VecDeque::from([start_id]);
        let mut seen = HashSet::from([start_id]);

        while let Some(check_id) = queue.pop_front() {
            if self.enqueue_anchor_dependencies(check_id, target_id, &mut queue, &mut seen) {
                return true;
            }
        }

        false
    }

    fn enqueue_anchor_dependencies(
        &self,
        frame_id: u64,
        target_id: u64,
        queue: &mut VecDeque<u64>,
        seen: &mut HashSet<u64>,
    ) -> bool {
        for anchor_target_id in self.anchor_target_ids(frame_id) {
            if anchor_target_id == target_id {
                return true;
            }
            if seen.insert(anchor_target_id) {
                queue.push_back(anchor_target_id);
            }
        }

        false
    }

    fn anchor_target_ids(&self, frame_id: u64) -> impl Iterator<Item = u64> + '_ {
        self.widgets
            .get(&frame_id)
            .into_iter()
            .flat_map(|frame| frame.anchors.iter())
            .filter_map(|anchor| anchor.relative_to_id.map(|target_id| target_id as u64))
    }

    /// Record that `frame_id` is anchored to `target_id`.
    pub fn add_anchor_dependent(&mut self, target_id: u64, frame_id: u64) {
        self.anchor_dependents
            .entry(target_id)
            .or_default()
            .insert(frame_id);
    }

    /// Remove `frame_id` from `target_id`'s dependents.
    pub fn remove_anchor_dependent(&mut self, target_id: u64, frame_id: u64) {
        if let Some(set) = self.anchor_dependents.get_mut(&target_id) {
            set.remove(&frame_id);
            if set.is_empty() {
                self.anchor_dependents.remove(&target_id);
            }
        }
    }

    /// Remove `frame_id` from all reverse-index entries by reading its current
    /// anchors to find the targets.
    pub fn remove_all_anchor_dependents_for(&mut self, frame_id: u64) {
        let targets: Vec<u64> = self
            .widgets
            .get(&frame_id)
            .map(|f| {
                f.anchors
                    .iter()
                    .filter_map(|a| a.relative_to_id.map(|t| t as u64))
                    .collect()
            })
            .unwrap_or_default();
        for target in targets {
            self.remove_anchor_dependent(target, frame_id);
        }
    }

    /// Get frame IDs anchored to `target_id`.
    pub fn get_anchor_dependents(&self, target_id: u64) -> Option<&HashSet<u64>> {
        self.anchor_dependents.get(&target_id)
    }

    /// Rebuild the reverse anchor index from all existing anchors.
    /// Call once after initial load to capture anchors set during XML parsing
    /// and frame creation.
    pub fn rebuild_anchor_index(&mut self) {
        self.anchor_dependents.clear();
        let entries: Vec<(u64, u64)> = self
            .widgets
            .values()
            .flat_map(|f| {
                f.anchors
                    .iter()
                    .filter_map(move |a| a.relative_to_id.map(|target| (target as u64, f.id)))
            })
            .collect();
        for (target, frame_id) in entries {
            self.anchor_dependents
                .entry(target)
                .or_default()
                .insert(frame_id);
        }
    }

    fn record_visual_dirty(&self, id: u64) {
        self.render_dirty_ids.borrow_mut().insert(id);
        let Some(source) = self.current_dirty_source.borrow().clone() else {
            return;
        };
        self.render_dirty_sources
            .borrow_mut()
            .entry(id)
            .or_default()
            .insert(source);
    }
}

fn hash_set_u64_bytes(values: &HashSet<u64>) -> usize {
    values.capacity() * std::mem::size_of::<u64>()
}

fn dirty_source_bytes(value: &Option<RenderDirtySource>) -> usize {
    value.as_ref().map_or(0, |source| source.method.capacity())
}

fn render_dirty_sources_bytes(values: &HashMap<u64, HashSet<RenderDirtySource>>) -> usize {
    values.capacity() * std::mem::size_of::<(u64, HashSet<RenderDirtySource>)>()
        + values
            .values()
            .map(render_dirty_source_set_bytes)
            .sum::<usize>()
}

fn render_dirty_source_set_bytes(values: &HashSet<RenderDirtySource>) -> usize {
    values.capacity() * std::mem::size_of::<RenderDirtySource>()
        + values
            .iter()
            .map(|source| source.method.capacity())
            .sum::<usize>()
}

fn hash_map_u64_hash_set_u64_bytes(values: &HashMap<u64, HashSet<u64>>) -> usize {
    values.capacity() * std::mem::size_of::<(u64, HashSet<u64>)>()
        + values.values().map(hash_set_u64_bytes).sum::<usize>()
}
