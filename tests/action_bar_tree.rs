//! ActionBar tree regression test.
//!
//! Captures the ActionButton1 shape master renders vs. rilua-migration. On
//! `master` at commit `322eba4a`, ActionButton1 has `Icon`, `HotKey`, and
//! `NormalTexture` children — and `ActionButton1Icon` carries a real spell
//! texture (`Interface\ICONS\Spell_Holy_FlashHeal` in the default seeded
//! layout). On `rilua-migration` those child textures never get attached, so
//! every action bar slot renders as an empty gray square.
//!
//! See `docs/wiki/investigations/partyframe-tree.md` — the same
//! `intern_string_static` registry-key mismatch that drops PartyFrame's
//! member enumeration also suppresses the button children that carry the
//! spell icon and keybind text.
//!
//! Master reference (dump-tree --filter ActionButton1):
//!
//! ```text
//! ActionButton1               [CheckButton] (45x45) visible MEDIUM:52 x=519 y=1110
//!   ActionButton1Icon         [Texture]     (45x45) visible MEDIUM:53 x=519 y=1110
//!     [texture] Interface\ICONS\Spell_Holy_FlashHeal
//!   ActionButton1Name         [FontString]  (36x0)  visible MEDIUM:53 x=523 y=1153
//!   ActionButton1NormalTexture[Texture]     (46x45) visible MEDIUM:53 x=519 y=1110
//!     [atlas] UI-HUD-ActionBar-IconFrame
//!     ActionButton1HotKey     [FontString]  (37x10) visible MEDIUM:54 x=523 y=1115 text="1"
//!     ActionButton1Count      [FontString]  (0x0)   visible MEDIUM:54 x=559 y=1150
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
fn action_button1_has_icon_hotkey_and_normal_texture() {
    test_timeout! {
        let env = load_settled_game_ui();

        // The slot must exist with the frame itself visible at master's
        // geometry first — if ActionButton1 isn't there, nothing else is.
        let (ab1_exists, ab1_w, ab1_h, ab1_visible): (bool, f64, f64, bool) = env
            .eval(
                r#"
                if not ActionButton1 then return false, 0, 0, false end
                local w, h = ActionButton1:GetSize()
                return true, w, h, ActionButton1:IsVisible()
                "#,
            )
            .expect("eval ActionButton1");
        assert!(ab1_exists, "ActionButton1 must exist after addons load");
        assert!(ab1_visible, "ActionButton1 must be IsVisible()");
        assert_eq!(
            (ab1_w as i32, ab1_h as i32),
            (45, 45),
            "ActionButton1 size must be 45x45",
        );

        // Master has three critical child regions on every action button:
        //   * Icon (holds the spell texture)
        //   * HotKey (shows the keybind text "1"..="=")
        //   * NormalTexture (the gold action-bar frame overlay)
        // On rilua-migration none of these exist, so the slot renders empty.
        let (has_icon, has_hotkey, has_normal): (bool, bool, bool) = env
            .eval(
                r#"
                return
                    _G.ActionButton1Icon ~= nil,
                    _G.ActionButton1HotKey ~= nil,
                    _G.ActionButton1NormalTexture ~= nil
                "#,
            )
            .expect("eval action button children");

        assert!(has_icon, "ActionButton1Icon global must be defined");
        assert!(
            has_hotkey,
            "ActionButton1HotKey global must be defined — keybind text needs it",
        );
        assert!(
            has_normal,
            "ActionButton1NormalTexture global must be defined — gold frame overlay needs it",
        );
    }
}

#[test]
fn action_button1_normal_texture_uses_iconframe_atlas() {
    test_timeout! {
        let env = load_settled_game_ui();

        let (atlas, width, height, visible): (String, f64, f64, bool) = env
            .eval(
                r#"
                local nt = _G.ActionButton1NormalTexture
                if not nt then return "<missing>", 0, 0, false end
                local a = (nt.GetAtlas and nt:GetAtlas()) or ""
                local w, h = nt:GetSize()
                return a, w, h, nt:IsVisible()
                "#,
            )
            .expect("eval ActionButton1NormalTexture");

        assert_ne!(
            atlas, "<missing>",
            "ActionButton1NormalTexture must exist so the gold action-bar frame can render",
        );
        assert_eq!(
            atlas.to_ascii_lowercase(),
            "ui-hud-actionbar-iconframe",
            "ActionButton1NormalTexture must use UI-HUD-ActionBar-IconFrame (got {atlas:?})",
        );
        assert!(visible, "ActionButton1NormalTexture must be IsVisible()");
        // Master dump: 46x45 (NormalTexture is 1px wider than the button to
        // cover both edges of the gold frame).
        assert_eq!((width as i32, height as i32), (46, 45));
    }
}

#[test]
fn action_button1_hotkey_shows_slot_one() {
    test_timeout! {
        let env = load_settled_game_ui();

        let (text, visible): (String, bool) = env
            .eval(
                r#"
                local hk = _G.ActionButton1HotKey
                if not hk then return "<missing>", false end
                local txt = hk.GetText and hk:GetText() or ""
                return txt or "", hk:IsVisible()
                "#,
            )
            .expect("eval ActionButton1HotKey");

        assert_ne!(text, "<missing>", "ActionButton1HotKey must exist");
        assert!(visible, "ActionButton1HotKey must be IsVisible()");
        // Master default binding set labels slot 1 as "1". Accept either the
        // single-digit label or the full "ACTIONBUTTON1" keybind placeholder
        // Blizzard uses before the keybinding mapping finishes.
        assert!(
            text == "1" || text == "ACTIONBUTTON1",
            "ActionButton1HotKey text should be \"1\" (mapped) or \"ACTIONBUTTON1\" (unmapped), got {text:?}",
        );
    }
}
