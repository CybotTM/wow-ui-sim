#![cfg(feature = "gui")]
//! Tests for spellbook first-load rendering.
//!
//! Bug: Spellbook shows blank on first open, spells appear only on second open.
//! Lua state is correct on first open (IsVisible=true, anchors set, dimensions correct)
//! but the rendering pipeline skips the spell item frames.

mod common;

use std::path::PathBuf;
use wow_ui_sim::iced_app::{build_quad_batch_for_registry, compute_frame_rect};
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::widget::{Frame, WidgetRegistry};

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn spellbook_tutorials_lua_path() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_PlayerSpells/SpellBook/Blizzard_SpellBookFrameTutorials.lua")
}

fn setup_full_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    let addons = discover_blizzard_addons(&ui);
    for (name, toc_path) in &addons {
        if let Err(e) = load_addon(&env.loader_env(), toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }
    env.apply_post_load_workarounds();
    fire_startup_sequence(&env);
    env
}

/// Match exact run_screenshot sequence:
/// fire_startup_events → apply_post_event_workarounds → process_pending_timers
/// → fire_one_on_update_tick → hide_runtime_hidden_frames
fn fire_startup_sequence(env: &WowLuaEnv) {
    wow_ui_sim::startup::fire_startup_events(env);
    env.apply_post_event_workarounds();
    wow_ui_sim::startup::process_pending_timers(env);
    wow_ui_sim::startup::fire_one_on_update_tick(env);
    let _ = wow_ui_sim::lua_api::globals::global_frames::hide_runtime_hidden_frames(&*env.rilua());
}

/// Open the spellbook once (first load, demand-loads Blizzard_PlayerSpells).
/// Does NOT process timers after toggle (matching the screenshot command flow
/// where no timer processing happens between exec-lua and quad building).
fn open_spellbook(env: &WowLuaEnv) {
    env.exec("PlayerSpellsUtil.ToggleSpellBookFrame()")
        .expect("Failed to toggle spellbook");
}

fn restore_spellbook_tutorials(env: &WowLuaEnv) {
    let source = std::fs::read_to_string(spellbook_tutorials_lua_path())
        .expect("Failed to read spellbook tutorials source");
    env.exec(&source)
        .expect("Failed to restore spellbook tutorials mixin");
}

/// Find spell item frame IDs by traversing the Rust registry.
/// Path: PlayerSpellsFrame -> SpellBookFrame -> PagedSpellsFrame -> ViewFrames -> items
fn find_spell_item_ids(registry: &WidgetRegistry) -> Vec<u64> {
    let psf_id = registry.get_id_by_name("PlayerSpellsFrame");
    let psf_id = match psf_id {
        Some(id) => id,
        None => return Vec::new(),
    };
    let psf = registry.get(psf_id).unwrap();

    // SpellBookFrame is a child key of PlayerSpellsFrame
    let sb_id = match psf.children_keys.get("SpellBookFrame") {
        Some(&id) => id,
        None => return Vec::new(),
    };
    let sb = registry.get(sb_id).unwrap();

    // PagedSpellsFrame is a child key of SpellBookFrame
    let paged_id = match sb.children_keys.get("PagedSpellsFrame") {
        Some(&id) => id,
        None => return Vec::new(),
    };
    collect_viewframe_children(registry, paged_id)
}

/// Collect visible children from all shown ViewFrames under a PagedSpellsFrame.
fn collect_viewframe_children(registry: &WidgetRegistry, paged_id: u64) -> Vec<u64> {
    let paged = match registry.get(paged_id) {
        Some(f) => f,
        None => return Vec::new(),
    };
    let mut items = Vec::new();
    for &child_id in &paged.children {
        let child = match registry.get(child_id) {
            Some(f) => f,
            None => continue,
        };
        // ViewFrames are Frame-type children that contain spell items
        if !child.visible {
            continue;
        }
        for &item_id in &child.children {
            if let Some(item) = registry.get(item_id) {
                if item.visible && item.width > 0.0 && item.height > 0.0 {
                    items.push(item_id);
                }
            }
        }
    }
    items
}

fn hover_first_spell_button(env: &WowLuaEnv) -> (String, f32, f32, f32, f32) {
    env.eval(
        r#"
        local paged = PlayerSpellsFrame and PlayerSpellsFrame.SpellBookFrame and PlayerSpellsFrame.SpellBookFrame.PagedSpellsFrame
        assert(paged, "PagedSpellsFrame should exist")

        for _, frame in paged:EnumerateFrames() do
            if frame
                and frame:IsShown()
                and frame.HasValidData
                and frame:HasValidData()
                and frame.spellBookItemInfo
                and frame.spellBookItemInfo.spellID
                and frame.Button
                and frame.Button:IsShown()
            then
                local onEnter = frame.Button:GetScript("OnEnter")
                assert(onEnter, "Spellbook button should have an OnEnter handler")
                onEnter(frame.Button)
                return frame.spellBookItemInfo.name, frame.Button:GetLeft(), frame.Button:GetBottom(), frame.Button:GetRight(), frame.Button:GetTop()
            end
        end

        error("No visible spellbook spell with tooltip data")
        "#,
    )
    .expect("Failed to hover a visible spellbook button")
}

/// Build strata buckets from a WowLuaEnv (mutable borrow), then return a clone.
fn build_strata_buckets(env: &WowLuaEnv) -> Vec<Vec<u64>> {
    let mut state = env.state().borrow_mut();
    let _ = state.get_strata_buckets();
    state.strata_buckets.as_ref().unwrap().clone()
}

/// Build quad batch for the full registry at 1024x768.
fn build_quads(env: &WowLuaEnv) -> usize {
    let buckets = build_strata_buckets(env);
    let state = env.state().borrow();
    let batch = build_quad_batch_for_registry(
        &state.widgets,
        (1024.0, 768.0),
        None,
        None,
        None,
        None,
        None,
        None,
        &buckets,
    );
    batch.quad_count()
}

fn build_quads_with_textures(env: &WowLuaEnv) -> (usize, Vec<String>) {
    let buckets = build_strata_buckets(env);
    let state = env.state().borrow();
    let batch = build_quad_batch_for_registry(
        &state.widgets,
        (1024.0, 768.0),
        None,
        None,
        None,
        None,
        None,
        None,
        &buckets,
    );
    let textures: Vec<String> = batch
        .texture_requests
        .iter()
        .map(|r| r.path.clone())
        .collect();
    (batch.quad_count(), textures)
}

/// Check that a frame is reachable from root via the children chain.
/// Returns (reachable, detail_string).
fn check_frame_reachability(registry: &WidgetRegistry, frame_id: u64) -> (bool, String) {
    let mut id = frame_id;
    let mut path = Vec::new();

    loop {
        let Some(frame) = registry.get(id) else {
            return missing_frame_result(id);
        };
        let name = frame_display_name(frame);
        path.push(format!("{}[{}]", name, id));

        let Some(parent_id) = frame.parent_id else {
            break;
        };
        if !parent_lists_child(registry, parent_id, id) {
            return broken_parent_link_result(registry, name, id, parent_id, &path);
        }
        id = parent_id;
    }
    path.reverse();
    (true, path.join(" -> "))
}

fn missing_frame_result(frame_id: u64) -> (bool, String) {
    (false, format!("Frame {} not found", frame_id))
}

fn frame_display_name(frame: &Frame) -> &str {
    frame.name.as_deref().unwrap_or("(anon)")
}

fn parent_lists_child(registry: &WidgetRegistry, parent_id: u64, child_id: u64) -> bool {
    registry
        .get(parent_id)
        .is_some_and(|parent| parent.children.contains(&child_id))
}

fn broken_parent_link_result(
    registry: &WidgetRegistry,
    frame_name: &str,
    frame_id: u64,
    parent_id: u64,
    path: &[String],
) -> (bool, String) {
    let parent_name = registry
        .get(parent_id)
        .map(frame_display_name)
        .unwrap_or("?");
    (
        false,
        format!(
            "BREAK: {}[{}] parent={}[{}] but NOT in children list. Path: {}",
            frame_name,
            frame_id,
            parent_name,
            parent_id,
            path.join(" -> ")
        ),
    )
}

/// Log the ancestor chain of a frame for debugging.
fn log_ancestor_chain(registry: &WidgetRegistry, frame_id: u64) {
    let mut id = frame_id;
    loop {
        let Some(frame) = registry.get(id) else {
            eprintln!("  Frame {} not found!", id);
            break;
        };
        let name = frame.name.as_deref().unwrap_or("(anon)");
        eprintln!(
            "  {} [{}]: visible={}, children={}, anchors={}",
            name,
            id,
            frame.visible,
            frame.children.len(),
            frame.anchors.len()
        );
        match frame.parent_id {
            Some(pid) => id = pid,
            None => break,
        }
    }
}

#[test]
fn spellbook_spells_visible_on_first_open() {
    test_timeout! {
        let env = setup_full_ui();
        open_spellbook(&env);

        let sb_visible: bool = env
            .eval(
                "return PlayerSpellsFrame and PlayerSpellsFrame.SpellBookFrame \
                 and PlayerSpellsFrame.SpellBookFrame:IsVisible() or false",
            )
            .unwrap();
        assert!(sb_visible, "SpellBookFrame should be visible after toggle");

        let item_ids = {
            let state = env.state().borrow();
            find_spell_item_ids(&state.widgets)
        };
        assert!(!item_ids.is_empty(), "Should have visible spell items");
        eprintln!("{} visible spell items on first open", item_ids.len());

        diagnose_missing_items(&env, &item_ids);

        let first_quads = build_quads(&env);
        eprintln!("First open quad count: {}", first_quads);

        // Close and reopen
        env.exec("PlayerSpellsUtil.ToggleSpellBookFrame()").unwrap();
        let _ = env.process_timers();
        env.exec("PlayerSpellsUtil.ToggleSpellBookFrame()").unwrap();
        let _ = env.process_timers();

        let second_quads = build_quads(&env);
        eprintln!("Second open quad count: {}", second_quads);
        assert!(
            second_quads > 0,
            "Second open should also produce quads after closing and reopening"
        );
    }
}

/// Check which items are missing from ancestor-visible set and log details.
/// Uses effective_alpha > 0 as the visibility check (requires get_strata_buckets called first).
fn diagnose_missing_items(env: &WowLuaEnv, item_ids: &[u64]) {
    // Propagate effective_alpha
    {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
    }

    let state = env.state().borrow();
    let registry = &state.widgets;

    let mut in_set = 0;
    let mut missing = 0;
    for &item_id in item_ids {
        let is_visible = registry
            .get(item_id)
            .is_some_and(|f| f.effective_alpha > 0.0);
        if is_visible {
            in_set += 1;
        } else {
            missing += 1;
            let (ok, detail) = check_frame_reachability(registry, item_id);
            eprintln!(
                "Item {} NOT ancestor-visible: ok={}, {}",
                item_id, ok, detail
            );
            log_ancestor_chain(registry, item_id);
        }
    }
    eprintln!("Ancestor-visible: {} in, {} missing", in_set, missing);
}

#[test]
fn spellbook_spell_items_in_ancestor_visible() {
    test_timeout! {
        let env = setup_full_ui();
        open_spellbook(&env);

        // Propagate effective_alpha
        {
            let mut state = env.state().borrow_mut();
            let _ = state.get_strata_buckets();
        }

        let state = env.state().borrow();
        let item_ids = find_spell_item_ids(&state.widgets);
        assert!(!item_ids.is_empty(), "Should have spell items");

        let missing: Vec<_> = item_ids
            .iter()
            .filter(|&&id| !state.widgets.get(id).is_some_and(|f| f.effective_alpha > 0.0))
            .map(|&id| {
                let name = state.widgets.get(id)
                    .and_then(|f| f.name.as_deref())
                    .unwrap_or("(anon)");
                format!("{}[{}]", name, id)
            })
            .collect();

        assert!(
            missing.is_empty(),
            "All visible spell items should have effective_alpha > 0.\n\
             Missing {} items: {:?}",
            missing.len(),
            &missing[..missing.len().min(10)]
        );
    }
}

#[test]
fn spellbook_icon_textures_in_ancestor_visible() {
    test_timeout! {
        let env = setup_full_ui();
        open_spellbook(&env);

        // Propagate effective_alpha
        {
            let mut state = env.state().borrow_mut();
            let _ = state.get_strata_buckets();
        }

        let state = env.state().borrow();
        let registry = &state.widgets;
        let item_ids = find_spell_item_ids(registry);
        assert!(!item_ids.is_empty(), "Should have spell items");

        // For each spell item, find its Button child, then Icon texture child
        let mut icons_found = 0u32;
        let mut icons_missing = 0u32;
        for &item_id in &item_ids {
            let Some(item) = registry.get(item_id) else { continue };
            let Some(&btn_id) = item.children_keys.get("Button") else { continue };
            let Some(btn) = registry.get(btn_id) else { continue };
            let Some(&icon_id) = btn.children_keys.get("Icon") else { continue };
            let Some(icon) = registry.get(icon_id) else { continue };

            if icon.effective_alpha > 0.0 {
                icons_found += 1;
            } else {
                icons_missing += 1;
                if icons_missing <= 3 {
                    let (ok, detail) = check_frame_reachability(registry, icon_id);
                    eprintln!("Icon {icon_id} NOT ancestor-visible: ok={ok} {detail}");
                    eprintln!("  icon: vis={} tex={:?} w={} h={} alpha={}",
                        icon.visible, icon.texture, icon.width, icon.height,
                        icon.effective_alpha);
                    eprintln!("  btn {btn_id}: vis={} children={:?}",
                        btn.visible, btn.children);
                    eprintln!("  item {item_id}: vis={} children={:?}",
                        item.visible, item.children);
                }
            }
        }
        eprintln!("Icons: found={icons_found} missing={icons_missing}");
        assert_eq!(icons_missing, 0,
            "All spell icon textures should have effective_alpha > 0");
    }
}

#[test]
fn spellbook_texture_requests_match_between_opens() {
    test_timeout! {
        let env = setup_full_ui();
        open_spellbook(&env);

        let (q1, tex1) = build_quads_with_textures(&env);
        let icon_tex1: Vec<_> = tex1.iter()
            .filter(|t| t.to_lowercase().contains("icons"))
            .collect();

        // Close and reopen
        env.exec("PlayerSpellsUtil.ToggleSpellBookFrame()").unwrap();
        let _ = env.process_timers();
        env.exec("PlayerSpellsUtil.ToggleSpellBookFrame()").unwrap();
        let _ = env.process_timers();

        let (q2, tex2) = build_quads_with_textures(&env);
        let icon_tex2: Vec<_> = tex2.iter()
            .filter(|t| t.to_lowercase().contains("icons"))
            .collect();

        eprintln!("First open: {} quads, {} textures, {} icon textures",
            q1, tex1.len(), icon_tex1.len());
        eprintln!("Second open: {} quads, {} textures, {} icon textures",
            q2, tex2.len(), icon_tex2.len());

        // Show icon textures unique to second open
        let set1: std::collections::HashSet<_> = icon_tex1.iter().collect();
        let new_icons: Vec<_> = icon_tex2.iter()
            .filter(|t| !set1.contains(t))
            .collect();
        if !new_icons.is_empty() {
            eprintln!("NEW icon textures on second open: {:?}", &new_icons[..new_icons.len().min(5)]);
        }

        assert_eq!(icon_tex1.len(), icon_tex2.len(),
            "Should have same icon texture count between opens");
    }
}

fn check_rect(
    registry: &WidgetRegistry,
    name: &str,
    sw: f32,
    sh: f32,
    ex: f32,
    ey: f32,
    ew: f32,
    eh: f32,
) {
    let id = registry
        .get_id_by_name(name)
        .unwrap_or_else(|| panic!("Frame '{name}' not found"));
    let rect = compute_frame_rect(registry, id, sw, sh);
    let tol = 2.0;
    assert!(
        (rect.x - ex).abs() <= tol
            && (rect.y - ey).abs() <= tol
            && (rect.width - ew).abs() <= tol
            && (rect.height - eh).abs() <= tol,
        "{name}: expected ({ex}, {ey}, {ew}x{eh}), got ({}, {}, {}x{})",
        rect.x,
        rect.y,
        rect.width,
        rect.height
    );
}

#[test]
fn spellbook_frame_positions() {
    test_timeout! {
        let env = setup_full_ui();
        open_spellbook(&env);

        let state = env.state().borrow();
        let registry = &state.widgets;
        let (sw, sh) = (1024.0, 768.0);

        // PlayerSpellsFrame — main container
        check_rect(registry, "PlayerSpellsFrame", sw, sh, 56.3, 41.0, 911.3, 497.4);

        let psf_id = registry.get_id_by_name("PlayerSpellsFrame").expect("PlayerSpellsFrame exists");
        let psf = registry.get(psf_id).unwrap();

        // SpellBookFrame — verify stored dimensions
        let sb_id = *psf.children_keys.get("SpellBookFrame").expect("SpellBookFrame child key");
        let sb = registry.get(sb_id).unwrap();
        assert!(sb.width > 900.0, "SpellBookFrame stored width {} should be > 900", sb.width);
        assert!(sb.height > 500.0, "SpellBookFrame stored height {} should be > 500", sb.height);

        // NineSlice border — should match PlayerSpellsFrame bounds
        let nine_id = *psf.children_keys.get("NineSlice").expect("NineSlice exists");
        let nine_rect = compute_frame_rect(registry, nine_id, sw, sh);
        let psf_rect = compute_frame_rect(registry, psf_id, sw, sh);
        assert!((nine_rect.x - psf_rect.x).abs() <= 1.0, "NineSlice x should match PlayerSpellsFrame");
        assert!((nine_rect.width - psf_rect.width).abs() <= 1.0, "NineSlice width should match");

        // tabSystem — should be near bottom of PlayerSpellsFrame
        if let Some(&tab_id) = psf.children_keys.get("tabSystem") {
            let tab_rect = compute_frame_rect(registry, tab_id, sw, sh);
            assert!(tab_rect.y > psf_rect.y + psf_rect.height - 50.0,
                "tabSystem y={} should be near bottom of PlayerSpellsFrame (bottom={})",
                tab_rect.y, psf_rect.y + psf_rect.height);
            assert!(tab_rect.width > 100.0, "tabSystem should have width > 100, got {}", tab_rect.width);
        }
    }
}

#[test]
fn spellbook_spell_items_have_nonzero_rect() {
    test_timeout! {
        let env = setup_full_ui();
        open_spellbook(&env);

        let state = env.state().borrow();
        let registry = &state.widgets;
        let item_ids = find_spell_item_ids(registry);
        assert!(!item_ids.is_empty(), "Should have spell items");

        let zero_rect: Vec<_> = item_ids
            .iter()
            .filter_map(|&id| {
                let rect = compute_frame_rect(registry, id, 1024.0, 768.0);
                if rect.width <= 0.0 || rect.height <= 0.0 {
                    let f = registry.get(id)?;
                    let name = f.name.as_deref().unwrap_or("(anon)");
                    Some(format!(
                        "{}[{}] rect={:?} fw={} fh={} anchors={}",
                        name, id, rect, f.width, f.height, f.anchors.len()
                    ))
                } else {
                    None
                }
            })
            .collect();

        assert!(
            zero_rect.is_empty(),
            "All visible spell items should have non-zero layout rects.\n\
             Zero-rect items ({}):\n{}",
            zero_rect.len(),
            zero_rect.join("\n")
        );
    }
}

#[test]
fn spellbook_first_open_is_stable_with_real_tutorial_logic_restored() {
    test_timeout! {
        let env = setup_full_ui();
        restore_spellbook_tutorials(&env);
        {
            let mut state = env.state().borrow_mut();
            state.lua_errors.clear();
            state.lua_error_records.clear();
            state.lua_error_counts.clear();
        }

        open_spellbook(&env);

        let errors = env.state().borrow().lua_errors.clone();
        assert!(
            errors.is_empty(),
            "Restoring the real tutorial logic should not break first-open spellbook startup: {errors:?}"
        );

        let item_ids = {
            let state = env.state().borrow();
            find_spell_item_ids(&state.widgets)
        };
        assert!(
            !item_ids.is_empty(),
            "Spellbook items should still be visible without tutorial suppression"
        );

        let quads = build_quads(&env);
        assert!(
            quads > 0,
            "Spellbook should still render quads when the real tutorial logic runs"
        );
    }
}

#[test]
fn spellbook_hover_shows_spell_tooltip() {
    test_timeout! {
        let env = setup_full_ui();
        open_spellbook(&env);

        let (expected_name, _button_left, _button_bottom, _button_right, _button_top) =
            hover_first_spell_button(&env);

        let visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
        let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
        assert!(visible, "GameTooltip should be visible after spellbook hover");
        assert!(
            num_lines >= 1,
            "Spellbook hover should populate spell tooltip lines, got {num_lines}"
        );

        let state = env.state().borrow();
        let tooltip_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        let tooltip = state
            .tooltips
            .get(&tooltip_id)
            .expect("tooltip data should exist after spellbook hover");
        assert_eq!(tooltip.lines[0].left_text, expected_name);
    }
}

#[test]
fn spellbook_hover_tooltip_is_sized_and_on_screen() {
    test_timeout! {
        let env = setup_full_ui();
        open_spellbook(&env);

        let (_spell_name, _button_left, _button_bottom, _button_right, _button_top) =
            hover_first_spell_button(&env);

        let mut font_sys = wow_ui_sim::render::font::WowFontSystem::new(&PathBuf::from("./fonts"));
        {
            let mut state = env.state().borrow_mut();
            let _ = state.widgets.take_render_dirty();
            wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut font_sys);
        }

        let tooltip_id = {
            let state = env.state().borrow();
            state.widgets.get_id_by_name("GameTooltip").unwrap()
        };

        let (tooltip_rect, tooltip_size) = {
            let state = env.state().borrow();
            let tooltip_frame = state.widgets.get(tooltip_id).unwrap();
            (
                compute_frame_rect(&state.widgets, tooltip_id, 1024.0, 768.0),
                (tooltip_frame.width, tooltip_frame.height),
            )
        };

        assert!(tooltip_size.0 > 0.0, "Tooltip width should be > 0 after spellbook hover");
        assert!(tooltip_size.1 > 0.0, "Tooltip height should be > 0 after spellbook hover");
        assert!(tooltip_rect.width > 0.0, "Tooltip rect width should be > 0");
        assert!(tooltip_rect.height > 0.0, "Tooltip rect height should be > 0");
        assert!(
            tooltip_rect.x >= 0.0 && tooltip_rect.x < 1024.0,
            "Tooltip x={} should be on screen",
            tooltip_rect.x
        );
        assert!(
            tooltip_rect.y >= 0.0 && tooltip_rect.y < 768.0,
            "Tooltip y={} should be on screen",
            tooltip_rect.y
        );

        let buckets = build_strata_buckets(&env);
        let state = env.state().borrow();
        let tooltip_data = wow_ui_sim::iced_app::tooltip::collect_tooltip_data(&state);
        assert!(
            tooltip_data.contains_key(&tooltip_id),
            "Tooltip render data should include GameTooltip after spellbook hover"
        );

        let mut glyph_atlas = wow_ui_sim::render::glyph::GlyphAtlas::new();
        let batch = build_quad_batch_for_registry(
            &state.widgets,
            (1024.0, 768.0),
            None,
            None,
            None,
            Some((&mut font_sys, &mut glyph_atlas)),
            Some(&state.message_frames),
            Some(&tooltip_data),
            &buckets,
        );

        assert!(
            batch.vertices.len() > 100,
            "Quad batch should include tooltip geometry after spellbook hover"
        );
    }
}
