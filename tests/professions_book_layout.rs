//! Layout regression test for the classic ProfessionsBookFrame.
//!
//! Opens the professions book via ProfessionMicroButton and asserts the
//! positions of PrimaryProfession1.SpellButton1/SpellButton2 relative to the
//! primary profession row. Mirrors the same test on master so layouts can be
//! cross-checked across branches.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

const BLIZZARD_ADDONS: &[(&str, &str)] = &[
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
    ("Blizzard_FrameEffects", "Blizzard_FrameEffects.toc"),
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
    ("Blizzard_GameMenu", "Blizzard_GameMenu_Mainline.toc"),
    ("Blizzard_UIWidgets", "Blizzard_UIWidgets_Mainline.toc"),
    ("Blizzard_Minimap", "Blizzard_Minimap_Mainline.toc"),
    ("Blizzard_AddOnList", "Blizzard_AddOnList.toc"),
    ("Blizzard_TimerunningUtil", "Blizzard_TimerunningUtil.toc"),
    ("Blizzard_Communities", "Blizzard_Communities_Mainline.toc"),
];

fn setup_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    let ui = blizzard_ui_dir();
    for (name, toc) in BLIZZARD_ADDONS {
        let toc_path = ui.join(name).join(toc);
        if !toc_path.exists() {
            continue;
        }
        if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }

    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
}

fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::call_global_if_present(env, "RequestTimePlayed");
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
        "PLAYER_LEAVING_WORLD",
    ] {
        let _ = env.fire_event(event);
    }
}

fn click(env: &WowLuaEnv, name: &str) {
    env.exec(&format!(
        r#"
        local btn = {name}
        assert(btn, "{name} missing")
        local on = btn:GetScript("OnClick")
        assert(on, "{name} has no OnClick")
        on(btn, "LeftButton", false)
        "#
    ))
    .expect("click failed");
}

fn rect(env: &WowLuaEnv, expr: &str) -> (f64, f64, f64, f64) {
    env.eval::<(f64, f64, f64, f64)>(&format!(
        "local f = {expr}; return f:GetLeft() or 0, f:GetBottom() or 0, f:GetWidth() or 0, f:GetHeight() or 0"
    ))
    .unwrap_or((0.0, 0.0, 0.0, 0.0))
}

#[test]
fn professions_book_primary_spell_buttons_layout() {
    let env = setup_env();
    click(&env, "ProfessionMicroButton");

    let shown: bool = env
        .eval("return ProfessionsBookFrame ~= nil and ProfessionsBookFrame:IsShown() == true")
        .unwrap_or(false);
    assert!(shown, "ProfessionsBookFrame should be shown");

    let b1 = rect(&env, "PrimaryProfession1.SpellButton1");
    let b2 = rect(&env, "PrimaryProfession1.SpellButton2");
    let primary = rect(&env, "PrimaryProfession1");

    eprintln!(
        "PrimaryProfession1 L={} B={} W={} H={}",
        primary.0, primary.1, primary.2, primary.3
    );
    eprintln!(
        "SpellButton1         L={} B={} W={} H={}",
        b1.0, b1.1, b1.2, b1.3
    );
    eprintln!(
        "SpellButton2         L={} B={} W={} H={}",
        b2.0, b2.1, b2.2, b2.3
    );

    assert!(b1.2 > 0.0 && b1.3 > 0.0, "SpellButton1 must have size");
    assert!(b2.2 > 0.0 && b2.3 > 0.0, "SpellButton2 must have size");

    let b1_top = b1.1 + b1.3;
    let b2_bottom = b2.1;
    assert!(
        (b1_top - b2_bottom).abs() < 1.0,
        "SpellButton1 top ({b1_top}) should touch SpellButton2 bottom ({b2_bottom})"
    );

    assert!(
        (b1.0 - b2.0).abs() < 1.0,
        "SpellButton1 left ({}) should match SpellButton2 left ({})",
        b1.0,
        b2.0
    );

    let primary_right = primary.0 + primary.2;
    let primary_top = primary.1 + primary.3;
    let b2_right = b2.0 + b2.2;
    let b2_top = b2.1 + b2.3;
    assert!(
        (primary_right - b2_right - 109.0).abs() < 1.0,
        "SpellButton2 right ({b2_right}) should be 109 px inside PrimaryProfession1 right ({primary_right})"
    );
    assert!(
        (primary_top - b2_top - 3.0).abs() < 1.0,
        "SpellButton2 top ({b2_top}) should be 3 px below PrimaryProfession1 top ({primary_top})"
    );

    // Reference values captured on master:
    // PrimaryProfession1 L=96 B=537 W=437 H=81
    // SpellButton1       L=384 B=535 W=40 H=40
    // SpellButton2       L=384 B=575 W=40 H=40
    assert_eq!(
        primary,
        (96.0, 537.0, 437.0, 81.0),
        "PrimaryProfession1 rect mismatch vs master"
    );
    assert_eq!(
        b1,
        (384.0, 535.0, 40.0, 40.0),
        "SpellButton1 rect mismatch vs master"
    );
    assert_eq!(
        b2,
        (384.0, 575.0, 40.0, 40.0),
        "SpellButton2 rect mismatch vs master"
    );

    // Rank status bar must show formatted "<rank>/<max>", never the raw
    // "%d/%d" format string. Regression for SetFormattedText writing the
    // formatted result to the wrong stack slot when base != 0.
    let rank_text: String = env
        .eval("return tostring(PrimaryProfession1.statusBar.rankText:GetText())")
        .unwrap();
    assert!(
        !rank_text.contains("%d"),
        "statusBar.rankText should be formatted, got {rank_text:?}"
    );
    assert!(
        rank_text.contains('/'),
        "statusBar.rankText should be '<rank>/<max>', got {rank_text:?}"
    );
}

#[test]
fn professions_book_frame_layout_stays_locked() {
    let env = setup_env();
    click(&env, "ProfessionMicroButton");

    let result: String = env
        .eval(
            r#"
            local EPS = 0.75

            local function approx(actual, expected, eps)
                if type(actual) ~= "number" or type(expected) ~= "number" then
                    return false
                end
                return math.abs(actual - expected) <= (eps or EPS)
            end

            local function rect(frame, name)
                if type(frame) ~= "table" then
                    return nil, name .. "_missing"
                end
                local l, b, w, h = frame:GetRect()
                if not (l and b and w and h) then
                    return nil, name .. "_missing_rect"
                end
                return { l = l, b = b, w = w, h = h, r = l + w, t = b + h }, nil
            end

            local function has_point(frame, point, rel, rel_point, x, y, eps)
                for i = 1, frame:GetNumPoints() do
                    local p, r, rp, ox, oy = frame:GetPoint(i)
                    local rel_matches = (r == rel) or (r == nil and rel ~= nil and frame.GetParent and frame:GetParent() == rel)
                    if p == point and rel_matches and rp == rel_point and approx(ox or 0, x, eps) and approx(oy or 0, y, eps) then
                        return true
                    end
                end
                return false
            end

            if not ProfessionsBookFrame then
                return "professions_book_missing"
            end
            if not ProfessionsBookFrame:IsShown() then
                return "professions_book_hidden"
            end
            if not ProfessionsBookFrameInset then
                return "inset_missing"
            end
            if not ProfessionsContentFrame then
                return "content_missing"
            end
            if not PrimaryProfession1 then
                return "primary_missing"
            end
            if not SecondaryProfession1 or not SecondaryProfession2 or not SecondaryProfession3 then
                return "secondary_rows_missing"
            end

            local panel_rect, panel_err = rect(ProfessionsBookFrame, "panel")
            if not panel_rect then return panel_err end
            local inset_rect, inset_err = rect(ProfessionsBookFrameInset, "inset")
            if not inset_rect then return inset_err end
            local content_rect, content_err = rect(ProfessionsContentFrame, "content")
            if not content_rect then return content_err end
            local close_rect, close_err = rect(ProfessionsBookFrameCloseButton, "close")
            if not close_rect then return close_err end

            if not approx(panel_rect.l, 16) or not approx(panel_rect.b, 160) then
                return "panel_origin=" .. tostring(panel_rect.l) .. "," .. tostring(panel_rect.b)
            end
            if not approx(panel_rect.w, 550, 0.1) or not approx(panel_rect.h, 525, 0.1) then
                return "panel_size=" .. tostring(panel_rect.w) .. "x" .. tostring(panel_rect.h)
            end
            if not approx(inset_rect.l, 20, 2.0) or not approx(inset_rect.b, 164, 2.0) then
                return "inset_origin=" .. tostring(inset_rect.l) .. "," .. tostring(inset_rect.b)
            end
            if not approx(inset_rect.w, 540, 0.1) or not approx(inset_rect.h, 497, 0.1) then
                return "inset_size=" .. tostring(inset_rect.w) .. "x" .. tostring(inset_rect.h)
            end

            if not approx(content_rect.l, panel_rect.l) or not approx(content_rect.b, panel_rect.b) then
                return "content_origin=" .. tostring(content_rect.l) .. "," .. tostring(content_rect.b)
            end
            if not approx(content_rect.w, panel_rect.w, 0.1) or not approx(content_rect.h, panel_rect.h, 0.1) then
                return "content_size=" .. tostring(content_rect.w) .. "x" .. tostring(content_rect.h)
            end

            if not approx(close_rect.w, 24, 0.1) or not approx(close_rect.h, 24, 0.1) then
                return "close_size=" .. tostring(close_rect.w) .. "x" .. tostring(close_rect.h)
            end
            local title = ProfessionsBookFrame.TitleContainer and ProfessionsBookFrame.TitleContainer.TitleText
            if not title then
                title = ProfessionsBookFrame.TitleText
            end
            if not title then
                return "title_missing"
            end
            if (title:GetText() or "") ~= "Professions" then
                return "title_text=" .. tostring(title:GetText())
            end

            local primary_rect, primary_err = rect(PrimaryProfession1, "primary")
            if not primary_rect then return primary_err end
            local b1 = PrimaryProfession1.SpellButton1
            local b2 = PrimaryProfession1.SpellButton2
            if not b1 or not b2 then
                return "primary_spell_buttons_missing"
            end
            local b1_rect, b1_err = rect(b1, "b1")
            if not b1_rect then return b1_err end
            local b2_rect, b2_err = rect(b2, "b2")
            if not b2_rect then return b2_err end

            if not approx(primary_rect.l, 96) or not approx(primary_rect.b, 537) then
                return "primary_origin=" .. tostring(primary_rect.l) .. "," .. tostring(primary_rect.b)
            end
            if not approx(primary_rect.w, 437, 0.1) or not approx(primary_rect.h, 81, 0.1) then
                return "primary_size=" .. tostring(primary_rect.w) .. "x" .. tostring(primary_rect.h)
            end
            if not approx(b1_rect.l, 384) or not approx(b1_rect.b, 535) then
                return "b1_origin=" .. tostring(b1_rect.l) .. "," .. tostring(b1_rect.b)
            end
            if not approx(b2_rect.l, 384) or not approx(b2_rect.b, 575) then
                return "b2_origin=" .. tostring(b2_rect.l) .. "," .. tostring(b2_rect.b)
            end
            if not approx(b1_rect.w, 40, 0.1) or not approx(b1_rect.h, 40, 0.1) then
                return "b1_size=" .. tostring(b1_rect.w) .. "x" .. tostring(b1_rect.h)
            end
            if not approx(b2_rect.w, 40, 0.1) or not approx(b2_rect.h, 40, 0.1) then
                return "b2_size=" .. tostring(b2_rect.w) .. "x" .. tostring(b2_rect.h)
            end
            if not approx(b2_rect.b - b1_rect.b, 40) then
                return "primary_spell_vertical_spacing=" .. tostring(b2_rect.b - b1_rect.b)
            end
            if not approx(b1_rect.l, b2_rect.l) then
                return "primary_spell_horizontal_mismatch"
            end

            local s1_rect, s1_err = rect(SecondaryProfession1, "secondary1")
            if not s1_rect then return s1_err end
            local s2_rect, s2_err = rect(SecondaryProfession2, "secondary2")
            if not s2_rect then return s2_err end
            local s3_rect, s3_err = rect(SecondaryProfession3, "secondary3")
            if not s3_rect then return s3_err end

            if not approx(s1_rect.l, 96) then
                return "secondary1_origin=" .. tostring(s1_rect.l) .. "," .. tostring(s1_rect.b)
            end
            if not approx(s2_rect.l, 96) then
                return "secondary2_origin=" .. tostring(s2_rect.l) .. "," .. tostring(s2_rect.b)
            end
            if not approx(s3_rect.l, 96) then
                return "secondary3_origin=" .. tostring(s3_rect.l) .. "," .. tostring(s3_rect.b)
            end
            if not approx(s1_rect.w, 437, 0.1) or not approx(s1_rect.h, 46, 0.1) then
                return "secondary1_size=" .. tostring(s1_rect.w) .. "x" .. tostring(s1_rect.h)
            end
            if not approx(s2_rect.w, 437, 0.1) or not approx(s2_rect.h, 46, 0.1) then
                return "secondary2_size=" .. tostring(s2_rect.w) .. "x" .. tostring(s2_rect.h)
            end
            if not approx(s3_rect.w, 437, 0.1) or not approx(s3_rect.h, 46, 0.1) then
                return "secondary3_size=" .. tostring(s3_rect.w) .. "x" .. tostring(s3_rect.h)
            end
            if not approx(s1_rect.b - s2_rect.b, 76) then
                return "secondary1_to_2_vertical_delta=" .. tostring(s1_rect.b - s2_rect.b)
            end
            if not approx(s2_rect.b - s3_rect.b, 76) then
                return "secondary2_to_3_vertical_delta=" .. tostring(s2_rect.b - s3_rect.b)
            end
            if not has_point(SecondaryProfession2, "TOPLEFT", SecondaryProfession1, "BOTTOMLEFT", 0, -30, 0.1) then
                return "secondary2_anchor_mismatch"
            end
            if not has_point(SecondaryProfession3, "TOPLEFT", SecondaryProfession2, "BOTTOMLEFT", 0, -30, 0.1) then
                return "secondary3_anchor_mismatch"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "ProfessionsBookFrame layout should remain locked: {result}"
    );
}
