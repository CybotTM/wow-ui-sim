//! Post-load workarounds that are still required on the live rilua path.

use std::time::Instant;

pub fn apply(env: &crate::lua_api::WowLuaEnv) {
    log_step(env, "patch_edit_mode_manager", || {
        crate::lua_api::workarounds_editmode::patch_edit_mode_manager(env);
    });
    log_step(env, "init_edit_mode_layout", || {
        crate::lua_api::workarounds_editmode::init_edit_mode_layout(env);
    });
    log_step(env, "patch_ui_parent_panel_toggles", || {
        patch_ui_parent_panel_toggles(env);
    });
    log_step(env, "patch_character_select_list", || {
        patch_character_select_list(env);
    });
    log_step(env, "patch_character_create_defaults", || {
        patch_character_create_defaults(env);
    });
    log_step(env, "patch_vignette_pin_template", || {
        patch_vignette_pin_template(env);
    });
    log_step(env, "patch_fog_of_war_pin_mixin", || {
        patch_fog_of_war_pin_mixin(env);
    });
    log_step(env, "patch_map_exploration_pin_mixin", || {
        patch_map_exploration_pin_mixin(env);
    });
}

pub fn apply_post_event(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(REFRESH_ACTION_BUTTONS_LUA);
}

pub fn apply_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if matches!(
        addon_name,
        "Blizzard_SharedTalentUI" | "Blizzard_PlayerSpells"
    ) {
        patch_shared_talent_util(env);
    }
    if addon_name == "Blizzard_MapCanvas" {
        patch_map_canvas_scroll_container(env);
    }
    if matches!(
        addon_name,
        "Blizzard_MapCanvas"
            | "Blizzard_SharedMapDataProviders"
            | "Blizzard_WorldMap"
            | "Blizzard_BattlefieldMap"
    ) {
        patch_fog_of_war_pin_mixin_for_runtime_addon_load(env);
        patch_map_exploration_pin_mixin_for_runtime_addon_load(env);
    }
    if addon_name == "Blizzard_AccountStore" {
        let _ = patch_account_store_set_storefront(env);
    }
}

fn log_with_timestamp(env: &crate::lua_api::WowLuaEnv, message: &str) {
    let start_time = env.state().borrow().start_time;
    eprintln!("{} {}", crate::logging::elapsed_prefix(start_time), message);
}

fn log_step(env: &crate::lua_api::WowLuaEnv, label: &str, apply_step: impl FnOnce()) {
    log_with_timestamp(env, &format!("[Workarounds] starting {label}"));
    let started = Instant::now();
    apply_step();
    log_with_timestamp(
        env,
        &format!(
            "[Workarounds] finished {label} in {:.2?}",
            started.elapsed()
        ),
    );
}

fn patch_ui_parent_panel_toggles(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(GETGLOBAL_HELPER_LUA);
    let _ = env.exec(TOGGLE_ACHIEVEMENT_FRAME_LUA);
    let _ = env.exec(TOGGLE_ENCOUNTER_JOURNAL_LUA);
    let _ = env.exec(TOGGLE_COLLECTIONS_JOURNAL_LUA);
    let _ = env.exec(MAIN_MENU_MICROBUTTON_CLICK_WORKAROUND_LUA);
}

fn patch_vignette_pin_template(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(VIGNETTE_PIN_TEMPLATE_WORKAROUND_LUA);
}

fn patch_character_select_list(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(CHARACTER_SELECT_LIST_WORKAROUND_LUA);
}

fn patch_fog_of_war_pin_mixin(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(FOG_OF_WAR_PIN_WORKAROUND_LUA);
}

fn patch_map_exploration_pin_mixin(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(MAP_EXPLORATION_PIN_WORKAROUND_LUA);
}

fn patch_character_create_defaults(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(CHARACTER_CREATE_DEFAULTS_WORKAROUND_LUA);
}

fn patch_fog_of_war_pin_mixin_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(FOG_OF_WAR_PIN_WORKAROUND_LUA);
}

fn patch_map_exploration_pin_mixin_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(MAP_EXPLORATION_PIN_WORKAROUND_LUA);
}

pub(crate) fn patch_account_store_set_storefront(
    env: &crate::lua_api::LoaderEnv<'_>,
) -> Result<(), crate::Error> {
    env.exec(
        r#"
        local function __wow_account_store_set_storefront_id(self, storeFrontID)
            self.storeFrontID = storeFrontID
        end

        if type(AccountStoreMixin) == "table" then
            AccountStoreMixin.SetStoreFrontID = __wow_account_store_set_storefront_id
        end
        if type(AccountStoreFrame) == "table" then
            AccountStoreFrame.SetStoreFrontID = __wow_account_store_set_storefront_id
        end
        "#,
    )
}

fn patch_map_canvas_scroll_container(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(
        r#"
local function __wow_find_first_scroll_frame_child(parent)
  if type(parent) ~= "table" or type(parent.GetNumChildren) ~= "function" or type(parent.GetChildren) ~= "function" then
    return nil
  end
  local count = parent:GetNumChildren()
  for index = 1, count do
    local child = select(index, parent:GetChildren())
    if type(child) == "table" then
      local isScrollFrame =
        (type(child.IsObjectType) == "function" and child:IsObjectType("ScrollFrame")) or
        (type(child.GetObjectType) == "function" and child:GetObjectType() == "ScrollFrame")
      if isScrollFrame then
        return child
      end
    end
  end
  return nil
end

local function __wow_ensure_map_canvas_scroll_container(frame)
  if type(frame) ~= "table" then
    return nil
  end

  local existing = rawget(frame, "ScrollContainer")
  if existing ~= nil then
    return existing
  end

  local scroll = __wow_find_first_scroll_frame_child(frame)
  if scroll ~= nil then
    rawset(frame, "ScrollContainer", scroll)
  end
  return scroll
end

local function __wow_try_init_map_canvas(frame)
  if type(frame) ~= "table" then
    return
  end

  __wow_ensure_map_canvas_scroll_container(frame)
  if rawget(frame, "__wow_map_canvas_onload_ran") then
    return
  end

  local scroll = rawget(frame, "ScrollContainer")
  if scroll == nil then
    return
  end

  rawset(frame, "__wow_map_canvas_onload_ran", true)
  local originalOnLoad = rawget(_G, "__wow_map_canvas_original_onload")
  if type(originalOnLoad) == "function" then
    originalOnLoad(frame)
  end
end

local function __wow_refresh_map_canvas_size(frame)
  if type(frame) ~= "table" then
    return
  end

  local scroll = rawget(frame, "ScrollContainer")
  if type(scroll) ~= "table" then
    return
  end

  local child = rawget(scroll, "Child")
  local childWidth = type(child) == "table" and type(child.GetWidth) == "function" and child:GetWidth() or 0
  local childHeight = type(child) == "table" and type(child.GetHeight) == "function" and child:GetHeight() or 0
  if childWidth ~= 0 and childHeight ~= 0 then
    return
  end

  local mapID = rawget(frame, "mapID")
  if (mapID == nil or mapID == 0) and type(frame.GetMapID) == "function" then
    mapID = frame:GetMapID()
  end

  if mapID ~= nil and mapID ~= 0 and type(scroll.SetMapID) == "function" then
    scroll:SetMapID(mapID)
  elseif type(scroll.OnCanvasSizeChanged) == "function" then
    scroll:OnCanvasSizeChanged()
  end

  childWidth = type(child) == "table" and type(child.GetWidth) == "function" and child:GetWidth() or 0
  childHeight = type(child) == "table" and type(child.GetHeight) == "function" and child:GetHeight() or 0
  if childWidth ~= 0 and childHeight ~= 0 then
    return
  end

  local layers = mapID ~= nil
    and mapID ~= 0
    and C_Map ~= nil
    and type(C_Map.GetMapArtLayers) == "function"
    and C_Map.GetMapArtLayers(mapID)
    or nil
  local layer = type(layers) == "table" and layers[1] or nil
  if type(child) ~= "table" or type(layer) ~= "table" then
    return
  end

  local layerWidth = layer.layerWidth or 0
  local layerHeight = layer.layerHeight or 0
  if layerWidth == 0 or layerHeight == 0 then
    return
  end

  if type(child.SetSize) == "function" then
    child:SetSize(layerWidth, layerHeight)
  end

  local tiledBackground = rawget(child, "TiledBackground")
  if type(tiledBackground) == "table" and type(tiledBackground.SetSize) == "function" then
    tiledBackground:SetSize(layerWidth * 2, layerHeight * 2)
  end

  if type(scroll.CalculateScaleExtents) == "function" then
    scroll:CalculateScaleExtents()
  end
  if type(scroll.CalculateScrollExtents) == "function" then
    scroll:CalculateScrollExtents()
  end
  if type(frame.OnCanvasSizeChanged) == "function" then
    frame:OnCanvasSizeChanged()
  end
end

local function __wow_patch_live_map_canvas(frame)
  if type(frame) ~= "table" or type(MapCanvasMixin) ~= "table" then
    return
  end

  if type(MapCanvasMixin.SetMapID) == "function" then
    frame.SetMapID = MapCanvasMixin.SetMapID
  end
  if type(MapCanvasMixin.GetCanvas) == "function" then
    frame.GetCanvas = MapCanvasMixin.GetCanvas
  end
  if type(MapCanvasMixin.GetCanvasContainer) == "function" then
    frame.GetCanvasContainer = MapCanvasMixin.GetCanvasContainer
  end
  if type(MapCanvasMixin.OnFrameSizeChanged) == "function" then
    frame.OnFrameSizeChanged = MapCanvasMixin.OnFrameSizeChanged
  end
  if type(MapCanvasMixin.OnShow) == "function" then
    frame.OnShow = MapCanvasMixin.OnShow
  end

  __wow_try_init_map_canvas(frame)
  __wow_refresh_map_canvas_size(frame)
end

local function __wow_patch_world_map_display_state(frame)
  if type(frame) ~= "table" or type(frame.SetDisplayState) ~= "function" then
    return
  end
  if rawget(frame, "__wow_display_state_refresh_patched") then
    return
  end

  local originalSetDisplayState = frame.SetDisplayState
  frame.SetDisplayState = function(self, ...)
    local result = originalSetDisplayState(self, ...)
    __wow_try_init_map_canvas(self)
    __wow_refresh_map_canvas_size(self)
    return result
  end

  rawset(frame, "__wow_display_state_refresh_patched", true)
end

local function __wow_ensure_map_canvas_zoom_levels(scroll)
  if type(scroll) ~= "table" or type(scroll.zoomLevels) == "table" then
    return
  end

  local mapID = rawget(scroll, "mapID")
  if (mapID == nil or mapID == 0) and type(scroll.GetMap) == "function" then
    local map = scroll:GetMap()
    if type(map) == "table" and type(map.GetMapID) == "function" then
      mapID = map:GetMapID()
    end
  end

  local layers = mapID ~= nil
    and mapID ~= 0
    and C_Map ~= nil
    and type(C_Map.GetMapArtLayers) == "function"
    and C_Map.GetMapArtLayers(mapID)
    or nil
  if type(layers) ~= "table" or type(layers[1]) ~= "table" then
    scroll.zoomLevels = { { scale = 1.0, layerIndex = 1 } }
    scroll.targetScale = scroll.targetScale or 1.0
    return
  end

  local zoomLevels = {}
  for index, layer in ipairs(layers) do
    zoomLevels[index] = {
      scale = layer.minScale or 1.0,
      layerIndex = index,
    }
  end
  scroll.zoomLevels = zoomLevels
  scroll.targetScale = scroll.targetScale or zoomLevels[1].scale or 1.0
end

local function __wow_refresh_world_map_canvas()
  __wow_patch_live_map_canvas(WorldMapFrame)
  __wow_patch_world_map_display_state(WorldMapFrame)
end

if type(MapCanvasMixin) == "table" and not rawget(_G, "__wow_map_canvas_scroll_container_patched") then
  if rawget(_G, "__wow_map_canvas_original_onload") == nil and type(MapCanvasMixin.OnLoad) == "function" then
    _G.__wow_map_canvas_original_onload = MapCanvasMixin.OnLoad
    MapCanvasMixin.OnLoad = function(self, ...)
      if rawget(self, "__wow_map_canvas_onload_ran") then
        return
      end
      __wow_try_init_map_canvas(self)
    end
  end

  if type(MapCanvasMixin.SetMapID) == "function" then
    local originalSetMapID = MapCanvasMixin.SetMapID
    MapCanvasMixin.SetMapID = function(self, ...)
      __wow_try_init_map_canvas(self)
      if rawget(self, "ScrollContainer") == nil then
        local mapID = ...
        self.mapID = mapID
        if C_Map and type(C_Map.GetMapArtID) == "function" then
          self.mapArtID = C_Map.GetMapArtID(mapID)
        end
        return
      end
      local result = originalSetMapID(self, ...)
      __wow_refresh_map_canvas_size(self)
      return result
    end
  end

  if type(MapCanvasMixin.OnShow) == "function" then
    local originalOnShow = MapCanvasMixin.OnShow
    MapCanvasMixin.OnShow = function(self, ...)
      __wow_try_init_map_canvas(self)
      local result = originalOnShow(self, ...)
      __wow_refresh_map_canvas_size(self)
      return result
    end
  end

  if type(MapCanvasMixin.GetCanvas) == "function" then
    MapCanvasMixin.GetCanvas = function(self, ...)
      __wow_try_init_map_canvas(self)
      local scroll = rawget(self, "ScrollContainer")
      return scroll and scroll.Child or nil
    end
  end

  if type(MapCanvasMixin.GetCanvasContainer) == "function" then
    MapCanvasMixin.GetCanvasContainer = function(self, ...)
      __wow_try_init_map_canvas(self)
      return rawget(self, "ScrollContainer")
    end
  end

  if type(MapCanvasMixin.OnFrameSizeChanged) == "function" then
    local originalOnFrameSizeChanged = MapCanvasMixin.OnFrameSizeChanged
    MapCanvasMixin.OnFrameSizeChanged = function(self, ...)
      __wow_try_init_map_canvas(self)
      if rawget(self, "ScrollContainer") == nil then
        return
      end
      return originalOnFrameSizeChanged(self, ...)
    end
  end

  if type(MapCanvasScrollControllerMixin) == "table"
    and type(MapCanvasScrollControllerMixin.GetZoomLevelIndexForScale) == "function"
  then
    local originalGetZoomLevelIndexForScale = MapCanvasScrollControllerMixin.GetZoomLevelIndexForScale
    MapCanvasScrollControllerMixin.GetZoomLevelIndexForScale = function(self, scale)
      __wow_ensure_map_canvas_zoom_levels(self)
      return originalGetZoomLevelIndexForScale(self, scale)
    end
  end

  __wow_map_canvas_scroll_container_patched = true
end

for _, mapName in ipairs({ "WorldMapFrame", "BattlefieldMapFrame" }) do
  __wow_patch_live_map_canvas(_G[mapName])
end
__wow_patch_world_map_display_state(WorldMapFrame)

if type(ToggleWorldMap) == "function" and not rawget(_G, "__wow_toggle_world_map_refresh_patched") then
  local originalToggleWorldMap = ToggleWorldMap
  ToggleWorldMap = function(...)
    local result = originalToggleWorldMap(...)
    __wow_refresh_world_map_canvas()
    return result
  end

  if type(OpenWorldMap) == "function" then
    local originalOpenWorldMap = OpenWorldMap
    OpenWorldMap = function(...)
      local result = originalOpenWorldMap(...)
      __wow_refresh_world_map_canvas()
      return result
    end
  end

  rawset(_G, "__wow_toggle_world_map_refresh_patched", true)
end
"#,
    );
}

const GETGLOBAL_HELPER_LUA: &str = r#"
local function __wow_getglobal(name)
    return getglobal(name)
end
_G.__wow_panel_getglobal = __wow_getglobal
"#;

const CHARACTER_SELECT_LIST_WORKAROUND_LUA: &str = r#"
if type(CharacterSelectCharacterFrame) == "table"
    and type(CharacterSelectCharacterFrame.UpdateCharacterSelection) == "function"
    and not rawget(_G, "__wow_character_select_list_refreshed") then
    pcall(function()
        CharacterSelectCharacterFrame:UpdateCharacterSelection()
    end)
    rawset(_G, "__wow_character_select_list_refreshed", true)
end
"#;

const CHARACTER_CREATE_DEFAULTS_WORKAROUND_LUA: &str = r#"
local function __wow_character_create_defaults_frame()
    if type(CharacterCreateFrame) ~= "table" then
        return nil
    end
    return CharacterCreateFrame.RaceAndClassFrame
end

local function __wow_seed_character_create_defaults(frame)
    if type(frame) ~= "table" then
        return
    end

    local raceID = C_CharacterCreation and C_CharacterCreation.GetSelectedRace and C_CharacterCreation.GetSelectedRace() or 1
    if type(frame.selectedRaceData) ~= "table" then
        frame.selectedRaceData = C_CharacterCreation and C_CharacterCreation.GetRaceDataByID and C_CharacterCreation.GetRaceDataByID(raceID) or { enabled = true, isNeutralRace = false, factionInternalName = "Alliance" }
    end
    if type(frame.selectedClassData) ~= "table" then
        frame.selectedClassData = C_CharacterCreation and C_CharacterCreation.GetSelectedClass and C_CharacterCreation.GetSelectedClass() or { classID = 2, earlyFactionChoice = false }
    end
    if frame.selectedFaction == nil and C_CharacterCreation and C_CharacterCreation.GetFactionForRace then
        frame.selectedFaction = C_CharacterCreation.GetFactionForRace(raceID)
    end
end

local function __wow_seed_character_create_frame(frame)
    if type(frame) ~= "table" then
        return
    end

    if type(frame.BGTex) ~= "table" then
        frame.BGTex = {}
    end

    if type(frame.BackButton) == "table"
        and type(frame.BackButton.UpdateText) == "function"
        and type(frame.BackButton.GetText) == "function"
        and (frame.BackButton:GetText() == nil or frame.BackButton:GetText() == "")
    then
        frame.BackButton:UpdateText(BACK, BACKWARD_ARROW)
    end

    if type(frame.UpdateForwardButton) == "function" then
        frame:UpdateForwardButton()
    end
end

local characterCreateFrame = type(CharacterCreateFrame) == "table" and CharacterCreateFrame or nil
local raceAndClassFrame = characterCreateFrame and characterCreateFrame.RaceAndClassFrame or nil
if raceAndClassFrame ~= nil then
    __wow_seed_character_create_defaults(raceAndClassFrame)
end
if characterCreateFrame ~= nil then
    __wow_seed_character_create_frame(characterCreateFrame)
end

if type(CharacterCreateMixin) == "table" and type(CharacterCreateMixin.CreateCharacter) == "function" and not rawget(_G, "__wow_character_create_defaults_patched") then
    local originalCreateCharacter = CharacterCreateMixin.CreateCharacter
    function CharacterCreateMixin:CreateCharacter(...)
        __wow_seed_character_create_defaults(__wow_character_create_defaults_frame())
        __wow_seed_character_create_frame(self)
        if A_Admin and type(A_Admin.SetPlayerName) == "function" and type(self.GetSelectedName) == "function" then
            A_Admin.SetPlayerName(self:GetSelectedName())
        end
        return originalCreateCharacter(self, ...)
    end
    rawset(_G, "__wow_character_create_defaults_patched", true)
end

if type(CharacterCreateRaceAndClassMixin) == "table" and type(CharacterCreateRaceAndClassMixin.GetCreateCharacterFaction) == "function" and not rawget(_G, "__wow_character_create_faction_patched") then
    local originalGetCreateCharacterFaction = CharacterCreateRaceAndClassMixin.GetCreateCharacterFaction
    function CharacterCreateRaceAndClassMixin:GetCreateCharacterFaction()
        __wow_seed_character_create_defaults(self)
        return originalGetCreateCharacterFaction(self)
    end
    rawset(_G, "__wow_character_create_faction_patched", true)
end

if type(CharacterCreateRaceAndClassMixin) == "table" and type(CharacterCreateRaceAndClassMixin.UpdateState) == "function" and not rawget(_G, "__wow_character_create_update_patched") then
    local originalUpdateState = CharacterCreateRaceAndClassMixin.UpdateState
    function CharacterCreateRaceAndClassMixin:UpdateState(selectedFaction)
        __wow_seed_character_create_defaults(self)
        local result = originalUpdateState(self, selectedFaction)
        __wow_seed_character_create_frame(CharacterCreateFrame)
        return result
    end
    rawset(_G, "__wow_character_create_update_patched", true)
end

if type(CharacterCreateMixin) == "table" and type(CharacterCreateMixin.UpdateBackgroundOverlays) == "function" and not rawget(_G, "__wow_character_create_background_overlay_patched") then
    local originalUpdateBackgroundOverlays = CharacterCreateMixin.UpdateBackgroundOverlays
    function CharacterCreateMixin:UpdateBackgroundOverlays(selectedClassData, selectedRaceData)
        local ok = pcall(originalUpdateBackgroundOverlays, self, selectedClassData, selectedRaceData)
        if ok then
            return
        end

        local backgroundTextures = self and self.BGTex or nil
        if type(backgroundTextures) == "table" then
            local iter_ok, iter, state, first = pcall(ipairs, backgroundTextures)
            if iter_ok and type(iter) == "function" then
                for _, texture in iter, state, first do
                    if type(texture) == "table" and type(texture.SetAlpha) == "function" then
                        texture:SetAlpha(1)
                    end
                end
                return
            end
        end

        if type(backgroundTextures) == "table" and type(backgroundTextures.SetAlpha) == "function" then
            backgroundTextures:SetAlpha(1)
        end
    end
    rawset(_G, "__wow_character_create_background_overlay_patched", true)
end
"#;

const FOG_OF_WAR_PIN_WORKAROUND_LUA: &str = r#"
local function __wow_clear_fog_of_war_pin_assets(pin)
    if type(pin) ~= "table" then
        return
    end
    if type(pin.SetFogOfWarID) == "function" then
        pin:SetFogOfWarID(nil, true)
    end
    if type(pin.SetFogOfWarBackgroundAtlas) == "function" then
        pin:SetFogOfWarBackgroundAtlas(nil)
    end
    if type(pin.SetFogOfWarMaskAtlas) == "function" then
        pin:SetFogOfWarMaskAtlas(nil)
    end
end

local function __wow_resolve_fog_of_war_map_id(pin)
    local mapID = nil
    if type(pin) == "table" and type(pin.GetMap) == "function" then
        local map = pin:GetMap()
        if map ~= nil and type(map.GetMapID) == "function" then
            mapID = map:GetMapID()
        end
    end

    if (mapID == nil or mapID == 0) and C_Map ~= nil and type(C_Map.GetCurrentMapID) == "function" then
        mapID = C_Map.GetCurrentMapID()
    end

    return mapID or 0
end

local function __wow_refresh_fog_of_war_pin(pin, forceUpdate)
    if type(pin) ~= "table" then
        return
    end

    local mapID = __wow_resolve_fog_of_war_map_id(pin)
    if type(pin.SetUiMapID) == "function" then
        pin:SetUiMapID(mapID)
    end

    if mapID == 0 then
        __wow_clear_fog_of_war_pin_assets(pin)
        if type(pin.Hide) == "function" then
            pin:Hide()
        end
        return
    end

    local fogOfWarID = nil
    if C_FogOfWar ~= nil and type(C_FogOfWar.GetFogOfWarForMap) == "function" then
        fogOfWarID = C_FogOfWar.GetFogOfWarForMap(mapID)
    end
    if type(pin.SetFogOfWarID) == "function" then
        pin:SetFogOfWarID(fogOfWarID, forceUpdate)
    end

    local hasBackgroundAtlas =
        type(pin.GetFogOfWarBackgroundAtlas) == "function" and pin:GetFogOfWarBackgroundAtlas() ~= nil
    local hasMaskAtlas =
        type(pin.GetFogOfWarMaskAtlas) == "function" and pin:GetFogOfWarMaskAtlas() ~= nil
    if fogOfWarID == nil or (not hasBackgroundAtlas and not hasMaskAtlas) then
        __wow_clear_fog_of_war_pin_assets(pin)
        if type(pin.Hide) == "function" then
            pin:Hide()
        end
    end
end

local function __wow_apply_fog_of_war_pin_workaround(pin)
    if type(pin) ~= "table" then
        return
    end
    if type(FogOfWarPinMixin) == "table" and type(FogOfWarPinMixin.OnMapChanged) == "function" then
        pin.OnMapChanged = FogOfWarPinMixin.OnMapChanged
    end
    if type(FogOfWarFrameMixin) == "table" and type(FogOfWarFrameMixin.TryFindingBestFogOfWarID) == "function" then
        pin.TryFindingBestFogOfWarID = FogOfWarFrameMixin.TryFindingBestFogOfWarID
    end
    __wow_refresh_fog_of_war_pin(pin, true)
end

local function __wow_patch_live_fog_of_war_pins(map)
    if type(map) ~= "table" then
        return
    end

    if type(map.EnumeratePinsByTemplate) == "function" then
        for pin in map:EnumeratePinsByTemplate("FogOfWarPinTemplate") do
            __wow_apply_fog_of_war_pin_workaround(pin)
        end
    end

    if type(map.dataProviders) ~= "table" then
        return
    end

    for provider in pairs(map.dataProviders) do
        local pin = type(provider) == "table" and rawget(provider, "pin") or nil
        if type(pin) == "table" then
            __wow_apply_fog_of_war_pin_workaround(pin)
        end
    end
end

if type(FogOfWarPinMixin) == "table" and not rawget(_G, "__wow_fog_of_war_pin_methods_patched") then
    if type(FogOfWarFrameMixin) == "table" and type(FogOfWarFrameMixin.TryFindingBestFogOfWarID) == "function" then
        FogOfWarFrameMixin.TryFindingBestFogOfWarID = function(self, forceUpdate)
            __wow_refresh_fog_of_war_pin(self, forceUpdate)
        end
    end

    if type(FogOfWarPinMixin.OnMapChanged) == "function" then
        FogOfWarPinMixin.OnMapChanged = function(self)
            __wow_refresh_fog_of_war_pin(self, true)
        end
    end

    rawset(_G, "__wow_fog_of_war_pin_methods_patched", true)
end

for _, mapName in ipairs({ "WorldMapFrame", "BattlefieldMapFrame" }) do
    __wow_patch_live_fog_of_war_pins(_G[mapName])
end
"#;

const MAP_EXPLORATION_PIN_WORKAROUND_LUA: &str = r#"
local function __wow_size_map_exploration_pin(pin)
    if type(pin) ~= "table" then
        return
    end
    if type(pin.OnCanvasSizeChanged) == "function" then
        pin:OnCanvasSizeChanged()
    end
end

local function __wow_patch_live_map_exploration_pins(map)
    if type(map) ~= "table" then
        return
    end

    if type(map.EnumeratePinsByTemplate) == "function" then
        for pin in map:EnumeratePinsByTemplate("MapExplorationPinTemplate") do
            __wow_size_map_exploration_pin(pin)
        end
    end

    if type(map.dataProviders) ~= "table" then
        return
    end

    for provider in pairs(map.dataProviders) do
        local pin = type(provider) == "table" and rawget(provider, "pin") or nil
        if type(pin) == "table"
            and type(pin.RefreshOverlays) == "function"
            and type(pin.OnCanvasSizeChanged) == "function"
        then
            if type(MapExplorationPinMixin) == "table" and type(MapExplorationPinMixin.RefreshOverlays) == "function" then
                pin.RefreshOverlays = MapExplorationPinMixin.RefreshOverlays
            end
            __wow_size_map_exploration_pin(pin)
        end
    end
end

if type(MapExplorationPinMixin) == "table" and not rawget(_G, "__wow_map_exploration_pin_patched") then
    if type(MapExplorationPinMixin.OnAcquired) == "function" then
        local originalOnAcquired = MapExplorationPinMixin.OnAcquired
        MapExplorationPinMixin.OnAcquired = function(self, dataProvider)
            originalOnAcquired(self, dataProvider)
            __wow_size_map_exploration_pin(self)
        end
    end

    if type(MapExplorationPinMixin.RefreshOverlays) == "function" then
        local originalRefreshOverlays = MapExplorationPinMixin.RefreshOverlays
        MapExplorationPinMixin.RefreshOverlays = function(self, fullUpdate)
            __wow_size_map_exploration_pin(self)
            return originalRefreshOverlays(self, fullUpdate)
        end
    end

    rawset(_G, "__wow_map_exploration_pin_patched", true)
end

for _, mapName in ipairs({ "WorldMapFrame", "BattlefieldMapFrame" }) do
    __wow_patch_live_map_exploration_pins(_G[mapName])
end
"#;

const TOGGLE_ACHIEVEMENT_FRAME_LUA: &str = r#"
if __wow_panel_getglobal ~= nil then
    local __wow_getglobal = __wow_panel_getglobal
    function ToggleAchievementFrame(stats)
        local kiosk = __wow_getglobal("Kiosk")
        if ( (kiosk and kiosk.IsEnabled and kiosk.IsEnabled()) or __wow_getglobal("DISALLOW_FRAME_TOGGLING") ) then
            return;
        end

        local cAddOns = __wow_getglobal("C_AddOns")
        if cAddOns and cAddOns.LoadAddOn and cAddOns.IsAddOnLoaded and not cAddOns.IsAddOnLoaded("Blizzard_AchievementUI") then
            cAddOns.LoadAddOn("Blizzard_AchievementUI");
        end
        local achievementFrame = __wow_getglobal("AchievementFrame")
        if not achievementFrame then
            return;
        end

        local requestedTab = stats and 3 or 1
        if achievementFrame:IsShown() and achievementFrame.selectedTab == requestedTab then
            achievementFrame:Hide();
        else
            achievementFrame.selectedTab = requestedTab
            achievementFrame:Show();
        end
    end
end
"#;

const TOGGLE_ENCOUNTER_JOURNAL_LUA: &str = r#"
if __wow_panel_getglobal ~= nil then
    local __wow_getglobal = __wow_panel_getglobal
    function ToggleEncounterJournal()
        local kiosk = __wow_getglobal("Kiosk")
        if ( (kiosk and kiosk.IsEnabled and kiosk.IsEnabled()) or __wow_getglobal("DISALLOW_FRAME_TOGGLING") ) then
            return;
        end

        if ( not __wow_getglobal("EncounterJournal") ) then
            local cAddOns = __wow_getglobal("C_AddOns")
            if cAddOns and cAddOns.LoadAddOn then
                cAddOns.LoadAddOn("Blizzard_EncounterJournal");
            end
        end
        local encounterJournal = __wow_getglobal("EncounterJournal")
        if ( encounterJournal ) then
            if encounterJournal:IsShown() then
                encounterJournal:Hide();
            else
                encounterJournal:Show();
            end
            return true;
        end
        return false;
    end
end
"#;

const MAIN_MENU_MICROBUTTON_CLICK_WORKAROUND_LUA: &str = r#"
local function __wow_toggle_main_menu()
    if type(Menu) == "table" and type(ToggleGameMenu) == "function" then
        return ToggleGameMenu()
    end
    local gameMenuFrame = rawget(_G, "GameMenuFrame")
    if not gameMenuFrame then
        return
    end
    if type(AreAllPanelsDisallowed) == "function" and AreAllPanelsDisallowed() then
        return
    end
    if gameMenuFrame:IsShown() then
        if type(PlaySound) == "function" and SOUNDKIT and SOUNDKIT.IG_MAINMENU_QUIT then
            PlaySound(SOUNDKIT.IG_MAINMENU_QUIT)
        end
        HideUIPanel(gameMenuFrame)
    else
        if type(SettingsPanel) == "table" and type(SettingsPanel.IsShown) == "function" and SettingsPanel:IsShown() and type(SettingsPanel.Close) == "function" then
            SettingsPanel:Close()
        end
        if type(CloseMenus) == "function" then
            CloseMenus()
        end
        if type(CloseAllWindows) == "function" then
            CloseAllWindows()
        end
        if type(PlaySound) == "function" and SOUNDKIT and SOUNDKIT.IG_MAINMENU_OPEN then
            PlaySound(SOUNDKIT.IG_MAINMENU_OPEN)
        end
        ShowUIPanel(gameMenuFrame)
    end
end

if type(MainMenuMicroButtonMixin) == "table" and not MainMenuMicroButtonMixin.__wow_uisim_click_patched then
    MainMenuMicroButtonMixin.__wow_uisim_click_patched = true
    MainMenuMicroButtonMixin.OnClick = function(self, button, down)
        return __wow_toggle_main_menu()
    end
end

if type(MainMenuMicroButton) == "table" and type(MainMenuMicroButton.SetScript) == "function" then
    MainMenuMicroButton:SetScript("OnClick", function(self, button, down)
        return __wow_toggle_main_menu()
    end)
end
"#;

const TOGGLE_COLLECTIONS_JOURNAL_LUA: &str = r#"
if __wow_panel_getglobal ~= nil then
    local __wow_getglobal = __wow_panel_getglobal
    function ToggleCollectionsJournal(tabIndex)
        if __wow_getglobal("DISALLOW_FRAME_TOGGLING") then
            return;
        end

        local collectionsJournal = __wow_getglobal("CollectionsJournal")
        if not collectionsJournal then
            local cAddOns = __wow_getglobal("C_AddOns")
            if cAddOns and cAddOns.LoadAddOn then
                cAddOns.LoadAddOn("Blizzard_Collections");
            end
            collectionsJournal = __wow_getglobal("CollectionsJournal")
        end
        if not collectionsJournal then
            return
        end

        if collectionsJournal:IsShown() then
            collectionsJournal:Hide();
        else
            collectionsJournal:Show();
        end
    end
end
"#;

const SHARED_TALENT_UTIL_COMBINE_COST_ARRAYS_LUA: &str = r###"
if TalentUtil and type(TalentUtil.CombineCostArrays) == "function" and not TalentUtil.__wow_ui_sim_nil_safe_combine then
    local original = TalentUtil.CombineCostArrays
    function TalentUtil.CombineCostArrays(...)
        local combinedCostMap = {}
        for i = 1, select("#", ...) do
            local costArray = select(i, ...)
            if type(costArray) == "table" then
                for _, cost in ipairs(costArray) do
                    combinedCostMap[cost.ID] = (combinedCostMap[cost.ID] or 0) + cost.amount
                end
            end
        end

        local combinedCostArray = {}
        for ID, amount in pairs(combinedCostMap) do
            table.insert(combinedCostArray, { ID = ID, amount = amount })
        end
        return combinedCostArray
    end
    TalentUtil.__wow_ui_sim_nil_safe_combine = true
    TalentUtil.__wow_ui_sim_original_combine = original
end
"###;

const VIGNETTE_PIN_TEMPLATE_WORKAROUND_LUA: &str = r###"
local function __wow_patch_vignette_provider(provider)
    if type(provider) ~= "table" then
        return
    end
    if type(provider.GetPinTemplate) ~= "function" then
        return
    end
    if type(provider.GetDefaultPinTemplate) ~= "function" then
        return
    end
    if provider.__wow_ui_sim_nil_safe_get_pin_template then
        return
    end
    if provider:GetDefaultPinTemplate() ~= "VignettePinTemplate" then
        return
    end

    local original = provider.GetPinTemplate
    function provider:GetPinTemplate(vignetteInfo)
        if vignetteInfo == nil then
            return self:GetDefaultPinTemplate()
        end
        return original(self, vignetteInfo)
    end
    provider.__wow_ui_sim_nil_safe_get_pin_template = true
end

__wow_patch_vignette_provider(VignetteDataProviderMixin)

for _, mapName in ipairs({"WorldMapFrame", "BattlefieldMapFrame", "FlightMapFrame"}) do
    local map = _G[mapName]
    if map and type(map.dataProviders) == "table" then
        for provider in pairs(map.dataProviders) do
            __wow_patch_vignette_provider(provider)
        end
    end
end
"###;

const REFRESH_ACTION_BUTTONS_LUA: &str = r###"
local function __wow_refresh_action_button(button)
    if type(button) ~= "table" then
        return
    end
    if type(button.UpdateButtonArt) == "function" then
        pcall(button.UpdateButtonArt, button)
    end
    if type(button.UpdateHotkeys) == "function" then
        pcall(button.UpdateHotkeys, button, button.buttonType)
    end
end

for i = 1, 12 do
    __wow_refresh_action_button(_G["ActionButton" .. i])
end
"###;

fn patch_shared_talent_util(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(SHARED_TALENT_UTIL_COMBINE_COST_ARRAYS_LUA);
}
