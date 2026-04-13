//! Tests for c_map_api.rs: C_Map, zone text, UiMapPoint, C_DateAndTime, C_Minimap, etc.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// C_Map
// ============================================================================

#[test]
fn test_get_area_info() {
    let env = env();
    let name: String = env.eval("return C_Map.GetAreaInfo(1)").unwrap();
    assert_eq!(name, "Dun Morogh");
}

#[test]
fn test_get_area_info_nil_for_unknown() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Map.GetAreaInfo(999999999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_get_map_info() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Map.GetMapInfo(1)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_get_map_info_has_name_field() {
    let env = env();
    let name: String = env.eval("return C_Map.GetMapInfo(1).name").unwrap();
    assert!(!name.is_empty());
}

#[test]
fn test_get_best_map_for_unit() {
    let env = env();
    let map_id: i32 = env
        .eval(r#"return C_Map.GetBestMapForUnit("player")"#)
        .unwrap();
    assert_eq!(map_id, 2248, "Default map should be Isle of Dorn");
}

#[test]
fn test_get_player_map_position() {
    let env = env();
    let is_table: bool = env
        .eval(
            r#"
        local pos = C_Map.GetPlayerMapPosition(1, "player")
        return type(pos) == "table" or type(pos) == "userdata"
    "#,
        )
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_get_map_children_info() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Map.GetMapChildrenInfo(1)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_get_map_world_size() {
    let env = env();
    let (w, h): (f64, f64) = env.eval("return C_Map.GetMapWorldSize(1)").unwrap();
    assert!(w > 0.0);
    assert!(h > 0.0);
}

// ============================================================================
// Map art layer textures (world map tile rendering)
// ============================================================================

#[test]
fn test_get_map_art_layer_textures_returns_filedata_ids() {
    let env = env();
    // GetMapArtLayerTextures must return a table of fileDataIDs for map tiles.
    // The detail layer code indexes into this table (1..numTiles) and passes
    // each value to SetTexture(). Without valid fileDataIDs, map tiles are blank.
    let count: i32 = env
        .eval(
            r#"
        local mapID = C_Map.GetCurrentMapID()
        local textures = C_Map.GetMapArtLayerTextures(mapID, 1)
        if type(textures) ~= "table" then return 0 end
        local n = 0
        for _ in pairs(textures) do n = n + 1 end
        return n
    "#,
        )
        .unwrap();
    assert!(
        count > 0,
        "GetMapArtLayerTextures should return at least one fileDataID for default map"
    );
}

#[test]
fn test_get_map_art_layer_textures_count_matches_grid() {
    let env = env();
    // The number of textures must equal ceil(layerHeight/tileHeight) * ceil(layerWidth/tileWidth)
    let matches: bool = env
        .eval(
            r#"
        local mapID = C_Map.GetCurrentMapID()
        local layers = C_Map.GetMapArtLayers(mapID)
        local layerInfo = layers[1]
        local numRows = math.ceil(layerInfo.layerHeight / layerInfo.tileHeight)
        local numCols = math.ceil(layerInfo.layerWidth / layerInfo.tileWidth)
        local expected = numRows * numCols

        local textures = C_Map.GetMapArtLayerTextures(mapID, 1)
        local actual = 0
        for _ in pairs(textures) do actual = actual + 1 end
        return actual == expected
    "#,
        )
        .unwrap();
    assert!(
        matches,
        "Texture count must match tile grid dimensions from GetMapArtLayers"
    );
}

#[test]
fn test_get_map_art_layer_textures_values_are_numbers() {
    let env = env();
    // Each value should be a numeric fileDataID
    let all_numbers: bool = env
        .eval(
            r#"
        local mapID = C_Map.GetCurrentMapID()
        local textures = C_Map.GetMapArtLayerTextures(mapID, 1)
        for k, v in pairs(textures) do
            if type(v) ~= "number" or v <= 0 then return false end
        end
        return true
    "#,
        )
        .unwrap();
    assert!(
        all_numbers,
        "All texture entries should be positive numeric fileDataIDs"
    );
}

#[test]
fn test_get_map_art_id_returns_nonzero() {
    let env = env();
    let art_id: i32 = env
        .eval("return C_Map.GetMapArtID(C_Map.GetCurrentMapID())")
        .unwrap();
    assert!(
        art_id > 0,
        "GetMapArtID should return a non-zero art ID for the current map"
    );
}

#[test]
fn test_get_map_art_background_atlas_returns_world_map_tile_background() {
    let env = env();
    let atlas: String = env
        .eval("return C_Map.GetMapArtBackgroundAtlas(C_Map.GetCurrentMapID())")
        .unwrap();
    assert_eq!(
        atlas, "AdventureMap_TileBg",
        "map art background should use the shared tiled world map atlas"
    );
}

// ============================================================================
// Zone text functions
// ============================================================================

#[test]
fn test_get_real_zone_text() {
    let env = env();
    let zone: String = env.eval("return GetRealZoneText()").unwrap();
    assert!(!zone.is_empty());
}

#[test]
fn test_get_zone_text() {
    let env = env();
    let zone: String = env.eval("return GetZoneText()").unwrap();
    assert!(!zone.is_empty());
}

#[test]
fn test_get_sub_zone_text() {
    let env = env();
    let zone: String = env.eval("return GetSubZoneText()").unwrap();
    assert!(!zone.is_empty());
}

#[test]
fn test_get_minimap_zone_text() {
    let env = env();
    let zone: String = env.eval("return GetMinimapZoneText()").unwrap();
    assert!(!zone.is_empty());
}

// ============================================================================
// UiMapPoint
// ============================================================================

#[test]
fn test_ui_map_point_create_from_vector() {
    let env = env();
    env.exec(
        r#"
        local pos = {x = 0.5, y = 0.5, GetXY = function(self) return self.x, self.y end}
        local point = UiMapPoint.CreateFromVector2D(1, pos)
        assert(point ~= nil, "Should create a map point")
        assert(point.uiMapID == 1, "Map ID should be 1")
    "#,
    )
    .unwrap();
}

#[test]
fn test_ui_map_point_create_from_coordinates() {
    let env = env();
    env.exec(
        r#"
        local point = UiMapPoint.CreateFromCoordinates(2, 0.3, 0.7)
        assert(point.uiMapID == 2)
    "#,
    )
    .unwrap();
}

// ============================================================================
// C_DateAndTime
// ============================================================================

#[test]
fn test_get_current_calendar_time() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_DateAndTime.GetCurrentCalendarTime()) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_get_server_time_local() {
    let env = env();
    let time: i32 = env
        .eval("return C_DateAndTime.GetServerTimeLocal()")
        .unwrap();
    assert_eq!(time, 0);
}

#[test]
fn test_get_seconds_until_daily_reset() {
    let env = env();
    let secs: i32 = env
        .eval("return C_DateAndTime.GetSecondsUntilDailyReset()")
        .unwrap();
    assert_eq!(secs, 86400);
}

#[test]
fn test_get_seconds_until_weekly_reset() {
    let env = env();
    let secs: i32 = env
        .eval("return C_DateAndTime.GetSecondsUntilWeeklyReset()")
        .unwrap();
    assert_eq!(secs, 604800);
}

// ============================================================================
// C_Minimap
// ============================================================================

#[test]
fn test_minimap_is_inside_quest_blob() {
    let env = env();
    let val: bool = env
        .eval("return C_Minimap.IsInsideQuestBlob(1, 0.5, 0.5)")
        .unwrap();
    assert!(!val);
}

#[test]
fn test_minimap_get_view_radius() {
    let env = env();
    let radius: f64 = env.eval("return C_Minimap.GetViewRadius()").unwrap();
    assert!(radius > 0.0);
}

#[test]
fn test_minimap_set_player_texture_no_error() {
    let env = env();
    env.exec("C_Minimap.SetPlayerTexture(0, 0)").unwrap();
}

// ============================================================================
// C_Navigation
// ============================================================================

#[test]
fn test_navigation_get_frame_nil() {
    let env = env();
    let is_nil: bool = env.eval("return C_Navigation.GetFrame() == nil").unwrap();
    assert!(is_nil);
}

#[test]
fn test_navigation_get_distance() {
    let env = env();
    let dist: f64 = env.eval("return C_Navigation.GetDistance()").unwrap();
    assert_eq!(dist, 0.0);
}

#[test]
fn test_navigation_is_auto_follow_enabled() {
    let env = env();
    let val: bool = env
        .eval("return C_Navigation.IsAutoFollowEnabled()")
        .unwrap();
    assert!(!val);
}

#[test]
fn test_navigation_set_auto_follow_enabled_no_error() {
    let env = env();
    env.exec("C_Navigation.SetAutoFollowEnabled(true)").unwrap();
}

// ============================================================================
// C_MapExplorationInfo
// ============================================================================

#[test]
fn test_get_explored_area_ids() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_MapExplorationInfo.GetExploredAreaIDsAtPosition(1, {})) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_get_explored_area_ids_only_cover_left_half() {
    let env = env();
    let (left_count, right_count): (i32, i32) = env
        .eval(
            r#"
        local mapID = C_Map.GetCurrentMapID()
        local layer = C_Map.GetMapArtLayers(mapID)[1]
        local overlays = C_MapExplorationInfo.GetExploredMapTextures(mapID)
        local overlay = overlays and overlays[1]
        assert(overlay, "expected at least one explored overlay on the default map")

        local sampleX = (overlay.offsetX + (overlay.textureWidth / 2)) / layer.layerWidth
        local sampleY = (overlay.offsetY + (overlay.textureHeight / 2)) / layer.layerHeight

        local function count(list)
            if type(list) ~= "table" then
                return -1
            end

            local n = 0
            for _ in ipairs(list) do
                n = n + 1
            end
            return n
        end

        local left = C_MapExplorationInfo.GetExploredAreaIDsAtPosition(mapID, { x = sampleX, y = sampleY })
        local right = C_MapExplorationInfo.GetExploredAreaIDsAtPosition(mapID, { x = 0.75, y = 0.50 })
        return count(left), count(right)
    "#,
        )
        .unwrap();

    assert!(
        left_count > 0,
        "left half should report explored area IDs for the default map"
    );
    assert_eq!(
        right_count, 0,
        "right half should remain unexplored for the default map"
    );
}

#[test]
fn test_get_explored_map_textures() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_MapExplorationInfo.GetExploredMapTextures(1)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_get_explored_map_textures_return_real_left_half_overlays_for_current_map() {
    let env = env();
    let matches_expected_overlay: bool = env
        .eval(
            r#"
        local mapID = C_Map.GetCurrentMapID()
        local layer = C_Map.GetMapArtLayers(mapID)[1]
        local explored = C_MapExplorationInfo.GetExploredMapTextures(mapID)
        if type(explored) ~= "table" or #explored == 0 then
            return false
        end

        local minCenter = 1
        local maxCenter = 0
        local overlayCount = 0
        local hasMultiTileOverlay = false
        local hasAreaTexture = false

        for _, overlay in ipairs(explored) do
            overlayCount = overlayCount + 1
            local fileCount = 0
            for _, fileDataID in ipairs(overlay.fileDataIDs or {}) do
                fileCount = fileCount + 1
                if fileDataID > 0 then
                    hasAreaTexture = true
                end
            end

            if fileCount > 1 then
                hasMultiTileOverlay = true
            end

            local center = (overlay.offsetX + (overlay.textureWidth / 2)) / layer.layerWidth
            minCenter = math.min(minCenter, center)
            maxCenter = math.max(maxCenter, center)
        end

        return overlayCount > 1
            and hasMultiTileOverlay
            and hasAreaTexture
            and minCenter < 0.5
            and maxCenter <= 0.5
    "#,
        )
        .unwrap();
    assert!(
        matches_expected_overlay,
        "GetExploredMapTextures should expose real explored-area overlays filtered to the explored half of the current map"
    );
}

#[test]
fn test_c_fog_of_war_returns_half_map_data_for_current_map() {
    let env = env();
    let (fog_id, background, mask, scalar): (i32, Option<String>, Option<String>, f64) = env
        .eval(
            r#"
            local mapID = C_Map.GetCurrentMapID()
            local fogID = C_FogOfWar.GetFogOfWarForMap(mapID)
            local info = C_FogOfWar.GetFogOfWarInfo(fogID)
            return fogID or 0, info.backgroundAtlas, info.maskAtlas, info.maskScalar
        "#,
        )
        .unwrap();
    assert!(fog_id > 0, "current map should expose simulator fog data");
    assert_eq!(background.as_deref(), Some("Interface/Map/MapFogOfWar"));
    assert_eq!(
        mask.as_deref(),
        Some("Interface/Map/MapFogOfWarMaskSoftEdge")
    );
    assert!((scalar - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_c_fog_of_war_returns_nil_for_unknown_map() {
    let env = env();
    let has_no_fog: bool = env
        .eval("return C_FogOfWar.GetFogOfWarForMap(-1) == nil")
        .unwrap();
    assert!(has_no_fog, "unknown maps should not invent fog IDs");
}

// ============================================================================
// C_TaxiMap
// ============================================================================

#[test]
fn test_get_all_taxi_nodes() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_TaxiMap.GetAllTaxiNodes(1)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_get_taxi_nodes_for_map() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_TaxiMap.GetTaxiNodesForMap(1)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_should_map_show_taxi_nodes() {
    let env = env();
    let val: bool = env
        .eval("return C_TaxiMap.ShouldMapShowTaxiNodes(1)")
        .unwrap();
    assert!(val);
}

#[test]
fn test_is_visible_ignores_alpha() {
    // Reproduces the world map blank canvas bug:
    // The detail layer sets alpha=0 while waiting for textures to load,
    // then relies on OnUpdate to detect loading is complete and set alpha=1.
    // But is_ancestor_visible (used by fire_on_update to filter frames)
    // checks effective_alpha > 0, creating a deadlock: OnUpdate can't fire
    // because alpha=0, and alpha can't become 1 because OnUpdate can't fire.
    //
    // WoW's IsVisible() only checks the shown/hidden state, not alpha.
    // A frame with alpha=0 is still "visible" and should receive OnUpdate.
    let env = env();
    let visible: bool = env
        .eval(
            r#"
        UIParent:Show()
        local f = CreateFrame("Frame", nil, UIParent)
        f:Show()
        f:SetAlpha(0)
        return f:IsVisible()
    "#,
        )
        .unwrap();
    assert!(
        visible,
        "IsVisible should return true for a shown frame with alpha=0 (alpha != visibility)"
    );
}

#[test]
fn test_create_texture_inherits_template_size() {
    use wow_ui_sim::xml::{SizeXml, TextureXml, register_texture_template};

    // Register a texture template with known size (simulates XML loading)
    register_texture_template(
        "TestTileTemplate",
        TextureXml {
            size: Some(SizeXml {
                x: Some(256.0),
                y: Some(256.0),
                abs_dimension: None,
            }),
            ..Default::default()
        },
    );

    let env = env();
    // CreateTexture(name, layer, inherits, subLevel) should apply template size
    let (w, h): (f64, f64) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", nil, UIParent)
        local tex = f:CreateTexture(nil, "BACKGROUND", "TestTileTemplate")
        return tex:GetSize()
    "#,
        )
        .unwrap();
    assert_eq!(
        w, 256.0,
        "CreateTexture with inherits should apply template width"
    );
    assert_eq!(
        h, 256.0,
        "CreateTexture with inherits should apply template height"
    );
}

#[test]
fn test_create_texture_applies_sublevel_argument() {
    let env = env();
    let (layer, sublevel): (String, i32) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", nil, UIParent)
        local tex = f:CreateTexture(nil, "OVERLAY", nil, 7)
        return tex:GetDrawLayer()
    "#,
        )
        .unwrap();

    assert_eq!(
        layer, "OVERLAY",
        "CreateTexture should keep the requested draw layer"
    );
    assert_eq!(
        sublevel, 7,
        "CreateTexture should apply the requested draw sublevel"
    );
}
