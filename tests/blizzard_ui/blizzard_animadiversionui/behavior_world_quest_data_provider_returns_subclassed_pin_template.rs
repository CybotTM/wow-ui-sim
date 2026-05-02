//! Anima Diversion world-quest provider and pin template probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const WORLD_QUEST_PIN_PROBE: &str = r#"
local provider = CreateFromMixins(AnimaDiversion_WorldQuestDataProviderMixin)
local templateName = provider:GetPinTemplate()
local calls = {}
local normalTexture = {}
local pin = {
    NormalTexture = normalTexture,
    SetDefaultMapPinScale = function()
        table.insert(calls, { name = "base" })
    end,
    OnMouseEnter = function() end,
    SetAlphaLimits = function(self, scaleStart, alphaStart, alphaEnd)
        table.insert(calls, {
            name = "alpha",
            first = scaleStart,
            second = alphaStart,
            third = alphaEnd,
        })
    end,
    SetScalingLimits = function(self, scaleStart, scaleMin, scaleMax)
        table.insert(calls, {
            name = "scaling",
            first = scaleStart,
            second = scaleMin,
            third = scaleMax,
        })
    end,
    SetNudgeTargetFactor = function(self, value)
        table.insert(calls, { name = "target", first = value })
    end,
    SetNudgeZoomedOutFactor = function(self, value)
        table.insert(calls, { name = "zoomedOut", first = value })
    end,
    SetNudgeZoomedInFactor = function(self, value)
        table.insert(calls, { name = "zoomedIn", first = value })
    end,
}
setmetatable(pin, { __index = AnimaDiversion_WorldQuestPinMixin })

pin:OnLoad()

local baseRanFirst = calls[1] and calls[1].name == "base"
local alphaMatches = calls[2]
    and calls[2].name == "alpha"
    and calls[2].first == 2.0
    and calls[2].second == 0.6
    and calls[2].third == 0.6
local scalingMatches = calls[3]
    and calls[3].name == "scaling"
    and calls[3].first == 1
    and calls[3].second == 0.4125
    and calls[3].third == 0.425
local nudgeMatches = calls[4]
    and calls[4].name == "target"
    and calls[4].first == 0.015
    and calls[5].name == "zoomedOut"
    and calls[5].first == 1.0
    and calls[6].name == "zoomedIn"
    and calls[6].first == 0.25

return templateName,
       #calls,
       baseRanFirst,
       pin.UpdateTooltip == pin.OnMouseEnter,
       pin.widgetAnimationTexture == normalTexture,
       alphaMatches,
       scalingMatches,
       nudgeMatches
"#;

#[test]
fn world_quest_provider_uses_subclassed_pin_template_and_onload_limits() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: WorldQuestPinState = env
            .eval(WORLD_QUEST_PIN_PROBE)
            .expect("world quest pin probe must run cleanly");

        assert_world_quest_pin_state(state);
    });
}

type WorldQuestPinState = (String, i64, bool, bool, bool, bool, bool, bool);

fn assert_world_quest_pin_state(state: WorldQuestPinState) {
    assert_eq!(
        state.0, "AnimaDiversion_WorldQuestPinTemplate",
        "Provider must return the Anima Diversion world quest pin template"
    );
    assert_onload_call_order((state.1, state.2));
    assert_base_onload_side_effects((state.3, state.4));
    assert_anima_diversion_limits((state.5, state.6, state.7));
}

fn assert_onload_call_order(state: (i64, bool)) {
    let (call_count, base_ran_first) = state;

    assert_eq!(call_count, 6, "OnLoad must make the expected setup calls");
    assert!(base_ran_first, "WorldQuestPinMixin.OnLoad must run first");
}

fn assert_base_onload_side_effects(state: (bool, bool)) {
    let (tooltip_matches, animation_texture_matches) = state;

    assert!(tooltip_matches, "Base OnLoad must set UpdateTooltip");
    assert!(
        animation_texture_matches,
        "Base OnLoad must copy NormalTexture to widgetAnimationTexture"
    );
}

fn assert_anima_diversion_limits(state: (bool, bool, bool)) {
    let (alpha_matches, scaling_matches, nudge_matches) = state;

    assert!(
        alpha_matches,
        "OnLoad must set Anima Diversion alpha limits"
    );
    assert!(
        scaling_matches,
        "OnLoad must set Anima Diversion scaling limits"
    );
    assert!(
        nudge_matches,
        "OnLoad must set Anima Diversion nudge factors"
    );
}
