//! `AdventureMapMixin:RefreshInsets` area-table filtering behavior.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AdventureMapInset;

const ROOT: &str = "Blizzard_AdventureMap";

#[test]
fn adventure_map_refresh_insets_filters_by_area_table_id() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.state().borrow_mut().adventure_map.insets = Some(vec![
            test_inset(1, 1001),
            test_inset(2, 1002),
            test_inset(3, 1003),
        ]);

        let surface: RefreshInsetFilterSurface = env
            .eval(
                r#"
                local originalMapCanvasRefreshInsets = MapCanvasMixin.RefreshInsets
                local originalGetMapInsetInfo = C_AdventureMap.GetMapInsetInfo
                local originalAddInset = AdventureMapFrame.AddInset

                local getInfoCount = 0
                local addedIndexes = {}
                local addedMapIDs = {}

                MapCanvasMixin.RefreshInsets = function() end
                C_AdventureMap.GetMapInsetInfo = function(...)
                    getInfoCount = getInfoCount + 1
                    return originalGetMapInsetInfo(...)
                end
                AdventureMapFrame.AddInset = function(self, insetIndex, mapID)
                    table.insert(addedIndexes, insetIndex)
                    table.insert(addedMapIDs, mapID)
                end

                AdventureMapFrame:ClearAreaTableIDAvailableForInsets()
                AdventureMapFrame:SetAreaTableIDAvailableForInsets(1001)
                AdventureMapFrame:SetAreaTableIDAvailableForInsets(1003)
                AdventureMapMixin.RefreshInsets(AdventureMapFrame)

                MapCanvasMixin.RefreshInsets = originalMapCanvasRefreshInsets
                C_AdventureMap.GetMapInsetInfo = originalGetMapInsetInfo
                AdventureMapFrame.AddInset = originalAddInset

                return getInfoCount,
                       table.concat(addedIndexes, ","),
                       table.concat(addedMapIDs, ",")
                "#,
            )
            .expect("AdventureMap RefreshInsets area-filter probe must run cleanly");

        assert_refresh_inset_filter_surface(surface);
    });
}

type RefreshInsetFilterSurface = (i64, String, String);

fn test_inset(index: i64, area_table_id: i64) -> AdventureMapInset {
    AdventureMapInset {
        map_id: 7000 + index,
        title: format!("Inset {index}"),
        description: format!("Description {index}"),
        collapsed_icon: format!("AdventureMapIcon-{index}"),
        area_table_id,
        num_detail_tiles: 0,
        normalized_x: 0.1 * index as f64,
        normalized_y: 0.2 * index as f64,
        detail_tiles: Vec::new(),
    }
}

fn assert_refresh_inset_filter_surface(surface: RefreshInsetFilterSurface) {
    let (get_info_count, added_indexes, added_map_ids) = surface;

    assert_eq!(
        get_info_count, 3,
        "`RefreshInsets` must inspect every seeded inset before applying the area-table filter"
    );
    assert_eq!(
        added_indexes, "1,3",
        "`RefreshInsets` must add only insets whose areaTableID was marked available"
    );
    assert_eq!(
        added_map_ids, "7001,7003",
        "`RefreshInsets` must pass through the matching inset map ids"
    );
}
