//! Zone-summary provider groups quest offers by child zone and inset.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::{AdventureMapInset, AdventureMapQuestOffer};

const ROOT: &str = "Blizzard_AdventureMap";
const MAP_ID: i64 = 999;
const WEST_QUEST_ID: i64 = 41001;
const EAST_QUEST_ID: i64 = 41002;
const INSET_QUEST_ID: i64 = 41003;
const INSET_INDEX: i64 = 1;
const ZONE_SUMMARY_PROBE: &str = r#"
local originalGetMapChildrenInfo = C_Map.GetMapChildrenInfo
local originalGetMapInfoAtPosition = C_Map.GetMapInfoAtPosition
local originalGetMapRectOnMap = C_Map.GetMapRectOnMap

C_Map.GetMapChildrenInfo = function(mapID, mapType)
    return {
        { mapID = 101, name = "Westmarch" },
        { mapID = 102, name = "Eastwatch" },
        { mapID = 103, name = "Empty Fields" },
    }
end
C_Map.GetMapInfoAtPosition = function(mapID, normalizedX, normalizedY)
    if normalizedX < 0.5 then
        return { mapID = 101, name = "Westmarch" }
    end
    return { mapID = 102, name = "Eastwatch" }
end
C_Map.GetMapRectOnMap = function(childMapID, parentMapID)
    if childMapID == 101 then
        return 0.10, 0.30, 0.20, 0.60
    elseif childMapID == 102 then
        return 0.50, 0.90, 0.10, 0.50
    end
    return 0, 0, 0, 0
end

local acquiredPins = {}
local mapCanvas = {
    GetMapID = function()
        return 999
    end,
    RemoveAllPinsByTemplate = function() end,
    IsMapInsetExpanded = function()
        return false
    end,
    AcquirePin = function(self, template, playRevealAnims)
        local pin = { template = template, playRevealAnims = playRevealAnims }
        pin.Text = { SetText = function(_, text) pin.text = text end }
        pin.SetPosition = function(self, normalizedX, normalizedY)
            self.normalizedX = normalizedX
            self.normalizedY = normalizedY
        end
        pin.SetShown = function(self, shown)
            self.shown = shown
        end
        table.insert(acquiredPins, pin)
        return pin
    end,
}
local provider = CreateFromMixins(AdventureMap_ZoneSummaryProviderMixin)
provider.owningMap = mapCanvas
provider.GatherMissions = function(self)
    self.missionsByZone = {}
end

provider:RefreshAllData(false)

C_Map.GetMapChildrenInfo = originalGetMapChildrenInfo
C_Map.GetMapInfoAtPosition = originalGetMapInfoAtPosition
C_Map.GetMapRectOnMap = originalGetMapRectOnMap

local zonePinCount = 0
local insetPinCount = 0
local westPinQuestCount = nil
local eastPinQuestCount = nil
local emptyZonePinSeen = false
local insetQuestCount = nil
local insetPinShown = nil
local westPinX = nil
local westPinY = nil

for _, pin in ipairs(acquiredPins) do
    if pin.template == "AdventureMap_ZoneSummaryPinTemplate" then
        zonePinCount = zonePinCount + 1
        if pin.title == "Westmarch" then
            westPinQuestCount = #pin.quests
            westPinX = pin.normalizedX
            westPinY = pin.normalizedY
        elseif pin.title == "Eastwatch" then
            eastPinQuestCount = #pin.quests
        elseif pin.title == "Empty Fields" then
            emptyZonePinSeen = true
        end
    elseif pin.template == "AdventureMap_ZoneSummaryInsetPinTemplate" then
        insetPinCount = insetPinCount + 1
        insetQuestCount = #pin.quests
        insetPinShown = pin.shown
    end
end

return #provider.questsByZone[101],
       #provider.questsByZone[102],
       provider.questsByZone[103] == nil,
       #provider.questsByInset[1],
       zonePinCount,
       insetPinCount,
       westPinQuestCount,
       eastPinQuestCount,
       emptyZonePinSeen,
       insetQuestCount,
       insetPinShown,
       westPinX,
       westPinY,
       provider.playRevealAnims == false
"#;

#[test]
fn zone_summary_provider_groups_quests_by_zone_and_inset() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_zone_summary_inputs(env);

        let surface: ZoneSummarySurface = env
            .eval(ZONE_SUMMARY_PROBE)
            .expect("AdventureMap zone-summary provider probe must run cleanly");

        assert_zone_summary_surface(surface);
    });
}

type ZoneSummarySurface = (
    i64,
    i64,
    bool,
    i64,
    i64,
    i64,
    i64,
    i64,
    bool,
    i64,
    bool,
    f64,
    f64,
    bool,
);
type ZoneGrouping = (i64, i64, bool, i64);
type SummaryPins = (i64, i64, i64, i64, bool, i64, bool);
type ZonePinPosition = (f64, f64);

fn seed_zone_summary_inputs(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.adventure_map.map_id = MAP_ID;
    state.adventure_map.quest_offers = vec![
        quest_offer(WEST_QUEST_ID, 0.25, 0.40, None),
        quest_offer(EAST_QUEST_ID, 0.75, 0.30, None),
        quest_offer(INSET_QUEST_ID, 0.60, 0.70, Some(INSET_INDEX)),
    ];
    state.adventure_map.insets = Some(vec![AdventureMapInset {
        map_id: 627,
        title: "Inset Title".to_string(),
        description: "Inset description".to_string(),
        collapsed_icon: "AdventureMapIcon-Stormheim".to_string(),
        area_table_id: 7558,
        num_detail_tiles: 8,
        normalized_x: 0.42,
        normalized_y: 0.18,
        detail_tiles: Vec::new(),
    }]);
}

fn quest_offer(
    quest_id: i64,
    normalized_x: f64,
    normalized_y: f64,
    inset_index: Option<i64>,
) -> AdventureMapQuestOffer {
    AdventureMapQuestOffer {
        quest_id,
        title: format!("Quest {quest_id}"),
        description: format!("Description {quest_id}"),
        normalized_x,
        normalized_y,
        inset_index,
        ..Default::default()
    }
}

fn assert_zone_summary_surface(surface: ZoneSummarySurface) {
    assert_zone_grouping(zone_grouping(&surface));
    assert_summary_pins(summary_pins(&surface));
    assert_zone_pin_position(zone_pin_position(&surface));
    assert_reveal_animation_mode_cleared(&surface);
}

fn zone_grouping(surface: &ZoneSummarySurface) -> ZoneGrouping {
    (surface.0, surface.1, surface.2, surface.3)
}

fn summary_pins(surface: &ZoneSummarySurface) -> SummaryPins {
    (
        surface.4, surface.5, surface.6, surface.7, surface.8, surface.9, surface.10,
    )
}

fn zone_pin_position(surface: &ZoneSummarySurface) -> ZonePinPosition {
    (surface.11, surface.12)
}

fn assert_reveal_animation_mode_cleared(surface: &ZoneSummarySurface) {
    assert!(
        surface.13,
        "`RefreshAllData(false)` must clear reveal-animation mode"
    );
}

fn assert_zone_grouping(grouping: ZoneGrouping) {
    let (west_zone_count, east_zone_count, empty_zone_absent, inset_count) = grouping;

    assert_eq!(west_zone_count, 1, "west child zone must receive one quest");
    assert_eq!(east_zone_count, 1, "east child zone must receive one quest");
    assert!(
        empty_zone_absent,
        "empty child zone must not get a quest bucket"
    );
    assert_eq!(inset_count, 1, "inset bucket must receive one quest");
}

fn assert_summary_pins(pins: SummaryPins) {
    let (
        zone_pin_count,
        inset_pin_count,
        west_pin_quest_count,
        east_pin_quest_count,
        empty_zone_pin_seen,
        inset_pin_quest_count,
        inset_pin_shown,
    ) = pins;

    assert_eq!(
        zone_pin_count, 2,
        "`RefreshAllData` must add one zone summary pin per non-empty child zone"
    );
    assert_eq!(
        inset_pin_count, 1,
        "`RefreshAllData` must add one inset summary pin per inset quest bucket"
    );
    assert_eq!(west_pin_quest_count, 1);
    assert_eq!(east_pin_quest_count, 1);
    assert!(
        !empty_zone_pin_seen,
        "empty child zones must not receive summary pins"
    );
    assert_eq!(inset_pin_quest_count, 1);
    assert!(inset_pin_shown, "collapsed inset summary pin must be shown");
}

fn assert_zone_pin_position(position: ZonePinPosition) {
    let (west_pin_x, west_pin_y) = position;

    assert_approx_eq(west_pin_x, 0.20, "west zone summary pin X");
    assert_approx_eq(west_pin_y, 0.34, "west zone summary pin Y");
}

fn assert_approx_eq(actual: f64, expected: f64, context: &str) {
    let delta = (actual - expected).abs();
    assert!(
        delta < 0.001,
        "{context}: expected {expected}, got {actual}"
    );
}
