//! `AdventureMapInsetMixin:Expand` pans, acquires a trigger, and reports expansion.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const COLLAPSED_ICON_ATLAS: &str = "AdventureMapIcon-Stormheim";
const TITLE: &str = "storm peaks";
const INSET_INDEX: i64 = 7;
const MAP_ID: i64 = 619;
const NUM_DETAIL_TILES: i64 = 8;
const NORMALIZED_X: f64 = 0.25;
const NORMALIZED_Y: f64 = 0.5;
const NORMALIZED_WIDTH: f64 = 0.8;
const NORMALIZED_HEIGHT: f64 = 0.6;
const EXPECTED_WIDTH: f64 = 751.5;
const EXPECTED_HEIGHT: f64 = 309.0;
const EXPECTED_STRETCH_X: f64 = NORMALIZED_WIDTH * 0.5;
const EXPECTED_STRETCH_Y: f64 = NORMALIZED_HEIGHT * 0.5;

#[test]
fn adventure_map_inset_expand_acquires_trigger_and_pans() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface: InsetExpandSurface = env
            .eval(&format!(
                r#"
                local inset = AdventureMapFrame:GetMapInsetPool():Acquire()
                inset.BuildDetailTiles = function() end
                AdventureMapFrame.GetScaleForMaxZoom = function()
                    return 2
                end
                AdventureMapFrame.OnMapInsetSizeChanged = function() end

                inset:Initialize(
                    AdventureMapFrame,
                    true,
                    {INSET_INDEX},
                    {MAP_ID},
                    {TITLE:?},
                    "description",
                    {COLLAPSED_ICON_ATLAS:?},
                    {NUM_DETAIL_TILES},
                    {NORMALIZED_X},
                    {NORMALIZED_Y}
                )

                local originalAcquireAreaTrigger = AdventureMapFrame.AcquireAreaTrigger
                local panX = nil
                local panY = nil
                local acquiredNamespace = nil
                local acquiredAreaTrigger = nil
                local resetCount = 0
                local centerX = nil
                local centerY = nil
                local stretchX = nil
                local stretchY = nil
                local normalizedHorizontalInput = nil
                local normalizedVerticalInput = nil
                local callbackTrigger = nil
                local expandedSizeChangeCount = 0
                local expandedMapID = nil
                local expandedInsetIndex = nil
                local expandedValue = false

                AdventureMapFrame.PanAndZoomTo = function(self, normalizedX, normalizedY)
                    panX = normalizedX
                    panY = normalizedY
                end
                AdventureMapFrame.ScrollContainer.MarkAreaTriggersDirty = function() end
                AdventureMapFrame.NormalizeHorizontalSize = function(self, size)
                    normalizedHorizontalInput = size
                    return {NORMALIZED_WIDTH}
                end
                AdventureMapFrame.NormalizeVerticalSize = function(self, size)
                    normalizedVerticalInput = size
                    return {NORMALIZED_HEIGHT}
                end
                AdventureMapFrame.AcquireAreaTrigger = function(self, namespace)
                    acquiredNamespace = namespace
                    acquiredAreaTrigger = originalAcquireAreaTrigger(self, namespace)
                    acquiredAreaTrigger.Reset = function()
                        resetCount = resetCount + 1
                    end
                    acquiredAreaTrigger.SetCenter = function(self, normalizedX, normalizedY)
                        centerX = normalizedX
                        centerY = normalizedY
                    end
                    acquiredAreaTrigger.Stretch = function(self, x, y)
                        stretchX = x
                        stretchY = y
                    end
                    return acquiredAreaTrigger
                end
                AdventureMapFrame.SetAreaTriggerEnclosedCallback = function(self, areaTrigger, callback)
                    callbackTrigger = areaTrigger
                end
                AdventureMapFrame.OnMapInsetSizeChanged = function(self, mapID, insetIndex, expanded)
                    if expanded == true then
                        expandedSizeChangeCount = expandedSizeChangeCount + 1
                        expandedMapID = mapID
                        expandedInsetIndex = insetIndex
                        expandedValue = expanded
                    end
                end

                inset:Expand()

                AdventureMapFrame.AcquireAreaTrigger = originalAcquireAreaTrigger

                return panX,
                       panY,
                       acquiredNamespace,
                       acquiredAreaTrigger ~= nil,
                       acquiredAreaTrigger and acquiredAreaTrigger.owner == inset,
                       callbackTrigger == acquiredAreaTrigger,
                       resetCount,
                       centerX,
                       centerY,
                       stretchX,
                       stretchY,
                       normalizedHorizontalInput,
                       normalizedVerticalInput,
                       expandedSizeChangeCount,
                       expandedMapID,
                       expandedInsetIndex,
                       expandedValue
                "#
            ))
            .expect("AdventureMap inset Expand trigger probe must run cleanly");

        assert_inset_expand_surface(surface);
    });
}

type InsetExpandSurface = (
    f64,
    f64,
    String,
    bool,
    bool,
    bool,
    i64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    i64,
    i64,
    i64,
    bool,
);
type PanTarget = (f64, f64);
type TriggerSetup = (String, bool, bool, bool, i64);
type TriggerGeometry = (f64, f64, f64, f64, f64, f64);
type SizeChange = (i64, i64, i64, bool);

fn assert_inset_expand_surface(surface: InsetExpandSurface) {
    assert_pan_target(pan_target(&surface));
    assert_area_trigger_setup(trigger_setup(&surface));
    assert_area_trigger_geometry(trigger_geometry(&surface));
    assert_expanded_size_change(size_change(&surface));
}

fn pan_target(surface: &InsetExpandSurface) -> PanTarget {
    (surface.0, surface.1)
}

fn trigger_setup(surface: &InsetExpandSurface) -> TriggerSetup {
    (
        surface.2.clone(),
        surface.3,
        surface.4,
        surface.5,
        surface.6,
    )
}

fn trigger_geometry(surface: &InsetExpandSurface) -> TriggerGeometry {
    (
        surface.7, surface.8, surface.9, surface.10, surface.11, surface.12,
    )
}

fn size_change(surface: &InsetExpandSurface) -> SizeChange {
    (surface.13, surface.14, surface.15, surface.16)
}

fn assert_pan_target(pan_target: PanTarget) {
    let (pan_x, pan_y) = pan_target;

    assert_approx_eq(
        pan_x,
        NORMALIZED_X,
        "`Expand` must pan to the inset normalized X",
    );
    assert_approx_eq(
        pan_y,
        NORMALIZED_Y,
        "`Expand` must pan to the inset normalized Y",
    );
}

fn assert_area_trigger_setup(trigger_setup: TriggerSetup) {
    let (
        acquired_namespace,
        acquired_area_trigger,
        trigger_owner_is_inset,
        callback_trigger_is_acquired,
        reset_count,
    ) = trigger_setup;

    assert_eq!(
        acquired_namespace, "AdventureMap_MapInset",
        "`Expand` must acquire the map-inset trigger namespace"
    );
    assert!(
        acquired_area_trigger,
        "`Expand` must acquire an area trigger when none exists"
    );
    assert!(
        trigger_owner_is_inset,
        "`Expand` must assign the inset as the area trigger owner"
    );
    assert!(
        callback_trigger_is_acquired,
        "`Expand` must register the enclosed callback on the acquired trigger"
    );
    assert_eq!(reset_count, 1, "`Expand` must reset the acquired trigger");
}

fn assert_area_trigger_geometry(trigger_geometry: TriggerGeometry) {
    let (
        center_x,
        center_y,
        stretch_x,
        stretch_y,
        normalized_horizontal_input,
        normalized_vertical_input,
    ) = trigger_geometry;

    assert_area_trigger_center(center_x, center_y);
    assert_area_trigger_stretch(
        stretch_x,
        stretch_y,
        normalized_horizontal_input,
        normalized_vertical_input,
    );
}

fn assert_area_trigger_center(center_x: f64, center_y: f64) {
    assert_approx_eq(
        center_x,
        NORMALIZED_X,
        "`Expand` must center the trigger at the inset normalized X",
    );
    assert_approx_eq(
        center_y,
        NORMALIZED_Y,
        "`Expand` must center the trigger at the inset normalized Y",
    );
}

fn assert_area_trigger_stretch(
    stretch_x: f64,
    stretch_y: f64,
    normalized_horizontal_input: f64,
    normalized_vertical_input: f64,
) {
    assert_approx_eq(
        normalized_horizontal_input,
        EXPECTED_WIDTH,
        "`Expand` must normalize the expanded frame width",
    );
    assert_approx_eq(
        normalized_vertical_input,
        EXPECTED_HEIGHT,
        "`Expand` must normalize the expanded frame height",
    );
    assert_approx_eq(
        stretch_x,
        EXPECTED_STRETCH_X,
        "`Expand` must stretch half the normalized width",
    );
    assert_approx_eq(
        stretch_y,
        EXPECTED_STRETCH_Y,
        "`Expand` must stretch half the normalized height",
    );
}

fn assert_expanded_size_change(size_change: SizeChange) {
    let (expanded_size_change_count, expanded_map_id, expanded_inset_index, expanded_value) =
        size_change;

    assert_eq!(
        expanded_size_change_count, 1,
        "`Expand` must notify exactly one expanded size change"
    );
    assert_eq!(
        expanded_map_id, MAP_ID,
        "`Expand` must notify size change for the inset map id"
    );
    assert_eq!(
        expanded_inset_index, INSET_INDEX,
        "`Expand` must notify size change for the inset index"
    );
    assert!(expanded_value, "`Expand` must notify `expanded=true`");
}

fn assert_approx_eq(actual: f64, expected: f64, context: &str) {
    let delta = (actual - expected).abs();
    assert!(
        delta < 0.001,
        "{context}: expected {expected}, got {actual}"
    );
}
