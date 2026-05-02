//! `AnimaDiversionFrameMixin:OnHide` behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const EXPECTED_CALLS: &[&str] = &[
    "MapCanvasMixin.OnHide",
    "UnregisterFrameForEvents",
    "ReinforceInfoFrame.Hide",
    "StopSound",
    "PlaySound",
];
const ON_HIDE_PROBE: &str = r#"
local frame = AnimaDiversionFrame
local reinforceInfo = frame.ReinforceInfoFrame
frame.gemsFullSoundHandle = 4242
reinforceInfo:Show()

local calls = {}
local unregisterSelfMatches = false
local unregisteredEvents = nil
local reinforceSelfMatches = false
local mapCanvasSelfMatches = false
local stoppedSoundHandle = nil
local playedSound = nil
local closeSoundKit = SOUNDKIT.UI_COVENANT_ANIMA_DIVERSION_CLOSE

local originalMapCanvasOnHide = MapCanvasMixin.OnHide
local originalUnregisterFrameForEvents = FrameUtil.UnregisterFrameForEvents
local originalReinforceHide = reinforceInfo.Hide
local originalStopSound = StopSound
local originalPlaySound = PlaySound

MapCanvasMixin.OnHide = function(self)
    table.insert(calls, "MapCanvasMixin.OnHide")
    mapCanvasSelfMatches = self == frame
end
FrameUtil.UnregisterFrameForEvents = function(target, events)
    table.insert(calls, "UnregisterFrameForEvents")
    unregisterSelfMatches = target == frame
    unregisteredEvents = events
end
reinforceInfo.Hide = function(self)
    table.insert(calls, "ReinforceInfoFrame.Hide")
    reinforceSelfMatches = self == reinforceInfo
    return originalReinforceHide(self)
end
StopSound = function(soundHandle)
    table.insert(calls, "StopSound")
    stoppedSoundHandle = soundHandle
end
PlaySound = function(soundKit)
    table.insert(calls, "PlaySound")
    playedSound = soundKit
end

frame:OnHide()
local gemsFullSoundHandleCleared = frame.gemsFullSoundHandle == nil
local reinforceInfoHidden = not reinforceInfo:IsShown()

MapCanvasMixin.OnHide = originalMapCanvasOnHide
FrameUtil.UnregisterFrameForEvents = originalUnregisterFrameForEvents
reinforceInfo.Hide = originalReinforceHide
StopSound = originalStopSound
PlaySound = originalPlaySound

return calls,
       mapCanvasSelfMatches,
       unregisterSelfMatches,
       unregisteredEvents,
       reinforceSelfMatches,
       reinforceInfoHidden,
       stoppedSoundHandle,
       gemsFullSoundHandleCleared,
       closeSoundKit,
       playedSound
"#;

#[test]
fn onhide_unregisters_events_stops_gems_and_plays_close_sound() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: OnHideState = env
            .eval(ON_HIDE_PROBE)
            .expect("AnimaDiversionFrame OnHide probe must run cleanly");

        assert_on_hide_state(state);
    });
}

type OnHideState = (
    Vec<String>,
    bool,
    bool,
    Vec<String>,
    bool,
    bool,
    i64,
    bool,
    i64,
    i64,
);

fn assert_on_hide_state(state: OnHideState) {
    let (
        calls,
        map_canvas_self_matches,
        unregister_self_matches,
        unregistered_events,
        reinforce_self_matches,
        reinforce_info_hidden,
        stopped_sound_handle,
        gems_full_sound_handle_cleared,
        close_sound_kit,
        played_sound,
    ) = state;

    assert_call_order(&calls);
    assert!(
        map_canvas_self_matches,
        "`OnHide` must pass the frame to `MapCanvasMixin.OnHide`"
    );
    assert!(
        unregister_self_matches,
        "`OnHide` must unregister events from `AnimaDiversionFrame`"
    );
    assert_unregistered_events(&unregistered_events);
    assert_reinforce_info_hidden(reinforce_self_matches, reinforce_info_hidden);
    assert_gems_full_sound_stopped(stopped_sound_handle, gems_full_sound_handle_cleared);
    assert_eq!(
        played_sound, close_sound_kit,
        "`OnHide` must play `SOUNDKIT.UI_COVENANT_ANIMA_DIVERSION_CLOSE`"
    );
}

fn assert_call_order(calls: &[String]) {
    for (index, expected) in EXPECTED_CALLS.iter().enumerate() {
        assert_eq!(
            calls[index], *expected,
            "`OnHide` must call `{expected}` in the expected sequence"
        );
    }
}

fn assert_unregistered_events(events: &[String]) {
    assert!(
        events.contains(&"ANIMA_DIVERSION_CLOSE".to_string()),
        "`OnHide` must unregister `ANIMA_DIVERSION_CLOSE`"
    );
    assert!(
        events.contains(&"CURRENCY_DISPLAY_UPDATE".to_string()),
        "`OnHide` must unregister `CURRENCY_DISPLAY_UPDATE`"
    );
}

fn assert_reinforce_info_hidden(self_matches: bool, hidden: bool) {
    assert!(
        self_matches,
        "`OnHide` must call `Hide` on `ReinforceInfoFrame`"
    );
    assert!(hidden, "`OnHide` must hide `ReinforceInfoFrame`");
}

fn assert_gems_full_sound_stopped(stopped_sound_handle: i64, handle_cleared: bool) {
    assert_eq!(
        stopped_sound_handle, 4242,
        "`OnHide` must stop the current `gemsFullSoundHandle`"
    );
    assert!(
        handle_cleared,
        "`OnHide` must clear `gemsFullSoundHandle` through `StopGemsFullSound`"
    );
}
