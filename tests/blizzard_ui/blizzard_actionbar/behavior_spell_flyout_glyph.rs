//! Behavior pin: pending glyph casts flow through
//! `SpellFlyoutPopupButtonMixin:UpdateGlyphState`.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";
const GLYPHED_SPELL_ID: u32 = 19750;

struct GlyphStateProbe {
    highlight: bool,
    glyph_icon: bool,
    glyph_activate: bool,
    glyph_translation: bool,
}

#[test]
fn spell_flyout_pending_glyph_marks_valid_spell_buttons() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_pending_glyph(env);
        install_popup_button(env);

        update_glyph_state(env);

        assert_pending_glyph_highlights_popup_button(env);
    });
    }
}

fn seed_pending_glyph(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.glyph.pending_glyph_name = Some("Glyph of Flash of Light".to_string());
    state.glyph.pending_glyph_removal = false;
}

fn install_popup_button(env: &WowLuaEnv) {
    let installed: bool = env
        .eval(&format!(
            r#"
            local button = CreateFrame(
                "CheckButton",
                "SpellFlyoutGlyphBehaviorButton",
                SpellFlyout,
                "SpellFlyoutPopupButtonTemplate"
            )
            button.spellID = {GLYPHED_SPELL_ID}
            button:Show()
            return button.GlyphIcon ~= nil
                and button.AbilityHighlight ~= nil
                and button.AbilityHighlightAnim ~= nil
            "#
        ))
        .expect("glyph flyout popup button installation must run cleanly");
    assert!(
        installed,
        "glyph flyout popup button must have glyph texture fields"
    );
}

fn update_glyph_state(env: &WowLuaEnv) {
    let updated: bool = env
        .eval(
            r#"
            SpellFlyoutGlyphBehaviorButton:UpdateGlyphState()
            return true
            "#,
        )
        .expect("SpellFlyout popup button glyph update must run cleanly");
    assert!(updated, "UpdateGlyphState must complete");
}

fn assert_pending_glyph_highlights_popup_button(env: &WowLuaEnv) {
    let probe = glyph_state_probe(env);

    assert!(
        probe.highlight,
        "valid pending glyph spell must show AbilityHighlight"
    );
    assert!(
        !probe.glyph_icon,
        "non-activated pending glyph must not show GlyphIcon"
    );
    assert!(
        !probe.glyph_activate,
        "non-activated pending glyph must not show GlyphActivate"
    );
    assert!(
        !probe.glyph_translation,
        "non-activated pending glyph must not show GlyphTranslation"
    );
}

fn glyph_state_probe(env: &WowLuaEnv) -> GlyphStateProbe {
    let (highlight, glyph_icon, glyph_activate, glyph_translation) = env
        .eval(
            r#"
            local button = SpellFlyoutGlyphBehaviorButton
            return button.AbilityHighlight:IsShown() == true,
                   button.GlyphIcon:IsShown() == true,
                   button.GlyphActivate:IsShown() == true,
                   button.GlyphTranslation:IsShown() == true
            "#,
        )
        .expect("pending glyph state probe must run cleanly");

    GlyphStateProbe {
        highlight,
        glyph_icon,
        glyph_activate,
        glyph_translation,
    }
}
