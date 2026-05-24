local function __wow_noop()
end

local function __wow_ensure_named_frame(frameType, name, parent)
  local existing = rawget(_G, name)
  if existing ~= nil then
    return existing
  end
  if CreateFrame == nil then
    return nil
  end
  return CreateFrame(frameType or "Frame", name, parent)
end

local function __wow_ensure_named_child(parent, key, frameType, name)
  if parent == nil then
    return nil
  end
  local existing = rawget(parent, key)
  if existing ~= nil then
    return existing
  end
  local child = CreateFrame(frameType or "Frame", name, parent)
  rawset(parent, key, child)
  return child
end

local function __wow_install_frame_helpers(frame)
  if frame == nil then
    return nil
  end

  if frame.AddDataProvider == nil then
    function frame:AddDataProvider(provider)
      local env = debug and debug.getfenv and debug.getfenv(self)
      local fields = type(env) == "table" and env[1] or nil
      if type(fields) ~= "table" then
        fields = {}
        if type(env) == "table" then
          env[1] = fields
        else
          return
        end
      end
      local providers = fields.dataProviders
      if type(providers) ~= "table" then
        providers = {}
        fields.dataProviders = providers
      end
      for i = 1, #providers do
        if providers[i] == provider then
          return
        end
      end
      providers[#providers + 1] = provider
      if type(provider) == "table" and type(provider.OnAdded) == "function" then
        pcall(provider.OnAdded, provider, self)
      end
      if type(provider) == "table" and provider.pin ~= nil then
        provider.pin.dataProvider = provider
      end
      if type(provider) == "table" and provider.pin == nil then
        provider.pin = { dataProvider = provider }
      end
    end
  end

  if frame.RemoveDataProvider == nil then
    function frame:RemoveDataProvider(provider)
      local env = debug and debug.getfenv and debug.getfenv(self)
      local providers = type(env) == "table" and env[1] and env[1].dataProviders or nil
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

  if frame.IsInitialized == nil then
    function frame:IsInitialized()
      return type(self.layoutInfo) == "table" or type(self.systemInfo) == "table"
    end
  end

  if frame.IsInDefaultPosition == nil then
    function frame:IsInDefaultPosition()
      local info = self.systemInfo
      return type(info) == "table" and info.isInDefaultPosition == true
    end
  end

  return frame
end

local function __wow_ensure_startup_navigation_surface()
  local uiParent = rawget(_G, "UIParent")

  local function ensure_frame(name)
    local frame = rawget(_G, name)
    if frame == nil then
      frame = __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", name, uiParent))
      rawset(_G, name, frame)
    end
    return frame
  end

  local function set_frame_visibility(name, visible)
    local frame = ensure_frame(name)
    if type(frame.Show) == "function" and visible then
      frame:Show()
    elseif type(frame.Hide) == "function" and not visible then
      frame:Hide()
    else
      rawset(frame, "visible", visible and true or false)
    end
    return frame
  end

  local function toggle_single_frame(name, extraNames)
    local frame = ensure_frame(name)
    local isShown = type(frame.IsShown) == "function" and frame:IsShown()
    local newVisible = not isShown
    set_frame_visibility(name, newVisible)
    if type(extraNames) == "table" then
      for _, extraName in ipairs(extraNames) do
        set_frame_visibility(extraName, newVisible)
      end
    end
    return newVisible
  end

  for _, name in ipairs({
    "MainActionBar",
    "MultiBarBottomLeft",
    "MultiBarBottomRight",
    "MultiBarRight",
    "MultiBarLeft",
    "MailFrame",
    "InboxFrame",
    "PVEFrame",
  }) do
    local frame = ensure_frame(name)
    if rawget(frame, "MarkAllSettingsDirty") == nil then
      function frame:MarkAllSettingsDirty() end
    end
  end

  if rawget(_G, "ToggleMailFrame") == nil then
    function ToggleMailFrame()
      toggle_single_frame("MailFrame", { "InboxFrame" })
    end
  end

  if rawget(_G, "OpenAllBags") == nil then
    function OpenAllBags()
      set_frame_visibility("ContainerFrameCombinedBags", true)
    end
  end

  if rawget(_G, "ToggleLFDParentFrame") == nil then
    function ToggleLFDParentFrame()
      local toggle = rawget(_G, "PVEFrame_ToggleFrame")
      if type(toggle) == "function" and toggle ~= ToggleLFDParentFrame then
        return toggle()
      end
      return toggle_single_frame("PVEFrame")
    end
  end

  if rawget(_G, "UpdateRaidAndPartyFrames") == nil then
    function UpdateRaidAndPartyFrames()
      if PartyFrame and type(PartyFrame.UpdatePartyFrames) == "function" then
        pcall(PartyFrame.UpdatePartyFrames, PartyFrame)
      end
    end
  end

  if rawget(_G, "HelpOpenWebTicketButton_OnUpdate") == nil then
    function HelpOpenWebTicketButton_OnUpdate() end
  end

  if type(ContentTrackingUtil) ~= "table" then
    ContentTrackingUtil = {}
  end
  if rawget(ContentTrackingUtil, "IsTrackingModifierDown") == nil then
    function ContentTrackingUtil.IsTrackingModifierDown() return false end
  end
  if rawget(ContentTrackingUtil, "IsContentTrackingEnabled") == nil then
    function ContentTrackingUtil.IsContentTrackingEnabled() return false end
  end
  if rawget(ContentTrackingUtil, "RegisterTrackableElement") == nil then
    function ContentTrackingUtil.RegisterTrackableElement() end
  end
  if rawget(ContentTrackingUtil, "UnregisterTrackableElement") == nil then
    function ContentTrackingUtil.UnregisterTrackableElement() end
  end
  if rawget(ContentTrackingUtil, "ProcessChatLink") == nil then
    function ContentTrackingUtil.ProcessChatLink() return false end
  end
  if rawget(ContentTrackingUtil, "GetTrackingMapInfoByEncounterID") == nil then
    function ContentTrackingUtil.GetTrackingMapInfoByEncounterID() return nil end
  end
  if rawget(ContentTrackingUtil, "IsContentTrackedInEncounter") == nil then
    function ContentTrackingUtil.IsContentTrackedInEncounter() return false end
  end
  if rawget(ContentTrackingUtil, "OpenMapToTrackable") == nil then
    function ContentTrackingUtil.OpenMapToTrackable() return false end
  end
  if rawget(ContentTrackingUtil, "DisplayTrackingError") == nil then
    function ContentTrackingUtil.DisplayTrackingError() end
  end
  if rawget(ContentTrackingUtil, "MakeCombinedID") == nil then
    function ContentTrackingUtil.MakeCombinedID(trackableType, trackableID)
      return tostring(trackableType or 0) .. ":" .. tostring(trackableID or 0)
    end
  end
  if rawget(ContentTrackingUtil, "SplitCombinedID") == nil then
    function ContentTrackingUtil.SplitCombinedID(combinedID)
      if type(combinedID) ~= "string" then
        return nil, nil
      end
      local a, b = string.match(combinedID, "^(.-):(.-)$")
      return tonumber(a), tonumber(b)
    end
  end
end

__wow_ensure_startup_navigation_surface()

if abs == nil and math ~= nil then abs = math.abs end
if ceil == nil and math ~= nil then ceil = math.ceil end
if floor == nil and math ~= nil then floor = math.floor end
if max == nil and math ~= nil then max = math.max end
if min == nil and math ~= nil then min = math.min end
if strlen == nil and string ~= nil then strlen = string.len end
if sort == nil and table ~= nil then sort = table.sort end

if strsplittable == nil then
  function strsplittable(delimiter, input, limit)
    return { strsplit(delimiter, input, limit) }
  end
end

if MergeTable == nil then
  function MergeTable(dest, src)
    if type(dest) ~= "table" or type(src) ~= "table" then
      return dest
    end
    for key, value in pairs(src) do
      dest[key] = value
    end
    return dest
  end
end

if tFilter == nil then
  function tFilter(t, predicate)
    if type(t) ~= "table" or type(predicate) ~= "function" then
      return t
    end
    local out = 1
    local len = #t
    for i = 1, len do
      local value = t[i]
      if value ~= nil and predicate(value, i, t) then
        if out ~= i then
          t[out] = value
        end
        out = out + 1
      end
    end
    for i = out, len do
      t[i] = nil
    end
    return t
  end
end

local function __wow_ensure_item_button_surface(button)
  if type(button) ~= "table" then
    return button
  end

  local icon = rawget(button, "icon")
  if icon == nil and type(button.CreateTexture) == "function" then
    icon = button:CreateTexture(nil, "BORDER")
    button.icon = icon
  end
  if icon ~= nil then
    if type(icon.SetParentKey) == "function" then
      pcall(icon.SetParentKey, icon, "icon", true)
    end
    if type(icon.ClearAllPoints) == "function" then
      icon:ClearAllPoints()
    end
    if type(icon.SetPoint) == "function" then
      icon:SetPoint("TOPLEFT", button, "TOPLEFT")
      icon:SetPoint("BOTTOMRIGHT", button, "BOTTOMRIGHT")
    end
  end

  local border = rawget(button, "IconBorder")
  if border == nil and type(button.CreateTexture) == "function" then
    border = button:CreateTexture(nil, "OVERLAY")
    button.IconBorder = border
  end
  if border ~= nil then
    if type(border.SetParentKey) == "function" then
      pcall(border.SetParentKey, border, "IconBorder", true)
    end
    if type(border.SetSize) == "function" then
      border:SetSize(37, 37)
    end
    if type(border.ClearAllPoints) == "function" then
      border:ClearAllPoints()
    end
    if type(border.SetPoint) == "function" then
      border:SetPoint("CENTER", button, "CENTER")
    end
  end

  return button
end

if CreateFrame ~= nil and __wow_original_CreateFrame == nil then
  __wow_original_CreateFrame = CreateFrame

  function CreateFrame(...)
    local frameType = select(1, ...)
    local inherits = select(4, ...)
    if type(inherits) == "string" then
      if string.find(inherits, "MapCanvasFrameTemplate", 1, true) or
         string.find(inherits, "MapCanvasFrameScrollContainerTemplate", 1, true) then
        __wow_patch_map_canvas_scroll_container_methods()
      end
    end
    local created = __wow_install_frame_helpers(__wow_original_CreateFrame(...))
    if frameType == "GameTooltip" and created and created.SetFrameStrata ~= nil then
      created:SetFrameStrata("TOOLTIP")
    end
    if frameType == "ItemButton" then
      created = __wow_ensure_item_button_surface(created)
    end
    local parent = select(3, ...)
    if type(parent) == "table" and type(inherits) == "string" then
      if string.find(inherits, "MapCanvasFrameScrollContainerTemplate", 1, true) then
        rawset(parent, "ScrollContainer", created)
      end
    end
    return created
  end
end

do
  local frameMeta = GetFrameMetatable and GetFrameMetatable()
  local frameIndex = frameMeta and frameMeta.__index
  if type(frameIndex) == "table" then
    if frameIndex.AddDataProvider == nil then
      function frameIndex:AddDataProvider(provider)
        local fields = debug.getfenv(self)
        local store = fields and fields[1]
        if type(store) ~= "table" then
          return
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

ChatFrameUtil = ChatFrameUtil or {}
if ChatFrameUtil.ProcessMessageEventFilters == nil then
  function ChatFrameUtil.ProcessMessageEventFilters(_frame, event, ...)
    return false, event, ...
  end
end
if ChatFrameUtil.GetChatWindowName == nil then
  function ChatFrameUtil.GetChatWindowName(index)
    return string.format("Chat Window %d", tonumber(index) or 1)
  end
end
if ChatFrameUtil.GetCommunitiesChannelColor == nil then
  function ChatFrameUtil.GetCommunitiesChannelColor(_clubId, streamId)
    if tonumber(streamId) == 2 then
      return 0.25, 0.75, 0.25
    end
    return 0.25, 1, 0.25
  end
end
if ChatFrameUtil.GetCommunitiesChannelLocalID == nil then
  function ChatFrameUtil.GetCommunitiesChannelLocalID(_clubId, _streamId)
    return nil
  end
end

ChatTypeGroup = ChatTypeGroup or {
  SYSTEM = { "SYSTEM", "ERRORS", "IGNORED", "ACHIEVEMENT", "CHANNEL_NOTICE_USER" },
  SAY = { "SAY" },
  YELL = { "YELL" },
  WHISPER = { "WHISPER", "WHISPER_INFORM" },
  PARTY = { "PARTY", "PARTY_LEADER" },
  RAID = { "RAID", "RAID_LEADER", "RAID_WARNING" },
  GUILD = { "GUILD", "OFFICER" },
  CHANNEL = { "CHANNEL", "CHANNEL_JOIN", "CHANNEL_LEAVE" },
  EMOTE = { "EMOTE" },
  BN_WHISPER = { "BN_WHISPER", "BN_WHISPER_INFORM", "BN_INLINE_TOAST_ALERT" },
  INSTANCE_CHAT = { "INSTANCE_CHAT", "INSTANCE_CHAT_LEADER" },
}

do
  local uiParent = UIParent
  __wow_install_frame_helpers(uiParent)
  local settingsPanel = __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "SettingsPanel", uiParent))
  local settingsContainer = __wow_ensure_named_child(settingsPanel, "Container", "Frame")
  local settingsList = __wow_ensure_named_child(settingsContainer, "SettingsList", "Frame")
  local scrollBox = __wow_ensure_named_child(settingsList, "ScrollBox", "Frame")
  __wow_ensure_named_child(scrollBox, "ScrollTarget", "Frame")
  local header = __wow_ensure_named_child(settingsList, "Header", "Frame")
  if header ~= nil and rawget(header, "Title") == nil and header.CreateFontString ~= nil then
    local title = header:CreateFontString(nil, "OVERLAY")
    title:SetText("")
    rawset(header, "Title", title)
  end

  local function __wow_seed_settings_preview(parent, key)
    local preview = __wow_install_frame_helpers(__wow_ensure_named_child(parent, key, "Frame"))
    if rawget(preview, "RegisterWithSettingInitializer") == nil then
      function preview:RegisterWithSettingInitializer(_initializer)
      end
    end
    if rawget(preview, "SetValueAccessor") == nil then
      function preview:SetValueAccessor(_getValue)
      end
    end
    if rawget(preview, "UpdatePreview") == nil then
      function preview:UpdatePreview(_value)
      end
    end
    return preview
  end

  __wow_seed_settings_preview(settingsPanel, "AccessibilityFontPreview")
  __wow_seed_settings_preview(settingsPanel, "QuestTextPreview")

  local objectiveTracker = __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "ObjectiveTrackerFrame", uiParent))
  if objectiveTracker ~= nil and rawget(objectiveTracker, "OnAdded") == nil then
    function objectiveTracker:OnAdded(backgroundAlpha)
      if not self.init then
        self.init = true
        if type(ObjectiveTrackerContainerMixin) == "table" and type(ObjectiveTrackerContainerMixin.Init) == "function" then
          ObjectiveTrackerContainerMixin.Init(self)
        elseif self.Header and self.Header.Text and type(self.Header.Text.SetText) == "function" then
          self.Header.Text:SetText(self.headerText or "")
        end
      end
      if type(self.SetBackgroundAlpha) == "function" then
        self:SetBackgroundAlpha(backgroundAlpha)
      end
    end
  end
  local objectiveHeader = __wow_ensure_named_child(objectiveTracker, "Header", "Frame")
  __wow_ensure_named_child(objectiveHeader, "MinimizeButton", "Button")

  local lfgListFrame = __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "LFGListFrame", uiParent))
  local searchPanel = __wow_ensure_named_child(lfgListFrame, "SearchPanel", "Frame")
  __wow_ensure_named_child(searchPanel, "SearchBox", "EditBox")

  local buffFrame = __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "BuffFrame", uiParent))
  local auraContainer = __wow_ensure_named_child(buffFrame, "AuraContainer", "Frame")
  if auraContainer ~= nil and auraContainer.iconScale == nil then
    auraContainer.iconScale = 1.0
  end

  local addonCompartmentFrame = __wow_install_frame_helpers(__wow_ensure_named_frame("Button", "AddonCompartmentFrame", uiParent))
  if addonCompartmentFrame ~= nil then
    addonCompartmentFrame.registeredAddons = addonCompartmentFrame.registeredAddons or {}
    if addonCompartmentFrame.RegisterAddon == nil then
      function addonCompartmentFrame:RegisterAddon(addon)
        self.registeredAddons[#self.registeredAddons + 1] = addon or true
      end
    end
    if addonCompartmentFrame.UnregisterAddon == nil then
      function addonCompartmentFrame:UnregisterAddon()
        table.remove(self.registeredAddons)
      end
    end
  end

  local alertFrame = __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "AlertFrame", uiParent))
  if alertFrame ~= nil then
    alertFrame.alertFrameSubSystems = alertFrame.alertFrameSubSystems or {}
    if alertFrame.AddQueuedAlertFrameSubSystem == nil then
      function alertFrame:AddQueuedAlertFrameSubSystem(templateName, factory, _maxVisible, anchorSlot)
        local subsystem = {
          templateName = templateName,
          factory = factory,
          anchorPriority = 1000 + math.max(0, (tonumber(anchorSlot) or 1) - 1) * 10,
          queuedAlerts = {},
          canShowMoreCondition = nil,
        }
        function subsystem:SetCanShowMoreConditionFunc(func)
          self.canShowMoreCondition = func
        end
        function subsystem:AddAlert(alert)
          if self.canShowMoreCondition ~= nil and not self.canShowMoreCondition() and #self.queuedAlerts >= 2 then
            return false
          end
          self.queuedAlerts[#self.queuedAlerts + 1] = alert
          return true
        end
        function subsystem:RemoveAlert(alert)
          for i = #self.queuedAlerts, 1, -1 do
            if self.queuedAlerts[i] == alert then
              table.remove(self.queuedAlerts, i)
            end
          end
        end
        function subsystem:ClearAllAlerts()
          self.queuedAlerts = {}
        end
        self.alertFrameSubSystems[#self.alertFrameSubSystems + 1] = subsystem
        return subsystem
      end
    end
  end

  PartyMemberFramePool = PartyMemberFramePool or {
    EnumerateActive = function()
      return function()
        return nil
      end
    end,
    GetNumActive = function()
      return 0
    end,
  }
  if PartyFrame ~= nil and PartyFrame.PartyMemberFramePool == nil then
    PartyFrame.PartyMemberFramePool = PartyMemberFramePool
  end

  ContainerFrameContainer = ContainerFrameContainer or { ContainerFrames = {} }
  ChatFrame1 = ChatFrame1 or __wow_install_frame_helpers(__wow_ensure_named_frame("ScrollingMessageFrame", "ChatFrame1", uiParent))
  EventToastManagerFrame = EventToastManagerFrame or __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "EventToastManagerFrame", uiParent))
  EditModeManagerFrame = EditModeManagerFrame or __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "EditModeManagerFrame", uiParent))
  RolePollPopup = RolePollPopup or __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "RolePollPopup", uiParent))
  TimerTracker = TimerTracker or __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "TimerTracker", uiParent))
  UIErrorsFrame = UIErrorsFrame or __wow_install_frame_helpers(__wow_ensure_named_frame("MessageFrame", "UIErrorsFrame", uiParent))
  SideDressUpFrame = SideDressUpFrame or __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "SideDressUpFrame", uiParent))
  ContainerFrameCombinedBags = ContainerFrameCombinedBags or __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "ContainerFrameCombinedBags", uiParent))
  LootFrame = LootFrame or __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "LootFrame", uiParent))
  GossipFrame = GossipFrame or __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "GossipFrame", uiParent))
  FriendsFrame = FriendsFrame or __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "FriendsFrame", uiParent))
end

if GetChannelName == nil then
  function GetChannelName(channel)
    if type(channel) == "number" then
      return 0, nil, 0, false
    end
    if type(channel) == "string" then
      return 0, channel, 0, false
    end
    return 0, nil, 0, false
  end
end

if GetActionInfo == nil then
  function GetActionInfo()
    return nil, nil, nil
  end
end

-- GetInventorySlotInfo is registered from Rust
-- (src/lua_api/globals/inventory_slot.rs). Returns the canonical
-- (slotId, textureFileID, checkRelic) triple; case-insensitive on the
-- slot-name key.

-- C_PvP.GetZonePVPInfo is registered from Rust
-- (src/lua_api/globals/zone_text.rs) — reads SimState::world.pvp_type /
-- .is_sub_zone_pvp / .pvp_faction_name. Admin: A_Admin.SetZonePVP.

-- GetZoneText / GetSubZoneText / GetMinimapZoneText / GetRealZoneText are
-- registered from Rust (src/lua_api/globals/zone_text.rs), backed by
-- SimState::world. Tests drive the values via A_Admin.SetZone / SetSubZone
-- / SetInstanceInfo.
-- IsShiftKeyDown / IsControlKeyDown / IsAltKeyDown / IsMetaKeyDown /
-- IsModifierKeyDown are registered from Rust
-- (src/lua_api/globals/modifier_keys.rs), backed by SimState::modifier_keys.
-- Admin: A_Admin.SetShiftKeyDown(b) / SetControlKeyDown / SetAltKeyDown /
-- SetMetaKeyDown toggle individual keys.

-- GetGuildLogoInfo is registered from Rust (src/lua_api/globals/guild_logo.rs),
-- backed by SimState::world.guild_logo. Admin: A_Admin.SetGuildEmblem(filename,
-- bkgR, bkgG, bkgB, borderR, borderG, borderB, emblemR, emblemG, emblemB) —
-- all args optional, missing = 0 or "".

-- GetNetStats is registered from Rust (src/lua_api/globals/rilua_net_stats.rs)
-- and reads from SimState::net_stats so tests can inject values via
-- A_Admin.SetNetStats(bandwidthIn, bandwidthOut, latencyHome, latencyWorld).

-- StoreFrame_IsShown is registered from Rust (src/lua_api/globals/rilua_store_frame.rs)
-- and reads from SimState::store_frame_shown so tests can toggle it via
-- A_Admin.SetStoreFrameShown(true) to exercise MainMenuBarMicroButtons'
-- pushed-state rendering for the Store micro-button.

-- UnitIsPlayer is registered from Rust (src/lua_api/globals/rilua_unit_probes.rs).
-- It resolves tokens against SimState: "player"/"self" always true, "target"/
-- "focus" read the respective TargetInfo.is_player flag, "partyN" (N=1..4)
-- checks sim.party_members[N-1] is populated, everything else false.

if UnitIsHumanPlayer == nil then
  function UnitIsHumanPlayer(unit)
    if UnitIsPlayer ~= nil then
      return UnitIsPlayer(unit)
    end
    return false
  end
end

if IsTargetLoose == nil then
  function IsTargetLoose()
    return false
  end
end

if UnitThreatSituation == nil then
  function UnitThreatSituation(_unit, _other_unit)
    return 0
  end
end

if UnitDetailedThreatSituation == nil then
  function UnitDetailedThreatSituation(_unit, _other_unit)
    return false, 0, 0, 0, 0
  end
end

if UnitThreatPercentageOfLead == nil then
  function UnitThreatPercentageOfLead(_unit, _other_unit)
    return 0
  end
end

if SetPortraitTexture == nil then
  function SetPortraitTexture(texture, unit, _disablePortraitMask)
    if not texture then
      return
    end

    if UnitIsPlayer ~= nil and UnitIsPlayer(unit) then
      local _, classFile = UnitClass(unit)
      if classFile then
        local coords = CLASS_ICON_TCOORDS and CLASS_ICON_TCOORDS[classFile]
        if coords and texture.SetTexture and texture.SetTexCoord then
          texture:SetTexture("Interface\\TargetingFrame\\UI-Classes-Circles")
          texture:SetTexCoord(unpack(coords))
          return
        end

        local atlas = GetClassAtlas and GetClassAtlas(classFile)
        if atlas and texture.SetAtlas then
          texture:SetAtlas(atlas)
          return
        end
      end
    end

    if texture.SetTexture then
      texture:SetTexture("Interface\\ICONS\\INV_Misc_QuestionMark")
    end
  end
end

if SetPortraitTextureFromCreatureDisplayID == nil then
  function SetPortraitTextureFromCreatureDisplayID(texture, _creatureDisplayID)
    if texture and texture.SetTexture then
      texture:SetTexture("Interface\\ICONS\\INV_Misc_QuestionMark")
    end
  end
end


if LE_TOKEN_REDEEM_TYPE_GAME_TIME == nil then
  LE_TOKEN_REDEEM_TYPE_GAME_TIME = 1
end

if LE_TOKEN_REDEEM_TYPE_BALANCE == nil then
  LE_TOKEN_REDEEM_TYPE_BALANCE = 2
end

if LE_TOKEN_RESULT_ERROR_BALANCE_NEAR_CAP == nil then
  LE_TOKEN_RESULT_ERROR_BALANCE_NEAR_CAP = 10
end

if IsTrialAccount == nil then
  function IsTrialAccount()
    return false
  end
end

if IsRestrictedAccount == nil then
  function IsRestrictedAccount()
    return false
  end
end

if IsTutorialFlagged == nil then
  function IsTutorialFlagged()
    return false
  end
end

if GetFileStreamingStatus == nil then
  function GetFileStreamingStatus()
    return 0
  end
end

if GetNumArenaOpponentSpecs == nil then
  function GetNumArenaOpponentSpecs()
    return 0
  end
end

if GetErrorCallstackHeight == nil then
  function GetErrorCallstackHeight()
    return 0
  end
end

if GetBackgroundLoadingStatus == nil then
  function GetBackgroundLoadingStatus()
    return 0
  end
end

if GetWebTicket == nil then
  function GetWebTicket()
    return nil
  end
end

if GetDungeonDifficultyID == nil then
  function GetDungeonDifficultyID()
    return 1
  end
end

if UnitInVehicle == nil then
  function UnitInVehicle()
    return false
  end
end

function UnitHasVehiclePlayerFrameUI()
  return false
end

if UnitGetAvailableRoles == nil then
  function UnitGetAvailableRoles()
    return true, true, true
  end
end

if debugstack == nil then
  local function debugstack_source(info)
    local source = info and info.source or nil
    if type(source) ~= "string" or source == "" then
      source = info and info.short_src or "?"
    end
    if source:sub(1, 1) == "@" then
      return "[" .. source:sub(2) .. "]"
    end
    return source
  end

  local function debugstack_line(info)
    local source = debugstack_source(info)
    local currentline = tonumber(info and info.currentline) or -1
    if currentline > 0 then
      source = source .. ":" .. currentline
    else
      source = source .. ":"
    end

    if info and type(info.name) == "string" and info.name ~= "" then
      return source .. ": in function '" .. info.name .. "'"
    end
    if info and info.what == "main" then
      return source .. ": in main chunk"
    end
    if info and type(info.linedefined) == "number" and info.linedefined > 0 then
      return source .. ": in function <" .. debugstack_source(info) .. ":" .. info.linedefined .. ">"
    end
    return source .. " ?"
  end

  function debugstack(level, count1, count2)
    if not debug or not debug.getinfo then
      return ""
    end
    local start = (tonumber(level) or 1) + 1
    local lines = {}
    local depth = start
    while true do
      local info = debug.getinfo(depth, "Sln")
      if not info then break end
      lines[#lines + 1] = debugstack_line(info)
      depth = depth + 1
    end

    if count1 or count2 then
      local top = tonumber(count1) or 12
      local bottom = tonumber(count2) or 10
      if #lines > top + bottom then
        local kept = {}
        for i = 1, top do kept[#kept + 1] = lines[i] end
        kept[#kept + 1] = "..."
        for i = #lines - bottom + 1, #lines do kept[#kept + 1] = lines[i] end
        return table.concat(kept, "\n") .. "\n"
      end
    end
    local stack = table.concat(lines, "\n")
    if stack ~= "" then stack = stack .. "\n" end
    return stack
  end
end

if debuglocals == nil then
  function debuglocals(level)
    if not debug or not debug.getinfo or not debug.getlocal then
      return ""
    end
    local start = (tonumber(level) or 1) + 1
    local info = debug.getinfo(start, "fS")
    if not info then return "" end
    local parts = {}
    local i = 1
    while true do
      local name, value = debug.getlocal(start, i)
      if not name then break end
      if not name:match("^%(") then
        parts[#parts + 1] = string.format("%s = %s", name, tostring(value))
      end
      i = i + 1
    end
    return table.concat(parts, "\n")
  end
end

if issecure == nil then
  function issecure()
    return not (debug and debug.getstacktaint and debug.getstacktaint())
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

local __wow_namespace_names = setmetatable({}, { __mode = "k" })
local __wow_logged_nil_symbols = {}
local __wow_namespace_mt = {
  __index = function(t, key)
    __wow_log_nil_symbol_access(__wow_namespace_names[t], key)
    local fn = function()
      return nil
    end
    rawset(t, key, fn)
    return fn
  end,
}

function __wow_log_nil_symbol_access(container, key)
  if type(__wow_record_nil_symbol_access) ~= "function" then
    return
  end

  if __wow_logged_nil_symbols == nil then
    __wow_logged_nil_symbols = {}
  end
  local cacheKey = tostring(container) .. "\001" .. tostring(key)
  if __wow_logged_nil_symbols[cacheKey] then
    return
  end
  __wow_logged_nil_symbols[cacheKey] = true

  local source
  local line
  for level = 2, 8 do
    local info = debug.getinfo(level, "Sl")
    if info ~= nil and type(info.source) == "string" and info.source:sub(1, 1) == "@" then
      if info.source:find("runtime_surface_bootstrap.lua", 1, true) == nil then
        source = info.source
        line = info.currentline
        break
      end
    end
  end

  __wow_record_nil_symbol_access(container, key, source, line)
end

local function __wow_attach_namespace_name(namespace, name)
  if type(namespace) ~= "table" then
    return namespace
  end
  if name ~= nil then
    __wow_namespace_names[namespace] = name
  end
  local mt = getmetatable(namespace)
  if mt == nil then
    setmetatable(namespace, __wow_namespace_mt)
  elseif mt.__index == nil then
    setmetatable(namespace, __wow_namespace_mt)
  end
  return namespace
end

local function __wow_namespace(defaults)
  return __wow_attach_namespace_name(defaults or {})
end

local function __wow_merge_namespace(existing, defaults)
  local namespace = type(existing) == "table" and existing or {}
  for key, value in pairs(defaults or {}) do
    if rawget(namespace, key) == nil then
      rawset(namespace, key, value)
    end
  end
  local mt = getmetatable(namespace)
  if mt == nil or mt.__index == nil then
    setmetatable(namespace, getmetatable(__wow_namespace()))
  end
  return namespace
end

local function __wow_seed_namespace_names()
  for key, value in pairs(_G) do
    if type(key) == "string" and key:match("^C_[A-Za-z0-9_]+$") and type(value) == "table" then
      __wow_attach_namespace_name(value, key)
    end
  end
end

local function __wow_copy_table(source)
  local copy = {}
  for key, value in pairs(source or {}) do
    copy[key] = value
  end
  return copy
end

local function __wow_make_calendar_time(dayOffset, minuteOffset)
  local day = 14 + (tonumber(dayOffset) or 0)
  local totalMinutes = (12 * 60) + (tonumber(minuteOffset) or 0)
  local hour = math.floor(totalMinutes / 60)
  local minute = totalMinutes % 60
  while minute < 0 do
    minute = minute + 60
    hour = hour - 1
  end
  while minute >= 60 do
    minute = minute - 60
    hour = hour + 1
  end
  while hour < 0 do
    hour = hour + 24
    day = day - 1
  end
  while hour >= 24 do
    hour = hour - 24
    day = day + 1
  end
  return {
    year = 2026,
    month = 4,
    monthDay = day,
    weekday = 3,
    hour = hour,
    minute = minute,
  }
end

-- C_LFGList is state-backed via `src/lua_api/globals/lfg_list.rs`.
-- C_AddOnProfiler is state-backed via `src/c_api/c_addon_profiler.rs`.
-- C_Ping.GetDefaultPingOptions is a temporary shim in `src/c_api/temporary_shims/c_ping.rs`.
-- C_ZoneAbility is state-backed via `src/lua_api/globals/missing_surface/zone_ability.rs`.

if AuraUtil == nil then
  AuraUtil = {}
end

if AuraUtil.AuraFilters == nil then
  AuraUtil.AuraFilters = {
    Helpful = "HELPFUL",
    Harmful = "HARMFUL",
    Raid = "RAID",
    IncludeNameplateOnly = "INCLUDE_NAME_PLATE_ONLY",
  }
end

if AuraUtil.CreateFilterString == nil then
  function AuraUtil.CreateFilterString(...)
    local filters = {}
    for i = 1, select("#", ...) do
      local value = select(i, ...)
      if type(value) == "string" and value ~= "" then
        filters[#filters + 1] = value
      end
    end
    return table.concat(filters, "|")
  end
end

if AuraUtil.UnpackAuraData == nil then
  function AuraUtil.UnpackAuraData(aura)
    if aura == nil then
      return nil
    end
    return aura.name,
      aura.icon,
      aura.applications,
      aura.dispelName,
      aura.duration,
      aura.expirationTime,
      aura.sourceUnit,
      aura.isStealable,
      nil,
      aura.spellId
  end
end

if AuraUtil.ForEachAura == nil then
  function AuraUtil.ForEachAura(unitToken, filter, maxCount, callback)
    local seen = 0
    local token
    repeat
      local results = { C_UnitAuras.GetAuraSlots(unitToken, filter, maxCount, token) }
      token = results[1]
      for i = 2, #results do
        local aura = C_UnitAuras.GetAuraDataBySlot(unitToken, results[i])
        if aura ~= nil then
          seen = seen + 1
          if callback ~= nil and callback(aura) then
            return
          end
          if maxCount ~= nil and seen >= maxCount then
            return
          end
        end
      end
    until token == nil
  end
end

if AuraUtil.FindAura == nil then
  function AuraUtil.FindAura(predicate, unitToken, filter, maxCount)
    local found = nil
    AuraUtil.ForEachAura(unitToken, filter, maxCount, function(aura)
      if predicate ~= nil and predicate(aura) then
        found = aura
        return true
      end
      return false
    end)
    return found
  end
end

if AuraUtil.FindAuraByName == nil then
  function AuraUtil.FindAuraByName(name, unitToken, filter)
    return AuraUtil.FindAura(function(aura)
      return aura ~= nil and aura.name == name
    end, unitToken, filter)
  end
end

if AuraUtil.GetAuraDataByAuraInstanceID == nil then
  function AuraUtil.GetAuraDataByAuraInstanceID(unitToken, auraInstanceID)
    if C_UnitAuras._providerSwitched then
      return nil
    end
    return C_UnitAuras.GetAuraDataByAuraInstanceID(unitToken, auraInstanceID)
  end
end

if GetPlayerAuraBySpellID == nil then
  function GetPlayerAuraBySpellID(spellID)
    return C_UnitAuras.GetPlayerAuraBySpellID(spellID)
  end
end

if UnitBuff == nil then
  function UnitBuff(unitToken, index)
    local aura = C_UnitAuras.GetBuffDataByIndex(unitToken, index)
    return AuraUtil.UnpackAuraData(aura)
  end
end

if UnitDebuff == nil then
  function UnitDebuff(unitToken, index)
    local aura = C_UnitAuras.GetDebuffDataByIndex(unitToken, index)
    return AuraUtil.UnpackAuraData(aura)
  end
end

if UnitAura == nil then
  function UnitAura(unitToken, index, filter)
    local aura = C_UnitAuras.GetAuraDataByIndex(unitToken, index, filter)
    return AuraUtil.UnpackAuraData(aura)
  end
end

if GetContainerNumSlots == nil and C_Container ~= nil then
  function GetContainerNumSlots(...)
    return C_Container.GetContainerNumSlots(...)
  end
end

if GetContainerItemID == nil and C_Container ~= nil then
  function GetContainerItemID(...)
    return C_Container.GetContainerItemID(...)
  end
end

if GetContainerItemLink == nil and C_Container ~= nil then
  function GetContainerItemLink(...)
    return C_Container.GetContainerItemLink(...)
  end
end

if GetItemID == nil then
  local function __wow_extract_item_id(value)
    if value == nil then
      return nil
    end
    if type(value) == "number" then
      return value > 0 and math.floor(value) or nil
    end
    if type(value) ~= "string" then
      return nil
    end

    local link_id = value:match("|Hitem:(%d+)")
    if link_id ~= nil then
      return tonumber(link_id)
    end

    local raw_id = value:match("^item:(%d+)")
    if raw_id ~= nil then
      return tonumber(raw_id)
    end

    local numeric = tonumber(value)
    if numeric ~= nil and numeric > 0 then
      return math.floor(numeric)
    end

    return nil
  end

  function GetItemID(itemInfo)
    return __wow_extract_item_id(itemInfo)
  end
end

if GetTradeSkillTexture == nil and C_TradeSkillUI ~= nil then
  function GetTradeSkillTexture(...)
    return C_TradeSkillUI.GetTradeSkillTexture(...)
  end
end

if IsArtifactRelicItem == nil and C_ItemSocketInfo ~= nil then
  function IsArtifactRelicItem(...)
    return C_ItemSocketInfo.IsArtifactRelicItem(...)
  end
end

if CombatLogGetCurrentEntry == nil then
  local __wow_combat_log_state = {
    currentEntry = 0,
    numEntries = 0,
    retentionTime = 300,
    filteredEventsEnabled = false,
    messageLimit = 300,
    entries = {},
    currentIndex = nil,
    createdMessages = {},
  }

  local function __wow_combat_log_entries()
    return __wow_combat_log_state.entries
  end

  local function __wow_combat_log_latest_entry()
    local entries = __wow_combat_log_entries()
    if type(entries) ~= "table" or #entries == 0 then
      return nil
    end
    local index = __wow_combat_log_state.currentIndex
    if type(index) ~= "number" or index < 1 or index > #entries then
      index = #entries
    end
    return entries[index]
  end

  local function __wow_combat_log_set_entry_count(count)
    __wow_combat_log_state.numEntries = math.max(0, tonumber(count) or 0)
  end

  local function __wow_combat_log_object_is_a(objectType, mask)
    local object = math.max(0, tonumber(objectType) or 0)
    local filter = math.max(0, tonumber(mask) or 0)
    while object > 0 and filter > 0 do
      if object % 2 == 1 and filter % 2 == 1 then
        return true
      end
      object = math.floor(object / 2)
      filter = math.floor(filter / 2)
    end
    return false
  end

  function CombatLogAddFilter(_filter)
    return true
  end

  function CombatLogResetFilter()
    __wow_combat_log_state.filteredEventsEnabled = false
    return true
  end

  function CombatLogAdvanceEntry(step)
    local amount = tonumber(step) or 0
    __wow_combat_log_state.currentEntry =
      math.max(0, __wow_combat_log_state.currentEntry + amount)
    return true
  end

  function CombatLogGetCurrentEntry()
    return __wow_combat_log_state.currentEntry
  end

  function CombatLogGetCurrentEventInfo()
    local entry = __wow_combat_log_latest_entry()
    if entry == nil then
      return nil
    end
    return unpack(entry)
  end

  function CombatLogGetNumEntries()
    local entries = __wow_combat_log_entries()
    if type(entries) == "table" then
      return #entries
    end
    return __wow_combat_log_state.numEntries
  end

  function CombatLogSetCurrentEntry(entry)
    __wow_combat_log_state.currentEntry = math.max(0, tonumber(entry) or 0)
  end

  function CombatLogShowCurrentEntry()
    local entries = __wow_combat_log_entries()
    if type(entries) == "table" and #entries > 0 then
      return true
    end
    return __wow_combat_log_state.currentEntry > 0
  end

  function CombatLogClearEntries()
    __wow_combat_log_state.entries = {}
    __wow_combat_log_state.currentIndex = nil
    __wow_combat_log_state.currentEntry = 0
    __wow_combat_log_set_entry_count(0)
    return true
  end

  function CombatLogSetRetentionTime(retentionTime)
    __wow_combat_log_state.retentionTime = tonumber(retentionTime) or 0
    return true
  end

  function CombatLogGetRetentionTime()
    return __wow_combat_log_state.retentionTime
  end

  function CombatLog_Object_IsA(objectType, mask)
    return __wow_combat_log_object_is_a(objectType, mask)
  end
end

-- Bonus / world-quest objective trackers iterate the task list at startup.
-- Return an empty table so the `for ... in ipairs(tasksTable)` loops no-op.
if GetTasksTable == nil then
  function GetTasksTable()
    return {}
  end
end

if SpellCanTargetQuest == nil then
  function SpellCanTargetQuest()
    return false
  end
end

-- Auto quest popups (tutorial toasts). Not simulated; `for i = 1, N do`
-- loops in AutoQuestPopUpTracker iterate zero times.
if GetNumAutoQuestPopUps == nil then
  function GetNumAutoQuestPopUps() return 0 end
end
if GetAutoQuestPopUp == nil then
  function GetAutoQuestPopUp(_index) return nil, nil end
end

-- AccountStore / DamageMeter / CooldownViewer: Blizzard data-provider init
-- iterates the returned category / session / cooldown list with ipairs.
-- None of these subsystems are simulated; return empty tables.

if EnumUtil == nil then
  EnumUtil = {}
end

if EnumUtil.MakeEnum == nil then
  function EnumUtil.MakeEnum(...)
    local enum = {}
    for index = 1, select("#", ...) do
      local name = select(index, ...)
      enum[name] = index
    end
    return enum
  end
end

if CreateCounter == nil then
  function CreateCounter()
    local nextID = 0
    return function()
      nextID = nextID + 1
      return nextID
    end
  end
end

if GetOrCreateTableEntry == nil then
  function GetOrCreateTableEntry(tbl, key)
    local value = tbl[key]
    if value == nil then
      value = {}
      tbl[key] = value
    end
    return value
  end
end

if GenerateClosure == nil then
  function GenerateClosure(fn, ...)
    local bound = { n = select("#", ...), ... }
    return function(...)
      local args = {}
      local argCount = 0
      for i = 1, bound.n do
        argCount = argCount + 1
        args[argCount] = bound[i]
      end
      for i = 1, select("#", ...) do
        argCount = argCount + 1
        args[argCount] = select(i, ...)
      end
      return fn(unpack(args, 1, argCount))
    end
  end
end

if SecureTypes == nil then
  SecureTypes = {}
end

do
  local __wow_original_securecall = securecall
  if type(__wow_original_securecall) == "function" and not rawget(_G, "__wow_securecall_accepts_names") then
    function securecall(fn, ...)
      if type(fn) == "string" then
        fn = _G[fn]
      end
      return __wow_original_securecall(fn, ...)
    end

    rawset(_G, "__wow_securecall_accepts_names", true)
  end
end

local function __wow_securetypes_call(fn, ...)
  if type(securecallfunction) == "function" then
    return securecallfunction(fn, ...)
  end
  return fn(...)
end

SecureTypes.CreateSecureMap = SecureTypes.CreateSecureMap or function(mixin)
  local SecureMap = {}

  function SecureMap:GetValue(key)
    return __wow_securetypes_call(rawget, self.tbl, key)
  end

  function SecureMap:SetValue(key, value)
    assert(not issecretvalue(key), "attempted to store a secret key in a SecureMap")
    assert(not issecretvalue(value), "attempted to store a secret value in a SecureMap")
    self.tbl[key] = value
  end

  function SecureMap:ClearValue(key)
    self.tbl[key] = nil
  end

  function SecureMap:HasKey(key)
    return self:GetValue(key) ~= nil
  end

  function SecureMap:GetNext(key)
    return __wow_securetypes_call(next, self.tbl, key)
  end

  function SecureMap:GetSize()
    local count = 0
    for _ in pairs(self.tbl) do
      count = count + 1
    end
    return count
  end

  function SecureMap:IsEmpty()
    return self:GetNext() == nil
  end

  function SecureMap:Wipe()
    for key in pairs(self.tbl) do
      self.tbl[key] = nil
    end
  end

  function SecureMap:Enumerate()
    local iterator, tbl, index = next, self.tbl, nil
    local function Iterator(_, key)
      return __wow_securetypes_call(iterator, tbl, key)
    end

    return Iterator, nil, index
  end

  function SecureMap:ExecuteRange(func, ...)
    return secureexecuterange(self.tbl, func, ...)
  end

  function SecureMap:ExecuteTable(func)
    return __wow_securetypes_call(func, self.tbl)
  end

  function SecureMap:Insert(key, value)
    self:SetValue(key, value)
  end

  function SecureMap:Remove(key)
    local value = self:GetValue(key)
    self:ClearValue(key)
    return value
  end

  function SecureMap:Find(key)
    return self:GetValue(key)
  end

  function SecureMap:Contains(key)
    return self:HasKey(key)
  end

  function SecureMap:Clear()
    self:Wipe()
  end

  SecureMap.__index = function(t, key)
    local mapValue = SecureMap[key]
    if mapValue then
      return mapValue
    end
    return SecureMap.GetValue(t, key)
  end

  SecureMap.__newindex = function(t, key, value)
    t:SetValue(key, value)
  end

  local map = { tbl = {} }
  setmetatable(map, SecureMap)

  if mixin and type(Mixin) == "function" then
    Mixin(map, mixin)
  end

  return map
end
SecureTypes.CreateSecureFunction = SecureTypes.CreateSecureFunction or function(fn) return fn end
SecureTypes.CreateSecureNumber = SecureTypes.CreateSecureNumber or function(value) return value or 0 end
SecureTypes.CreateSecureArray = SecureTypes.CreateSecureArray or function()
  local array = {}
  local methods = {}
  function methods:Insert(value)
    self[#self + 1] = value
  end
  function methods:Remove(value)
    for index, existing in ipairs(self) do
      if existing == value then
        table.remove(self, index)
        return true
      end
    end
    return false
  end
  function methods:Clear()
    for index = #self, 1, -1 do
      self[index] = nil
    end
  end
  function methods:Enumerate()
    local index = 0
    return function()
      index = index + 1
      if index <= #self then
        return self[index]
      end
    end
  end
  function methods:FindInTableIf(predicate)
    return FindInTableIf(self, predicate)
  end
  return setmetatable(array, { __index = methods })
end

for _, __wow_camera_verb in ipairs({
  "MoveViewOutStart", "MoveViewOutStop",
  "MoveViewInStart", "MoveViewInStop",
  "MoveViewLeftStart", "MoveViewLeftStop",
  "MoveViewRightStart", "MoveViewRightStop",
  "MoveViewUpStart", "MoveViewUpStop",
  "MoveViewDownStart", "MoveViewDownStop",
}) do
  if _G[__wow_camera_verb] == nil then
    _G[__wow_camera_verb] = function()
    end
  end
end

if GetPVPLifetimeStats == nil then
  function GetPVPLifetimeStats()
    return 0, 0
  end
end
if GetQuestResetTime == nil then
  function GetQuestResetTime()
    if C_DateAndTime and type(C_DateAndTime.GetSecondsUntilDailyReset) == "function" then
      return C_DateAndTime.GetSecondsUntilDailyReset()
    end
    return 86400
  end
end

local __wow_modified_clicks = __wow_modified_clicks or {}
if GetModifiedClick == nil then
  function GetModifiedClick(action)
    return __wow_modified_clicks[action] or "NONE"
  end
end
if SetModifiedClick == nil then
  function SetModifiedClick(action, modifier)
    __wow_modified_clicks[action] = modifier or "NONE"
  end
end

if UnitClass == nil then
  function UnitClass(_unit)
    return "Paladin", "PALADIN", 2
  end
end

if UnitRace == nil then
  function UnitRace(_unit)
    return "Human", "Human", 1
  end
end

if UnitNameUnmodified == nil then
  function UnitNameUnmodified(_unit)
    return "SimPlayer", GetRealmName()
  end
end

if GetChatTypeIndex == nil then
  function GetChatTypeIndex()
    return 1
  end
end

if GetScenariosChoiceOrder == nil then
  function GetScenariosChoiceOrder()
    return {}
  end
end

if GetNumRandomScenarios == nil then
  function GetNumRandomScenarios()
    return 0
  end
end

if GetRandomScenarioInfo == nil then
  function GetRandomScenarioInfo(_)
    return nil
  end
end

if GetLFDRoleRestrictions == nil then
  function GetLFDRoleRestrictions(_)
    return false, false, false
  end
end

if GetLFGRoleShortageRewards == nil then
  function GetLFGRoleShortageRewards(_, _)
    return false, false, false, false, 0, 0, 0
  end
end

if GetProfessionSkillLineID == nil then
  function GetProfessionSkillLineID()
    return 0
  end
end

if UnitSex == nil then
  function UnitSex()
    return 2
  end
end

if UnitIsGhost == nil then
  function UnitIsGhost(_unit)
    return false
  end
end

if UnitIsDead == nil then
  function UnitIsDead(_unit)
    return UnitHealth(_unit) <= 0
  end
end

if CanAutoSetGamePadCursorControl == nil then
  function CanAutoSetGamePadCursorControl(_enabled)
    return false
  end
end

if SetGamePadCursorControl == nil then
  function SetGamePadCursorControl(_enabled)
  end
end

if LocalizedClassList == nil then
  function LocalizedClassList(_female)
    return {
      WARRIOR = "Warrior",
      PALADIN = "Paladin",
      HUNTER = "Hunter",
      ROGUE = "Rogue",
      PRIEST = "Priest",
      DEATHKNIGHT = "Death Knight",
      SHAMAN = "Shaman",
      MAGE = "Mage",
      WARLOCK = "Warlock",
      MONK = "Monk",
      DRUID = "Druid",
      DEMONHUNTER = "Demon Hunter",
      EVOKER = "Evoker",
    }
  end
end

if RegisterUIPanel == nil then
  function RegisterUIPanel()
  end
end

if CloseAllWindows == nil then
  function CloseAllWindows()
    return false
  end
end

if AddTooltipDataAccessor == nil then
  function AddTooltipDataAccessor()
  end
end

if GetScreenWidth == nil then
  function GetScreenWidth()
    return 1024
  end
end

if GetScreenHeight == nil then
  function GetScreenHeight()
    return 768
  end
end

if GetPhysicalScreenSize == nil then
  function GetPhysicalScreenSize()
    return GetScreenWidth(), GetScreenHeight()
  end
end

if GetNumLanguages == nil then
  function GetNumLanguages()
    return 0
  end
end

if UnitName == nil then
  function UnitName(unit)
    return UnitNameUnmodified(unit)
  end
end

if UnitGUID == nil then
  function UnitGUID(unit)
    return "Player-0-00000000-" .. tostring(unit or "player")
  end
end

if UnitIsConnected == nil then
  function UnitIsConnected(_unit)
    return true
  end
end

if UnitIsPossessed == nil then
  function UnitIsPossessed(_unit)
    return false
  end
end

if LE_REALM_RELATION_SAME == nil then
  LE_REALM_RELATION_SAME = 0
end

if UnitRealmRelationship == nil then
  function UnitRealmRelationship(_unit)
    return LE_REALM_RELATION_SAME
  end
end

if UnitPosition == nil then
  function UnitPosition(_unit)
    return 0, 0, 0, 0
  end
end

if UnitLevel == nil then
  function UnitLevel(unit)
    if unit == "player" or unit == "pet" then
      return GetMaxPlayerLevel and GetMaxPlayerLevel() or 80
    end
    return 1
  end
end

if UnitEffectiveLevel == nil then
  function UnitEffectiveLevel(unit)
    return UnitLevel(unit)
  end
end

if GetMaxPlayerLevel == nil then
  function GetMaxPlayerLevel()
    return 80
  end
end

if PlayerHasToy == nil then
  function PlayerHasToy(itemID)
    return C_ToyBox ~= nil and C_ToyBox.GetToyInfo ~= nil and C_ToyBox.GetToyInfo(itemID) ~= nil
  end
end

if UnitInPartyIsAI == nil then
  function UnitInPartyIsAI(_unit)
    return false
  end
end

if UnitAffectingCombat == nil then
  function UnitAffectingCombat(_unit)
    return false
  end
end

if UnitIsPVPFreeForAll == nil then
  function UnitIsPVPFreeForAll(_unit)
    return false
  end
end

if UnitPhaseReason == nil then
  function UnitPhaseReason(_unit)
    return nil
  end
end

if PartialPlayTime == nil then
  function PartialPlayTime()
    return false
  end
end

if NoPlayTime == nil then
  function NoPlayTime()
    return false
  end
end

if GetBillingTimeRested == nil then
  function GetBillingTimeRested()
    return 0
  end
end

if GetUnitTotalModifiedMaxHealthPercent == nil then
  function GetUnitTotalModifiedMaxHealthPercent(_unit)
    return 0
  end
end

if GetNumShapeshiftForms == nil then
  function GetNumShapeshiftForms()
    return 0
  end
end

if GetShapeshiftForm == nil then
  function GetShapeshiftForm()
    return 0
  end
end

if GetTotemInfo == nil then
  function GetTotemInfo(_slot)
    return false, nil, 0, 0, nil
  end
end

if GetNumSpecializations == nil then
  function GetNumSpecializations()
    return 3
  end
end

if GetSpecializationInfoForClassID == nil then
  function GetSpecializationInfoForClassID(classID, index, _sex)
    local specID = ((tonumber(classID) or 0) * 100) + (tonumber(index) or 1)
    return specID, "Spec " .. tostring(index or 1), "", 0, "DAMAGER", false, true
  end
end

if GetDifficultyInfo == nil then
  function GetDifficultyInfo(_difficultyID)
    return "Normal", 0, false, false, false, false
  end
end

if GetReleaseTimeRemaining == nil then
  function GetReleaseTimeRemaining()
    return 0
  end
end

if GetInventoryItemTexture == nil then
  function GetInventoryItemTexture(_unit, _slot)
    return nil
  end
end

if SetItemButtonTexture == nil then
  function SetItemButtonTexture(button, texture)
    if type(button) ~= "table" then
      return
    end
    local icon = button.icon or button.Icon
    if icon ~= nil and type(icon.SetTexture) == "function" then
      icon:SetTexture(texture)
      if texture ~= nil and type(icon.Show) == "function" then
        icon:Show()
      end
    end
  end
end

if SetItemButtonCount == nil then
  function SetItemButtonCount(button, count)
    if type(button) ~= "table" then
      return
    end
    local countText = button.Count
    if countText ~= nil and type(countText.SetText) == "function" then
      if count == nil or count == 0 then
        countText:SetText("")
      else
        countText:SetText(tostring(count))
      end
    end
  end
end

if SetItemButtonTextureVertexColor == nil then
  function SetItemButtonTextureVertexColor(button, r, g, b)
    if type(button) ~= "table" then
      return
    end
    local icon = button.icon or button.Icon
    if icon ~= nil and type(icon.SetVertexColor) == "function" then
      icon:SetVertexColor(r or 1, g or 1, b or 1)
    end
  end
end

if SetItemButtonNormalTextureVertexColor == nil then
  function SetItemButtonNormalTextureVertexColor(button, r, g, b)
    if type(button) ~= "table" then
      return
    end
    local normalTexture = button.NormalTexture or button.normalTexture
    if normalTexture ~= nil and type(normalTexture.SetVertexColor) == "function" then
      normalTexture:SetVertexColor(r or 1, g or 1, b or 1)
      return
    end
    SetItemButtonTextureVertexColor(button, r, g, b)
  end
end

if IsInventoryItemProfessionBag == nil then
  function IsInventoryItemProfessionBag(_unit, _slot)
    return false
  end
end

if GetSendMailPrice == nil then
  function GetSendMailPrice()
    return 0
  end
end

if GetMerchantFilter == nil then
  function GetMerchantFilter()
    return 0
  end
end

if SetMerchantFilter == nil then
  function SetMerchantFilter(_filter)
  end
end

if IsVeteranTrialAccount == nil then
  function IsVeteranTrialAccount()
    return false
  end
end

if IsAccountSecured == nil then
  function IsAccountSecured()
    return true
  end
end

if AbbreviateNumbers == nil then
  function AbbreviateNumbers(value)
    return tostring(value or 0)
  end
end

if BNGetInfo == nil then
  function BNGetInfo()
    return nil
  end
end

if GetLFGDeserterExpiration == nil then
  function GetLFGDeserterExpiration()
    return 0
  end
end

if StoreSecureReference == nil then
  function StoreSecureReference(name, value)
    if type(name) == "string" then
      rawset(_G, name, value)
    end
  end
end

if UnitStagger == nil then
  function UnitStagger(_unit)
    return 0
  end
end

if GetPossessInfo == nil then
  function GetPossessInfo(_index)
    return nil, nil, false
  end
end

if IsInJailersTower == nil then
  function IsInJailersTower()
    return false
  end
end

TooltipDataProcessor = TooltipDataProcessor or __wow_namespace({
  AllTypes = 0,
  AddTooltipPostCall = __wow_noop,
  AddLinePostCall = __wow_noop,
})

EventRegistry = EventRegistry or __wow_namespace({
  RegisterCallback = __wow_noop,
  TriggerEvent = __wow_noop,
  RegisterFrameEventAndCallback = __wow_noop,
})

UIWidgetManager = UIWidgetManager or __wow_namespace({
  RegisterWidgetVisTypeTemplate = __wow_noop,
})

Settings = Settings or __wow_namespace({
  GetOrCreateSettingsGroup = function()
    return __wow_namespace({
      AddInitializer = __wow_noop,
      AddSetting = __wow_noop,
      AddCategory = __wow_noop,
      SetValue = __wow_noop,
      GetValue = function() return nil end,
    })
  end,
})

do
  local settingsPanel = rawget(_G, "SettingsPanel")
  local categories = rawget(Settings, "_categories")
  if type(categories) ~= "table" then
    categories = {}
    rawset(Settings, "_categories", categories)
  end

  local function ensure_category(id, name)
    local category = categories[id]
    if type(category) ~= "table" then
      category = {
        id = id,
        name = name,
        GetID = function(self) return self.id end,
        GetName = function(self) return self.name end,
      }
      categories[id] = category
    end
    return category
  end

  local interfaceCategory = ensure_category(1, "Interface")
  local audioCategory = ensure_category(2, "Audio")
  rawset(Settings, "INTERFACE_CATEGORY_ID", interfaceCategory:GetID())
  rawset(Settings, "AUDIO_CATEGORY_ID", audioCategory:GetID())

  if rawget(Settings, "GetCategory") == nil then
    function Settings.GetCategory(id)
      id = tonumber(id)
      if categories[id] == nil then
        if id == rawget(Settings, "INTERFACE_CATEGORY_ID") then
          return ensure_category(id, "Interface")
        end
        if id == rawget(Settings, "AUDIO_CATEGORY_ID") then
          return ensure_category(id, "Audio")
        end
      end
      return categories[id]
    end
  end

  if type(settingsPanel) == "table" then
    settingsPanel._layouts = settingsPanel._layouts or {}

    local function ensure_layout(category)
      local categoryID = category:GetID()
      local layout = settingsPanel._layouts[categoryID]
      if type(layout) ~= "table" then
        layout = {
          _initializers = {},
          GetInitializers = function(self) return self._initializers end,
        }
        settingsPanel._layouts[categoryID] = layout
      end
      return layout
    end

    if rawget(settingsPanel, "GetLayout") == nil then
      function settingsPanel:GetLayout(category)
        if type(category) ~= "table" or type(category.GetID) ~= "function" then
          return nil
        end
        return self._layouts and self._layouts[category:GetID()] or nil
      end
    end

    if rawget(settingsPanel, "GetCurrentCategory") == nil then
      function settingsPanel:GetCurrentCategory()
        return rawget(self, "_currentCategory")
      end
    end

    local nextDynamicCategoryID = 1000

    local function set_category_frame_shown(category, shown)
      local frame = category and category.frame or nil
      if type(frame) ~= "table" then
        return
      end
      if type(frame.SetShown) == "function" then
        pcall(frame.SetShown, frame, shown)
        return
      end
      if shown and type(frame.Show) == "function" then
        pcall(frame.Show, frame)
      elseif not shown and type(frame.Hide) == "function" then
        pcall(frame.Hide, frame)
      end
    end

    local function hide_inactive_category_frames(activeCategory)
      for _, registeredCategory in pairs(categories) do
        if registeredCategory ~= activeCategory then
          set_category_frame_shown(registeredCategory, false)
        end
      end
    end

    if rawget(Settings, "RegisterCanvasLayoutCategory") == nil then
      function Settings.RegisterCanvasLayoutCategory(frame, name, parentCategory)
        nextDynamicCategoryID = nextDynamicCategoryID + 1
        local categoryName = name
        if categoryName == nil and type(frame) == "table" then
          categoryName = rawget(frame, "name") or rawget(frame, "Name")
          if categoryName == nil and type(frame.GetName) == "function" then
            categoryName = frame:GetName()
          end
        end
        local category = ensure_category(nextDynamicCategoryID, categoryName or "AddOn")
        category.frame = frame
        category.parentCategory = parentCategory
        settingsPanel._layouts[category:GetID()] = {
          frame = frame,
          GetFrame = function(self) return self.frame end,
        }
        set_category_frame_shown(category, false)
        return category, settingsPanel._layouts[category:GetID()]
      end
    end

    if rawget(Settings, "RegisterCanvasLayoutSubcategory") == nil then
      function Settings.RegisterCanvasLayoutSubcategory(parentCategory, frame, name)
        local category = Settings.RegisterCanvasLayoutCategory(frame, name, parentCategory)
        return category, settingsPanel._layouts[category:GetID()]
      end
    end

    if rawget(Settings, "RegisterAddOnCategory") == nil then
      function Settings.RegisterAddOnCategory(_category) end
    end

    local audioLayout = ensure_layout(audioCategory)
    if #audioLayout:GetInitializers() == 0 then
      local setting = {
        GetVariable = function() return "Sound_OutputDriverIndex" end,
      }
      local initializer = {
        GetSetting = function() return setting end,
        GetOptions = function()
          return function()
            return {
              { value = 0, label = "Silent Output Device" },
            }
          end
        end,
      }
      table.insert(audioLayout:GetInitializers(), initializer)
    end

    ensure_layout(interfaceCategory)

    function Settings.OpenToCategory(categoryID)
      local category = Settings.GetCategory(categoryID)
      if category == nil then
        return nil
      end
      local panel = rawget(_G, "SettingsPanel") or settingsPanel
      rawset(panel, "_currentCategory", category)
      if type(panel.SetShown) == "function" then
        pcall(panel.SetShown, panel, true)
      end
      if type(panel.Show) == "function" then
        pcall(panel.Show, panel)
      end
      hide_inactive_category_frames(category)
      set_category_frame_shown(category, true)
      return category
    end
  end
end

if rawget(_G, "InterfaceOptions_AddCategory") == nil then
  function InterfaceOptions_AddCategory(frame, addonName, position)
    if Settings and type(Settings.RegisterCanvasLayoutCategory) == "function" then
      local category = Settings.RegisterCanvasLayoutCategory(frame, addonName, position)
      if type(Settings.RegisterAddOnCategory) == "function" then
        Settings.RegisterAddOnCategory(category)
      end
      return category
    end
    return frame
  end
end

EditModeAccountSettingsMixin = EditModeAccountSettingsMixin or {}
BaseActionButtonMixin = BaseActionButtonMixin or {}

ActionButtonSpellAlertManager = ActionButtonSpellAlertManager or __wow_namespace({
  _defaultAlertType = 1,
  activeAlerts = {},
})

local function __wow_action_button_alert_fields(button)
  local env = debug.getfenv and debug.getfenv(button)
  if type(env) ~= "table" then
    return nil
  end
  local fields = env[1]
  if type(fields) ~= "table" then
    fields = {}
    env[1] = fields
  end
  return fields
end

if rawget(ActionButtonSpellAlertManager, "HasAlert") == nil then
  function ActionButtonSpellAlertManager:HasAlert(button)
    local alertType = self.activeAlerts and self.activeAlerts[button]
    if alertType ~= nil then
      return true, alertType
    end
    return false
  end
end

if rawget(ActionButtonSpellAlertManager, "ShowAlert") == nil then
  function ActionButtonSpellAlertManager:ShowAlert(button, alertType)
    if button == nil then
      return
    end
    alertType = alertType or self._defaultAlertType or 1
    self.activeAlerts[button] = alertType
    local fields = __wow_action_button_alert_fields(button)
    local alert = fields and rawget(fields, "SpellActivationAlert")
    if alert == nil then
      alert = CreateFrame("Frame", nil, UIParent or button)
      if fields then
        rawset(fields, "SpellActivationAlert", alert)
      end
      button.SpellActivationAlert = alert
    end
    button:Show()
    alert:Show()
  end
end

if rawget(ActionButtonSpellAlertManager, "HideAlert") == nil then
  function ActionButtonSpellAlertManager:HideAlert(button)
    if button == nil then
      return
    end
    self.activeAlerts[button] = nil
    local fields = __wow_action_button_alert_fields(button)
    local alert = fields and rawget(fields, "SpellActivationAlert")
    if alert ~= nil then
      alert:Hide()
    end
  end
end

if bit == nil then
  local function normalize(v)
    v = math.floor(tonumber(v) or 0)
    if v < 0 then
      v = 0x100000000 + v
    end
    return v % 0x100000000
  end

  local function fold(values, identity, step)
    local result = identity
    for i = 1, #values do
      result = step(result, normalize(values[i]))
    end
    return normalize(result)
  end

  local function lshift(a, n)
    return normalize(normalize(a) * (2 ^ normalize(n)))
  end

  local function rshift(a, n)
    return math.floor(normalize(a) / (2 ^ normalize(n)))
  end

  local function band2(a, b)
    local result = 0
    local bitValue = 1
    a = normalize(a)
    b = normalize(b)
    while a > 0 or b > 0 do
      local abit = a % 2
      local bbit = b % 2
      if abit == 1 and bbit == 1 then
        result = result + bitValue
      end
      a = math.floor(a / 2)
      b = math.floor(b / 2)
      bitValue = bitValue * 2
    end
    return result
  end

  local function bor2(a, b)
    local result = 0
    local bitValue = 1
    a = normalize(a)
    b = normalize(b)
    while a > 0 or b > 0 do
      local abit = a % 2
      local bbit = b % 2
      if abit == 1 or bbit == 1 then
        result = result + bitValue
      end
      a = math.floor(a / 2)
      b = math.floor(b / 2)
      bitValue = bitValue * 2
    end
    return result
  end

  bit = {
    band = function(...)
      return fold({...}, 0xFFFFFFFF, band2)
    end,
    bor = function(...)
      return fold({...}, 0, bor2)
    end,
    bxor = function(a, b)
      a = normalize(a)
      b = normalize(b)
      local result = 0
      local bitValue = 1
      while a > 0 or b > 0 do
        local abit = a % 2
        local bbit = b % 2
        if abit ~= bbit then
          result = result + bitValue
        end
        a = math.floor(a / 2)
        b = math.floor(b / 2)
        bitValue = bitValue * 2
      end
      return result
    end,
    bnot = function(a)
      return 0xFFFFFFFF - normalize(a)
    end,
    lshift = lshift,
    rshift = rshift,
    arshift = rshift,
    mod = function(a, b)
      return normalize(a) % normalize(b)
    end,
  }
end

TextureKitConstants = TextureKitConstants or {
  SetVisibility = true,
  DoNotSetVisibility = false,
  UseAtlasSize = true,
  IgnoreAtlasSize = false,
  AddressModeClamp = 1,
  AddressModeWrap = 2,
  AddressModeAllowAssetToDetermine = 3,
}

if C_CharacterCreation == nil then
  C_CharacterCreation = __wow_namespace()
end
local __wow_character_create_races = rawget(_G, "__wow_character_create_races")
if __wow_character_create_races == nil then
  __wow_character_create_races = {
    { raceID = 1, name = "Human", fileName = "Human", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Versatile and determined.", createScreenIconAtlas = "charactercreate-humans" },
    { raceID = 2, name = "Orc", fileName = "Orc", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Fierce warriors from Draenor.", createScreenIconAtlas = "charactercreate-orcs" },
    { raceID = 3, name = "Dwarf", fileName = "Dwarf", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Stout defenders of Khaz Modan.", createScreenIconAtlas = "charactercreate-dwarves" },
    { raceID = 4, name = "Night Elf", fileName = "NightElf", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Ancient guardians of nature.", createScreenIconAtlas = "charactercreate-nightelves" },
    { raceID = 5, name = "Undead", fileName = "Scourge", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Forsaken who fight for their future.", createScreenIconAtlas = "charactercreate-undead" },
    { raceID = 6, name = "Tauren", fileName = "Tauren", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Noble protectors of the plains.", createScreenIconAtlas = "charactercreate-tauren" },
    { raceID = 7, name = "Gnome", fileName = "Gnome", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Inventive and resilient.", createScreenIconAtlas = "charactercreate-gnomes" },
    { raceID = 8, name = "Troll", fileName = "Troll", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Regenerating jungle fighters.", createScreenIconAtlas = "charactercreate-trolls" },
    { raceID = 9, name = "Goblin", fileName = "Goblin", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Clever deal-makers and engineers.", createScreenIconAtlas = "charactercreate-goblins" },
    { raceID = 10, name = "Blood Elf", fileName = "BloodElf", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Arcane masters with fierce pride.", createScreenIconAtlas = "charactercreate-bloodelves" },
    { raceID = 11, name = "Draenei", fileName = "Draenei", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Exiles guided by the Light.", createScreenIconAtlas = "charactercreate-draenei" },
    { raceID = 22, name = "Worgen", fileName = "Worgen", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Ferocious survivors of Gilneas.", createScreenIconAtlas = "charactercreate-worgen" },
    { raceID = 24, name = "Pandaren", fileName = "Pandaren", factionInternalName = "Neutral", enabled = true, isNeutralRace = true, isAlliedRace = false, loreDescription = "Wanderers seeking balance.", createScreenIconAtlas = "charactercreate-pandaren" },
    { raceID = 25, name = "Nightborne", fileName = "Nightborne", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Arcwine-fueled children of Suramar.", createScreenIconAtlas = "charactercreate-nightborne" },
    { raceID = 26, name = "Highmountain Tauren", fileName = "HighmountainTauren", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Descendants of Huln Highmountain.", createScreenIconAtlas = "charactercreate-highmountaintauren" },
    { raceID = 27, name = "Void Elf", fileName = "VoidElf", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Ren'dorei shaped by the Void.", createScreenIconAtlas = "charactercreate-voidelves" },
    { raceID = 28, name = "Lightforged Draenei", fileName = "LightforgedDraenei", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Veterans of the Army of the Light.", createScreenIconAtlas = "charactercreate-lightforgeddraenei" },
    { raceID = 29, name = "Zandalari Troll", fileName = "ZandalariTroll", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Ancient kings of troll empires.", createScreenIconAtlas = "charactercreate-zandalaritroll" },
    { raceID = 30, name = "Kul Tiran", fileName = "KulTiran", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Seafaring mariners and tide sages.", createScreenIconAtlas = "charactercreate-kultiran" },
    { raceID = 31, name = "Dark Iron Dwarf", fileName = "DarkIronDwarf", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Fire-forged dwarves of Blackrock.", createScreenIconAtlas = "charactercreate-darkirondwarf" },
    { raceID = 32, name = "Mag'har Orc", fileName = "MagharOrc", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Uncorrupted clans from alternate Draenor.", createScreenIconAtlas = "charactercreate-magharorc" },
    { raceID = 34, name = "Mechagnome", fileName = "Mechagnome", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Tinkerers enhanced with machinery.", createScreenIconAtlas = "charactercreate-mechagnomes" },
    { raceID = 35, name = "Vulpera", fileName = "Vulpera", factionInternalName = "Horde", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Resourceful nomads of Vol'dun.", createScreenIconAtlas = "charactercreate-vulpera" },
    { raceID = 36, name = "Dracthyr", fileName = "Dracthyr", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = false, loreDescription = "Awakened draconic soldiers.", createScreenIconAtlas = "charactercreate-dracthyr" },
    { raceID = 37, name = "Earthen", fileName = "Earthen", factionInternalName = "Alliance", enabled = true, isNeutralRace = false, isAlliedRace = true, loreDescription = "Titan-forged explorers of the deep places.", createScreenIconAtlas = "charactercreate-earthen" },
  }
  rawset(_G, "__wow_character_create_races", __wow_character_create_races)
end

local __wow_character_create_classes = rawget(_G, "__wow_character_create_classes")
if __wow_character_create_classes == nil then
  __wow_character_create_classes = {
    { classID = 1, fileName = "WARRIOR", name = "Warrior", description = "Front-line melee fighter.", roleInfo = "Tank, Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 2, fileName = "PALADIN", name = "Paladin", description = "Holy crusader of the Light.", roleInfo = "Tank, Healer, Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 3, fileName = "HUNTER", name = "Hunter", description = "Ranged weapon master.", roleInfo = "Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 4, fileName = "ROGUE", name = "Rogue", description = "Stealth and precision specialist.", roleInfo = "Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 5, fileName = "PRIEST", name = "Priest", description = "Devout wielder of Light and Shadow.", roleInfo = "Healer, Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 6, fileName = "DEATHKNIGHT", name = "Death Knight", description = "Runeblade champion of undeath.", roleInfo = "Tank, Damage", enabled = true, animLoopWaitTimeSeconds = 0.5, earlyFactionChoice = true },
    { classID = 7, fileName = "SHAMAN", name = "Shaman", description = "Channeler of the elements.", roleInfo = "Healer, Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 8, fileName = "MAGE", name = "Mage", description = "Master of arcane power.", roleInfo = "Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 9, fileName = "WARLOCK", name = "Warlock", description = "Fel caster with demonic allies.", roleInfo = "Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 10, fileName = "MONK", name = "Monk", description = "Martial artist with mystic focus.", roleInfo = "Tank, Healer, Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 11, fileName = "DRUID", name = "Druid", description = "Shapeshifter of the wilds.", roleInfo = "Tank, Healer, Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 12, fileName = "DEMONHUNTER", name = "Demon Hunter", description = "Agile hunter of the Legion.", roleInfo = "Tank, Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
    { classID = 13, fileName = "EVOKER", name = "Evoker", description = "Dracthyr spellcaster wielding dragonflights.", roleInfo = "Healer, Damage", enabled = true, animLoopWaitTimeSeconds = 0.5 },
  }
  rawset(_G, "__wow_character_create_classes", __wow_character_create_classes)
end

local function __wow_chr_customization_option_type(kind)
  if Enum ~= nil and Enum.ChrCustomizationOptionType ~= nil and Enum.ChrCustomizationOptionType[kind] ~= nil then
    return Enum.ChrCustomizationOptionType[kind]
  end
  if kind == "Checkbox" then
    return 1
  elseif kind == "Slider" then
    return 2
  end
  return 0
end

local function __wow_clone_table(value)
  local copy = {}
  for k, v in pairs(value) do
    copy[k] = v
  end
  return copy
end

local function __wow_find_character_create_race(raceID)
  for _, raceData in ipairs(__wow_character_create_races) do
    if raceData.raceID == raceID then
      return __wow_clone_table(raceData)
    end
  end
  return nil
end

local function __wow_find_character_create_class(classID)
  for _, classData in ipairs(__wow_character_create_classes) do
    if classData.classID == classID then
      return __wow_clone_table(classData)
    end
  end
  return nil
end

local function __wow_character_create_categories()
  local function choices(baseID, names)
    local out = {}
    for index, name in ipairs(names) do
      out[index] = {
        id = baseID + index - 1,
        choiceIndex = index,
        name = name,
      }
    end
    return out
  end

  return {
    {
      id = 1,
      name = "Face",
      options = {
        { id = 101, orderIndex = 1, name = "Face Shape", optionType = __wow_chr_customization_option_type("Dropdown"), currentChoiceIndex = 1, choices = choices(1001, { "Face 1", "Face 2", "Face 3" }) },
        { id = 102, orderIndex = 2, name = "Skin Tone", optionType = __wow_chr_customization_option_type("Slider"), currentChoiceIndex = 2, choices = choices(1011, { "Tone 1", "Tone 2", "Tone 3" }) },
      },
    },
    {
      id = 2,
      name = "Hair",
      options = {
        { id = 201, orderIndex = 1, name = "Hair Style", optionType = __wow_chr_customization_option_type("Dropdown"), currentChoiceIndex = 1, choices = choices(2001, { "Style 1", "Style 2", "Style 3" }) },
        { id = 202, orderIndex = 2, name = "Hair Color", optionType = __wow_chr_customization_option_type("Dropdown"), currentChoiceIndex = 2, choices = choices(2011, { "Color 1", "Color 2", "Color 3" }) },
      },
    },
    {
      id = 3,
      name = "Details",
      options = {
        { id = 301, orderIndex = 1, name = "Accessories", optionType = __wow_chr_customization_option_type("Checkbox"), currentChoiceIndex = 1, choices = choices(3001, { "Off", "On" }) },
        { id = 302, orderIndex = 2, name = "Markings", optionType = __wow_chr_customization_option_type("Dropdown"), currentChoiceIndex = 1, choices = choices(3011, { "Marking 1", "Marking 2" }) },
      },
    },
  }
end

rawset(_G, "__wow_selected_race_id", rawget(_G, "__wow_selected_race_id") or __wow_character_create_races[1].raceID)
rawset(_G, "__wow_selected_class_id", rawget(_G, "__wow_selected_class_id") or __wow_character_create_classes[1].classID)
rawset(_G, "__wow_selected_sex_id", rawget(_G, "__wow_selected_sex_id") or 0)
rawset(
  _G,
  "__wow_character_create_type",
  rawget(_G, "__wow_character_create_type")
    or (Enum ~= nil and Enum.CharacterCreateType ~= nil and Enum.CharacterCreateType.Normal or 0)
)
function C_CharacterCreation.GetNumCharacterTemplates()
  return 0
end
if rawget(C_CharacterCreation, "GetBlockedRaces") == nil then
  function C_CharacterCreation.GetBlockedRaces()
    return {}
  end
end
if rawget(C_CharacterCreation, "GetSelectedRace") == nil then
  function C_CharacterCreation.GetSelectedRace()
    return rawget(_G, "__wow_selected_race_id") or __wow_character_create_races[1].raceID
  end
end
if rawget(C_CharacterCreation, "SetSelectedRace") == nil then
  function C_CharacterCreation.SetSelectedRace(raceID)
    local selectedRace = __wow_find_character_create_race(raceID)
    rawset(_G, "__wow_selected_race_id", selectedRace and selectedRace.raceID or __wow_character_create_races[1].raceID)
  end
end
if rawget(C_CharacterCreation, "GetAvailableRaces") == nil then
  function C_CharacterCreation.GetAvailableRaces()
    local races = {}
    for index, raceData in ipairs(__wow_character_create_races) do
      races[index] = __wow_clone_table(raceData)
    end
    return races
  end
end
if rawget(C_CharacterCreation, "GetRaceDataByID") == nil then
  function C_CharacterCreation.GetRaceDataByID(raceID)
    return raceID ~= nil and __wow_find_character_create_race(raceID) or nil
  end
end
if rawget(C_CharacterCreation, "SetSelectedClass") == nil then
  function C_CharacterCreation.SetSelectedClass(classID)
    local selectedClass = __wow_find_character_create_class(classID)
    rawset(_G, "__wow_selected_class_id", selectedClass and selectedClass.classID or __wow_character_create_classes[1].classID)
  end
end
if rawget(C_CharacterCreation, "GetAvailableClasses") == nil then
  function C_CharacterCreation.GetAvailableClasses()
    local classes = {}
    for index, classData in ipairs(__wow_character_create_classes) do
      classes[index] = __wow_clone_table(classData)
    end
    return classes
  end
end
if rawget(C_CharacterCreation, "GetSelectedClass") == nil then
  function C_CharacterCreation.GetSelectedClass()
    return __wow_find_character_create_class(rawget(_G, "__wow_selected_class_id"))
      or __wow_find_character_create_class(__wow_character_create_classes[1].classID)
  end
end
if rawget(C_CharacterCreation, "SetSelectedSex") == nil then
  function C_CharacterCreation.SetSelectedSex(sexID)
    rawset(_G, "__wow_selected_sex_id", sexID or 0)
  end
end
if rawget(C_CharacterCreation, "GetSelectedSex") == nil then
  function C_CharacterCreation.GetSelectedSex()
    return rawget(_G, "__wow_selected_sex_id") or 0
  end
end
if rawget(C_CharacterCreation, "GetFactionForRace") == nil then
  function C_CharacterCreation.GetFactionForRace(raceID)
    local raceData = __wow_find_character_create_race(raceID)
    return raceData and raceData.factionInternalName or "Alliance"
  end
end
if rawget(C_CharacterCreation, "GetNameForRace") == nil then
  function C_CharacterCreation.GetNameForRace(raceID)
    local raceData = __wow_find_character_create_race(raceID)
    return raceData and raceData.name or "Human"
  end
end
if rawget(C_CharacterCreation, "GetClassAchievementRequirements") == nil then
  function C_CharacterCreation.GetClassAchievementRequirements(_raceID, _classID)
    return {}
  end
end
if rawget(C_CharacterCreation, "GetValidRacesForClass") == nil then
  function C_CharacterCreation.GetValidRacesForClass(_classID)
    return C_CharacterCreation.GetAvailableRaces()
  end
end
if rawget(C_CharacterCreation, "GetAlliedRaceAchievementRequirements") == nil then
  function C_CharacterCreation.GetAlliedRaceAchievementRequirements(_raceID)
    return {}
  end
end
if rawget(C_CharacterCreation, "UseBeginnerMode") == nil then
  function C_CharacterCreation.UseBeginnerMode()
    return false
  end
end
if rawget(C_CharacterCreation, "IsViewingAlteredForm") == nil then
  function C_CharacterCreation.IsViewingAlteredForm()
    return false
  end
end
if rawget(C_CharacterCreation, "IsUsingCharacterTemplate") == nil then
  function C_CharacterCreation.IsUsingCharacterTemplate()
    return false
  end
end
if rawget(C_CharacterCreation, "IsForcingCharacterTemplate") == nil then
  function C_CharacterCreation.IsForcingCharacterTemplate()
    return false
  end
end
if rawget(C_CharacterCreation, "IsTimerunningEnabled") == nil then
  function C_CharacterCreation.IsTimerunningEnabled()
    return rawget(_G, "__wow_timerunning_season_id") ~= nil
  end
end
if rawget(C_CharacterCreation, "IsNewPlayerRestricted") == nil then
  function C_CharacterCreation.IsNewPlayerRestricted()
    return false
  end
end
if rawget(C_CharacterCreation, "IsTrialAccountRestricted") == nil then
  function C_CharacterCreation.IsTrialAccountRestricted()
    return false
  end
end
if rawget(C_CharacterCreation, "GetCharacterCreateType") == nil then
  function C_CharacterCreation.GetCharacterCreateType()
    return rawget(_G, "__wow_character_create_type")
      or (Enum ~= nil and Enum.CharacterCreateType ~= nil and Enum.CharacterCreateType.Normal or 0)
  end
end
if rawget(C_CharacterCreation, "SetCharacterCreateType") == nil then
  function C_CharacterCreation.SetCharacterCreateType(characterCreateType)
    rawset(_G, "__wow_character_create_type", characterCreateType)
  end
end
if rawget(C_CharacterCreation, "SetTimerunningSeasonID") == nil then
  function C_CharacterCreation.SetTimerunningSeasonID(seasonID)
    rawset(_G, "__wow_timerunning_season_id", seasonID)
  end
end
if rawget(C_CharacterCreation, "ClearCharacterTemplate") == nil then
  C_CharacterCreation.ClearCharacterTemplate = __wow_noop
end
if rawget(C_CharacterCreation, "ResetCharCustomize") == nil then
  C_CharacterCreation.ResetCharCustomize = __wow_noop
end
if rawget(C_CharacterCreation, "SetCharCustomizeFrame") == nil then
  C_CharacterCreation.SetCharCustomizeFrame = __wow_noop
end
if rawget(C_CharacterCreation, "SetCharCustomizeBackground") == nil then
  C_CharacterCreation.SetCharCustomizeBackground = __wow_noop
end
if rawget(C_CharacterCreation, "GetCreateBackgroundModel") == nil then
  function C_CharacterCreation.GetCreateBackgroundModel()
    return 0
  end
end
if rawget(C_CharacterCreation, "SetModelAlpha") == nil then
  C_CharacterCreation.SetModelAlpha = __wow_noop
end
if rawget(C_CharacterCreation, "PlayClassIdleAnimationOnCharacter") == nil then
  C_CharacterCreation.PlayClassIdleAnimationOnCharacter = __wow_noop
end
if rawget(C_CharacterCreation, "PlayCustomizationIdleAnimationOnCharacter") == nil then
  C_CharacterCreation.PlayCustomizationIdleAnimationOnCharacter = __wow_noop
end
if rawget(C_CharacterCreation, "DestroyAuxModel") == nil then
  C_CharacterCreation.DestroyAuxModel = __wow_noop
end
if rawget(C_CharacterCreation, "GetAvailableCustomizations") == nil then
  function C_CharacterCreation.GetAvailableCustomizations()
    return __wow_character_create_categories()
  end
end
if rawget(C_CharacterCreation, "IsCharacterNameValid") == nil then
  function C_CharacterCreation.IsCharacterNameValid(_name)
    return true, ""
  end
end
if rawget(C_CharacterCreation, "IsGuildNameValid") == nil then
  function C_CharacterCreation.IsGuildNameValid(_name)
    return true, ""
  end
end
if rawget(C_CharacterCreation, "CreateCharacter") == nil then
  function C_CharacterCreation.CreateCharacter(name)
    if A_Admin and A_Admin.SetPlayerName then
      A_Admin.SetPlayerName(name)
    end
  end
end

GameRulesUtil = GameRulesUtil or {}
if rawget(GameRulesUtil, "ShouldShowPlayerCastBar") == nil then
  function GameRulesUtil.ShouldShowPlayerCastBar()
    return true
  end
end

-- Pet battles: lightly modeled, but fresh simulator state is not in a battle.
-- `GetNumPets` is compared numerically during PetBattleFrame OnLoad refresh,
-- so returning nil crashes `petIndex > GetNumPets(owner)`. Zero is the
-- accurate "no pets" answer.
-- C_PetBattles.GetNumPets / GetBattleState are registered from Rust
-- (src/lua_api/globals/pet_battles.rs), backed by SimState::pet_battles.
-- The earlier __wow_merge_namespace at the top of this file already
-- installed the C_PetBattles namespace with stub methods; our Rust
-- registration overrides the two that the PLAN called out.
local __wow_pet_battle_state = rawget(_G, "__wow_pet_battle_state")
if type(__wow_pet_battle_state) ~= "table" then
  __wow_pet_battle_state = {
    battleState = 0,
    numPetsPlayer = 0,
    numPetsEnemy = 0,
    isWildBattle = false,
    queueStatus = Enum.PetBattleQueueStatus and Enum.PetBattleQueueStatus.None or 0,
    queueEstimatedTime = 12,
    queueTime = 4,
    canAcceptQueuedPVPMatch = false,
    selectedActionType = nil,
    selectedActionIndex = nil,
    pendingReportBattlePetTarget = nil,
    pendingReportTargetUnit = nil,
    pvpDuel = {
      pending = false,
      challengedUnit = nil,
      exactMatch = false,
      accepted = false,
    },
    sampleSeeded = false,
  }
  rawset(_G, "__wow_pet_battle_state", __wow_pet_battle_state)
end

local __wow_pet_battle_waiting_state = Enum.PetbattleState and Enum.PetbattleState.WaitingPreBattle or 1
local __wow_pet_battle_finished_state = Enum.PetbattleState and Enum.PetbattleState.Finished or 7

local function __wow_pet_battle_seed_sample()
  if __wow_pet_battle_state.sampleSeeded then
    return
  end

  __wow_pet_battle_state.sampleSeeded = true
  __wow_pet_battle_state.numPetsPlayer = 3
  __wow_pet_battle_state.numPetsEnemy = 2
  __wow_pet_battle_state.isWildBattle = true
  __wow_pet_battle_state.playerPets = {
    {
      name = "Arcane Familiar",
      level = 25,
      health = 1120,
      maxHealth = 1420,
      power = 18,
      speed = 21,
      petType = 7,
      xp = 45,
      maxXP = 100,
      abilities = {
        [1] = { id = 1001, name = "Arcane Bite", icon = 0, maxCooldown = 2, description = "Arcane bite.", numTurns = 1, petType = 7, usable = true, cooldown = 0, lockdown = 0 },
        [2] = { id = 1002, name = "Blink Ward", icon = 0, maxCooldown = 1, description = "Blink ward.", numTurns = 1, petType = 7, usable = true, cooldown = 1, lockdown = 0 },
      },
      auras = {
        { auraID = 1002, instanceID = 9001, turnsRemaining = 2, isBuff = true },
      },
    },
    {
      name = "Clockwork Hopper",
      level = 24,
      health = 910,
      maxHealth = 1180,
      power = 15,
      speed = 17,
      petType = 9,
      xp = 15,
      maxXP = 100,
      abilities = {
        [1] = { id = 1003, name = "Spring-Loaded", icon = 0, maxCooldown = 2, description = "Jump forward.", numTurns = 1, petType = 9, usable = true, cooldown = 0, lockdown = 0 },
      },
      auras = {},
    },
    {
      name = "Frost Pup",
      level = 23,
      health = 870,
      maxHealth = 1110,
      power = 14,
      speed = 19,
      petType = 8,
      xp = 10,
      maxXP = 100,
      abilities = {
        [1] = { id = 1004, name = "Snowball", icon = 0, maxCooldown = 1, description = "Throw snowball.", numTurns = 1, petType = 8, usable = true, cooldown = 0, lockdown = 0 },
      },
      auras = {},
    },
  }
  __wow_pet_battle_state.enemyPets = {
    {
      name = "Stone Lurker",
      level = 24,
      health = 980,
      maxHealth = 1320,
      power = 16,
      speed = 14,
      petType = 9,
      xp = 0,
      maxXP = 100,
      abilities = {
        [1] = { id = 1101, name = "Pebble Toss", icon = 0, maxCooldown = 1, description = "Pebble toss.", numTurns = 1, petType = 9, usable = true, cooldown = 0, lockdown = 0 },
      },
      auras = {},
    },
    {
      name = "Bog Hopper",
      level = 24,
      health = 930,
      maxHealth = 1210,
      power = 13,
      speed = 20,
      petType = 9,
      xp = 0,
      maxXP = 100,
      abilities = {
        [1] = { id = 1102, name = "Bog Kick", icon = 0, maxCooldown = 1, description = "Bog kick.", numTurns = 1, petType = 9, usable = true, cooldown = 0, lockdown = 0 },
      },
      auras = {},
    },
  }
  __wow_pet_battle_state.abilitiesByID = {
    [1001] = __wow_pet_battle_state.playerPets[1].abilities[1],
    [1002] = __wow_pet_battle_state.playerPets[1].abilities[2],
    [1003] = __wow_pet_battle_state.playerPets[2].abilities[1],
    [1004] = __wow_pet_battle_state.playerPets[3].abilities[1],
    [1101] = __wow_pet_battle_state.enemyPets[1].abilities[1],
    [1102] = __wow_pet_battle_state.enemyPets[2].abilities[1],
  }
end

local function __wow_pet_battle_ensure_active()
  if not __wow_pet_battle_state.sampleSeeded then
    __wow_pet_battle_seed_sample()
  end
end

local function __wow_pet_battle_get_pet(owner, petIndex)
  __wow_pet_battle_ensure_active()
  local pets
  if owner == (Enum.BattlePetOwner and Enum.BattlePetOwner.Ally or 1) then
    pets = __wow_pet_battle_state.playerPets
  elseif owner == (Enum.BattlePetOwner and Enum.BattlePetOwner.Enemy or 2) then
    pets = __wow_pet_battle_state.enemyPets
  else
    return nil
  end

  return pets and pets[petIndex] or nil
end

local function __wow_pet_battle_get_ability(owner, petIndex, abilityIndex)
  local pet = __wow_pet_battle_get_pet(owner, petIndex)
  return pet and pet.abilities and pet.abilities[abilityIndex] or nil
end

C_PetBattles._state = __wow_pet_battle_state
C_PetBattles.IsInBattle = function()
  local battleState = C_PetBattles.GetBattleState()
  return battleState ~= 0 and battleState ~= __wow_pet_battle_finished_state
end
C_PetBattles.IsWildBattle = function()
  return C_PetBattles.IsInBattle() and __wow_pet_battle_state.isWildBattle == true
end
C_PetBattles.GetAbilityInfo = function(owner, petIndex, abilityIndex)
  local ability = __wow_pet_battle_get_ability(owner, petIndex, abilityIndex)
  if not ability then
    return nil
  end
  return ability.id, ability.name, ability.icon, ability.maxCooldown, ability.description, ability.numTurns, ability.petType
end
C_PetBattles.GetAbilityInfoByID = function(abilityID)
  __wow_pet_battle_ensure_active()
  local ability = __wow_pet_battle_state.abilitiesByID and __wow_pet_battle_state.abilitiesByID[abilityID]
  if not ability then
    return nil
  end
  return ability.id, ability.name, ability.icon, ability.maxCooldown, ability.description, ability.numTurns, ability.petType
end
C_PetBattles.GetAbilityState = function(owner, petIndex, abilityIndex)
  local ability = __wow_pet_battle_get_ability(owner, petIndex, abilityIndex)
  if not ability then
    return false, 0, 0
  end
  return ability.usable ~= false, ability.cooldown or 0, ability.lockdown or 0
end
C_PetBattles.GetAuraInfo = function(owner, petIndex, auraIndex)
  local pet = __wow_pet_battle_get_pet(owner, petIndex)
  local aura = pet and pet.auras and pet.auras[auraIndex]
  if not aura then
    return nil
  end
  return aura.auraID, aura.instanceID, aura.turnsRemaining, aura.isBuff
end
C_PetBattles.GetNumAuras = function(owner, petIndex)
  local pet = __wow_pet_battle_get_pet(owner, petIndex)
  return pet and pet.auras and #pet.auras or 0
end
C_PetBattles.GetHealth = function(owner, petIndex)
  local pet = __wow_pet_battle_get_pet(owner, petIndex)
  return pet and pet.health or 0
end
C_PetBattles.GetMaxHealth = function(owner, petIndex)
  local pet = __wow_pet_battle_get_pet(owner, petIndex)
  return pet and pet.maxHealth or 0
end
C_PetBattles.GetPower = function(owner, petIndex)
  local pet = __wow_pet_battle_get_pet(owner, petIndex)
  return pet and pet.power or 0
end
C_PetBattles.GetSpeed = function(owner, petIndex)
  local pet = __wow_pet_battle_get_pet(owner, petIndex)
  return pet and pet.speed or 0
end
C_PetBattles.GetLevel = function(owner, petIndex)
  local pet = __wow_pet_battle_get_pet(owner, petIndex)
  return pet and pet.level or 0
end
if C_PetBattles.GetXP == nil then
  C_PetBattles.GetXP = function(owner, petIndex)
    local pet = __wow_pet_battle_get_pet(owner, petIndex)
    if not pet then
      return 0, 0
    end
    return pet.xp or 0, pet.maxXP or 0
  end
end
C_PetBattles.GetAttackModifier = function(attackerType, defenderType)
  if attackerType == 7 and defenderType == 9 then
    return 1.5
  end
  return 1.0
end
C_PetBattles.GetAllStates = function(parserEnv)
  if type(parserEnv) ~= "table" then
    return
  end
  parserEnv.STATE_Stat_Power = 18
end
C_PetBattles.GetPVPMatchmakingInfo = function()
  return __wow_pet_battle_state.queueStatus, __wow_pet_battle_state.queueEstimatedTime, __wow_pet_battle_state.queueTime
end
C_PetBattles.CanAcceptQueuedPVPMatch = function()
  return __wow_pet_battle_state.canAcceptQueuedPVPMatch == true
end
if C_PetBattles.StartPVPMatchmaking == nil then
  C_PetBattles.StartPVPMatchmaking = function()
    __wow_pet_battle_ensure_active()
    __wow_pet_battle_state.queueStatus = Enum.PetBattleQueueStatus and Enum.PetBattleQueueStatus.Matchmaking or 1
    __wow_pet_battle_state.canAcceptQueuedPVPMatch = true
  end
end
C_PetBattles.AcceptQueuedPVPMatch = function()
  __wow_pet_battle_state.queueStatus = Enum.PetBattleQueueStatus and Enum.PetBattleQueueStatus.MatchAccepted or 2
  __wow_pet_battle_state.canAcceptQueuedPVPMatch = false
end
C_PetBattles.GetSelectedAction = function()
  return __wow_pet_battle_state.selectedActionType, __wow_pet_battle_state.selectedActionIndex
end
C_PetBattles.UseAbility = function(abilityIndex)
  __wow_pet_battle_state.selectedActionType = Enum.BattlePetAction and Enum.BattlePetAction.Ability or 1
  __wow_pet_battle_state.selectedActionIndex = abilityIndex
end
C_PetBattles.ChangePet = function(petIndex)
  __wow_pet_battle_state.selectedActionType = Enum.BattlePetAction and Enum.BattlePetAction.SwitchPet or 2
  __wow_pet_battle_state.selectedActionIndex = petIndex
end
C_PetBattles.UseTrap = function()
  __wow_pet_battle_state.selectedActionType = Enum.BattlePetAction and Enum.BattlePetAction.Trap or 3
  __wow_pet_battle_state.selectedActionIndex = nil
end
C_PetBattles.SkipTurn = function()
  __wow_pet_battle_state.selectedActionType = Enum.BattlePetAction and Enum.BattlePetAction.Skip or 4
  __wow_pet_battle_state.selectedActionIndex = nil
end
C_PetBattles.StartPVPDuel = function(unitToken, exactMatch)
  __wow_pet_battle_state.pvpDuel.pending = true
  __wow_pet_battle_state.pvpDuel.challengedUnit = unitToken
  __wow_pet_battle_state.pvpDuel.exactMatch = exactMatch == true
  __wow_pet_battle_state.pvpDuel.accepted = false
end
C_PetBattles.AcceptPVPDuel = function()
  __wow_pet_battle_state.pvpDuel.pending = false
  __wow_pet_battle_state.pvpDuel.accepted = true
end
C_PetBattles.SetPendingReportBattlePetTarget = function(petIndex)
  __wow_pet_battle_state.pendingReportBattlePetTarget = petIndex
end
C_PetBattles.SetPendingReportTargetFromUnit = function(unitToken)
  __wow_pet_battle_state.pendingReportTargetUnit = unitToken
end
C_PetBattles.ForfeitGame = function()
  __wow_pet_battle_state.battleState = __wow_pet_battle_finished_state
end

function HasCompletedAnyAchievement()
  return true
end
function CanShowAchievementUI()
  return true
end

local __wow_store_state = rawget(_G, "__wow_store_state")
if type(__wow_store_state) ~= "table" then
  local featuredGroupID = 501
  local featuredEntryID = 1003
  local featuredProductID = 2003
  local vasServiceType = Enum.VasServiceType and Enum.VasServiceType.NameChange or 1
  local vasDecorator = Enum.BattlepayProductDecorator and Enum.BattlepayProductDecorator.VasService or 0
  local fullCardWithBuy = Enum.BattlepayCardType and Enum.BattlepayCardType.MediumCardWithBuyButton or 0
  local purchasable = Enum.PurchaseEligibility and (Enum.PurchaseEligibility.Ok or Enum.PurchaseEligibility.Purchasable) or 0
  local regionUS = REGION_US or 1

  local featuredEntry = {
    entryID = featuredEntryID,
    productID = featuredProductID,
    sharedData = {
      name = "Apprentice Rider Bundle",
      description = "A seeded store product used for simulator storefront coverage.",
      tooltip = "A seeded store product used for simulator storefront coverage.",
      texture = "Interface\\Icons\\Ability_Mount_RidingHorse",
      productDecorator = vasDecorator,
      cardType = fullCardWithBuy,
      buyableHere = true,
      eligibility = purchasable,
      flags = 0,
      currentDollars = 10,
      currentCents = 0,
      normalDollars = 10,
      normalCents = 0,
      deliverables = {},
      cards = {},
      vasServiceType = vasServiceType,
      canChangeAccount = true,
      canChangeBNetAccount = true,
      boostType = nil,
      instructions = "",
    },
  }

  __wow_store_state = {
    available = true,
    duplicateKey = nil,
    disconnectOnLogout = false,
    failureCode = nil,
    failureReason = nil,
    confirmationProductID = featuredProductID,
    bnetGuid = 3001,
    gameAccounts = { "WoW2", "WoW3" },
    localAccounts = { WoW1 = 1001 },
    remoteAccounts = { WoW2 = 2002, WoW3 = 2003 },
    realms = {
      { virtualRealmAddress = 101, realmName = "Azeroth" },
      { virtualRealmAddress = 202, realmName = "Kalimdor" },
    },
    characters = {
      [101] = {
        { guid = 501001, name = "Simhero", realmName = "Azeroth", wowAccount = 1001, guildMaster = true },
        { guid = 501002, name = "Simshaman", realmName = "Azeroth", wowAccount = 1001, guildMaster = false },
      },
      [202] = {
        { guid = 602001, name = "KalimdorMage", realmName = "Kalimdor", wowAccount = 2002, guildMaster = false },
      },
    },
    productGroups = {
      { groupID = featuredGroupID, parentGroupID = 0 },
    },
    productGroupInfo = {
      [featuredGroupID] = {
        groupName = "Featured",
        texture = "Interface\\Icons\\INV_Misc_Coin_01",
        flags = 0,
        disabledTooltip = nil,
      },
    },
    productsByGroup = {
      [featuredGroupID] = { featuredEntryID },
    },
    entriesByID = {
      [featuredEntryID] = featuredEntry,
    },
    productsByID = {
      [featuredProductID] = featuredEntry,
    },
    currencyInfo = {
      sharedData = {
        regionID = regionUS,
        requireLicenseAccept = false,
        browseHasStar = false,
        hideBrowseNotice = false,
        hideConfirmationBrowseNotice = false,
        licenseAcceptText = "",
        formatShort = function(dollars, cents)
          return string.format("$%d.%02d", dollars or 0, cents or 0)
        end,
        formatLong = function(dollars, cents)
          return string.format("$%d.%02d", dollars or 0, cents or 0)
        end,
      },
    },
    completion = {
      productID = nil,
      guid = nil,
      realmName = nil,
    },
  }
  rawset(_G, "__wow_store_state", __wow_store_state)
end

local function __wow_store_realm_name(virtualRealmAddress)
  for _, realm in ipairs(__wow_store_state.realms) do
    if realm.virtualRealmAddress == virtualRealmAddress then
      return realm.realmName
    end
  end
  return nil
end

local function __wow_store_character_by_guid(guid)
  for _, realmCharacters in pairs(__wow_store_state.characters) do
    for _, character in ipairs(realmCharacters) do
      if character.guid == guid then
        return character
      end
    end
  end
  return nil
end

local function __wow_store_patch_card_enumerator()
  if not StoreFrame or not StoreFrame.productCardPoolCollection then
    return
  end

  local pool = StoreFrame.productCardPoolCollection
  if pool.__wowSimPatched then
    return
  end

  local originalEnumerateActive = pool.EnumerateActive
  pool.__wowSimPatched = true
  function pool:EnumerateActive()
    if type(StoreFrame.__wowSimCards) == "table" and #StoreFrame.__wowSimCards > 0 then
      local cards = StoreFrame.__wowSimCards
      local index = 0
      return function()
        index = index + 1
        return cards[index]
      end, nil, nil
    end
    if type(originalEnumerateActive) == "function" then
      return originalEnumerateActive(self)
    end
    return function()
      return nil
    end, nil, nil
  end
end

local function __wow_store_ensure_debug_cards()
  if not StoreFrame or not StoreFrame.productCardPoolCollection then
    return
  end

  __wow_store_patch_card_enumerator()
  if type(StoreFrame.__wowSimCards) == "table" and #StoreFrame.__wowSimCards > 0 then
    return
  end

  local card = CreateFrame("Button", "WowStoreSimCard1", StoreFrame, "MediumStoreCardWithBuyButtonTemplate")
  if not card then
    return
  end

  card:SetID(1003)
  card:SetPoint("TOPLEFT", StoreFrame, "TOPLEFT", 40, -140)
  card:SetSize(277, 224)
  card:Show()

  if type(card.UpdateCard) == "function" then
    pcall(card.UpdateCard, card, 1003, true)
  end

  StoreFrame.__wowSimCards = { card }
end

C_StoreSecure = __wow_merge_namespace(C_StoreSecure, {})
C_StoreSecure.IsAvailable = function()
  return __wow_store_state.available == true
end
C_StoreSecure.HasPurchaseList = function()
  return true
end
C_StoreSecure.HasProductList = function()
  return true
end
C_StoreSecure.HasDistributionList = function()
  return true
end
C_StoreSecure.HasPurchaseInProgress = function()
  return false
end
C_StoreSecure.GetCurrencyID = function()
  return 1
end
C_StoreSecure.GetCurrencyInfo = function()
  return __wow_store_state.currencyInfo
end
C_StoreSecure.GetPurchaseList = function()
  if StoreFrame and type(StoreFrame.IsShown) == "function" and StoreFrame:IsShown() then
    FireEvent("STORE_PURCHASE_LIST_UPDATED")
  end
  return true
end
C_StoreSecure.GetProductList = function()
  local storeShown = StoreFrame and type(StoreFrame.IsShown) == "function" and StoreFrame:IsShown()
  if storeShown then
    FireEvent("STORE_PRODUCTS_UPDATED")
    FireEvent("PRODUCT_DISTRIBUTIONS_UPDATED")
    if type(StoreFrame_OnEvent) == "function" and StoreFrame then
      StoreFrame_OnEvent(StoreFrame, "STORE_PRODUCTS_UPDATED")
    elseif type(StoreFrame_UpdateSelectedCategory) == "function" then
      StoreFrame_UpdateSelectedCategory()
      if type(StoreFrame_SetCategory) == "function" then
        StoreFrame_SetCategory(true)
      end
    end
    __wow_store_ensure_debug_cards()
  end
  return true
end
C_StoreSecure.GetProductGroups = function()
  return __wow_store_state.productGroups
end
C_StoreSecure.GetProductGroupInfo = function(groupID)
  return __wow_store_state.productGroupInfo[groupID]
end
C_StoreSecure.GetProducts = function(groupID)
  return __wow_store_state.productsByGroup[groupID] or {}
end
C_StoreSecure.GetEntryInfo = function(entryID)
  return __wow_store_state.entriesByID[entryID]
end
C_StoreSecure.GetProductInfo = function(productID)
  return __wow_store_state.productsByID[productID]
end
C_StoreSecure.GetWoWAccountGUIDFromName = function(accountName, isLocalAccount)
  if isLocalAccount then
    return __wow_store_state.localAccounts[accountName]
  end
  return __wow_store_state.remoteAccounts[accountName]
end
C_StoreSecure.ValidateBnetTransfer = function(_email)
  FireEvent("VAS_TRANSFER_VALIDATION_UPDATE", false)
end
C_StoreSecure.GetBnetTransferInfo = function()
  return __wow_store_state.bnetGuid, __wow_store_state.gameAccounts
end
C_StoreSecure.GetRealmList = function()
  return __wow_store_state.realms
end
C_StoreSecure.GetVASRealmList = function()
  return __wow_store_state.realms
end
C_StoreSecure.GetCharactersForRealm = function(virtualRealmAddress, guildOnly)
  local allCharacters = __wow_store_state.characters[virtualRealmAddress] or {}
  if not guildOnly then
    return allCharacters
  end

  local guildCharacters = {}
  for _, character in ipairs(allCharacters) do
    if character.guildMaster then
      table.insert(guildCharacters, character)
    end
  end
  return guildCharacters
end
C_StoreSecure.GetCharacterInfoByGUID = function(guid)
  return __wow_store_character_by_guid(guid)
end
C_StoreSecure.GetEligibleRacesForVASService = function(_guid, _serviceType)
  return {
    { raceID = 1, raceName = "Human", isAlliedRace = false },
    { raceID = 29, raceName = "Void Elf", isAlliedRace = true },
  }
end
C_StoreSecure.GetVASGuildMasterInfoForCharacterByGUID = function(guid)
  if guid == 501001 then
    return {
      guildName = "Simulator Guild",
      guildMasterName = "Simleader",
    }
  end
  return nil
end
C_StoreSecure.GetVasServiceType = function(productID)
  local product = C_StoreSecure.GetProductInfo(productID)
  return product and product.sharedData and product.sharedData.vasServiceType or nil
end
C_StoreSecure.IsRegionLocked = function()
  return false
end
C_StoreSecure.GetLastProductListResponseError = function()
  return 0
end
C_StoreSecure.GetVASErrors = function()
  return {}
end
C_StoreSecure.RequestRealmGuildMasterInfo = function(virtualRealmAddress)
  FireEvent("STORE_GUILD_MASTER_INFO_RECEIVED", virtualRealmAddress)
end
C_StoreSecure.RequestCharacterGuildFollowInfo = function(guid, _virtualRealmAddress)
  FireEvent("STORE_GUILD_FOLLOW_INFO_RECEIVED", guid, { transferredRealm = "Kalimdor" })
end
C_StoreSecure.OpenNydusLink = function(entryID)
  local entry = C_StoreSecure.GetEntryInfo(entryID)
  if entry then
    __wow_store_state.confirmationProductID = entry.productID
  end
end
C_StoreSecure.GetConfirmationInfo = function()
  return __wow_store_state.confirmationProductID, "Blizzard Balance", nil, nil, 10, 0
end
C_StoreSecure.GetUnrevokedBoostInfo = function()
  return "Level 70 Character Boost", "Simhero", "Azeroth"
end
C_StoreSecure.PurchaseVASProduct = function(productID, guid, _newName, _guildName, _guildMasterGuid, destinationRealmAddress)
  local realmName = __wow_store_realm_name(destinationRealmAddress)
  local duplicateKey = string.format("%s:%s:%s", tostring(productID), tostring(guid), tostring(realmName))
  if __wow_store_state.duplicateKey == duplicateKey then
    __wow_store_state.failureCode = Enum.StoreError and Enum.StoreError.Other or 1
    __wow_store_state.failureReason = "DuplicateVASPurchase"
    return false
  end

  __wow_store_state.duplicateKey = duplicateKey
  __wow_store_state.completion.productID = productID
  __wow_store_state.completion.guid = guid
  __wow_store_state.completion.realmName = realmName
  return true
end
C_StoreSecure.GetVASCompletionInfo = function()
  return __wow_store_state.completion.productID, __wow_store_state.completion.guid, __wow_store_state.completion.realmName, __wow_store_state.disconnectOnLogout == true
end
C_StoreSecure.GetFailureInfo = function()
  return __wow_store_state.failureCode, __wow_store_state.failureReason
end
C_StoreSecure.AckFailure = function()
  __wow_store_state.failureCode = nil
  __wow_store_state.failureReason = nil
end
C_StoreSecure.ClearPreGeneratedExternalTransactionID = function()
  __wow_store_state.duplicateKey = nil
end
C_StoreSecure.SetDisconnectOnLogout = function(shouldDisconnect)
  __wow_store_state.disconnectOnLogout = shouldDisconnect == true
end
C_StoreSecure.SetVASProductReady = function(isReady)
  if isReady then
    FireEvent("STORE_VAS_PURCHASE_COMPLETE")
  end
end
C_StoreSecure.RequestAllDynamicPriceInfo = __wow_noop
C_StoreSecure.HasDynamicPriceData = function()
  return true
end
C_StoreSecure.IsDynamicBundle = function()
  return false
end

local __wow_store_public_state = {
  shown = false,
  context_key = nil,
}

local __wow_store_secure_state = {
  available = true,
  has_purchase_list = true,
  has_product_list = true,
  has_distribution_list = true,
  region_locked = false,
  last_product_list_response_error = 0,
  vas_errors = {},
  failure_code = nil,
  failure_reason = nil,
  confirmation_product_id = nil,
  confirmation_wallet_name = "Blizzard Balance",
  confirmation_current_dollars = 10,
  confirmation_current_cents = 0,
  completion_product_id = nil,
  completion_guid = nil,
  completion_realm_name = nil,
  completion_should_handle = false,
  disconnect_on_logout = false,
  purchase_in_progress = false,
  pre_generated_external_transaction_id = false,
  bnet_transfer_guid = 3001,
  bnet_transfer_game_accounts = { "WoW2", "WoW3" },
  bnet_transfer_validated = false,
}

local __wow_store_realms = {
  { realmName = "Azeroth", virtualRealmAddress = 101 },
  { realmName = "Kalimdor", virtualRealmAddress = 102 },
}

local __wow_store_characters = {
  [101] = {
    {
      guid = 501001,
      name = "Simhero",
      realmName = "Azeroth",
      currentServer = 101,
      classFileName = "WARRIOR",
      className = "Warrior",
      level = 70,
      raceName = "Human",
      faction = 0,
      wowAccount = 1001,
      createScreenIconAtlas = "",
    },
    {
      guid = 501002,
      name = "Simalt",
      realmName = "Azeroth",
      currentServer = 101,
      classFileName = "MAGE",
      className = "Mage",
      level = 70,
      raceName = "Void Elf",
      faction = 1,
      wowAccount = 1002,
      createScreenIconAtlas = "",
    },
  },
  [102] = {
    {
      guid = 502001,
      name = "KalimdorHero",
      realmName = "Kalimdor",
      currentServer = 102,
      classFileName = "PRIEST",
      className = "Priest",
      level = 70,
      raceName = "Night Elf",
      faction = 1,
      wowAccount = 2001,
      createScreenIconAtlas = "",
    },
  },
}

local __wow_store_guild_master_info = {
  [501001] = {
    guildName = "Simulator Guild",
    guildMasterName = "Simleader",
    guildMasterGuid = 501001,
  },
}

local __wow_store_product_groups = {
  {
    groupID = 22,
    parentGroupID = nil,
    groupName = "Services",
    texture = "Interface\\Icons\\INV_Misc_QuestionMark",
    flags = 0,
    disabledTooltip = nil,
  },
}

local __wow_store_products = {
  [2003] = {
    productID = 2003,
    sharedData = {
      name = "Apprentice Rider Bundle",
      description = "Simulator store product.",
      tooltip = "",
      texture = "Interface\\Icons\\INV_Misc_Note_02",
      productDecorator = Enum.BattlepayProductDecorator.VasService,
      vasServiceType = Enum.VasServiceType.NameChange,
      cardType = Enum.BattlepayCardType.MediumCardWithBuyButton,
      flags = 0,
      eligibility = Enum.PurchaseEligibility.Ok,
      buyableHere = true,
      currentDollars = 10,
      currentCents = 0,
      normalDollars = 10,
      normalCents = 0,
      instructions = "",
      canChangeAccount = true,
      canChangeBNetAccount = true,
      canChangeRealm = true,
      deliverables = {},
      cards = {},
    },
  },
  [189] = {
    productID = 189,
    sharedData = {
      name = "Character Transfer",
      description = "Simulator character transfer.",
      tooltip = "",
      texture = "Interface\\Icons\\INV_Misc_Note_02",
      productDecorator = Enum.BattlepayProductDecorator.VasService,
      vasServiceType = Enum.VasServiceType.CharacterTransfer,
      cardType = Enum.BattlepayCardType.MediumCardWithBuyButton,
      flags = 0,
      eligibility = Enum.PurchaseEligibility.Ok,
      buyableHere = true,
      currentDollars = 25,
      currentCents = 0,
      normalDollars = 25,
      normalCents = 0,
      instructions = "",
      canChangeAccount = true,
      canChangeBNetAccount = true,
      canChangeRealm = true,
      deliverables = {},
      cards = {},
    },
  },
  [239] = {
    productID = 239,
    sharedData = {
      name = "Character Transfer Bundle",
      description = "Simulator transfer bundle.",
      tooltip = "",
      texture = "Interface\\Icons\\INV_Misc_Note_02",
      productDecorator = Enum.BattlepayProductDecorator.VasService,
      vasServiceType = Enum.VasServiceType.CharacterTransfer,
      cardType = Enum.BattlepayCardType.MediumCardWithBuyButton,
      flags = 0,
      eligibility = Enum.PurchaseEligibility.Ok,
      buyableHere = true,
      currentDollars = 25,
      currentCents = 0,
      normalDollars = 25,
      normalCents = 0,
      instructions = "",
      canChangeAccount = true,
      canChangeBNetAccount = true,
      canChangeRealm = true,
      deliverables = {},
      cards = {},
    },
  },
  [476] = {
    productID = 476,
    sharedData = {
      name = "Guild Transfer",
      description = "Simulator guild transfer.",
      tooltip = "",
      texture = "Interface\\Icons\\INV_Misc_Note_02",
      productDecorator = Enum.BattlepayProductDecorator.VasService,
      vasServiceType = Enum.VasServiceType.GuildTransfer,
      cardType = Enum.BattlepayCardType.MediumCardWithBuyButton,
      flags = 0,
      eligibility = Enum.PurchaseEligibility.Ok,
      buyableHere = true,
      currentDollars = 35,
      currentCents = 0,
      normalDollars = 35,
      normalCents = 0,
      instructions = "",
      canChangeAccount = true,
      canChangeBNetAccount = true,
      canChangeRealm = true,
      deliverables = {},
      cards = {},
    },
  },
  [477] = {
    productID = 477,
    sharedData = {
      name = "Guild Transfer Bundle",
      description = "Simulator guild transfer bundle.",
      tooltip = "",
      texture = "Interface\\Icons\\INV_Misc_Note_02",
      productDecorator = Enum.BattlepayProductDecorator.VasService,
      vasServiceType = Enum.VasServiceType.GuildTransfer,
      cardType = Enum.BattlepayCardType.MediumCardWithBuyButton,
      flags = 0,
      eligibility = Enum.PurchaseEligibility.Ok,
      buyableHere = true,
      currentDollars = 35,
      currentCents = 0,
      normalDollars = 35,
      normalCents = 0,
      instructions = "",
      canChangeAccount = true,
      canChangeBNetAccount = true,
      canChangeRealm = true,
      deliverables = {},
      cards = {},
    },
  },
}

local function __wow_store_realm_name(address)
  if address == 101 then
    return "Azeroth"
  elseif address == 102 then
    return "Kalimdor"
  end
  return tostring(address or "")
end

local function __wow_store_find_character(guid)
  for _, realmCharacters in pairs(__wow_store_characters) do
    for _, character in ipairs(realmCharacters) do
      if character.guid == guid then
        return character
      end
    end
  end
  return nil
end

local function __wow_store_product(productID)
  return __wow_store_products[tonumber(productID) or -1]
end

-- Store / shop public API: sim has no store.
C_StorePublic = C_StorePublic or __wow_namespace()
if rawget(C_StorePublic, "IsEnabled") == nil then
  function C_StorePublic.IsEnabled() return true end
end
if rawget(C_StorePublic, "IsDisabledByParentalControls") == nil then
  function C_StorePublic.IsDisabledByParentalControls() return false end
end
if rawget(C_StorePublic, "DoesGroupHavePurchaseableProducts") == nil then
  function C_StorePublic.DoesGroupHavePurchaseableProducts(groupID)
    local products = C_StoreSecure and C_StoreSecure.GetProducts and C_StoreSecure.GetProducts(groupID) or {}
    return #products > 0
  end
end
if rawget(C_StorePublic, "EventStoreUISetShown") == nil then
  function C_StorePublic.EventStoreUISetShown(shown, contextKey)
    __wow_store_public_state.shown = shown and true or false
    __wow_store_public_state.context_key = contextKey
  end
end

C_StoreSecure = C_StoreSecure or __wow_namespace()
if rawget(C_StoreSecure, "_state") == nil then
  C_StoreSecure._state = __wow_store_secure_state
end
if rawget(C_StoreSecure, "IsAvailable") == nil then
  function C_StoreSecure.IsAvailable() return C_StoreSecure._state.available end
end
if rawget(C_StoreSecure, "HasPurchaseList") == nil then
  function C_StoreSecure.HasPurchaseList() return C_StoreSecure._state.has_purchase_list end
end
if rawget(C_StoreSecure, "HasProductList") == nil then
  function C_StoreSecure.HasProductList() return C_StoreSecure._state.has_product_list end
end
if rawget(C_StoreSecure, "HasDistributionList") == nil then
  function C_StoreSecure.HasDistributionList() return C_StoreSecure._state.has_distribution_list end
end
if rawget(C_StoreSecure, "HasPurchaseInProgress") == nil then
  function C_StoreSecure.HasPurchaseInProgress() return C_StoreSecure._state.purchase_in_progress end
end
if rawget(C_StoreSecure, "IsRegionLocked") == nil then
  function C_StoreSecure.IsRegionLocked() return C_StoreSecure._state.region_locked end
end
if rawget(C_StoreSecure, "GetLastProductListResponseError") == nil then
  function C_StoreSecure.GetLastProductListResponseError() return C_StoreSecure._state.last_product_list_response_error end
end
if rawget(C_StoreSecure, "GetVASErrors") == nil then
  function C_StoreSecure.GetVASErrors() return C_StoreSecure._state.vas_errors end
end
if rawget(C_StoreSecure, "GetCurrencyInfo") == nil then
  function C_StoreSecure.GetCurrencyInfo()
    return {
      sharedData = {
        regionID = 1,
        formatShort = "%s",
        formatLong = "%s",
        licenseAcceptText = "",
        requireLicenseAccept = false,
        browseHasStar = false,
        hideBrowseNotice = false,
        hideConfirmationBrowseNotice = false,
      },
    }
  end
end
if rawget(C_StoreSecure, "GetProductGroups") == nil then
  function C_StoreSecure.GetProductGroups() return __wow_store_product_groups end
end
if rawget(C_StoreSecure, "GetProductGroupInfo") == nil then
  function C_StoreSecure.GetProductGroupInfo(groupID)
    for _, group in ipairs(__wow_store_product_groups) do
      if group.groupID == groupID then
        return group
      end
    end
    return nil
  end
end
if rawget(C_StoreSecure, "GetProducts") == nil then
  function C_StoreSecure.GetProducts(groupID)
    if groupID == 22 then
      return { 2003, 189, 239, 476, 477 }
    end
    return {}
  end
end
if rawget(C_StoreSecure, "GetEntryInfo") == nil then
  function C_StoreSecure.GetEntryInfo(entryID) return __wow_store_product(entryID) end
end
if rawget(C_StoreSecure, "GetProductInfo") == nil then
  function C_StoreSecure.GetProductInfo(productID) return __wow_store_product(productID) end
end
if rawget(C_StoreSecure, "IsDynamicBundle") == nil then
  function C_StoreSecure.IsDynamicBundle(_productID) return false end
end
if rawget(C_StoreSecure, "HasDynamicPriceData") == nil then
  function C_StoreSecure.HasDynamicPriceData(_productID) return true end
end
if rawget(C_StoreSecure, "RequestAllDynamicPriceInfo") == nil then
  function C_StoreSecure.RequestAllDynamicPriceInfo() return nil end
end
if rawget(C_StoreSecure, "GetProductList") == nil then
  function C_StoreSecure.GetProductList()
    __wow_store_secure_state.has_product_list = true
    FireEvent("STORE_PRODUCTS_UPDATED")
    return nil
  end
end
if rawget(C_StoreSecure, "GetPurchaseList") == nil then
  function C_StoreSecure.GetPurchaseList()
    __wow_store_secure_state.has_purchase_list = true
    FireEvent("STORE_PURCHASE_LIST_UPDATED")
    return nil
  end
end
if rawget(C_StoreSecure, "GetDistributionList") == nil then
  function C_StoreSecure.GetDistributionList()
    __wow_store_secure_state.has_distribution_list = true
    return {}
  end
end
if rawget(C_StoreSecure, "GetFailureInfo") == nil then
  function C_StoreSecure.GetFailureInfo()
    return C_StoreSecure._state.failure_code, C_StoreSecure._state.failure_reason
  end
end
if rawget(C_StoreSecure, "AckFailure") == nil then
  function C_StoreSecure.AckFailure()
    C_StoreSecure._state.failure_code = nil
    C_StoreSecure._state.failure_reason = nil
  end
end
if rawget(C_StoreSecure, "ClearPreGeneratedExternalTransactionID") == nil then
  function C_StoreSecure.ClearPreGeneratedExternalTransactionID()
    C_StoreSecure._state.pre_generated_external_transaction_id = false
  end
end
if rawget(C_StoreSecure, "OpenNydusLink") == nil then
  function C_StoreSecure.OpenNydusLink(productID)
    local normalized = tonumber(productID) or 0
    if normalized == 1003 then
      normalized = 2003
    end
    local product = __wow_store_product(normalized)
    if product then
      C_StoreSecure._state.confirmation_product_id = normalized
      C_StoreSecure._state.confirmation_wallet_name = "Blizzard Balance"
      C_StoreSecure._state.confirmation_current_dollars = product.sharedData.currentDollars
      C_StoreSecure._state.confirmation_current_cents = product.sharedData.currentCents
    end
  end
end
if rawget(C_StoreSecure, "GetConfirmationInfo") == nil then
  function C_StoreSecure.GetConfirmationInfo()
    return C_StoreSecure._state.confirmation_product_id, C_StoreSecure._state.confirmation_wallet_name, nil, nil, C_StoreSecure._state.confirmation_current_dollars, C_StoreSecure._state.confirmation_current_cents
  end
end
if rawget(C_StoreSecure, "GetUnrevokedBoostInfo") == nil then
  function C_StoreSecure.GetUnrevokedBoostInfo()
    return "Level 70 Character Boost", "Simhero", "Azeroth"
  end
end
if rawget(C_StoreSecure, "GetVASCompletionInfo") == nil then
  function C_StoreSecure.GetVASCompletionInfo()
    return C_StoreSecure._state.completion_product_id, C_StoreSecure._state.completion_guid, C_StoreSecure._state.completion_realm_name, C_StoreSecure._state.completion_should_handle
  end
end
if rawget(C_StoreSecure, "SetDisconnectOnLogout") == nil then
  function C_StoreSecure.SetDisconnectOnLogout(disconnectOnLogout)
    C_StoreSecure._state.disconnect_on_logout = disconnectOnLogout and true or false
    if C_StoreSecure._state.completion_product_id then
      C_StoreSecure._state.completion_should_handle = C_StoreSecure._state.disconnect_on_logout
    end
  end
end
if rawget(C_StoreSecure, "SetVASProductReady") == nil then
  function C_StoreSecure.SetVASProductReady(ready)
    if ready and C_StoreSecure._state.completion_product_id then
      C_StoreSecure._state.purchase_in_progress = false
      FireEvent("STORE_VAS_PURCHASE_COMPLETE")
    end
  end
end
if rawget(C_StoreSecure, "PurchaseVASProduct") == nil then
  function C_StoreSecure.PurchaseVASProduct(productID, guid, _name, _oldGuildName, _newGuildMasterGuid, realmValue, _wowAccountGuid, _bnetAccountGuid, _transferFactionChangeBundle, _isGuildFollow)
    if C_StoreSecure._state.completion_product_id and C_StoreSecure._state.pre_generated_external_transaction_id then
      C_StoreSecure._state.failure_code = Enum.StoreError.Other
      C_StoreSecure._state.failure_reason = "DuplicateVASPurchase"
      return false
    end

    local product = __wow_store_product(productID)
    if not product then
      C_StoreSecure._state.failure_code = Enum.StoreError.Other
      C_StoreSecure._state.failure_reason = "UnknownVASProduct"
      return false
    end

    C_StoreSecure._state.confirmation_product_id = productID
    C_StoreSecure._state.confirmation_wallet_name = "Blizzard Balance"
    C_StoreSecure._state.confirmation_current_dollars = product.sharedData.currentDollars
    C_StoreSecure._state.confirmation_current_cents = product.sharedData.currentCents
    C_StoreSecure._state.completion_product_id = productID
    C_StoreSecure._state.completion_guid = guid
    C_StoreSecure._state.completion_realm_name = __wow_store_realm_name(realmValue)
    C_StoreSecure._state.completion_should_handle = C_StoreSecure._state.disconnect_on_logout
    C_StoreSecure._state.purchase_in_progress = true
    C_StoreSecure._state.pre_generated_external_transaction_id = true
    C_StoreSecure._state.failure_code = nil
    C_StoreSecure._state.failure_reason = nil
    return true
  end
end
if rawget(C_StoreSecure, "PurchaseProduct") == nil then
  function C_StoreSecure.PurchaseProduct(productID)
    return C_StoreSecure.PurchaseVASProduct(productID, 0, nil, nil, nil, 101, nil, nil, false, false)
  end
end
if rawget(C_StoreSecure, "PurchaseProductConfirm") == nil then
  function C_StoreSecure.PurchaseProductConfirm(confirm, _dollars, _cents)
    if confirm and C_StoreSecure._state.completion_product_id then
      C_StoreSecure._state.purchase_in_progress = false
      FireEvent("STORE_VAS_PURCHASE_COMPLETE")
    end
    return true
  end
end
if rawget(C_StoreSecure, "ValidateBnetTransfer") == nil then
  function C_StoreSecure.ValidateBnetTransfer(_email)
    C_StoreSecure._state.bnet_transfer_validated = true
    FireEvent("VAS_TRANSFER_VALIDATION_UPDATE", false)
  end
end
if rawget(C_StoreSecure, "GetBnetTransferInfo") == nil then
  function C_StoreSecure.GetBnetTransferInfo()
    return C_StoreSecure._state.bnet_transfer_guid, C_StoreSecure._state.bnet_transfer_game_accounts
  end
end
if rawget(C_StoreSecure, "GetWoWAccountGUIDFromName") == nil then
  function C_StoreSecure.GetWoWAccountGUIDFromName(name, isLocal)
    if isLocal and name == "WoW1" then
      return 1001
    elseif not isLocal and name == "WoW2" then
      return 2002
    end
    return nil
  end
end
if rawget(C_StoreSecure, "GetRealmList") == nil then
  function C_StoreSecure.GetRealmList() return __wow_store_realms end
end
if rawget(C_StoreSecure, "GetVASRealmList") == nil then
  function C_StoreSecure.GetVASRealmList() return __wow_store_realms end
end
if rawget(C_StoreSecure, "GetCharactersForRealm") == nil then
  function C_StoreSecure.GetCharactersForRealm(realmAddress, guildOnly)
    local realmCharacters = __wow_store_characters[tonumber(realmAddress) or -1] or {}
    local filtered = {}
    for _, character in ipairs(realmCharacters) do
      if not guildOnly or character.guid == 501001 then
        table.insert(filtered, character)
      end
    end
    return filtered
  end
end
if rawget(C_StoreSecure, "GetCharacterInfoByGUID") == nil then
  function C_StoreSecure.GetCharacterInfoByGUID(guid)
    return __wow_store_find_character(tonumber(guid) or -1)
  end
end
if rawget(C_StoreSecure, "GetEligibleRacesForVASService") == nil then
  function C_StoreSecure.GetEligibleRacesForVASService(_characterGuid, vasServiceType)
    if vasServiceType == Enum.VasServiceType.NameChange then
      return {
        { raceName = "Human", isAlliedRace = false, isHeritageArmorUnlocked = true },
        { raceName = "Void Elf", isAlliedRace = true, isHeritageArmorUnlocked = true },
      }
    end
    return {}
  end
end
if rawget(C_StoreSecure, "GetVASGuildMasterInfoForCharacterByGUID") == nil then
  function C_StoreSecure.GetVASGuildMasterInfoForCharacterByGUID(guid)
    return __wow_store_guild_master_info[tonumber(guid) or -1]
  end
end
if rawget(C_StoreSecure, "GetVasServiceType") == nil then
  function C_StoreSecure.GetVasServiceType(productID)
    local normalized = tonumber(productID) or -1
    if normalized == 2003 then
      return Enum.VasServiceType.NameChange
    elseif normalized == 189 or normalized == 239 then
      return Enum.VasServiceType.CharacterTransfer
    elseif normalized == 476 or normalized == 477 then
      return Enum.VasServiceType.GuildTransfer
    end
    return nil
  end
end
if rawget(C_StoreSecure, "RequestRealmGuildMasterInfo") == nil then
  function C_StoreSecure.RequestRealmGuildMasterInfo(realmAddress)
    FireEvent("STORE_GUILD_MASTER_INFO_RECEIVED", realmAddress)
  end
end
if rawget(C_StoreSecure, "RequestCharacterGuildFollowInfo") == nil then
  function C_StoreSecure.RequestCharacterGuildFollowInfo(guid, realmAddress)
    FireEvent("STORE_GUILD_FOLLOW_INFO_RECEIVED", guid, { transferredRealm = __wow_store_realm_name(realmAddress) })
  end
end
if rawget(C_StoreSecure, "AckFailure") == nil then
  function C_StoreSecure.AckFailure()
    C_StoreSecure._state.failure_code = nil
    C_StoreSecure._state.failure_reason = nil
  end
end
if rawget(C_StoreSecure, "ClearPreGeneratedExternalTransactionID") == nil then
  function C_StoreSecure.ClearPreGeneratedExternalTransactionID()
    C_StoreSecure._state.pre_generated_external_transaction_id = false
  end
end

-- GetAvailableLocaleInfo is registered from Rust
-- (src/lua_api/globals/locale_info.rs). Returns the 12-locale retail list
-- as { localeId, localeName, englishName, displayName } entries.
-- GuildControlSetRank / GuildControlGetRankName / GuildControlGetNumRanks /
-- GuildControlGetRankFlags are registered from Rust
-- (src/lua_api/globals/guild_control.rs), backed by SimState::world.guild_ranks.
-- Admin: A_Admin.SetGuildRanks({ {name="Leader", flags={true,...}}, ... }).
if C_EditMode == nil then
  C_EditMode = __wow_namespace()
end
if rawget(C_EditMode, "GetAccountSettings") == nil then
  local function __wow_copy_edit_mode_value(value)
    if type(value) ~= "table" then
      return value
    end

    local copy = {}
    for key, child in pairs(value) do
      copy[__wow_copy_edit_mode_value(key)] = __wow_copy_edit_mode_value(child)
    end
    return copy
  end

  local function __wow_default_edit_mode_account_setting(setting)
    if setting == Enum.EditModeAccountSetting.ShowGrid then
      return 0
    elseif setting == Enum.EditModeAccountSetting.GridSpacing then
      return Constants.EditModeConsts.EditModeDefaultGridSpacing or 100
    elseif setting == Enum.EditModeAccountSetting.SettingsExpanded then
      return 0
    elseif setting == Enum.EditModeAccountSetting.EnableAdvancedOptions then
      return 0
    end
    return 1
  end

  local __wow_edit_mode_layout_state = {
    layouts = {},
    activeLayout = 1,
  }
  local __wow_edit_mode_account_setting_state = nil

  local function __wow_build_default_edit_mode_account_settings()
    local settings = {}
    for _, setting in pairs(Enum.EditModeAccountSetting or {}) do
      if type(setting) == "number" then
        table.insert(settings, {
          setting = setting,
          value = __wow_default_edit_mode_account_setting(setting),
        })
      end
    end
    table.sort(settings, function(a, b) return a.setting < b.setting end)
    return settings
  end

  local function __wow_merge_edit_mode_account_settings(accountSettings)
    local merged = __wow_build_default_edit_mode_account_settings()
    local bySetting = {}
    for _, settingInfo in ipairs(merged) do
      bySetting[settingInfo.setting] = settingInfo
    end

    for _, settingInfo in ipairs(accountSettings or {}) do
      local existing = bySetting[settingInfo.setting]
      if existing then
        existing.value = settingInfo.value
      else
        table.insert(merged, {
          setting = settingInfo.setting,
          value = settingInfo.value,
        })
      end
    end

    table.sort(merged, function(a, b) return a.setting < b.setting end)
    return merged
  end

  local __wow_edit_mode_frame_points = {
    [0] = "TOPLEFT",
    [1] = "TOP",
    [2] = "TOPRIGHT",
    [3] = "LEFT",
    [4] = "CENTER",
    [5] = "RIGHT",
    [6] = "BOTTOMLEFT",
    [7] = "BOTTOM",
    [8] = "BOTTOMRIGHT",
  }

  local function __wow_edit_mode_tokens(text)
    local tokens = {}
    if type(text) ~= "string" then
      return tokens
    end
    text = string.gsub(text, "%z", "")
    for token in string.gmatch(text, "%S+") do
      table.insert(tokens, token)
    end
    return tokens
  end

  local function __wow_edit_mode_read(tokens, cursor)
    return tokens[cursor], cursor + 1
  end

  local function __wow_edit_mode_read_number(tokens, cursor, fallback)
    local token
    token, cursor = __wow_edit_mode_read(tokens, cursor)
    return tonumber(token) or fallback or 0, cursor
  end

  local function __wow_edit_mode_decode_settings(encoded)
    local settings = {}
    if type(encoded) ~= "string" then
      return settings
    end
    local lastSetting = nil
    local lastInfo = nil
    local placeValue = 1
    for i = 1, string.len(encoded), 2 do
      local settingByte = string.byte(encoded, i)
      local valueByte = string.byte(encoded, i + 1)
      if settingByte and valueByte then
        local setting = settingByte - 35
        local valueChunk = valueByte - 35
        if setting == lastSetting and lastInfo then
          placeValue = placeValue * 90
          lastInfo.value = lastInfo.value + (valueChunk * placeValue)
        else
          lastInfo = {
            setting = setting,
            value = valueChunk,
          }
          table.insert(settings, lastInfo)
          lastSetting = setting
          placeValue = 1
        end
      end
    end
    return settings
  end

  local function __wow_edit_mode_parse_system(tokens, cursor)
    local system, systemIndex, isInDefaultPosition, point, relativePoint
    local relativeTo, offsetX, offsetY, settingsText
    system, cursor = __wow_edit_mode_read_number(tokens, cursor)
    systemIndex, cursor = __wow_edit_mode_read_number(tokens, cursor, -1)
    if systemIndex >= 0 then
      systemIndex = systemIndex + 1
    end
    isInDefaultPosition, cursor = __wow_edit_mode_read_number(tokens, cursor)
    point, cursor = __wow_edit_mode_read_number(tokens, cursor)
    relativePoint, cursor = __wow_edit_mode_read_number(tokens, cursor)
    relativeTo, cursor = __wow_edit_mode_read(tokens, cursor)
    offsetX, cursor = __wow_edit_mode_read_number(tokens, cursor)
    offsetY, cursor = __wow_edit_mode_read_number(tokens, cursor)
    _, cursor = __wow_edit_mode_read(tokens, cursor)
    settingsText, cursor = __wow_edit_mode_read(tokens, cursor)

    return {
      system = system,
      systemIndex = systemIndex,
      isInDefaultPosition = isInDefaultPosition ~= 0,
      anchorInfo = {
        point = __wow_edit_mode_frame_points[point] or "CENTER",
        relativeTo = relativeTo or "UIParent",
        relativePoint = __wow_edit_mode_frame_points[relativePoint] or "CENTER",
        offsetX = offsetX,
        offsetY = offsetY,
      },
      settings = __wow_edit_mode_decode_settings(settingsText),
    }, cursor
  end

  local function __wow_edit_mode_parse_account_cache(text)
    local tokens = __wow_edit_mode_tokens(text)
    local cursor = 1
    local layoutCount, accountSettingCount
    layoutCount, cursor = __wow_edit_mode_read_number(tokens, cursor)
    accountSettingCount, cursor = __wow_edit_mode_read_number(tokens, cursor)

    local accountSettings = {}
    for setting = 0, accountSettingCount - 1 do
      local value
      value, cursor = __wow_edit_mode_read_number(tokens, cursor)
      table.insert(accountSettings, { setting = setting, value = value })
    end

    local layouts = {}
    for _ = 1, layoutCount do
      local layoutIndex, layoutName, systemCount
      layoutIndex, cursor = __wow_edit_mode_read_number(tokens, cursor)
      layoutName, cursor = __wow_edit_mode_read(tokens, cursor)
      systemCount, cursor = __wow_edit_mode_read_number(tokens, cursor)
      local systems = {}
      for systemIndex = 1, systemCount do
        systems[systemIndex], cursor = __wow_edit_mode_parse_system(tokens, cursor)
      end
      table.insert(layouts, {
        layoutIndex = layoutIndex,
        layoutName = layoutName or "",
        layoutType = Enum.EditModeLayoutType.Account,
        systems = systems,
      })
    end

    return layouts, accountSettings
  end

  local function __wow_edit_mode_active_layout_from_character_cache(text, activeSpecIndex)
    local tokens = __wow_edit_mode_tokens(text)
    local active = tonumber(tokens[activeSpecIndex or 1])
    if active and active > 0 then
      return active
    end
    for _, token in ipairs(tokens) do
      active = tonumber(token)
      if active and active > 0 then
        return active
      end
    end
    return nil
  end

  local function __wow_edit_mode_active_layout_from_override(layouts, preferredLayout)
    if type(preferredLayout) ~= "string" or preferredLayout == "" then
      return nil
    end

    local preferredIndex = tonumber(preferredLayout)
    if preferredIndex and preferredIndex > 0 then
      return preferredIndex
    end

    for index, layout in ipairs(layouts or {}) do
      if layout.layoutName == preferredLayout then
        return index
      end
    end

    local loweredPreferredLayout = string.lower(preferredLayout)
    for index, layout in ipairs(layouts or {}) do
      local layoutName = type(layout.layoutName) == "string" and layout.layoutName or ""
      if string.lower(layoutName) == loweredPreferredLayout then
        return index
      end
    end

    return nil
  end

  function C_EditMode.GetAccountSettings()
    if __wow_edit_mode_account_setting_state == nil then
      __wow_edit_mode_account_setting_state = __wow_build_default_edit_mode_account_settings()
    end
    return __wow_copy_edit_mode_value(__wow_edit_mode_account_setting_state)
  end

  function C_EditMode.GetLayouts()
    return __wow_copy_edit_mode_value(__wow_edit_mode_layout_state)
  end

  function C_EditMode.SaveLayouts(saveInfo)
    if type(saveInfo) ~= "table" then
      return
    end

    __wow_edit_mode_layout_state = {
      layouts = __wow_copy_edit_mode_value(saveInfo.layouts or {}),
      activeLayout = saveInfo.activeLayout or __wow_edit_mode_layout_state.activeLayout or 1,
    }
  end

  function C_EditMode.SetActiveLayout(layoutIndex)
    if type(layoutIndex) == "number" then
      __wow_edit_mode_layout_state.activeLayout = layoutIndex
    end
  end

  function C_EditMode.SetAccountSetting(setting, value)
    if __wow_edit_mode_account_setting_state == nil then
      __wow_edit_mode_account_setting_state = __wow_build_default_edit_mode_account_settings()
    end
    for _, settingInfo in ipairs(__wow_edit_mode_account_setting_state) do
      if settingInfo.setting == setting then
        settingInfo.value = value
        return
      end
    end
    table.insert(__wow_edit_mode_account_setting_state, { setting = setting, value = value })
    table.sort(__wow_edit_mode_account_setting_state, function(a, b) return a.setting < b.setting end)
  end

  function C_EditMode.__LoadCache(accountCache, characterCache, activeSpecIndex, preferredLayout)
    local layouts, accountSettings = __wow_edit_mode_parse_account_cache(accountCache)
    local activeLayout = __wow_edit_mode_active_layout_from_character_cache(characterCache, activeSpecIndex)
    activeLayout = __wow_edit_mode_active_layout_from_override(layouts, preferredLayout) or activeLayout
    __wow_edit_mode_layout_state = {
      layouts = layouts,
      activeLayout = activeLayout or __wow_edit_mode_layout_state.activeLayout or 1,
    }
    if #accountSettings > 0 then
      __wow_edit_mode_account_setting_state = __wow_merge_edit_mode_account_settings(accountSettings)
    end
  end
end
local function __wow_copy_mixin_methods(target, source)
  if type(target) ~= "table" or type(source) ~= "table" then
    return target
  end
  for key, value in pairs(source) do
    if rawget(target, key) == nil then
      rawset(target, key, value)
    end
  end
  return target
end

if type(DropdownSelectionTextMixin) ~= "table" then
  DropdownSelectionTextMixin = {}

  function DropdownSelectionTextMixin:SetDefaultText(text)
    self.defaultText = text
  end

  function DropdownSelectionTextMixin:SetSelectionTranslator(translator)
    self.selectionTranslator = translator
  end

  function DropdownSelectionTextMixin:SetSelectionText(selectionFunc)
    self.selectionFunc = selectionFunc
  end

  function DropdownSelectionTextMixin:UpdateToMenuSelections(menuDescription, currentSelections)
    if self.disableSelectionText then
      return
    end
    local text = nil
    if type(self.selectionFunc) == "function" then
      text = self.selectionFunc(currentSelections or {})
    end
    if text == nil then
      text = self.defaultText
    end
    if text ~= nil and type(self.SetText) == "function" then
      self:SetText(text)
    end
  end

  function DropdownSelectionTextMixin:OnShow()
    if type(self.GenerateMenu) == "function" then
      self:GenerateMenu()
    end
  end
end

if type(WowStyle1DropdownMixin) ~= "table" then
  WowStyle1DropdownMixin = __wow_copy_mixin_methods({}, DropdownButtonMixin)

  function WowStyle1DropdownMixin:OnLoad()
    DropdownButtonMixin.OnLoad(self)
  end

  function WowStyle1DropdownMixin:OnButtonStateChanged() end
  function WowStyle1DropdownMixin:GetArrowAtlas() return nil end
end
__wow_copy_mixin_methods(WowStyle1DropdownMixin, DropdownSelectionTextMixin)

if type(WowStyle1FilterDropdownMixin) ~= "table" then
  WowStyle1FilterDropdownMixin = __wow_copy_mixin_methods({}, WowStyle1DropdownMixin)
end
__wow_copy_mixin_methods(WowStyle1FilterDropdownMixin, WowStyle1DropdownMixin)
__wow_copy_mixin_methods(WowStyle1FilterDropdownMixin, DropdownSelectionTextMixin)

if type(WowStyle1ArrowDropdownMixin) ~= "table" then
  WowStyle1ArrowDropdownMixin = __wow_copy_mixin_methods({}, WowStyle1DropdownMixin)
end
__wow_copy_mixin_methods(WowStyle1ArrowDropdownMixin, WowStyle1DropdownMixin)
__wow_copy_mixin_methods(WowStyle1ArrowDropdownMixin, DropdownSelectionTextMixin)

if type(WowDropdownFilterBehaviorMixin) ~= "table" then
  WowDropdownFilterBehaviorMixin = {}

  function WowDropdownFilterBehaviorMixin:OnLoad()
    if type(self.SetSelectionText) ~= "function" and DropdownButtonMixin ~= nil then
      self.SetSelectionText = DropdownButtonMixin.SetSelectionText
      self.GetSelectionText = DropdownButtonMixin.GetSelectionText
    end
  end

  function WowDropdownFilterBehaviorMixin:OnShow() end
  function WowDropdownFilterBehaviorMixin:SetDefaultCallback(callback)
    self.__wow_default_callback = callback
  end
  function WowDropdownFilterBehaviorMixin:SetIsDefaultCallback(callback)
    self.__wow_is_default_callback = callback
  end
  function WowDropdownFilterBehaviorMixin:SetUpdateCallback(callback)
    self.__wow_update_callback = callback
  end
  function WowDropdownFilterBehaviorMixin:NotifyUpdate(description)
    if type(self.__wow_update_callback) == "function" then
      self.__wow_update_callback(description)
    end
  end
  function WowDropdownFilterBehaviorMixin:Reset() end
  function WowDropdownFilterBehaviorMixin:ValidateResetState() end
  function WowDropdownFilterBehaviorMixin:OnMenuResponse(menu, description)
    self:NotifyUpdate(description)
  end
  function WowDropdownFilterBehaviorMixin:OnMenuAssigned() end
end

if type(WowFilterButtonMixin) ~= "table" then
  WowFilterButtonMixin = __wow_copy_mixin_methods({}, WowDropdownFilterBehaviorMixin)
end
__wow_copy_mixin_methods(WowFilterButtonMixin, WowDropdownFilterBehaviorMixin)
__wow_copy_mixin_methods(WowFilterButtonMixin, DropdownSelectionTextMixin)

local function __wow_ensure_achievement_search_previews()
  local frame = AchievementFrame
  local container = frame and frame.SearchPreviewContainer
  if type(container) ~= "table" and type(container) ~= "userdata" then
    return
  end

  local previews = container.searchPreviews
  if type(previews) ~= "table" then
    previews = {}
    container.searchPreviews = previews
  end

  local count = ACHIEVEMENT_FRAME_NUM_SEARCH_PREVIEWS or 5
  for index = 1, count do
    if previews[index] == nil then
      previews[index] = container["SearchPreview" .. index]
    end
  end
end

local function __wow_patch_achievement_search_preview_selection()
  if rawget(_G, "__wow_achievement_search_preview_patched") then
    return
  end
  if type(AchievementFrame_SetSearchPreviewSelection) ~= "function" then
    return
  end

  local original = AchievementFrame_SetSearchPreviewSelection
  AchievementFrame_SetSearchPreviewSelection = function(selectedIndex)
    __wow_ensure_achievement_search_previews()
    return original(selectedIndex)
  end
  __wow_achievement_search_preview_patched = true
end

local function __wow_patch_achievement_summary_empty_text_overlap()
  if rawget(_G, "__wow_achievement_summary_empty_text_patched") then
    return
  end
  if type(AchievementFrameSummary_UpdateAchievements) ~= "function" then
    return
  end

  local original = AchievementFrameSummary_UpdateAchievements
  AchievementFrameSummary_UpdateAchievements = function(...)
    local numAchievements = select("#", ...)
    local results = { original(...) }

    local emptyText = rawget(_G, "AchievementFrameSummaryAchievementsEmptyText")
    local summary = rawget(_G, "AchievementFrameSummaryAchievements")
    local buttons = summary and summary.buttons
    local hasVisibleSummaryButton = false

    if type(buttons) == "table" then
      for _, button in ipairs(buttons) do
        if (type(button) == "table" or type(button) == "userdata")
          and type(button.IsShown) == "function"
          and button:IsShown() then
          hasVisibleSummaryButton = true
          break
        end
      end
    end

    if (type(emptyText) == "table" or type(emptyText) == "userdata")
      and type(emptyText.SetShown) == "function" then
      emptyText:SetShown(numAchievements == 0 and not hasVisibleSummaryButton)
    end

    return unpack(results)
  end

  __wow_achievement_summary_empty_text_patched = true
end

local function __wow_find_first_scroll_frame_child(parent)
  if parent == nil or type(parent.GetChildren) ~= "function" then
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

local function __wow_patch_map_canvas_scroll_container_methods()
  if rawget(_G, "__wow_map_canvas_scroll_container_patched") then
    return
  end
  if type(MapCanvasMixin) ~= "table" then
    return
  end

  if type(MapCanvasMixin.SetMapID) == "function" then
    local originalSetMapID = MapCanvasMixin.SetMapID
    MapCanvasMixin.SetMapID = function(self, ...)
      if __wow_ensure_map_canvas_scroll_container(self) == nil then
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
      local scroll = __wow_ensure_map_canvas_scroll_container(self)
      return scroll and scroll.Child or nil
    end
  end

  if type(MapCanvasMixin.GetCanvasContainer) == "function" then
    MapCanvasMixin.GetCanvasContainer = function(self, ...)
      return __wow_ensure_map_canvas_scroll_container(self)
    end
  end

  if type(MapCanvasMixin.OnFrameSizeChanged) == "function" then
    local originalOnFrameSizeChanged = MapCanvasMixin.OnFrameSizeChanged
    MapCanvasMixin.OnFrameSizeChanged = function(self, ...)
      if __wow_ensure_map_canvas_scroll_container(self) == nil then
        return
      end
      return originalOnFrameSizeChanged(self, ...)
    end
  end

  __wow_map_canvas_scroll_container_patched = true
end

if rawget(_G, "PVEFrame_ToggleFrame") == nil then
  function PVEFrame_ToggleFrame(...)
    local loadAddOn = C_AddOns and C_AddOns.LoadAddOn
    if type(loadAddOn) == "function" then
      pcall(loadAddOn, "Blizzard_GroupFinder")
    end

    local loaded = rawget(_G, "PVEFrame_ToggleFrame")
    if type(loaded) == "function" and loaded ~= PVEFrame_ToggleFrame then
      return loaded(...)
    end
  end
end

__wow_patch_map_canvas_scroll_container_methods()

local function __wow_patch_fog_of_war_pin_methods()
  if rawget(_G, "__wow_fog_of_war_pin_methods_patched") then
    return
  end
  if type(FogOfWarPinMixin) ~= "table" then
    return
  end

  if type(FogOfWarPinMixin.OnMapChanged) == "function" then
    FogOfWarPinMixin.OnMapChanged = function(self)
      local mapID = nil
      if type(self.GetMap) == "function" then
        local map = self:GetMap()
        if map ~= nil and type(map.GetMapID) == "function" then
          mapID = map:GetMapID()
        end
      end

      if (mapID == nil or mapID == 0) and C_Map ~= nil and type(C_Map.GetCurrentMapID) == "function" then
        mapID = C_Map.GetCurrentMapID()
      end

      if type(self.SetUiMapID) == "function" then
        self:SetUiMapID(mapID)
      end

      if type(self.TryFindingBestFogOfWarID) == "function" then
        self:TryFindingBestFogOfWarID(true)
      elseif (mapID == nil or mapID == 0) and type(self.Hide) == "function" then
        self:Hide()
      end
    end
  end

  rawset(_G, "__wow_fog_of_war_pin_methods_patched", true)
end

__wow_patch_fog_of_war_pin_methods()

local function __wow_patch_character_select_nav_bar()
  if rawget(_G, "__wow_character_select_nav_bar_patched") then
    return
  end
  if type(CharacterSelectNavBarMixin) ~= "table" then
    return
  end

  if type(CharacterSelectNavBarMixin.SetRealmsButtonEnabled) == "function" then
    local original_set_realms_button_enabled = CharacterSelectNavBarMixin.SetRealmsButtonEnabled
    CharacterSelectNavBarMixin.SetRealmsButtonEnabled = function(self, enabled)
      if type(self) ~= "table" or self.RealmsButton == nil then
        return
      end
      return original_set_realms_button_enabled(self, enabled)
    end
  end

  rawset(_G, "__wow_character_select_nav_bar_patched", true)
end

__wow_patch_character_select_nav_bar()

local function __wow_patch_uiparent_onupdate_worklists()
  if type(FCF_OnUpdate) == "function" and rawget(_G, "__wow_fcf_onupdate_wrapper") ~= FCF_OnUpdate then
    local original_fcf_onupdate = FCF_OnUpdate
    local wrapper = function(elapsed)
      if type(CHAT_FRAMES) == "table" and next(CHAT_FRAMES) == nil then
        return
      end
      return original_fcf_onupdate(elapsed)
    end
    FCF_OnUpdate = wrapper
    rawset(_G, "__wow_fcf_onupdate_wrapper", wrapper)
  end

  if type(ButtonPulse_OnUpdate) == "function"
    and rawget(_G, "__wow_button_pulse_onupdate_wrapper") ~= ButtonPulse_OnUpdate then
    local original_button_pulse_onupdate = ButtonPulse_OnUpdate
    local wrapper = function(elapsed)
      if type(PULSEBUTTONS) == "table" and next(PULSEBUTTONS) == nil then
        return
      end
      return original_button_pulse_onupdate(elapsed)
    end
    ButtonPulse_OnUpdate = wrapper
    rawset(_G, "__wow_button_pulse_onupdate_wrapper", wrapper)
  end

  if type(AnimatedShine_OnUpdate) == "function"
    and rawget(_G, "__wow_animated_shine_onupdate_wrapper") ~= AnimatedShine_OnUpdate then
    local original_animated_shine_onupdate = AnimatedShine_OnUpdate
    local wrapper = function(elapsed)
      if type(SHINES_TO_ANIMATE) == "table" and next(SHINES_TO_ANIMATE) == nil then
        return
      end
      return original_animated_shine_onupdate(elapsed)
    end
    AnimatedShine_OnUpdate = wrapper
    rawset(_G, "__wow_animated_shine_onupdate_wrapper", wrapper)
  end

  if type(UIParent) == "table"
    and type(UIParent.GetScript) == "function"
    and type(UIParent.SetScript) == "function" then
    local wrapper = rawget(_G, "__wow_ui_parent_onupdate_worklist_wrapper")
    if UIParent:GetScript("OnUpdate") ~= wrapper then
      wrapper = function(self, elapsed)
        if type(CHAT_FRAMES) ~= "table" or next(CHAT_FRAMES) ~= nil then
          FCF_OnUpdate(elapsed)
        end
        if type(PULSEBUTTONS) ~= "table" or next(PULSEBUTTONS) ~= nil then
          ButtonPulse_OnUpdate(elapsed)
        end
        if type(SHINES_TO_ANIMATE) ~= "table" or next(SHINES_TO_ANIMATE) ~= nil then
          AnimatedShine_OnUpdate(elapsed)
        end
        if type(HelpOpenWebTicketButton_OnUpdate) == "function" then
          HelpOpenWebTicketButton_OnUpdate(HelpOpenWebTicketButton, elapsed)
        end
      end
      UIParent:SetScript("OnUpdate", wrapper)
      rawset(_G, "__wow_ui_parent_onupdate_worklist_wrapper", wrapper)
    end
  end
end

__wow_patch_uiparent_onupdate_worklists()

if C_AddOns and type(C_AddOns.LoadAddOn) == "function" then
  hooksecurefunc(C_AddOns, "LoadAddOn", function(addonName)
    if addonName == "Blizzard_AchievementUI" then
      __wow_ensure_achievement_search_previews()
      __wow_patch_achievement_search_preview_selection()
      __wow_patch_achievement_summary_empty_text_overlap()
    elseif addonName == "Blizzard_CharacterSelectNavBar" then
      __wow_patch_character_select_nav_bar()
    elseif addonName == "Blizzard_UIParent"
      or addonName == "Blizzard_UIParent_Mainline"
      or addonName == "Blizzard_FrameXML"
      or addonName == "Blizzard_ChatFrameBase" then
      __wow_patch_uiparent_onupdate_worklists()
    elseif addonName == "Blizzard_MapCanvas"
      or addonName == "Blizzard_SharedMapDataProviders"
      or addonName == "Blizzard_WorldMap"
      or addonName == "Blizzard_BattlefieldMap" then
      __wow_patch_map_canvas_scroll_container_methods()
      __wow_patch_fog_of_war_pin_methods()
    end
  end)
end

AUTOCOMPLETE_LIST = AUTOCOMPLETE_LIST or {}
AUTOCOMPLETE_LIST.ADDFRIEND = AUTOCOMPLETE_LIST.ADDFRIEND or {}
if type(setprinthandler) ~= "function" then
  function setprinthandler() end
end

if rawget(_G, "ToggleCollectionsJournal") == nil then
  function ToggleCollectionsJournal(tabIndex)
    if DISALLOW_FRAME_TOGGLING then
      return
    end
    if not CollectionsJournal and type(CollectionsJournal_LoadUI) == "function" then
      CollectionsJournal_LoadUI()
    end
    if CollectionsJournal and type(SetCollectionsJournalShown) == "function" then
      local tabMatches = not tabIndex or tabIndex == PanelTemplates_GetSelectedTab(CollectionsJournal)
      local isShown = CollectionsJournal:IsShown() and tabMatches
      SetCollectionsJournalShown(not isShown, tabIndex)
    elseif CollectionsJournal then
      if CollectionsJournal:IsShown() then
        CollectionsJournal:Hide()
      else
        CollectionsJournal:Show()
      end
    end
  end
end

if rawget(_G, "ToggleEncounterJournal") == nil then
  function ToggleEncounterJournal()
    if DISALLOW_FRAME_TOGGLING then
      return
    end
    if not EncounterJournal and type(EncounterJournal_LoadUI) == "function" then
      EncounterJournal_LoadUI()
    end
    if not EncounterJournal and type(C_AddOns) == "table" and type(C_AddOns.LoadAddOn) == "function" then
      C_AddOns.LoadAddOn("Blizzard_EncounterJournal")
    end
    if EncounterJournal then
      if EncounterJournal:IsShown() then
        if type(HideUIPanel) == "function" then
          HideUIPanel(EncounterJournal)
        else
          EncounterJournal:Hide()
        end
      else
        if type(ShowUIPanel) == "function" then
          ShowUIPanel(EncounterJournal)
        else
          EncounterJournal:Show()
        end
      end
    end
  end
end

if CreateTemplateInfoCache == nil then
  function CreateTemplateInfoCache()
    local cache = {
      templateInfos = {},
      infoAddedCallback = nil,
    }

    function cache:Init()
    end

    function cache:SetInfoAddedCallback(callback)
      self.infoAddedCallback = callback
    end

    function cache:FlushTemplateInfos()
      self.templateInfos = {}
    end

    function cache:GetTemplateInfo(frameTemplate)
      local info = self.templateInfos[frameTemplate]
      if info == nil and C_XMLUtil and C_XMLUtil.GetTemplateInfo then
        info = C_XMLUtil.GetTemplateInfo(frameTemplate)
        self.templateInfos[frameTemplate] = info
      end
      if info ~= nil and self.infoAddedCallback then
        self.infoAddedCallback(info)
      end
      return info
    end

    function cache:GetTemplateInfos()
      return self.templateInfos
    end

    cache:Init()
    return cache
  end
end

local function __wow_frame_fields(frame)
  local env = debug and debug.getfenv and debug.getfenv(frame)
  if type(env) ~= "table" then
    return nil
  end
  if type(env[1]) ~= "table" then
    env[1] = {}
  end
  return env[1]
end

local function __wow_remove_array_value(values, target)
  if type(values) ~= "table" then
    return
  end
  for index = #values, 1, -1 do
    if values[index] == target then
      table.remove(values, index)
      break
    end
  end
end

local function __wow_register_core_frame_methods()
  local mt = GetFrameMetatable and GetFrameMetatable()
  local methods = mt and mt.__index
  if type(methods) ~= "table" then
    return
  end

  if methods.IsInitialized == nil then
    function methods:IsInitialized()
      return type(self.layoutInfo) == "table" or type(self.systemInfo) == "table"
    end
  end

  if methods.IsInDefaultPosition == nil then
    function methods:IsInDefaultPosition()
      local systemInfo = self.systemInfo
      if type(systemInfo) == "table" and systemInfo.isInDefaultPosition ~= nil then
        return systemInfo.isInDefaultPosition == true
      end
      return false
    end
  end

  if methods.AddDataProvider == nil then
    function methods:AddDataProvider(provider)
      local fields = __wow_frame_fields(self)
      if fields == nil or provider == nil then
        return
      end
      local providers = fields.dataProviders
      if type(providers) ~= "table" then
        providers = {}
        fields.dataProviders = providers
      end
      for _, existing in ipairs(providers) do
        if existing == provider then
          return
        end
      end
      table.insert(providers, provider)
      if type(provider) == "table" and provider.pin ~= nil then
        provider.pin.dataProvider = provider
      end
      if type(provider) == "table" and provider.pin == nil then
        provider.pin = { dataProvider = provider }
      end
    end
  end

  if methods.SetTitle == nil then
    function methods:SetTitle(title)
      self.title = title
      if self.TitleText and type(self.TitleText.SetText) == "function" then
        self.TitleText:SetText(title or "")
      elseif self.TitleContainer and self.TitleContainer.TitleText and type(self.TitleContainer.TitleText.SetText) == "function" then
        self.TitleContainer.TitleText:SetText(title or "")
      elseif self.Header and self.Header.Text and type(self.Header.Text.SetText) == "function" then
        self.Header.Text:SetText(title or "")
      end
    end
  end

  if methods.SetPortraitToAsset == nil then
    function methods:SetPortraitToAsset(texture)
      if self.GetPortrait and type(self.GetPortrait) == "function" then
        local portrait = self:GetPortrait()
        if portrait and type(portrait.SetTexture) == "function" then
          portrait:SetTexture(texture)
          return
        end
      end
      if self.PortraitContainer and self.PortraitContainer.portrait and type(self.PortraitContainer.portrait.SetTexture) == "function" then
        self.PortraitContainer.portrait:SetTexture(texture)
      end
    end
  end

  if methods.SetUpdateCallback == nil then
    function methods:SetUpdateCallback(callback)
      self.updateCallback = callback
    end
  end

  if methods.SetDefaultCallback == nil then
    function methods:SetDefaultCallback(callback)
      self.defaultCallback = callback
    end
  end

  if methods.SetIsDefaultCallback == nil then
    function methods:SetIsDefaultCallback(callback)
      self.isDefaultCallback = callback
    end
  end

  if methods.SetInterpolateScroll == nil then
    function methods:SetInterpolateScroll(enabled)
      self.interpolateScroll = enabled and true or false
    end
  end

  if methods.CanInterpolateScroll == nil then
    function methods:CanInterpolateScroll()
      return false
    end
  end

  if methods.Update == nil then
    function methods:Update()
      if type(self.updateCallback) == "function" then
        return self.updateCallback(self)
      end
    end
  end

  if methods.SetDirtyMethod == nil then
    function methods:SetDirtyMethod(method)
      self.dirtyCallback = function()
        method(self)
        self.dirty = nil
      end
    end
  end

  if methods.MarkDirty == nil then
    function methods:MarkDirty()
      if not self.dirty then
        if type(self.dirtyCallback) == "function" then
          RunNextFrame(self.dirtyCallback)
        end
      end
      self.dirty = true
    end
  end

  if methods.IsDirty == nil then
    function methods:IsDirty()
      return self.dirty
    end
  end

  function __wow_mark_nearest_layout_parent_dirty(frame)
    local parent = frame and frame.GetParent and frame:GetParent() or nil
    while parent do
      if __wow_mark_layout_frame_dirty(parent) then
        return
      end
      parent = parent.GetParent and parent:GetParent() or nil
    end
  end

  function __wow_mark_layout_frame_dirty(frame)
    if frame and frame.IsLayoutFrame and frame:IsLayoutFrame() then
      frame:MarkDirty()
      return true
    end
    return false
  end

  if methods.AddModule == nil then
    function methods:AddModule(module)
      local fields = __wow_frame_fields(self)
      if fields == nil or module == nil then
        return
      end
      local modules = fields.modules
      if type(modules) ~= "table" then
        modules = {}
        fields.modules = modules
      end
      for _, existing in ipairs(modules) do
        if existing == module then
          return
        end
      end
      table.insert(modules, module)
      if type(module.SetContainer) == "function" then
        module:SetContainer(self)
      end
    end
  end

  if methods.RemoveModule == nil then
    function methods:RemoveModule(module)
      local fields = __wow_frame_fields(self)
      local modules = fields and fields.modules
      if type(modules) ~= "table" then
        return
      end
      for i, existing in ipairs(modules) do
        if existing == module then
          table.remove(modules, i)
          break
        end
      end
    end
  end

  if methods.RemoveAllModules == nil then
    function methods:RemoveAllModules()
      local fields = __wow_frame_fields(self)
      if fields ~= nil then
        fields.modules = {}
      end
    end
  end

  if methods.HasModule == nil then
    function methods:HasModule(module)
      local fields = __wow_frame_fields(self)
      local modules = fields and fields.modules
      if type(modules) ~= "table" then
        return false
      end
      for _, existing in ipairs(modules) do
        if existing == module then
          return true
        end
      end
      return false
    end
  end

  if methods.RemoveDataProvider == nil then
    function methods:RemoveDataProvider(provider)
      local fields = __wow_frame_fields(self)
      local providers = fields and fields.dataProviders
      __wow_remove_array_value(providers, provider)
    end
  end

  if methods.SetDefaultText == nil then
    function methods:SetDefaultText(text)
      self.defaultText = text
    end
  end

  if methods.SetSelectionTranslator == nil then
    function methods:SetSelectionTranslator(translator)
      self.selectionTranslator = translator
    end
  end

  if methods.SetSelectionText == nil then
    function methods:SetSelectionText(selectionFunc)
      self.selectionFunc = selectionFunc
    end
  end

  if methods.EnableRegenerateOnResponse == nil then
    function methods:EnableRegenerateOnResponse()
      self.shouldRegenerateOnResponse = true
    end
  end

  if methods.GetSelectionText == nil then
    function methods:GetSelectionText()
      if type(self.selectionFunc) == "function" then
        return self.selectionFunc({})
      end
      return self.defaultText
    end
  end

  if methods.UpdateToMenuSelections == nil then
    function methods:UpdateToMenuSelections(menuDescription, currentSelections)
      local text = nil
      if type(self.selectionFunc) == "function" then
        text = self.selectionFunc(currentSelections or {})
      end
      if text == nil then
        text = self.defaultText
      end
      if text ~= nil and type(self.SetText) == "function" then
        self:SetText(text)
      end
    end
  end

  if methods.SetDefaultCallback == nil then
    function methods:SetDefaultCallback(callback)
      self.__wow_default_callback = callback
    end
  end

  if methods.SetIsDefaultCallback == nil then
    function methods:SetIsDefaultCallback(callback)
      self.__wow_is_default_callback = callback
    end
  end

  if methods.SetUpdateCallback == nil then
    function methods:SetUpdateCallback(callback)
      self.__wow_update_callback = callback
    end
  end

  if methods.NotifyUpdate == nil then
    function methods:NotifyUpdate(description)
      if type(self.__wow_update_callback) == "function" then
        self.__wow_update_callback(description)
      end
    end
  end
end

local function __wow_make_named_frame(widgetType, name, parent)
  local existing = rawget(_G, name)
  if existing ~= nil then
    return existing
  end
  local frame = CreateFrame(widgetType or "Frame", name, parent)
  rawset(_G, name, frame)
  return frame
end

local function __wow_seed_global_frame_path(root, path)
  local current = root
  for index = 1, #path do
    local name = path[index]
    local child = current[name]
    if child == nil then
      local child_type = (index == #path and name == "Title") and "FontString" or "Frame"
      if child_type == "FontString" then
        child = current:CreateFontString(nil, "OVERLAY")
        if type(child.SetText) == "function" then
          child:SetText("")
        end
      else
        child = CreateFrame("Frame", nil, current)
      end
      current[name] = child
    end
    current = child
  end
  return current
end

local function __wow_register_addon_compartment()
  local frame = __wow_make_named_frame("Frame", "AddonCompartmentFrame", UIParent)
  frame.registeredAddons = frame.registeredAddons or {}
  if frame.RegisterAddon == nil then
    function frame:RegisterAddon(addon)
      self.registeredAddons = self.registeredAddons or {}
      table.insert(self.registeredAddons, addon)
    end
  end
  if frame.UnregisterAddon == nil then
    function frame:UnregisterAddon(addon)
      self.registeredAddons = self.registeredAddons or {}
      if addon == nil then
        return
      end
      __wow_remove_array_value(self.registeredAddons, addon)
    end
  end
end

local function __wow_register_alert_frame()
  local frame = __wow_make_named_frame("Frame", "AlertFrame", UIParent)
  frame.alertFrameSubSystems = frame.alertFrameSubSystems or {}
  if frame.AddQueuedAlertFrameSubSystem == nil then
    function frame:AddQueuedAlertFrameSubSystem(template, setupFn, maxAlerts, anchorSlot)
      local subsystem = {
        template = template,
        setupFn = setupFn,
        maxAlerts = tonumber(maxAlerts) or 0,
        anchorPriority = 1000 + ((#self.alertFrameSubSystems + 1) * 10),
        anchorSlot = anchorSlot,
        queuedAlerts = {},
      }

      function subsystem:SetCanShowMoreConditionFunc(fn)
        self.canShowMoreConditionFunc = fn
      end

      function subsystem:AddAlert(alert)
        if self.maxAlerts > 0 and #self.queuedAlerts >= self.maxAlerts then
          return false
        end
        table.insert(self.queuedAlerts, alert)
        return true
      end

      function subsystem:RemoveAlert(alert)
        __wow_remove_array_value(self.queuedAlerts, alert)
      end

      function subsystem:ClearAllAlerts()
        self.queuedAlerts = {}
      end

      table.insert(self.alertFrameSubSystems, subsystem)
      return subsystem
    end
  end
end

local function __wow_register_chat_frame_globals()
  if rawget(_G, "ChatFrame1") == nil then
    CreateFrame("ScrollingMessageFrame", "ChatFrame1", UIParent)
  end

  if ChatTypeGroup == nil then
    ChatTypeGroup = {
      SYSTEM = { "SYSTEM", "IGNORED", "SKILL", "LOOT", "CHANNEL_NOTICE_USER" },
      SAY = { "SAY" },
      PARTY = { "PARTY", "PARTY_LEADER" },
      RAID = { "RAID", "RAID_LEADER", "RAID_WARNING" },
      GUILD = { "GUILD", "OFFICER" },
      WHISPER = { "WHISPER", "WHISPER_INFORM" },
      CHANNEL = { "CHANNEL", "CHANNEL_JOIN", "CHANNEL_LEAVE" },
      EMOTE = { "EMOTE", "TEXT_EMOTE" },
      BN_WHISPER = { "BN_WHISPER", "BN_WHISPER_INFORM", "BN_INLINE_TOAST_ALERT" },
      YELL = { "YELL" },
      INSTANCE_CHAT = { "INSTANCE_CHAT", "INSTANCE_CHAT_LEADER" },
    }
  end

  if ChatFrameUtil == nil then
    ChatFrameUtil = {}
  end
  if ChatFrameUtil.ProcessMessageEventFilters == nil then
    function ChatFrameUtil.ProcessMessageEventFilters(_frame, event, ...)
      return false, event, ...
    end
  end
  if ChatFrameUtil.GetChatWindowName == nil then
    function ChatFrameUtil.GetChatWindowName(id)
      return "Chat Window " .. tostring(id or 1)
    end
  end
  if ChatFrameUtil.GetCommunitiesChannelColor == nil then
    function ChatFrameUtil.GetCommunitiesChannelColor(_clubId, streamId)
      if tonumber(streamId) == 2 then
        return 0.25, 0.75, 0.25
      end
      return 0.25, 1, 0.25
    end
  end
  if ChatFrameUtil.GetCommunitiesChannelLocalID == nil then
    function ChatFrameUtil.GetCommunitiesChannelLocalID(_clubId, _streamId)
      return nil
    end
  end
end

local function __wow_register_catalog_shop_inbound_globals()
  local function ensure_inbound_interface(name)
    if rawget(_G, name) ~= nil then
      return
    end

    local inbound = {}

    function inbound.IsShown()
      return false
    end

    function inbound.SetShown(_shown, _contextKey)
    end

    function inbound.EscapePressed()
      return false
    end

    function inbound.SelectSubscriptionProduct()
    end

    function inbound.SetTokenCategory()
    end

    function inbound.CheckForFree(_event)
    end

    function inbound.OpenGamesCategory()
    end

    function inbound.SetGamesCategory()
    end

    function inbound.SetServicesCategory()
    end

    function inbound.SelectBoost(_boostType, _reason, _guid)
    end

    function inbound.SelectGameTimeProduct()
    end

    function inbound.SelectSpecificProduct(_productID)
    end

    rawset(_G, name, inbound)
  end

  ensure_inbound_interface("CatalogShopInboundInterface")
  ensure_inbound_interface("CatalogShopTopUpFlowInboundInterface")
  ensure_inbound_interface("CatalogShopRefundFlowInboundInterface")
  ensure_inbound_interface("SimpleCheckoutInboundInterface")
end

local function __wow_register_dropdown_globals()
  local function __wow_seed_dropdown_button_template_children(button, button_name)
    local highlight = __wow_ensure_named_child(button, "Highlight", "Texture", button_name .. "Highlight")
    if highlight ~= nil and highlight.Hide ~= nil then
      highlight:Hide()
    end

    local check = __wow_ensure_named_child(button, "Check", "Texture", button_name .. "Check")
    if check ~= nil and check.SetTexture ~= nil then
      check:SetTexture("Interface\\Common\\UI-DropDownRadioChecks")
    end

    local uncheck = __wow_ensure_named_child(button, "UnCheck", "Texture", button_name .. "UnCheck")
    if uncheck ~= nil and uncheck.SetTexture ~= nil then
      uncheck:SetTexture("Interface\\Common\\UI-DropDownRadioChecks")
    end

    local icon = __wow_ensure_named_child(button, "Icon", "Texture", button_name .. "Icon")
    if icon ~= nil and icon.Hide ~= nil then
      icon:Hide()
    end

    local color_swatch = __wow_ensure_named_child(button, "ColorSwatch", "Button", button_name .. "ColorSwatch")
    if color_swatch ~= nil then
      if color_swatch.Hide ~= nil then
        color_swatch:Hide()
      end
      local color = __wow_ensure_named_child(color_swatch, "Color", "Texture", button_name .. "ColorSwatchColor")
      if color ~= nil then
        rawset(color_swatch, "Color", color)
      end
    end

    local expand_arrow = __wow_ensure_named_child(button, "ExpandArrow", "Button", button_name .. "ExpandArrow")
    if expand_arrow ~= nil and expand_arrow.Hide ~= nil then
      expand_arrow:Hide()
    end

    local invisible_button = __wow_ensure_named_child(button, "invisibleButton", "Button", button_name .. "InvisibleButton")
    if invisible_button ~= nil and invisible_button.Hide ~= nil then
      invisible_button:Hide()
    end

    local new_feature = __wow_ensure_named_child(button, "NewFeature", "Frame", button_name .. "NewFeature")
    if new_feature ~= nil and new_feature.Hide ~= nil then
      new_feature:Hide()
    end

    local text = __wow_ensure_named_child(button, "Text", "FontString", button_name .. "NormalText")
    if text ~= nil then
      if text.SetFontObject ~= nil then
        text:SetFontObject("GameFontHighlightSmall")
      end
      if text.SetText ~= nil then
        text:SetText("")
      end
    end
  end

  local function __wow_seed_dropdown_list(level)
    local list_name = "DropDownList" .. tostring(level)
    local list = __wow_install_frame_helpers(__wow_ensure_named_frame("Button", list_name, UIParent))
    if list == nil then
      return
    end

    if list.SetFrameStrata ~= nil then
      list:SetFrameStrata("FULLSCREEN_DIALOG")
    end
    if list.SetClampedToScreen ~= nil then
      list:SetClampedToScreen(true)
    end
    if list.Hide ~= nil then
      list:Hide()
    end
    list.numButtons = 0
    list.maxWidth = 0

    for index = 1, 8 do
      local button_name = list_name .. "Button" .. tostring(index)
      local button = __wow_ensure_named_child(list, "Button" .. tostring(index), "Button", button_name)
      if button ~= nil then
        if button.Hide ~= nil then
          button:Hide()
        end
        __wow_seed_dropdown_button_template_children(button, button_name)
      end
    end

    if level == 1 then
      local button1 = rawget(list, "Button1")
      local normal_text = button1 ~= nil and rawget(button1, "Text") or nil
      if normal_text ~= nil and normal_text.GetFont ~= nil then
        local _, font_height = normal_text:GetFont()
        if font_height ~= nil then
          UIDROPDOWNMENU_DEFAULT_TEXT_HEIGHT = font_height
        end
      end
    end
  end

  for level = 1, 3 do
    __wow_seed_dropdown_list(level)
  end
end

local function __wow_register_misc_global_frames()
  __wow_make_named_frame("Frame", "EventToastManagerFrame", UIParent)
  __wow_make_named_frame("Frame", "EditModeManagerFrame", UIParent)
  __wow_make_named_frame("Frame", "RolePollPopup", UIParent)
  __wow_make_named_frame("Frame", "TimerTracker", UIParent)
  __wow_make_named_frame("Frame", "UIErrorsFrame", UIParent)
  __wow_make_named_frame("Frame", "SideDressUpFrame", UIParent)
  __wow_make_named_frame("Frame", "ContainerFrameCombinedBags", UIParent)
  __wow_make_named_frame("Frame", "LootFrame", UIParent)
  __wow_make_named_frame("Frame", "RaidWarningFrame", UIParent)
  __wow_make_named_frame("Frame", "GossipFrame", UIParent)
  __wow_make_named_frame("Frame", "FriendsFrame", UIParent)
  __wow_make_named_frame("Frame", "HelpFrame", UIParent)

  local gameMenu = __wow_make_named_frame("Frame", "GameMenuFrame", UIParent)
  if type(gameMenu.Hide) == "function" then
    gameMenu:Hide()
  end
  if gameMenu.buttonPool == nil and type(CreateFramePool) == "function" then
    local buttonPool = CreateFramePool("Button", gameMenu)
    local function ensure_button_text(text)
      local button = buttonPool:Acquire()
      if type(button.SetText) == "function" then
        button:SetText(text)
      end
      if type(button.Show) == "function" then
        button:Show()
      end
      return button
    end
    ensure_button_text(GAMEMENU_OPTIONS or "Options")
    ensure_button_text(LOGOUT or "Logout")
    gameMenu.buttonPool = buttonPool
  end

  local settings = __wow_make_named_frame("Frame", "SettingsPanel", UIParent)
  __wow_seed_global_frame_path(settings, { "Container", "SettingsList", "ScrollBox", "ScrollTarget" })
  __wow_seed_global_frame_path(settings, { "Container", "SettingsList", "Header", "Title" })
  __wow_seed_global_frame_path(settings, { "AccessibilityFontPreview" })
  __wow_seed_global_frame_path(settings, { "QuestTextPreview" })

  local objective = __wow_make_named_frame("Frame", "ObjectiveTrackerFrame", UIParent)
  __wow_seed_global_frame_path(objective, { "Header", "MinimizeButton" })

  local lfg_list = __wow_make_named_frame("Frame", "LFGListFrame", UIParent)
  __wow_seed_global_frame_path(lfg_list, { "SearchPanel", "SearchBox" })

  local buff_frame = rawget(_G, "BuffFrame")
  local aura_container = rawget(_G, "BuffFrameAuraContainer")
  if buff_frame ~= nil and aura_container ~= nil and buff_frame.AuraContainer == nil then
    buff_frame.AuraContainer = aura_container
  end
  if buff_frame ~= nil and buff_frame.AuraContainer ~= nil and buff_frame.AuraContainer.iconScale == nil then
    buff_frame.AuraContainer.iconScale = 1.0
  end

  if ContainerFrameContainer == nil then
    ContainerFrameContainer = { ContainerFrames = {} }
  elseif ContainerFrameContainer.ContainerFrames == nil then
    ContainerFrameContainer.ContainerFrames = {}
  end

  if PartyMemberFramePool == nil then
    PartyMemberFramePool = CreateFramePool("Frame", UIParent)
  end
end

__wow_register_core_frame_methods()
__wow_register_chat_frame_globals()
__wow_register_catalog_shop_inbound_globals()
__wow_register_dropdown_globals()
__wow_register_addon_compartment()
__wow_register_alert_frame()
__wow_register_misc_global_frames()

EVERY_X_PERCENT = EVERY_X_PERCENT or "%d%%"
TRANSMOGRIFY_TOOLTIP_APPEARANCE_KNOWN = TRANSMOGRIFY_TOOLTIP_APPEARANCE_KNOWN or "Known"
ERR_QUEST_SESSION_RESULT_RESYNC = ERR_QUEST_SESSION_RESULT_RESYNC or "Resync"
CLASS_SORT_ORDER = CLASS_SORT_ORDER or { "WARRIOR", "PALADIN", "HUNTER", "ROGUE", "PRIEST", "DEATHKNIGHT", "SHAMAN", "MAGE", "WARLOCK", "MONK", "DRUID", "DEMONHUNTER", "EVOKER" }
EVENT_TOAST_MANAGER_OFFSET_Y_OVERRIDE = false
CLOCK_TICKER_Y_OVERRIDE = CLOCK_TICKER_Y_OVERRIDE or false

local __global_mt = getmetatable(_G) or {}
local __prev_index = __global_mt.__index
local __prev_newindex = __global_mt.__newindex
local function __wow_is_color_constant_key(key)
  if type(key) ~= "string" then
    return false
  end
  if key:match("_COLOR$") then
    return true
  end
  if not key:match("_COLOR_[A-Z0-9_]+$") then
    return false
  end
  return not key:match("_COLOR_CODE")
     and not key:match("_COLOR_TABLE")
     and not key:match("_COLOR_ATLASES")
end
local function __wow_preserve_nil_global(key)
  if type(key) ~= "string" then
    return false
  end
  return key:match("^SLASH_[A-Z0-9_]+%d+$") ~= nil
      or key:match("^EMOTE%d+_CMD%d+$") ~= nil
      or key:match("^EMOTE%d+_TOKEN$") ~= nil
end
local function __wow_make_settings_initializer_placeholder()
  local initializer = {
    data = {},
  }

  function initializer:SetSearchIgnoredInLayout(layout)
    self.searchIgnoredInLayout = layout
  end

  function initializer:SetParentInitializer(parentInitializer, modifyPredicate)
    self.parentInitializer = parentInitializer
    self.modifyPredicate = modifyPredicate
  end

  function initializer:SetKioskProtected()
    self.kioskProtected = true
  end

  function initializer:GetName()
    return self.name or ""
  end

  return initializer
end

local function __wow_prepare_global_assignment(key, value)
  if key == "Settings" and type(value) == "table" and value.PingSoundsInitializer == nil then
    value.PingSoundsInitializer = __wow_make_settings_initializer_placeholder()
  elseif key == "SettingsRegistrar" and type(value) == "table" then
    local registrar_mt = getmetatable(value) or {}
    local registrar_prev_newindex = registrar_mt.__newindex

    registrar_mt.__newindex = function(tbl, subkey, subvalue)
      if subkey == "AddRegistrant" and type(subvalue) == "function" then
        local original = subvalue
        subvalue = function(self, registrant)
          if type(rawget(_G, "Settings")) == "table" and rawget(Settings, "PingSoundsInitializer") == nil then
            rawset(Settings, "PingSoundsInitializer", __wow_make_settings_initializer_placeholder())
          end
          return original(self, registrant)
        end
      end
      if registrar_prev_newindex ~= nil then
        if type(registrar_prev_newindex) == "function" then
          registrar_prev_newindex(tbl, subkey, subvalue)
          return
        end
        registrar_prev_newindex[subkey] = subvalue
        return
      end
      rawset(tbl, subkey, subvalue)
    end
    setmetatable(value, registrar_mt)
  end
  return value
end
__global_mt.__index = function(t, key)
  local value = nil
  if __prev_index ~= nil then
    if type(__prev_index) == "function" then
      value = __prev_index(t, key)
    else
      value = __prev_index[key]
    end
  end
  if value ~= nil then
    return value
  end
  if __wow_preserve_nil_global(key) then
    return nil
  end

  if key == "HIGHLIGHT_FONT_COLOR" then
    value = CreateColor(1, 1, 1, 1)
  elseif __wow_is_color_constant_key(key) then
    value = CreateColor(1, 1, 1, 1)
  elseif key == "PLAYER_FACTION_COLOR_HORDE" then
    value = CreateColor(1, 0.1, 0.1, 1)
  elseif key == "PLAYER_FACTION_COLOR_ALLIANCE" then
    value = CreateColor(0.2, 0.4, 1, 1)
  elseif type(key) == "string" and key:match("^C_[A-Za-z0-9_]+$") then
    __wow_log_nil_symbol_access("_G", key)
    value = __wow_attach_namespace_name(__wow_namespace(), key)
  elseif type(key) == "string" and key:match("^ERR_") then
    value = key
  end

  if value ~= nil then
    rawset(t, key, value)
    return value
  end
  __wow_log_nil_symbol_access("_G", key)
  return nil
end
__global_mt.__newindex = function(t, key, value)
  value = __wow_prepare_global_assignment(key, value)
  local taint = debug and debug.getstacktaint and debug.getstacktaint()
  if __prev_newindex ~= nil then
    if type(__prev_newindex) == "function" then
      __prev_newindex(t, key, value)
      return
    end
    __prev_newindex[key] = value
    if taint and type(__sim_mark_slot_taint) == "function" then
      __sim_mark_slot_taint(__prev_newindex, key, taint)
    end
    return
  end
  rawset(t, key, value)
  if taint and type(__sim_mark_slot_taint) == "function" then
    __sim_mark_slot_taint(t, key, taint)
  end
end
setmetatable(_G, __global_mt)
__wow_seed_namespace_names()

if type(rawget(_G, "Settings")) == "table" then
  __wow_prepare_global_assignment("Settings", rawget(_G, "Settings"))
end
