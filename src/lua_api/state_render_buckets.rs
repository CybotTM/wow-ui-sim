use crate::widget::WidgetRegistry;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RegionEntry {
    depth: u32,
    id: u64,
}

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

    let mut regions = Vec::new();
    let mut child_frames = Vec::new();
    collect_frame_regions(
        frame,
        strata_idx,
        widgets,
        visible,
        0,
        &mut regions,
        &mut child_frames,
    );
    sort_regions(&mut regions, widgets);
    out.extend(regions.into_iter().map(|entry| entry.id));

    sort_child_frames(&mut child_frames, widgets);
    for child_id in child_frames {
        let child = widgets.get(child_id);
        let suppress_regions = child.is_some_and(is_transparent_wrapper);
        dfs_emit_with_region_mode(
            child_id,
            strata_idx,
            widgets,
            visible,
            out,
            suppress_regions,
        );
    }
}

fn dfs_emit_with_region_mode(
    id: u64,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    out: &mut Vec<u64>,
    suppress_regions: bool,
) {
    let Some(frame) = widgets.get(id) else {
        return;
    };
    out.push(id);

    let mut regions = Vec::new();
    let mut child_frames = Vec::new();
    if !suppress_regions {
        collect_frame_regions(
            frame,
            strata_idx,
            widgets,
            visible,
            0,
            &mut regions,
            &mut child_frames,
        );
        sort_regions(&mut regions, widgets);
        out.extend(regions.into_iter().map(|entry| entry.id));
    } else {
        collect_frame_children_only(frame, strata_idx, widgets, visible, &mut child_frames);
    }

    sort_child_frames(&mut child_frames, widgets);
    for child_id in child_frames {
        let child = widgets.get(child_id);
        let child_suppress = child.is_some_and(is_transparent_wrapper);
        dfs_emit_with_region_mode(child_id, strata_idx, widgets, visible, out, child_suppress);
    }
}

fn collect_frame_regions(
    frame: &crate::widget::Frame,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    depth: u32,
    regions: &mut Vec<RegionEntry>,
    child_frames: &mut Vec<u64>,
) {
    collect_frame_regions_inner(
        frame,
        strata_idx,
        widgets,
        visible,
        depth,
        regions,
        child_frames,
    );
}

fn collect_frame_regions_inner(
    frame: &crate::widget::Frame,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    depth: u32,
    regions: &mut Vec<RegionEntry>,
    child_frames: &mut Vec<u64>,
) {
    for &child_id in &frame.children {
        collect_child_for_emit(
            child_id,
            strata_idx,
            widgets,
            visible,
            depth,
            regions,
            child_frames,
        );
    }
}

fn collect_child_for_emit(
    child_id: u64,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    depth: u32,
    regions: &mut Vec<RegionEntry>,
    child_frames: &mut Vec<u64>,
) {
    if !visible.contains(&child_id) {
        return;
    }
    let Some(child) = widgets.get(child_id) else {
        return;
    };
    if is_region(child.widget_type) {
        regions.push(RegionEntry {
            depth,
            id: child_id,
        });
        return;
    }
    if child.frame_strata.as_index() == strata_idx {
        child_frames.push(child_id);
        if is_transparent_wrapper(child) {
            collect_transparent_wrapper_regions(
                child_id,
                strata_idx,
                widgets,
                visible,
                depth + 1,
                regions,
            );
        }
    }
}

fn collect_transparent_wrapper_regions(
    id: u64,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    depth: u32,
    regions: &mut Vec<RegionEntry>,
) {
    let Some(frame) = widgets.get(id) else {
        return;
    };
    for &child_id in &frame.children {
        if !visible.contains(&child_id) {
            continue;
        }
        let Some(child) = widgets.get(child_id) else {
            continue;
        };
        if is_region(child.widget_type) {
            regions.push(RegionEntry {
                depth,
                id: child_id,
            });
            continue;
        }
        if child.frame_strata.as_index() == strata_idx && is_transparent_wrapper(child) {
            collect_transparent_wrapper_regions(
                child_id,
                strata_idx,
                widgets,
                visible,
                depth + 1,
                regions,
            );
        }
    }
}

fn collect_frame_children_only(
    frame: &crate::widget::Frame,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    child_frames: &mut Vec<u64>,
) {
    for &child_id in &frame.children {
        if !visible.contains(&child_id) {
            continue;
        }
        let Some(child) = widgets.get(child_id) else {
            continue;
        };
        if is_region(child.widget_type) {
            continue;
        }
        if child.frame_strata.as_index() == strata_idx {
            child_frames.push(child_id);
        }
    }
}

fn is_transparent_wrapper(frame: &crate::widget::Frame) -> bool {
    matches!(
        frame.widget_type,
        crate::widget::WidgetType::Frame | crate::widget::WidgetType::ScrollFrame
    ) && !is_strata_root_boundary(frame)
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

fn sort_regions(regions: &mut [RegionEntry], widgets: &WidgetRegistry) {
    use std::cmp::Reverse;

    regions.sort_by(|a, b| {
        let (frame_a, frame_b) = match (widgets.get(a.id), widgets.get(b.id)) {
            (Some(frame_a), Some(frame_b)) => (frame_a, frame_b),
            _ => return a.id.cmp(&b.id),
        };
        let type_flag = |frame: &crate::widget::Frame| -> u8 {
            u8::from(frame.widget_type == crate::widget::WidgetType::FontString)
        };
        (
            a.depth,
            frame_a.draw_layer as i32,
            frame_a.draw_sub_layer,
            type_flag(frame_a),
            Reverse(a.id),
        )
            .cmp(&(
                b.depth,
                frame_b.draw_layer as i32,
                frame_b.draw_sub_layer,
                type_flag(frame_b),
                Reverse(b.id),
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
    use super::{collect_child_for_emit, same_strata_subtree_segment_end};
    use crate::widget::{Frame, FrameStrata, WidgetRegistry, WidgetType};
    use std::collections::HashSet;

    #[test]
    fn same_strata_subtree_segment_end_stops_at_first_non_subtree_id() {
        let bucket = vec![10, 11, 12, 99, 13];
        let subtree_ids = HashSet::from([10, 11, 12, 13]);

        assert_eq!(same_strata_subtree_segment_end(&bucket, 0, &subtree_ids), 3);
    }

    #[test]
    fn collect_child_for_emit_routes_regions_and_same_strata_frames() {
        let mut widgets = WidgetRegistry::default();
        widgets.register(Frame {
            id: 2,
            widget_type: WidgetType::Texture,
            ..Default::default()
        });
        widgets.register(Frame {
            id: 3,
            widget_type: WidgetType::Frame,
            frame_strata: FrameStrata::Medium,
            ..Default::default()
        });
        widgets.register(Frame {
            id: 4,
            widget_type: WidgetType::Frame,
            frame_strata: FrameStrata::High,
            ..Default::default()
        });
        widgets.register(Frame {
            id: 5,
            widget_type: WidgetType::Texture,
            ..Default::default()
        });

        let visible = HashSet::from([2, 3, 4]);
        let mut regions = Vec::new();
        let mut child_frames = Vec::new();

        for child_id in [2, 3, 4, 5] {
            collect_child_for_emit(
                child_id,
                FrameStrata::Medium.as_index(),
                &widgets,
                &visible,
                0,
                &mut regions,
                &mut child_frames,
            );
        }

        assert_eq!(
            regions.iter().map(|entry| entry.id).collect::<Vec<_>>(),
            vec![2]
        );
        assert_eq!(child_frames, vec![3]);
    }
}
