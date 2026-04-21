//! ObjectiveTracker tree regression test.
//!
//! On master at commit `322eba4a` the tracker paints "All Objectives",
//! "Quests", and three quest blocks (The Lost Expedition, Defending the
//! Gates, Supply Run). On `rilua-migration` the same environment leaves
//! `ObjectiveTrackerFrame` `hidden` with no child Header, so the screenshot
//! shows nothing in the right-hand tracker area past the minimap.
//!
//! Reference dump (dump-tree --filter-key ObjectiveTrackerFrame):
//!
//! ```text
//! ObjectiveTrackerFrame  [Frame] (260x847) visible LOW:3 x=1335 y=260
//!   .Header              [Frame] (260x32)  visible LOW:4 x=1335 y=260
//!     .Text              [FontString]      text="All Objectives"
//!     .Header            [Frame] (260x26)  visible LOW:5
//!       .Text            [FontString]      text="Quests"
//!       .HeaderText      [FontString]      text="The Lost Expedition"
//!       .HeaderText      [FontString]      text="Defending the Gates"
//!       .HeaderText      [FontString]      text="Supply Run"
//! ```

mod common;

use std::path::PathBuf;

use wow_ui_sim::iced_app::build_quad_batch_for_registry_with_quest_blobs;
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::{GlyphAtlas, WowFontSystem};
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn load_settled_game_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];

    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("Failed to load Blizzard addon {name}: {err}"));
    }

    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    env
}

fn build_text_quad_batch(env: &WowLuaEnv) -> wow_ui_sim::render::QuadBatch {
    let mut font_sys = WowFontSystem::new(&PathBuf::from("./fonts"));
    let mut glyph_atlas = GlyphAtlas::new();
    let buckets = {
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let state = env.state().borrow();
    build_quad_batch_for_registry_with_quest_blobs(
        &state.widgets,
        (1024.0, 768.0),
        None,
        None,
        None,
        Some((&mut font_sys, &mut glyph_atlas)),
        Some(&state.message_frames),
        None,
        Some(&state.quest_blobs),
        &buckets,
    )
}

#[test]
fn objective_tracker_frame_is_visible_after_startup() {
    test_timeout! {
        let env = load_settled_game_ui();

        let (exists, shown, is_visible, width, height): (bool, bool, bool, f64, f64) = env
            .eval(
                r#"
                if not ObjectiveTrackerFrame then return false, false, false, 0, 0 end
                local w, h = ObjectiveTrackerFrame:GetSize()
                return true,
                    ObjectiveTrackerFrame:IsShown(),
                    ObjectiveTrackerFrame:IsVisible(),
                    w, h
                "#,
            )
            .expect("eval ObjectiveTrackerFrame");

        assert!(exists, "ObjectiveTrackerFrame global must exist");
        assert!(
            shown,
            "ObjectiveTrackerFrame:IsShown() must be true after startup (currently hidden on rilua-migration)",
        );
        assert!(
            is_visible,
            "ObjectiveTrackerFrame:IsVisible() must be true — dump-tree shows it as `hidden` on the branch",
        );
        // Master dump: 260x847 post-EditMode layout.
        assert_eq!(width as i32, 260, "ObjectiveTrackerFrame width mismatch");
        assert!(
            height > 100.0,
            "ObjectiveTrackerFrame height must at least cover its header (got {height})",
        );
    }
}

#[test]
fn objective_tracker_header_shows_all_objectives_text() {
    test_timeout! {
        let env = load_settled_game_ui();

        // Master mounts a `.Header` child with a `.Text` FontString reading
        // "All Objectives". Without the header, the rest of the tracker
        // (quests list, category sub-headers, objectives) never paints.
        let (header_exists, header_visible, text, text_visible): (bool, bool, String, bool) = env
            .eval(
                r#"
                local tracker = ObjectiveTrackerFrame
                if not tracker then return false, false, "<no tracker>", false end
                local header = tracker.Header
                if not header then return false, false, "<no header>", false end
                local text_region = header.Text
                local txt = text_region and text_region.GetText and text_region:GetText() or ""
                return true,
                    header:IsVisible(),
                    txt or "",
                    text_region and text_region:IsVisible() or false
                "#,
            )
            .expect("eval ObjectiveTracker header");

        assert!(header_exists, "ObjectiveTrackerFrame.Header must exist");
        assert!(
            header_visible,
            "ObjectiveTrackerFrame.Header must be IsVisible() so it paints above the quest list",
        );
        assert!(
            text_visible,
            "ObjectiveTrackerFrame.Header.Text must be IsVisible() — the 'All Objectives' title",
        );
        assert_eq!(
            text, "All Objectives",
            "Header text must be \"All Objectives\" per master dump (got {text:?})",
        );
    }
}

#[test]
fn objective_tracker_quest_module_header_shows_quests_text() {
    test_timeout! {
        let env = load_settled_game_ui();

        let (
            header_exists,
            header_visible,
            text,
            text_visible,
            text_alpha,
            text_r,
            text_g,
            text_b,
            text_color_a,
            quest_block_count,
        ): (bool, bool, String, bool, f64, f64, f64, f64, f64, i32) = env
            .eval(
                r#"
                local module = QuestObjectiveTracker
                if not module then return false, false, "<no module>", false end
                local header = module.Header
                if not header then return false, false, "<no header>", false end
                local text_region = header.Text
                local txt = text_region and text_region.GetText and text_region:GetText() or ""
                local alpha = text_region and text_region.GetAlpha and text_region:GetAlpha() or -1
                local r, g, b, a = 0, 0, 0, 0
                if text_region and text_region.GetTextColor then
                    r, g, b, a = text_region:GetTextColor()
                end
                local count = 0
                if module.usedBlocks then
                    for _, blocks in pairs(module.usedBlocks) do
                        if type(blocks) == "table" then
                            for _ in pairs(blocks) do
                                count = count + 1
                            end
                        end
                    end
                end
                return true,
                    header:IsVisible(),
                    txt or "",
                    text_region and text_region:IsVisible() or false,
                    alpha,
                    r or 0,
                    g or 0,
                    b or 0,
                    a or 0,
                    count
                "#,
            )
            .expect("eval QuestObjectiveTracker header");

        assert!(header_exists, "QuestObjectiveTracker.Header must exist");
        assert!(
            quest_block_count > 0,
            "QuestObjectiveTracker must contain at least one quest block in seeded startup state",
        );
        assert!(
            header_visible,
            "QuestObjectiveTracker.Header must be visible so the module title paints",
        );
        assert!(
            text_visible,
            "QuestObjectiveTracker.Header.Text must be visible so the 'Quests' label paints",
        );
        assert_eq!(
            text, "Quests",
            "Quest module header text must be \"Quests\" (got {text:?})",
        );
        assert!(
            text_alpha > 0.0,
            "QuestObjectiveTracker.Header.Text alpha must be > 0 so title is not fully transparent",
        );
        // Header text should be readable (gold/yellow-ish), not blacked out.
        assert!(
            text_r > 0.2 || text_g > 0.2 || text_b > 0.2,
            "QuestObjectiveTracker.Header.Text color must not be black (r={text_r}, g={text_g}, b={text_b})",
        );
        assert!(
            text_color_a > 0.0,
            "QuestObjectiveTracker.Header.Text color alpha must be > 0 (got {text_color_a})",
        );
    }
}

#[test]
fn objective_tracker_quest_module_header_recovers_after_runtime_corruption() {
    test_timeout! {
        let env = load_settled_game_ui();

        env.exec(
            r#"
            local module = QuestObjectiveTracker
            assert(module and module.Header and module.Header.Text, "QuestObjectiveTracker header missing")
            local text = module.Header.Text
            text:SetText("")
            text:Hide()
            text:SetAlpha(0)
            text:SetTextColor(1, 1, 0, 0)
            if ObjectiveTrackerManager and ObjectiveTrackerManager.UpdateAll then
                ObjectiveTrackerManager:UpdateAll()
            end
            "#,
        )
        .expect("corrupt + update quest header text");

        let (text, text_visible, text_alpha, text_r, text_g, text_b, text_color_a): (String, bool, f64, f64, f64, f64, f64) = env
            .eval(
                r#"
                local text_region = QuestObjectiveTracker and QuestObjectiveTracker.Header and QuestObjectiveTracker.Header.Text
                if not text_region then
                    return "<missing>", false, -1, 0, 0, 0, 0
                end
                local r, g, b, a = text_region:GetTextColor()
                return text_region:GetText() or "",
                    text_region:IsVisible(),
                    text_region:GetAlpha() or -1,
                    r or 0,
                    g or 0,
                    b or 0,
                    a or 0
                "#,
            )
            .expect("eval restored quest header");

        assert_eq!(text, "Quests", "quest module header text must be restored to 'Quests'");
        assert!(text_visible, "quest module header text must be visible after recovery");
        assert!(text_alpha > 0.0, "quest module header text alpha must be restored (> 0)");
        assert!(
            text_r > 0.2 || text_g > 0.2 || text_b > 0.2,
            "quest module header text color must be readable after recovery (r={text_r}, g={text_g}, b={text_b})",
        );
        assert!(
            text_color_a > 0.0,
            "quest module header text color alpha must recover (> 0), got {text_color_a}",
        );
    }
}

#[test]
fn objective_tracker_quest_module_header_emits_glyph_quads() {
    test_timeout! {
        let env = load_settled_game_ui();
        let (exists, left, right, top, bottom): (bool, f64, f64, f64, f64) = env
            .eval(
                r#"
                local text_region = QuestObjectiveTracker and QuestObjectiveTracker.Header and QuestObjectiveTracker.Header.Text
                if not text_region then
                    return false, 0, 0, 0, 0
                end
                return true,
                    text_region:GetLeft() or 0,
                    text_region:GetRight() or 0,
                    text_region:GetTop() or 0,
                    text_region:GetBottom() or 0
                "#,
            )
            .expect("eval quest header text bounds");

        assert!(exists, "QuestObjectiveTracker.Header.Text must exist");

        let ui_scale = wow_ui_sim::render::texture::UI_SCALE;
        let min_x = left.min(right) as f32 * ui_scale - 2.0;
        let max_x = left.max(right) as f32 * ui_scale + 2.0;
        // GetTop/GetBottom are in WoW coordinates (origin bottom-left), while
        // quad vertices are in renderer coordinates (origin top-left).
        let screen_height = 768.0_f32;
        let top_y = screen_height - top as f32 * ui_scale;
        let bottom_y = screen_height - bottom as f32 * ui_scale;
        let min_y = top_y.min(bottom_y) - 2.0;
        let max_y = top_y.max(bottom_y) + 2.0;

        let batch = build_text_quad_batch(&env);
        let glyph_tex_index = wow_ui_sim::render::shader::GLYPH_ATLAS_TEX_INDEX;
        let glyph_vertices_in_bounds = batch
            .vertices
            .iter()
            .filter(|v| v.tex_index == glyph_tex_index)
            .filter(|v| {
                let x = v.position[0];
                let y = v.position[1];
                x >= min_x && x <= max_x && y >= min_y && y <= max_y
            })
            .count();

        assert!(
            glyph_vertices_in_bounds > 0,
            "QuestObjectiveTracker.Header.Text should emit glyph vertices in bounds; got 0",
        );
    }
}
