//! `AnimaDiversionFrameMixin:OnEvent("ANIMA_DIVERSION_CLOSE")` behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const INSTALL_HIDE_SPY: &str = r#"
local frame = AnimaDiversionFrame
frame.mapID = C_Map.GetCurrentMapID()
frame.bolsterProgress = 0
frame.covenantData = { animaChannelActiveSoundKit = 0 }
frame:Show()

__animaCloseOriginalHideUIPanel = HideUIPanel
__animaCloseHideCallCount = 0
__animaCloseHideTargetMatches = false
HideUIPanel = function(panel)
    __animaCloseHideCallCount = __animaCloseHideCallCount + 1
    __animaCloseHideTargetMatches = panel == frame
    return __animaCloseOriginalHideUIPanel(panel)
end
"#;
const READ_HIDE_SPY: &str = r#"
local hideCallCount = __animaCloseHideCallCount
local hideTargetMatches = __animaCloseHideTargetMatches
local panelHidden = not AnimaDiversionFrame:IsShown()

HideUIPanel = __animaCloseOriginalHideUIPanel
__animaCloseOriginalHideUIPanel = nil
__animaCloseHideCallCount = nil
__animaCloseHideTargetMatches = nil

return hideCallCount, hideTargetMatches, panelHidden
"#;

#[test]
fn anima_close_event_hides_panel_once() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        env.exec(INSTALL_HIDE_SPY)
            .expect("ANIMA_DIVERSION_CLOSE hide spy setup must run cleanly");

        env.fire_event("ANIMA_DIVERSION_CLOSE")
            .expect("ANIMA_DIVERSION_CLOSE event dispatch must run cleanly");

        let state: CloseEventState = env
            .eval(READ_HIDE_SPY)
            .expect("ANIMA_DIVERSION_CLOSE hide spy readout must run cleanly");

        assert_close_event_state(state);
    });
}

type CloseEventState = (i64, bool, bool);

fn assert_close_event_state(state: CloseEventState) {
    let (hide_call_count, hide_target_matches, panel_hidden) = state;

    assert_eq!(
        hide_call_count, 1,
        "`ANIMA_DIVERSION_CLOSE` must call `HideUIPanel` exactly once"
    );
    assert!(
        hide_target_matches,
        "`ANIMA_DIVERSION_CLOSE` must pass `AnimaDiversionFrame` to `HideUIPanel`"
    );
    assert!(
        panel_hidden,
        "`ANIMA_DIVERSION_CLOSE` must hide `AnimaDiversionFrame`"
    );
}
