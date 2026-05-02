//! `AnimaDiversionPinMixin:SetVisualState` disabled-state probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const SET_VISUAL_STATE_PROBE: &str = r#"
local state = Enum.AnimaDiversionNodeState
local host = CreateFrame("Frame")
local pin = {
    textureKit = "Kyrian",
    Icon = host:CreateTexture(nil, "ARTWORK"),
    IconDisabledOverlay = host:CreateTexture(nil, "OVERLAY"),
}
setmetatable(pin, { __index = AnimaDiversionPinMixin })

pin:SetVisualState(state.Unavailable)

local disabledR, disabledG, disabledB, disabledA = pin.IconDisabledOverlay:GetVertexColor()
local unavailableOverlayShown = pin.IconDisabledOverlay:IsShown()
local unavailableIconDesaturated = pin.Icon:IsDesaturated()

local otherStates = {
    state.Available,
    state.SelectedTemporary,
    state.SelectedPermanent,
    state.Cooldown,
}
local otherStatesHideOverlay = true
local otherStatesClearDesaturation = true

for _, visualState in ipairs(otherStates) do
    pin.IconDisabledOverlay:Show()
    pin.Icon:SetDesaturated(true)
    pin:SetVisualState(visualState)

    otherStatesHideOverlay = otherStatesHideOverlay and not pin.IconDisabledOverlay:IsShown()
    otherStatesClearDesaturation = otherStatesClearDesaturation and not pin.Icon:IsDesaturated()
end

return disabledR,
       disabledG,
       disabledB,
       disabledA,
       unavailableOverlayShown,
       unavailableIconDesaturated,
       otherStatesHideOverlay,
       otherStatesClearDesaturation
"#;

#[test]
fn set_visual_state_dims_only_unavailable_pins() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: VisualStateProbe = env
            .eval(SET_VISUAL_STATE_PROBE)
            .expect("set visual state probe must run cleanly");

        assert_visual_state_probe(state);
    });
}

type VisualStateProbe = (f32, f32, f32, f32, bool, bool, bool, bool);

fn assert_visual_state_probe(state: VisualStateProbe) {
    assert_unavailable_overlay_color((state.0, state.1, state.2, state.3));
    assert!(
        state.4,
        "Unavailable state must show the disabled overlay texture"
    );
    assert!(
        state.5,
        "Unavailable state must desaturate the pin icon texture"
    );
    assert!(
        state.6,
        "Every non-unavailable state must hide the disabled overlay"
    );
    assert!(
        state.7,
        "Every non-unavailable state must clear icon desaturation"
    );
}

fn assert_unavailable_overlay_color(color: (f32, f32, f32, f32)) {
    let (red, green, blue, alpha) = color;

    assert_eq!(red, 0.0, "Disabled overlay red channel");
    assert_eq!(green, 0.0, "Disabled overlay green channel");
    assert_eq!(blue, 0.0, "Disabled overlay blue channel");
    assert!(
        (alpha - 0.4).abs() < 0.001,
        "Disabled overlay alpha should be 0.4, got {alpha}"
    );
}
