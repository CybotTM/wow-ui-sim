//! Behavior pin for `UIThemeContainerMixin:UpdateTheme` →
//! `UpdateFontStrings`.
//!
//! `UpdateFontStrings` walks the registered FontStrings and pushes both a
//! `SetFixedColor(fixed)` and a `SetTextColor(color:GetRGB())` per entry. The
//! `(fixed, color)` pair is selected from a file-local `textColors` table
//! keyed on `IsDarkMode(cvarValue) == QuestTextContrast.UseLightText(cvar)`:
//!
//! | cvarValue | UseLightText | Mode  | fixedColor | color                          |
//! |-----------|--------------|-------|------------|--------------------------------|
//! | 0         | `false`      | light | `false`    | `PARCHMENT_MATERIAL_TEXT_COLOR` |
//! | 4         | `true`       | dark  | `true`     | `STONE_MATERIAL_TEXT_COLOR`     |
//!
//! `PARCHMENT_MATERIAL_TEXT_COLOR` and `STONE_MATERIAL_TEXT_COLOR` are
//! referenced here but are never defined in the wow-ui-source vendor extract;
//! real WoW seeds them from the C side, so the test installs sentinel
//! `CreateColor` fixtures with distinguishable RGB triples before invoking
//! `UpdateTheme`. The assertion is then `fs.text_color == fixture` plus
//! `frame.text_color_fixed == expected`, which proves both `SetTextColor` and
//! `SetFixedColor` were dispatched against the registered FontString.
//!
//! `text_color_fixed` is read directly off the simulator's widget state because
//! there is no `IsFixedColor` getter on the Lua surface — the only place
//! `SetFixedColor` writes is `frame.text_color_fixed`, so reading that field is
//! the strictly-necessary check.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccessibilityTemplates";
const FONTSTRING_NAME: &str = "ThemeUpdateFS";
const PARCHMENT_RGB: (f32, f32, f32) = (0.40, 0.30, 0.20);
const STONE_RGB: (f32, f32, f32) = (0.95, 0.85, 0.50);

fn install_theme_fixtures_and_container(env: &WowLuaEnv) {
    seed_material_color_globals_and_container(env);
    repoint_text_colors_upvalue(env);
    assert_exactly_one_font_string_registered(env);
}

fn seed_material_color_globals_and_container(env: &WowLuaEnv) {
    let setup = format!(
        r#"
        PARCHMENT_MATERIAL_TEXT_COLOR = CreateColor({pr}, {pg}, {pb}, 1)
        STONE_MATERIAL_TEXT_COLOR     = CreateColor({sr}, {sg}, {sb}, 1)
        local container = CreateFrame("Frame", "ThemeUpdateContainer", UIParent, "UIThemeContainerFrame")
        local fs = container:CreateFontString("{FONTSTRING_NAME}", "ARTWORK")
        container:RegisterFontString(fs)
        "#,
        pr = PARCHMENT_RGB.0,
        pg = PARCHMENT_RGB.1,
        pb = PARCHMENT_RGB.2,
        sr = STONE_RGB.0,
        sg = STONE_RGB.1,
        sb = STONE_RGB.2,
    );
    env.exec(&setup)
        .expect("failed to install theme fixtures and UIThemeContainer instance");
}

// AccessibilityTemplates.lua captures `textColors` as a file-local upvalue on
// UpdateFontStrings at addon-load time, when PARCHMENT_MATERIAL_TEXT_COLOR /
// STONE_MATERIAL_TEXT_COLOR are still nil (real WoW seeds them from C; the
// vendor extract does not). Walk UpdateFontStrings' upvalues, find the
// `textColors` table, and re-seed its color slots in-place. Without this,
// UpdateFontStrings dispatches SetTextColor(nil:GetRGB()) → (1,1,1) fallback.
fn repoint_text_colors_upvalue(env: &WowLuaEnv) {
    env.exec(
        r#"
        local idx = 1
        while true do
            local name, value = debug.getupvalue(UIThemeContainerMixin.UpdateFontStrings, idx)
            if not name then break end
            if name == "textColors" then
                value[false][2] = PARCHMENT_MATERIAL_TEXT_COLOR
                value[true][2]  = STONE_MATERIAL_TEXT_COLOR
                break
            end
            idx = idx + 1
        end
        "#,
    )
    .expect("failed to repoint textColors upvalue on UpdateFontStrings");
}

fn assert_exactly_one_font_string_registered(env: &WowLuaEnv) {
    let registered_count: i32 = env
        .eval(
            "local n = 0; for _ in pairs(ThemeUpdateContainer.fontStrings or {}) do n = n + 1 end; return n",
        )
        .expect("failed to count registered FontStrings");
    assert_eq!(
        registered_count, 1,
        "Setup expected exactly one FontString to land in the container's \
         `fontStrings` set (the one we just RegisterFontString'd); got \
         {registered_count}. If this is 0, either the intrinsic mixin's \
         OnPreLoad never seeded `self.fontStrings = {{}}` or RegisterFontString \
         dispatched against a different self."
    );
}

fn read_font_string_text_color(env: &WowLuaEnv) -> (f32, f32, f32) {
    let probe = format!("local r, g, b = {FONTSTRING_NAME}:GetTextColor(); return r, g, b");
    let (r, g, b): (f64, f64, f64) = env
        .eval(&probe)
        .expect("failed to read FontString text color");
    (r as f32, g as f32, b as f32)
}

fn read_font_string_fixed_color(env: &WowLuaEnv) -> bool {
    let state = env.state();
    let state_ref = state.borrow();
    let id = state_ref
        .widgets
        .get_id_by_name(FONTSTRING_NAME)
        .unwrap_or_else(|| panic!("FontString `{FONTSTRING_NAME}` missing from registry"));
    state_ref
        .widgets
        .get(id)
        .unwrap_or_else(|| panic!("FontString `{FONTSTRING_NAME}` widget vanished after lookup"))
        .text_color_fixed
}

fn assert_rgb_close(actual: (f32, f32, f32), expected: (f32, f32, f32), label: &str) {
    let (ar, ag, ab) = actual;
    let (er, eg, eb) = expected;
    let close = (ar - er).abs() < 1e-4 && (ag - eg).abs() < 1e-4 && (ab - eb).abs() < 1e-4;
    assert!(
        close,
        "Expected `{FONTSTRING_NAME}` text color in {label} mode to match \
         ({er:.4}, {eg:.4}, {eb:.4}); got ({ar:.4}, {ag:.4}, {ab:.4}). \
         If this regresses, `UpdateFontStrings` either skipped the registered \
         FontString or unpacked the wrong `textColors` row."
    );
}

#[test]
fn update_theme_light_mode_clears_fixed_color_and_paints_parchment() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        install_theme_fixtures_and_container(env);
        env.exec("ThemeUpdateContainer:UpdateTheme(0)")
            .expect("UpdateTheme(0) raised");

        assert_rgb_close(read_font_string_text_color(env), PARCHMENT_RGB, "light");
        assert!(
            !read_font_string_fixed_color(env),
            "Light mode (cvarValue=0) must dispatch SetFixedColor(false) — the \
             `textColors[CONTRAST_LIGHT_MODE]` row pairs `false` with PARCHMENT, \
             so a `true` here means UpdateFontStrings inverted the boolean."
        );
    });
}

#[test]
fn update_theme_dark_mode_sets_fixed_color_and_paints_stone() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        install_theme_fixtures_and_container(env);
        env.exec("ThemeUpdateContainer:UpdateTheme(4)")
            .expect("UpdateTheme(4) raised");

        assert_rgb_close(read_font_string_text_color(env), STONE_RGB, "dark");
        assert!(
            read_font_string_fixed_color(env),
            "Dark mode (cvarValue=4) must dispatch SetFixedColor(true) — the \
             `textColors[CONTRAST_DARK_MODE]` row pairs `true` with STONE, so a \
             `false` here means UpdateFontStrings dropped the SetFixedColor call."
        );
    });
}
