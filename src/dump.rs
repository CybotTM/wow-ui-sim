//! Frame tree dump and diagnostic utilities.
//!
//! Single implementation used by both `wow-sim dump-tree` (headless) and
//! the connected `wow-cli dump-tree` (via iced_app debug server).

use crate::LayoutRect;
use crate::layout::{anchor_position, compute_frame_rect};
use crate::widget::{Anchor, Frame, WidgetRegistry, WidgetType};
use regex::RegexBuilder;

mod display;

pub use display::strip_wow_escapes;
use display::{
    print_anchor_diagnostic, resolve_addon_name, resolve_button_state_texture,
    resolve_display_name, resolve_display_text, resolve_texture_format,
};

/// Print the frame tree to stdout (headless subcommand).
pub fn print_frame_tree(
    widgets: &WidgetRegistry,
    addon_names: &[String],
    filter: Option<&str>,
    filter_key: Option<&str>,
    visible_only: bool,
    verbose: bool,
    screen_width: f32,
    screen_height: f32,
) {
    print_anchor_diagnostic(widgets);
    eprintln!("\n=== Frame Tree ===\n");
    let lines = build_tree(
        widgets,
        addon_names,
        filter,
        filter_key,
        visible_only,
        verbose,
        screen_width,
        screen_height,
    );
    for line in &lines {
        println!("{line}");
    }
}

/// Build the frame tree as lines (for connected dump-tree server).
pub fn build_tree(
    widgets: &WidgetRegistry,
    addon_names: &[String],
    filter: Option<&str>,
    filter_key: Option<&str>,
    visible_only: bool,
    verbose: bool,
    screen_width: f32,
    screen_height: f32,
) -> Vec<String> {
    let mut lines = Vec::new();
    let roots = sorted_root_frames(widgets);
    emit_tree_lines(
        widgets,
        addon_names,
        &roots,
        filter,
        filter_key,
        visible_only,
        verbose,
        screen_width,
        screen_height,
        &mut lines,
    );
    lines
}

fn sorted_root_frames(widgets: &WidgetRegistry) -> Vec<(u64, Option<String>)> {
    let mut roots = collect_root_frames(widgets);
    roots.sort_by(|a, b| {
        let left_name = a.1.as_deref().unwrap_or("");
        let right_name = b.1.as_deref().unwrap_or("");
        left_name.cmp(right_name)
    });
    roots
}

fn emit_tree_lines(
    widgets: &WidgetRegistry,
    addon_names: &[String],
    roots: &[(u64, Option<String>)],
    filter: Option<&str>,
    filter_key: Option<&str>,
    visible_only: bool,
    verbose: bool,
    screen_width: f32,
    screen_height: f32,
    lines: &mut Vec<String>,
) {
    if let Some(key_filter) = filter_key {
        emit_key_filtered_subtrees(
            widgets,
            addon_names,
            roots,
            key_filter,
            visible_only,
            verbose,
            screen_width,
            screen_height,
            lines,
        );
        return;
    }

    emit_roots_with_filter(
        widgets,
        addon_names,
        roots,
        filter,
        visible_only,
        verbose,
        screen_width,
        screen_height,
        lines,
    );
}

fn compile_dump_regex(pat: &str) -> regex::Regex {
    RegexBuilder::new(pat)
        .case_insensitive(true)
        .build()
        .unwrap_or_else(|_| {
            RegexBuilder::new(&regex::escape(pat))
                .case_insensitive(true)
                .build()
                .unwrap()
        })
}

fn emit_key_filtered_subtrees(
    widgets: &WidgetRegistry,
    addon_names: &[String],
    roots: &[(u64, Option<String>)],
    key_filter: &str,
    visible_only: bool,
    verbose: bool,
    screen_width: f32,
    screen_height: f32,
    lines: &mut Vec<String>,
) {
    let re = compile_dump_regex(key_filter);
    let matching = collect_key_matches(widgets, roots, &re);
    let mut ctx = DumpRenderCtx {
        widgets,
        addon_names,
        visible_only,
        verbose,
        screen_width,
        screen_height,
        lines,
    };
    for id in matching {
        emit_subtree(&mut ctx, id, 0);
    }
}

fn emit_roots_with_filter(
    widgets: &WidgetRegistry,
    addon_names: &[String],
    roots: &[(u64, Option<String>)],
    filter: Option<&str>,
    visible_only: bool,
    verbose: bool,
    screen_width: f32,
    screen_height: f32,
    lines: &mut Vec<String>,
) {
    let re = filter.map(compile_dump_regex);
    let mut ctx = DumpRenderCtx {
        widgets,
        addon_names,
        visible_only,
        verbose,
        screen_width,
        screen_height,
        lines,
    };
    for (id, _) in roots {
        emit_filtered(&mut ctx, *id, 0, re.as_ref());
    }
}

/// Build a compact dump with warning flags (for debug server Dump command).
pub fn build_warning_dump(
    widgets: &WidgetRegistry,
    addon_names: &[String],
    screen_width: f32,
    screen_height: f32,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("WoW UI Simulator - Frame Dump".to_string());
    lines.push(format!(
        "Screen: {}x{}",
        screen_width as i32, screen_height as i32
    ));
    lines.push(String::new());

    let mut root_ids: Vec<u64> = widgets
        .iter_ids()
        .filter(|&id| {
            widgets
                .get(id)
                .map(|f| f.parent_id.is_none() || f.parent_id == Some(1))
                .unwrap_or(false)
        })
        .collect();
    root_ids.sort();

    for id in root_ids {
        emit_warning_recursive(
            widgets,
            addon_names,
            id,
            0,
            screen_width,
            screen_height,
            &mut lines,
        );
    }
    lines
}

struct DumpRenderCtx<'a> {
    widgets: &'a WidgetRegistry,
    addon_names: &'a [String],
    visible_only: bool,
    verbose: bool,
    screen_width: f32,
    screen_height: f32,
    lines: &'a mut Vec<String>,
}

/// Emit a single frame line with computed rect, stored size, anchors, texture.
fn emit_frame_line(
    frame: &Frame,
    id: u64,
    display_name: &str,
    depth: usize,
    ctx: &mut DumpRenderCtx<'_>,
) {
    let indent = "  ".repeat(depth);
    let rect = compute_frame_rect(ctx.widgets, id, ctx.screen_width, ctx.screen_height);
    emit_frame_summary_line(frame, display_name, &indent, &rect, ctx);
    emit_anchor_lines(
        ctx.widgets,
        frame,
        &indent,
        ctx.screen_width,
        ctx.screen_height,
        ctx.lines,
    );
    emit_texture_detail_line(frame, id, &indent, &rect, ctx);
    if let Some(line) = atlas_detail_line(frame, &indent) {
        ctx.lines.push(line);
    }
    if let Some(line) = mask_detail_line(ctx.widgets, frame, &indent) {
        ctx.lines.push(line);
    }
}

fn emit_frame_summary_line(
    frame: &Frame,
    display_name: &str,
    indent: &str,
    rect: &LayoutRect,
    ctx: &mut DumpRenderCtx<'_>,
) {
    let vis = if frame.visible { "visible" } else { "hidden" };
    let strata_str = format!(" {}:{}", frame.frame_strata.as_str(), frame.frame_level);
    let mask_str = if frame.is_mask { " MASK" } else { "" };
    let owner_str = resolve_addon_name(ctx.addon_names, frame.owner_addon)
        .map(|name| format!(" @{name}"))
        .unwrap_or_default();
    ctx.lines.push(format!(
        "{indent}{display_name} [{:?}] {} {vis}{strata_str}{mask_str}{owner_str}{}{}{}{}",
        frame.widget_type,
        format_size_str(frame, rect),
        format_stale_str(frame, rect),
        format_info_str(frame, rect),
        format_text_str(ctx.widgets, frame),
        format_font_str(frame),
    ));
}

fn emit_texture_detail_line(
    frame: &Frame,
    id: u64,
    indent: &str,
    rect: &LayoutRect,
    ctx: &mut DumpRenderCtx<'_>,
) {
    let tex_path = frame
        .texture
        .as_deref()
        .or_else(|| resolve_button_state_texture(ctx.widgets, frame, id));
    if let Some(path) = tex_path {
        let fmt = resolve_texture_format(path);
        let detail = format_texture_detail_str(frame, rect, ctx.verbose);
        ctx.lines
            .push(format!("{indent}  [texture] {path}{fmt}{detail}"));
    }
}

fn atlas_detail_line(frame: &Frame, indent: &str) -> Option<String> {
    frame
        .atlas
        .as_ref()
        .map(|atlas| format!("{indent}  [atlas] {atlas}"))
}

fn mask_detail_line(widgets: &WidgetRegistry, frame: &Frame, indent: &str) -> Option<String> {
    if frame.mask_textures.is_empty() {
        return None;
    }
    let mask_names: Vec<_> = frame
        .mask_textures
        .iter()
        .map(|mid| {
            widgets
                .get(*mid)
                .map(|m| m.texture.as_deref().unwrap_or("?"))
                .unwrap_or("missing")
        })
        .collect();
    Some(format!("{indent}  [masks] {}", mask_names.join(", ")))
}

/// Emit anchor detail lines for a frame.
fn emit_anchor_lines(
    widgets: &WidgetRegistry,
    frame: &Frame,
    indent: &str,
    screen_width: f32,
    screen_height: f32,
    lines: &mut Vec<String>,
) {
    if frame.anchors.is_empty() {
        return;
    }
    let parent_rect = anchor_parent_rect(widgets, frame, screen_width, screen_height);

    for anchor in &frame.anchors {
        let (rel_name, rel_rect) =
            resolve_anchor_target(widgets, anchor, parent_rect, screen_width, screen_height);
        lines.push(format_anchor_line(indent, anchor, rel_name, rel_rect));
    }
}

fn anchor_parent_rect(
    widgets: &WidgetRegistry,
    frame: &Frame,
    screen_width: f32,
    screen_height: f32,
) -> LayoutRect {
    frame
        .parent_id
        .map(|pid| compute_frame_rect(widgets, pid, screen_width, screen_height))
        .unwrap_or(LayoutRect {
            x: 0.0,
            y: 0.0,
            width: screen_width,
            height: screen_height,
        })
}

fn resolve_anchor_target<'a>(
    widgets: &'a WidgetRegistry,
    anchor: &'a Anchor,
    parent_rect: LayoutRect,
    screen_width: f32,
    screen_height: f32,
) -> (&'a str, LayoutRect) {
    if let Some(rel_id) = anchor.relative_to_id {
        let rect = compute_frame_rect(widgets, rel_id as u64, screen_width, screen_height);
        let name = widgets
            .get(rel_id as u64)
            .and_then(|f| f.name.as_deref())
            .unwrap_or("(anon)");
        (name, rect)
    } else {
        (
            anchor.relative_to.as_deref().unwrap_or("$parent"),
            parent_rect,
        )
    }
}

fn format_anchor_line(
    indent: &str,
    anchor: &Anchor,
    rel_name: &str,
    rel_rect: LayoutRect,
) -> String {
    let (ax, ay) = anchor_position(
        anchor.relative_point,
        rel_rect.x,
        rel_rect.y,
        rel_rect.width,
        rel_rect.height,
    );
    format!(
        "{indent}  [anchor] {} -> {}:{} offset({:.0},{:.0}) -> ({:.0},{:.0})",
        anchor.point.as_str(),
        rel_name,
        anchor.relative_point.as_str(),
        anchor.x_offset,
        anchor.y_offset,
        ax + anchor.x_offset,
        ay - anchor.y_offset,
    )
}
/// Computed rect, with stored size annotation when it differs.
fn format_size_str(frame: &Frame, rect: &LayoutRect) -> String {
    let differs =
        (frame.width - rect.width).abs() > 0.5 || (frame.height - rect.height).abs() > 0.5;
    if differs && (frame.width > 0.0 || frame.height > 0.0) {
        format!(
            "({}x{}) [stored={}x{}]",
            rect.width as i32, rect.height as i32, frame.width as i32, frame.height as i32,
        )
    } else {
        format!("({}x{})", rect.width as i32, rect.height as i32)
    }
}

/// layout_rect staleness: show if cached rect diverges from computed rect.
fn format_stale_str(frame: &Frame, rect: &LayoutRect) -> String {
    match frame.layout_rect {
        Some(lr)
            if (lr.x - rect.x).abs() > 0.5
                || (lr.y - rect.y).abs() > 0.5
                || (lr.width - rect.width).abs() > 0.5
                || (lr.height - rect.height).abs() > 0.5 =>
        {
            format!(
                " [layout_rect=({:.0},{:.0}) {:.0}x{:.0}]",
                lr.x, lr.y, lr.width, lr.height
            )
        }
        None => " [layout_rect=None]".to_string(),
        _ => String::new(),
    }
}

fn format_info_str(frame: &Frame, rect: &LayoutRect) -> String {
    let scale_str = if (frame.scale - 1.0).abs() > 0.001 {
        format!(" scale={:.2}", frame.scale)
    } else {
        String::new()
    };
    format!(
        " x={}, y={}, alpha={:.2}{scale_str}",
        rect.x as i32, rect.y as i32, frame.alpha,
    )
}

fn format_texture_detail_str(frame: &Frame, rect: &LayoutRect, verbose: bool) -> String {
    if !verbose {
        return String::new();
    }

    let mut detail = format!(
        " rect=({:.0},{:.0} {:.0}x{:.0})",
        rect.x, rect.y, rect.width, rect.height
    );
    if let Some((left, right, top, bottom)) = frame.tex_coords {
        detail.push_str(&format!(
            " tex_coords=({left:.3},{right:.3},{top:.3},{bottom:.3})"
        ));
    }
    if let Some((left, right, top, bottom)) = frame.atlas_tex_coords {
        detail.push_str(&format!(
            " atlas_tex_coords=({left:.3},{right:.3},{top:.3},{bottom:.3})"
        ));
    }
    if let Some(raw) = frame.tex_coords_quad {
        detail.push_str(&format!(
            " tex_coords_quad=({:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3})",
            raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7]
        ));
    }
    detail
}

fn format_text_str(widgets: &WidgetRegistry, frame: &Frame) -> String {
    resolve_display_text(widgets, frame)
        .map(|t| format!(" text={:?}", t))
        .unwrap_or_default()
}

fn format_font_str(frame: &Frame) -> String {
    if frame.widget_type == WidgetType::FontString {
        format!(
            " font={:?} size={}",
            frame.font.as_deref().unwrap_or("(none)"),
            frame.font_size
        )
    } else {
        String::new()
    }
}
/// Emit a full subtree unconditionally (for filter_key matches).
fn emit_subtree(ctx: &mut DumpRenderCtx<'_>, id: u64, depth: usize) {
    emit_matching_subtree(ctx, id, depth, &|_| true, true);
}

/// Emit frames matching a name filter (regex, case-insensitive).
fn emit_filtered(
    ctx: &mut DumpRenderCtx<'_>,
    id: u64,
    depth: usize,
    filter: Option<&regex::Regex>,
) {
    emit_matching_subtree(
        ctx,
        id,
        depth,
        &|name| filter.map(|re| re.is_match(name)).unwrap_or(true),
        false,
    );
}

fn emit_matching_subtree<F>(
    ctx: &mut DumpRenderCtx<'_>,
    id: u64,
    depth: usize,
    should_emit: &F,
    emit_all_descendants: bool,
) where
    F: Fn(&str) -> bool,
{
    let Some(frame) = ctx.widgets.get(id) else {
        return;
    };
    if ctx.visible_only && !ctx.widgets.is_ancestor_visible(id) {
        return;
    }
    let name = resolve_display_name(ctx.widgets, frame, id);
    let should_emit_frame = emit_all_descendants || should_emit(&name);
    let child_ids = frame.children.clone();

    if should_emit_frame {
        emit_frame_line(frame, id, &name, depth, ctx);
    }
    for child_id in child_ids {
        emit_matching_subtree(ctx, child_id, depth + 1, should_emit, emit_all_descendants);
    }
}

/// Emit a frame with warning flags (compact format for debug server).
fn emit_warning_recursive(
    widgets: &WidgetRegistry,
    addon_names: &[String],
    id: u64,
    depth: usize,
    screen_width: f32,
    screen_height: f32,
    lines: &mut Vec<String>,
) {
    let Some(frame) = widgets.get(id) else { return };
    let rect = compute_frame_rect(widgets, id, screen_width, screen_height);
    let indent = "  ".repeat(depth);
    let name = frame.name.as_deref().unwrap_or("(anon)");
    let owner_str = resolve_addon_name(addon_names, frame.owner_addon)
        .map(|a| format!(" @{a}"))
        .unwrap_or_default();
    let warnings = build_warnings(frame, &rect, screen_width, screen_height);
    let warn_str = if warnings.is_empty() {
        String::new()
    } else {
        format!(" ! {}", warnings.join(", "))
    };
    lines.push(format!(
        "{indent}{name} [{}] ({:.0},{:.0} {}x{}){owner_str}{warn_str}",
        frame.widget_type.as_str(),
        rect.x,
        rect.y,
        rect.width as i32,
        rect.height as i32,
    ));
    for &child_id in &frame.children {
        emit_warning_recursive(
            widgets,
            addon_names,
            child_id,
            depth + 1,
            screen_width,
            screen_height,
            lines,
        );
    }
}

fn build_warnings(
    frame: &Frame,
    rect: &LayoutRect,
    screen_width: f32,
    screen_height: f32,
) -> Vec<&'static str> {
    let mut w = Vec::new();
    if rect.width <= 0.0 {
        w.push("ZERO_WIDTH");
    }
    if rect.height <= 0.0 {
        w.push("ZERO_HEIGHT");
    }
    if rect.x + rect.width < 0.0 || rect.x > screen_width {
        w.push("OFFSCREEN_X");
    }
    if rect.y + rect.height < 0.0 || rect.y > screen_height {
        w.push("OFFSCREEN_Y");
    }
    if !frame.visible {
        w.push("HIDDEN");
    }
    w
}
fn collect_key_matches(
    widgets: &WidgetRegistry,
    roots: &[(u64, Option<String>)],
    re: &regex::Regex,
) -> Vec<u64> {
    let mut result = Vec::new();
    for &(id, _) in roots {
        collect_key_matches_recursive(widgets, id, re, &mut result);
    }
    result
}

fn collect_key_matches_recursive(
    widgets: &WidgetRegistry,
    id: u64,
    re: &regex::Regex,
    result: &mut Vec<u64>,
) {
    let Some(frame) = widgets.get(id) else { return };
    let display = resolve_display_name(widgets, frame, id);
    if re.is_match(&display) {
        result.push(id);
        return;
    }
    for &child_id in &frame.children {
        collect_key_matches_recursive(widgets, child_id, re, result);
    }
}

fn collect_root_frames(widgets: &WidgetRegistry) -> Vec<(u64, Option<String>)> {
    widgets
        .iter_ids()
        .filter_map(|id| {
            let w = widgets.get(id)?;
            if w.parent_id.is_none() {
                Some((id, w.name.clone()))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LayoutRect;

    #[test]
    fn mask_detail_line_lists_resolved_unknown_and_missing_masks() {
        let mut widgets = WidgetRegistry::new();

        let mut mask = Frame::new(WidgetType::Texture, Some("MaskTex".to_string()), None);
        mask.texture = Some("Interface/Masks/QuestMask".to_string());
        let mask_id = mask.id;
        widgets.register(mask);

        let mut frame = Frame::new(WidgetType::Texture, Some("Owner".to_string()), None);
        frame.mask_textures = vec![mask_id, 424242];

        let line = mask_detail_line(&widgets, &frame, "  ")
            .expect("mask line should exist when masks are present");

        assert_eq!(line, "    [masks] Interface/Masks/QuestMask, missing");
    }

    #[test]
    fn filter_key_visible_only_excludes_children_of_hidden_parents() {
        let mut widgets = WidgetRegistry::default();
        let mut parent = Frame::new(WidgetType::Frame, Some("HiddenParent".to_string()), None);
        parent.visible = false;
        parent.layout_rect = Some(LayoutRect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        });
        let parent_id = parent.id;

        let mut child = Frame::new(
            WidgetType::Frame,
            Some("LocallyVisibleChild".to_string()),
            Some(parent_id),
        );
        child.visible = true;
        child.layout_rect = Some(LayoutRect {
            x: 10.0,
            y: 10.0,
            width: 20.0,
            height: 20.0,
        });
        let child_id = child.id;

        widgets.register(parent);
        widgets.register(child);
        widgets.add_child(parent_id, child_id);

        let lines = build_tree(
            &widgets,
            &[],
            None,
            Some("LocallyVisibleChild"),
            true,
            false,
            100.0,
            100.0,
        );

        assert!(
            lines.is_empty(),
            "visible-only filter-key dump should exclude hidden-ancestor children: {lines:?}"
        );
    }
}
