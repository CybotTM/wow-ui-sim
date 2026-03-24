//! Frame collection and sorting helpers for rendering.

use crate::widget::{FrameStrata, WidgetType};

/// Frame names excluded from hit testing (full-screen or non-interactive overlays).
pub const HIT_TEST_EXCLUDED: &[&str] = &[
    "UIParent",
    "Minimap",
    "WorldFrame",
    "DEFAULT_CHAT_FRAME",
    "ChatFrame1",
    "EventToastManagerFrame",
    "EditModeManagerFrame",
];

/// Result of collecting frames for hit testing.
///
/// Rects are in unscaled WoW coordinates (caller applies UI_SCALE).
pub struct CollectedFrames {
    /// Frames eligible for hit testing, sorted by strata/level/id (low to high).
    pub hittable: Vec<(u64, crate::LayoutRect)>,
}

/// Collect all frame IDs in the subtree rooted at the named frame.
pub fn collect_subtree_ids(
    registry: &crate::widget::WidgetRegistry,
    root_name: &str,
) -> std::collections::HashSet<u64> {
    let mut ids = std::collections::HashSet::new();
    let root_id = registry.iter_ids().find(|&id| {
        registry
            .get(id)
            .map(|f| f.name.as_deref() == Some(root_name))
            .unwrap_or(false)
    });
    if let Some(root_id) = root_id {
        let mut queue = vec![root_id];
        while let Some(id) = queue.pop() {
            ids.insert(id);
            if let Some(f) = registry.get(id) {
                queue.extend(f.children.iter().copied());
            }
        }
    }
    ids
}

/// Sort key type for frame rendering order within a strata bucket.
pub type IntraStrataKey = (
    i32,
    i32,
    u64,
    u8,
    i32,
    i32,
    u8,
    u64,
);

/// Intra-strata sort key for rendering order within the same frame strata.
///
/// In WoW, regions (Texture/FontString) render as part of their parent frame,
/// not independently. Regions use their parent's frame_level and group with
/// their parent via `parent_id`, ensuring all regions of a frame render
/// immediately after that frame (before any higher-level content).
///
/// Non-regions sort by `(frame_level, raise_order, id)` — higher frame_level
/// renders on top; within the same level, raise_order (adjusted by
/// Raise()/Lower()) breaks ties; within the same raise_order, later-created
/// frames render on top. Regions follow the same rule within the same parent
/// draw layer so later-created overlays do not get buried under earlier
/// background textures. FontStrings (type_flag=1) render above Textures
/// (type_flag=0) in the same draw layer per WoW rules.
pub fn intra_strata_sort_key(
    f: &crate::widget::Frame,
    id: u64,
    registry: &crate::widget::WidgetRegistry,
) -> IntraStrataKey {
    if matches!(
        f.widget_type,
        WidgetType::Texture | WidgetType::FontString | WidgetType::Line
    ) {
        let (parent_level, parent_raise_order, parent_id) = f
            .parent_id
            .and_then(|pid| {
                registry
                    .get(pid)
                    .map(|p| (p.frame_level, p.raise_order, pid))
            })
            .unwrap_or((f.frame_level, f.raise_order, id));
        let type_flag = if f.widget_type == WidgetType::FontString {
            1u8
        } else {
            0u8
        };
        (
            parent_level,
            parent_raise_order,
            parent_id,
            1,
            f.draw_layer as i32,
            f.draw_sub_layer,
            type_flag,
            id,
        )
    } else {
        (f.frame_level, f.raise_order, id, 0, 0, 0, 0, 0)
    }
}

/// Build a hit-test list from visible-only strata buckets.
///
/// Returns visible, mouse-enabled frames sorted by strata/level/id,
/// excluding non-interactive overlays.
pub fn collect_hittable_frames(
    registry: &crate::widget::WidgetRegistry,
    strata_buckets: &[Vec<u64>],
) -> CollectedFrames {
    let mut hittable: Vec<(u64, FrameStrata, i32, crate::LayoutRect)> = Vec::new();

    for bucket in strata_buckets {
        for &id in bucket {
            let Some(f) = registry.get(id) else { continue };
            let Some(rect) = f.layout_rect else { continue };
            if f.visible
                && f.effective_alpha > 0.0
                && f.mouse_enabled
                && !f
                    .name
                    .as_deref()
                    .is_some_and(|n| HIT_TEST_EXCLUDED.contains(&n))
            {
                hittable.push((id, f.frame_strata, f.frame_level, rect));
            }
        }
    }

    hittable.sort_by(|a, b| {
        a.1.cmp(&b.1)
            .then_with(|| a.2.cmp(&b.2))
            .then_with(|| a.0.cmp(&b.0))
    });

    CollectedFrames {
        hittable: hittable.into_iter().map(|(id, _, _, r)| (id, r)).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::intra_strata_sort_key;
    use crate::widget::{Frame, WidgetRegistry, WidgetType};

    #[test]
    fn later_created_regions_sort_after_earlier_regions_in_same_layer() {
        let mut registry = WidgetRegistry::new();

        let parent = Frame::new(WidgetType::Frame, Some("Parent".to_string()), None);
        let parent_id = parent.id;
        registry.register(parent);

        let first = Frame::new(WidgetType::Texture, Some("First".to_string()), Some(parent_id));
        let first_id = first.id;
        registry.register(first);
        registry.add_child(parent_id, first_id);

        let second = Frame::new(WidgetType::Texture, Some("Second".to_string()), Some(parent_id));
        let second_id = second.id;
        registry.register(second);
        registry.add_child(parent_id, second_id);

        let first_key = intra_strata_sort_key(registry.get(first_id).unwrap(), first_id, &registry);
        let second_key =
            intra_strata_sort_key(registry.get(second_id).unwrap(), second_id, &registry);

        assert!(
            first_key < second_key,
            "later-created texture should sort later/on top within the same parent layer"
        );
    }
}
