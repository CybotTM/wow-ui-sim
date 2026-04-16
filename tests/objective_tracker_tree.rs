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

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
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
