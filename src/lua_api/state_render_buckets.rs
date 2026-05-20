use crate::widget::WidgetRegistry;
use std::collections::HashSet;

#[path = "state_render_buckets/trace.rs"]
mod trace;

pub(super) use trace::should_trace_strata_invalidations;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RegionEntry {
    depth: u32,
    id: u64,
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
/// EditBox is the exception: its internal text/caret are emitted by the frame
/// itself, so child regions must come first or opaque backgrounds cover input.
pub(super) fn dfs_emit(
    id: u64,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    out: &mut Vec<u64>,
) {
    dfs_emit_with_region_mode(id, strata_idx, widgets, visible, out, false);
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
    let tooltip_nineslice_id = tooltip_nineslice_child(frame, strata_idx, widgets, visible);
    emit_tooltip_nineslice(
        tooltip_nineslice_id,
        strata_idx,
        widgets,
        visible,
        out,
        suppress_regions,
    );

    let (regions, mut child_frames) =
        collect_regions_and_children(frame, strata_idx, widgets, visible, suppress_regions);
    if let Some(nineslice_id) = tooltip_nineslice_id {
        child_frames.retain(|&child_id| child_id != nineslice_id);
    }

    let region_placement = region_placement_for_frame(frame, suppress_regions);
    emit_frame_with_regions(
        id,
        regions,
        &mut child_frames,
        RegionEmitContext {
            strata_idx,
            widgets,
            visible,
            out,
            suppress_regions,
        },
        region_placement,
    );
}

fn emit_tooltip_nineslice(
    tooltip_nineslice_id: Option<u64>,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    out: &mut Vec<u64>,
    suppress_regions: bool,
) {
    if let Some(nineslice_id) = tooltip_nineslice_id {
        dfs_emit_with_region_mode(
            nineslice_id,
            strata_idx,
            widgets,
            visible,
            out,
            suppress_regions,
        );
    }
}

fn region_placement_for_frame(
    frame: &crate::widget::Frame,
    suppress_regions: bool,
) -> RegionPlacement {
    if !suppress_regions && frame.widget_type == crate::widget::WidgetType::EditBox {
        return RegionPlacement::BeforeFrame;
    }
    RegionPlacement::AfterFrame
}

#[derive(Clone, Copy)]
enum RegionPlacement {
    BeforeFrame,
    AfterFrame,
}

struct RegionEmitContext<'a> {
    strata_idx: usize,
    widgets: &'a WidgetRegistry,
    visible: &'a HashSet<u64>,
    out: &'a mut Vec<u64>,
    suppress_regions: bool,
}

fn emit_frame_with_regions(
    id: u64,
    regions: Vec<RegionEntry>,
    child_frames: &mut [u64],
    ctx: RegionEmitContext<'_>,
    placement: RegionPlacement,
) {
    let mut deferred_font_regions =
        emit_regions_around_frame(id, regions, ctx.widgets, ctx.out, placement);
    emit_child_frames_and_hoisted(
        child_frames,
        ctx.strata_idx,
        ctx.widgets,
        ctx.visible,
        ctx.out,
        ctx.suppress_regions,
        &mut deferred_font_regions,
    );
    sort_regions(&mut deferred_font_regions, ctx.widgets);
    ctx.out
        .extend(deferred_font_regions.into_iter().map(|entry| entry.id));
}

fn emit_regions_around_frame(
    id: u64,
    regions: Vec<RegionEntry>,
    widgets: &WidgetRegistry,
    out: &mut Vec<u64>,
    placement: RegionPlacement,
) -> Vec<RegionEntry> {
    match placement {
        RegionPlacement::BeforeFrame => {
            emit_all_regions(regions, widgets, out);
            out.push(id);
            Vec::new()
        }
        RegionPlacement::AfterFrame => {
            out.push(id);
            emit_immediate_regions(regions, widgets, out)
        }
    }
}

fn tooltip_nineslice_child(
    frame: &crate::widget::Frame,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
) -> Option<u64> {
    if frame.widget_type != crate::widget::WidgetType::GameTooltip {
        return None;
    }
    let nineslice_id = *frame.children_keys.get("NineSlice")?;
    if !visible.contains(&nineslice_id) {
        return None;
    }
    let child = widgets.get(nineslice_id)?;
    (child.frame_strata.as_index() == strata_idx).then_some(nineslice_id)
}

fn collect_regions_and_children(
    frame: &crate::widget::Frame,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    suppress_regions: bool,
) -> (Vec<RegionEntry>, Vec<u64>) {
    let mut regions = Vec::new();
    let mut child_frames = Vec::new();
    if suppress_regions {
        collect_frame_children_only(frame, strata_idx, widgets, visible, &mut child_frames);
    } else {
        collect_frame_regions(
            frame,
            strata_idx,
            widgets,
            visible,
            0,
            &mut regions,
            &mut child_frames,
        );
    }
    (regions, child_frames)
}

fn emit_immediate_regions(
    mut regions: Vec<RegionEntry>,
    widgets: &WidgetRegistry,
    out: &mut Vec<u64>,
) -> Vec<RegionEntry> {
    sort_regions(&mut regions, widgets);
    let (immediate_regions, deferred_regions) = split_font_regions(regions, widgets);
    out.extend(immediate_regions.into_iter().map(|entry| entry.id));
    deferred_regions
}

fn emit_all_regions(mut regions: Vec<RegionEntry>, widgets: &WidgetRegistry, out: &mut Vec<u64>) {
    sort_regions(&mut regions, widgets);
    out.extend(regions.into_iter().map(|entry| entry.id));
}

fn emit_child_frames_and_hoisted(
    child_frames: &mut [u64],
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
    out: &mut Vec<u64>,
    suppress_regions: bool,
    deferred_regions: &mut Vec<RegionEntry>,
) {
    sort_child_frames(child_frames, widgets);
    for &child_id in child_frames.iter() {
        let child = widgets.get(child_id);
        let child_suppress = child.is_some_and(|frame| {
            is_regionless_transparent_wrapper(frame, strata_idx, widgets, visible)
        });
        if !suppress_regions && child_suppress {
            let mut hoisted_regions = Vec::new();
            collect_transparent_wrapper_regions(
                child_id,
                strata_idx,
                widgets,
                visible,
                1,
                &mut hoisted_regions,
            );
            let deferred_hoisted = emit_immediate_regions(hoisted_regions, widgets, out);
            deferred_regions.extend(deferred_hoisted);
        }
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

fn is_regionless_transparent_wrapper(
    frame: &crate::widget::Frame,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
) -> bool {
    is_transparent_wrapper(frame)
        && !has_visible_same_strata_region_child(frame, strata_idx, widgets, visible)
}

fn has_visible_same_strata_region_child(
    frame: &crate::widget::Frame,
    strata_idx: usize,
    widgets: &WidgetRegistry,
    visible: &HashSet<u64>,
) -> bool {
    frame.children.iter().copied().any(|child_id| {
        visible.contains(&child_id)
            && widgets.get(child_id).is_some_and(|child| {
                is_region(child.widget_type) && child.frame_strata.as_index() == strata_idx
            })
    })
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
    regions.sort_by(|a, b| {
        let (frame_a, frame_b) = match (widgets.get(a.id), widgets.get(b.id)) {
            (Some(frame_a), Some(frame_b)) => (frame_a, frame_b),
            _ => return a.id.cmp(&b.id),
        };
        match region_sort_key(*a, frame_a).cmp(&region_sort_key(*b, frame_b)) {
            std::cmp::Ordering::Equal if frame_a.parent_id == frame_b.parent_id => {
                (frame_a.region_order, a.id).cmp(&(frame_b.region_order, b.id))
            }
            std::cmp::Ordering::Equal => a.id.cmp(&b.id),
            structural_order => structural_order,
        }
    });
}

fn region_sort_key(entry: RegionEntry, frame: &crate::widget::Frame) -> (u32, i32, i32, u8) {
    let text_region = matches!(
        frame.widget_type,
        crate::widget::WidgetType::FontString | crate::widget::WidgetType::SimpleHTML
    ) as u8;
    (
        entry.depth,
        frame.draw_layer as i32,
        frame.draw_sub_layer,
        text_region,
    )
}

fn split_font_regions(
    regions: Vec<RegionEntry>,
    widgets: &WidgetRegistry,
) -> (Vec<RegionEntry>, Vec<RegionEntry>) {
    regions.into_iter().partition(|entry| {
        widgets
            .get(entry.id)
            .is_none_or(|frame| frame.widget_type != crate::widget::WidgetType::FontString)
    })
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
    use super::{collect_child_for_emit, dfs_emit, same_strata_subtree_segment_end};
    use crate::widget::{DrawLayer, Frame, FrameStrata, WidgetRegistry, WidgetType};
    use std::collections::HashSet;

    fn test_frame(id: u64, widget_type: WidgetType, parent_id: Option<u64>) -> Frame {
        Frame {
            id,
            widget_type,
            parent_id,
            frame_strata: FrameStrata::Medium,
            visible: true,
            ..Default::default()
        }
    }

    #[test]
    fn editbox_regions_render_before_internal_text_emitter() {
        let mut widgets = WidgetRegistry::default();
        widgets.register(test_frame(1, WidgetType::EditBox, None));
        widgets.register(Frame {
            draw_layer: DrawLayer::Background,
            ..test_frame(2, WidgetType::Texture, Some(1))
        });
        widgets.add_child(1, 2);

        let visible = HashSet::from([1, 2]);
        let mut emitted = Vec::new();
        dfs_emit(
            1,
            FrameStrata::Medium.as_index(),
            &widgets,
            &visible,
            &mut emitted,
        );

        assert_eq!(
            emitted,
            vec![2, 1],
            "EditBox child art must render below the EditBox internal text/caret emitter"
        );
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

    #[test]
    fn dfs_emit_renders_tooltip_nineslice_before_tooltip_frame() {
        let mut widgets = WidgetRegistry::default();

        let tooltip_id = 100;
        let nineslice_id = 101;
        let border_tex_id = 102;

        let mut tooltip = test_frame(tooltip_id, WidgetType::GameTooltip, None);
        tooltip.children = vec![nineslice_id];
        tooltip
            .children_keys
            .insert("NineSlice".to_string(), nineslice_id);
        widgets.register(tooltip);

        let mut nineslice = test_frame(nineslice_id, WidgetType::Frame, Some(tooltip_id));
        nineslice.children = vec![border_tex_id];
        widgets.register(nineslice);

        let border = test_frame(border_tex_id, WidgetType::Texture, Some(nineslice_id));
        widgets.register(border);

        let visible = HashSet::from([tooltip_id, nineslice_id, border_tex_id]);
        let mut out = Vec::new();
        dfs_emit(
            tooltip_id,
            crate::widget::FrameStrata::Medium.as_index(),
            &widgets,
            &visible,
            &mut out,
        );

        let tooltip_pos = out
            .iter()
            .position(|&id| id == tooltip_id)
            .expect("tooltip should be emitted");
        let nineslice_pos = out
            .iter()
            .position(|&id| id == nineslice_id)
            .expect("nineslice should be emitted");
        assert!(
            nineslice_pos < tooltip_pos,
            "NineSlice should render before tooltip frame so tooltip text stays on top"
        );
    }

    #[test]
    fn same_strata_subtree_segment_end_stops_at_first_non_subtree_id() {
        let bucket = vec![10, 11, 12, 99, 13];
        let subtree_ids = HashSet::from([10, 11, 12, 13]);

        assert_eq!(same_strata_subtree_segment_end(&bucket, 0, &subtree_ids), 3);
    }

    #[test]
    fn dfs_emit_keeps_transparent_wrapper_regions_after_wrapper_frame_and_parent_text_last() {
        let mut widgets = WidgetRegistry::default();
        widgets.register(test_frame(1, WidgetType::Frame, None));
        widgets.register(test_frame(2, WidgetType::Texture, Some(1)));
        widgets.register(test_frame(3, WidgetType::Frame, Some(1)));
        widgets.register(test_frame(4, WidgetType::Texture, Some(3)));
        widgets.register(test_frame(5, WidgetType::FontString, Some(1)));
        widgets.add_child(1, 2);
        widgets.add_child(1, 3);
        widgets.add_child(1, 5);
        widgets.add_child(3, 4);

        let visible = HashSet::from([1, 2, 3, 4, 5]);
        let mut bucket = Vec::new();

        dfs_emit(
            1,
            FrameStrata::Medium.as_index(),
            &widgets,
            &visible,
            &mut bucket,
        );

        assert_eq!(bucket, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn dfs_emit_keeps_wrapper_owned_regions_before_child_frames() {
        let mut widgets = WidgetRegistry::default();
        widgets.register(test_frame(1, WidgetType::Frame, None));
        widgets.register(test_frame(2, WidgetType::Frame, Some(1)));
        widgets.register(test_frame(3, WidgetType::Texture, Some(2)));
        widgets.register(test_frame(4, WidgetType::Frame, Some(2)));
        widgets.register(test_frame(5, WidgetType::Texture, Some(4)));
        widgets.add_child(1, 2);
        widgets.add_child(2, 3);
        widgets.add_child(2, 4);
        widgets.add_child(4, 5);

        let visible = HashSet::from([1, 2, 3, 4, 5]);
        let mut bucket = Vec::new();

        dfs_emit(
            1,
            FrameStrata::Medium.as_index(),
            &widgets,
            &visible,
            &mut bucket,
        );

        assert_eq!(
            bucket,
            vec![1, 2, 3, 4, 5],
            "wrapper-owned regions should render before descendant child frames"
        );
    }
}
