//! `AnimaDiversionFrameMixin:OnShow` behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const EXPECTED_CALLS: &[&str] = &[
    "UpdateTutorialTips",
    "SetMapID",
    "MapCanvasMixin.OnShow",
    "ResetZoom",
    "RegisterFrameForEvents",
];
const ON_SHOW_PROBE: &str = r#"
local frame = AnimaDiversionFrame
frame.mapID = 1543
frame.covenantData = { animaChannelActiveSoundKit = 778899 }

local calls = {}
local sounds = {}
local mapIDSeen = nil
local mapCanvasSelfMatches = false
local registerSelfMatches = false
local registeredEvents = nil
local openSoundKit = SOUNDKIT.UI_COVENANT_ANIMA_DIVERSION_OPEN

local originalUpdateTutorialTips = frame.UpdateTutorialTips
local originalSetMapID = frame.SetMapID
local originalMapCanvasOnShow = MapCanvasMixin.OnShow
local originalResetZoom = frame.ResetZoom
local originalRegisterFrameForEvents = FrameUtil.RegisterFrameForEvents
local originalPlaySound = PlaySound
local originalIsAnyNodeActive = AnimaDiversionUtil.IsAnyNodeActive

frame.UpdateTutorialTips = function(self)
    table.insert(calls, "UpdateTutorialTips")
end
frame.SetMapID = function(self, mapID)
    table.insert(calls, "SetMapID")
    mapIDSeen = mapID
end
MapCanvasMixin.OnShow = function(self)
    table.insert(calls, "MapCanvasMixin.OnShow")
    mapCanvasSelfMatches = self == frame
end
frame.ResetZoom = function(self)
    table.insert(calls, "ResetZoom")
end
FrameUtil.RegisterFrameForEvents = function(target, events)
    table.insert(calls, "RegisterFrameForEvents")
    registerSelfMatches = target == frame
    registeredEvents = events
end
PlaySound = function(soundKit)
    table.insert(sounds, soundKit)
end

AnimaDiversionUtil.IsAnyNodeActive = function()
    return false
end
frame:OnShow()
local inactiveSoundCount = #sounds
local inactiveOpenSound = sounds[1]

AnimaDiversionUtil.IsAnyNodeActive = function()
    return true
end
frame:OnShow()
local activeOpenSound = sounds[2]
local activeChannelSound = sounds[3]

frame.UpdateTutorialTips = originalUpdateTutorialTips
frame.SetMapID = originalSetMapID
MapCanvasMixin.OnShow = originalMapCanvasOnShow
frame.ResetZoom = originalResetZoom
FrameUtil.RegisterFrameForEvents = originalRegisterFrameForEvents
PlaySound = originalPlaySound
AnimaDiversionUtil.IsAnyNodeActive = originalIsAnyNodeActive

return calls,
       mapIDSeen,
       mapCanvasSelfMatches,
       registerSelfMatches,
       registeredEvents,
       openSoundKit,
       inactiveSoundCount,
       inactiveOpenSound,
       activeOpenSound,
       activeChannelSound
"#;

#[test]
fn onshow_registers_events_and_plays_expected_sounds() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: OnShowState = env
            .eval(ON_SHOW_PROBE)
            .expect("AnimaDiversionFrame OnShow probe must run cleanly");

        assert_on_show_state(state);
    });
}

type OnShowState = (
    Vec<String>,
    i64,
    bool,
    bool,
    Vec<String>,
    i64,
    i64,
    i64,
    i64,
    i64,
);

fn assert_on_show_state(state: OnShowState) {
    let (
        calls,
        map_id_seen,
        map_canvas_self_matches,
        register_self_matches,
        registered_events,
        open_sound_kit,
        inactive_sound_count,
        inactive_open_sound,
        active_open_sound,
        active_channel_sound,
    ) = state;

    assert_call_order(&calls);
    assert_eq!(
        map_id_seen, 1543,
        "`OnShow` must call `SetMapID(self.mapID)`"
    );
    assert!(
        map_canvas_self_matches,
        "`OnShow` must pass the frame to `MapCanvasMixin.OnShow`"
    );
    assert!(
        register_self_matches,
        "`OnShow` must register events on `AnimaDiversionFrame`"
    );
    assert_registered_events(&registered_events);
    assert_open_sound_branch(open_sound_kit, inactive_sound_count, inactive_open_sound);
    assert_active_node_sound_branch(open_sound_kit, active_open_sound, active_channel_sound);
}

fn assert_call_order(calls: &[String]) {
    for (index, expected) in EXPECTED_CALLS.iter().enumerate() {
        assert_eq!(
            calls[index], *expected,
            "`OnShow` must call `{expected}` in the expected sequence"
        );
    }
}

fn assert_registered_events(events: &[String]) {
    assert!(
        events.contains(&"ANIMA_DIVERSION_CLOSE".to_string()),
        "`OnShow` must register `ANIMA_DIVERSION_CLOSE`"
    );
    assert!(
        events.contains(&"CURRENCY_DISPLAY_UPDATE".to_string()),
        "`OnShow` must register `CURRENCY_DISPLAY_UPDATE`"
    );
}

fn assert_open_sound_branch(open_sound_kit: i64, sound_count: i64, open_sound: i64) {
    assert_eq!(
        sound_count, 1,
        "`OnShow` must only play the open sound when no anima node is active"
    );
    assert_eq!(
        open_sound, open_sound_kit,
        "`OnShow` must play `SOUNDKIT.UI_COVENANT_ANIMA_DIVERSION_OPEN`"
    );
}

fn assert_active_node_sound_branch(open_sound_kit: i64, open_sound: i64, channel_sound: i64) {
    assert_eq!(
        open_sound, open_sound_kit,
        "`OnShow` must always play the open sound first"
    );
    assert_eq!(
        channel_sound, 778899,
        "`OnShow` must play `covenantData.animaChannelActiveSoundKit` when a node is active"
    );
}
