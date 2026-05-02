//! Event-registration surface for `Blizzard_AnimaDiversionUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const FRAME_SHOW_EVENTS: &[&str] = &["ANIMA_DIVERSION_CLOSE", "CURRENCY_DISPLAY_UPDATE"];
const EVENT_LIFECYCLE_PROBE: &str = r#"
AnimaDiversionFrame:Hide()
local beforeShow = {
    AnimaDiversionFrame:IsEventRegistered("ANIMA_DIVERSION_CLOSE"),
    AnimaDiversionFrame:IsEventRegistered("CURRENCY_DISPLAY_UPDATE"),
}

AnimaDiversionFrame.mapID = C_Map.GetCurrentMapID()
AnimaDiversionFrame.bolsterProgress = 0
AnimaDiversionFrame.covenantData = { animaChannelActiveSoundKit = 0 }
AnimaDiversionFrame:Show()

local afterShow = {
    AnimaDiversionFrame:IsEventRegistered("ANIMA_DIVERSION_CLOSE"),
    AnimaDiversionFrame:IsEventRegistered("CURRENCY_DISPLAY_UPDATE"),
}

AnimaDiversionFrame:Hide()
local afterHide = {
    AnimaDiversionFrame:IsEventRegistered("ANIMA_DIVERSION_CLOSE"),
    AnimaDiversionFrame:IsEventRegistered("CURRENCY_DISPLAY_UPDATE"),
}

return beforeShow, afterShow, afterHide
"#;

#[test]
fn anima_diversion_frame_registers_show_events_only_while_shown() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let lifecycle: FrameEventLifecycle = env
            .eval(EVENT_LIFECYCLE_PROBE)
            .expect("AnimaDiversionFrame show/hide event probe must run cleanly");

        assert_frame_event_lifecycle(lifecycle);
    });
}

type FrameEventLifecycle = (Vec<bool>, Vec<bool>, Vec<bool>);

fn assert_frame_event_lifecycle(lifecycle: FrameEventLifecycle) {
    let (before_show, after_show, after_hide) = lifecycle;

    for (index, event) in FRAME_SHOW_EVENTS.iter().enumerate() {
        assert_show_event_lifecycle(
            event,
            before_show[index],
            after_show[index],
            after_hide[index],
        );
    }
}

fn assert_show_event_lifecycle(event: &str, before_show: bool, after_show: bool, after_hide: bool) {
    assert!(
        !before_show,
        "`AnimaDiversionFrame` must not register `{event}` while hidden"
    );
    assert!(
        after_show,
        "`AnimaDiversionFrameMixin:OnShow` must register `{event}`"
    );
    assert!(
        !after_hide,
        "`AnimaDiversionFrameMixin:OnHide` must unregister `{event}`"
    );
}
