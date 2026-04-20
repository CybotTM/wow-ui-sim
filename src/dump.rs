//! Frame tree dump and diagnostic utilities.
//!
//! Single implementation used by both `wow-sim dump-tree` (headless) and
//! the connected `wow-cli dump-tree` (via iced_app debug server).

use crate::LayoutRect;
use crate::layout::{anchor_position, compute_frame_rect};
use crate::widget::{Anchor, Frame, WidgetRegistry, WidgetType};
use regex::RegexBuilder;

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
    for id in matching {
        emit_subtree(
            widgets,
            addon_names,
            id,
            0,
            visible_only,
            verbose,
            screen_width,
            screen_height,
            lines,
        );
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
    for (id, _) in roots {
        emit_filtered(
            widgets,
            addon_names,
            *id,
            0,
            re.as_ref(),
            visible_only,
            verbose,
            screen_width,
            screen_height,
            lines,
        );
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
fn emit_subtree(
    widgets: &WidgetRegistry,
    addon_names: &[String],
    id: u64,
    depth: usize,
    visible_only: bool,
    verbose: bool,
    screen_width: f32,
    screen_height: f32,
    lines: &mut Vec<String>,
) {
    let Some(frame) = widgets.get(id) else { return };
    if visible_only && !frame.visible {
        return;
    }
    let name = resolve_display_name(widgets, frame, id);
    let mut render = DumpRenderCtx {
        widgets,
        addon_names,
        verbose,
        screen_width,
        screen_height,
        lines,
    };
    emit_frame_line(frame, id, &name, depth, &mut render);
    for &child_id in &frame.children {
        emit_subtree(
            widgets,
            addon_names,
            child_id,
            depth + 1,
            visible_only,
            verbose,
            screen_width,
            screen_height,
            lines,
        );
    }
}

/// Emit frames matching a name filter (regex, case-insensitive).
#[allow(clippy::too_many_arguments)]
fn emit_filtered(
    widgets: &WidgetRegistry,
    addon_names: &[String],
    id: u64,
    depth: usize,
    filter: Option<&regex::Regex>,
    visible_only: bool,
    verbose: bool,
    screen_width: f32,
    screen_height: f32,
    lines: &mut Vec<String>,
) {
    let Some(frame) = widgets.get(id) else { return };
    if visible_only && !frame.visible {
        return;
    }
    let name = resolve_display_name(widgets, frame, id);
    let matches = filter.map(|re| re.is_match(&name)).unwrap_or(true);
    if matches {
        let mut render = DumpRenderCtx {
            widgets,
            addon_names,
            verbose,
            screen_width,
            screen_height,
            lines,
        };
        emit_frame_line(frame, id, &name, depth, &mut render);
    }
    for &child_id in &frame.children {
        emit_filtered(
            widgets,
            addon_names,
            child_id,
            depth + 1,
            filter,
            visible_only,
            verbose,
            screen_width,
            screen_height,
            lines,
        );
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

/// Resolve owner addon index to folder name.
fn resolve_addon_name(addon_names: &[String], owner: Option<u16>) -> Option<&str> {
    owner.and_then(|idx| addon_names.get(idx as usize).map(|s| s.as_str()))
}

/// Prefer semantic child keys when the runtime name is a lowercase or
/// synthetic fallback.
fn resolve_display_name(widgets: &WidgetRegistry, frame: &Frame, id: u64) -> String {
    if let Some(parent_key) = frame.parent_key.as_deref()
        && should_prefer_parent_key(frame.name.as_deref(), parent_key)
    {
        return format!(".{parent_key}");
    }
    if let Some(parent_id) = frame.parent_id
        && let Some(parent) = widgets.get(parent_id)
    {
        for (key, &child_id) in &parent.children_keys {
            if child_id == id && should_prefer_parent_key(frame.name.as_deref(), key) {
                return format!(".{key}");
            }
        }
    }
    if let Some(ref name) = frame.name
        && !name.starts_with("__")
    {
        return name.clone();
    }
    // For anonymous frames with text, show a text preview
    if let Some(ref text) = frame.text {
        if text.len() > 20 {
            return format!("\"{}...\"", &text[..17]);
        }
        return format!("\"{text}\"");
    }
    frame.name.as_deref().unwrap_or("(anonymous)").to_string()
}

fn should_prefer_parent_key(name: Option<&str>, parent_key: &str) -> bool {
    let Some(name) = name else {
        return true;
    };
    if name.starts_with("__") {
        return true;
    }
    if name.eq_ignore_ascii_case(parent_key) && name != parent_key {
        return true;
    }
    name.chars().next().is_some_and(|c| c.is_ascii_lowercase())
        && parent_key.chars().any(|c| c.is_ascii_uppercase())
}

fn resolve_display_text(widgets: &WidgetRegistry, frame: &Frame) -> Option<String> {
    if let Some(ref t) = frame.text
        && !t.is_empty()
    {
        return Some(strip_wow_escapes(t));
    }
    for key in &["Title", "TitleText"] {
        if let Some(&child_id) = frame.children_keys.get(*key)
            && let Some(child) = widgets.get(child_id)
            && let Some(ref t) = child.text
            && !t.is_empty()
        {
            return Some(strip_wow_escapes(t));
        }
    }
    None
}
fn print_anchor_diagnostic(widgets: &WidgetRegistry) {
    let mut anchored = 0;
    let mut unanchored = 0;
    let mut unanchored_keys: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for id in widgets.iter_ids() {
        let Some(w) = widgets.get(id) else { continue };
        if !w.anchors.is_empty() {
            anchored += 1;
            continue;
        }
        unanchored += 1;
        let parent_key = find_parent_key(widgets, w, id);
        let parent_name = w
            .parent_id
            .and_then(|pid| widgets.get(pid))
            .and_then(|p| p.name.clone())
            .unwrap_or_else(|| "(no parent)".into());
        let detail = format!("  {:?} on {} ({:?})", w.widget_type, parent_name, w.name);
        let key = parent_key.unwrap_or_else(|| "(no key)".into());
        unanchored_keys.entry(key).or_default().push(detail);
    }
    print_anchor_summary(&unanchored_keys, anchored, unanchored);
}

fn print_anchor_summary(
    keys: &std::collections::HashMap<String, Vec<String>>,
    anchored: usize,
    unanchored: usize,
) {
    let mut kv: Vec<_> = keys.iter().map(|(k, v)| (k.clone(), v.len())).collect();
    kv.sort_by(|a, b| b.1.cmp(&a.1));
    eprintln!("Anchored: {anchored}, Unanchored: {unanchored}");
    eprintln!("Top unanchored keys: {:?}", &kv[..kv.len().min(15)]);
    for (key, _) in kv.iter().take(5) {
        if let Some(details) = keys.get(key) {
            eprintln!("  {key}:");
            for d in details.iter().take(3) {
                eprintln!("  {d}");
            }
        }
    }
    if let Some(no_key) = keys.get("(no key)") {
        print_no_key_breakdown(no_key);
    }
}

fn print_no_key_breakdown(no_key: &[String]) {
    let mut by_type: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for d in no_key {
        let wtype = d.trim().split(' ').next().unwrap_or("?");
        *by_type.entry(wtype.to_string()).or_default() += 1;
    }
    let mut tv: Vec<_> = by_type.iter().collect();
    tv.sort_by(|a, b| b.1.cmp(a.1));
    eprintln!("  (no key) by type: {tv:?}");
}

fn find_parent_key(widgets: &WidgetRegistry, w: &Frame, id: u64) -> Option<String> {
    let pid = w.parent_id?;
    let p = widgets.get(pid)?;
    p.children_keys
        .iter()
        .find(|(_, cid)| **cid == id)
        .map(|(k, _)| k.clone())
}
/// Strip WoW escape sequences (|T...|t texture, |c...|r color) for cleaner display.
pub fn strip_wow_escapes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '|' {
            skip_wow_escape(&mut chars);
        } else {
            result.push(c);
        }
    }
    result.trim().to_string()
}

fn skip_wow_escape(chars: &mut std::iter::Peekable<std::str::Chars>) {
    match chars.peek() {
        Some('H') => {
            chars.next();
            while let Some(ch) = chars.next() {
                if ch == '|' && chars.peek() == Some(&'h') {
                    chars.next();
                    break;
                }
            }
        }
        Some('h') => {
            chars.next();
        }
        Some('T') => {
            chars.next();
            while let Some(&ch) = chars.peek() {
                chars.next();
                if ch == '|' {
                    chars.next();
                    break;
                }
            }
        }
        Some('t') => {
            chars.next();
        }
        Some('c') => {
            chars.next();
            for _ in 0..8 {
                chars.next();
            }
        }
        Some('r') => {
            chars.next();
        }
        _ => {}
    }
}
/// For Texture children with parentKey like NormalTexture/PushedTexture/etc.,
/// look up the texture path from the parent button's corresponding field.
fn resolve_button_state_texture<'a>(
    widgets: &'a WidgetRegistry,
    frame: &Frame,
    id: u64,
) -> Option<&'a str> {
    if frame.widget_type != WidgetType::Texture {
        return None;
    }
    let parent = widgets.get(frame.parent_id?)?;
    let key = parent
        .children_keys
        .iter()
        .find(|&(_, cid)| *cid == id)
        .map(|(k, _)| k.as_str())?;
    match key {
        "NormalTexture" => parent.normal_texture.as_deref(),
        "PushedTexture" => parent.pushed_texture.as_deref(),
        "HighlightTexture" => parent.highlight_texture.as_deref(),
        "DisabledTexture" => parent.disabled_texture.as_deref(),
        _ => None,
    }
}
/// Resolve a WoW texture path and return a suffix indicating the format found.
/// Returns e.g. " (webp)", " (BLP)", or " (MISSING)".
fn resolve_texture_format(wow_path: &str) -> String {
    use crate::texture::{TextureManager, normalize_wow_path};
    use std::sync::OnceLock;

    static TEX_MGR: OnceLock<TextureManager> = OnceLock::new();
    let mgr = TEX_MGR.get_or_init(|| {
        let home = dirs::home_dir().unwrap_or_default();
        let local = std::path::PathBuf::from("./textures");
        let base = if local.exists() {
            local
        } else {
            home.join("Repos/wow-ui-textures")
        };
        TextureManager::new(base)
            .with_interface_path(home.join("Projects/wow/Interface"))
            .with_addons_path(std::path::PathBuf::from("./Interface/AddOns"))
            .with_disk_cache("./cache/textures")
    });

    let normalized = normalize_wow_path(wow_path);
    match mgr.resolve_path(&normalized) {
        Some(p) => {
            let ext = p
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            format!(" ({ext})")
        }
        None => " (MISSING)".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
