use crate::widget::WidgetRegistry;
use std::collections::HashSet;

pub(super) fn should_trace_strata_invalidations(start_time: &std::time::Instant) -> bool {
    if std::env::var_os("WOW_SIM_TRACE_STRATA_INVALIDATIONS").is_none() {
        return false;
    }

    let after_ms = std::env::var("WOW_SIM_TRACE_STRATA_INVALIDATIONS_AFTER_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok());
    match after_ms {
        Some(ms) => start_time.elapsed() >= std::time::Duration::from_millis(ms),
        None => true,
    }
}

pub(super) fn uses_parent_alpha_fallback(frame: &crate::widget::Frame) -> bool {
    matches!(
        frame.parent_key.as_deref(),
        Some("NormalTexture" | "PushedTexture" | "HighlightTexture" | "DisabledTexture")
    )
}

pub(super) fn is_region(wt: crate::widget::WidgetType) -> bool {
    matches!(
        wt,
        crate::widget::WidgetType::Texture
            | crate::widget::WidgetType::FontString
            | crate::widget::WidgetType::Line
    )
}

pub(super) fn effective_frame_level(frame: &crate::widget::Frame) -> i32 {
    frame.frame_level.saturating_add(frame.raise_order)
}

pub(super) fn is_strata_root_boundary(frame: &crate::widget::Frame) -> bool {
    matches!(frame.name.as_deref(), Some("UIParent" | "WorldFrame"))
}

/// DFS emit: parent frame, then its Texture regions (sorted by draw_layer),
/// then child frames (recursively), then its FontString regions.
///
/// FontStrings are deferred past child frames so that parent text renders on top
/// of child frame backgrounds. In WoW's flat render model, all regions at the
/// same frame_level are interleaved by draw_layer, so we approximate this by
/// splitting regions into textures before children and fontstrings after.
pub(super) fn dfs_emit(
    id: u64,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    out: &mut Vec<u64>,
) {
    let Some(frame) = widgets.get(id) else {
        return;
    };
    out.push(id);

    let (mut regions, mut child_frames) = partition_children(frame, strata_idx, widgets, visible);
    sort_regions(&mut regions, widgets);

    let split = regions.partition_point(|&region_id| {
        widgets
            .get(region_id)
            .is_none_or(|region| region.widget_type != crate::widget::WidgetType::FontString)
    });
    let (texture_regions, fontstring_regions) = regions.split_at(split);
    out.extend_from_slice(texture_regions);

    sort_child_frames(&mut child_frames, widgets);
    for child_id in child_frames {
        dfs_emit(child_id, strata_idx, widgets, visible, out);
    }

    out.extend_from_slice(fontstring_regions);
}

fn partition_children(
    frame: &crate::widget::Frame,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
) -> (Vec<u64>, Vec<u64>) {
    let mut regions = Vec::new();
    let mut child_frames = Vec::new();
    for &child_id in &frame.children {
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

pub(super) fn collect_same_strata_subtree_ids(
    id: u64,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    out: &mut HashSet<u64>,
) {
    if !out.insert(id) {
        return;
    }
    let Some(frame) = widgets.get(id) else {
        return;
    };
    for &child_id in &frame.children {
        let Some(child) = widgets.get(child_id) else {
            continue;
        };
        if is_region(child.widget_type) {
            out.insert(child_id);
            continue;
        }
        if child.frame_strata.as_index() == strata_idx {
            collect_same_strata_subtree_ids(child_id, strata_idx, widgets, out);
        }
    }
}

pub(super) fn same_strata_subtree_segment_end(
    bucket: &[u64],
    start: usize,
    subtree_ids: &HashSet<u64>,
) -> usize {
    let mut end = start + 1;
    while end < bucket.len() && subtree_ids.contains(&bucket[end]) {
        end += 1;
    }
    end
}

fn sort_regions(regions: &mut [u64], widgets: &WidgetRegistry) {
    use std::cmp::Reverse;

    regions.sort_by(|&a, &b| {
        let (frame_a, frame_b) = match (widgets.get(a), widgets.get(b)) {
            (Some(frame_a), Some(frame_b)) => (frame_a, frame_b),
            _ => return a.cmp(&b),
        };
        let type_flag = |frame: &crate::widget::Frame| -> u8 {
            u8::from(frame.widget_type == crate::widget::WidgetType::FontString)
        };
        (
            frame_a.draw_layer as i32,
            frame_a.draw_sub_layer,
            type_flag(frame_a),
            Reverse(a),
        )
            .cmp(&(
                frame_b.draw_layer as i32,
                frame_b.draw_sub_layer,
                type_flag(frame_b),
                Reverse(b),
            ))
    });
}

fn sort_child_frames(frames: &mut [u64], widgets: &WidgetRegistry) {
    frames.sort_by(|&a, &b| match (widgets.get(a), widgets.get(b)) {
        (Some(frame_a), Some(frame_b)) => (
            effective_frame_level(frame_a),
            frame_a.frame_level,
            frame_a.raise_order,
            a,
        )
            .cmp(&(
                effective_frame_level(frame_b),
                frame_b.frame_level,
                frame_b.raise_order,
                b,
            )),
        _ => a.cmp(&b),
    });
}

#[cfg(test)]
mod tests {
    use super::same_strata_subtree_segment_end;
    use std::collections::HashSet;

    #[test]
    fn same_strata_subtree_segment_end_stops_at_first_non_subtree_id() {
        let bucket = vec![10, 11, 12, 99, 13];
        let subtree_ids = HashSet::from([10, 11, 12, 13]);

        assert_eq!(same_strata_subtree_segment_end(&bucket, 0, &subtree_ids), 3);
    }
}
