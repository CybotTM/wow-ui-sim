//! `AdventureMapInsetMixin:Initialize` sizing, collapsed label, atlas, and anchoring.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const COLLAPSED_ICON_ATLAS: &str = "AdventureMapIcon-Stormheim";
const TITLE: &str = "storm peaks";
const NUM_DETAIL_TILES: i64 = 8;
const NORMALIZED_X: f64 = 0.25;
const NORMALIZED_Y: f64 = 0.5;
const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 600.0;
const EXPECTED_WIDTH: f64 = 751.5;
const EXPECTED_HEIGHT: f64 = 309.0;
const EXPECTED_ANCHOR_X: f64 = CANVAS_WIDTH * NORMALIZED_X;
const EXPECTED_ANCHOR_Y: f64 = -(CANVAS_HEIGHT * NORMALIZED_Y);

#[test]
fn adventure_map_inset_initialize_sizes_textures_and_positions() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface: InsetInitializeSurface = env
            .eval(&format!(
                r#"
                local canvas = AdventureMapFrame:GetCanvas()
                canvas:SetSize({CANVAS_WIDTH}, {CANVAS_HEIGHT})

                local inset = AdventureMapFrame:GetMapInsetPool():Acquire()
                inset.BuildDetailTiles = function(self, insetIndex, numDetailTiles)
                    self.observedInsetIndex = insetIndex
                    self.observedNumDetailTiles = numDetailTiles
                end
                AdventureMapFrame.GetScaleForMaxZoom = function()
                    return 2
                end
                AdventureMapFrame.OnMapInsetSizeChanged = function(self, mapID, insetIndex, expanded)
                    self.observedMapID = mapID
                    self.observedInsetIndex = insetIndex
                    self.observedInsetExpanded = expanded
                end

                inset:Initialize(
                    AdventureMapFrame,
                    true,
                    7,
                    619,
                    {TITLE:?},
                    "description",
                    {COLLAPSED_ICON_ATLAS:?},
                    {NUM_DETAIL_TILES},
                    {NORMALIZED_X},
                    {NORMALIZED_Y}
                )

                local point, relativeTo, relativePoint, xOfs, yOfs = inset:GetPoint(1)

                return inset:GetWidth(),
                       inset:GetHeight(),
                       inset.ExpandedFrame:GetWidth(),
                       inset.ExpandedFrame:GetHeight(),
                       inset.CollapsedFrame.Text:GetText(),
                       inset.CollapsedFrame.Icon:GetAtlas(),
                       point,
                       relativeTo == canvas,
                       relativePoint,
                       xOfs,
                       yOfs,
                       inset.observedNumDetailTiles,
                       AdventureMapFrame.observedInsetExpanded
                "#
            ))
            .expect("AdventureMap inset Initialize probe must run cleanly");

        assert_inset_initialize_surface(surface);
    });
}

type InsetInitializeSurface = (
    f64,
    f64,
    f64,
    f64,
    String,
    String,
    String,
    bool,
    String,
    f64,
    f64,
    i64,
    bool,
);

fn assert_inset_initialize_surface(surface: InsetInitializeSurface) {
    let (
        width,
        height,
        expanded_width,
        expanded_height,
        collapsed_text,
        collapsed_icon_atlas,
        anchor_point,
        anchor_relative_to_canvas,
        anchor_relative_point,
        anchor_x,
        anchor_y,
        observed_num_detail_tiles,
        observed_inset_expanded,
    ) = surface;

    assert_inset_sizes(width, height, expanded_width, expanded_height);
    assert_collapsed_surface(collapsed_text, collapsed_icon_atlas);
    assert_inset_anchor(
        anchor_point,
        anchor_relative_to_canvas,
        anchor_relative_point,
        anchor_x,
        anchor_y,
    );
    assert_eq!(
        observed_num_detail_tiles, NUM_DETAIL_TILES,
        "`Initialize` must pass the inset detail tile count to `BuildDetailTiles`"
    );
    assert!(
        !observed_inset_expanded,
        "`Initialize` must notify the map that a collapsed inset is not expanded"
    );
}

fn assert_inset_sizes(width: f64, height: f64, expanded_width: f64, expanded_height: f64) {
    assert_approx_eq(
        width,
        EXPECTED_WIDTH,
        "`Initialize` must compute inset width",
    );
    assert_approx_eq(
        height,
        EXPECTED_HEIGHT,
        "`Initialize` must compute inset height",
    );
    assert_approx_eq(
        expanded_width,
        EXPECTED_WIDTH,
        "`Initialize` must size `ExpandedFrame` to match inset width",
    );
    assert_approx_eq(
        expanded_height,
        EXPECTED_HEIGHT,
        "`Initialize` must size `ExpandedFrame` to match inset height",
    );
}

fn assert_collapsed_surface(collapsed_text: String, collapsed_icon_atlas: String) {
    assert_eq!(
        collapsed_text, "STORM PEAKS",
        "`Initialize` must upper-case the collapsed inset label"
    );
    assert_eq!(
        collapsed_icon_atlas, COLLAPSED_ICON_ATLAS,
        "`Initialize` must apply the collapsed icon atlas"
    );
}

fn assert_inset_anchor(
    anchor_point: String,
    anchor_relative_to_canvas: bool,
    anchor_relative_point: String,
    anchor_x: f64,
    anchor_y: f64,
) {
    assert_eq!(
        anchor_point, "CENTER",
        "`Initialize` must anchor from CENTER"
    );
    assert!(
        anchor_relative_to_canvas,
        "`Initialize` must anchor relative to the map canvas"
    );
    assert_eq!(
        anchor_relative_point, "TOPLEFT",
        "`Initialize` must anchor to the canvas TOPLEFT"
    );
    assert_approx_eq(
        anchor_x,
        EXPECTED_ANCHOR_X,
        "`Initialize` must scale normalized X by canvas width",
    );
    assert_approx_eq(
        anchor_y,
        EXPECTED_ANCHOR_Y,
        "`Initialize` must scale normalized Y by negative canvas height",
    );
}

fn assert_approx_eq(actual: f64, expected: f64, context: &str) {
    let delta = (actual - expected).abs();
    assert!(
        delta < 0.001,
        "{context}: expected {expected}, got {actual}"
    );
}
