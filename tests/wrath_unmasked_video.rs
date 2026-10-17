//! Original Effects event; explicit BaseMip input, not a fresh-default claim.
#![cfg(feature = "client-wrath")]
#[path = "wrath_unmasked_support/mod.rs"]
mod support;
use support::{env, load, run};

#[test]
fn wrath_unmasked_original_effects_disables_unsupported_player_texture() {
    let env = env();
    run(&env, "RegisterCVar('BaseMip', '0')");
    load(
        &env,
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
    // FrameXML/VideoOptionsPanels.lua:418-425 installs the original custom
    // handler AFTER generic setup. :427-443 reaches the capability boundary.
    // Invoke the installed handler with its original event input, without
    // dispatching unrelated Resolution-panel callbacks (GetVideoCaps gap).
    run(
        &env,
        r#"
        local panel = VideoOptionsEffectsPanel
        assert(panel:GetScript('OnEvent') == VideoOptionsEffectsPanel_OnEvent)
        assert(panel:IsEventRegistered('PLAYER_ENTERING_WORLD'))
        local control = VideoOptionsEffectsPanelPlayerTexture
        control:Enable()
        assert(control:IsEnabled())
        panel:GetScript('OnEvent')(panel, 'PLAYER_ENTERING_WORLD')
        assert(IsPlayerResolutionAvailable() == false)
        assert(not control:IsEnabled())
        local slider = VideoOptionsEffectsPanelViewDistance
        local low, high = slider:GetMinMaxValues()
        assert(high > low)
        assert(slider:GetValueStep() == (high - low) / 10)
        assert(type(panel.controls) == 'table' and #panel.controls > 0)
        assert(VideoOptionsEffectsPanelTextureResolution:GetCurrentValue() == 1)
        assert(not panel:IsEventRegistered('PLAYER_ENTERING_WORLD'))
    "#,
    );
}
