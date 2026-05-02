//! Quest-offer pin clicks pan, show the dialog, and anchor the area trigger.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const QUEST_ID: i64 = 40519;
const NORMALIZED_X: f64 = 0.37;
const NORMALIZED_Y: f64 = 0.62;
const EXPECTED_STRETCH: f64 = 0.1;
const QUEST_OFFER_CLICK_PROBE: &str = r#"
local questID = __questOfferClickQuestID
local previousOfferActive = __questOfferClickPreviousActive
local mapCanvas = {
    AcquireAreaTrigger = function(self, namespace)
        self.acquiredNamespace = namespace
        local trigger = {
            resetCount = 0,
            Reset = function(self)
                self.resetCount = self.resetCount + 1
            end,
            SetCenter = function(self, normalizedX, normalizedY)
                self.centerX = normalizedX
                self.centerY = normalizedY
            end,
            Stretch = function(self, x, y)
                self.stretchX = x
                self.stretchY = y
            end,
        }
        self.acquiredTrigger = trigger
        return trigger
    end,
    SetAreaTriggerEnclosedCallback = function(self, trigger, callback)
        self.callbackTrigger = trigger
        self.callbackIsFunction = type(callback) == "function"
    end,
}
local pin = {
    questID = questID,
    panToCount = 0,
    panAndZoomToCount = 0,
    PanTo = function(self)
        self.panToCount = self.panToCount + 1
    end,
    PanAndZoomTo = function(self)
        self.panAndZoomToCount = self.panAndZoomToCount + 1
    end,
    GetGlobalPosition = function()
        return __questOfferClickNormalizedX, __questOfferClickNormalizedY
    end,
}
local previousPin = {}
local provider = CreateFromMixins(AdventureMap_QuestOfferDataProviderMixin)
provider.owningMap = mapCanvas
provider.currentOfferPin = previousOfferActive and previousPin or nil

local originalShowWithQuest = AdventureMapQuestChoiceDialog.ShowWithQuest
local originalSetPortraitAtlas = AdventureMapQuestChoiceDialog.SetPortraitAtlas
local dialogMapMatches = false
local dialogPinMatches = false
local dialogQuestID = nil
local callbackIsFunction = false
local dialogScale = nil
local portraitAtlas = nil

AdventureMapQuestChoiceDialog.ShowWithQuest = function(self, map, shownPin, shownQuestID, callback, scale)
    dialogMapMatches = map == mapCanvas
    dialogPinMatches = shownPin == pin
    dialogQuestID = shownQuestID
    callbackIsFunction = type(callback) == "function"
    dialogScale = scale
end
AdventureMapQuestChoiceDialog.SetPortraitAtlas = function(self, atlas)
    portraitAtlas = atlas
end

provider:OnQuestOfferPinClicked(pin)

AdventureMapQuestChoiceDialog.ShowWithQuest = originalShowWithQuest
AdventureMapQuestChoiceDialog.SetPortraitAtlas = originalSetPortraitAtlas

local trigger = mapCanvas.acquiredTrigger
return pin.panAndZoomToCount,
       pin.panToCount,
       provider.currentOfferPin == pin,
       dialogMapMatches,
       dialogPinMatches,
       dialogQuestID,
       callbackIsFunction,
       dialogScale,
       portraitAtlas,
       mapCanvas.acquiredNamespace,
       trigger ~= nil,
       trigger and trigger.owner == provider,
       mapCanvas.callbackTrigger == trigger,
       mapCanvas.callbackIsFunction == true,
       trigger and trigger.resetCount or 0,
       trigger and trigger.centerX or nil,
       trigger and trigger.centerY or nil,
       trigger and trigger.stretchX or nil,
       trigger and trigger.stretchY or nil,
       trigger and trigger.pin == pin
"#;

#[test]
fn quest_offer_pin_click_anchors_dialog_and_trigger() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let midnight_first_offer = click_offer_surface(env, "midnight", false);
        assert_click_offer_surface(
            midnight_first_offer,
            ExpectedPan::PanAndZoom,
            "ui-prey-scoutingmap",
        );

        let default_previous_offer = click_offer_surface(env, "dragonflight", true);
        assert_click_offer_surface(
            default_previous_offer,
            ExpectedPan::PanOnly,
            "FXAM-QuestBang",
        );
    });
}

#[derive(Clone, Copy)]
enum ExpectedPan {
    PanAndZoom,
    PanOnly,
}

type QuestOfferClickSurface = (
    i64,
    i64,
    bool,
    bool,
    bool,
    i64,
    bool,
    f64,
    String,
    String,
    bool,
    bool,
    bool,
    bool,
    i64,
    f64,
    f64,
    f64,
    f64,
    bool,
);
type DialogInvocation = (bool, bool, bool, i64, bool, f64, String);

fn click_offer_surface(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    texture_kit: &str,
    previous_offer_active: bool,
) -> QuestOfferClickSurface {
    seed_offer_probe(env, texture_kit, previous_offer_active);
    env.eval(QUEST_OFFER_CLICK_PROBE)
        .expect("AdventureMap quest-offer pin click probe must run cleanly")
}

fn seed_offer_probe(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    texture_kit: &str,
    previous_offer_active: bool,
) {
    env.state().borrow_mut().adventure_map.texture_kit = texture_kit.to_string();
    env.exec(&format!(
        r#"
        __questOfferClickQuestID = {QUEST_ID}
        __questOfferClickPreviousActive = {previous_offer_active}
        __questOfferClickNormalizedX = {NORMALIZED_X}
        __questOfferClickNormalizedY = {NORMALIZED_Y}
        "#
    ))
    .expect("AdventureMap quest-offer pin click probe setup must run cleanly");
}

fn assert_click_offer_surface(
    surface: QuestOfferClickSurface,
    expected_pan: ExpectedPan,
    expected_portrait_atlas: &str,
) {
    assert_pan_branch(surface.0, surface.1, expected_pan);
    assert_dialog_invocation(dialog_invocation(&surface), expected_portrait_atlas);
    assert_area_trigger_setup(
        surface.9, surface.10, surface.11, surface.12, surface.13, surface.14, surface.19,
    );
    assert_area_trigger_geometry(surface.15, surface.16, surface.17, surface.18);
}

fn dialog_invocation(surface: &QuestOfferClickSurface) -> DialogInvocation {
    (
        surface.2,
        surface.3,
        surface.4,
        surface.5,
        surface.6,
        surface.7,
        surface.8.clone(),
    )
}

fn assert_pan_branch(pan_and_zoom_count: i64, pan_to_count: i64, expected_pan: ExpectedPan) {
    match expected_pan {
        ExpectedPan::PanAndZoom => {
            assert_eq!(
                pan_and_zoom_count, 1,
                "first quest-offer click must call `PanAndZoomTo`"
            );
            assert_eq!(
                pan_to_count, 0,
                "first quest-offer click must not call `PanTo`"
            );
        }
        ExpectedPan::PanOnly => {
            assert_eq!(
                pan_to_count, 1,
                "clicking with a previous offer active must call `PanTo`"
            );
            assert_eq!(
                pan_and_zoom_count, 0,
                "clicking with a previous offer active must not call `PanAndZoomTo`"
            );
        }
    }
}

fn assert_dialog_invocation(dialog: DialogInvocation, expected_portrait_atlas: &str) {
    let (
        provider_current_pin,
        dialog_map_matches,
        dialog_pin_matches,
        dialog_quest_id,
        callback_is_function,
        dialog_scale,
        portrait_atlas,
    ) = dialog;

    assert!(
        provider_current_pin,
        "clicked pin must become the current offer pin"
    );
    assert!(dialog_map_matches, "dialog must receive the provider map");
    assert!(
        dialog_pin_matches,
        "dialog must anchor to the clicked offer pin"
    );
    assert_eq!(
        dialog_quest_id, QUEST_ID,
        "dialog must receive the offer quest id"
    );
    assert!(
        callback_is_function,
        "dialog must receive an OnClosed callback"
    );
    assert_approx_eq(dialog_scale, 0.0, "quest-offer dialog animation delay");
    assert_eq!(
        portrait_atlas, expected_portrait_atlas,
        "quest-offer portrait atlas must follow the adventure-map texture kit"
    );
}

fn assert_area_trigger_setup(
    acquired_namespace: String,
    acquired_trigger: bool,
    trigger_owner_is_provider: bool,
    callback_trigger_is_acquired: bool,
    area_callback_is_function: bool,
    reset_count: i64,
    trigger_pin_is_clicked: bool,
) {
    assert_eq!(
        acquired_namespace, "AdventureMap_QuestOffer",
        "quest-offer click must acquire the quest-offer trigger namespace"
    );
    assert!(
        acquired_trigger,
        "quest-offer click must acquire an area trigger"
    );
    assert!(
        trigger_owner_is_provider,
        "quest-offer area trigger owner must be the provider"
    );
    assert!(
        callback_trigger_is_acquired,
        "area-trigger callback must be registered on the acquired trigger"
    );
    assert!(
        area_callback_is_function,
        "area-trigger enclosed callback must be a function"
    );
    assert_eq!(
        reset_count, 1,
        "quest-offer click must reset the area trigger"
    );
    assert!(
        trigger_pin_is_clicked,
        "quest-offer area trigger must keep a reference to the clicked pin"
    );
}

fn assert_area_trigger_geometry(center_x: f64, center_y: f64, stretch_x: f64, stretch_y: f64) {
    assert_approx_eq(
        center_x,
        NORMALIZED_X,
        "quest-offer trigger center must use the pin global X",
    );
    assert_approx_eq(
        center_y,
        NORMALIZED_Y,
        "quest-offer trigger center must use the pin global Y",
    );
    assert_approx_eq(
        stretch_x,
        EXPECTED_STRETCH,
        "quest-offer trigger horizontal stretch",
    );
    assert_approx_eq(
        stretch_y,
        EXPECTED_STRETCH,
        "quest-offer trigger vertical stretch",
    );
}

fn assert_approx_eq(actual: f64, expected: f64, context: &str) {
    let delta = (actual - expected).abs();
    assert!(
        delta < 0.001,
        "{context}: expected {expected}, got {actual}"
    );
}
