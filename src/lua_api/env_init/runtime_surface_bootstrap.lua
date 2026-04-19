local function __wow_make_color(r, g, b, a)
  local color = {
    r = r or 1,
    g = g or 1,
    b = b or 1,
    a = a or 1,
  }

  function color:GetRGB()
    return self.r, self.g, self.b
  end

  function color:GetRGBA()
    return self.r, self.g, self.b, self.a
  end

  function color:GenerateHexColor()
    return string.format("%02X%02X%02X", math.floor(self.r * 255), math.floor(self.g * 255), math.floor(self.b * 255))
  end

  function color:GenerateHexColorMarkup()
    return "|cFF" .. self:GenerateHexColor()
  end

  function color:WrapTextInColorCode(text)
    return self:GenerateHexColorMarkup() .. tostring(text or "") .. "|r"
  end

  return color
end

if CreateColor == nil then
  function CreateColor(r, g, b, a)
    return __wow_make_color(r, g, b, a)
  end
end

local function __wow_noop()
end

if FCF_OnUpdate == nil then
  FCF_OnUpdate = __wow_noop
end

local __wow_clock_start = os.clock and os.clock() or 0

if GetText == nil then
  function GetText(token)
    if type(token) ~= "string" then
      return token
    end
    local value = rawget(_G, token)
    return value ~= nil and value or token
  end
end

BACK = BACK or "Back"
NEXT = NEXT or "Next"
PREVIEW = PREVIEW or "Preview"
CUSTOMIZE = CUSTOMIZE or "Customize"
FINISH = FINISH or "Finish"

if BreakUpLargeNumbers == nil then
  function BreakUpLargeNumbers(value)
    return tostring(value)
  end
end

if tAppendAll == nil then
  function tAppendAll(tbl, addedArray)
    if type(tbl) ~= "table" or type(addedArray) ~= "table" then
      return tbl
    end

    for _, value in ipairs(addedArray) do
      table.insert(tbl, value)
    end

    return tbl
  end
end

if StaticPopup_Hide == nil then
  function StaticPopup_Hide(_which, _data)
  end
end

if StaticPopup_Show == nil then
  function StaticPopup_Show(_which, _text_arg1, _text_arg2, _data)
    return nil
  end
end

if ReloadUI == nil then
  function ReloadUI()
  end
end

if GetGameTime == nil then
  function GetGameTime()
    return 12, 0
  end
end

if time == nil then
  function time(dateTable)
    if os and type(os.time) == "function" then
      return os.time(dateTable)
    end
    return math.floor(GetTime())
  end
end

if GetLocale == nil then
  function GetLocale()
    return "enUS"
  end
end

if GetMoney == nil then
  function GetMoney()
    return 0
  end
end

if GetCursorMoney == nil then
  function GetCursorMoney()
    return 0
  end
end

if ActionButtonUtil == nil then
  ActionButtonUtil = {}
end

ActionButtonUtil.ActionBarActionStatus = ActionButtonUtil.ActionBarActionStatus or {
  NotMissing = 1,
  MissingFromAllBars = 2,
  OnInactiveBonusBar = 3,
  OnDisabledActionBar = 4,
}

ActionButtonUtil.ActionBarButtonNames = ActionButtonUtil.ActionBarButtonNames or {}

if ActionButtonUtil.ShowAllActionButtonGrids == nil then
  ActionButtonUtil.ShowAllActionButtonGrids = __wow_noop
end

if ActionButtonUtil.HideAllActionButtonGrids == nil then
  ActionButtonUtil.HideAllActionButtonGrids = __wow_noop
end

if ActionButtonUtil.SetAllQuickKeybindButtonHighlights == nil then
  ActionButtonUtil.SetAllQuickKeybindButtonHighlights = __wow_noop
end

if ActionButtonUtil.ShowAllQuickKeybindButtonHighlights == nil then
  ActionButtonUtil.ShowAllQuickKeybindButtonHighlights = __wow_noop
end

if ActionButtonUtil.HideAllQuickKeybindButtonHighlights == nil then
  ActionButtonUtil.HideAllQuickKeybindButtonHighlights = __wow_noop
end

if ActionButtonUtil.GetActionBarStatusForSpell == nil then
  function ActionButtonUtil.GetActionBarStatusForSpell(_spellID, _excludeNonPlayerBars, _excludeSpecialPlayerBars)
    return ActionButtonUtil.ActionBarActionStatus.NotMissing
  end
end

if ActionButtonUtil.GetActionBarStatusForPetAction == nil then
  function ActionButtonUtil.GetActionBarStatusForPetAction(_petActionID)
    return ActionButtonUtil.ActionBarActionStatus.NotMissing
  end
end

if ActionButtonUtil.GetActionBarStatusForFlyout == nil then
  function ActionButtonUtil.GetActionBarStatusForFlyout(_flyoutActionID)
    return ActionButtonUtil.ActionBarActionStatus.NotMissing
  end
end

if GetFramerate == nil then
  function GetFramerate()
    return 60
  end
end

if GetCategoryList == nil then
  function GetCategoryList()
    return {}
  end
end

if GetAverageItemLevel == nil then
  function GetAverageItemLevel()
    return 0, 0, 0
  end
end

if UI_SPECIAL_FRAMES == nil then
  UI_SPECIAL_FRAMES = {}
end

if UISpecialFrames == nil then
  UISpecialFrames = UI_SPECIAL_FRAMES
end

if GetGuildCategoryList == nil then
  function GetGuildCategoryList()
    return {}
  end
end

if GetStatisticsCategoryList == nil then
  function GetStatisticsCategoryList()
    return {}
  end
end

if GetDefaultScale == nil then
  function GetDefaultScale()
    return 1
  end
end

if GetMinRenderScale == nil then
  function GetMinRenderScale()
    return 0.5
  end
end

if GetMaxRenderScale == nil then
  function GetMaxRenderScale()
    return 1.0
  end
end

if IsExpansionTrial == nil then
  function IsExpansionTrial()
    return false
  end
end

local function __wow_ensure_glue_character_select_surface()
  IsExpansionTrial = function()
    return false
  end

  C_RecruitAFriend = C_RecruitAFriend or {}
  if type(C_RecruitAFriend.GetRecruitInfo) ~= "function" then
    function C_RecruitAFriend.GetRecruitInfo()
      return false, nil
    end
  end
end

__wow_ensure_glue_character_select_surface()

local function __wow_ensure_spellbook_surface()
  rawset(_G, "IsSelectedSpellBookItem", function(_slotIndex, _unit)
    return false
  end)
end

__wow_ensure_spellbook_surface()

local function __wow_ensure_prototype_dialog_surface()
  local existing = rawget(_G, "C_PrototypeDialog")
  local namespace = {
    _activeDialogs = {},
    _removedDialogs = {},
    _transitionHistory = {},
  }

  if type(existing) == "table" then
    for key, value in pairs(existing) do
      if key ~= "_activeDialogs"
        and key ~= "_removedDialogs"
        and key ~= "_transitionHistory"
        and key ~= "SelectOption"
        and key ~= "EnsureRemoved" then
        namespace[key] = value
      end
    end
  end

  function namespace.SelectOption(dialogID, optionID)
    if type(dialogID) ~= "number" or type(optionID) ~= "number" then
      return false
    end

    local priorState = namespace._activeDialogs[dialogID]
    local selectionCount = 1
    if type(priorState) == "table" and type(priorState.selectionCount) == "number" then
      selectionCount = priorState.selectionCount + 1
    end

    namespace._activeDialogs[dialogID] = {
      dialogID = dialogID,
      selectedOptionID = optionID,
      selectionCount = selectionCount,
    }
    namespace._removedDialogs[dialogID] = nil
    table.insert(namespace._transitionHistory, {
      transition = "selected",
      dialogID = dialogID,
      optionID = optionID,
      selectionCount = selectionCount,
    })
    return true
  end

  function namespace.EnsureRemoved(dialogID)
    if type(dialogID) ~= "number" then
      return false
    end

    local hadActiveDialog = namespace._activeDialogs[dialogID] ~= nil
    namespace._activeDialogs[dialogID] = nil
    namespace._removedDialogs[dialogID] = true
    table.insert(namespace._transitionHistory, {
      transition = "removed",
      dialogID = dialogID,
    })
    return hadActiveDialog
  end

  C_PrototypeDialog = namespace
end

__wow_ensure_prototype_dialog_surface()

local function __wow_dispatcher_invoke_callback(callbackData, ...)
  local callback = callbackData and callbackData.Callback
  if type(callback) == "function" then
    return callback(...)
  end
  if type(callback) ~= "table" then
    return nil
  end

  local method = callback[callbackData.EventFunctionOrScript]
  if type(method) == "function" then
    return method(callback, ...)
  end
  return nil
end

local function __wow_dispatcher_find_id(callbackTable, ownerOrID)
  if type(callbackTable) ~= "table" or ownerOrID == nil then
    return nil
  end
  if type(ownerOrID) == "number" then
    return ownerOrID
  end

  for id, callbackData in pairs(callbackTable) do
    if type(callbackData) == "table" and callbackData.Callback == ownerOrID then
      return id
    end
  end
  return nil
end

local function __wow_dispatcher_collect_ids(callbackTable)
  local ids = {}
  if type(callbackTable) ~= "table" then
    return ids
  end
  for id in pairs(callbackTable) do
    table.insert(ids, id)
  end
  return ids
end

local function __wow_ensure_dispatcher_surface()
  local existing = rawget(_G, "Dispatcher")
  if type(existing) == "table" and rawget(existing, "__wow_sim_dispatcher") == true then
    return
  end

  DISPATCHER_VERSION = 2.0

  local dispatcher = {
    __wow_sim_dispatcher = true,
    EventFrame = nil,
    NextEventID = 1,
    NextFunctionID = 1,
    NextScriptID = 1,
    Events = {},
    Functions = {
      Global = {},
      Owners = {},
    },
    Scripts = {},
  }

  function dispatcher:_CreateCallbackData(eventFunctionOrScript, callback, oneTime)
    return {
      EventFunctionOrScript = eventFunctionOrScript,
      Callback = callback,
      OneTime = oneTime == true,
    }
  end

  function dispatcher:Initialize()
    if type(self.EventFrame) == "table" then
      return
    end

    self.EventFrame = CreateFrame("Frame", "DispatcherFrame")
    self.EventFrame:SetScript("OnEvent", function(_, event, ...)
      self:OnEvent(event, ...)
    end)
  end

  function dispatcher:RegisterEvent(event, callback, oneTime)
    self:Initialize()

    if type(event) ~= "string" then
      return nil
    end
    if type(callback) == "table" then
      self:UnregisterEvent(event, callback)
    end

    local callbacks = self.Events[event]
    if type(callbacks) ~= "table" then
      callbacks = {}
      self.Events[event] = callbacks
      if event == "OnUpdate" then
        self.EventFrame:SetScript("OnUpdate", function(_, elapsed)
          self:OnEvent("OnUpdate", elapsed)
        end)
      else
        self.EventFrame:RegisterEvent(event)
      end
    end

    local id = self.NextEventID
    self.NextEventID = id + 1
    callbacks[id] = self:_CreateCallbackData(event, callback, oneTime)
    return id
  end

  function dispatcher:UnregisterEvent(event, ownerOrID)
    local callbacks = self.Events[event]
    if type(callbacks) ~= "table" then
      return
    end

    local id = __wow_dispatcher_find_id(callbacks, ownerOrID)
    if id ~= nil then
      callbacks[id] = nil
    end

    if next(callbacks) ~= nil then
      return
    end

    self.Events[event] = nil
    if type(self.EventFrame) ~= "table" then
      return
    end
    if event == "OnUpdate" then
      self.EventFrame:SetScript("OnUpdate", nil)
    else
      self.EventFrame:UnregisterEvent(event)
    end
  end

  function dispatcher:UnregisterAllEvents(owner)
    for event, callbacks in pairs(self.Events) do
      if __wow_dispatcher_find_id(callbacks, owner) ~= nil then
        self:UnregisterEvent(event, owner)
      end
    end
  end

  function dispatcher:OnEvent(event, ...)
    local callbacks = self.Events[event]
    if type(callbacks) ~= "table" then
      return
    end

    local idsToRemove = {}
    for _, id in ipairs(__wow_dispatcher_collect_ids(callbacks)) do
      local callbackData = callbacks[id]
      if type(callbackData) == "table" then
        __wow_dispatcher_invoke_callback(callbackData, ...)
        if callbackData.OneTime then
          table.insert(idsToRemove, id)
        end
      end
    end

    for _, id in ipairs(idsToRemove) do
      self:UnregisterEvent(event, id)
    end
  end

  function dispatcher:_GetFunctionBucket(functionOwner, functionName)
    if type(functionOwner) == "table" then
      local owned = self.Functions.Owners[functionOwner]
      return type(owned) == "table" and owned[functionName] or nil
    end
    return self.Functions.Global[functionName]
  end

  function dispatcher:_SetFunctionTarget(functionOwner, functionName, func)
    if type(functionOwner) == "table" then
      functionOwner[functionName] = func
    else
      _G[functionName] = func
    end
  end

  function dispatcher:RegisterFunction(functionOwner, functionName, callback, oneTime)
    if type(functionOwner) ~= "table" then
      functionOwner, functionName, callback, oneTime = nil, functionOwner, functionName, callback
    end

    if type(functionName) ~= "string" then
      return nil
    end

    local original = type(functionOwner) == "table" and functionOwner[functionName] or _G[functionName]
    if type(original) ~= "function" then
      return nil
    end

    local bucket = self:_GetFunctionBucket(functionOwner, functionName)
    if type(bucket) ~= "table" then
      bucket = {
        callbacks = {},
        original = original,
      }

      if type(functionOwner) == "table" then
        local owned = self.Functions.Owners[functionOwner]
        if type(owned) ~= "table" then
          owned = {}
          self.Functions.Owners[functionOwner] = owned
        end
        owned[functionName] = bucket
      else
        self.Functions.Global[functionName] = bucket
      end

      local dispatcher_ref = self
      local wrapper = function(...)
        bucket.original(...)
        dispatcher_ref:OnSecureFunc(functionOwner, functionName, ...)
      end
      bucket.wrapper = wrapper
      self:_SetFunctionTarget(functionOwner, functionName, wrapper)
    end

    local id = self.NextFunctionID
    self.NextFunctionID = id + 1
    bucket.callbacks[id] = self:_CreateCallbackData(functionName, callback, oneTime)
    return id
  end

  function dispatcher:UnregisterFunction(functionOwner, functionName, ownerOrID)
    if type(functionOwner) ~= "table" then
      functionOwner, functionName, ownerOrID = nil, functionOwner, functionName
    end

    local bucket = self:_GetFunctionBucket(functionOwner, functionName)
    if type(bucket) ~= "table" then
      return
    end

    local id = __wow_dispatcher_find_id(bucket.callbacks, ownerOrID)
    if id ~= nil then
      bucket.callbacks[id] = nil
    end

    if next(bucket.callbacks) ~= nil then
      return
    end

    self:_SetFunctionTarget(functionOwner, functionName, bucket.original)
    if type(functionOwner) == "table" then
      local owned = self.Functions.Owners[functionOwner]
      if type(owned) == "table" then
        owned[functionName] = nil
        if next(owned) == nil then
          self.Functions.Owners[functionOwner] = nil
        end
      end
    else
      self.Functions.Global[functionName] = nil
    end
  end

  function dispatcher:UnregisterAllFunctions(owner)
    for functionName, bucket in pairs(self.Functions.Global) do
      if __wow_dispatcher_find_id(bucket.callbacks, owner) ~= nil then
        self:UnregisterFunction(functionName, owner)
      end
    end

    for functionOwner, owned in pairs(self.Functions.Owners) do
      for functionName, bucket in pairs(owned) do
        if __wow_dispatcher_find_id(bucket.callbacks, owner) ~= nil then
          self:UnregisterFunction(functionOwner, functionName, owner)
        end
      end
    end
  end

  function dispatcher:OnSecureFunc(functionOwner, functionName, ...)
    local bucket = self:_GetFunctionBucket(functionOwner, functionName)
    if type(bucket) ~= "table" then
      return
    end

    local idsToRemove = {}
    for _, id in ipairs(__wow_dispatcher_collect_ids(bucket.callbacks)) do
      local callbackData = bucket.callbacks[id]
      if type(callbackData) == "table" then
        __wow_dispatcher_invoke_callback(callbackData, ...)
        if callbackData.OneTime then
          table.insert(idsToRemove, id)
        end
      end
    end

    for _, id in ipairs(idsToRemove) do
      self:UnregisterFunction(functionOwner, functionName, id)
    end
  end

  function dispatcher:RegisterScript(frame, script, callback, oneTime)
    if type(frame) ~= "table" or type(script) ~= "string" or not frame:HasScript(script) then
      return nil
    end

    local frameScripts = self.Scripts[frame]
    if type(frameScripts) ~= "table" then
      frameScripts = {}
      self.Scripts[frame] = frameScripts
    end

    local callbacks = frameScripts[script]
    if type(callbacks) ~= "table" then
      callbacks = {}
      frameScripts[script] = callbacks
      frame:HookScript(script, function(...)
        self:OnScript(frame, script, ...)
      end)
    end

    local id = self.NextScriptID
    self.NextScriptID = id + 1
    callbacks[id] = self:_CreateCallbackData(script, callback, oneTime)
    return id
  end

  function dispatcher:UnregisterScript(frame, script, ownerOrID)
    local frameScripts = self.Scripts[frame]
    local callbacks = type(frameScripts) == "table" and frameScripts[script] or nil
    if type(callbacks) ~= "table" then
      return
    end

    local id = __wow_dispatcher_find_id(callbacks, ownerOrID)
    if id ~= nil then
      callbacks[id] = nil
    end
  end

  function dispatcher:UnregisterAllScripts(owner)
    for frame, frameScripts in pairs(self.Scripts) do
      for script, callbacks in pairs(frameScripts) do
        if __wow_dispatcher_find_id(callbacks, owner) ~= nil then
          self:UnregisterScript(frame, script, owner)
        end
      end
    end
  end

  function dispatcher:OnScript(frame, script, ...)
    local frameScripts = self.Scripts[frame]
    local callbacks = type(frameScripts) == "table" and frameScripts[script] or nil
    if type(callbacks) ~= "table" then
      return
    end

    local idsToRemove = {}
    for _, id in ipairs(__wow_dispatcher_collect_ids(callbacks)) do
      local callbackData = callbacks[id]
      if type(callbackData) == "table" then
        __wow_dispatcher_invoke_callback(callbackData, ...)
        if callbackData.OneTime then
          table.insert(idsToRemove, id)
        end
      end
    end

    for _, id in ipairs(idsToRemove) do
      self:UnregisterScript(frame, script, id)
    end
  end

  function dispatcher:UnregisterAll(owner)
    self:UnregisterAllEvents(owner)
    self:UnregisterAllFunctions(owner)
    self:UnregisterAllScripts(owner)
  end

  Dispatcher = dispatcher
  dispatcher:Initialize()
end

__wow_ensure_dispatcher_surface()

if GetSpecializationInfoForSpecID == nil then
  function GetSpecializationInfoForSpecID(_specID)
    return nil, ""
  end
end

if GetUpgradeExpansionLevel == nil then
  function GetUpgradeExpansionLevel()
    return 80
  end
end

if GetCharacterUndeleteStatus == nil then
  function GetCharacterUndeleteStatus()
    return false, false, 0, 0
  end
end

if IsCharacterTimerunning == nil then
  function IsCharacterTimerunning(_characterIndex)
    return false
  end
end

if ShouldShowExpansionUpgradeBanner == nil then
  function ShouldShowExpansionUpgradeBanner()
    return false
  end
end

if GetCameraFOVDefaults == nil then
  function GetCameraFOVDefaults()
    return 0, 30, 110
  end
end

if GetGraphicsAPIs == nil then
  function GetGraphicsAPIs()
    return "D3D12", "D3D11"
  end
end

if GetCharacterListGroupsInfo == nil then
  function GetCharacterListGroupsInfo()
    return {}
  end
end

if GetInventoryItemLink == nil then
  function GetInventoryItemLink(_unit, _slot)
    return nil
  end
end

if GetWeaponEnchantInfo == nil then
  function GetWeaponEnchantInfo()
    return false, 0, 0, 0, false, 0, 0, 0
  end
end

if GetTutorialsEnabled == nil then
  function GetTutorialsEnabled()
    return false
  end
end

if GetChatWindowInfo == nil then
  function GetChatWindowInfo(id)
    -- Default chat frame color: black with 25% alpha (DEFAULT_CHATFRAME_COLOR / DEFAULT_CHATFRAME_ALPHA)
    -- Only ChatFrame1 (General) and ChatFrame2 (CombatLog) shown by default;
    -- ChatFrame3-10 exist in XML but stay hidden until user creates them.
    local realId = id or 1
    local shown = (realId == 1) or (realId == 2)
    local docked = (realId == 1)
    return "Chat " .. tostring(realId), 12, 0, 0, 0, 0.25, shown, false, docked, false
  end
end

ChatTypeInfo = ChatTypeInfo or {}
ChatTypeInfo.SYSTEM = ChatTypeInfo.SYSTEM or {
  r = 1,
  g = 1,
  b = 0,
  id = 1,
}

local __wow_chat_window_state = __wow_chat_window_state or {}

if SetChatWindowShown == nil then
  function SetChatWindowShown(id, shown)
    local chat = __wow_chat_window_state[id] or {}
    chat.shown = shown == true
    __wow_chat_window_state[id] = chat
  end
end

if GetChatWindowSavedDimensions == nil then
  function GetChatWindowSavedDimensions(id)
    local chat = __wow_chat_window_state[id]
    if not chat then
      return nil, nil
    end
    return chat.width, chat.height
  end
end

if SetChatWindowSavedDimensions == nil then
  function SetChatWindowSavedDimensions(id, width, height)
    local chat = __wow_chat_window_state[id] or {}
    chat.width = width
    chat.height = height
    __wow_chat_window_state[id] = chat
  end
end

if GetChatWindowSavedPosition == nil then
  function GetChatWindowSavedPosition(id)
    local chat = __wow_chat_window_state[id]
    if not chat then
      return nil, nil, nil
    end
    return chat.point, chat.xOffset, chat.yOffset
  end
end

if SetChatWindowSavedPosition == nil then
  function SetChatWindowSavedPosition(id, point, xOffset, yOffset)
    local chat = __wow_chat_window_state[id] or {}
    chat.point = point
    chat.xOffset = xOffset
    chat.yOffset = yOffset
    __wow_chat_window_state[id] = chat
  end
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

local function __wow_ensure_chat_voice_button_surface()
  local uiParent = rawget(_G, "UIParent")
  QuickJoinToastButton = QuickJoinToastButton or __wow_install_frame_helpers(__wow_ensure_named_frame("Button", "QuickJoinToastButton", uiParent))

  local channelButton = rawget(_G, "ChatFrameChannelButton")
  if channelButton == nil then
    return
  end

  local icon = rawget(channelButton, "Icon")
  if icon == nil and type(channelButton.CreateTexture) == "function" then
    icon = channelButton:CreateTexture(nil, "ARTWORK")
    channelButton.Icon = icon
    if type(icon.SetParentKey) == "function" then
      pcall(icon.SetParentKey, icon, "Icon", true)
    end
    if type(icon.SetAllPoints) == "function" then
      icon:SetAllPoints(channelButton)
    end
  end

  if icon ~= nil then
    if type(icon.SetParentKey) == "function" then
      pcall(icon.SetParentKey, icon, "Icon", true)
    end
    if type(icon.SetAtlas) == "function" then
      icon:SetAtlas("chatframe-button-icon-voicechat", true)
    else
      rawset(icon, "atlas", "chatframe-button-icon-voicechat")
    end
    if type(icon.Show) == "function" then
      icon:Show()
    end
  end
end

__wow_ensure_chat_voice_button_surface()

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

if CreateFrame ~= nil and __wow_original_CreateFrame == nil then
  __wow_original_CreateFrame = CreateFrame
  function CreateFrame(...)
    local inherits = select(4, ...)
    if type(inherits) == "string" then
      if string.find(inherits, "MapCanvasFrameTemplate", 1, true) or
         string.find(inherits, "MapCanvasFrameScrollContainerTemplate", 1, true) then
        __wow_patch_map_canvas_scroll_container_methods()
      end
    end
    local created = __wow_install_frame_helpers(__wow_original_CreateFrame(...))
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

  local partyFrame = __wow_install_frame_helpers(__wow_ensure_named_frame("Frame", "PartyFrame", uiParent))
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
  if partyFrame ~= nil and partyFrame.PartyMemberFramePool == nil then
    partyFrame.PartyMemberFramePool = PartyMemberFramePool
  end

  ContainerFrameContainer = ContainerFrameContainer or { ContainerFrames = {} }
  ChatFrame1 = ChatFrame1 or __wow_install_frame_helpers(__wow_ensure_named_frame("MessageFrame", "ChatFrame1", uiParent))
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

if GetTime == nil then
  function GetTime()
    if os.clock == nil then
      return 0
    end
    return os.clock() - __wow_clock_start
  end
end

if rawget(_G, "time") == nil then
  function time()
    return GetTime()
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
if UnitGroupRolesAssigned == nil then
  function UnitGroupRolesAssigned() return "NONE" end
end
if UnitGroupRolesAssignedEnum == nil then
  function UnitGroupRolesAssignedEnum() return -1 end
end
if GetInventoryItemID == nil then
  function GetInventoryItemID() return nil end
end
if GetChatWindowChannels == nil then
  function GetChatWindowChannels() return end
end
if IsInventoryItemLocked == nil then
  function IsInventoryItemLocked() return false end
end

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

if UnitTrialBankedLevels == nil then
  function UnitTrialBankedLevels(_unit)
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
        local atlas = GetClassAtlas and GetClassAtlas(classFile)
        if atlas and texture.SetAtlas then
          texture:SetAtlas(atlas)
          return
        end

        local coords = CLASS_ICON_TCOORDS and CLASS_ICON_TCOORDS[classFile]
        if coords and texture.SetTexture and texture.SetTexCoord then
          texture:SetTexture("Interface\\TargetingFrame\\UI-Classes-Circles")
          texture:SetTexCoord(unpack(coords))
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

if UnitGetAvailableRoles == nil then
  function UnitGetAvailableRoles()
    return true, true, true
  end
end

if debugstack == nil then
  function debugstack()
    return ""
  end
end

if debuglocals == nil then
  function debuglocals()
    return ""
  end
end

if issecure == nil then
  function issecure()
    return true
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

local function __wow_namespace(defaults)
  return setmetatable(defaults or {}, {
    __index = function(t, key)
      local fn = function()
        return nil
      end
      rawset(t, key, fn)
      return fn
    end,
  })
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

local function __wow_copy_table(source)
  local copy = {}
  for key, value in pairs(source or {}) do
    copy[key] = value
  end
  return copy
end

local function __wow_duration_object()
  local methods = {}
  local ids = setmetatable({}, { __mode = "k" })
  local next_id = 1

  local function new_duration_object()
    local object = {}
    ids[object] = next_id
    next_id = next_id + 1
    return setmetatable(object, {
      __index = function(t, key)
        if type(key) ~= "string" then
          return nil
        end
        local value = rawget(t, key)
        if value ~= nil then
          return value
        end
        return methods[key]
      end,
      __newindex = function(t, key, value)
        if type(key) == "string" and (
          methods[key] ~= nil or
          key == "__eq" or
          key == "__index" or
          key == "__metatable" or
          key == "__newindex" or
          key == "__tostring"
        ) then
          error("Attempted to assign to read-only key " .. key, 2)
        end
        rawset(t, key, value)
      end,
      __metatable = false,
      __tostring = function(t)
        return string.format("LuaDurationObject: 0x%016x", ids[t] or 0)
      end,
    })
  end

  function methods.Assign()
  end

  function methods.Copy()
    return new_duration_object()
  end

  function methods.EvaluateElapsedDuration()
    return 0
  end

  function methods.EvaluateElapsedPercent()
    return 0
  end

  function methods.EvaluateRemainingDuration()
    return 0
  end

  function methods.EvaluateRemainingPercent()
    return 0
  end

  function methods.GetClockTime()
    return 0
  end

  function methods.GetElapsedDuration()
    return 0
  end

  function methods.GetElapsedPercent()
    return 0
  end

  function methods.GetEndTime()
    return 0
  end

  function methods.GetModRate()
    return 1
  end

  function methods.GetRemainingDuration()
    return 0
  end

  function methods.GetRemainingPercent()
    return 0
  end

  function methods.GetStartTime()
    return 0
  end

  function methods.GetTotalDuration()
    return 0
  end

  function methods.HasSecretValues()
    return false
  end

  function methods.IsZero()
    return true
  end

  function methods.Reset()
  end

  function methods.SetTimeFromEnd()
  end

  function methods.SetTimeFromStart()
  end

  function methods.SetTimeSpan()
  end

  function methods.SetToDefaults()
  end

  return new_duration_object
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

Kiosk = __wow_merge_namespace(Kiosk, {
  IsEnabled = function() return false end,
  IsCompetitiveModeEnabled = function() return false end,
})

C_ChatInfo = __wow_merge_namespace(C_ChatInfo, {
  PerformEmote = function() return false end,
  CancelEmote = __wow_noop,
  IsValidChatLine = function() return false end,
  ReplaceIconAndGroupExpressions = function(message) return message end,
  SendChatMessage = __wow_noop,
  AreOutgoingAddonChatMessagesRestricted = function() return false end,
  GetNumReservedChatWindows = function() return 0 end,
  GetNumActiveChannels = function() return 0 end,
  GetChannelRulesetForChannelID = function() return 0 end,
  GetChannelRuleset = function() return 0 end,
  GetChannelInfoFromIdentifier = function() return nil end,
  GetChatLineText = function() return nil end,
  IsTimerunningPlayer = function() return false end,
  UncensorChatLine = __wow_noop,
  DropCautionaryChatMessage = __wow_noop,
  SendCautionaryChatMessage = __wow_noop,
  GetChannelShortcut = function(index) return tostring(index or "") end,
  GetGeneralChannelLocalID = function() return 0 end,
  GetGeneralChannelID = function() return 0 end,
  GetChannelShortcutForChannelID = function() return "" end,
  IsChannelRegionalForChannelID = function() return false end,
})

C_LFGList = __wow_merge_namespace(C_LFGList, {
  GetApplications = function() return {} end,
  GetApplicationInfo = function() return nil end,
  GetAvailableCategories = function() return {} end,
  GetAvailableRoles = function() return false, false, false end,
  -- GetNumApplications / GetNumApplicants overridden from Rust
  -- (src/lua_api/globals/lfg_list.rs), backed by SimState::lfg_list_counts.
  GetPremadeGroupFinderStyle = function() return 0 end,
  GetActivityFullName = function() return "" end,
  GetActivityInfoTable = function() return nil end,
  GetSearchResultInfo = function() return nil end,
  GetSearchResultMemberCounts = function() return 0, 0, 0, 0 end,
  HasActiveEntryInfo = function() return false end,
  HasSearchResultInfo = function() return false end,
  CanCreateQuestGroup = function() return false end,
  CanCreateScenarioGroup = function() return false end,
  IsPremadeGroupFinderEnabled = function() return false end,
  RemoveListing = __wow_noop,
})

C_AddOnProfiler = __wow_merge_namespace(C_AddOnProfiler, {
  CheckForPerformanceMessage = function() return nil end,
})

C_Ping = __wow_merge_namespace(C_Ping, {
  GetDefaultPingOptions = function() return {} end,
})

C_ZoneAbility = __wow_merge_namespace(C_ZoneAbility, {
  GetActiveAbilities = function() return {} end,
})

if rawget(C_ZoneAbility, "GetActiveAbilities") == nil then
  function C_ZoneAbility.GetActiveAbilities()
    return {}
  end
end

C_AuthChallenge = __wow_merge_namespace(C_AuthChallenge, {
  SetFrame = __wow_noop,
  Submit = __wow_noop,
  Cancel = __wow_noop,
  OnTabPressed = __wow_noop,
  DidChallengeSucceed = function() return false end,
})

C_ClassTrial = __wow_merge_namespace(C_ClassTrial, {
  IsClassTrialCharacter = function() return false end,
  GetClassTrialLogoutTimeSeconds = function() return 0 end,
})

C_CharacterServices = __wow_merge_namespace(C_CharacterServices, {
  HasRequiredBoostForClassTrial = function() return false end,
  GetCharacterServiceDisplayInfo = function() return {} end,
  GetVASDistributions = function() return {} end,
  GetCharacterServiceDisplayData = function(_boostType)
    return {
      boostLevel = GetMaxPlayerLevel and GetMaxPlayerLevel() or 80,
      flowTitle = CHARACTER_UPGRADE or "Character Upgrade",
      popupInfo = {
        textureKit = "characterupdate",
      },
    }
  end,
})

C_SharedCharacterServices = __wow_merge_namespace(C_SharedCharacterServices, {
  GetUpgradeDistributions = function() return {} end,
})

C_SocialQueue = __wow_merge_namespace(C_SocialQueue, {
  GetAllGroups = function() return {} end,
  GetConfig = function() return {} end,
  GetGroupForPlayer = function() return nil end,
  GetGroupInfo = function() return nil end,
  GetGroupMembers = function() return {} end,
  GetGroupQueues = function() return {} end,
  RequestToJoin = __wow_noop,
  SignalToastDisplayed = __wow_noop,
})

C_ArrowCalloutManager = __wow_merge_namespace(C_ArrowCalloutManager, {})

if type(rawget(C_ArrowCalloutManager, "_activeCallouts")) ~= "table" then
  C_ArrowCalloutManager._activeCallouts = {}
end

if type(rawget(C_ArrowCalloutManager, "_acknowledgedCallouts")) ~= "table" then
  C_ArrowCalloutManager._acknowledgedCallouts = {}
end

local function __wow_arrow_callout_dispatch(eventName, payload)
  local manager = rawget(_G, "ArrowCalloutFrameManager")
  if type(manager) ~= "table" then
    return
  end
  local onEvent = rawget(manager, "OnEvent")
  if type(onEvent) == "function" then
    onEvent(manager, eventName, payload)
  end
end

if rawget(C_ArrowCalloutManager, "ShowCallout") == nil then
  function C_ArrowCalloutManager.ShowCallout(calloutInfo)
    if type(calloutInfo) ~= "table" or calloutInfo.calloutID == nil then
      return false
    end
    C_ArrowCalloutManager._activeCallouts[calloutInfo.calloutID] = calloutInfo
    __wow_arrow_callout_dispatch("SHOW_ARROW_CALLOUT", calloutInfo)
    return true
  end
end

if rawget(C_ArrowCalloutManager, "HideCallout") == nil then
  function C_ArrowCalloutManager.HideCallout(calloutID)
    if calloutID == nil then
      return
    end
    if C_ArrowCalloutManager._activeCallouts[calloutID] ~= nil then
      C_ArrowCalloutManager._activeCallouts[calloutID] = nil
      __wow_arrow_callout_dispatch("HIDE_ARROW_CALLOUT", calloutID)
    end
  end
end

if rawget(C_ArrowCalloutManager, "AcknowledgeCallout") == nil then
  function C_ArrowCalloutManager.AcknowledgeCallout(calloutID)
    if calloutID == nil then
      return
    end
    C_ArrowCalloutManager._acknowledgedCallouts[calloutID] = true
    C_ArrowCalloutManager.HideCallout(calloutID)
  end
end

if rawget(C_ArrowCalloutManager, "IsCalloutActive") == nil then
  function C_ArrowCalloutManager.IsCalloutActive(calloutID)
    return C_ArrowCalloutManager._activeCallouts[calloutID] ~= nil
  end
end

if rawget(C_ArrowCalloutManager, "IsCalloutAcknowledged") == nil then
  function C_ArrowCalloutManager.IsCalloutAcknowledged(calloutID)
    return C_ArrowCalloutManager._acknowledgedCallouts[calloutID] == true
  end
end

C_EventScheduler = __wow_merge_namespace(C_EventScheduler, {})

local function __wow_event_scheduler_seed_state()
  local now = (os and type(os.time) == "function") and os.time() or 0
  return {
    canShowEvents = nil,
    suppressDisplay = false,
    ongoingEvents = {
      {
        areaPoiID = 1001,
        eventID = 1001,
        eventKey = "warsong-gulch",
        displayInfo = {},
        rewardsClaimed = false,
      },
      {
        areaPoiID = 1002,
        eventID = 1002,
        eventKey = "cinderbrew-meadery",
        displayInfo = {},
        rewardsClaimed = false,
      },
    },
    scheduledEvents = {
      {
        areaPoiID = 1001,
        eventID = 2001,
        eventKey = "pvp-brawl-blitz",
        startTime = now + 3600,
        endTime = now + 7200,
        duration = 3600,
        hasReminder = false,
        rewardsClaimed = false,
        displayInfo = {},
      },
      {
        areaPoiID = 1004,
        eventID = 2002,
        eventKey = "darkmoon-island",
        startTime = now + 7200,
        endTime = now + 10800,
        duration = 3600,
        hasReminder = true,
        rewardsClaimed = false,
        displayInfo = {},
      },
    },
    reminders = {},
  }
end

if type(rawget(C_EventScheduler, "_state")) ~= "table" then
  C_EventScheduler._state = __wow_event_scheduler_seed_state()
end

if rawget(C_EventScheduler, "CanShowEvents") == nil then
  function C_EventScheduler.CanShowEvents()
    local state = C_EventScheduler._state
    if type(state) ~= "table" then
      return false
    end
    if state.canShowEvents ~= nil then
      return state.canShowEvents == true
    end
    if state.suppressDisplay == true then
      return false
    end
    return #(state.ongoingEvents or {}) > 0 or #(state.scheduledEvents or {}) > 0
  end
end

if rawget(C_EventScheduler, "RequestEvents") == nil then
  function C_EventScheduler.RequestEvents()
    C_EventScheduler._state = __wow_event_scheduler_seed_state()
  end
end

if rawget(C_EventScheduler, "GetOngoingEvents") == nil then
  function C_EventScheduler.GetOngoingEvents()
    return C_EventScheduler._state.ongoingEvents
  end
end

if rawget(C_EventScheduler, "GetScheduledEvents") == nil then
  function C_EventScheduler.GetScheduledEvents()
    return C_EventScheduler._state.scheduledEvents
  end
end

if rawget(C_EventScheduler, "HasData") == nil then
  function C_EventScheduler.HasData()
    local state = C_EventScheduler._state
    return #(state.ongoingEvents or {}) > 0 or #(state.scheduledEvents or {}) > 0
  end
end

if rawget(C_EventScheduler, "GetEventZoneName") == nil then
  function C_EventScheduler.GetEventZoneName(areaPoiID)
    local poi = C_AreaPoiInfo.GetAreaPOIInfo(nil, areaPoiID)
    return poi and poi.name or ""
  end
end

if rawget(C_EventScheduler, "GetEventUiMapID") == nil then
  function C_EventScheduler.GetEventUiMapID(areaPoiID)
    local poi = C_AreaPoiInfo.GetAreaPOIInfo(nil, areaPoiID)
    return (poi and poi.uiMapID) or 0
  end
end

if rawget(C_EventScheduler, "HasSavedReminders") == nil then
  function C_EventScheduler.HasSavedReminders()
    local reminders = C_EventScheduler._state.reminders or {}
    return next(reminders) ~= nil
  end
end

if rawget(C_EventScheduler, "SetReminder") == nil then
  function C_EventScheduler.SetReminder(eventKey)
    if eventKey ~= nil then
      C_EventScheduler._state.reminders[tostring(eventKey)] = true
    end
  end
end

if rawget(C_EventScheduler, "ClearReminder") == nil then
  function C_EventScheduler.ClearReminder(eventKey)
    if eventKey ~= nil then
      C_EventScheduler._state.reminders[tostring(eventKey)] = nil
    end
  end
end

if rawget(C_EventScheduler, "GetActiveContinentName") == nil then
  function C_EventScheduler.GetActiveContinentName()
    return nil
  end
end

C_UnitAuras = __wow_merge_namespace(C_UnitAuras, {
  SetPrivateWarningTextAnchor = __wow_noop,
})

if C_UnitAuras._blockedAuras == nil then
  C_UnitAuras._blockedAuras = {}
end

if C_UnitAuras._providerSwitched == nil then
  C_UnitAuras._providerSwitched = false
end

if C_UnitAuras.AddBlockedAura == nil then
  function C_UnitAuras.AddBlockedAura(unitToken, auraInstanceID)
    if unitToken == nil or auraInstanceID == nil then
      return
    end
    C_UnitAuras._blockedAuras[tostring(unitToken) .. ":" .. tostring(auraInstanceID)] = true
  end
end

if C_UnitAuras.SwitchAuraDataProvider == nil then
  function C_UnitAuras.SwitchAuraDataProvider()
    C_UnitAuras._providerSwitched = true
  end
end

if C_UnitAuras.ResetAuraDataProvider == nil then
  function C_UnitAuras.ResetAuraDataProvider()
    C_UnitAuras._providerSwitched = false
  end
end

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
  }

  function CombatLogAddFilter(_filter)
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
    return nil
  end

  function CombatLogGetNumEntries()
    return __wow_combat_log_state.numEntries
  end

  function CombatLogSetCurrentEntry(entry)
    __wow_combat_log_state.currentEntry = math.max(0, tonumber(entry) or 0)
  end
end

C_UnitAurasPrivate = __wow_merge_namespace(C_UnitAurasPrivate, {})

if type(C_UnitAurasPrivate._state) ~= "table" then
  C_UnitAurasPrivate._state = {}
end

local function __wow_private_aura_state()
  local state = C_UnitAurasPrivate._state
  if type(state.anchors) ~= "table" then
    state.anchors = {}
  end
  if type(state.privateAurasByUnit) ~= "table" then
    state.privateAurasByUnit = {}
  end
  if type(state.auraDataByUnit) ~= "table" then
    state.auraDataByUnit = {}
  end
  if type(state.updateCallbacksByUnit) ~= "table" then
    state.updateCallbacksByUnit = {}
  end
  if type(state.nextAnchorID) ~= "number" then
    state.nextAnchorID = 1
  end
  return state
end

__wow_private_aura_state()

local function __wow_copy_private_aura_value(value, seen)
  if type(value) ~= "table" then
    return value
  end
  seen = seen or {}
  if seen[value] ~= nil then
    return seen[value]
  end
  local copy = {}
  seen[value] = copy
  for key, nested in pairs(value) do
    copy[__wow_copy_private_aura_value(key, seen)] = __wow_copy_private_aura_value(nested, seen)
  end
  local mt = getmetatable(value)
  if mt ~= nil then
    setmetatable(copy, mt)
  end
  return copy
end

local function __wow_copy_private_aura_list(list)
  local copy = {}
  for index = 1, #(list or {}) do
    copy[index] = __wow_copy_private_aura_value(list[index])
  end
  return copy
end

if rawget(C_UnitAurasPrivate, "SetPrivateAuraAnchorAddedCallback") == nil then
  function C_UnitAurasPrivate.SetPrivateAuraAnchorAddedCallback(callback)
    C_UnitAurasPrivate._anchorAddedCallback = callback
  end
end

if rawget(C_UnitAurasPrivate, "SetPrivateAuraAnchorRemovedCallback") == nil then
  function C_UnitAurasPrivate.SetPrivateAuraAnchorRemovedCallback(callback)
    C_UnitAurasPrivate._anchorRemovedCallback = callback
  end
end

if rawget(C_UnitAurasPrivate, "GetPrivateAuraAnchors") == nil then
  function C_UnitAurasPrivate.GetPrivateAuraAnchors(unitToken)
    local anchors = {}
    local state = __wow_private_aura_state()
    for index = 1, #state.anchors do
      local anchor = state.anchors[index]
      if unitToken == nil or anchor.unitToken == unitToken then
        anchors[#anchors + 1] = __wow_copy_private_aura_value(anchor)
      end
    end
    return anchors
  end
end

if rawget(C_UnitAurasPrivate, "_AddPrivateAuraAnchorForTest") == nil then
  function C_UnitAurasPrivate._AddPrivateAuraAnchorForTest(anchorInfo)
    local state = __wow_private_aura_state()
    local anchor = __wow_copy_private_aura_value(anchorInfo or {})
    anchor.anchorID = state.nextAnchorID
    state.nextAnchorID = state.nextAnchorID + 1
    state.anchors[#state.anchors + 1] = anchor
    if type(C_UnitAurasPrivate._anchorAddedCallback) == "function" then
      C_UnitAurasPrivate._anchorAddedCallback(__wow_copy_private_aura_value(anchor))
    end
    return anchor.anchorID
  end
end

if rawget(C_UnitAurasPrivate, "_RemovePrivateAuraAnchorForTest") == nil then
  function C_UnitAurasPrivate._RemovePrivateAuraAnchorForTest(anchorID)
    local state = __wow_private_aura_state()
    for index = 1, #state.anchors do
      if state.anchors[index].anchorID == anchorID then
        table.remove(state.anchors, index)
        if type(C_UnitAurasPrivate._anchorRemovedCallback) == "function" then
          C_UnitAurasPrivate._anchorRemovedCallback(anchorID)
        end
        return true
      end
    end
    return false
  end
end

if rawget(C_UnitAurasPrivate, "SetPrivateWarningTextFrame") == nil then
  function C_UnitAurasPrivate.SetPrivateWarningTextFrame(frame)
    __wow_private_aura_state().warningTextFrame = frame
  end
end

if rawget(C_UnitAurasPrivate, "SetShowDispelTypeCallback") == nil then
  function C_UnitAurasPrivate.SetShowDispelTypeCallback(callback)
    C_UnitAurasPrivate._showDispelTypeCallback = callback
  end
end

if rawget(C_UnitAuras, "TriggerPrivateAuraShowDispelType") == nil then
  function C_UnitAuras.TriggerPrivateAuraShowDispelType(showDispelType)
    local state = __wow_private_aura_state()
    state.lastShowDispelType = showDispelType
    if type(C_UnitAurasPrivate._showDispelTypeCallback) == "function" then
      C_UnitAurasPrivate._showDispelTypeCallback(showDispelType)
    end
  end
end

if rawget(C_UnitAurasPrivate, "AddPrivateAuraUpdateCallback") == nil then
  function C_UnitAurasPrivate.AddPrivateAuraUpdateCallback(unitToken, callback)
    local state = __wow_private_aura_state()
    local key = tostring(unitToken or "")
    local callbacks = state.updateCallbacksByUnit[key]
    if type(callbacks) ~= "table" then
      callbacks = {}
      state.updateCallbacksByUnit[key] = callbacks
    end
    callbacks[#callbacks + 1] = callback
  end
end

if rawget(C_UnitAurasPrivate, "_TriggerPrivateAuraUpdate") == nil then
  function C_UnitAurasPrivate._TriggerPrivateAuraUpdate(unitToken, privateSource, updateInfo)
    local state = __wow_private_aura_state()
    local callbacks = state.updateCallbacksByUnit[tostring(unitToken or "")]
    local fired = 0
    for index = 1, #(callbacks or {}) do
      if type(callbacks[index]) == "function" then
        callbacks[index](privateSource, __wow_copy_private_aura_value(updateInfo))
        fired = fired + 1
      end
    end
    return fired
  end
end

if rawget(C_UnitAurasPrivate, "GetAllPrivateAuras") == nil then
  function C_UnitAurasPrivate.GetAllPrivateAuras(unitToken)
    local state = __wow_private_aura_state()
    return __wow_copy_private_aura_list(state.privateAurasByUnit[tostring(unitToken or "")] or {})
  end
end

if rawget(C_UnitAurasPrivate, "GetAuraDataByAuraInstanceIDPrivate") == nil then
  function C_UnitAurasPrivate.GetAuraDataByAuraInstanceIDPrivate(unitToken, auraInstanceID)
    local state = __wow_private_aura_state()
    local byUnit = state.auraDataByUnit[tostring(unitToken or "")]
    if type(byUnit) ~= "table" then
      return nil
    end
    local aura = byUnit[auraInstanceID]
    if aura == nil and auraInstanceID ~= nil then
      aura = byUnit[tonumber(auraInstanceID)]
    end
    if aura == nil then
      return nil
    end
    return __wow_copy_private_aura_value(aura)
  end
end

C_PetBattles = __wow_merge_namespace(C_PetBattles, {
  GetAllEffectNames = function() return end,
  GetAbilityState = function() return false, 0, 0 end,
  GetActivePet = function() return 1 end,
  IsTrapAvailable = function() return false, 0 end,
  ShouldShowPetSelect = function() return false end,
})

C_VoiceChat = __wow_merge_namespace(C_VoiceChat, {
  GetTtsVoices = function() return {} end,
  IsTranscriptionAllowed = function() return false end,
})

C_TTSSettings = __wow_merge_namespace(C_TTSSettings, {
  GetSpeechVolume = function() return 100 end,
  SetSpeechVolume = __wow_noop,
  GetSpeechRate = function() return 0 end,
  SetSpeechRate = __wow_noop,
  GetVoiceOptionID = function() return 0 end,
  SetVoiceOptionID = __wow_noop,
})

C_ClubFinder = __wow_merge_namespace(C_ClubFinder, {
  GetClubRecruitmentSettings = function()
    return {
      playStyleDungeon = false,
      playStyleRaids = false,
      playStylePvp = false,
      playStyleRP = false,
      playStyleSocial = false,
      maxLevelOnly = false,
      enableListing = false,
    }
  end,
  GetPlayerApplicantSettings = function()
    return {
      playStyleDungeon = false,
      playStyleRaids = false,
      playStylePvp = false,
      playStyleRP = false,
      playStyleSocial = false,
      roleTank = false,
      roleHealer = false,
      roleDps = false,
      sizeSmall = false,
      sizeMedium = false,
      sizeLarge = false,
      sortRelevance = true,
      sortMembers = false,
      sortNewest = false,
      crossFaction = false,
    }
  end,
})

C_PartyInfo = __wow_merge_namespace(C_PartyInfo, {
  AllowedToDoPartyConversion = function() return false end,
  IsPartyWalkIn = function() return false end,
})

C_Map = __wow_merge_namespace(C_Map, {})

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

-- Not in a scenario by default. Blizzard_ScenarioObjectiveTracker.lua:186
-- calls `numStages > 0` on the returned value, so numStages must be a
-- real zero, not nil.
C_Scenario = __wow_merge_namespace(C_Scenario, {
  GetInfo = function()
    -- scenarioName, currentStage, numStages, flags, _, _, _, xp, money,
    -- scenarioType, _, textureKit, scenarioID
    return nil, 0, 0, 0, nil, nil, nil, 0, 0, 0, nil, "evergreen-scenario", 0
  end,
  IsInScenario = function() return false end,
  GetStepInfo = function() return nil, 0, 0, false, false, 0, 0, 0, 0, false, false end,
})

-- AccountStore / DamageMeter / CooldownViewer: Blizzard data-provider init
-- iterates the returned category / session / cooldown list with ipairs.
-- None of these subsystems are simulated; return empty tables.
C_AccountStore = __wow_merge_namespace(C_AccountStore, {
  GetCategories = function() return {} end,
  GetCategoryInfo = function() return nil end,
  GetItemInfo = function() return nil end,
  GetCurrencyAvailable = function() return 0 end,
})

C_DamageMeter = __wow_merge_namespace(C_DamageMeter, {
  GetAvailableCombatSessions = function() return {} end,
  GetCurrentCombatSessionID = function() return nil end,
  GetDamageMeterEntries = function() return {} end,
})

C_CooldownViewer = __wow_merge_namespace(C_CooldownViewer, {
  GetCooldownViewerCategorySet = function() return {} end,
  GetCooldownViewerCooldownInfo = function() return nil end,
  GetCooldownID = function() return nil end,
})

C_Minimap = __wow_merge_namespace(C_Minimap, {
  GetNumTrackingTypes = function() return 0 end,
  GetTrackingInfo = function() return nil end,
  GetTrackingFilter = function()
    return { filterID = 0, spellID = 0 }
  end,
  SetTracking = __wow_noop,
  ClearAllTracking = __wow_noop,
})

C_Navigation = __wow_merge_namespace(C_Navigation, {
  WasClampedToScreen = function() return false end,
  GetTargetState = function() return 0 end,
  HasValidScreenPosition = function() return false end,
  GetDistance = function() return 0 end,
  GetNearestPartyMemberToken = function() return nil end,
  GetFrame = function() return UIParent end,
})

C_DateAndTime = __wow_merge_namespace(C_DateAndTime, {
  GetCurrentCalendarTime = function()
    return __wow_make_calendar_time(0, 0)
  end,
  AdjustTimeByDays = function(calendarTime, deltaDays)
    local time = __wow_copy_table(calendarTime)
    time.monthDay = (time.monthDay or 14) + (tonumber(deltaDays) or 0)
    return time
  end,
  AdjustTimeByMinutes = function(calendarTime, deltaMinutes)
    local base = __wow_copy_table(calendarTime)
    local totalMinutes = ((base.hour or 12) * 60) + (base.minute or 0) + (tonumber(deltaMinutes) or 0)
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
      base.monthDay = (base.monthDay or 14) - 1
    end
    while hour >= 24 do
      hour = hour - 24
      base.monthDay = (base.monthDay or 14) + 1
    end
    base.hour = hour
    base.minute = minute
    return base
  end,
  GetCalendarTimeFromEpoch = function(epoch)
    return __wow_make_calendar_time(0, math.floor((tonumber(epoch) or 0) / 60))
  end,
  GetWeeklyResetStartTime = function()
    return 0
  end,
  GetSecondsUntilDailyReset = function()
    return 0
  end,
})

C_CatalogShop = __wow_merge_namespace(C_CatalogShop, {
  IsShop2Enabled = function() return false end,
  HasNewProducts = function() return false end,
  GetAvailableCategoryIDs = function() return {} end,
  GetFailureInfo = function() return nil, nil end,
  RefreshVirtualCurrencyBalance = __wow_noop,
  GetVirtualCurrencyBalance = function() return 0 end,
  OpenCatalogShopInteractionFromShop = function() return nil end,
  OpenCatalogShopInteractionFromHouse = function() return nil end,
  CloseCatalogShopInteraction = __wow_noop,
  GetFirstCategoryByProductID = function() return nil end,
  ShouldShowHousingWarning = function() return false end,
  GetProductInfo = function() return nil end,
  GetCatalogShopProductDisplayInfo = function() return nil end,
  GetProductIDsForBundle = function() return {} end,
  GetSpellVisualInfoForMount = function() return nil end,
  PurchaseProduct = __wow_noop,
  ConfirmHousingPurchase = __wow_noop,
  ProductDisplayedTelemetry = __wow_noop,
  OnLegalDisclaimerClicked = __wow_noop,
  FindBestCurrencyProductForNeededAmount = function() return nil end,
  IsProductIncludedInAnyBundle = function() return false end,
  GetProductAvailabilityTimeRemainingSecs = function() return 0 end,
})

C_WowTokenPublic = __wow_merge_namespace(C_WowTokenPublic, {
  GetCommerceSystemStatus = function() return false, 0, false end,
  UpdateTokenCount = __wow_noop,
  GetCurrentMarketPrice = function() return 0, 0 end,
  GetGuaranteedPrice = function() return 0 end,
  BuyToken = __wow_noop,
  UpdateListedAuctionableTokens = __wow_noop,
  UpdateMarketPrice = __wow_noop,
  IsAuctionableWowToken = function() return false end,
})

C_WowTokenSecure = __wow_merge_namespace(C_WowTokenSecure, {
  CancelRedeem = __wow_noop,
  GetBalanceRedeemAmount = function() return 0 end,
  SetBalanceAmountString = __wow_noop,
  GetBalanceRedemptionInfo = function() return 0, 0, false, nil end,
  GetGameTimeRedemptionInfo = function() return false, 0 end,
  GetRemainingGameTime = function() return 0 end,
  CanRedeemForBalance = function() return false end,
  RedeemToken = __wow_noop,
  WillKickFromWorld = function() return false end,
  GetTokenCount = function() return 0 end,
  RedeemTokenConfirm = __wow_noop,
  IsRedemptionStillValid = function() return false end,
  ConfirmSellToken = __wow_noop,
  ConfirmBuyToken = __wow_noop,
  GetPriceLockDuration = function() return 0 end,
})

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
SecureTypes.CreateSecureMap = SecureTypes.CreateSecureMap or function()
  local map = {}
  function map:Insert(key, value)
    self[key] = value
  end
  function map:Remove(key)
    local value = self[key]
    self[key] = nil
    return value
  end
  function map:Find(key)
    return self[key]
  end
  function map:Contains(key)
    return self[key] ~= nil
  end
  function map:Enumerate()
    return next, self, nil
  end
  function map:Clear()
    for key in pairs(self) do
      self[key] = nil
    end
  end
  return map
end
SecureTypes.CreateSecureFunction = SecureTypes.CreateSecureFunction or function(fn) return fn end
SecureTypes.CreateSecureNumber = SecureTypes.CreateSecureNumber or function(value) return value or 0 end
SecureTypes.CreateSecureArray = SecureTypes.CreateSecureArray or function()
  local array = {}
  function array:Insert(value)
    self[#self + 1] = value
  end
  function array:Remove(value)
    for index, existing in ipairs(self) do
      if existing == value then
        table.remove(self, index)
        return true
      end
    end
    return false
  end
  function array:Clear()
    for index = #self, 1, -1 do
      self[index] = nil
    end
  end
  function array:Enumerate()
    local index = 0
    return function()
      index = index + 1
      if index <= #self then
        return self[index]
      end
    end
  end
  return array
end

ProxyUtil = ProxyUtil or {}
ProxyConvertableMixin = ProxyConvertableMixin or {}
ProxyUtil.CreateProxy = ProxyUtil.CreateProxy or function(value) return value end
ProxyUtil.CreateProxyMixin = ProxyUtil.CreateProxyMixin or function() return {} end
ProxyUtil.SetPrivateReference = ProxyUtil.SetPrivateReference or __wow_noop
ProxyUtil.ReleasePrivateReference = ProxyUtil.ReleasePrivateReference or __wow_noop
ProxyUtil.CreateProxyDirectory = ProxyUtil.CreateProxyDirectory or function()
  return {
    ToPrivate = function(_, value) return value end,
    ToPublic = function(_, value) return value end,
  }
end

if CreateFramePool == nil then
  function CreateFramePool(frameType, parent, template, resetter)
    local pool = {
      frameType = frameType or "Frame",
      parent = parent,
      template = template,
      resetter = resetter,
      inactive = {},
      active = {},
      known = {},
    }

    function pool:Acquire()
      local frame = table.remove(self.inactive)
      local isNew = false
      if frame == nil then
        frame = CreateFrame(self.frameType, nil, self.parent, self.template)
        isNew = true
        self.known[frame] = true
      end
      self.active[frame] = true
      return frame, isNew
    end

    function pool:Release(frame)
      if frame == nil or not self:DoesObjectBelongToPool(frame) then
        return false
      end
      self.active[frame] = nil
      if self.resetter then
        self.resetter(self, frame, false, self.template)
      end
      table.insert(self.inactive, frame)
      return true
    end

    function pool:ReleaseAll()
      local frames = {}
      for frame in pairs(self.active) do
        table.insert(frames, frame)
      end
      for _, frame in ipairs(frames) do
        self:Release(frame)
      end
    end

    function pool:GetNumActive()
      local count = 0
      for _ in pairs(self.active) do
        count = count + 1
      end
      return count
    end

    function pool:DoesObjectBelongToPool(frame)
      return self.known[frame] == true
    end

    function pool:EnumerateActive()
      local frames = {}
      for frame in pairs(self.active) do
        frames[#frames + 1] = frame
      end
      local index = 0
      return function()
        index = index + 1
        return frames[index]
      end
    end

    return pool
  end
end

local function __wow_make_region_pool(acquire_region)
  return function(parent, layer, subLevel, template, resetter)
    local pool = {
      parent = parent,
      layer = layer,
      subLevel = subLevel,
      template = template,
      resetter = resetter,
      inactive = {},
      active = {},
      known = {},
    }

    function pool:Acquire()
      local region = table.remove(self.inactive)
      local isNew = false
      if region == nil then
        region = acquire_region(self.parent, self.layer, self.subLevel, self.template)
        isNew = true
        self.known[region] = true
      end
      self.active[region] = true
      return region, isNew
    end

    function pool:Release(region)
      if region == nil or not self:DoesObjectBelongToPool(region) then
        return false
      end
      self.active[region] = nil
      if self.resetter then
        self.resetter(self, region, false, self.template)
      end
      table.insert(self.inactive, region)
      return true
    end

    function pool:GetNumActive()
      local count = 0
      for _ in pairs(self.active) do
        count = count + 1
      end
      return count
    end

    function pool:DoesObjectBelongToPool(region)
      return self.known[region] == true
    end

    function pool:ReleaseAll()
      local regions = {}
      for region in pairs(self.active) do
        regions[#regions + 1] = region
      end
      for _, region in ipairs(regions) do
        self:Release(region)
      end
    end

    function pool:EnumerateActive()
      local regions = {}
      for region in pairs(self.active) do
        regions[#regions + 1] = region
      end
      local index = 0
      return function()
        index = index + 1
        return regions[index]
      end
    end

    return pool
  end
end

if CreateTexturePool == nil then
  CreateTexturePool = __wow_make_region_pool(function(parent, layer)
    return parent:CreateTexture(nil, layer or "ARTWORK")
  end)
end

if CreateFontStringPool == nil then
  CreateFontStringPool = __wow_make_region_pool(function(parent, layer)
    return parent:CreateFontString(nil, layer or "ARTWORK")
  end)
end

if CreateFramePoolCollection == nil then
  function CreateFramePoolCollection()
    local collection = { pools = {} }

    local function pool_key(frameType, parent, template, specialization)
      return table.concat({
        tostring(frameType or "Frame"),
        tostring(parent),
        tostring(template),
        tostring(specialization),
      }, "|")
    end

    function collection:CreatePool(frameType, parent, template, resetter, _forbidden, specialization)
      local key = pool_key(frameType, parent, template, specialization)
      local pool = CreateFramePool(frameType, parent, template, resetter)
      self.pools[key] = pool
      return pool
    end

    function collection:GetOrCreatePool(frameType, parent, template, resetter, forbidden, specialization)
      local key = pool_key(frameType, parent, template, specialization)
      local pool = self.pools[key]
      if pool == nil then
        pool = self:CreatePool(frameType, parent, template, resetter, forbidden, specialization)
      end
      return pool
    end

    function collection:GetNumActive()
      local total = 0
      for _, pool in pairs(self.pools) do
        total = total + (pool.GetNumActive and pool:GetNumActive() or 0)
      end
      return total
    end

    function collection:DoesObjectBelongToPool(object)
      for _, pool in pairs(self.pools) do
        if pool.DoesObjectBelongToPool and pool:DoesObjectBelongToPool(object) then
          return true
        end
      end
      return false
    end

    function collection:Release(object)
      for _, pool in pairs(self.pools) do
        if pool.Release and pool:Release(object) then
          return true
        end
      end
      return false
    end

    function collection:ReleaseAll()
      for _, pool in pairs(self.pools) do
        if pool.ReleaseAll then
          pool:ReleaseAll()
        end
      end
    end

    function collection:EnumerateActive()
      local objects = {}
      for _, pool in pairs(self.pools) do
        if pool.EnumerateActive then
          for object in pool:EnumerateActive() do
            objects[#objects + 1] = object
          end
        end
      end
      local index = 0
      return function()
        index = index + 1
        return objects[index]
      end
    end

    return collection
  end
end

if CreateFrameFactory == nil then
  function CreateFrameFactory()
    local cache = {}

    function cache:GetTemplateInfo(template)
      if C_XMLUtil and C_XMLUtil.GetTemplateInfo then
        local info = C_XMLUtil.GetTemplateInfo(template)
        if info then
          return info
        end
      end
      return { width = 0, height = 0 }
    end

    local factory = {}

    function factory:GetTemplateInfoCache()
      return cache
    end

    function factory:Create(parent, frameTypeOrTemplate, resetFunc)
      local frame = nil
      if type(frameTypeOrTemplate) == "string" and C_XMLUtil and C_XMLUtil.GetTemplateInfo and C_XMLUtil.GetTemplateInfo(frameTypeOrTemplate) then
        frame = CreateFrame("Frame", nil, parent, frameTypeOrTemplate)
      else
        frame = CreateFrame(type(frameTypeOrTemplate) == "string" and frameTypeOrTemplate or "Frame", nil, parent)
      end
      if resetFunc then
        resetFunc(nil, frame, true, frameTypeOrTemplate)
      end
      return frame, true
    end

    return factory
  end
end

if AddSourceLocationExclude == nil then
  function AddSourceLocationExclude()
  end
end

if GetGlobalEnvironment == nil then
  function GetGlobalEnvironment()
    return _G
  end
end

if GetButtonMetatable == nil then
  function GetButtonMetatable()
    if CreateFrame == nil then
      return nil
    end
    local frame = CreateFrame("Button")
    return frame and getmetatable(frame) or nil
  end
end

if secretwrap == nil then
  function secretwrap(fn)
    return fn
  end
end

if GetCallstackHeight == nil then
  function GetCallstackHeight()
    return 0
  end
end

if SetErrorCallstackHeight == nil then
  function SetErrorCallstackHeight()
  end
end

if GetBuildInfo == nil then
  function GetBuildInfo()
    return "12.0.5", "66102", "Apr 14 2026", 120005, "", " "
  end
end

if GetRealmName == nil then
  function GetRealmName()
    return "SimulatedRealm"
  end
end

if GetNormalizedRealmName == nil then
  function GetNormalizedRealmName()
    return "SimulatedRealm"
  end
end

if GetRealmID == nil then
  function GetRealmID()
    return 1
  end
end

if GetExpansionLevel == nil then
  function GetExpansionLevel()
    return 10
  end
end

if IsMacClient == nil then
  function IsMacClient()
    return false
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
    return false
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

StaticPopupDialogs = StaticPopupDialogs or {}

if StaticPopup_AddShowCondition == nil then
  function StaticPopup_AddShowCondition()
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

if RegisterEventCallback == nil then
  function RegisterEventCallback(_event, _callback)
  end
end

if DevTools_AddMessageHandler == nil then
  function DevTools_AddMessageHandler(_handler)
  end
end

if UnregisterEventCallback == nil then
  function UnregisterEventCallback(_event, _callback)
  end
end

if RegisterUnitEventCallback == nil then
  function RegisterUnitEventCallback(_event, _callback, _unit)
  end
end

if UnregisterUnitEventCallback == nil then
  function UnregisterUnitEventCallback(_event, _callback, _unit)
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

if EJ_GetInstanceInfo == nil then
  function EJ_GetInstanceInfo(_instanceID)
    return "", "", 0, 0, 0, 0, 0, 0, false, 0, 0, false
  end
end

if EJ_GetInstanceByIndex == nil then
  local __wow_ej_raid_instances = { 1200, 1208, 2549, 2657, 2522, 2569 }

  function EJ_GetInstanceByIndex(index, isRaid)
    if isRaid ~= true then
      return nil
    end

    local instanceID = __wow_ej_raid_instances[index]
    if instanceID == nil then
      return nil
    end

    local name, description, bgImage, buttonImage1, loreImage, buttonImage2, dungeonAreaMapID, linkRaidID = C_EncounterJournal.GetInstanceInfo(instanceID)
    return instanceID, name or "", description or "", bgImage or "", buttonImage1 or "", loreImage or "", buttonImage2 or "", 0, linkRaidID or 0, false, dungeonAreaMapID or 0
  end
end

local __wow_ej_tier_state = rawget(_G, "__wow_ej_tier_state")
if type(__wow_ej_tier_state) ~= "table" then
  __wow_ej_tier_state = {
    currentTier = GetClientDisplayExpansionLevel ~= nil and GetClientDisplayExpansionLevel() or 10,
  }
  rawset(_G, "__wow_ej_tier_state", __wow_ej_tier_state)
end

if EJ_GetCurrentTier == nil then
  function EJ_GetCurrentTier()
    return __wow_ej_tier_state.currentTier or 10
  end
end

if EJ_SelectTier == nil then
  function EJ_SelectTier(tier)
    if type(tier) == "number" then
      __wow_ej_tier_state.currentTier = tier
    end
  end
end

if EJ_GetLootFilter == nil then
  function EJ_GetLootFilter()
    return 0, 0
  end
end

if EJ_GetNumLoot == nil then
  function EJ_GetNumLoot()
    return 0
  end
end

if GetClientDisplayExpansionLevel == nil then
  function GetClientDisplayExpansionLevel()
    return 10
  end
end

if GetAccountExpansionLevel == nil then
  function GetAccountExpansionLevel()
    return GetClientDisplayExpansionLevel()
  end
end

if GetMaxLevelForExpansionLevel == nil then
  function GetMaxLevelForExpansionLevel(_expansion_level)
    return GetMaxPlayerLevel()
  end
end

if GetMaxLevelForPlayerExpansion == nil then
  function GetMaxLevelForPlayerExpansion()
    return GetMaxLevelForExpansionLevel(GetAccountExpansionLevel())
  end
end

if GetExpansionDisplayInfo == nil then
  function GetExpansionDisplayInfo(_expansionLevel, _desiredReleaseType)
    return {
      logo = 0,
      banner = "",
      features = {},
      highResBackgroundID = 0,
      lowResBackgroundID = 0,
      textureKit = "",
      glueAmbianceSoundKit = nil,
      glueMusicSoundKit = nil,
      glueCreditsSoundKit = nil,
    }
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

if GetShapeshiftFormInfo == nil then
  function GetShapeshiftFormInfo(_index)
    return nil, false, false, nil
  end
end

if GetTotemInfo == nil then
  function GetTotemInfo(_slot)
    return false, nil, 0, 0, nil
  end
end

if GetPetActionInfo == nil then
  function GetPetActionInfo(_index)
    return nil, nil, false, false, false, false, nil, false, false
  end
end

if GetPetActionCooldown == nil then
  function GetPetActionCooldown(_index)
    return 0, 0, 0
  end
end

if PetHasActionBar == nil then
  function PetHasActionBar()
    return false
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

if GetExpansionTrialInfo == nil then
  function GetExpansionTrialInfo()
    return false, 0
  end
end

if GetInventoryItemTexture == nil then
  function GetInventoryItemTexture(_unit, _slot)
    return nil
  end
end

if IsSelectedSpellBookItem == nil then
  function IsSelectedSpellBookItem(_slotIndex, _unit)
    return false
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
      return category
    end
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

local __cvars = rawget(_G, "__wow_cvars") or {}
rawset(_G, "__wow_cvars", __cvars)
-- Pre-seed CVars that Blizzard OnLoad code reads before any addon has
-- a chance to SetCVar. Each default matches the value WoW ships with.
local __cvar_defaults = {
  timeMgrAlarmTime = "0",
  timeMgrUseMilitaryTime = "0",
  timeMgrUseLocalTime = "0",
  showTimestamps = "none",
  spellActivationOverlayOpacity = "1.0",
}
for k, v in pairs(__cvar_defaults) do
  if __cvars[k] == nil then __cvars[k] = v end
end

C_CVar = C_CVar or __wow_namespace({
  GetCVar = function(name)
    return __cvars[name]
  end,
  SetCVar = function(name, value)
    __cvars[name] = value == nil and nil or tostring(value)
    return true
  end,
  GetCVarBool = function(name)
    local value = __cvars[name]
    return value ~= nil and value ~= "0" and value ~= false
  end,
  GetCVarDefault = function(name)
    return __cvars[name] or "0"
  end,
  RegisterCVar = __wow_noop,
  ResetTestCVars = __wow_noop,
  GetCVarBitfield = function() return false end,
  SetCVarBitfield = function() return true end,
})

C_UIColor = C_UIColor or __wow_namespace({
  GetColors = function()
    return {
      { baseTag = "HIGHLIGHT_FONT_COLOR", color = { r = 1, g = 1, b = 1, a = 1 } },
      { baseTag = "PLAYER_FACTION_COLOR_HORDE", color = { r = 1, g = 0.1, b = 0.1, a = 1 } },
      { baseTag = "PLAYER_FACTION_COLOR_ALLIANCE", color = { r = 0.2, g = 0.4, b = 1, a = 1 } },
      { baseTag = "NORMAL_FONT_COLOR", color = { r = 1, g = 0.82, b = 0, a = 1 } },
    }
  end,
})

C_ColorUtil = C_ColorUtil or __wow_namespace({
  ConvertRGBToHSV = function(r, g, b)
    return 0, 0, math.max(r or 0, g or 0, b or 0)
  end,
  ConvertHSVToHSL = function(h, s, v)
    return h or 0, s or 0, v or 0
  end,
  GenerateTextColorCode = function(color)
    local r = math.floor((color.r or 1) * 255)
    local g = math.floor((color.g or 1) * 255)
    local b = math.floor((color.b or 1) * 255)
    return string.format("ff%02x%02x%02x", r, g, b)
  end,
  WrapTextInColor = function(text, color)
    return "|c" .. C_ColorUtil.GenerateTextColorCode(color) .. tostring(text or "") .. "|r"
  end,
  WrapTextInColorCode = function(text, colorCode)
    local code = tostring(colorCode or "ffffffff"):gsub("^|c", "")
    return "|c" .. code .. tostring(text or "") .. "|r"
  end,
})

C_CurveUtil = C_CurveUtil or __wow_namespace({
  CreateCurve = nil,
  CreateColorCurve = nil,
})

C_EventUtils = C_EventUtils or __wow_namespace({
  IsEventValid = function() return true end,
})

C_FunctionContainers = C_FunctionContainers or __wow_namespace({
  CreateCallback = nil,
})

C_Club = C_Club or __wow_namespace()

local __wow_proxy_object_id = 1

local function __wow_next_proxy_label(prefix)
  local label = prefix .. ":" .. tostring(__wow_proxy_object_id)
  __wow_proxy_object_id = __wow_proxy_object_id + 1
  return label
end

local function __wow_make_proxy_object(prefix, methods, initial_state)
  local object = initial_state or {}
  local label = __wow_next_proxy_label(prefix)
  return setmetatable(object, {
    __index = function(t, key)
      local value = rawget(t, key)
      if value ~= nil then
        return value
      end
      return methods[key]
    end,
    __newindex = function(t, key, value)
      if methods[key] ~= nil then
        error("read-only key: " .. tostring(key), 2)
      end
      rawset(t, key, value)
    end,
    __tostring = function()
      return label
    end,
  })
end

local function __wow_clone_proxy_points(points)
  local copy = {}
  for index = 1, #(points or {}) do
    local point = points[index]
    copy[index] = {
      x = point.x,
      y = point.y,
    }
  end
  return copy
end

local function __wow_curve_methods(prefix)
  local methods = {}

  function methods:AddPoint(x, y)
    self.points[#self.points + 1] = { x = x or 0, y = y or 0 }
  end

  function methods:SetType(curveType)
    self.curveType = curveType or 0
  end

  function methods:GetPointCount()
    return #self.points
  end

  function methods:Evaluate(x)
    local points = self.points
    if #points == 0 then
      return 0
    end
    if #points == 1 then
      return points[1].y
    end

    local target = x or 0
    for index = 1, #points - 1 do
      local left = points[index]
      local right = points[index + 1]
      if target <= right.x then
        local dx = right.x - left.x
        if dx == 0 then
          return right.y
        end
        local fraction = (target - left.x) / dx
        return left.y + (right.y - left.y) * fraction
      end
    end

    return points[#points].y
  end

  function methods:Copy()
    return __wow_make_proxy_object(prefix, methods, {
      points = __wow_clone_proxy_points(self.points),
      curveType = self.curveType,
    })
  end

  return methods
end

if rawget(C_CurveUtil, "CreateCurve") == nil then
  local curveMethods = __wow_curve_methods("LuaCurveObject")
  function C_CurveUtil.CreateCurve()
    return __wow_make_proxy_object("LuaCurveObject", curveMethods, {
      points = {},
      curveType = 0,
    })
  end
end

if rawget(C_CurveUtil, "CreateColorCurve") == nil then
  local colorCurveMethods = __wow_curve_methods("LuaColorCurveObject")
  function C_CurveUtil.CreateColorCurve()
    return __wow_make_proxy_object("LuaColorCurveObject", colorCurveMethods, {
      points = {},
      curveType = 0,
    })
  end
end

if rawget(C_FunctionContainers, "CreateCallback") == nil then
  local functionContainerMethods = {}

  function functionContainerMethods:Cancel()
    self._cancelled = true
  end

  function functionContainerMethods:IsCancelled()
    return self._cancelled == true
  end

  function functionContainerMethods:Invoke(...)
    if self._cancelled or type(self._callback) ~= "function" then
      return nil
    end
    return self._callback(...)
  end

  function C_FunctionContainers.CreateCallback(fn)
    return __wow_make_proxy_object("LuaFunctionContainer", functionContainerMethods, {
      _callback = fn,
      _cancelled = false,
    })
  end
end

if rawget(C_Club, "GetClubInfo") == nil then
  function C_Club.GetClubInfo(clubId)
    if clubId == nil then
      return nil
    end
    return { id = clubId }
  end
end

if CreateAbbreviateConfig == nil then
  local abbreviateMethods = {}

  function abbreviateMethods:GetAbbreviateNumberData()
    return self._abbreviateNumberData
  end

  function abbreviateMethods:SetAbbreviateNumberData(data)
    self._abbreviateNumberData = data
  end

  function CreateAbbreviateConfig(initial)
    local state = type(initial) == "table" and __wow_copy_table(initial) or {}
    state._abbreviateNumberData = state._abbreviateNumberData
    return __wow_make_proxy_object("AbbreviateConfig", abbreviateMethods, state)
  end
end

if CreateUnitHealPredictionCalculator == nil then
  local healPredictionMethods = {}

  function healPredictionMethods:Reset()
    self._damageAbsorbClampMode = 0
    self._incomingHeals = 0
  end

  function healPredictionMethods:GetIncomingHeals()
    return self._incomingHeals or 0
  end

  function healPredictionMethods:GetDamageAbsorbClampMode()
    return self._damageAbsorbClampMode or 0
  end

  function healPredictionMethods:SetDamageAbsorbClampMode(mode)
    self._damageAbsorbClampMode = mode or 0
  end

  function CreateUnitHealPredictionCalculator()
    return __wow_make_proxy_object("UnitHealPredictionCalculator", healPredictionMethods, {
      _damageAbsorbClampMode = 0,
      _incomingHeals = 0,
    })
  end
end

C_DurationUtil = C_DurationUtil or __wow_namespace({
  CreateDuration = __wow_duration_object(),
  GetCurrentTime = function()
    return 0
  end,
})

TextureKitConstants = TextureKitConstants or {
  SetVisibility = true,
  DoNotSetVisibility = false,
  UseAtlasSize = true,
  IgnoreAtlasSize = false,
  AddressModeClamp = 1,
  AddressModeWrap = 2,
  AddressModeAllowAssetToDetermine = 3,
}

if GetIconForRole == nil then
  local roleIcons = {
    GUIDE = "UI-LFG-RoleIcon-Leader",
    TANK = "UI-LFG-RoleIcon-Tank",
    HEALER = "UI-LFG-RoleIcon-Healer",
    DAMAGER = "UI-LFG-RoleIcon-DPS",
    NONE = "UI-LFG-RoleIcon-DPS",
  }
  local disabledRoleIcons = {
    GUIDE = "UI-LFG-RoleIcon-Leader-Disabled",
    TANK = "UI-LFG-RoleIcon-Tank-Disabled",
    HEALER = "UI-LFG-RoleIcon-Healer-Disabled",
    DAMAGER = "UI-LFG-RoleIcon-DPS-Disabled",
    NONE = "UI-LFG-RoleIcon-DPS-Disabled",
  }
  function GetIconForRole(role, showDisabled)
    local iconSet = showDisabled and disabledRoleIcons or roleIcons
    return iconSet[role] or iconSet.NONE
  end
end

if GetIconForRoleEnum == nil then
  function GetIconForRoleEnum(role, showDisabled)
    local roleName = "NONE"
    if role == 0 then
      roleName = "TANK"
    elseif role == 1 then
      roleName = "HEALER"
    elseif role == 2 then
      roleName = "DAMAGER"
    elseif Constants ~= nil
        and Constants.LFG_ROLEConstants ~= nil
        and role == Constants.LFG_ROLEConstants.LFG_ROLE_NO_ROLE then
      roleName = "GUIDE"
    end
    return GetIconForRole(roleName, showDisabled)
  end
end

C_Sound = C_Sound or __wow_namespace()
if C_UI == nil then
  C_UI = __wow_namespace()
end
if C_UI.DoesAnyDisplayHaveNotch == nil then
  function C_UI.DoesAnyDisplayHaveNotch()
    return false
  end
end

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

-- C_GameRules.* probes listed in PLAN are registered from Rust
-- (src/lua_api/globals/game_rules.rs), backed by SimState::game_rules.
-- Admin: A_Admin.SetGameRule(name, value) / A_Admin.SetActiveGameMode(mode,
-- glueScreen?). Merge the stub-namespace __index so unimplemented members
-- (IsHardcoreActive, etc.) still return the no-op function expected by
-- Blizzard callsites.
C_GameRules = __wow_merge_namespace(C_GameRules, {})
GameRulesUtil = GameRulesUtil or {}
if rawget(GameRulesUtil, "ShouldShowPlayerCastBar") == nil then
  function GameRulesUtil.ShouldShowPlayerCastBar()
    return true
  end
end

-- Pet battles: not simulated. `GetNumPets` is compared numerically
-- during PetBattleFrame OnLoad refresh, so returning nil crashes
-- `petIndex > GetNumPets(owner)`. Zero is the accurate "no pets" answer.
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
  __wow_pet_battle_state.battleState = __wow_pet_battle_waiting_state
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
  __wow_pet_battle_ensure_active()
  return (__wow_pet_battle_state.battleState or 0) ~= __wow_pet_battle_finished_state
end
C_PetBattles.IsWildBattle = function()
  __wow_pet_battle_ensure_active()
  return __wow_pet_battle_state.isWildBattle == true
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
C_PetBattles.GetXP = function(owner, petIndex)
  local pet = __wow_pet_battle_get_pet(owner, petIndex)
  if not pet then
    return 0, 0
  end
  return pet.xp or 0, pet.maxXP or 0
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
C_PetBattles.StartPVPMatchmaking = function()
  __wow_pet_battle_ensure_active()
  __wow_pet_battle_state.queueStatus = Enum.PetBattleQueueStatus and Enum.PetBattleQueueStatus.Matchmaking or 1
  __wow_pet_battle_state.canAcceptQueuedPVPMatch = true
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

-- LFG group-finder probes. Neither applies in the sim: no group-finder
-- usage and no active proposal. `GetLFGProposal` returns 15 values
-- callers destructure, so match that shape.
if rawget(C_LFGInfo or {}, "CanPlayerUseGroupFinder") == nil then
  C_LFGInfo = C_LFGInfo or __wow_namespace()
  function C_LFGInfo.CanPlayerUseGroupFinder()
    return false, ""
  end
end
if rawget(C_LFGInfo or {}, "IsInLFGFollowerDungeon") == nil then
  C_LFGInfo = C_LFGInfo or __wow_namespace()
  function C_LFGInfo.IsInLFGFollowerDungeon()
    return false
  end
end
if GetLFGProposal == nil then
  function GetLFGProposal()
    -- (proposalExists, id, typeID, subtypeID, name, backgroundTexture,
    --  role, hasResponded, totalEncounters/numBosses, completedEncounters,
    --  numMembers, isLeader, isHoliday, _, isSilent)
    return false, 0, 0, 0, "", "", "", false, 0, 0, 0, false, false, nil, false
  end
end
if GetLFGProposalEncounter == nil then
  function GetLFGProposalEncounter(_i)
    return "", "", false
  end
end
if GetLFGInfoServer == nil then
  function GetLFGInfoServer()
    return false, false, false, false, false, 0, 0, 0, ""
  end
end
if GetLFGRoleUpdate == nil then
  function GetLFGRoleUpdate()
    -- (inProgress, slots, members, category, lfgID, bgQueue)
    return false, 0, 0, 0, 0, false
  end
end
function HasCompletedAnyAchievement()
  return true
end
function CanShowAchievementUI()
  return true
end
if GetTotalAchievementPoints == nil then
  function GetTotalAchievementPoints()
    return 0
  end
end
if GetPartyLFGID == nil then
  function GetPartyLFGID() return 0 end
end

-- Adventure journal: the sim has no adventure content.
C_AdventureJournal = C_AdventureJournal or __wow_namespace()
if rawget(C_AdventureJournal, "CanBeShown") == nil then
  function C_AdventureJournal.CanBeShown()
    return true
  end
end
if rawget(C_AdventureJournal, "UpdateSuggestions") == nil then
  function C_AdventureJournal.UpdateSuggestions(_forceUpdate)
  end
end
if rawget(C_AdventureJournal, "GetPrimaryOffset") == nil then
  function C_AdventureJournal.GetPrimaryOffset()
    return 0
  end
end
if rawget(C_AdventureJournal, "SetPrimaryOffset") == nil then
  function C_AdventureJournal.SetPrimaryOffset(_offset)
  end
end
if rawget(C_AdventureJournal, "GetNumAvailableSuggestions") == nil then
  function C_AdventureJournal.GetNumAvailableSuggestions()
    return 0
  end
end
if rawget(C_AdventureJournal, "GetSuggestions") == nil then
  function C_AdventureJournal.GetSuggestions(suggestions)
    if type(suggestions) == "table" then
      for index = #suggestions, 1, -1 do
        suggestions[index] = nil
      end
    end
  end
end
if rawget(C_AdventureJournal, "GetReward") == nil then
  function C_AdventureJournal.GetReward(_suggestionIndex)
    return nil
  end
end
if rawget(C_AdventureJournal, "ActivateEntry") == nil then
  function C_AdventureJournal.ActivateEntry(_suggestionIndex)
  end
end

if type(AdventureGuideUtil) ~= "table" then
  AdventureGuideUtil = {}
end
if rawget(AdventureGuideUtil, "IsAvailable") == nil then
  function AdventureGuideUtil.IsAvailable()
    local kioskEnabled = Kiosk and Kiosk.IsEnabled and Kiosk.IsEnabled()
    return not kioskEnabled and C_AdventureJournal.CanBeShown()
  end
end
if rawget(AdventureGuideUtil, "OpenJournalLink") == nil then
  function AdventureGuideUtil.OpenJournalLink(_journalType, _id, _difficultyID)
    if not EncounterJournal and type(EncounterJournal_LoadUI) == "function" then
      EncounterJournal_LoadUI()
    end
    if EncounterJournal then
      ShowUIPanel(EncounterJournal)
      return true
    end
    return false
  end
end
if rawget(AdventureGuideUtil, "OpenHyperLink") == nil then
  function AdventureGuideUtil.OpenHyperLink(_tag, journalType, id, difficultyID)
    if not AdventureGuideUtil.IsAvailable() then
      return false
    end
    return AdventureGuideUtil.OpenJournalLink(
      tonumber(journalType),
      tonumber(id),
      tonumber(difficultyID)
    )
  end
end
if rawget(AdventureGuideUtil, "GetCurrentJournalInstance") == nil then
  function AdventureGuideUtil.GetCurrentJournalInstance()
    return nil
  end
end
if rawget(AdventureGuideUtil, "IsInInstance") == nil then
  function AdventureGuideUtil.IsInInstance(_journalInstanceID)
    return false
  end
end

if type(DifficultyUtil) ~= "table" then
  DifficultyUtil = {}
end
if rawget(DifficultyUtil, "ID") == nil then
  DifficultyUtil.ID = {
    DungeonNormal = 1,
    DungeonHeroic = 2,
    Raid10Normal = 3,
    Raid25Normal = 4,
    Raid10Heroic = 5,
    Raid25Heroic = 6,
    RaidLFR = 7,
    DungeonChallenge = 8,
    Raid40 = 9,
    PrimaryRaidNormal = 14,
    PrimaryRaidHeroic = 15,
    PrimaryRaidMythic = 16,
    PrimaryRaidLFR = 17,
    DungeonMythic = 23,
    DungeonTimewalker = 24,
    RaidTimewalker = 33,
    RaidStory = 220,
  }
end
if rawget(DifficultyUtil, "GetDifficultyName") == nil then
  local __wow_difficulty_names = {
    [DifficultyUtil.ID.DungeonNormal] = PLAYER_DIFFICULTY1 or "Normal",
    [DifficultyUtil.ID.DungeonHeroic] = PLAYER_DIFFICULTY2 or "Heroic",
    [DifficultyUtil.ID.Raid10Normal] = PLAYER_DIFFICULTY1 or "Normal",
    [DifficultyUtil.ID.Raid25Normal] = PLAYER_DIFFICULTY1 or "Normal",
    [DifficultyUtil.ID.Raid10Heroic] = PLAYER_DIFFICULTY2 or "Heroic",
    [DifficultyUtil.ID.Raid25Heroic] = PLAYER_DIFFICULTY2 or "Heroic",
    [DifficultyUtil.ID.RaidLFR] = PLAYER_DIFFICULTY3 or "Raid Finder",
    [DifficultyUtil.ID.DungeonChallenge] = PLAYER_DIFFICULTY_MYTHIC_PLUS or "Mythic+",
    [DifficultyUtil.ID.Raid40] = LEGACY_RAID_DIFFICULTY or "Legacy Raid",
    [DifficultyUtil.ID.PrimaryRaidNormal] = PLAYER_DIFFICULTY1 or "Normal",
    [DifficultyUtil.ID.PrimaryRaidHeroic] = PLAYER_DIFFICULTY2 or "Heroic",
    [DifficultyUtil.ID.PrimaryRaidMythic] = PLAYER_DIFFICULTY6 or "Mythic",
    [DifficultyUtil.ID.PrimaryRaidLFR] = PLAYER_DIFFICULTY3 or "Raid Finder",
    [DifficultyUtil.ID.DungeonMythic] = PLAYER_DIFFICULTY6 or "Mythic",
    [DifficultyUtil.ID.DungeonTimewalker] = PLAYER_DIFFICULTY_TIMEWALKER or "Timewalking",
    [DifficultyUtil.ID.RaidTimewalker] = PLAYER_DIFFICULTY_TIMEWALKER or "Timewalking",
    [DifficultyUtil.ID.RaidStory] = PLAYER_DIFFICULTY_STORY_RAID or "Story",
  }

  function DifficultyUtil.GetDifficultyName(difficultyID)
    return __wow_difficulty_names[difficultyID]
  end
end
if rawget(DifficultyUtil, "IsPrimaryRaid") == nil then
  local __wow_primary_raids = {
    [DifficultyUtil.ID.PrimaryRaidLFR] = true,
    [DifficultyUtil.ID.PrimaryRaidNormal] = true,
    [DifficultyUtil.ID.PrimaryRaidHeroic] = true,
    [DifficultyUtil.ID.PrimaryRaidMythic] = true,
  }

  function DifficultyUtil.IsPrimaryRaid(difficultyID)
    return __wow_primary_raids[difficultyID] or false
  end
end
if rawget(DifficultyUtil, "GetMaxPlayers") == nil then
  local __wow_max_players = {
    [DifficultyUtil.ID.DungeonNormal] = 5,
    [DifficultyUtil.ID.DungeonHeroic] = 5,
    [DifficultyUtil.ID.DungeonMythic] = 5,
    [DifficultyUtil.ID.DungeonChallenge] = 5,
    [DifficultyUtil.ID.DungeonTimewalker] = 5,
    [DifficultyUtil.ID.Raid10Normal] = 10,
    [DifficultyUtil.ID.Raid10Heroic] = 10,
    [DifficultyUtil.ID.Raid25Normal] = 25,
    [DifficultyUtil.ID.Raid25Heroic] = 25,
    [DifficultyUtil.ID.Raid40] = 40,
  }

  function DifficultyUtil.GetMaxPlayers(difficultyID)
    return __wow_max_players[difficultyID]
  end
end

if type(PVPUtil) ~= "table" then
  PVPUtil = {}
end
if rawget(PVPUtil, "GetTierName") == nil then
  function PVPUtil.GetTierName(_tierEnum)
    return ""
  end
end
if rawget(PVPUtil, "GetTierDescription") == nil then
  function PVPUtil.GetTierDescription(_tierEnum)
    return ""
  end
end
if rawget(PVPUtil, "GetBracketName") == nil then
  function PVPUtil.GetBracketName(_bracketIndex)
    return ""
  end
end
if rawget(PVPUtil, "IsInActiveBattlefield") == nil then
  function PVPUtil.IsInActiveBattlefield()
    return false
  end
end
if rawget(PVPUtil, "GetCurrentSeasonNumber") == nil then
  function PVPUtil.GetCurrentSeasonNumber()
    return 0
  end
end

if type(PlayerSpellsUtil) ~= "table" then
  PlayerSpellsUtil = {}
end
if rawget(PlayerSpellsUtil, "FrameTabs") == nil then
  PlayerSpellsUtil.FrameTabs = {
    ClassSpecializations = 1,
    ClassTalents = 2,
    SpellBook = 3,
  }
end
if rawget(PlayerSpellsUtil, "SpellBookCategories") == nil then
  PlayerSpellsUtil.SpellBookCategories = {
    Class = 1,
    General = 2,
    Pet = 3,
  }
end

local function __wow_load_player_spells_frame()
  if not PlayerSpellsFrame and type(PlayerSpellsFrame_LoadUI) == "function" then
    PlayerSpellsFrame_LoadUI()
  end
  return PlayerSpellsFrame
end

local function __wow_set_playerspells_inspect_unit(inspectUnit)
  local frame = PlayerSpellsFrame
  if not frame then
    return
  end
  if inspectUnit and type(frame.SetInspectUnit) == "function" then
    frame:SetInspectUnit(inspectUnit)
  elseif type(frame.IsInspecting) == "function"
    and frame:IsInspecting()
    and type(frame.ClearInspectUnit) == "function"
  then
    frame:ClearInspectUnit()
  end
end

if rawget(PlayerSpellsUtil, "GetCurrentTabID") == nil then
  function PlayerSpellsUtil.GetCurrentTabID()
    local frame = __wow_load_player_spells_frame()
    if not frame or type(frame.GetCurrentTabID) ~= "function" then
      return nil
    end
    return frame:GetCurrentTabID()
  end
end
if rawget(PlayerSpellsUtil, "TogglePlayerSpellsFrame") == nil then
  function PlayerSpellsUtil.TogglePlayerSpellsFrame(suggestedTab, inspectUnit)
    if DISALLOW_FRAME_TOGGLING then
      return false
    end

    local frame = __wow_load_player_spells_frame()
    if not frame then
      return false
    end

    if not suggestedTab
      and type(frame.ShouldOpenToSpecTab) == "function"
      and frame:ShouldOpenToSpecTab()
    then
      suggestedTab = PlayerSpellsUtil.FrameTabs.ClassSpecializations
    end

    local alreadyShowing = frame:IsShown()
      and (
        not suggestedTab
        or (type(frame.IsFrameTabActive) == "function" and frame:IsFrameTabActive(suggestedTab))
      )

    if alreadyShowing then
      HideUIPanel(frame)
      return true
    end

    __wow_set_playerspells_inspect_unit(inspectUnit)
    if suggestedTab and type(frame.TrySetTab) == "function" and not frame:TrySetTab(suggestedTab) then
      return false
    end
    ShowUIPanel(frame)
    return true
  end
end
if rawget(PlayerSpellsUtil, "OpenToSpellBookTabAtSpell") == nil then
  function PlayerSpellsUtil.OpenToSpellBookTabAtSpell(spellID, knownSpellsOnly, toggleFlyout, flyoutReason)
    local frame = __wow_load_player_spells_frame()
    if not frame then
      return nil
    end
    __wow_set_playerspells_inspect_unit(nil)
    if type(frame.TrySetTab) == "function" and not frame:TrySetTab(PlayerSpellsUtil.FrameTabs.SpellBook) then
      return nil
    end
    ShowUIPanel(frame)
    local spellBook = frame.SpellBookFrame
    if type(spellBook) == "table" and type(spellBook.GoToSpell) == "function" then
      return spellBook:GoToSpell(spellID, knownSpellsOnly, toggleFlyout, flyoutReason)
    end
    return nil
  end
end
if rawget(PlayerSpellsUtil, "ToggleSpellBookFrame") == nil then
  function PlayerSpellsUtil.ToggleSpellBookFrame(spellBookCategory)
    if DISALLOW_FRAME_TOGGLING then
      return false
    end
    local frame = __wow_load_player_spells_frame()
    if not frame then
      return false
    end

    local spellBook = frame.SpellBookFrame
    local categoryMatches = (
      not spellBookCategory
      or not spellBook
      or type(spellBook.IsCategoryActive) ~= "function"
      or spellBook:IsCategoryActive(spellBookCategory)
    )
    local alreadyShowing = frame:IsShown()
      and type(frame.IsFrameTabActive) == "function"
      and frame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.SpellBook)
      and categoryMatches

    if alreadyShowing then
      HideUIPanel(frame)
      return true
    end

    __wow_set_playerspells_inspect_unit(nil)
    if type(frame.TrySetTab) == "function" and not frame:TrySetTab(PlayerSpellsUtil.FrameTabs.SpellBook) then
      return false
    end
    if spellBookCategory and spellBook and type(spellBook.TrySetCategory) == "function" then
      spellBook:TrySetCategory(spellBookCategory)
    end
    ShowUIPanel(frame)
    return true
  end
end
if rawget(PlayerSpellsUtil, "ToggleClassTalentFrame") == nil then
  function PlayerSpellsUtil.ToggleClassTalentFrame(inspectUnit)
    return PlayerSpellsUtil.TogglePlayerSpellsFrame(PlayerSpellsUtil.FrameTabs.ClassTalents, inspectUnit)
  end
end
if rawget(PlayerSpellsUtil, "OpenToClassTalentsTab") == nil then
  function PlayerSpellsUtil.OpenToClassTalentsTab(inspectUnit)
    return PlayerSpellsUtil.TogglePlayerSpellsFrame(PlayerSpellsUtil.FrameTabs.ClassTalents, inspectUnit)
  end
end
if rawget(PlayerSpellsUtil, "OpenToClassSpecializationsTab") == nil then
  function PlayerSpellsUtil.OpenToClassSpecializationsTab()
    return PlayerSpellsUtil.TogglePlayerSpellsFrame(PlayerSpellsUtil.FrameTabs.ClassSpecializations)
  end
end
if rawget(PlayerSpellsUtil, "OpenToSpellBookTab") == nil then
  function PlayerSpellsUtil.OpenToSpellBookTab()
    return PlayerSpellsUtil.ToggleSpellBookFrame()
  end
end
if TogglePlayerSpellsFrame == nil then
  function TogglePlayerSpellsFrame(suggestedTab, inspectUnit)
    return PlayerSpellsUtil.TogglePlayerSpellsFrame(suggestedTab, inspectUnit)
  end
end

if type(StaticModelInfo) ~= "table" then
  StaticModelInfo = {}
end
if rawget(StaticModelInfo, "CreateModelSceneEntry") == nil then
  function StaticModelInfo.CreateModelSceneEntry(sceneID, displayID)
    return {
      sceneID = sceneID,
      displayID = displayID,
    }
  end
end

-- Store / shop public API: sim has no store.
C_StorePublic = C_StorePublic or __wow_namespace()
if rawget(C_StorePublic, "IsEnabled") == nil then
  function C_StorePublic.IsEnabled() return true end
end
if rawget(C_StorePublic, "IsDisabledByParentalControls") == nil then
  function C_StorePublic.IsDisabledByParentalControls() return false end
end

-- Additional LFG helpers.
if GetLFGCategoryForID == nil then
  function GetLFGCategoryForID() return 0 end
end

-- Battle.net friends count: sim has no BNet connection.
if BNGetNumFriends == nil then
  function BNGetNumFriends() return 0, 0 end
end
if BNGetNumFriendInvites == nil then
  function BNGetNumFriendInvites() return 0 end
end

-- Friend list: sim has no social layer.
C_FriendList = C_FriendList or __wow_namespace()
if rawget(C_FriendList, "GetNumFriends") == nil then
  function C_FriendList.GetNumFriends() return 0 end
end
if rawget(C_FriendList, "GetNumOnlineFriends") == nil then
  function C_FriendList.GetNumOnlineFriends() return 0 end
end

-- Region / language / battlefield stubs.
if GetCurrentRegionName == nil then
  function GetCurrentRegionName() return "US" end
end
if GetDefaultLanguage == nil then
  function GetDefaultLanguage() return "Common", 1 end
end
if GetMaxBattlefieldID == nil then
  function GetMaxBattlefieldID() return 0 end
end
if IsActiveBattlefieldArena == nil then
  function IsActiveBattlefieldArena() return false end
end
if UnitExists == nil then
  function UnitExists(unit)
    return unit == "player"
  end
end

-- Social / commentator: sim has no social restrictions or spectator.
C_SocialRestrictions = C_SocialRestrictions or __wow_namespace()
if rawget(C_SocialRestrictions, "IsChatDisabled") == nil then
  function C_SocialRestrictions.IsChatDisabled() return false end
end
C_Commentator = C_Commentator or __wow_namespace()
if rawget(C_Commentator, "IsSpectating") == nil then
  function C_Commentator.IsSpectating() return false end
end
if rawget(C_Commentator, "SendAddonMessage") == nil then
  function C_Commentator.SendAddonMessage(_prefix, _message, _channel)
    return Enum and Enum.SendAddonMessageResult and Enum.SendAddonMessageResult.Success or 0
  end
end

C_CampaignInfo = __wow_merge_namespace(C_CampaignInfo, {
  GetCampaignID = function(campaignID)
    return tonumber(campaignID) or 0
  end,
  GetState = function(_campaignID)
    return Enum and Enum.CampaignState and Enum.CampaignState.Invalid or 0
  end,
})

C_CovenantSanctumUI = __wow_merge_namespace(C_CovenantSanctumUI, {
  GetRenownRewardsForLevel = function(factionID, level)
    if tonumber(factionID) == 1 and tonumber(level) == 5 then
      return {
        {
          name = "Path of Ascension",
          description = "Unlocks a new covenant activity.",
          toastDescription = "Path of Ascension unlocked",
          icon = 4089529,
        },
      }
    end
    return {}
  end,
})

local __wow_level_link_state = type(C_LevelLink) == "table" and rawget(C_LevelLink, "_state") or nil
C_LevelLink = __wow_merge_namespace(C_LevelLink, {
  _state = __wow_level_link_state or {
    lockedActions = {},
    lockedSpells = {},
    lastActionQuery = nil,
    lastSpellQuery = nil,
  },
})

if rawget(C_LevelLink, "IsActionLocked") == nil then
  function C_LevelLink.IsActionLocked(actionID)
    local normalized = tonumber(actionID)
    if normalized == nil then
      C_LevelLink._state.lastActionQuery = nil
      return false
    end
    local entry = C_LevelLink._state.lockedActions[normalized]
    C_LevelLink._state.lastActionQuery = normalized
    if type(entry) == "table" then
      return entry.locked == true
    end
    return entry == true
  end
end

if rawget(C_LevelLink, "IsSpellLocked") == nil then
  function C_LevelLink.IsSpellLocked(spellID)
    local normalized = tonumber(spellID)
    if normalized == nil then
      C_LevelLink._state.lastSpellQuery = nil
      return false
    end
    local entry = C_LevelLink._state.lockedSpells[normalized]
    C_LevelLink._state.lastSpellQuery = normalized
    if type(entry) == "table" then
      return entry.locked == true
    end
    return entry == true
  end
end

-- Guild bank: not simulated; single callsite in GuildControlUI.
C_GuildBank = C_GuildBank or __wow_namespace()

-- C_GuildInfo.GetClubId / IsGuildOfficer / CanSpeakInGuildChat are
-- registered from Rust (src/lua_api/globals/guild_info.rs), backed by
-- SimState::world.guild_club_id / guild_is_officer / guild_can_speak_in_chat.
-- Merge the stub-namespace __index fallback so other unimplemented
-- C_GuildInfo members resolve to the no-op metamethod.
C_GuildInfo = __wow_merge_namespace(C_GuildInfo, {})
if rawget(C_GuildInfo, "_textState") == nil then
  C_GuildInfo._textState = {
    motd = "Raid invites tonight at 20:00 server. Repairs are on for progression.",
    infoText = "Mythic-focused guild recruiting healers and a warlock for weekend raids.",
  }
end
if rawget(C_GuildInfo, "GetMOTD") == nil then
  function C_GuildInfo.GetMOTD()
    return C_GuildInfo._textState.motd
  end
end
if rawget(C_GuildInfo, "GetInfoText") == nil then
  function C_GuildInfo.GetInfoText()
    return C_GuildInfo._textState.infoText
  end
end
if rawget(C_GuildInfo, "SetInfoText") == nil then
  function C_GuildInfo.SetInfoText(text)
    C_GuildInfo._textState.infoText = tostring(text or "")
  end
end
-- GetAvailableLocaleInfo is registered from Rust
-- (src/lua_api/globals/locale_info.rs). Returns the 12-locale retail list
-- as { localeId, localeName, englishName, displayName } entries.
if GetGuildFactionGroup == nil then
  function GetGuildFactionGroup()
    return 1
  end
end
-- GuildControlSetRank / GuildControlGetRankName / GuildControlGetNumRanks /
-- GuildControlGetRankFlags are registered from Rust
-- (src/lua_api/globals/guild_control.rs), backed by SimState::world.guild_ranks.
-- Admin: A_Admin.SetGuildRanks({ {name="Leader", flags={true,...}}, ... }).
if GetGroupMemberCounts == nil then
  function GetGroupMemberCounts()
    return {
      TANK = 0,
      HEALER = 0,
      DAMAGER = 0,
      NOROLE = 0,
    }
  end
end
if GetLootSpecialization == nil then
  function GetLootSpecialization()
    return 0
  end
end

if GetSpellConfirmationPromptsInfo == nil then
  function GetSpellConfirmationPromptsInfo()
    return {}
  end
end

if GetActiveLootRollIDs == nil then
  function GetActiveLootRollIDs()
    return {}
  end
end

if GetNumArenaOpponents == nil then
  function GetNumArenaOpponents()
    return 0
  end
end
if C_EditMode == nil then
  C_EditMode = __wow_namespace()
end
if rawget(C_EditMode, "GetAccountSettings") == nil then
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

  function C_EditMode.GetAccountSettings()
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

  function C_EditMode.GetLayouts()
    return {
      layouts = {},
      activeLayout = 1,
    }
  end

  function C_EditMode.SetActiveLayout(_layoutIndex)
  end
end
if WorldLootObjectExists == nil then
  function WorldLootObjectExists(_unit)
    return false
  end
end

-- Housing: not simulated. The only surface accessed at load is
-- IsHousingServiceEnabled (MainMenuBarMicroButtons gates the housing
-- micro-button on it).
-- C_Housing.IsHousingServiceEnabled is registered from Rust
-- (src/lua_api/globals/housing.rs), backed by SimState::housing_service_enabled.
-- Admin: A_Admin.SetHousingServiceEnabled(b?).
-- Merge stub-namespace fallback so other unimplemented C_Housing members
-- resolve to the no-op metamethod.
C_Housing = __wow_merge_namespace(C_Housing, {})
C_RestrictedActions = __wow_merge_namespace(C_RestrictedActions, {
  CheckAllowProtectedFunctions = function()
    return true
  end,
})
C_ScriptedAnimations = C_ScriptedAnimations or __wow_namespace()
C_PaperDollInfo = C_PaperDollInfo or __wow_namespace()
C_CombatAudioAlert = C_CombatAudioAlert or __wow_namespace()
C_ContentTracking = __wow_merge_namespace(C_ContentTracking, {
  -- AchievementObjectiveTracker iterates `#trackedAchievements` at load.
  -- Return an empty list so the subsequent for-loop is a no-op.
  GetTrackedIDs = function() return {} end,
  IsTracking = function() return false end,
})

-- InitiativeTasksObjectiveTracker indexes `.trackedIDs` on the returned
-- value, so return a real table even when there are no initiatives.
C_NeighborhoodInitiative = __wow_merge_namespace(C_NeighborhoodInitiative, {
  IsInitiativeEnabled = function() return false end,
  GetAvailableHouseXP = function() return 0 end,
  GetTrackedInitiativeTasks = function()
    return { trackedIDs = {} }
  end,
  GetInitiativeTaskInfo = function() return nil end,
  RemoveTrackedInitiativeTask = __wow_noop,
  AddTrackedInitiativeTask = __wow_noop,
})
C_Widget = C_Widget or __wow_namespace()
C_SuperTrack = __wow_merge_namespace(C_SuperTrack, {
  GetSuperTrackedQuestID = function() return 0 end,
  SetSuperTrackedQuestID = __wow_noop,
})
C_AutoComplete = __wow_merge_namespace(C_AutoComplete, {
  GetAutoCompleteRealms = function() return {} end,
})
C_TransmogOutfitInfo = C_TransmogOutfitInfo or __wow_namespace({
  __activeOutfitID = 0,
  __currentlyViewedOutfitID = 0,
  __pendingSheatheCategories = {},
  GetOutfitInfo = function() return nil end,
})
if rawget(C_TransmogOutfitInfo, "GetActiveOutfitID") == nil then
  function C_TransmogOutfitInfo.GetActiveOutfitID()
    return rawget(C_TransmogOutfitInfo, "__activeOutfitID") or 0
  end
end
if rawget(C_TransmogOutfitInfo, "GetCurrentlyViewedOutfitID") == nil then
  function C_TransmogOutfitInfo.GetCurrentlyViewedOutfitID()
    return rawget(C_TransmogOutfitInfo, "__currentlyViewedOutfitID") or 0
  end
end
if rawget(C_TransmogOutfitInfo, "GetAllTransmogOutfitOptionSheatheCategoryInfo") == nil then
  function C_TransmogOutfitInfo.GetAllTransmogOutfitOptionSheatheCategoryInfo(slotTransmogID)
    if tonumber(slotTransmogID) ~= 190001 then
      return nil
    end
    return {
      {
        sheatheCategory = Enum.TransmogOutfitSlotOptionSheatheCategory.Default,
        categoryName = "Default",
      },
      {
        sheatheCategory = Enum.TransmogOutfitSlotOptionSheatheCategory.Back,
        categoryName = "Back",
      },
      {
        sheatheCategory = Enum.TransmogOutfitSlotOptionSheatheCategory.Side,
        categoryName = "Side",
      },
      {
        sheatheCategory = Enum.TransmogOutfitSlotOptionSheatheCategory.Hide,
        categoryName = "Hide",
      },
    }
  end
end
if rawget(C_TransmogOutfitInfo, "SetPendingTransmogSheatheCategory") == nil then
  function C_TransmogOutfitInfo.SetPendingTransmogSheatheCategory(slotID, optionID, category)
    local pending = rawget(C_TransmogOutfitInfo, "__pendingSheatheCategories") or {}
    pending[string.format("%s:%s", tostring(slotID), tostring(optionID))] = category
    rawset(C_TransmogOutfitInfo, "__pendingSheatheCategories", pending)
  end
end
if rawget(C_TransmogOutfitInfo, "ChangeToOutfit") == nil then
  function C_TransmogOutfitInfo.ChangeToOutfit(outfitID, clear)
    if clear then
      rawset(C_TransmogOutfitInfo, "__activeOutfitID", 0)
      rawset(C_TransmogOutfitInfo, "__currentlyViewedOutfitID", 0)
      rawset(C_TransmogOutfitInfo, "__pendingSheatheCategories", {})
      return
    end
    local id = tonumber(outfitID) or 0
    rawset(C_TransmogOutfitInfo, "__activeOutfitID", id)
    rawset(C_TransmogOutfitInfo, "__currentlyViewedOutfitID", id)
  end
end
if rawget(C_TransmogOutfitInfo, "ClearOutfit") == nil then
  function C_TransmogOutfitInfo.ClearOutfit()
    rawset(C_TransmogOutfitInfo, "__activeOutfitID", 0)
    rawset(C_TransmogOutfitInfo, "__currentlyViewedOutfitID", 0)
    rawset(C_TransmogOutfitInfo, "__pendingSheatheCategories", {})
  end
end
C_Macro = C_Macro or __wow_namespace({
  GetNumMacros = function() return 0, 0 end,
})
C_ActionBar = C_ActionBar or __wow_namespace({
  HasVehicleActionBar = function() return false end,
  HasOverrideActionBar = function() return false end,
  GetOverrideBarSkin = function() return nil end,
  HasBonusActionBar = function() return false end,
  HasTempShapeshiftActionBar = function() return false end,
  HasExtraActionBar = function() return false end,
  IsPossessBarVisible = function() return false end,
  HasAssistedCombatActionButtons = function() return false end,
  IsAssistedCombatAction = function() return false end,
  GetVehicleBarIndex = function() return 1 end,
  GetOverrideBarIndex = function() return 1 end,
  GetTempShapeshiftBarIndex = function() return 1 end,
  GetBonusBarIndex = function() return 1 end,
  GetExtraBarIndex = function() return 1 end,
  GetMultiCastBarIndex = function() return 1 end,
  GetActionBarPage = function() return 1 end,
  SetActionBarPage = __wow_noop,
  HasAction = function(slot)
    if type(_G.HasAction) == "function" then
      return _G.HasAction(slot)
    end
    return false
  end,
  IsPressHoldReleaseSpell = function() return false end,
  GetActionTexture = function(slot)
    if type(_G.GetActionTexture) == "function" then
      return _G.GetActionTexture(slot)
    end
    return nil
  end,
  UsesActionText = function() return false end,
  GetActionText = function() return "" end,
  FindSpellActionButtons = function() return {} end,
  FindPetActionButtons = function() return {} end,
  FindFlyoutActionButtons = function() return {} end,
  GetPetActionPetBarIndices = function() return {} end,
})
if rawget(C_ActionBar, "GetBonusBarOffset") == nil then
  function C_ActionBar.GetBonusBarOffset()
    local index = tonumber(C_ActionBar.GetBonusBarIndex and C_ActionBar.GetBonusBarIndex() or 1) or 1
    return math.max(0, index - 6)
  end
end
if GetBonusBarOffset == nil then
  function GetBonusBarOffset()
    return C_ActionBar.GetBonusBarOffset()
  end
end
if type(C_SpellBook) ~= "table" then
  C_SpellBook = __wow_namespace()
end
if rawget(C_SpellBook, "FindSpellOverrideByID") == nil then
  function C_SpellBook.FindSpellOverrideByID(_spellID)
    return nil
  end
end
if GameTime_GetTime == nil then
  function GameTime_GetTime(_useLocalTime)
    return "12:00"
  end
end
if C_TradeInfo == nil then
  C_TradeInfo = __wow_namespace()
end
if rawget(C_TradeInfo, "ShouldShowTradeOfferWarning") == nil then
  function C_TradeInfo.ShouldShowTradeOfferWarning()
    return false
  end
end
if C_ProfSpecs == nil then
  C_ProfSpecs = __wow_namespace()
end
if rawget(C_ProfSpecs, "ShouldShowSpecTab") == nil then
  function C_ProfSpecs.ShouldShowSpecTab() return true end
end
if rawget(C_ProfSpecs, "GetDefaultSpecSkillLine") == nil then
  function C_ProfSpecs.GetDefaultSpecSkillLine() return 164 end
end
if rawget(C_ProfSpecs, "GetConfigIDForSkillLine") == nil then
  function C_ProfSpecs.GetConfigIDForSkillLine(skillLineID)
    if tonumber(skillLineID) == 164 then
      return 1
    end
    return nil
  end
end
if rawget(C_ProfSpecs, "GetSpecTabIDsForSkillLine") == nil then
  function C_ProfSpecs.GetSpecTabIDsForSkillLine(skillLineID)
    if tonumber(skillLineID) == 164 then
      return { 101 }
    end
    return {}
  end
end
if rawget(C_ProfSpecs, "GetTabInfo") == nil then
  function C_ProfSpecs.GetTabInfo(tabID)
    if tonumber(tabID) == 101 then
      return {
        tabID = 101,
        rootNodeID = 1001,
        name = "Armorsmithing",
      }
    end
    return nil
  end
end

if type(_G.IsPressHoldReleaseSpell) ~= "function" then
  function IsPressHoldReleaseSpell(...)
    if C_Spell and type(C_Spell.IsPressHoldReleaseSpell) == "function" then
      return C_Spell.IsPressHoldReleaseSpell(...)
    end
    return false
  end
end

if type(DropdownButtonMixin) ~= "table" then
  DropdownButtonMixin = {
    Event = {
      OnMenuOpen = "OnMenuOpen",
      OnMenuClose = "OnMenuClose",
      OnUpdate = "OnUpdate",
    },
  }

  function DropdownButtonMixin:OnLoad()
    self.__wow_menu_open = self.__wow_menu_open or false
  end

  function DropdownButtonMixin:OnLoad_Intrinsic()
    self:OnLoad()
  end

  function DropdownButtonMixin:SetupMenu(generator)
    self.__wow_menu_generator = generator
  end

  function DropdownButtonMixin:RegisterMenu(menu_description)
    self.__wow_menu_description = menu_description
  end

  function DropdownButtonMixin:RegisterCallback() end
  function DropdownButtonMixin:UnregisterCallback() end

  function DropdownButtonMixin:IsMenuOpen()
    return self.__wow_menu_open == true
  end

  function DropdownButtonMixin:SetMenuOpen(open)
    self.__wow_menu_open = open and true or false
  end

  function DropdownButtonMixin:OpenMenu()
    self:SetMenuOpen(true)
  end

  function DropdownButtonMixin:CloseMenu()
    self:SetMenuOpen(false)
  end

  function DropdownButtonMixin:OnMenuOpened(menu)
    self:SetMenuOpen(true)
  end

  function DropdownButtonMixin:OnMenuClosed(menu, closeReason)
    self:SetMenuOpen(false)
  end

  function DropdownButtonMixin:OnMenuResponse(menu, description) end
  function DropdownButtonMixin:OnMenuAssigned() end
  function DropdownButtonMixin:OnMenuChanged() end
  function DropdownButtonMixin:SignalUpdate() end
  function DropdownButtonMixin:Update() end
  function DropdownButtonMixin:GenerateMenu() return self.__wow_menu_description end
  function DropdownButtonMixin:GetMenuDescription() return self.__wow_menu_description end
  function DropdownButtonMixin:HasElements() return self.__wow_menu_description ~= nil end
  function DropdownButtonMixin:SetSelectionText(selection_func)
    self.__wow_selection_text_func = selection_func
  end
  function DropdownButtonMixin:GetSelectionText()
    if type(self.__wow_selection_text_func) == "function" then
      return self.__wow_selection_text_func({})
    end
    return nil
  end
  function DropdownButtonMixin:EnableRegenerateOnResponse()
    self.shouldRegenerateOnResponse = true
  end
  function DropdownButtonMixin:CollectSelectionData() return nil, nil, {} end
  function DropdownButtonMixin:GetSelectionData() return nil, nil, {} end
  function DropdownButtonMixin:HasStickyFocus() return false end
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
    elseif addonName == "Blizzard_GlueXML_Mainline"
      or addonName == "Blizzard_GlueXML"
      or addonName == "Blizzard_CharacterCreate" then
      __wow_ensure_glue_character_select_surface()
    elseif addonName == "Blizzard_ProfessionsBook"
      or addonName == "Blizzard_PlayerSpells" then
      __wow_ensure_spellbook_surface()
    elseif addonName == "Blizzard_Dispatcher" then
      __wow_ensure_dispatcher_surface()
    elseif addonName == "Blizzard_ChatFrame"
      or addonName == "Blizzard_QuickJoin"
      or addonName == "Blizzard_Channels"
      or addonName == "Blizzard_VoiceToggleButton" then
      __wow_ensure_chat_voice_button_surface()
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

if C_Container ~= nil and type(C_Container.SetBagPortraitTexture) ~= "function" then
  function C_Container.SetBagPortraitTexture(texture, bagID)
    if texture ~= nil and type(texture.SetTexture) == "function" then
      texture:SetTexture(nil)
    end
  end
end

if C_Item ~= nil and type(C_Item.RequestLoadItemDataByID) ~= "function" then
  function C_Item.RequestLoadItemDataByID(itemID)
    if ItemEventListener and type(ItemEventListener.FireCallbacks) == "function" then
      ItemEventListener:FireCallbacks(itemID)
    end
    return true
  end
end

if C_Spell ~= nil and type(C_Spell.RequestLoadSpellData) ~= "function" then
  function C_Spell.RequestLoadSpellData(spellID)
    if SpellEventListener and type(SpellEventListener.FireCallbacks) == "function" then
      SpellEventListener:FireCallbacks(spellID)
    end
    return true
  end
end

if C_QuestLog ~= nil and type(C_QuestLog.RequestLoadQuestByID) ~= "function" then
  function C_QuestLog.RequestLoadQuestByID(questID)
    if QuestEventListener and type(QuestEventListener.FireCallbacks) == "function" then
      QuestEventListener:FireCallbacks(questID)
    end
    return true
  end
end

AUTOCOMPLETE_LIST = AUTOCOMPLETE_LIST or {}
AUTOCOMPLETE_LIST.ADDFRIEND = AUTOCOMPLETE_LIST.ADDFRIEND or {}
if type(setprinthandler) ~= "function" then
  function setprinthandler() end
end

C_Traits = C_Traits or __wow_namespace({
  GetTreeNodes = function() return {} end,
  GetNodeInfo = function()
    return {
      ranksIncreased = 0,
      entryIDToRanksIncreased = {},
      totalMaxRanks = 0,
    }
  end,
})

C_TradeSkillUI = __wow_merge_namespace(C_TradeSkillUI, {
  GetProfessionSkillLineID = function(professionID)
    return tonumber(professionID) or 0
  end,
  IsGuildTradeSkillsEnabled = function()
    return false
  end,
  GetTradeSkillTexture = function()
    return nil
  end,
  GetTradeSkillDisplayName = function()
    return ""
  end,
  OpenTradeSkill = function()
    local frame = rawget(_G, "ProfessionsFrame")
    if frame ~= nil and type(frame.Show) == "function" then
      frame:Show()
    end
    return frame ~= nil
  end,
  -- GetRecipesTracked / IsRecipeTracked / SetRecipeTracked are now backed
  -- by Rust functions in `globals/rilua_missing_surface/professions.rs`.
  -- The merge here is a no-op because those names are already populated
  -- by `register_profession_surface` before runtime-surface bootstrap runs.
})

C_QuestLog = __wow_merge_namespace(C_QuestLog, {
  ReadyForTurnIn = function()
    return false
  end,
  -- World-quest watch list: empty (no watched quests).
  GetNumWorldQuestWatches = function() return 0 end,
  GetQuestIDForWorldQuestWatchIndex = function() return nil end,
  -- Objective-tracker iteration for active quests: empty.
  GetNumQuestWatches = function() return 0 end,
  GetQuestIDForQuestWatchIndex = function() return nil end,
})

C_ColorOverrides = __wow_merge_namespace(C_ColorOverrides, {
  GetColorForQuality = function()
    return CreateColor(1, 1, 1)
  end,
})

C_PvP = __wow_merge_namespace(C_PvP, {
  IsInBrawl = function() return false end,
  IsSoloShuffle = function() return false end,
  GetArenaCrowdControlInfo = function() return nil, 0, 0 end,
})

C_ScriptedAnimations = __wow_merge_namespace(C_ScriptedAnimations, {
  GetAllScriptedAnimationEffects = function()
    return {}
  end,
})

C_XMLUtil = C_XMLUtil or __wow_namespace({
  GetTemplateInfo = function()
    return nil
  end,
})

C_ToyBoxInfo = __wow_merge_namespace(C_ToyBoxInfo, {
  IsUsingDefaultFilters = function() return true end,
})

C_HeirloomInfo = __wow_merge_namespace(C_HeirloomInfo, {
  IsUsingDefaultFilters = function() return true end,
})

C_TransmogCollection = __wow_merge_namespace(C_TransmogCollection, {
  IsUsingDefaultFilters = function() return true end,
})

C_MountJournal = __wow_merge_namespace(C_MountJournal, {
  IsUsingDefaultFilters = function() return true end,
  ClearRecentFanfares = function() end,
  GetDynamicFlightModeSpellID = function() return 0 end,
  GetMountEquipmentUnlockLevel = function() return 0 end,
  IsDragonridingUnlocked = function() return false end,
})

C_PetJournal = __wow_merge_namespace(C_PetJournal, {
  IsUsingDefaultFilters = function() return true end,
  ClearRecentFanfares = function() end,
  GetSummonBattlePetCooldown = function() return 0, 0, false end,
  PetNeedsFanfare = function() return false end,
})

C_Calendar = __wow_merge_namespace(C_Calendar, {
  GetDefaultGuildFilter = function()
    return {
      minLevel = 1,
      maxLevel = GetMaxLevelForLatestExpansion(),
      rank = 1,
    }
  end,
})

C_MajorFactions = __wow_merge_namespace(C_MajorFactions, {
  GetMajorFactionIDs = function(_expansionLevel)
    return {}
  end,
  IsMajorFactionHiddenFromExpansionPage = function(_factionID)
    return false
  end,
  ShouldDisplayMajorFactionAsJourney = function(_factionID)
    return false
  end,
  GetMajorFactionData = function(factionID)
    return {
      factionID = factionID or 0,
      name = "",
      description = "",
      textureKit = "majorfactions",
      renownLevel = 1,
      renownLevelThreshold = 1,
      renownReputationEarned = 0,
      isUnlocked = false,
    }
  end,
  HasMaximumRenown = function(_factionID)
    return false
  end,
  GetCurrentRenownLevel = function(_factionID)
    return 1
  end,
  GetRenownLevels = function(_factionID)
    return {
      {
        level = 1,
        rewardInfo = {},
      },
    }
  end,
  GetRenownRewardsForLevel = function(_factionID, _level)
    return {}
  end,
  ShouldUseJourneyRewardTrack = function(_factionID)
    return false
  end,
  GetRenownNPCFactionID = function()
    return 0
  end,
})

C_EncounterJournal = __wow_merge_namespace(C_EncounterJournal, {
  OnOpen = function() end,
  InitalizeSelectedTier = function()
    __wow_ej_tier_state.currentTier = GetClientDisplayExpansionLevel ~= nil and GetClientDisplayExpansionLevel() or 10
  end,
})

C_SpecializationInfo = __wow_merge_namespace(C_SpecializationInfo, {
  GetInspectSelectedPvpTalent = function() return nil end,
})

if IsPlayerInWorld == nil then
  function IsPlayerInWorld()
    return true
  end
end

AssistedCombatManager = AssistedCombatManager or {}
if AssistedCombatManager.HasActionSpell == nil then
  function AssistedCombatManager:HasActionSpell()
    return false
  end
end
if AssistedCombatManager.GetActionSpellID == nil then
  function AssistedCombatManager:GetActionSpellID()
    return 0
  end
end
if AssistedCombatManager.GetActionSpellDescription == nil then
  function AssistedCombatManager:GetActionSpellDescription()
    return ""
  end
end
if AssistedCombatManager.SetCanHighlightSpellbookSpells == nil then
  function AssistedCombatManager:SetCanHighlightSpellbookSpells(_enabled)
  end
end
if AssistedCombatManager.ShouldHighlightSpellbookSpell == nil then
  function AssistedCombatManager:ShouldHighlightSpellbookSpell(_spellID)
    return false
  end
end
if AssistedCombatManager.AddSpellTooltipLine == nil then
  function AssistedCombatManager:AddSpellTooltipLine(_tooltip, _spellID, _overriddenSpellID)
  end
end

C_PerksActivities = __wow_merge_namespace(C_PerksActivities, {
  AddTrackedPerksActivity = function(_id) end,
  ClearPerksActivitiesPendingCompletion = function() end,
  GetAllPerksActivityTags = function()
    return { tagName = {} }
  end,
  GetPerksActivitiesInfo = function()
    return {
      activePerksMonth = 0,
      displayMonthName = "",
      secondsRemaining = 0,
      activities = {},
      thresholds = {},
    }
  end,
  GetPerksActivitiesPendingCompletion = function()
    return { pendingIDs = {} }
  end,
  GetPerksActivityChatLink = function(_id)
    return ""
  end,
  GetPerksActivityInfo = function(_id)
    return nil
  end,
  GetPerksUIThemePrefix = function()
    return ""
  end,
  GetTrackedPerksActivities = function()
    return { trackedIDs = {} }
  end,
  RemoveTrackedPerksActivity = function(_id) end,
})

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
      if type(provider) == "table" and provider.pin == nil then
        provider.pin = { dataProvider = provider }
      end
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
    value = __wow_make_color(1, 1, 1, 1)
  elseif __wow_is_color_constant_key(key) then
    value = __wow_make_color(1, 1, 1, 1)
  elseif key == "PLAYER_FACTION_COLOR_HORDE" then
    value = __wow_make_color(1, 0.1, 0.1, 1)
  elseif key == "PLAYER_FACTION_COLOR_ALLIANCE" then
    value = __wow_make_color(0.2, 0.4, 1, 1)
  elseif type(key) == "string" and key:match("^C_[A-Za-z0-9_]+$") then
    value = __wow_namespace()
  elseif type(key) == "string" and key:match("^ERR_") then
    value = key
  end

  if value ~= nil then
    rawset(t, key, value)
    return value
  end
  return nil
end
setmetatable(_G, __global_mt)
