//! `AdventureMapMixin:OnShow` map-id and inset area reset behavior.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const SEEDED_ADVENTURE_MAP_ID: i64 = 619;
const STALE_AREA_TABLE_ID: i32 = 424_242;

#[test]
fn adventure_map_onshow_clears_area_ids_and_sets_map_id() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.state().borrow_mut().adventure_map.map_id = SEEDED_ADVENTURE_MAP_ID;

        let surface: OnShowSurface = env
            .eval(&format!(
                r#"
                local originalMapCanvasOnShow = MapCanvasMixin.OnShow
                MapCanvasMixin.OnShow = function() end

                AdventureMapFrame:Hide()
                AdventureMapFrame.areaTableIDsToDisplay = {{ [{STALE_AREA_TABLE_ID}] = true }}
                AdventureMapMixin.OnShow(AdventureMapFrame)

                MapCanvasMixin.OnShow = originalMapCanvasOnShow

                return C_AdventureMap.GetMapID(),
                       AdventureMapFrame:GetMapID(),
                       AdventureMapFrame.ScrollContainer:GetMapID(),
                       type(AdventureMapFrame.areaTableIDsToDisplay),
                       AdventureMapFrame.areaTableIDsToDisplay[{STALE_AREA_TABLE_ID}] == nil
                "#
            ))
            .expect("AdventureMap OnShow map-id probe must run cleanly");

        assert_onshow_surface(surface);
    });
}

type OnShowSurface = (i64, i64, i64, String, bool);

fn assert_onshow_surface(surface: OnShowSurface) {
    let (
        c_adventure_map_id,
        frame_map_id,
        scroll_container_map_id,
        area_ids_type,
        stale_area_id_cleared,
    ) = surface;
    let expected_map_id = SEEDED_ADVENTURE_MAP_ID;

    assert_eq!(
        c_adventure_map_id, expected_map_id,
        "`C_AdventureMap.GetMapID()` must return the simulator-seeded adventure map id"
    );
    assert_eq!(
        frame_map_id, expected_map_id,
        "`AdventureMapMixin:OnShow` must feed `C_AdventureMap.GetMapID()` into `SetMapID`"
    );
    assert_eq!(
        scroll_container_map_id, expected_map_id,
        "`MapCanvasMixin.SetMapID` must forward the adventure map id to the scroll container"
    );
    assert_eq!(
        area_ids_type, "table",
        "`AdventureMapMixin:OnShow` must leave `areaTableIDsToDisplay` as a fresh table"
    );
    assert!(
        stale_area_id_cleared,
        "`AdventureMapMixin:OnShow` must clear stale area ids before refreshing the map"
    );
}
