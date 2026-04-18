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

use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn action_bar_toc(addon: &str, toc_name: &str) -> PathBuf {
    blizzard_ui_dir().join(addon).join(toc_name)
}

fn action_bar_addons() -> &'static [(&'static str, &'static str)] {
    &[
        ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
        ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
        ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
        (
            "Blizzard_SharedXMLGame",
            "Blizzard_SharedXMLGame_Mainline.toc",
        ),
        (
            "Blizzard_UIPanelTemplates",
            "Blizzard_UIPanelTemplates_Mainline.toc",
        ),
        (
            "Blizzard_FrameXMLBase",
            "Blizzard_FrameXMLBase_Mainline.toc",
        ),
        ("Blizzard_LoadLocale", "Blizzard_LoadLocale.toc"),
        ("Blizzard_Fonts_Shared", "Blizzard_Fonts_Shared.toc"),
        ("Blizzard_HelpPlate", "Blizzard_HelpPlate.toc"),
        (
            "Blizzard_AccessibilityTemplates",
            "Blizzard_AccessibilityTemplates.toc",
        ),
        ("Blizzard_ObjectAPI", "Blizzard_ObjectAPI_Mainline.toc"),
        ("Blizzard_UIParent", "Blizzard_UIParent_Mainline.toc"),
        ("Blizzard_TextStatusBar", "Blizzard_TextStatusBar.toc"),
        ("Blizzard_MoneyFrame", "Blizzard_MoneyFrame_Mainline.toc"),
        ("Blizzard_POIButton", "Blizzard_POIButton.toc"),
        ("Blizzard_Flyout", "Blizzard_Flyout.toc"),
        ("Blizzard_StoreUI", "Blizzard_StoreUI_Mainline.toc"),
        ("Blizzard_MicroMenu", "Blizzard_MicroMenu_Mainline.toc"),
        ("Blizzard_EditMode", "Blizzard_EditMode.toc"),
        ("Blizzard_GarrisonBase", "Blizzard_GarrisonBase.toc"),
        ("Blizzard_GameTooltip", "Blizzard_GameTooltip_Mainline.toc"),
        (
            "Blizzard_UIParentPanelManager",
            "Blizzard_UIParentPanelManager_Mainline.toc",
        ),
        (
            "Blizzard_Settings_Shared",
            "Blizzard_Settings_Shared_Mainline.toc",
        ),
        (
            "Blizzard_SettingsDefinitions_Shared",
            "Blizzard_SettingsDefinitions_Shared.toc",
        ),
        (
            "Blizzard_SettingsDefinitions_Frame",
            "Blizzard_SettingsDefinitions_Frame_Mainline.toc",
        ),
        (
            "Blizzard_FrameXMLUtil",
            "Blizzard_FrameXMLUtil_Mainline.toc",
        ),
        ("Blizzard_ItemButton", "Blizzard_ItemButton_Mainline.toc"),
        ("Blizzard_QuickKeybind", "Blizzard_QuickKeybind.toc"),
        ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
        (
            "Blizzard_UIPanels_Game",
            "Blizzard_UIPanels_Game_Mainline.toc",
        ),
        (
            "Blizzard_MapCanvasSecureUtil",
            "Blizzard_MapCanvasSecureUtil.toc",
        ),
        ("Blizzard_MapCanvas", "Blizzard_MapCanvas.toc"),
        (
            "Blizzard_SharedMapDataProviders",
            "Blizzard_SharedMapDataProviders_Mainline.toc",
        ),
        ("Blizzard_WorldMap", "Blizzard_WorldMap_Mainline.toc"),
        ("Blizzard_ActionBar", "Blizzard_ActionBar_Mainline.toc"),
    ]
}

fn fire_startup_events(env: &WowLuaEnv) {
    let _ = env.fire_event_with_args("ADDON_LOADED", &[env.lua_string("WoWUISim")]);
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    let _ = env.fire_event_with_args(
        "PLAYER_ENTERING_WORLD",
        &[rilua::Val::Bool(true), rilua::Val::Bool(false)],
    );
    let _ = env.fire_edit_mode_layouts_updated();
    let _ = env.fire_event("ACTIONBAR_SHOWGRID");
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }
}

fn load_settled_game_ui() -> common::LockedEnv {
    common::lock_env(|| {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);
        env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];

        for (name, toc) in action_bar_addons() {
            let toc_path = action_bar_toc(name, toc);
            if toc_path.exists() {
                load_addon(&env.loader_env(), &toc_path)
                    .unwrap_or_else(|err| panic!("Failed to load Blizzard addon {name}: {err}"));
            }
        }

        env.apply_post_load_workarounds();
        fire_startup_events(&env);
        env
    })
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
