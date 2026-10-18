//! Approved simulator policies through unchanged, cache-pinned Wrath consumers.
#![cfg(feature = "client-wrath")]
#[path = "wrath_unmasked_support/mod.rs"]
mod support;
use support::{env, load, run};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_video_options(env: &WowLuaEnv) {
    load(
        env,
        &[
            "GlobalStrings.lua",
            "Constants.lua",
            "Fonts.xml",
            "FontStyles.xml",
            "UIPanelTemplates.lua",
            "UIPanelTemplates.xml",
            "UIDropDownMenu.xml",
            "OptionsFrameTemplates.xml",
            "OptionsPanelTemplates.xml",
            "VideoOptionsFrame.xml",
            "VideoOptionsPanels.xml",
        ],
    );
}

#[test]
fn wrath_gamma_initial_state_is_neutral() {
    let env = env();
    run(&env, "assert(GetGamma() == 0, 'approved neutral initial gamma')");
    env.state().borrow_mut().legacy_gamma = -0.25;
    run(&env, "assert(GetGamma() == -0.25); SetGamma(0.25)");
    assert_eq!(env.state().borrow().legacy_gamma, 0.25);
    let other = support::env();
    run(&other, "assert(GetGamma() == 0)");
}

#[test]
fn wrath_gamma_original_resolution_range_callback_and_round_trip() {
    let env = env();
    load_video_options(&env);
    // Reproduce the startup failure boundary: range reconciliation clamps an
    // existing out-of-range display value before SetupControl reads GetGamma.
    // Supply widget state without invoking/replacing the original Lua callback.
    {
        let mut state = env.state().borrow_mut();
        let id = state
            .widgets
            .get_id_by_name("VideoOptionsResolutionPanelGammaSlider")
            .unwrap();
        state.widgets.get_mut(id).unwrap().slider_value = 1.0;
    }
    run(
        &env,
        r#"
        local panel = VideoOptionsResolutionPanel
        local slider = VideoOptionsResolutionPanelGammaSlider
        assert(panel:IsShown(), 'original gamma parent must remain shown')
        assert(panel:GetScript('OnEvent') == VideoOptionsResolutionPanel_OnEvent)
        assert(panel:IsEventRegistered('PLAYER_ENTERING_WORLD'))
        panel:GetScript('OnEvent')(panel, 'PLAYER_ENTERING_WORLD')
        local low, high = slider:GetMinMaxValues()
        assert(low == -0.5 and high == 0.5)
        assert(slider:GetValueStep() == 0.1)
        assert(slider.newValue == 0.5, 'range reconciliation must run OnValueChanged')
        assert(GetGamma() == 0.5 and slider.value == 0.5)
        assert(not panel:IsEventRegistered('PLAYER_ENTERING_WORLD'))
        for _, value in ipairs({-0.5, 0, 0.5}) do
            slider:SetValue(value)
            assert(GetGamma() == value)
            assert(slider:GetCurrentValue() == value)
            assert(slider:GetValue() == value)
        end
        "#,
    );
}

#[test]
fn wrath_basemip_original_effects_fresh_default_and_inverse_setter() {
    let env = env();
    load_video_options(&env);
    run(
        &env,
        r#"
        local panel = VideoOptionsEffectsPanel
        assert(panel:GetScript('OnEvent') == VideoOptionsEffectsPanel_OnEvent)
        assert(panel:IsEventRegistered('PLAYER_ENTERING_WORLD'))
        panel:GetScript('OnEvent')(panel, 'PLAYER_ENTERING_WORLD')
        local slider = VideoOptionsEffectsPanelTextureResolution
        assert(GetCVarDefault('bAsEmIp') == '0')
        assert(GetCVar('BaseMip') == GetCVarDefault('BaseMip'), 'no supplied override')
        assert(slider:GetCurrentValue() == 1 and slider.value == 1)
        assert(not panel:IsEventRegistered('PLAYER_ENTERING_WORLD'))
        for _, value in ipairs({0, 1}) do
            slider:SetValue(value)
            assert(tonumber(GetCVar('BaseMip')) == 1 - value)
            assert(slider:GetCurrentValue() == value)
            assert(GetCVarDefault('BaseMip') == '0')
        end
        "#,
    );
}

#[test]
fn wrath_basemip_original_effects_preserves_supplied_current_values() {
    for value in [0, 1] {
        let env = env();
        env.state().borrow().cvars.set("BaseMip", &value.to_string());
        load_video_options(&env);
        run(
            &env,
            &format!(
                "local panel = VideoOptionsEffectsPanel; \
                 panel:GetScript('OnEvent')(panel, 'PLAYER_ENTERING_WORLD'); \
                 local slider = VideoOptionsEffectsPanelTextureResolution; \
                 assert(tonumber(GetCVar('BASEMIP')) == {value}); \
                 assert(GetCVarDefault('BaseMip') == '0'); \
                 assert(slider:GetCurrentValue() == 1 - {value}); \
                 assert(slider.value == 1 - {value})"
            ),
        );
    }
}
