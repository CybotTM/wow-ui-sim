//! Temporary map runtime state and helper surface.
//!
//! Core map data is partly Rust-backed in `c_api::c_map` and
//! `c_api::c_map_exploration_info`, but quest-log map selection and the
//! `UiMapPoint` helper shape still live as Lua compatibility behavior.

const MAP_RUNTIME_STATE_LUA: &str = r#"
if type(C_Map) ~= "table" then
    C_Map = {}
end
if type(UiMapPoint) ~= "table" then
    UiMapPoint = {}
end
if type(C_MapExplorationInfo) ~= "table" then
    C_MapExplorationInfo = {}
end

local __wow_map_area_names = {
  [1] = "Dun Morogh",
  [2248] = "The Isle of Dorn",
}

local function __wow_map_layer_dimensions(mapID)
  if C_Map == nil or type(C_Map.GetMapArtLayers) ~= "function" then
    return nil, nil
  end
  local layers = C_Map.GetMapArtLayers(mapID)
  if type(layers) ~= "table" then
    return nil, nil
  end
  local layer = layers[1]
  if type(layer) ~= "table" then
    return nil, nil
  end
  return layer.layerWidth, layer.layerHeight
end

local function __wow_map_make_overlay(offsetX, offsetY, textureWidth, textureHeight, fileDataIDs)
  return {
    offsetX = offsetX,
    offsetY = offsetY,
    textureWidth = textureWidth,
    textureHeight = textureHeight,
    isShownByMouseOver = false,
    isDrawOnTopLayer = false,
    fileDataIDs = fileDataIDs,
    hitRect = {
      top = offsetY,
      bottom = offsetY + textureHeight,
      left = offsetX,
      right = offsetX + textureWidth,
    },
  }
end

local function __wow_map_exploration_overlays(mapID)
  local layerWidth, layerHeight = __wow_map_layer_dimensions(mapID)
  if layerWidth == nil or layerHeight == nil then
    return {}
  end

  local topOffset = math.floor(layerHeight * 0.02)
  local overlayHeight = math.max(math.floor(layerHeight * 0.94), 1)
  local leftWidth = math.floor(layerWidth * 0.55)
  local rightOffset = math.floor(layerWidth * 0.82)
  local rightWidth = math.max(math.floor(layerWidth * 0.18), 1)
  return {
    __wow_map_make_overlay(0, topOffset, leftWidth, overlayHeight, { 4556093, 4741460 }),
    __wow_map_make_overlay(rightOffset, topOffset, rightWidth, overlayHeight, { 4556094 }),
  }
end

local function __wow_map_point_from_table(mapID, pos)
  if type(pos) ~= "table" then
    return nil
  end
  return {
    uiMapID = mapID,
    x = tonumber(pos.x) or 0.5,
    y = tonumber(pos.y) or 0.5,
  }
end

UiMapPoint.CreateFromVector2D = function(mapID, pos)
  return __wow_map_point_from_table(mapID, pos)
end

UiMapPoint.CreateFromCoordinates = function(mapID, x, y)
  return { uiMapID = mapID, x = tonumber(x) or 0, y = tonumber(y) or 0 }
end

if type(C_MapExplorationInfo.GetExploredAreaIDsAtPosition) ~= "function" then
  C_MapExplorationInfo.GetExploredAreaIDsAtPosition = function(mapID, pos)
    local areas = {}
    local point = __wow_map_point_from_table(mapID, pos)
    if point == nil then
      return areas
    end

    if mapID == C_Map.GetCurrentMapID() then
      if point.x < 0.10 or point.y < 0.05 then
        return areas
      end
      if point.x >= 0.68 and point.x <= 0.74 and point.y >= 0.20 and point.y <= 0.50 then
        return areas
      end
      if point.x <= 0.55 and point.y >= 0.05 and point.y <= 0.95 then
        areas[1] = 1
        areas[2] = 2
        return areas
      end
      if point.x >= 0.82 and point.y >= 0.05 and point.y <= 0.95 then
        areas[1] = 3
        return areas
      end
    end

    return areas
  end
end

if type(C_MapExplorationInfo.GetExploredMapTextures) ~= "function" then
  C_MapExplorationInfo.GetExploredMapTextures = function(mapID)
    if mapID ~= C_Map.GetCurrentMapID() and mapID ~= 1 then
      return {}
    end
    return __wow_map_exploration_overlays(mapID)
  end
end

local __wow_map_runtime_state = rawget(_G, "__wow_map_runtime_state")
if type(__wow_map_runtime_state) ~= "table" then
  __wow_map_runtime_state = {
    currentMapID = type(C_Map.GetCurrentMapID) == "function" and C_Map.GetCurrentMapID() or 2248,
  }
  rawset(_G, "__wow_map_runtime_state", __wow_map_runtime_state)
end

C_Map.GetCurrentMapID = function()
  return __wow_map_runtime_state.currentMapID or 2248
end

C_Map.SetMapForQuestLog = function(mapID)
  if type(mapID) ~= "number" then
    return
  end

  __wow_map_runtime_state.currentMapID = mapID

  if WorldMapFrame and type(WorldMapFrame.SetMapID) == "function" then
    WorldMapFrame:SetMapID(mapID)
  end

  if QuestMapFrame and type(QuestMapFrame.SetMapID) == "function" then
    QuestMapFrame:SetMapID(mapID)
  end
end

C_Map.GetBestMapForUnit = function(unitToken)
  if unitToken ~= nil and unitToken ~= "player" then
    return nil
  end
  if C_Map.GetCurrentMapID ~= nil then
    local currentMapID = C_Map.GetCurrentMapID()
    if currentMapID ~= nil then
      return currentMapID
    end
  end
  return 2248
end

C_Map.GetFallbackWorldMapID = function()
  if C_Map.GetCurrentMapID ~= nil then
    local currentMapID = C_Map.GetCurrentMapID()
    if currentMapID ~= nil then
      return currentMapID
    end
  end
  return 2248
end

C_Map.GetAreaInfo = function(areaID)
  if areaID == nil then
    return nil
  end
  return __wow_map_area_names[areaID]
end

C_Map.GetMapWorldSize = function(mapID)
  local layerWidth, layerHeight = __wow_map_layer_dimensions(mapID)
  if layerWidth == nil or layerHeight == nil then
    return nil
  end
  return layerWidth, layerHeight
end

C_Map.MapHasArt = function(mapID)
  if mapID == nil then
    return false
  end
  if C_Map.GetMapArtID ~= nil then
    local artID = C_Map.GetMapArtID(mapID)
    if artID ~= nil then
      return artID ~= 0
    end
  end
  return true
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(MAP_RUNTIME_STATE_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_map_runtime_state_and_helpers() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local point = UiMapPoint.CreateFromCoordinates(2248, 0.25, 0.75)
                if point.uiMapID ~= 2248 or point.x ~= 0.25 or point.y ~= 0.75 then
                    return "bad_point"
                end
                C_Map.SetMapForQuestLog(1)
                if C_Map.GetCurrentMapID() ~= 1 then
                    return "bad_current_map"
                end
                if C_Map.GetBestMapForUnit("player") ~= 1 then
                    return "bad_best_map"
                end
                if C_Map.GetBestMapForUnit("target") ~= nil then
                    return "bad_non_player_map"
                end
                if C_Map.GetAreaInfo(1) ~= "Dun Morogh" then
                    return "bad_area"
                end
                local width, height = C_Map.GetMapWorldSize(1)
                if type(width) ~= "number" or type(height) ~= "number" then
                    return "bad_world_size"
                end
                return "ok"
                "#,
            )
            .expect("map runtime probe should run");

        assert_eq!(result, "ok");
    }
}
