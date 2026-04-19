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
    log_step(env, "patch_vignette_pin_template", || {
        patch_vignette_pin_template(env);
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
}

fn patch_vignette_pin_template(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(VIGNETTE_PIN_TEMPLATE_WORKAROUND_LUA);
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
      return originalSetMapID(self, ...)
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

  __wow_map_canvas_scroll_container_patched = true
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
