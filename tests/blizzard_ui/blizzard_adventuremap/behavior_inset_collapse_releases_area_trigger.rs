//! `AdventureMapInsetMixin:Collapse` releases the active map-inset area trigger.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const COLLAPSED_ICON_ATLAS: &str = "AdventureMapIcon-Stormheim";
const TITLE: &str = "storm peaks";
const INSET_INDEX: i64 = 7;
const MAP_ID: i64 = 619;
const NUM_DETAIL_TILES: i64 = 0;
const NORMALIZED_X: f64 = 0.25;
const NORMALIZED_Y: f64 = 0.5;

#[test]
fn adventure_map_inset_collapse_releases_area_trigger() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface: InsetCollapseSurface = env
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

                AdventureMapFrame.PanAndZoomTo = function() end
                AdventureMapFrame.NormalizeHorizontalSize = function(self, size) return size end
                AdventureMapFrame.NormalizeVerticalSize = function(self, size) return size end
                AdventureMapFrame.ScrollContainer.MarkAreaTriggersDirty = function() end

                local originalAcquireAreaTrigger = AdventureMapFrame.AcquireAreaTrigger
                local originalReleaseAreaTrigger = AdventureMapFrame.ReleaseAreaTrigger
                local acquiredAreaTrigger = nil
                local releaseCount = 0
                local releasedNamespace = nil
                local releasedPreviouslyAcquiredTrigger = false
                local collapsedSizeChangeCount = 0
                local collapsedMapID = nil
                local collapsedInsetIndex = nil
                local collapsedExpandedValue = true

                AdventureMapFrame.AcquireAreaTrigger = function(self, namespace)
                    acquiredAreaTrigger = originalAcquireAreaTrigger(self, namespace)
                    return acquiredAreaTrigger
                end
                AdventureMapFrame.ReleaseAreaTrigger = function(self, namespace, areaTrigger)
                    releaseCount = releaseCount + 1
                    releasedNamespace = namespace
                    releasedPreviouslyAcquiredTrigger = areaTrigger == acquiredAreaTrigger
                    return originalReleaseAreaTrigger(self, namespace, areaTrigger)
                end
                AdventureMapFrame.OnMapInsetSizeChanged = function(self, mapID, insetIndex, expanded)
                    if expanded == false then
                        collapsedSizeChangeCount = collapsedSizeChangeCount + 1
                        collapsedMapID = mapID
                        collapsedInsetIndex = insetIndex
                        collapsedExpandedValue = expanded
                    end
                end

                inset:Expand()
                inset:Collapse()

                AdventureMapFrame.AcquireAreaTrigger = originalAcquireAreaTrigger
                AdventureMapFrame.ReleaseAreaTrigger = originalReleaseAreaTrigger

                return acquiredAreaTrigger ~= nil,
                       inset.areaTrigger == nil,
                       releaseCount,
                       releasedNamespace,
                       releasedPreviouslyAcquiredTrigger,
                       collapsedSizeChangeCount,
                       collapsedMapID,
                       collapsedInsetIndex,
                       collapsedExpandedValue
                "#,
            ))
            .expect("AdventureMap inset Collapse release probe must run cleanly");

        assert_inset_collapse_surface(surface);
    });
}

type InsetCollapseSurface = (bool, bool, i64, String, bool, i64, i64, i64, bool);

fn assert_inset_collapse_surface(surface: InsetCollapseSurface) {
    let (
        acquired_area_trigger,
        cleared_area_trigger,
        release_count,
        released_namespace,
        released_previously_acquired_trigger,
        collapsed_size_change_count,
        collapsed_map_id,
        collapsed_inset_index,
        collapsed_expanded_value,
    ) = surface;

    assert_area_trigger_release(
        acquired_area_trigger,
        cleared_area_trigger,
        release_count,
        released_namespace,
        released_previously_acquired_trigger,
    );
    assert_collapsed_size_change(
        collapsed_size_change_count,
        collapsed_map_id,
        collapsed_inset_index,
        collapsed_expanded_value,
    );
}

fn assert_area_trigger_release(
    acquired_area_trigger: bool,
    cleared_area_trigger: bool,
    release_count: i64,
    released_namespace: String,
    released_previously_acquired_trigger: bool,
) {
    assert!(
        acquired_area_trigger,
        "`Expand` must acquire an `AdventureMap_MapInset` area trigger before collapse"
    );
    assert!(
        cleared_area_trigger,
        "`Collapse` must clear `self.areaTrigger` after releasing it"
    );
    assert_eq!(
        release_count, 1,
        "`Collapse` must release exactly one area trigger"
    );
    assert_eq!(
        released_namespace, "AdventureMap_MapInset",
        "`Collapse` must release the map-inset trigger namespace"
    );
    assert!(
        released_previously_acquired_trigger,
        "`Collapse` must release the trigger that `Expand` acquired"
    );
}

fn assert_collapsed_size_change(
    collapsed_size_change_count: i64,
    collapsed_map_id: i64,
    collapsed_inset_index: i64,
    collapsed_expanded_value: bool,
) {
    assert_eq!(
        collapsed_size_change_count, 1,
        "`Collapse` must notify exactly one collapsed size change"
    );
    assert_eq!(
        collapsed_map_id, MAP_ID,
        "`Collapse` must notify size change for the inset map id"
    );
    assert_eq!(
        collapsed_inset_index, INSET_INDEX,
        "`Collapse` must notify size change for the inset index"
    );
    assert!(
        !collapsed_expanded_value,
        "`Collapse` must notify `expanded=false`"
    );
}
