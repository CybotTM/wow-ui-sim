//! `AdventureMapInsetMixin:OnCanvasScaleChanged` collapses only while zooming out.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const COLLAPSED_ICON_ATLAS: &str = "AdventureMapIcon-Stormheim";
const TITLE: &str = "storm peaks";
const INSET_INDEX: i64 = 7;
const MAP_ID: i64 = 619;
const NUM_DETAIL_TILES: i64 = 8;
const NORMALIZED_X: f64 = 0.25;
const NORMALIZED_Y: f64 = 0.5;
const EXPANDED_ZOOMING_OUT_SCALE: f64 = 2.5;
const EXPANDED_STABLE_SCALE: f64 = 2.0;
const COLLAPSED_ZOOMING_OUT_SCALE: f64 = 4.0;
const INSET_SCALE_PROBE: &str = r#"
local canvas = AdventureMapFrame:GetCanvas()
canvas:SetSize(800, 600)

AdventureMapFrame.GetScaleForMaxZoom = function()
    return 2
end
AdventureMapFrame.IsZoomingOut = function(self)
    return self.zoomingOut == true
end
AdventureMapFrame.GetCanvasScale = function(self)
    return self.canvasScale
end
AdventureMapFrame.OnMapInsetSizeChanged = function(self, mapID, insetIndex, expanded)
    self.sizeChangeCount = (self.sizeChangeCount or 0) + 1
    self.lastExpanded = expanded
end

local function exercise(collapsed, zoomingOut, canvasScale)
    local inset = AdventureMapFrame:GetMapInsetPool():Acquire()
    inset.BuildDetailTiles = function() end

    inset:Initialize(
        AdventureMapFrame,
        collapsed,
        __insetCanvasScaleInsetIndex,
        __insetCanvasScaleMapID,
        __insetCanvasScaleTitle,
        "description",
        __insetCanvasScaleCollapsedIcon,
        __insetCanvasScaleNumDetailTiles,
        __insetCanvasScaleNormalizedX,
        __insetCanvasScaleNormalizedY
    )

    AdventureMapFrame.sizeChangeCount = 0
    AdventureMapFrame.lastExpanded = nil
    AdventureMapFrame.zoomingOut = zoomingOut
    AdventureMapFrame.canvasScale = canvasScale

    inset:OnCanvasScaleChanged()

    return inset.collapsed,
           inset.CollapsedFrame:GetScale(),
           AdventureMapFrame.sizeChangeCount,
           AdventureMapFrame.lastExpanded == false
end

local expandedZoomingOutCollapsed,
      expandedZoomingOutScale,
      expandedZoomingOutSizeChangeCount,
      expandedZoomingOutReportedCollapsed = exercise(false, true, __expandedZoomingOutScale)
local expandedStableCollapsed,
      expandedStableScale,
      expandedStableSizeChangeCount = exercise(false, false, __expandedStableScale)
local collapsedZoomingOutCollapsed,
      collapsedZoomingOutScale,
      collapsedZoomingOutSizeChangeCount = exercise(true, true, __collapsedZoomingOutScale)

return expandedZoomingOutCollapsed,
       expandedZoomingOutScale,
       expandedZoomingOutSizeChangeCount,
       expandedZoomingOutReportedCollapsed,
       expandedStableCollapsed,
       expandedStableScale,
       expandedStableSizeChangeCount,
       collapsedZoomingOutCollapsed,
       collapsedZoomingOutScale,
       collapsedZoomingOutSizeChangeCount
"#;

#[test]
fn adventure_map_inset_canvas_scale_change_collapses_only_when_zooming_out() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface = inset_scale_surface(env);
        assert_inset_scale_surface(surface);
    });
}

type InsetScaleSurface = (bool, f64, i64, bool, bool, f64, i64, bool, f64, i64);

fn inset_scale_surface(env: &wow_ui_sim::lua_api::WowLuaEnv) -> InsetScaleSurface {
    seed_inset_scale_probe(env);
    env.eval(INSET_SCALE_PROBE)
        .expect("AdventureMap inset canvas-scale probe must run cleanly")
}

fn seed_inset_scale_probe(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(&format!(
        "__insetCanvasScaleInsetIndex = {INSET_INDEX}; \
         __insetCanvasScaleMapID = {MAP_ID}; \
         __insetCanvasScaleTitle = {TITLE:?}; \
         __insetCanvasScaleCollapsedIcon = {COLLAPSED_ICON_ATLAS:?}; \
         __insetCanvasScaleNumDetailTiles = {NUM_DETAIL_TILES}; \
         __insetCanvasScaleNormalizedX = {NORMALIZED_X}; \
         __insetCanvasScaleNormalizedY = {NORMALIZED_Y}; \
         __expandedZoomingOutScale = {EXPANDED_ZOOMING_OUT_SCALE}; \
         __expandedStableScale = {EXPANDED_STABLE_SCALE}; \
         __collapsedZoomingOutScale = {COLLAPSED_ZOOMING_OUT_SCALE}"
    ))
    .expect("AdventureMap inset canvas-scale setup must run cleanly");
}

fn assert_inset_scale_surface(surface: InsetScaleSurface) {
    assert_zooming_out_collapse(zooming_out_collapse(&surface));
    assert_stable_expanded_rescale(stable_expanded_rescale(&surface));
    assert_collapsed_rescale(collapsed_rescale(&surface));
}

type ZoomingOutCollapse = (bool, f64, i64, bool);
type StableExpandedRescale = (bool, f64, i64);
type CollapsedRescale = (bool, f64, i64);

fn zooming_out_collapse(surface: &InsetScaleSurface) -> ZoomingOutCollapse {
    (surface.0, surface.1, surface.2, surface.3)
}

fn stable_expanded_rescale(surface: &InsetScaleSurface) -> StableExpandedRescale {
    (surface.4, surface.5, surface.6)
}

fn collapsed_rescale(surface: &InsetScaleSurface) -> CollapsedRescale {
    (surface.7, surface.8, surface.9)
}

fn assert_zooming_out_collapse(result: ZoomingOutCollapse) {
    let (collapsed, scale, size_change_count, reported_collapsed) = result;

    assert!(collapsed, "expanded inset must collapse while zooming out");
    assert_approx_eq(
        scale,
        1.0 / EXPANDED_ZOOMING_OUT_SCALE,
        "zooming-out collapse must still rescale the collapsed frame",
    );
    assert_eq!(
        size_change_count, 1,
        "collapsing from expanded must notify the map once"
    );
    assert!(
        reported_collapsed,
        "collapse notification must report the inset as not expanded"
    );
}

fn assert_stable_expanded_rescale(result: StableExpandedRescale) {
    let (collapsed, scale, size_change_count) = result;

    assert!(
        !collapsed,
        "expanded inset must remain expanded when not zooming out"
    );
    assert_approx_eq(
        scale,
        1.0 / EXPANDED_STABLE_SCALE,
        "stable expanded inset must rescale the collapsed frame",
    );
    assert_eq!(
        size_change_count, 0,
        "rescaling without collapse must not report an inset size change"
    );
}

fn assert_collapsed_rescale(result: CollapsedRescale) {
    let (collapsed, scale, size_change_count) = result;

    assert!(collapsed, "already collapsed inset must remain collapsed");
    assert_approx_eq(
        scale,
        1.0 / COLLAPSED_ZOOMING_OUT_SCALE,
        "already collapsed inset must rescale the collapsed frame",
    );
    assert_eq!(
        size_change_count, 0,
        "already collapsed inset must not collapse again"
    );
}

fn assert_approx_eq(actual: f64, expected: f64, context: &str) {
    let delta = (actual - expected).abs();
    assert!(
        delta < 0.001,
        "{context}: expected {expected}, got {actual}"
    );
}
