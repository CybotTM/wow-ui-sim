//! Frame collection and sorting helpers for rendering.

use std::sync::LazyLock;

use rustc_hash::FxHashSet;

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

static HIT_TEST_EXCLUDED_NAMES: LazyLock<FxHashSet<&'static str>> =
    LazyLock::new(|| HIT_TEST_EXCLUDED.iter().copied().collect());

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
) -> FxHashSet<u64> {
    let mut ids = FxHashSet::default();
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
pub type IntraStrataKey = (i32, i32, u64, u8, i32, i32, u8, u64);

fn effective_frame_level(frame: &crate::widget::Frame) -> i32 {
    frame.frame_level.saturating_add(frame.raise_order)
}

/// Intra-strata sort key for rendering order within the same frame strata.
///
/// In WoW, regions (Texture/FontString) render as part of their parent frame,
/// not independently. Regions use their parent's frame_level and group with
/// their parent via `parent_id`, ensuring all regions of a frame render
/// immediately after that frame (before any higher-level content).
///
/// Non-regions sort by effective raised level `(frame_level + raise_order)`.
/// `Raise()` changes `raise_order` without mutating `frame_level`, so render
/// ordering needs the combined value to match WoW's `GetRaisedFrameLevel()`.
/// Regions follow the same parent effective level within the same parent draw
/// layer so later-created overlays do not get buried under earlier background
/// textures. FontStrings (type_flag=1) render above Textures (type_flag=0) in
/// the same draw layer per WoW rules.
pub fn intra_strata_sort_key(
    f: &crate::widget::Frame,
    id: u64,
    registry: &crate::widget::WidgetRegistry,
) -> IntraStrataKey {
    if matches!(
        f.widget_type,
        WidgetType::Texture | WidgetType::FontString | WidgetType::Line
    ) {
        let (parent_level, parent_frame_level, parent_id) = f
            .parent_id
            .and_then(|pid| {
                registry
                    .get(pid)
                    .map(|p| (effective_frame_level(p), p.frame_level, pid))
            })
            .unwrap_or((effective_frame_level(f), f.frame_level, id));
        let type_flag = if f.widget_type == WidgetType::FontString {
            1u8
        } else {
            0u8
        };
        (
            parent_level,
            parent_frame_level,
            parent_id,
            1,
            f.draw_layer as i32,
            f.draw_sub_layer,
            type_flag,
            id,
        )
    } else {
        (effective_frame_level(f), f.frame_level, id, 0, 0, 0, 0, 0)
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
    let mut hittable: Vec<HittableFrameEntry> = strata_buckets
        .iter()
        .flat_map(|bucket| bucket.iter().copied())
        .filter_map(|id| hittable_frame_entry(registry, id))
        .collect();

    hittable.sort_by(|a, b| {
        a.1.cmp(&b.1)
            .then_with(|| a.2.cmp(&b.2))
            .then_with(|| a.3.cmp(&b.3))
            .then_with(|| a.0.cmp(&b.0))
    });

    CollectedFrames {
        hittable: hittable
            .into_iter()
            .map(|(id, _, _, _, r)| (id, r))
            .collect(),
    }
}

type HittableFrameEntry = (u64, FrameStrata, i32, i32, crate::LayoutRect);

fn hittable_frame_entry(
    registry: &crate::widget::WidgetRegistry,
    id: u64,
) -> Option<HittableFrameEntry> {
    let frame = registry.get(id)?;
    if !crate::layout::frame_has_render_layout(registry, id) {
        return None;
    }
    let rect = frame.layout_rect?;

    is_frame_hittable(frame).then(|| {
        (
            id,
            frame.frame_strata,
            effective_frame_level(frame),
            frame.frame_level,
            rect,
        )
    })
}

fn is_frame_hittable(frame: &crate::widget::Frame) -> bool {
    frame.visible
        && frame.effective_alpha > 0.0
        && (frame.mouse_enabled || matches!(frame.widget_type, WidgetType::EditBox))
        && !is_hit_test_excluded(frame)
}

fn is_hit_test_excluded(frame: &crate::widget::Frame) -> bool {
    frame
        .name
        .as_deref()
        .is_some_and(|name| HIT_TEST_EXCLUDED_NAMES.contains(name))
}

pub fn frame_accepts_mouse_button(frame: &crate::widget::Frame, button_name: &str) -> bool {
    let mouse_enabled = frame.mouse_enabled || matches!(frame.widget_type, WidgetType::EditBox);
    mouse_enabled
        && !frame
            .pass_through_buttons
            .contains(&button_name.to_ascii_lowercase())
        && frame_has_registered_mouse_button(frame, button_name)
}

fn frame_has_registered_mouse_button(frame: &crate::widget::Frame, button_name: &str) -> bool {
    if frame.registered_mouse_buttons.is_empty() {
        return true;
    }

    frame_mouse_registration_matches(frame, button_name, true)
        || frame_mouse_registration_matches(frame, button_name, false)
}

pub fn frame_mouse_registration_matches(
    frame: &crate::widget::Frame,
    button_name: &str,
    down: bool,
) -> bool {
    if frame.registered_mouse_buttons.is_empty() {
        return true;
    }

    let edge = if down { "Down" } else { "Up" };
    frame
        .registered_mouse_buttons
        .contains(&format!("{button_name}{edge}"))
        || frame
            .registered_mouse_buttons
            .contains(&format!("Any{edge}"))
}

#[cfg(test)]
mod tests {
    use super::intra_strata_sort_key;
    use crate::widget::{Frame, WidgetRegistry, WidgetType};

    #[test]
    fn excluded_overlay_names_are_not_hittable() {
        let mut registry = WidgetRegistry::new();
        let excluded_id = register_hittable_frame(&mut registry, "UIParent", 10);
        let included_id = register_hittable_frame(&mut registry, "ClickableFrame", 20);

        let strata_buckets = vec![vec![excluded_id, included_id]];
        let collected = super::collect_hittable_frames(&registry, &strata_buckets);
        let collected_ids: Vec<u64> = collected
            .hittable
            .into_iter()
            .map(|(id, _rect)| id)
            .collect();

        assert_eq!(collected_ids, vec![included_id]);
    }

    #[test]
    fn unanchored_frames_are_not_hittable_at_parent_origin() {
        let mut registry = WidgetRegistry::new();
        let mut parent = Frame::new(WidgetType::Frame, Some("UIParent".to_string()), None);
        parent.id = 1;
        parent.layout_rect = Some(crate::LayoutRect {
            x: 0.0,
            y: 0.0,
            width: 1600.0,
            height: 1200.0,
        });
        registry.register(parent);

        let child = register_hittable_frame(&mut registry, "UnanchoredPanel", 0);
        registry.get_mut(child).unwrap().parent_id = Some(1);
        registry.add_child(1, child);

        let strata_buckets = vec![vec![child]];
        let collected = super::collect_hittable_frames(&registry, &strata_buckets);

        assert!(
            collected.hittable.is_empty(),
            "visible unanchored frames should not be mouse targets at parent origin"
        );
    }

    #[test]
    fn later_created_regions_sort_after_earlier_regions_in_same_layer() {
        let mut registry = WidgetRegistry::new();

        let parent = Frame::new(WidgetType::Frame, Some("Parent".to_string()), None);
        let parent_id = parent.id;
        registry.register(parent);

        let first = Frame::new(
            WidgetType::Texture,
            Some("First".to_string()),
            Some(parent_id),
        );
        let first_id = first.id;
        registry.register(first);
        registry.add_child(parent_id, first_id);

        let second = Frame::new(
            WidgetType::Texture,
            Some("Second".to_string()),
            Some(parent_id),
        );
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

    fn register_hittable_frame(registry: &mut WidgetRegistry, name: &str, x: i32) -> u64 {
        let mut frame = Frame::new(WidgetType::Frame, Some(name.to_string()), None);
        frame.mouse_enabled = true;
        frame.layout_rect = Some(crate::LayoutRect {
            x: x as f32,
            y: 0.0,
            width: 20.0,
            height: 20.0,
        });
        let id = frame.id;
        registry.register(frame);
        id
    }
}
