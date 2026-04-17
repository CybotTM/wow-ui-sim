//! Party-frame tree regression test.
//!
//! Captures the rendered PartyFrame shape for a 4-member group against the
//! reference dump taken on `master` at commit `322eba4a` (see
//! `docs/wiki/investigations/partyframe-tree.md`):
//!
//! ```text
//! PartyFrame          (120x244) visible LOW:2 x=22  y=147
//!   .MemberFrame1     (120x53)  visible LOW:2 x=22  y=147
//!   .MemberFrame2     (120x53)  visible LOW:2 x=22  y=210
//!   .MemberFrame3     (120x53)  visible LOW:2 x=22  y=273
//!   .MemberFrame4     (120x53)  visible LOW:2 x=22  y=336
//! ```
//!
//! The `rilua-migration` branch currently renders `PartyFrame` at `(4x2)`
//! with zero member frames. That's a visible regression vs. `master`, so
//! this test is expected to fail on the branch until the Blizzard UnitFrame
//! load path is restored (see `intern_string_static` registry-key mismatch
//! in `rilua_methods.rs`). Once it passes here, it becomes a guard against
//! future regressions in party-frame layout or member enumeration.

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

/// PartyFrame itself has the shape `master` produces.
#[test]
fn party_frame_has_master_reference_shape() {
    test_timeout! {
        let env = load_settled_game_ui();
        env.exec("A_Admin.SetPartySize(4)").unwrap();
        env.exec(
            r#"
            if PartyFrame and PartyFrame.UpdatePartyFrames then
                pcall(PartyFrame.UpdatePartyFrames, PartyFrame)
            end
            "#,
        )
        .unwrap();

        let (exists, width, height, visible, x, y): (bool, f64, f64, bool, f64, f64) = env
            .eval(
                r#"
                if not PartyFrame then return false, 0, 0, false, 0, 0 end
                local w, h = PartyFrame:GetSize()
                local x, y = PartyFrame:GetLeft() or 0, PartyFrame:GetTop() or 0
                return true, w, h, PartyFrame:IsVisible(), x, y
                "#,
            )
            .expect("eval PartyFrame");

        assert!(exists, "PartyFrame must be a global frame after addons load");
        assert!(visible, "PartyFrame must be IsVisible() after a 4-member party is set");
        // Master dump: size 120x244, top-left at (22, 147).
        assert_eq!(
            (width as i32, height as i32),
            (120, 244),
            "PartyFrame size must match master reference (got {width}x{height})",
        );
        assert_eq!(
            x as i32, 22,
            "PartyFrame left edge must be x=22 (got x={x})",
        );
    }
}

/// All four member frames populate with the 63px vertical stride master uses.
#[test]
fn party_frame_member_frames_render_at_master_offsets() {
    test_timeout! {
        let env = load_settled_game_ui();
        env.exec("A_Admin.SetPartySize(4)").unwrap();
        env.exec(
            r#"
            if PartyFrame and PartyFrame.UpdatePartyFrames then
                pcall(PartyFrame.UpdatePartyFrames, PartyFrame)
            end
            "#,
        )
        .unwrap();

        // Member 1..=4: (120x53) visible with y = 147, 210, 273, 336 (63px
        // stride). `GetTop` returns the bottom-up coordinate of the top edge.
        let results: Vec<(String, bool, f64, f64, f64, f64)> = (1..=4)
            .map(|i| {
                let key = format!("MemberFrame{i}");
                let eval_src = format!(
                    r#"
                    local mf = PartyFrame and PartyFrame.{key}
                    if not mf then return "{key}|missing", false, 0, 0, 0, 0 end
                    local w, h = mf:GetSize()
                    local x, y = mf:GetLeft() or 0, mf:GetTop() or 0
                    return "{key}|ok", mf:IsVisible(), w, h, x, y
                    "#
                );
                env.eval::<(String, bool, f64, f64, f64, f64)>(&eval_src)
                    .expect("eval MemberFrame")
            })
            .collect();

        let expected_y = [147.0, 210.0, 273.0, 336.0];
        for (idx, (tag, visible, w, h, x, y)) in results.iter().enumerate() {
            assert!(
                tag.ends_with("|ok"),
                "MemberFrame{} missing from PartyFrame ({tag})",
                idx + 1,
            );
            assert!(
                *visible,
                "MemberFrame{} must be IsVisible() with a 4-member party",
                idx + 1,
            );
            assert_eq!(
                (*w as i32, *h as i32),
                (120, 53),
                "MemberFrame{} size mismatch (got {w}x{h})",
                idx + 1,
            );
            assert_eq!(*x as i32, 22, "MemberFrame{} x mismatch", idx + 1);
            // y can be expressed bottom-up; master reports the top of each
            // slot: 147, 210, 273, 336. Allow either orientation by matching
            // the absolute offset from the first member.
            let baseline_y = results[0].5;
            let rel = *y - baseline_y;
            let expected_rel = expected_y[idx] - expected_y[0];
            assert!(
                (rel - expected_rel).abs() < 1.0,
                "MemberFrame{} y offset mismatch: got {}, expected {} (baseline {})",
                idx + 1,
                rel,
                expected_rel,
                baseline_y,
            );
        }
    }
}

/// Structural sanity: the four decorative templates master emits
/// (Selection + Background + Selection.MouseOverHighlight.Center) are
/// present on the branch too.
#[test]
fn party_frame_has_background_and_selection_children() {
    test_timeout! {
        let env = load_settled_game_ui();
        env.exec("A_Admin.SetPartySize(4)").unwrap();

        let (has_selection, has_background, selection_w, background_w): (
            bool,
            bool,
            f64,
            f64,
        ) = env
            .eval(
                r#"
                if not PartyFrame then return false, false, 0, 0 end
                local sel = PartyFrame.Selection
                local bg = PartyFrame.Background
                local sw = sel and sel.GetWidth and sel:GetWidth() or 0
                local bw = bg and bg.GetWidth and bg:GetWidth() or 0
                return sel ~= nil, bg ~= nil, sw, bw
                "#,
            )
            .expect("eval PartyFrame decorations");

        assert!(has_selection, "PartyFrame.Selection must be attached");
        assert!(has_background, "PartyFrame.Background must be attached");
        // Master: Selection 120, Background 144.
        assert_eq!(
            selection_w as i32, 120,
            "PartyFrame.Selection width mismatch (got {selection_w})",
        );
        assert_eq!(
            background_w as i32, 144,
            "PartyFrame.Background width mismatch (got {background_w})",
        );
    }
}

#[test]
fn party_frame_selection_tracks_parent_size_in_registry() {
    test_timeout! {
        let env = load_settled_game_ui();
        env.exec("A_Admin.SetPartySize(4)").unwrap();
        env.exec(
            r#"
            if PartyFrame and PartyFrame.UpdatePartyFrames then
                pcall(PartyFrame.UpdatePartyFrames, PartyFrame)
            end
            "#,
        )
        .unwrap();
        let selection_width_before_ensure: f64 = env
            .eval(
                r#"
                if not PartyFrame or not PartyFrame.Selection then return 0 end
                return PartyFrame.Selection:GetWidth()
                "#,
            )
            .expect("read PartyFrame.Selection width before ensure_layout_rects");

        let state = env.state();
        let sim = state.borrow();
        let party_id = sim
            .widgets
            .get_id_by_name("PartyFrame")
            .expect("PartyFrame id");
        let party = sim.widgets.get(party_id).expect("PartyFrame widget");
        let selection_id = *party
            .children_keys
            .get("Selection")
            .expect("PartyFrame.Selection child id");
        let selection = sim.widgets.get(selection_id).expect("Selection widget");

        assert_eq!(
            selection.parent_id,
            Some(party_id),
            "PartyFrame.Selection must stay parented to PartyFrame",
        );
        assert!(
            party.children.contains(&selection_id),
            "PartyFrame.Selection must stay in PartyFrame.children",
        );
        assert_eq!(
            selection.anchors.len(),
            2,
            "PartyFrame.Selection must keep TOPLEFT/BOTTOMRIGHT anchors",
        );
        assert!(
            sim.widgets.is_rect_dirty(selection_id) || selection.layout_rect.is_some(),
            "PartyFrame.Selection must either be dirty or already have a layout rect",
        );
        drop(sim);

        state.borrow_mut().ensure_layout_rects();
        let sim = state.borrow();
        let party = sim.widgets.get(party_id).expect("PartyFrame widget");
        let selection = sim.widgets.get(selection_id).expect("Selection widget");
        let party_rect = party.layout_rect.expect("PartyFrame layout rect");
        let selection_rect = selection.layout_rect.expect("Selection layout rect");
        assert_eq!(
            selection_width_before_ensure as i32,
            selection_rect.width as i32,
            "Lua GetWidth() must agree with the resolved registry width",
        );
        assert_eq!(
            selection_rect.width as i32,
            party_rect.width as i32,
            "PartyFrame.Selection cached width must track PartyFrame width (selection={selection_rect:?}, party={party_rect:?})",
        );
        assert_eq!(
            selection_rect.height as i32,
            party_rect.height as i32,
            "PartyFrame.Selection cached height must track PartyFrame height (selection={selection_rect:?}, party={party_rect:?})",
        );
    }
}
