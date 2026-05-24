if Mixin == nil then
  function Mixin(object, ...)
    for i = 1, select("#", ...) do
      local mixin = select(i, ...)
      if type(mixin) == "table" then
        for k, v in pairs(mixin) do
          object[k] = v
        end
      end
    end
    return object
  end
end

if CreateFromMixins == nil then
  function CreateFromMixins(...)
    return Mixin({}, ...)
  end
end

if CreateAndInitFromMixin == nil then
  function CreateAndInitFromMixin(mixin, ...)
    local object = CreateFromMixins(mixin)
    if object.Init then
      object:Init(...)
    end
    return object
  end
end

SOUNDKIT = SOUNDKIT or {}
if SOUNDKIT.CATALOG_SHOP_SELECT_NAV_MENU == nil then
  SOUNDKIT.CATALOG_SHOP_SELECT_NAV_MENU = 303824
end
if SOUNDKIT.CATALOG_SHOP_SELECT_GENERIC_UI_BUTTON == nil then
  SOUNDKIT.CATALOG_SHOP_SELECT_GENERIC_UI_BUTTON = 303826
end
-- Played by ModelSceneControlFrame, AlliedRaces banner clicks, and the
-- character-select rotation button. Canonical id from
-- Blizzard_SharedXML/Mainline/SoundKitConstants.lua:51.
if SOUNDKIT.IG_INVENTORY_ROTATE_CHARACTER == nil then
  SOUNDKIT.IG_INVENTORY_ROTATE_CHARACTER = 861
end

if CatalogShopConstants == nil then
  CatalogShopConstants = {
    ScrollViewType = {
      List = 1,
      Grid = 2,
    },
    ScrollViewElementType = {
      Header = 1,
      Product = 2,
    },
    Default = {
      PreviewBackgroundTexture = "shop-bg-map-blue",
    },
    CardTemplate = {
      Header = "CatalogShopSectionHeaderTemplate",
      HeaderPersonalized = "CatalogShopSectionHeaderOptOutLinkTemplate",
      Wide = "WideCatalogShopProductCardTemplate",
      WideCardToken = "WideWoWTokenCatalogShopCardTemplate",
      WideCardSubscription = "WideSubscriptionCatalogShopCardTemplate",
      WideCardGameTime = "WideGameTimeCatalogShopCardTemplate",
      Small = "SmallCatalogShopProductCardTemplate",
      SmallServices = "SmallCatalogShopServicesCardTemplate",
      SmallSubscription = "SmallCatalogShopSubscriptionCardTemplate",
      SmallGameTime = "SmallCatalogShopGameTimeCardTemplate",
      SmallTender = "SmallCatalogShopTenderCardTemplate",
      SmallToys = "SmallCatalogShopToysCardTemplate",
      SmallAccess = "SmallCatalogShopAccessCardTemplate",
      SmallDecor = "SmallCatalogShopDecorCardTemplate",
      SmallRoom = "SmallCatalogShopRoomCardTemplate",
      SmallExteriorType = "SmallCatalogShopExteriorTypeCardTemplate",
      Details = "DetailsCatalogShopProductCardTemplate",
      DetailsServices = "DetailsCatalogShopServicesCardTemplate",
      DetailsSubscription = "DetailsCatalogShopSubscriptionCardTemplate",
      DetailsGameTime = "DetailsCatalogShopGameTimeCardTemplate",
      DetailsTender = "DetailsCatalogShopTenderCardTemplate",
      DetailsToys = "DetailsCatalogShopToysCardTemplate",
      DetailsAccess = "DetailsCatalogShopAccessCardTemplate",
      DetailsDecor = "DetailsCatalogShopDecorCardTemplate",
      DetailsRoom = "DetailsCatalogShopRoomCardTemplate",
      DetailsExteriorType = "DetailsCatalogShopExteriorTypeCardTemplate",
    },
    ModelSceneContext = {
      SmallCard = 1,
      WideCard = 2,
      PreviewScene = 3,
    },
    ScreenPadding = {
      Horizontal = 100,
      Vertical = 100,
    },
    DefaultActorTag = {
      Pet = "pet",
      Mount = "mount",
      Toy = "actor",
      Celebrate = "fanfare",
      Transmog = "transmog",
      Decor = "decor",
    },
    ProductType = {
      Pet = "Pet",
      Mount = "Mount",
      Toy = "Toy",
      Transmog = "Transmog",
      Services = "Services",
      Token = "WoW Token",
      Bundle = "Bundle",
      Subscription = "Subscription",
      TradersTenders = "Tender",
      Decor = "Decor",
      Access = "Access",
      GameTime = "Game Time",
      Room = "Room",
      HousingExteriorType = "Housing Exteriors",
    },
    GameTypes = {
      Classic = "classic",
      Modern = "modern",
    },
    GameTypeGlobalStringTag = {
      Classic = CATALOG_SHOP_WOW_FLAVOR_CLASSIC,
      Modern = CATALOG_SHOP_WOW_FLAVOR_MODERN,
    },
    ShopGlobalStringTag = {
      MissingLicenseCaptionText = CATALOG_SHOP_MISSING_LICENSE_ITEM,
    },
    DefaultCameraTag = {
      Primary = "primary",
    },
    DefaultAnimID = {
      MountSelfIdle = 618,
      MountSpecialAnimKit = 1371,
      PetDefault = 23877,
    },
    Celebrate = {
      CreatureID = 27823,
      SpellVisualID = 259750,
    },
    NoResults = {
      BackgroundTexture = "shop-bg-map-blue",
    },
    LicenseTermTypes = {
      NotApplicable = 0,
      Days = 1,
      Years = 2,
      Fixed = 3,
      Hours = 4,
      Minutes = 5,
      Weeks = 6,
      Months = 7,
    },
    CategoryLinks = {
      Pets = "pets",
      Mounts = "mounts",
      Transmogs = "transmogs",
      Subscriptions = "subscriptions",
      GameTime = "gametimeoffers",
      GameUpgrades = "game upgrades",
      Services = "services",
      Featured = "featured",
      Housing = "housing",
    },
  }
end

if SetPortraitToTexture == nil then
  function SetPortraitToTexture(texture, texturePath)
    if type(texture) ~= "table" or type(texture.SetTexture) ~= "function" then
      return
    end

    texture:SetTexture(texturePath)

    if texture.__wowPortraitMask ~= nil then
      return
    end

    local parent = type(texture.GetParent) == "function" and texture:GetParent() or nil
    if type(parent) ~= "table" or type(parent.CreateMaskTexture) ~= "function" then
      return
    end

    local mask = parent:CreateMaskTexture(nil, "BACKGROUND")
    if type(mask) ~= "table" or type(texture.AddMaskTexture) ~= "function" then
      return
    end

    if type(mask.SetAllPoints) == "function" then
      mask:SetAllPoints(texture)
    end
    if type(mask.SetTexture) == "function" then
      mask:SetTexture("Interface\\CharacterFrame\\TempPortraitAlphaMask")
    end

    texture:AddMaskTexture(mask)
    texture.__wowPortraitMask = mask
  end
end

if mapvalues == nil then
  function mapvalues(fn, ...)
    local count = select("#", ...)
    if count == 0 then
      return
    end

    local values = {}
    for index = 1, count do
      values[index] = fn(select(index, ...))
    end

    return unpack(values, 1, count)
  end
end

if UI_LOCALE == nil then
  if type(GetLocale) == "function" then
    UI_LOCALE = GetLocale()
  else
    UI_LOCALE = "enUS"
  end
end

local function __wow_pack_results(...)
  return { n = select("#", ...), ... }
end

if SetupLocalization == nil then
  local localizeFramesCallbacks = {}

  local function call_localization_function(l10nTable, key)
    if type(l10nTable) ~= "table" then
      return
    end
    local localeTable = l10nTable[UI_LOCALE]
    if type(localeTable) ~= "table" then
      return
    end
    local localizeFn = localeTable[key]
    if type(localizeFn) == "function" then
      localizeFn()
    end
  end

  function LocalizeFrames()
    local callbacks = localizeFramesCallbacks
    localizeFramesCallbacks = {}
    for index = 1, #callbacks do
      local ok = pcall(callbacks[index])
      local _ = ok
    end
  end

  function SetupLocalization(l10nTable)
    call_localization_function(l10nTable, "localize")
    table.insert(localizeFramesCallbacks, function()
      call_localization_function(l10nTable, "localizeFrames")
    end)
  end
end

if FrameUtil == nil then
  FrameUtil = {}

  function FrameUtil.RegisterFrameForEvents(frame, events)
    if type(frame) ~= "table" or type(events) ~= "table" then
      return
    end
    for index = 1, #events do
      if type(frame.RegisterEvent) == "function" then
        frame:RegisterEvent(events[index])
      end
    end
  end

  function FrameUtil.UnregisterFrameForEvents(frame, events)
    if type(frame) ~= "table" or type(events) ~= "table" then
      return
    end
    for index = 1, #events do
      if type(frame.UnregisterEvent) == "function" then
        frame:UnregisterEvent(events[index])
      end
    end
  end

  function FrameUtil.RegisterFrameForUnitEvents(frame, events, ...)
    if type(frame) ~= "table" or type(events) ~= "table" then
      return
    end
    for index = 1, #events do
      if type(frame.RegisterUnitEvent) == "function" then
        frame:RegisterUnitEvent(events[index], ...)
      end
    end
  end

  function FrameUtil.GetRootParent(frame)
    if type(frame) ~= "table" or type(frame.GetParent) ~= "function" then
      return nil
    end

    local parent = frame:GetParent()
    while parent do
      if type(parent.GetParent) ~= "function" then
        break
      end
      local nextParent = parent:GetParent()
      if not nextParent then
        break
      end
      parent = nextParent
    end
    return parent
  end

  function FrameUtil.SetParentMaintainRenderLayering(frame, parent)
    if type(frame) ~= "table" or parent == nil then
      return
    end

    local origStrata = type(frame.GetFrameStrata) == "function" and frame:GetFrameStrata() or nil
    local origFrameLevel = type(frame.GetFrameLevel) == "function" and frame:GetFrameLevel() or nil

    if type(frame.SetParent) == "function" then
      frame:SetParent(parent)
    end
    if origStrata ~= nil and type(frame.SetFrameStrata) == "function" then
      frame:SetFrameStrata(origStrata)
    end
    if origFrameLevel ~= nil and type(frame.SetFrameLevel) == "function" then
      frame:SetFrameLevel(origFrameLevel)
    end
  end

  function FrameUtil.UpdateScaleForFit(frame, extraWidth, extraHeight)
    extraWidth = extraWidth or 0
    extraHeight = extraHeight or 0
    FrameUtil.UpdateScaleForFitSpecific(
      frame,
      frame:GetWidth() + extraWidth,
      frame:GetHeight() + extraHeight
    )
  end

  function FrameUtil.UpdateScaleForFitSpecific(frame, specificWidth, specificHeight)
    if type(frame) ~= "table"
      or type(frame.SetScale) ~= "function"
      or type(frame.GetWidth) ~= "function"
      or type(frame.GetHeight) ~= "function"
      or type(GetAppropriateTopLevelParent) ~= "function"
    then
      return
    end

    frame:SetScale(1)

    local topLevelParent = GetAppropriateTopLevelParent()
    if type(topLevelParent) ~= "table"
      or type(topLevelParent.GetWidth) ~= "function"
      or type(topLevelParent.GetHeight) ~= "function"
    then
      return
    end

    local horizRatio = topLevelParent:GetWidth() / (specificWidth or frame:GetWidth())
    local vertRatio = topLevelParent:GetHeight() / (specificHeight or frame:GetHeight())

    if horizRatio < 1 or vertRatio < 1 then
      frame:SetScale(math.min(horizRatio, vertRatio))
    end
  end

  function FrameUtil.UpdateTopLevelParent(frame)
    if type(frame) ~= "table"
      or type(frame.GetParent) ~= "function"
      or type(GetAppropriateTopLevelParent) ~= "function"
    then
      return
    end

    local oldParent = frame:GetParent()
    local newParent = GetAppropriateTopLevelParent(oldParent)
    if oldParent ~= newParent then
      FrameUtil.SetParentMaintainRenderLayering(frame, newParent)
    end
  end

  function FrameUtil.RegisterForTopLevelParentChanged(frame)
    if type(EventRegistry) ~= "table" or type(EventRegistry.RegisterCallback) ~= "function" then
      return
    end
    EventRegistry:RegisterCallback("UI.AlternateTopLevelParentChanged", function()
      FrameUtil.UpdateTopLevelParent(frame)
    end, frame)
  end
end

if EventUtil == nil then
  local eventUtilState = {
    allEventsWatchers = {},
    onceWatchers = {},
    registeredEvents = {},
    seenEvents = {},
    variablesLoadedCallbacks = {},
    variablesLoadedTriggered = false,
  }

  local dispatcher = nil

  local function ensure_dispatcher()
    if dispatcher == nil and type(CreateFrame) == "function" then
      dispatcher = CreateFrame("Frame")
      dispatcher:SetScript("OnEvent", function(_, event, ...)
        eventUtilState.seenEvents[event] = true

        if event == "VARIABLES_LOADED" and not eventUtilState.variablesLoadedTriggered then
          eventUtilState.variablesLoadedTriggered = true
          if type(UIParent) == "table" then
            UIParent.variablesLoaded = true
          end
          local callbacks = eventUtilState.variablesLoadedCallbacks
          eventUtilState.variablesLoadedCallbacks = {}
          for i = 1, #callbacks do
            local callback = callbacks[i]
            if type(callback) == "function" then
              callback()
            end
          end
        end

        for index = #eventUtilState.onceWatchers, 1, -1 do
          local watcher = eventUtilState.onceWatchers[index]
          if watcher.event == event then
            local matches = true
            for requiredIndex = 1, watcher.requiredArgs.n do
              if select(requiredIndex, ...) ~= watcher.requiredArgs[requiredIndex] then
                matches = false
                break
              end
            end
            if matches then
              watcher.handle.registered = false
              table.remove(eventUtilState.onceWatchers, index)
              if type(watcher.callback) == "function" then
                watcher.callback(...)
              end
            end
          end
        end

        for index = #eventUtilState.allEventsWatchers, 1, -1 do
          local watcher = eventUtilState.allEventsWatchers[index]
          local haveAllEvents = true
          for eventIndex = 1, #watcher.events do
            if not eventUtilState.seenEvents[watcher.events[eventIndex]] then
              haveAllEvents = false
              break
            end
          end
          if haveAllEvents then
            table.remove(eventUtilState.allEventsWatchers, index)
            if type(watcher.callback) == "function" then
              watcher.callback()
            end
          end
        end
      end)
    end

    return dispatcher
  end

  local function ensure_event_registration(event)
    local frame = ensure_dispatcher()
    if frame == nil or eventUtilState.registeredEvents[event] then
      return
    end
    eventUtilState.registeredEvents[event] = true
    frame:RegisterEvent(event)
  end

  local function all_events_seen(events)
    for index = 1, #events do
      if not eventUtilState.seenEvents[events[index]] then
        return false
      end
    end
    return true
  end

  local function remove_once_watcher(handle)
    for index = #eventUtilState.onceWatchers, 1, -1 do
      if eventUtilState.onceWatchers[index].handle == handle then
        table.remove(eventUtilState.onceWatchers, index)
        break
      end
    end
  end

  EventUtil = {}

  function EventUtil.ContinueAfterAllEvents(callback, ...)
    local events = {}
    for index = 1, select("#", ...) do
      local event = select(index, ...)
      events[index] = event
      ensure_event_registration(event)
    end

    if all_events_seen(events) then
      if type(callback) == "function" then
        callback()
      end
      return
    end

    table.insert(eventUtilState.allEventsWatchers, {
      callback = callback,
      events = events,
    })
  end

  function EventUtil.AreVariablesLoaded()
    return eventUtilState.variablesLoadedTriggered
      or GlueParent
      or (type(UIParent) == "table" and UIParent.variablesLoaded)
  end

  function EventUtil.ContinueOnVariablesLoaded(callback)
    if EventUtil.AreVariablesLoaded() then
      if type(callback) == "function" then
        callback()
      end
      return
    end

    ensure_event_registration("VARIABLES_LOADED")
    table.insert(eventUtilState.variablesLoadedCallbacks, callback)
  end

  function EventUtil.TriggerOnVariablesLoaded()
    local frame = ensure_dispatcher()
    if frame ~= nil then
      frame:GetScript("OnEvent")(frame, "VARIABLES_LOADED")
    end
  end

  function EventUtil.ContinueOnAddOnLoaded(addOnName, callback)
    local isLoadedOrLoading, isLoaded = C_AddOns.IsAddOnLoaded(addOnName)
    if isLoaded then
      if type(callback) == "function" then
        callback()
      end
      return
    end

    EventUtil.RegisterOnceFrameEventAndCallback("ADDON_LOADED", callback, addOnName)
  end

  function EventUtil.ContinueOnPlayerLogin(callback)
    if IsLoggedIn() then
      if type(callback) == "function" then
        callback()
      end
      return
    end

    EventUtil.RegisterOnceFrameEventAndCallback("PLAYER_LOGIN", callback)
  end

  function EventUtil.RegisterOnceFrameEventAndCallback(frameEvent, callback, ...)
    ensure_event_registration(frameEvent)

    local handle = { registered = true }
    function handle:Unregister()
      if not self.registered then
        return
      end
      self.registered = false
      remove_once_watcher(self)
    end

    table.insert(eventUtilState.onceWatchers, {
      callback = callback,
      event = frameEvent,
      handle = handle,
      requiredArgs = __wow_pack_results(...),
    })

    return handle
  end

  CallbackHandleContainerMixin = CallbackHandleContainerMixin or {}

  function CallbackHandleContainerMixin:Init()
    self.handles = {}
  end

  function CallbackHandleContainerMixin:RegisterCallback(cbr, event, callback, owner)
    self:AddHandle(cbr:RegisterCallbackWithHandle(event, callback, owner))
  end

  function CallbackHandleContainerMixin:AddHandle(handle)
    table.insert(self.handles, handle)
  end

  function CallbackHandleContainerMixin:Unregister()
    for index = 1, #self.handles do
      local handle = self.handles[index]
      if handle and type(handle.Unregister) == "function" then
        handle:Unregister()
      end
    end
    self.handles = {}
  end

  function CallbackHandleContainerMixin:IsEmpty()
    return #self.handles == 0
  end

  function EventUtil.CreateCallbackHandleContainer()
    local cbrHandles = CreateFromMixins(CallbackHandleContainerMixin)
    cbrHandles:Init()
    return cbrHandles
  end
end

table = table or {}

if unpack == nil then
  function unpack(list, first, last)
    if type(list) ~= "table" then
      return nil
    end
    first = first or 1
    last = last or #list
    if first > last then
      return
    end
    return list[first], unpack(list, first + 1, last)
  end
end

if table.unpack == nil then
  table.unpack = unpack
end

format = format or string.format

SlashCmdList = SlashCmdList or {}

if table.wipe == nil then
  function table.wipe(tbl)
    if type(tbl) ~= "table" then
      return tbl
    end
    for key in pairs(tbl) do
      tbl[key] = nil
    end
    return tbl
  end
end

tWipe = tWipe or table.wipe

function hooksecurefunc(target, methodName, hook)
  local object = target
  local key = methodName
  local callback = hook

  if type(target) == "string" and type(methodName) == "function" and hook == nil then
    object = _G
    key = target
    callback = methodName
  end

  if type(object) ~= "table" or type(key) ~= "string" or type(callback) ~= "function" then
    return
  end

  local original = object[key]
  if type(original) ~= "function" then
    return
  end

  object[key] = function(...)
    local results = __wow_pack_results(original(...))
    callback(...)
    return unpack(results, 1, results.n)
  end
end

if getn == nil then
  function getn(tbl)
    if type(tbl) ~= "table" then
      return nil
    end
    return #tbl
  end
end

if table.getn == nil then
  table.getn = getn
end

if strtrim == nil then
  function strtrim(value)
    value = tostring(value or "")
    return (value:gsub("^%s+", ""):gsub("%s+$", ""))
  end
end

if Clamp == nil then
  function Clamp(value, min, max)
    if value > max then
      return max
    elseif value < min then
      return min
    end
    return value
  end
end

if Saturate == nil then
  function Saturate(value)
    return Clamp(value, 0.0, 1.0)
  end
end

if CooldownFrame_Set == nil then
  function CooldownFrame_Set(self, start, duration, enable, forceShowDrawEdge, modRate)
    if enable and enable ~= 0 and start > 0 and duration > 0 then
      self:SetDrawEdge(forceShowDrawEdge)
      self:SetCooldown(start, duration, modRate)
    else
      CooldownFrame_Clear(self)
    end
  end
end

if CooldownFrame_Clear == nil then
  function CooldownFrame_Clear(self)
    self:Clear()
    self:SetCooldown(0, 0)
  end
end

if CooldownFrame_SetDisplayAsPercentage == nil then
  function CooldownFrame_SetDisplayAsPercentage(self, percentage)
    local seconds = 100
    self:Pause()
    self:SetCooldown(GetTime() - (seconds * Saturate(percentage)), seconds)
  end
end

if FadingFrame_SetFadeInTime == nil then
  function FadingFrame_SetFadeInTime(fadingFrame, time)
    fadingFrame.fadeInTime = time
  end
end

if FadingFrame_SetHoldTime == nil then
  function FadingFrame_SetHoldTime(fadingFrame, time)
    fadingFrame.holdTime = time
  end
end

if FadingFrame_SetFadeOutTime == nil then
  function FadingFrame_SetFadeOutTime(fadingFrame, time)
    fadingFrame.fadeOutTime = time
  end
end

if FadingFrame_OnLoad == nil then
  function FadingFrame_OnLoad(fadingFrame)
    assert(fadingFrame)
    fadingFrame.fadeInTime = 0
    fadingFrame.holdTime = 0
    fadingFrame.fadeOutTime = 0
    fadingFrame:Hide()
  end
end

if FadingFrame_Show == nil then
  function FadingFrame_Show(fadingFrame)
    assert(fadingFrame)
    fadingFrame.startTime = GetTime()
    fadingFrame:Show()
  end
end

if FadingFrame_OnUpdate == nil then
  function FadingFrame_OnUpdate(fadingFrame)
    assert(fadingFrame)
    local elapsed = GetTime() - fadingFrame.startTime
    local fadeInTime = fadingFrame.fadeInTime
    if elapsed < fadeInTime then
      fadingFrame:SetAlpha(elapsed / fadeInTime)
      return
    end

    local holdTime = fadingFrame.holdTime
    if elapsed < (fadeInTime + holdTime) then
      fadingFrame:SetAlpha(1.0)
      return
    end

    local fadeOutTime = fadingFrame.fadeOutTime
    if elapsed < (fadeInTime + holdTime + fadeOutTime) then
      fadingFrame:SetAlpha(1.0 - ((elapsed - holdTime - fadeInTime) / fadeOutTime))
      return
    end

    fadingFrame:Hide()
  end
end

if FadingFrame_GetRemainingTime == nil then
  function FadingFrame_GetRemainingTime(fadingFrame)
    local elapsed = GetTime() - fadingFrame.startTime
    return fadingFrame.holdTime + fadingFrame.fadeInTime + fadingFrame.fadeOutTime - elapsed
  end
end

if FadingFrame_CopyTimes == nil then
  function FadingFrame_CopyTimes(src, dest)
    dest.fadeInTime = src.fadeInTime
    dest.holdTime = src.holdTime
    dest.fadeOutTime = src.fadeOutTime
    dest.startTime = src.startTime
  end
end

local function __wow_deep_copy_table(source, seen)
  if type(source) ~= "table" then
    return source
  end
  seen = seen or {}
  if seen[source] ~= nil then
    return seen[source]
  end
  local copy = {}
  seen[source] = copy
  for key, value in pairs(source) do
    copy[__wow_deep_copy_table(key, seen)] = __wow_deep_copy_table(value, seen)
  end
  local mt = getmetatable(source)
  if mt ~= nil then
    setmetatable(copy, __wow_deep_copy_table(mt, seen))
  end
  return copy
end

local function __wow_shallow_copy_table(source)
  if type(source) ~= "table" then
    return source
  end
  local copy = {}
  for key, value in pairs(source) do
    copy[key] = value
  end
  local mt = getmetatable(source)
  if mt ~= nil then
    setmetatable(copy, mt)
  end
  return copy
end

if CopyTable == nil then
  function CopyTable(source, shallow)
    if shallow then
      return __wow_shallow_copy_table(source)
    end
    return __wow_deep_copy_table(source)
  end
end

if TextureKitConstants == nil then
  TextureKitConstants = {
    SetVisibility = true,
    DoNotSetVisibility = false,
    UseAtlasSize = true,
    IgnoreAtlasSize = false,
    AddressModeClamp = 1,
    AddressModeWrap = 2,
    AddressModeAllowAssetToDetermine = 3,
  }
end

local __wow_lfg_role_icons = {
  GUIDE = "UI-LFG-RoleIcon-Leader",
  TANK = "UI-LFG-RoleIcon-Tank",
  HEALER = "UI-LFG-RoleIcon-Healer",
  DAMAGER = "UI-LFG-RoleIcon-DPS",
  NONE = "UI-LFG-RoleIcon-DPS",
}

local __wow_lfg_role_icons_disabled = {
  GUIDE = "UI-LFG-RoleIcon-Leader-Disabled",
  TANK = "UI-LFG-RoleIcon-Tank-Disabled",
  HEALER = "UI-LFG-RoleIcon-Healer-Disabled",
  DAMAGER = "UI-LFG-RoleIcon-DPS-Disabled",
  NONE = "UI-LFG-RoleIcon-DPS-Disabled",
}

if GetIconForRole == nil then
  function GetIconForRole(role, showDisabled)
    local iconSet = showDisabled and __wow_lfg_role_icons_disabled or __wow_lfg_role_icons
    return iconSet[role] or iconSet.NONE
  end
end

local function __wow_lfg_role_name_from_enum(role)
  if role == 0 then
    return "TANK"
  end
  if role == 1 then
    return "HEALER"
  end
  if role == 2 then
    return "DAMAGER"
  end
  if Constants ~= nil
      and Constants.LFG_ROLEConstants ~= nil
      and role == Constants.LFG_ROLEConstants.LFG_ROLE_NO_ROLE then
    return "GUIDE"
  end
  return "NONE"
end

if GetIconForRoleEnum == nil then
  function GetIconForRoleEnum(role, showDisabled)
    return GetIconForRole(__wow_lfg_role_name_from_enum(role), showDisabled)
  end
end

if GetFrameMetatable == nil then
  function GetFrameMetatable(frame)
    if frame == nil then
      if CreateFrame == nil then
        return nil
      end
      frame = CreateFrame("Frame")
    end
    return frame and getmetatable(frame) or nil
  end
end

if GetFontStringMetatable == nil then
  function GetFontStringMetatable(fontString)
    if fontString == nil then
      if CreateFrame == nil then
        return nil
      end
      local frame = CreateFrame("Frame")
      fontString = frame and frame:CreateFontString()
    end
    return fontString and getmetatable(fontString) or nil
  end
end

do
  local frameMeta = GetFrameMetatable and GetFrameMetatable()
  local frameIndex = frameMeta and frameMeta.__index
  if type(frameIndex) == "table" then
    if frameIndex.AddDataProvider == nil then
      function frameIndex:AddDataProvider(provider)
        local fields = debug.getfenv(self)
        if type(fields) ~= "table" then
          return
        end
        local store = fields[1]
        if type(store) ~= "table" then
          store = {}
          fields[1] = store
        end
        local providers = store.dataProviders
        if type(providers) ~= "table" then
          providers = {}
          store.dataProviders = providers
        end
        for i = 1, #providers do
          if providers[i] == provider then
            return
          end
        end
        providers[#providers + 1] = provider
      end
    end

    if frameIndex.RemoveDataProvider == nil then
      function frameIndex:RemoveDataProvider(provider)
        local fields = debug.getfenv(self)
        local providers = fields and fields[1] and fields[1].dataProviders
        if type(providers) ~= "table" then
          return
        end
        for i = #providers, 1, -1 do
          if providers[i] == provider then
            table.remove(providers, i)
          end
        end
      end
    end

    if frameIndex.IsInitialized == nil then
      function frameIndex:IsInitialized()
        return type(self.layoutInfo) == "table" or type(self.systemInfo) == "table"
      end
    end

    if frameIndex.IsInDefaultPosition == nil then
      function frameIndex:IsInDefaultPosition()
        local info = self.systemInfo
        return type(info) == "table" and info.isInDefaultPosition == true
      end
    end
  end
end

if IsTestBuild == nil then
  function IsTestBuild()
    return true
  end
end

local __wow_saved_account_name = ""
local __wow_saved_account_list = ""
local __wow_uses_token = false

if GetSavedAccountName == nil then
  function GetSavedAccountName()
    return __wow_saved_account_name
  end
end

if SetSavedAccountName == nil then
  function SetSavedAccountName(accountName)
    __wow_saved_account_name = accountName or ""
  end
end

if GetSavedAccountList == nil then
  function GetSavedAccountList()
    return __wow_saved_account_list
  end
end

if ClearSavedAccountList == nil then
  function ClearSavedAccountList()
    __wow_saved_account_list = ""
  end
end

if SetUsesToken == nil then
  function SetUsesToken(usesToken)
    __wow_uses_token = not not usesToken
  end
end

if WasScreenFirstDisplayed == nil then
  function WasScreenFirstDisplayed(_screenName)
    return false
  end
end

if GetLoginScreenBackground == nil then
  function GetLoginScreenBackground(highResBackground, lowResBackground)
    if highResBackground ~= nil then
      return highResBackground
    end
    return lowResBackground
  end
end

if GetMinimumExpansionLevel == nil then
  function GetMinimumExpansionLevel()
    return 0
  end
end

if GetServerName == nil then
  function GetServerName()
    return nil
  end
end

if IsConnectedToServer == nil then
  function IsConnectedToServer()
    return false
  end
end

local __wow_char_select_model_frame_name = nil
local __wow_char_select_map_scene_frame_name = nil
local __wow_character_screen_initialized = false

if SetWorldFrameStrata == nil then
  function SetWorldFrameStrata(frame)
    if type(frame) == "table" and type(frame.SetFrameStrata) == "function" then
      frame:SetFrameStrata("BACKGROUND")
    end
  end
end

if SetCharSelectModelFrame == nil then
  function SetCharSelectModelFrame(frameName)
    __wow_char_select_model_frame_name = frameName
  end
end

if SetCharSelectMapSceneFrame == nil then
  function SetCharSelectMapSceneFrame(frameName)
    __wow_char_select_map_scene_frame_name = frameName
  end
end

if InitializeCharacterScreenData == nil then
  function InitializeCharacterScreenData()
    __wow_character_screen_initialized = true
  end
end

if GetMaxWarbandGroupCount == nil then
  function GetMaxWarbandGroupCount()
    return 4
  end
end

if GetActiveTimerunningSeasonID == nil then
  function GetActiveTimerunningSeasonID()
    return nil
  end
end

if GetCharacterListUpdate == nil then
  local function __wow_character_select_event(frame, event, ...)
    if type(frame) == "table" and type(frame.OnEvent) == "function" then
      frame:OnEvent(event, ...)
    end
  end

  function GetCharacterListUpdate()
    local includeEmptySlots = true
    local listSize = type(GetNumCharacters) == "function" and GetNumCharacters(includeEmptySlots) or 0
    if type(CharacterSelect) == "table" then
      CharacterSelect.waitingforCharacterList = false
    end
    __wow_character_select_event(CharacterSelect, "CHARACTER_LIST_UPDATE", listSize)
    __wow_character_select_event(CharacterSelectCharacterFrame, "CHARACTER_LIST_UPDATE", listSize)

    local selectedCharacter = 0
    if type(GetCharacterSelection) == "function" then
      selectedCharacter = GetCharacterSelection() or 0
    elseif listSize > 0 then
      selectedCharacter = 1
    end
    __wow_character_select_event(CharacterSelect, "UPDATE_SELECTED_CHARACTER", selectedCharacter)
    if type(CharacterSelectCharacterFrame) == "table"
      and type(CharacterSelectCharacterFrame.UpdateCharacterSelection) == "function"
    then
      CharacterSelectCharacterFrame:UpdateCharacterSelection()
    end
  end
end

if PlayGlueMusic == nil then
  function PlayGlueMusic()
  end
end

if StopGlueMusic == nil then
  function StopGlueMusic()
  end
end

if PlayGlueAmbience == nil then
  function PlayGlueAmbience()
  end
end

if StopGlueAmbience == nil then
  function StopGlueAmbience()
  end
end

if HasCheckedSystemRequirements == nil
  or SetCheckedSystemRequirements == nil
  or CheckSystemRequirements == nil
then
  local __wow_checked_system_requirements = false

  if HasCheckedSystemRequirements == nil then
    function HasCheckedSystemRequirements()
      return __wow_checked_system_requirements
    end
  end

  if SetCheckedSystemRequirements == nil then
    function SetCheckedSystemRequirements(checked)
      __wow_checked_system_requirements = not not checked
    end
  end

  if CheckSystemRequirements == nil then
    function CheckSystemRequirements(includeSeenWarnings)
      if type(C_ConfigurationWarnings) ~= "table"
        or type(C_ConfigurationWarnings.GetConfigurationWarnings) ~= "function"
      then
        return
      end

      local warnings = C_ConfigurationWarnings.GetConfigurationWarnings(includeSeenWarnings)
      if type(warnings) ~= "table" then
        return
      end

      if type(StaticPopup_Queue) ~= "function" then
        return
      end

      for _, warning in ipairs(warnings) do
        local text = nil
        if type(C_ConfigurationWarnings.GetConfigurationWarningString) == "function" then
          text = C_ConfigurationWarnings.GetConfigurationWarningString(warning)
        end
        if text then
          StaticPopup_Queue("CONFIGURATION_WARNING", text, nil, { configurationWarning = warning })
        end
      end
    end
  end
end

if StoreFrame_WaitingForCharacterListUpdate == nil then
  function StoreFrame_WaitingForCharacterListUpdate()
    return false
  end
end

if UpdateSelectionCustomizationScene == nil then
  function UpdateSelectionCustomizationScene()
  end
end

if PlayerLocation == nil then
  PlayerLocation = {}

  local function create_player_location(kind, payload)
    local location = {
      kind = kind,
      payload = payload,
    }

    function location:Clear()
      self.kind = nil
      self.payload = nil
    end

    function location:IsGUID()
      return self.kind == "guid"
    end

    function location:IsUnit()
      return self.kind == "unit"
    end

    function location:IsCommunityData()
      return self.kind == "community"
    end

    function location:IsBattleNetID()
      return self.kind == "battle_net"
    end

    function location:GetGUID()
      if self.kind == "guid" and self.payload then
        return self.payload.guid
      end
      return nil
    end

    function location:IsValid()
      if self.kind == "guid" and self.payload then
        if type(C_AccountInfo) == "table"
          and type(C_AccountInfo.IsGUIDBattleNetAccountType) == "function"
          and C_AccountInfo.IsGUIDBattleNetAccountType(self.payload.guid)
        then
          return true
        end
        return type(C_PlayerInfo) == "table"
          and type(C_PlayerInfo.GUIDIsPlayer) == "function"
          and C_PlayerInfo.GUIDIsPlayer(self.payload.guid)
          or false
      elseif self.kind == "unit" and self.payload then
        return type(UnitIsHumanPlayer) == "function" and UnitIsHumanPlayer(self.payload.unit) or false
      elseif self.kind == "community" and self.payload then
        return type(C_Club) == "table"
          and type(C_Club.CanResolvePlayerLocationFromClubMessageData) == "function"
          and C_Club.CanResolvePlayerLocationFromClubMessageData(
            self.payload.clubID,
            self.payload.streamID,
            self.payload.epoch,
            self.payload.position
          )
          or false
      elseif self.kind == "battle_net" and self.payload then
        return self.payload.battleNetID ~= nil
      elseif self.kind == "voice" and self.payload then
        return self.payload.memberID ~= nil and self.payload.channelID ~= nil
      end
      return false
    end

    return location
  end

  function PlayerLocation:CreateFromGUID(guid)
    return create_player_location("guid", { guid = guid })
  end

  function PlayerLocation:CreateFromUnit(unit)
    return create_player_location("unit", { unit = unit })
  end

  function PlayerLocation:CreateFromCommunityChatData(clubID, streamID, epoch, position)
    return create_player_location("community", {
      clubID = clubID,
      streamID = streamID,
      epoch = epoch,
      position = position,
    })
  end

  function PlayerLocation:CreateFromBattleNetID(battleNetID)
    return create_player_location("battle_net", { battleNetID = battleNetID })
  end

  function PlayerLocation:CreateFromVoiceID(memberID, channelID)
    return create_player_location("voice", {
      memberID = memberID,
      channelID = channelID,
    })
  end
end
