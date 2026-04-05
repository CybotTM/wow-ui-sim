//! Strata rendering, layout, and visibility methods for SimState.

use crate::widget::WidgetRegistry;
use std::collections::HashSet;

use super::state::SimState;

fn uses_parent_alpha_fallback(frame: &crate::widget::Frame) -> bool {
    matches!(
        frame.parent_key.as_deref(),
        Some("NormalTexture" | "PushedTexture" | "HighlightTexture" | "DisabledTexture")
    )
}

fn is_region(wt: crate::widget::WidgetType) -> bool {
    matches!(
        wt,
        crate::widget::WidgetType::Texture
            | crate::widget::WidgetType::FontString
            | crate::widget::WidgetType::Line
    )
}

/// DFS emit: parent frame, then its regions (sorted by draw_layer), then child
/// frames (sorted by frame_level/raise_order/id, recursively).
/// This ensures all descendants of a frame render as a contiguous group.
fn dfs_emit(
    id: u64,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    out: &mut Vec<u64>,
) {
    let Some(f) = widgets.get(id) else { return };
    out.push(id);

    let (mut regions, mut child_frames) = partition_children(f, strata_idx, widgets, visible);
    sort_regions(&mut regions, widgets);
    out.extend(regions);

    sort_child_frames(&mut child_frames, widgets);
    for child_id in child_frames {
        dfs_emit(child_id, strata_idx, widgets, visible, out);
    }
}

/// Split visible children into regions (textures/fontstrings) and child frames in same strata.
fn partition_children(
    f: &crate::widget::Frame,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
) -> (Vec<u64>, Vec<u64>) {
    let mut regions = Vec::new();
    let mut child_frames = Vec::new();
    for &child_id in &f.children {
        if !visible.contains(&child_id) {
            continue;
        }
        let Some(child) = widgets.get(child_id) else {
            continue;
        };
        if is_region(child.widget_type) {
            regions.push(child_id);
        } else if child.frame_strata.as_index() == strata_idx {
            child_frames.push(child_id);
        }
    }
    (regions, child_frames)
}

/// Sort regions by (draw_layer, draw_sub_layer, type_flag, id).
/// FontStrings sort after Textures within the same layer (type_flag=1 vs 0).
fn sort_regions(regions: &mut [u64], widgets: &WidgetRegistry) {
    regions.sort_by(|&a, &b| {
        let (fa, fb) = match (widgets.get(a), widgets.get(b)) {
            (Some(fa), Some(fb)) => (fa, fb),
            _ => return a.cmp(&b),
        };
        let type_flag = |f: &crate::widget::Frame| -> u8 {
            u8::from(f.widget_type == crate::widget::WidgetType::FontString)
        };
        (fa.draw_layer as i32, fa.draw_sub_layer, type_flag(fa), a).cmp(&(
            fb.draw_layer as i32,
            fb.draw_sub_layer,
            type_flag(fb),
            b,
        ))
    });
}

/// Sort child frames by (frame_level, raise_order, id).
fn sort_child_frames(frames: &mut [u64], widgets: &WidgetRegistry) {
    frames.sort_by(|&a, &b| match (widgets.get(a), widgets.get(b)) {
        (Some(fa), Some(fb)) => {
            (fa.frame_level, fa.raise_order, a).cmp(&(fb.frame_level, fb.raise_order, b))
        }
        _ => a.cmp(&b),
    });
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
        // Step 1: Collect visible frame IDs per strata (unordered).
        let mut visible: HashSet<u64> = HashSet::new();
        let mut strata_map: Vec<Vec<u64>> = vec![Vec::new(); crate::widget::FrameStrata::COUNT];
        for id in self.widgets.iter_ids() {
            let Some(f) = self.widgets.get(id) else {
                continue;
            };
            if self.frame_render_alpha(f) <= 0.0 {
                continue;
            }
            visible.insert(id);
            let strata = self.frame_bucket_strata(f);
            strata_map[strata.as_index()].push(id);
        }

        // Step 2: For each strata, identify roots and DFS-emit in grouped order.
        let mut buckets = vec![Vec::new(); crate::widget::FrameStrata::COUNT];
        for (si, ids) in strata_map.iter().enumerate() {
            let mut roots = self.find_strata_roots(ids, si, &visible);
            self.sort_by_frame_level(&mut roots);
            let bucket = &mut buckets[si];
            for root_id in roots {
                dfs_emit(root_id, si, &self.widgets, &visible, bucket);
            }
        }
        buckets
    }

    /// Find root frames in a strata: no parent, parent in different strata, or parent not visible.
    fn find_strata_roots(
        &self,
        ids: &[u64],
        strata_idx: usize,
        visible: &HashSet<u64>,
    ) -> Vec<u64> {
        ids.iter()
            .copied()
            .filter(|&id| {
                let Some(f) = self.widgets.get(id) else {
                    return false;
                };
                if is_region(f.widget_type) {
                    return false;
                }
                match f.parent_id {
                    None => true,
                    Some(pid) => {
                        let same_strata = self.widgets.get(pid).map_or(false, |p| {
                            self.frame_bucket_strata(p).as_index() == strata_idx
                        });
                        !same_strata || !visible.contains(&pid)
                    }
                }
            })
            .collect()
    }

    /// Sort frame IDs by (frame_level, raise_order, id).
    fn sort_by_frame_level(&self, ids: &mut [u64]) {
        ids.sort_by(|&a, &b| {
            let fa = self.widgets.get(a);
            let fb = self.widgets.get(b);
            match (fa, fb) {
                (Some(fa), Some(fb)) => {
                    (fa.frame_level, fa.raise_order, a).cmp(&(fb.frame_level, fb.raise_order, b))
                }
                _ => a.cmp(&b),
            }
        });
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

    pub(crate) fn recompute_layout_subtree(
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
    pub(crate) fn recompute_anchor_dependents(
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
            self.invalidate_strata_buckets();
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
        let mut subtree = HashSet::new();
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

    /// Invalidate strata buckets so they rebuild on next access.
    /// Used after show/reparent operations that change DFS traversal order.
    pub(crate) fn invalidate_strata_buckets(&mut self) {
        self.strata_buckets = None;
    }

    pub(crate) fn frame_render_alpha(&self, frame: &crate::widget::Frame) -> f32 {
        if frame.effective_alpha > 0.0 {
            return frame.effective_alpha;
        }
        if frame.alpha > 0.0 && uses_parent_alpha_fallback(frame) {
            return frame
                .parent_id
                .and_then(|parent_id| self.widgets.get(parent_id))
                .map(|parent| parent.effective_alpha)
                .unwrap_or(0.0);
        }
        0.0
    }

    pub(crate) fn frame_bucket_strata(
        &self,
        frame: &crate::widget::Frame,
    ) -> crate::widget::FrameStrata {
        use crate::widget::WidgetType;

        if matches!(
            frame.widget_type,
            WidgetType::Texture | WidgetType::FontString | WidgetType::Line
        ) {
            return frame
                .parent_id
                .and_then(|parent_id| self.widgets.get(parent_id))
                .map(|parent| parent.frame_strata)
                .unwrap_or(frame.frame_strata);
        }
        frame.frame_strata
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
            self.invalidate_strata_buckets();
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
            self.invalidate_strata_buckets();
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
