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
    return string.format("FF%02X%02X%02X", math.floor(self.r * 255), math.floor(self.g * 255), math.floor(self.b * 255))
  end

  function color:GenerateHexColorMarkup()
    return "|c" .. self:GenerateHexColor()
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

if GetMoneyString == nil then
  -- Mirrors Blizzard's FormattingUtil.lua plain-text path with the
  -- 2-arg signature the simulator surfaces. The icon-texture escapes
  -- and colorblind CVar branches are not modelled, so the output is
  -- the chat-style "123g 45s 67c" form. Zero-copper segments are
  -- elided unless the entire amount is zero (canonicalises 0 → "0c").
  local function __wow_separate_thousands(n)
    local digits = tostring(n)
    if #digits <= 3 then
      return digits
    end
    local out = digits:sub(-3)
    local i = #digits - 3
    while i > 0 do
      local chunk_start = math.max(1, i - 2)
      out = digits:sub(chunk_start, i) .. "," .. out
      i = chunk_start - 1
    end
    return out
  end
  function GetMoneyString(money, separateThousands)
    money = math.floor(tonumber(money) or 0)
    if money < 0 then money = 0 end
    local gold = math.floor(money / 10000)
    local silver = math.floor((money - gold * 10000) / 100)
    local copper = money % 100
    local gold_text = separateThousands and __wow_separate_thousands(gold) or tostring(gold)
    local parts = {}
    if gold > 0 then parts[#parts + 1] = gold_text .. "g" end
    if silver > 0 then parts[#parts + 1] = silver .. "s" end
    if copper > 0 or #parts == 0 then parts[#parts + 1] = copper .. "c" end
    return table.concat(parts, " ")
  end
end

if GetColorForCurrencyReward == nil then
  -- Mirrors Blizzard's UIParent.lua. The currency-overflow probe is not
  -- modelled, so the overflow branch is omitted; callers always pass
  -- through the defaultColor / HIGHLIGHT_FONT_COLOR fallback.
  function GetColorForCurrencyReward(_currencyID, _rewardQuantity, defaultColor)
    if defaultColor ~= nil then
      return defaultColor
    end
    if HIGHLIGHT_FONT_COLOR ~= nil then
      return HIGHLIGHT_FONT_COLOR
    end
    return __wow_make_color(1, 1, 1, 1)
  end
end

local __wow_console_font_height = 14

if ConsoleGetColorFromType == nil then
  function ConsoleGetColorFromType(_colorType)
    return CreateColor(1, 1, 1)
  end
end

if ConsoleGetFontHeight == nil then
  function ConsoleGetFontHeight()
    return __wow_console_font_height
  end
end

if ConsoleSetFontHeight == nil then
  function ConsoleSetFontHeight(fontHeightInPixels)
    __wow_console_font_height = tonumber(fontHeightInPixels) or __wow_console_font_height
  end
end

local function __wow_noop()
end

if AbbreviateLargeNumbers == nil then
  function AbbreviateLargeNumbers(value)
    return tostring(math.floor(tonumber(value) or 0))
  end
end

if HasArtifactEquipped == nil then
  function HasArtifactEquipped()
    return false
  end
end

if IsPVPTimerRunning == nil then
  function IsPVPTimerRunning()
    return false
  end
end

if GetAlternativeDefaultLanguage == nil then
  function GetAlternativeDefaultLanguage()
    return nil
  end
end

if GetChannelList == nil then
  function GetChannelList()
    return nil
  end
end

CombatLogInbound = CombatLogInbound or {
  GenerateMessage = function()
    return "", 1, 1, 1
  end,
}

C_GossipInfo = C_GossipInfo or __wow_namespace()
if rawget(C_GossipInfo, "GetFriendshipReputation") == nil then
  function C_GossipInfo.GetFriendshipReputation(_factionID)
    return {
      friendshipFactionID = 0,
      reaction = 0,
      currentReactionThreshold = 0,
      nextReactionThreshold = 0,
      currentStanding = 0,
    }
  end
end
if rawget(C_GossipInfo, "GetFriendshipReputationRanks") == nil then
  function C_GossipInfo.GetFriendshipReputationRanks(_factionID)
    return {
      currentLevel = 0,
      maxLevel = 0,
    }
  end
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

if CalculateStringEditDistance == nil then
  function CalculateStringEditDistance(firstString, secondString)
    if type(firstString) ~= "string" or type(secondString) ~= "string" then
      return 0
    end
    local firstLen = #firstString
    local secondLen = #secondString
    if firstLen == 0 then return secondLen end
    if secondLen == 0 then return firstLen end

    local previousRow = {}
    for column = 0, secondLen do
      previousRow[column] = column
    end

    local currentRow = {}
    for row = 1, firstLen do
      currentRow[0] = row
      local firstChar = firstString:byte(row)
      for column = 1, secondLen do
        local substitutionCost = (firstChar == secondString:byte(column)) and 0 or 1
        local deletion = previousRow[column] + 1
        local insertion = currentRow[column - 1] + 1
        local substitution = previousRow[column - 1] + substitutionCost
        currentRow[column] = math.min(deletion, insertion, substitution)
      end
      for column = 0, secondLen do
        previousRow[column] = currentRow[column]
      end
    end

    return previousRow[secondLen]
  end
end

do
  local stringMeta = getmetatable("")
  if type(stringMeta) == "table" then
    local stringIndex = stringMeta.__index
    if type(stringIndex) == "table" then
      function stringIndex:split(delimiter, limit)
        return strsplittable(delimiter, self, limit)
      end
    end

    function stringMeta:split(delimiter, limit)
      return strsplittable(delimiter, self, limit)
    end
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

if UpdateAddOnMemoryUsage == nil then
  function UpdateAddOnMemoryUsage()
    return 0
  end
end

if UpdateAddOnCPUUsage == nil then
  function UpdateAddOnCPUUsage()
    return 0
  end
end

if ResetCPUUsage == nil then
  function ResetCPUUsage()
    return 0
  end
end

if GetAddOnMemoryUsage == nil then
  function GetAddOnMemoryUsage(_name)
    return 0
  end
end

if GetAddOnCPUUsage == nil then
  function GetAddOnCPUUsage(_name)
    return 0
  end
end

if GetFrameCPUUsage == nil then
  function GetFrameCPUUsage(_frame, _includeChildren)
    return 0, 0
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

if GetInventoryItemsForSlot == nil then
  function GetInventoryItemsForSlot(_slot, _itemTable)
    -- Equipment flyout callers expect the table argument to be populated.
    -- The simulator does not model alternate swap candidates, so leave it empty.
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

ChatTypeInfo = ChatTypeInfo or {}
ChatTypeInfo.SYSTEM = ChatTypeInfo.SYSTEM or {
  r = 1,
  g = 1,
  b = 0,
  id = 1,
}
ChatTypeInfo.BN_WHISPER = ChatTypeInfo.BN_WHISPER or {
  r = 0,
  g = 1,
  b = 0.96470594406128,
  id = 19,
}

local __wow_chat_window_state = __wow_chat_window_state or {}

local function __wow_is_chat_window_shown_by_default(id)
  return id == 1
end

local function __wow_is_chat_window_docked_by_default(id)
  return id == 1
end

if GetChatWindowInfo == nil then
  function GetChatWindowInfo(id)
    -- Default chat frame color: black with 25% alpha (DEFAULT_CHATFRAME_COLOR / DEFAULT_CHATFRAME_ALPHA)
    -- Only ChatFrame1 (General) is enabled by default.
    -- ChatFrame2-10 exist in XML but stay hidden until user enables them.
    local realId = id or 1
    local chat = __wow_chat_window_state[realId]
    local shown = __wow_is_chat_window_shown_by_default(realId)
    local docked = __wow_is_chat_window_docked_by_default(realId)
    if chat and chat.shown ~= nil then
      shown = chat.shown == true
    end
    if chat and chat.docked ~= nil then
      docked = chat.docked == true
    end
    return "Chat " .. tostring(realId), 12, 0, 0, 0, 0.25, shown, false, docked, false
  end
end

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

local function __wow_ensure_chat_voice_button_surface()
  local uiParent = rawget(_G, "UIParent")
  QuickJoinToastButton = QuickJoinToastButton or __wow_install_frame_helpers(__wow_ensure_named_frame("Button", "QuickJoinToastButton", uiParent))

  local channelButton = rawget(_G, "ChatFrameChannelButton")
  if channelButton == nil then
    return
  end

  local icon = rawget(channelButton, "Icon")
  if icon ~= nil then
    if type(icon.SetParentKey) == "function" then
      pcall(icon.SetParentKey, icon, "Icon", true)
    end
    if type(icon.GetWidth) == "function" and type(icon.GetHeight) == "function"
        and (icon:GetWidth() == 0 or icon:GetHeight() == 0)
        and type(icon.SetSize) == "function" then
      icon:SetSize(channelButton.fixedIconWidth or 15, channelButton.fixedIconHeight or 15)
    end
    if type(icon.GetNumPoints) == "function" and icon:GetNumPoints() == 0
        and type(icon.SetPoint) == "function" then
      icon:SetPoint("CENTER", channelButton, "CENTER", 0, 0)
    end
    if type(icon.SetAtlas) == "function" then
      icon:SetAtlas("chatframe-button-icon-voicechat")
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
      if predicate(value, i, t) then
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
  function debugstack(level, count1, count2)
    if not debug or not debug.traceback then
      return ""
    end
    local start = (tonumber(level) or 1) + 1
    local tb = debug.traceback("", start) or ""
    tb = tb:gsub("^\n?stack traceback:\n?", "")
    tb = tb:gsub("^%s+", "")
    if count1 or count2 then
      local top = tonumber(count1) or 12
      local bottom = tonumber(count2) or 10
      local lines = {}
      for line in tb:gmatch("([^\n]*)\n?") do
        if line ~= "" then lines[#lines + 1] = line end
      end
      if #lines > top + bottom then
        local kept = {}
        for i = 1, top do kept[#kept + 1] = lines[i] end
        kept[#kept + 1] = "..."
        for i = #lines - bottom + 1, #lines do kept[#kept + 1] = lines[i] end
        return table.concat(kept, "\n") .. "\n"
      end
    end
    if tb ~= "" and not tb:match("\n$") then tb = tb .. "\n" end
    return tb
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

local __wow_namespace_names = setmetatable({}, { __mode = "k" })
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
  SendChatMessage = function(...)
    local fn = rawget(_G, "__wow_send_chat_message")
    if type(fn) == "function" then
      return fn(...)
    end
  end,
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

-- C_LFGList is state-backed via `src/lua_api/globals/lfg_list.rs`.

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
UiMapPoint = __wow_merge_namespace(UiMapPoint, {})
C_MapExplorationInfo = __wow_merge_namespace(C_MapExplorationInfo, {})

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
C_EncounterTimeline = __wow_merge_namespace(C_EncounterTimeline, {
  IsFeatureAvailable = function() return true end,
  IsFeatureEnabled = function() return true end,
  GetEventList = function() return { 1 } end,
  GetEventInfo = function(eventID)
    if eventID ~= 1 then
      return nil
    end
    return {
      spellID = 19750,
      spellName = "Flash of Light",
    }
  end,
  GetEventTimer = function(eventID)
    if eventID ~= 1 then
      return nil
    end
    local timer = { remaining = 12.5 }
    function timer:GetRemainingDuration()
      return self.remaining
    end
    return timer
  end,
  GetEventTrack = function(eventID)
    if eventID ~= 1 then
      return nil, nil
    end
    return Enum.EncounterTimelineTrack.Short, 1
  end,
  HasActiveEvents = function() return true end,
  HasVisibleEvents = function() return true end,
})

local __wow_encounter_events_state = {
  events = {
    [1] = {
      encounterEventID = 1,
      name = "Default Encounter Event",
      color = nil,
      sounds = {},
    },
  },
  nextSoundHandle = 1,
}
C_EncounterEvents = __wow_merge_namespace(C_EncounterEvents, {
  _state = __wow_encounter_events_state,
})
if rawget(C_EncounterEvents, "GetEventList") == nil then
  function C_EncounterEvents.GetEventList()
    return { 1 }
  end
end
if rawget(C_EncounterEvents, "HasEventInfo") == nil then
  function C_EncounterEvents.HasEventInfo(eventID)
    eventID = tonumber(eventID)
    return eventID ~= nil and C_EncounterEvents._state.events[eventID] ~= nil
  end
end
if rawget(C_EncounterEvents, "GetEventInfo") == nil then
  function C_EncounterEvents.GetEventInfo(eventID)
    eventID = tonumber(eventID)
    local event = eventID and C_EncounterEvents._state.events[eventID] or nil
    if not event then
      return nil
    end
    local info = {
      encounterEventID = event.encounterEventID,
      name = event.name,
    }
    if event.color ~= nil then
      info.color = {
        r = event.color.r,
        g = event.color.g,
        b = event.color.b,
      }
    end
    return info
  end
end
if rawget(C_EncounterEvents, "SetEventColor") == nil then
  function C_EncounterEvents.SetEventColor(eventID, color)
    eventID = tonumber(eventID)
    local event = eventID and C_EncounterEvents._state.events[eventID] or nil
    if not event then
      return
    end
    if color == nil then
      event.color = nil
      return
    end
    event.color = {
      r = tonumber(color.r) or 0,
      g = tonumber(color.g) or 0,
      b = tonumber(color.b) or 0,
    }
  end
end
if rawget(C_EncounterEvents, "GetEventColor") == nil then
  function C_EncounterEvents.GetEventColor(eventID)
    eventID = tonumber(eventID)
    local event = eventID and C_EncounterEvents._state.events[eventID] or nil
    if not event or event.color == nil then
      return nil
    end
    return {
      r = event.color.r,
      g = event.color.g,
      b = event.color.b,
    }
  end
end
if rawget(C_EncounterEvents, "SetEventSound") == nil then
  function C_EncounterEvents.SetEventSound(eventID, triggerID, sound)
    eventID = tonumber(eventID)
    triggerID = tonumber(triggerID)
    local event = eventID and C_EncounterEvents._state.events[eventID] or nil
    if not event or triggerID == nil then
      return
    end
    if sound == nil then
      event.sounds[triggerID] = nil
      return
    end
    event.sounds[triggerID] = {
      file = tonumber(sound.file) or 0,
      channel = tostring(sound.channel or ""),
      volume = tonumber(sound.volume) or 0,
    }
  end
end
if rawget(C_EncounterEvents, "GetEventSound") == nil then
  function C_EncounterEvents.GetEventSound(eventID, triggerID)
    eventID = tonumber(eventID)
    triggerID = tonumber(triggerID)
    local event = eventID and C_EncounterEvents._state.events[eventID] or nil
    if not event or triggerID == nil then
      return nil
    end
    local sound = event.sounds[triggerID]
    if sound == nil then
      return nil
    end
    return {
      file = sound.file,
      channel = sound.channel,
      volume = sound.volume,
    }
  end
end
if rawget(C_EncounterEvents, "PlayEventSound") == nil then
  function C_EncounterEvents.PlayEventSound(eventID, triggerID)
    local sound = C_EncounterEvents.GetEventSound(eventID, triggerID)
    if sound == nil then
      return nil
    end
    local handle = C_EncounterEvents._state.nextSoundHandle
    C_EncounterEvents._state.nextSoundHandle = handle + 1
    return handle
  end
end

C_AccountStore = __wow_merge_namespace(C_AccountStore, {
  GetCategories = function() return {} end,
  GetCategoryInfo = function() return nil end,
  GetItemInfo = function() return nil end,
  GetCurrencyAvailable = function() return 0 end,
})

local __wow_seeded_damage_meter_source = {
  name = "Player",
  isLocalPlayer = true,
  sourceGUID = "Player-1-00000001",
  sourceCreatureID = 1,
  totalAmount = 52000,
  amountPerSecond = 1300,
  combatSpells = {
    { spellID = 19750, totalAmount = 52000, amountPerSecond = 1300 },
  },
}
local __wow_seeded_damage_meter_session = {
  sessionID = 1,
  totalAmount = 52000,
  maxAmount = 52000,
  durationSeconds = 40,
  combatSources = {
    __wow_seeded_damage_meter_source,
    {
      name = "Companion",
      isLocalPlayer = false,
      sourceGUID = "Creature-1-00000002",
      sourceCreatureID = 2,
      totalAmount = 3333,
      amountPerSecond = 83.325,
      combatSpells = {
        { spellID = 1337, totalAmount = 3333, amountPerSecond = 83.325 },
      },
    },
  },
}
C_DamageMeter = __wow_merge_namespace(C_DamageMeter, {
  IsDamageMeterAvailable = function() return true, nil end,
  GetAvailableCombatSessions = function() return { { sessionID = 1 } } end,
  GetCurrentCombatSessionID = function() return 1 end,
  GetDamageMeterEntries = function() return {} end,
  GetCombatSessionFromType = function(sessionType, damageType)
    if sessionType == Enum.DamageMeterSessionType.Overall and damageType == Enum.DamageMeterType.DamageDone then
      return __wow_seeded_damage_meter_session
    end
    return nil
  end,
  GetCombatSessionSourceFromType = function(sessionType, damageType, sourceGUID, sourceCreatureID)
    if sessionType ~= Enum.DamageMeterSessionType.Overall then
      return nil
    end
    if damageType ~= Enum.DamageMeterType.DamageDone then
      return nil
    end
    if sourceGUID ~= __wow_seeded_damage_meter_source.sourceGUID then
      return nil
    end
    if sourceCreatureID ~= __wow_seeded_damage_meter_source.sourceCreatureID then
      return nil
    end
    return __wow_seeded_damage_meter_source
  end,
  GetCombatSessionFromID = function(sessionID, damageType)
    if sessionID ~= __wow_seeded_damage_meter_session.sessionID then
      return nil
    end
    if damageType ~= Enum.DamageMeterType.DamageDone then
      return nil
    end
    return __wow_seeded_damage_meter_session
  end,
  GetCombatSessionSourceFromID = function(sessionID, damageType, sourceGUID, sourceCreatureID)
    if sessionID ~= __wow_seeded_damage_meter_session.sessionID then
      return nil
    end
    if damageType ~= Enum.DamageMeterType.DamageDone then
      return nil
    end
    if sourceGUID ~= __wow_seeded_damage_meter_source.sourceGUID then
      return nil
    end
    if sourceCreatureID ~= __wow_seeded_damage_meter_source.sourceCreatureID then
      return nil
    end
    return __wow_seeded_damage_meter_source
  end,
  GetSessionDurationSeconds = function(sessionType, sessionID)
    if sessionType == Enum.DamageMeterSessionType.Overall or sessionID == __wow_seeded_damage_meter_session.sessionID then
      return __wow_seeded_damage_meter_session.durationSeconds
    end
    return 0
  end,
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
  GetViewRadius = function() return 200 end,
})

C_Navigation = __wow_merge_namespace(C_Navigation, {
  WasClampedToScreen = function() return false end,
  GetTargetState = function() return 0 end,
  HasValidScreenPosition = function() return false end,
  GetDistance = function() return 0 end,
  GetNearestPartyMemberToken = function() return nil end,
  GetFrame = function() return nil end,
})

C_DateAndTime = __wow_merge_namespace(C_DateAndTime, {
  GetCurrentCalendarTime = function()
    return __wow_make_calendar_time(0, 0)
  end,
  GetServerTimeLocal = function()
    return 0
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
    local seconds = tonumber(epoch) or 0
    if seconds > 1000000000000 then
      seconds = seconds / 1000000
    end
    local totalMinutes = math.floor(seconds / 60)
    local dayOffset = math.floor(totalMinutes / 1440) % 30
    local minuteOffset = totalMinutes % 1440
    return __wow_make_calendar_time(dayOffset, minuteOffset)
  end,
  GetWeeklyResetStartTime = function()
    return 0
  end,
  GetSecondsUntilDailyReset = function()
    return 86400
  end,
  GetSecondsUntilWeeklyReset = function()
    return 604800
  end,
})

C_TaxiMap = __wow_merge_namespace(C_TaxiMap, {
  GetAllTaxiNodes = function()
    return {}
  end,
  GetTaxiNodesForMap = function()
    return {}
  end,
  ShouldMapShowTaxiNodes = function()
    return true
  end,
})

local __wow_housing_entry_type = Enum.HousingCatalogEntryType and Enum.HousingCatalogEntryType.Decor or 0
local __wow_housing_all_category_id = Constants.HousingCatalogConsts.HOUSING_CATALOG_ALL_CATEGORY_ID

local __wow_housing_seeded_entries = {
  [1001] = {
    recordID = 1001,
    entryType = __wow_housing_entry_type,
    itemID = 1001,
    name = "Sunspire Chair",
    asset = nil,
    iconTexture = nil,
    iconAtlas = nil,
    uiModelSceneID = nil,
    categoryIDs = { __wow_housing_all_category_id, 101 },
    subcategoryIDs = { 1001 },
    dataTagsByID = {},
    size = Enum.HousingCatalogEntrySize and Enum.HousingCatalogEntrySize.Small or 2,
    placementCost = 1,
    totalNumStored = 1,
    remainingRedeemable = 0,
    totalNumPlaced = 1,
    destroyableInstanceCount = 1,
    isUniqueTrophy = false,
    isAllowedOutdoors = true,
    isAllowedIndoors = true,
    canCustomize = true,
    isPrefab = false,
    quality = 2,
    firstAcquisitionBonus = 0,
    sourceText = "",
  },
  [1002] = {
    recordID = 1002,
    entryType = __wow_housing_entry_type,
    itemID = 1002,
    name = "Azure Reading Lamp",
    asset = nil,
    iconTexture = nil,
    iconAtlas = nil,
    uiModelSceneID = nil,
    categoryIDs = { __wow_housing_all_category_id, 101 },
    subcategoryIDs = { 1002 },
    dataTagsByID = {},
    size = Enum.HousingCatalogEntrySize and Enum.HousingCatalogEntrySize.Small or 2,
    placementCost = 1,
    totalNumStored = 1,
    remainingRedeemable = 0,
    totalNumPlaced = 1,
    destroyableInstanceCount = 1,
    isUniqueTrophy = false,
    isAllowedOutdoors = true,
    isAllowedIndoors = true,
    canCustomize = true,
    isPrefab = false,
    quality = 2,
    firstAcquisitionBonus = 0,
    sourceText = "",
  },
}

local __wow_housing_seeded_variants = {
  [1001] = {
    [1] = { variantID = 1, productID = 91001, name = "Sunspire Chair", numStored = 1 },
    [2] = { variantID = 2, productID = 91002, name = "Azure Upholstery", numStored = 0 },
  },
  [1002] = {
    [1] = { variantID = 1, productID = 91003, name = "Azure Reading Lamp", numStored = 1 },
  },
}

local __wow_housing_seeded_featured_small_products = {
  {
    entryID = 1001,
    entryVariantID = { recordID = 1001, entryType = __wow_housing_entry_type, variantIdentifier = 1 },
    productID = 91001,
    price = 100,
    originalPrice = nil,
    canPreview = true,
  },
  {
    entryID = 1002,
    entryVariantID = { recordID = 1002, entryType = __wow_housing_entry_type, variantIdentifier = 1 },
    productID = 91003,
    price = 125,
    originalPrice = nil,
    canPreview = true,
  },
}

local __wow_housing_seeded_bundle_state = {
  [5001] = {
    productID = 5001,
    price = 500,
    originalPrice = nil,
    entryIDs = { 1001, 1002 },
    decorEntries = {
      { decorID = 1001, quantity = 1 },
      { decorID = 1002, quantity = 1 },
    },
    nonDecorProducts = {},
    canPreview = true,
    wasViewed = false,
  },
}

local __wow_housing_seeded_market_state = {
  [1001] = { price = 100, productID = 91001, bundleIDs = { 5001 }, isInCart = false, cartCount = 0, wasViewedInStore = false },
  [1002] = { price = 125, productID = 91003, bundleIDs = { 5001 }, isInCart = false, cartCount = 0, wasViewedInStore = false },
}

local CatalogShopConstants = rawget(_G, "CatalogShopConstants") or {
  ProductType = {
    Bundle = 1,
    Decor = 2,
  },
  CategoryLinks = {
    Featured = "featured",
    Housing = "housing",
  },
}
_G.CatalogShopConstants = CatalogShopConstants

local __wow_housing_seeded_product_infos = {
  [2003] = {
    catalogShopProductID = 2003,
    name = "Apprentice Rider Bundle",
    type = "Bundle",
    description = "A seeded store product used for simulator storefront coverage.",
    iconTexture = "Interface\\Icons\\Ability_Mount_RidingHorse",
    isFullyOwned = false,
    isPurchasePending = false,
    refundable = false,
    price = "10",
    originalPrice = "10",
    discountPercentage = 0,
    itemID = 0,
    mountID = 0,
    mountTypeName = "",
    speciesID = 0,
    transmogSetID = 0,
    itemModifiedAppearanceID = 0,
    subItems = {},
    subItemsLoaded = true,
    backgroundTexture = "shop-bg-map-blue",
    foregroundTexture = nil,
    smallCardBGTexture = nil,
    smallCardFGTexture = nil,
    wideCardBGTexture = nil,
    wideCardFGTexture = nil,
    previewIconTexture = nil,
    optionalWideCardBackgroundTexture = nil,
    isBundle = true,
    bundleChildrenSize = 2,
    licenseTermType = 0,
    licenseTermDuration = 0,
    virtualCurrencies = {},
    isHidden = false,
    hasPendingOrders = false,
    numBundleDetailCards = 2,
    isDynamicallyDiscounted = false,
    shouldShowOriginalPrice = false,
    wideCardBGOverrideProductURL = nil,
    previewBGOverrideProductURL = nil,
    previewSmallBGOverrideProductURL = nil,
    decorQuantity = nil,
    isVCProduct = false,
    containsHousingItem = true,
    creatureDisplayInfoIDs = {},
    spellVisualIDs = {},
    itemModifiedAppearanceIDs = {},
    mainHandItemModifiedAppearanceID = nil,
    offHandItemModifiedAppearanceID = nil,
    decorFileDataID = nil,
    houseTextureAtlas = nil,
    productType = 1,
    productIDList = { 20031, 20032 },
  },
  [20031] = {
    catalogShopProductID = 20031,
    name = "Apprentice Rider Saddle",
    type = "Decor",
    description = "Bundle child decor.",
    iconTexture = "Interface\\Icons\\INV_Misc_QuestionMark",
    isFullyOwned = false,
    isPurchasePending = false,
    refundable = false,
    price = "5",
    originalPrice = "5",
    discountPercentage = 0,
    itemID = 0,
    mountID = 0,
    mountTypeName = "",
    speciesID = 0,
    transmogSetID = 0,
    itemModifiedAppearanceID = 0,
    subItems = {},
    subItemsLoaded = true,
    backgroundTexture = "shop-bg-map-blue",
    foregroundTexture = nil,
    smallCardBGTexture = nil,
    smallCardFGTexture = nil,
    wideCardBGTexture = nil,
    wideCardFGTexture = nil,
    previewIconTexture = nil,
    optionalWideCardBackgroundTexture = nil,
    isBundle = false,
    bundleChildrenSize = 0,
    licenseTermType = 0,
    licenseTermDuration = 0,
    virtualCurrencies = {},
    isHidden = false,
    hasPendingOrders = false,
    numBundleDetailCards = 0,
    isDynamicallyDiscounted = false,
    shouldShowOriginalPrice = false,
    wideCardBGOverrideProductURL = nil,
    previewBGOverrideProductURL = nil,
    previewSmallBGOverrideProductURL = nil,
    decorQuantity = nil,
    isVCProduct = false,
    containsHousingItem = true,
    creatureDisplayInfoIDs = {},
    spellVisualIDs = {},
    itemModifiedAppearanceIDs = {},
    mainHandItemModifiedAppearanceID = nil,
    offHandItemModifiedAppearanceID = nil,
    decorFileDataID = 91001,
    houseTextureAtlas = nil,
    productType = 2,
  },
  [20032] = {
    catalogShopProductID = 20032,
    name = "Apprentice Rider Bridle",
    type = "Decor",
    description = "Bundle child decor.",
    iconTexture = "Interface\\Icons\\INV_Misc_QuestionMark",
    isFullyOwned = false,
    isPurchasePending = false,
    refundable = false,
    price = "5",
    originalPrice = "5",
    discountPercentage = 0,
    itemID = 0,
    mountID = 0,
    mountTypeName = "",
    speciesID = 0,
    transmogSetID = 0,
    itemModifiedAppearanceID = 0,
    subItems = {},
    subItemsLoaded = true,
    backgroundTexture = "shop-bg-map-blue",
    foregroundTexture = nil,
    smallCardBGTexture = nil,
    smallCardFGTexture = nil,
    wideCardBGTexture = nil,
    wideCardFGTexture = nil,
    previewIconTexture = nil,
    optionalWideCardBackgroundTexture = nil,
    isBundle = false,
    bundleChildrenSize = 0,
    licenseTermType = 0,
    licenseTermDuration = 0,
    virtualCurrencies = {},
    isHidden = false,
    hasPendingOrders = false,
    numBundleDetailCards = 0,
    isDynamicallyDiscounted = false,
    shouldShowOriginalPrice = false,
    wideCardBGOverrideProductURL = nil,
    previewBGOverrideProductURL = nil,
    previewSmallBGOverrideProductURL = nil,
    decorQuantity = nil,
    isVCProduct = false,
    containsHousingItem = true,
    creatureDisplayInfoIDs = {},
    spellVisualIDs = {},
    itemModifiedAppearanceIDs = {},
    mainHandItemModifiedAppearanceID = nil,
    offHandItemModifiedAppearanceID = nil,
    decorFileDataID = 91002,
    houseTextureAtlas = nil,
    productType = 2,
  },
}

local __wow_housing_preview_cart_state = {}

local __wow_housing_theme_set_names = {
  [1] = "Sunspire",
}

local function __wow_catalog_shop_emit_seeded_refresh(session_id)
  local frame = rawget(_G, "CatalogShopFrame")
  if type(frame) ~= "table" then
    return
  end

  frame.shoppingSessionUUIDStr = session_id

  local on_event = frame.OnEvent_CatalogShop
  if type(on_event) ~= "function" and type(frame.GetScript) == "function" then
    on_event = frame:GetScript("OnEvent")
  end

  if type(on_event) ~= "function" then
    return
  end

  on_event(frame, "CATALOG_SHOP_DATA_REFRESH", session_id)
  on_event(frame, "CATALOG_SHOP_FETCH_SUCCESS", session_id)

  local category_ids = C_CatalogShop and C_CatalogShop.GetAvailableCategoryIDs and C_CatalogShop.GetAvailableCategoryIDs() or nil
  local initial_category_id = type(category_ids) == "table" and category_ids[1] or nil
  if initial_category_id ~= nil and type(frame.OnCategorySelected) == "function" then
    frame:OnCategorySelected(initial_category_id)
  end
end

local __wow_housing_customize_mode_selected_decor = {
  decorGUID = "Decor-Selection-1001",
  decorID = 1001,
  name = "Sunspire Chair",
  isLocked = false,
  canBeCustomized = true,
  canBeRemoved = true,
  isAllowedOutdoors = true,
  isAllowedIndoors = true,
  isRefundable = false,
  dyeSlots = {},
  dataTagsByID = {},
  size = Enum.HousingCatalogEntrySize and Enum.HousingCatalogEntrySize.Small or 2,
}

local __wow_housing_decor_name_by_id = {
  [1001] = "Sunspire Chair",
  [1002] = "Azure Upholstery",
  [2001] = "Azure Reading Lamp",
  [91001] = "Sunspire Chair",
  [91002] = "Azure Upholstery",
  [91003] = "Azure Reading Lamp",
}

local __wow_housing_decor_icon_by_id = {
  [1001] = 0,
  [1002] = 0,
  [2001] = 0,
  [91001] = 0,
  [91002] = 0,
  [91003] = 0,
}

local __wow_housing_decor_info_by_guid = {
  ["Decor-Selection-1001"] = {
    decorGUID = "Decor-Selection-1001",
    decorID = 1001,
    name = "Sunspire Chair",
    isLocked = false,
    canBeCustomized = true,
    canBeRemoved = true,
    isAllowedOutdoors = true,
    isAllowedIndoors = true,
    isRefundable = false,
    dyeSlots = {},
    dataTagsByID = {},
    size = Enum.HousingCatalogEntrySize and Enum.HousingCatalogEntrySize.Small or 2,
  },
  ["Decor-Selection-2001"] = {
    decorGUID = "Decor-Selection-2001",
    decorID = 2001,
    name = "Azure Reading Lamp",
    isLocked = false,
    canBeCustomized = true,
    canBeRemoved = true,
    isAllowedOutdoors = true,
    isAllowedIndoors = true,
    isRefundable = false,
    dyeSlots = {},
    dataTagsByID = {},
    size = Enum.HousingCatalogEntrySize and Enum.HousingCatalogEntrySize.Small or 2,
  },
}

local __wow_housing_decor_state = {
  preview = false,
  gridVisible = false,
  selectedDecorGUID = "Decor-Selection-2001",
  hoveredDecorGUID = nil,
  placedDecor = {
    __wow_housing_decor_info_by_guid["Decor-Selection-1001"],
    __wow_housing_decor_info_by_guid["Decor-Selection-2001"],
  },
}

local __wow_housing_neighborhood_state = {
  houseInfo = {
    plotID = 27,
    houseName = "Sunspire Retreat",
    ownerName = "Simhero",
    plotCost = 0,
    neighborhoodName = "Dawnmeadow",
    moveOutTime = nil,
    plotReserved = false,
    neighborhoodGUID = "Neighborhood-Dawnmeadow",
    houseGUID = "House-Sunspire-Retreat",
  },
  neighborhoodInfo = {
    neighborhoodType = "Public",
    neighborhoodOwnerType = Enum.NeighborhoodOwnerType and Enum.NeighborhoodOwnerType.None or 0,
    neighborhoodName = "Dawnmeadow",
    neighborhoodGUID = "Neighborhood-Dawnmeadow",
    ownerGUID = "Player-Simhero",
    suggestionReason = nil,
    ownerName = "Simhero",
    locationName = "Dawnmeadow",
  },
}

local __wow_housing_exterior_state = {
  decorHidden = false,
  selectedExteriorType = 1,
  selectedExteriorTypeName = "Sunspire Cottage",
  selectedSize = Enum.HousingFixtureSize and Enum.HousingFixtureSize.Medium or 3,
  selectedFixturePoint = {
    fixtureID = 1,
    pointID = 1,
    name = "Front Door",
  },
  baseFixtureInfo = {
    selectedStyleFixtureID = 101,
    selectedVariantFixtureID = 201,
    styleOptions = {
      { fixtureID = 101, name = "Sunspire Base", colorID = 1 },
      { fixtureID = 102, name = "Moonspire Base", colorID = 2 },
    },
    currentStyleVariantOptions = {
      { fixtureID = 201, name = "Sunspire Base", colorID = 1 },
      { fixtureID = 202, name = "Moonspire Base", colorID = 2 },
    },
  },
  roofFixtureInfo = {
    selectedStyleFixtureID = 301,
    selectedVariantFixtureID = 401,
    styleOptions = {
      { fixtureID = 301, name = "Sunspire Roof", colorID = 1 },
      { fixtureID = 302, name = "Moonspire Roof", colorID = 2 },
    },
    currentStyleVariantOptions = {
      { fixtureID = 401, name = "Sunspire Roof", colorID = 1 },
      { fixtureID = 402, name = "Moonspire Roof", colorID = 2 },
    },
  },
}

local function __wow_housing_clone_table(source)
  local copy = {}
  for key, value in pairs(source or {}) do
    copy[key] = value
  end
  return copy
end

local function __wow_housing_copy_core_fixture_info(info)
  if not info then
    return nil
  end
  local copy = __wow_housing_clone_table(info)
  copy.styleOptions = __wow_housing_clone_table(info.styleOptions)
  copy.currentStyleVariantOptions = __wow_housing_clone_table(info.currentStyleVariantOptions)
  return copy
end

local function __wow_housing_variant_id(entry_id, variant_id)
  return {
    recordID = entry_id,
    entryType = __wow_housing_entry_type,
    variantIdentifier = variant_id,
  }
end

local function __wow_housing_list_contains(list, needle)
  for _, value in ipairs(list or {}) do
    if value == needle then
      return true
    end
  end
  return false
end

local function __wow_housing_copy_variant_info(entry_id, variant_id)
  local variant_info = __wow_housing_seeded_variants[entry_id] and __wow_housing_seeded_variants[entry_id][variant_id]
  if not variant_info then
    return nil
  end
  local info = __wow_housing_clone_table(variant_info)
  info.entryVariantID = __wow_housing_variant_id(entry_id, variant_id)
  info.variantID = variant_id
  return info
end

local function __wow_housing_copy_entry_info(entry_id)
  local entry = __wow_housing_seeded_entries[entry_id]
  if not entry then
    return nil
  end
  local info = __wow_housing_clone_table(entry)
  info.entryID = __wow_housing_variant_id(entry_id, 0)
  return info
end

local function __wow_housing_copy_featured_small_products()
  local infos = {}
  for index, item in ipairs(__wow_housing_seeded_featured_small_products) do
    local copy = __wow_housing_clone_table(item)
    copy.entryVariantID = __wow_housing_variant_id(item.entryID, 1)
    copy.entryID = item.entryID
    infos[index] = copy
  end
  return infos
end

local function __wow_housing_copy_bundle_info(bundle_product_id)
  local bundle = __wow_housing_seeded_bundle_state[bundle_product_id]
  if not bundle then
    return nil
  end
  local info = __wow_housing_clone_table(bundle)
  info.entryIDs = __wow_housing_clone_table(bundle.entryIDs)
  info.decorEntries = {}
  for index, decor_entry in ipairs(bundle.decorEntries) do
    info.decorEntries[index] = __wow_housing_clone_table(decor_entry)
  end
  info.nonDecorProducts = __wow_housing_clone_table(bundle.nonDecorProducts)
  return info
end

local function __wow_housing_copy_market_info(decor_id)
  local market = __wow_housing_seeded_market_state[decor_id]
  if not market then
    return nil
  end
  local info = __wow_housing_clone_table(market)
  info.bundleIDs = __wow_housing_clone_table(market.bundleIDs)
  info.isInCart = market.cartCount > 0
  info.cartCount = market.cartCount
  info.wasViewedInStore = market.wasViewedInStore
  return info
end

local function __wow_housing_copy_product_info(product_id)
  local product = __wow_housing_seeded_product_infos[product_id]
  if not product then
    return nil
  end
  local copy = __wow_housing_clone_table(product)
  copy.subItems = __wow_housing_clone_table(product.subItems)
  copy.virtualCurrencies = __wow_housing_clone_table(product.virtualCurrencies)
  copy.creatureDisplayInfoIDs = __wow_housing_clone_table(product.creatureDisplayInfoIDs)
  copy.spellVisualIDs = __wow_housing_clone_table(product.spellVisualIDs)
  copy.itemModifiedAppearanceIDs = __wow_housing_clone_table(product.itemModifiedAppearanceIDs)
  copy.productIDList = __wow_housing_clone_table(product.productIDList)
  return copy
end

local function __wow_housing_copy_product_display_info(product_id)
  local product = __wow_housing_seeded_product_infos[product_id]
  if not product then
    return nil
  end
  return {
    defaultPreviewModelSceneID = 0,
    defaultCardModelSceneID = 0,
    defaultWideCardModelSceneID = 0,
    itemID = product.itemID or 0,
    overridePreviewModelSceneID = nil,
    overrideCardModelSceneID = nil,
    overrideWideCardModelSceneID = nil,
    creatureDisplayInfoIDs = __wow_housing_clone_table(product.creatureDisplayInfoIDs),
    spellVisualIDs = __wow_housing_clone_table(product.spellVisualIDs),
    mainHandItemModifiedAppearanceID = product.mainHandItemModifiedAppearanceID,
    offHandItemModifiedAppearanceID = product.offHandItemModifiedAppearanceID,
    itemModifiedAppearanceIDs = __wow_housing_clone_table(product.itemModifiedAppearanceIDs),
    iconFileDataID = nil,
    iconTextureKit = nil,
    productType = product.productType,
    itemDescription = product.description,
    hasUnknownLicense = false,
    productPMTURL = nil,
    additionalProductPMTURLs = {},
    otherProductImageAtlasName = nil,
    otherProductGameTitleBaseTag = nil,
    otherProductGameType = nil,
    customLoopingSoundStart = nil,
    customLoopingSoundMiddle = nil,
    customLoopingSoundEnd = nil,
    specialActorID_1 = nil,
    specialActorID_2 = nil,
    specialActorID_3 = nil,
    specialActorID_4 = nil,
    specialActorID_5 = nil,
    gameFlavorID = nil,
    decorFileDataID = product.decorFileDataID,
    quantity = product.decorQuantity,
    houseTextureAtlas = product.houseTextureAtlas,
  }
end

local function __wow_housing_catalog_search_results(searcher_state)
  local results = {}
  local search_text = (searcher_state.searchText or ""):lower()
  local filtered_category = searcher_state.filteredCategoryID
  local filtered_subcategory = searcher_state.filteredSubcategoryID
  local base_variant_only = searcher_state.baseVariantOnly

  for entry_id, entry in pairs(__wow_housing_seeded_entries) do
    local entry_matches_category = (not filtered_category) or filtered_category == __wow_housing_all_category_id or __wow_housing_list_contains(entry.categoryIDs, filtered_category)
    local entry_matches_subcategory = (not filtered_subcategory) or __wow_housing_list_contains(entry.subcategoryIDs, filtered_subcategory)
    local entry_matches_search = search_text == "" or entry.name:lower():find(search_text, 1, true) ~= nil
    if entry_matches_category and entry_matches_subcategory and entry_matches_search then
      local variants = __wow_housing_seeded_variants[entry_id] or {}
      for variant_id, variant in pairs(variants) do
        if (not base_variant_only) or variant_id == 1 then
          local variant_id_table = __wow_housing_variant_id(entry_id, variant_id)
          results[#results + 1] = variant_id_table
        end
      end
    end
  end

  table.sort(results, function(lhs, rhs)
    if lhs.recordID ~= rhs.recordID then
      return lhs.recordID < rhs.recordID
    end
    return (lhs.variantIdentifier or 0) < (rhs.variantIdentifier or 0)
  end)
  return results
end

local function __wow_housing_make_catalog_searcher()
  local state = {
    searchText = nil,
    filteredCategoryID = __wow_housing_all_category_id,
    filteredSubcategoryID = nil,
    sortType = Enum.HousingCatalogSortType and Enum.HousingCatalogSortType.Alphabetical or 0,
    customizableOnly = false,
    allowedIndoors = true,
    allowedOutdoors = true,
    collected = true,
    uncollected = true,
    firstAcquisitionBonusOnly = false,
    storedOnly = false,
    baseVariantOnly = false,
    editorModeContext = nil,
    searchResults = {},
    callback = nil,
    inProgress = false,
    tagStatus = {},
  }

  local function refresh()
    state.searchResults = __wow_housing_catalog_search_results(state)
    state.inProgress = false
    if state.callback then
      state.callback()
    end
  end

  local searcher = {}
  function searcher:SetResultsUpdatedCallback(callback)
    state.callback = callback
  end
  function searcher:SetAutoUpdateOnParamChanges(_enabled) end
  function searcher:SetStoredOnly(enabled) state.storedOnly = not not enabled end
  function searcher:IsStoredOnlyActive() return state.storedOnly end
  function searcher:SetBaseVariantOnly(enabled) state.baseVariantOnly = not not enabled end
  function searcher:IsBaseVariantOnlyActive() return state.baseVariantOnly end
  function searcher:SetEditorModeContext(mode) state.editorModeContext = mode end
  function searcher:GetEditorModeContext() return state.editorModeContext end
  function searcher:SetAllowedIndoors(enabled) state.allowedIndoors = not not enabled end
  function searcher:IsAllowedIndoorsActive() return state.allowedIndoors end
  function searcher:SetAllowedOutdoors(enabled) state.allowedOutdoors = not not enabled end
  function searcher:IsAllowedOutdoorsActive() return state.allowedOutdoors end
  function searcher:SetCollected(enabled) state.collected = not not enabled end
  function searcher:IsCollectedActive() return state.collected end
  function searcher:SetUncollected(enabled) state.uncollected = not not enabled end
  function searcher:IsUncollectedActive() return state.uncollected end
  function searcher:SetCustomizableOnly(enabled) state.customizableOnly = not not enabled end
  function searcher:IsCustomizableOnlyActive() return state.customizableOnly end
  function searcher:SetFirstAcquisitionBonusOnly(enabled) state.firstAcquisitionBonusOnly = not not enabled end
  function searcher:IsFirstAcquisitionBonusOnlyActive() return state.firstAcquisitionBonusOnly end
  function searcher:SetSortType(sortType) state.sortType = sortType end
  function searcher:GetSortType() return state.sortType end
  function searcher:SetFilteredCategoryID(categoryID) state.filteredCategoryID = categoryID or __wow_housing_all_category_id end
  function searcher:GetFilteredCategoryID() return state.filteredCategoryID end
  function searcher:SetFilteredSubcategoryID(subcategoryID) state.filteredSubcategoryID = subcategoryID end
  function searcher:GetFilteredSubcategoryID() return state.filteredSubcategoryID end
  function searcher:SetSearchText(searchText) state.searchText = searchText end
  function searcher:GetSearchText() return state.searchText end
  function searcher:SetFilterTagStatus(groupID, tagID, enabled)
    state.tagStatus[groupID] = state.tagStatus[groupID] or {}
    state.tagStatus[groupID][tagID] = not not enabled
  end
  function searcher:GetFilterTagStatus(groupID, tagID)
    return state.tagStatus[groupID] and state.tagStatus[groupID][tagID] or false
  end
  function searcher:SetAllInFilterTagGroup(groupID, enabled)
    state.tagStatus[groupID] = state.tagStatus[groupID] or {}
    state.tagStatus[groupID].__all = not not enabled
  end
  function searcher:IsSearchInProgress() return state.inProgress end
  function searcher:GetSearchCount() return #state.searchResults end
  function searcher:GetNumSearchItems() return #state.searchResults end
  function searcher:GetAllSearchItems() return state.searchResults end
  function searcher:GetCatalogSearchResults() return state.searchResults end
  function searcher:RunSearch()
    refresh()
  end

  refresh()
  return searcher
end

C_CatalogShop = __wow_merge_namespace(C_CatalogShop, {
  IsShop2Enabled = function() return false end,
  HasNewProducts = function() return false end,
  GetAvailableCategoryIDs = function() return { __wow_housing_all_category_id, 101 } end,
  GetProductIDsForCategory = function(categoryID)
    if categoryID == __wow_housing_all_category_id then
      return { 2003, 20031, 20032 }
    elseif categoryID == 101 then
      return { 2003 }
    elseif categoryID == 102 then
      return { 20031, 20032 }
    end
    return {}
  end,
  GetProductIDsForCategorySection = function(categoryID, sectionID)
    if sectionID ~= 1 then
      return {}
    end
    return C_CatalogShop.GetProductIDsForCategory(categoryID)
  end,
  GetCategoryInfo = function(categoryID)
    if categoryID == __wow_housing_all_category_id then
      return { ID = categoryID, displayName = "All", iconTexture = "Interface\\Icons\\INV_Misc_QuestionMark", linkTag = "all", isDisabled = false, showPersistentRefundButton = false }
    elseif categoryID == 101 then
      return { ID = categoryID, displayName = "Featured", iconTexture = "Interface\\Icons\\INV_Misc_QuestionMark", linkTag = "featured", isDisabled = false, showPersistentRefundButton = false }
    elseif categoryID == 102 then
      return { ID = categoryID, displayName = "Decor", iconTexture = "Interface\\Icons\\INV_Misc_QuestionMark", linkTag = "housing", isDisabled = false, showPersistentRefundButton = false }
    end
    return nil
  end,
  GetProductSortOrder = function(_categoryID, _sectionID, productID)
    local product = __wow_housing_seeded_product_infos[productID]
    if product then
      return product.sortOrder or productID
    end
    return productID
  end,
  GetSectionIDsForCategory = function(categoryID)
    if categoryID == __wow_housing_all_category_id then
      return { 1 }
    end
    return { 1 }
  end,
  GetCategorySectionInfo = function(categoryID, sectionID)
    return {
      ID = sectionID,
      displayName = "Featured",
      parentCatalogShopCategoryInfoID = categoryID,
      cardType = nil,
      scrollGridSize = 1,
      shouldShowRecommendationOptOutDisclaimer = false,
    }
  end,
  GetFailureInfo = function() return nil, nil end,
  RefreshVirtualCurrencyBalance = __wow_noop,
  GetVirtualCurrencyBalance = function() return 0 end,
  OpenCatalogShopInteractionFromShop = function()
    local session_id = "seeded-catalog-shop-session"
    __wow_catalog_shop_emit_seeded_refresh(session_id)
    return session_id
  end,
  OpenCatalogShopInteractionFromHouse = function()
    local session_id = "seeded-catalog-house-session"
    __wow_catalog_shop_emit_seeded_refresh(session_id)
    return session_id
  end,
  CloseCatalogShopInteraction = __wow_noop,
  GetFirstCategoryByProductID = function(productID)
    if productID == 2003 or productID == 20031 or productID == 20032 then
      return C_CatalogShop.GetCategoryInfo(101)
    end
    return nil
  end,
  ShouldShowHousingWarning = function() return false end,
  GetProductInfo = function(productID) return __wow_housing_copy_product_info(productID) end,
  GetCatalogShopProductDisplayInfo = function(productID) return __wow_housing_copy_product_display_info(productID) end,
  GetRefundableDecors = function()
    return {}, 0
  end,
  GetProductIDsForBundle = function(productID)
    local product = __wow_housing_seeded_product_infos[productID]
    if not product or not product.isBundle then
      return {}
    end
    local children = {}
    for index, child_id in ipairs(product.productIDList or {}) do
      children[index] = { childProductID = child_id, displayOrder = index, quantityInBundle = 1 }
    end
    return children
  end,
  GetSpellVisualInfoForMount = function() return nil end,
  PurchaseProduct = __wow_noop,
  ConfirmHousingPurchase = __wow_noop,
  ProductDisplayedTelemetry = __wow_noop,
  OnLegalDisclaimerClicked = __wow_noop,
  FindBestCurrencyProductForNeededAmount = function() return nil end,
  IsProductIncludedInAnyBundle = function(productID)
    return productID == 20031 or productID == 20032
  end,
  GetProductAvailabilityTimeRemainingSecs = function() return 1 end,
  OnLegalPersonalizedOptOutClicked = __wow_noop,
  ProductSelectedTelemetry = __wow_noop,
})

C_CraftingOrders = __wow_merge_namespace(C_CraftingOrders, {
  GetPersonalOrdersInfo = function() return {} end,
})

C_Calendar = __wow_merge_namespace(C_Calendar, {
  GetNumPendingInvites = function() return 0 end,
  GetClubCalendarEvents = function()
    return {}
  end,
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
      elseif frame.Hide then
        frame:Hide()
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

    function pool:IsActive(frame)
      return self.active[frame] == true
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

    function pool:IsActive(region)
      return self.active[region] == true
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

    local function find_pool_by_template(collection, template, specialization)
      for _, pool in pairs(collection.pools) do
        if pool.template == template and pool.specialization == specialization then
          return pool
        end
      end
      return nil
    end

    function collection:CreatePool(frameType, parent, template, resetter, _forbidden, specialization)
      local key = pool_key(frameType, parent, template, specialization)
      local pool = CreateFramePool(frameType, parent, template, resetter)
      pool.specialization = specialization
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

    function collection:Acquire(template, specialization)
      local pool = find_pool_by_template(self, template, specialization)
      if pool == nil then
        return nil
      end
      return pool:Acquire()
    end

    function collection:GetNumActive()
      local total = 0
      for _, pool in pairs(self.pools) do
        total = total + (pool.GetNumActive and pool:GetNumActive() or 0)
      end
      return total
    end

    function collection:IsActive(object)
      for _, pool in pairs(self.pools) do
        if pool.IsActive and pool:IsActive(object) then
          return true
        end
      end
      return false
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
    local factory = {
      templateInfoCache = CreateTemplateInfoCache and CreateTemplateInfoCache() or nil,
      poolCollection = CreateFramePoolCollection and CreateFramePoolCollection() or nil,
    }

    function factory:GetTemplateInfoCache()
      return self.templateInfoCache
    end

    function factory:Create(parent, frameTypeOrTemplate, resetFunc)
      local info = self.templateInfoCache and self.templateInfoCache:GetTemplateInfo(frameTypeOrTemplate) or nil
      local frameTemplate = nil
      local frameType = nil
      local specialization = nil

      if info then
        frameTemplate = frameTypeOrTemplate
        frameType = info.type
      else
        frameTemplate = ""
        frameType = type(frameTypeOrTemplate) == "string" and frameTypeOrTemplate or "Frame"
        specialization = frameType
      end

      if self.poolCollection and self.poolCollection.GetOrCreatePool then
        local pool = self.poolCollection:GetOrCreatePool(frameType, parent, frameTemplate, resetFunc, nil, specialization)
        local frame, isNew = pool:Acquire()
        return frame, isNew, info
      end

      local frame = CreateFrame(frameType, nil, parent, frameTemplate)
      if resetFunc then
        resetFunc(nil, frame, true, frameTemplate)
      end
      return frame, true, info
    end

    function factory:GetNumActive()
      if self.poolCollection and self.poolCollection.GetNumActive then
        return self.poolCollection:GetNumActive()
      end
      return 0
    end

    function factory:ReleaseAll()
      if self.poolCollection and self.poolCollection.ReleaseAll then
        self.poolCollection:ReleaseAll()
      end
    end

    function factory:Release(frame)
      if self.poolCollection and self.poolCollection.Release then
        return self.poolCollection:Release(frame)
      end
      return false
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

if GetEditBoxMetatable == nil then
  function GetEditBoxMetatable()
    if CreateFrame == nil then
      return nil
    end
    local frame = CreateFrame("EditBox")
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

-- AchievementDisplayMixin: stubbed in the simulator. The live mixin
-- (Blizzard_FrameXML/AchievementDisplayFrame.lua) renders bullet rows
-- via a frame pool keyed off `GetAchievementInfo`. The simulator skips
-- the bullet rendering — the AlliedRaces panel that drives this method
-- is already cosmetic without 3D models — and only records the ID list
-- so addons that round-trip `self.achievementIds` see what they wrote.
-- A post-load workaround re-installs this stub after Blizzard FrameXML
-- loads, because that file overwrites `AchievementDisplayMixin = {}`.
AchievementDisplayMixin = AchievementDisplayMixin or {}
if rawget(AchievementDisplayMixin, "SetAchievements") == nil then
  function AchievementDisplayMixin:SetAchievements(achievementIds)
    self.achievementIds = achievementIds
  end
end

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
      -- Blizzard_Professions panels look up the tradeskill experience bar
      -- fill color by baseTag in the C_UIColor.GetColors() return value.
      { baseTag = "TRADESKILL_EXPERIENCE_COLOR", color = { r = 0.25, g = 0.25, b = 0.75, a = 1 } },
    }
  end,
})

QuestDifficultyColors = QuestDifficultyColors or {}
QuestDifficultyColors.trivial = QuestDifficultyColors.trivial or { r = 0.50, g = 0.50, b = 0.50 }
QuestDifficultyColors.standard = QuestDifficultyColors.standard or { r = 0.25, g = 0.75, b = 0.25 }
QuestDifficultyColors.difficult = QuestDifficultyColors.difficult or { r = 1.00, g = 1.00, b = 0.00 }
QuestDifficultyColors.verydifficult = QuestDifficultyColors.verydifficult or { r = 1.00, g = 0.50, b = 0.25 }
QuestDifficultyColors.impossible = QuestDifficultyColors.impossible or { r = 1.00, g = 0.10, b = 0.10 }

QuestDifficultyHighlightColors = QuestDifficultyHighlightColors or {}
QuestDifficultyHighlightColors.trivial = QuestDifficultyHighlightColors.trivial or { r = 0.70, g = 0.70, b = 0.70 }
QuestDifficultyHighlightColors.standard = QuestDifficultyHighlightColors.standard or { r = 0.50, g = 1.00, b = 0.50 }
QuestDifficultyHighlightColors.difficult = QuestDifficultyHighlightColors.difficult or { r = 1.00, g = 1.00, b = 0.50 }
QuestDifficultyHighlightColors.verydifficult = QuestDifficultyHighlightColors.verydifficult or { r = 1.00, g = 0.75, b = 0.50 }
QuestDifficultyHighlightColors.impossible = QuestDifficultyHighlightColors.impossible or { r = 1.00, g = 0.40, b = 0.40 }

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
if GetLFGQueuedList == nil then
  function GetLFGQueuedList(_category, queuedList)
    queuedList = queuedList or {}
    for key in pairs(queuedList) do
      queuedList[key] = nil
    end
    return queuedList
  end
end
if GetLFGReadyCheckUpdate == nil then
  function GetLFGReadyCheckUpdate()
    return false, false
  end
end
function HasCompletedAnyAchievement()
  return true
end
function CanShowAchievementUI()
  return true
end
if GetPartyLFGID == nil then
  function GetPartyLFGID() return 0 end
end

-- Adventure journal: fallback only; Rust registration seeds visible suggestions.
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
  if not PlayerSpellsFrame
    and type(C_AddOns) == "table"
    and type(C_AddOns.IsAddOnLoaded) == "function"
    and type(C_AddOns.LoadAddOn) == "function"
    and not C_AddOns.IsAddOnLoaded("Blizzard_PlayerSpells")
  then
    C_AddOns.LoadAddOn("Blizzard_PlayerSpells")
  end
  if not PlayerSpellsFrame and type(PlayerSpellsFrame_LoadUI) == "function" then
    PlayerSpellsFrame_LoadUI()
  end
  return PlayerSpellsFrame
end

local function __wow_call_playerspells_util(methodName, bootstrapMethod, ...)
  __wow_load_player_spells_frame()
  if type(PlayerSpellsUtil) ~= "table" then
    return nil
  end
  local method = rawget(PlayerSpellsUtil, methodName)
  if type(method) ~= "function" or method == bootstrapMethod then
    return nil
  end
  return method(...)
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
local __wow_bootstrap_toggle_player_spells_frame
if rawget(PlayerSpellsUtil, "TogglePlayerSpellsFrame") == nil then
  __wow_bootstrap_toggle_player_spells_frame = function(suggestedTab, inspectUnit)
    return __wow_call_playerspells_util(
      "TogglePlayerSpellsFrame",
      __wow_bootstrap_toggle_player_spells_frame,
      suggestedTab,
      inspectUnit
    )
  end
  PlayerSpellsUtil.TogglePlayerSpellsFrame = __wow_bootstrap_toggle_player_spells_frame
end
local __wow_bootstrap_open_to_spellbook_tab_at_spell
if rawget(PlayerSpellsUtil, "OpenToSpellBookTabAtSpell") == nil then
  __wow_bootstrap_open_to_spellbook_tab_at_spell = function(
    spellID,
    knownSpellsOnly,
    toggleFlyout,
    flyoutReason
  )
    return __wow_call_playerspells_util(
      "OpenToSpellBookTabAtSpell",
      __wow_bootstrap_open_to_spellbook_tab_at_spell,
      spellID,
      knownSpellsOnly,
      toggleFlyout,
      flyoutReason
    )
  end
  PlayerSpellsUtil.OpenToSpellBookTabAtSpell = __wow_bootstrap_open_to_spellbook_tab_at_spell
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
if rawget(PlayerSpellsUtil, "ToggleSpellBookFrame") == nil then
  function PlayerSpellsUtil.ToggleSpellBookFrame(spellBookCategory)
    return PlayerSpellsUtil.TogglePlayerSpellsFrame(
      PlayerSpellsUtil.FrameTabs.SpellBook,
      spellBookCategory
    )
  end
end
local __wow_bootstrap_open_to_spellbook_tab
if rawget(PlayerSpellsUtil, "OpenToSpellBookTab") == nil then
  __wow_bootstrap_open_to_spellbook_tab = function()
    return __wow_call_playerspells_util(
      "OpenToSpellBookTab",
      __wow_bootstrap_open_to_spellbook_tab
    )
  end
  PlayerSpellsUtil.OpenToSpellBookTab = __wow_bootstrap_open_to_spellbook_tab
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

local __wow_reputation_state = rawget(_G, "__wow_reputation_state")
if type(__wow_reputation_state) ~= "table" then
  __wow_reputation_state = {
    selectedFaction = 0,
    watchedFactionID = 2590,
    sortType = Enum.ReputationSortType and Enum.ReputationSortType.None or 0,
    showLegacy = true,
    factions = {
      { factionID = 0, name = "The War Within", description = "", reaction = 0, standing = 0, bottom = 0, top = 0, isHeader = true, isCollapsed = false, isChild = false, isAccountWide = false, isLegacy = false },
      { factionID = 2590, name = "Council of Dornogal", description = "The governing body of Dornogal.", reaction = 6, standing = 8200, bottom = 0, top = 12000, isHeader = false, isCollapsed = false, isChild = true, isAccountWide = true, isLegacy = false },
      { factionID = 2570, name = "Hallowfall Arathi", description = "The Arathi settlers of Hallowfall.", reaction = 7, standing = 4500, bottom = 0, top = 21000, isHeader = false, isCollapsed = false, isChild = true, isAccountWide = true, isLegacy = false },
      { factionID = 2600, name = "The Assembly of the Deeps", description = "United denizens of the deep.", reaction = 6, standing = 11000, bottom = 0, top = 12000, isHeader = false, isCollapsed = false, isChild = true, isAccountWide = true, isLegacy = false },
      { factionID = 2605, name = "The Severed Threads", description = "A coalition of Nerubian outcasts.", reaction = 5, standing = 4800, bottom = 0, top = 6000, isHeader = false, isCollapsed = false, isChild = true, isAccountWide = true, isLegacy = false },
      { factionID = 0, name = "Dragonflight", description = "", reaction = 0, standing = 0, bottom = 0, top = 0, isHeader = true, isCollapsed = false, isChild = false, isAccountWide = false, isLegacy = false },
      { factionID = 2507, name = "Dragonscale Expedition", description = "Explorers of the Dragon Isles.", reaction = 8, standing = 999, bottom = 0, top = 1000, isHeader = false, isCollapsed = false, isChild = true, isAccountWide = true, isLegacy = false },
      { factionID = 2510, name = "Valdrakken Accord", description = "The united dragonflights.", reaction = 8, standing = 999, bottom = 0, top = 1000, isHeader = false, isCollapsed = false, isChild = true, isAccountWide = true, isLegacy = false },
      { factionID = 0, name = "Classic", description = "", reaction = 0, standing = 0, bottom = 0, top = 0, isHeader = true, isCollapsed = false, isChild = false, isAccountWide = false, isLegacy = true },
      { factionID = 72, name = "Stormwind", description = "The Kingdom of Stormwind.", reaction = 8, standing = 999, bottom = 0, top = 1000, isHeader = false, isCollapsed = false, isChild = true, isAccountWide = false, isLegacy = true },
      { factionID = 47, name = "Ironforge", description = "The Dwarven capital.", reaction = 8, standing = 999, bottom = 0, top = 1000, isHeader = false, isCollapsed = false, isChild = true, isAccountWide = false, isLegacy = true },
    },
  }
  rawset(_G, "__wow_reputation_state", __wow_reputation_state)
end

local function __wow_reputation_normalize_faction(faction)
  if type(faction) ~= "table" then
    return faction
  end
  faction.currentReactionThreshold = faction.currentReactionThreshold or faction.bottom or 0
  faction.nextReactionThreshold = faction.nextReactionThreshold or faction.top or faction.currentReactionThreshold
  faction.currentStanding = faction.currentStanding or faction.standing or faction.currentReactionThreshold
  if faction.isAccountWide == nil then
    faction.isAccountWide = false
  end
  if faction.isLegacy == nil then
    faction.isLegacy = false
  end
  return faction
end

for _, faction in ipairs(__wow_reputation_state.factions) do
  __wow_reputation_normalize_faction(faction)
end

local function __wow_reputation_visible_factions()
  local visible = {}
  local hideChildren = false
  for _, faction in ipairs(__wow_reputation_state.factions) do
    if faction.isHeader then
      hideChildren = faction.isCollapsed == true
      table.insert(visible, faction)
    elseif not hideChildren then
      table.insert(visible, faction)
    end
  end
  return visible
end

local function __wow_reputation_find_visible(index)
  return __wow_reputation_visible_factions()[index]
end

local function __wow_reputation_find_by_id(factionID)
  for _, faction in ipairs(__wow_reputation_state.factions) do
    if not faction.isHeader and faction.factionID == factionID then
      return faction
    end
  end
  return nil
end

local function __wow_reputation_find_header(index)
  local visible = __wow_reputation_visible_factions()
  local faction = visible[index]
  if faction and faction.isHeader then
    return faction
  end
  return nil
end

C_Reputation = __wow_merge_namespace(C_Reputation, {})
C_Reputation.GetNumFactions = function()
  return #__wow_reputation_visible_factions()
end
C_Reputation.GetFactionDataByIndex = function(index)
  return __wow_reputation_find_visible(index)
end
C_Reputation.GetFactionDataByID = function(factionID)
  return __wow_reputation_find_by_id(factionID)
end
C_Reputation.GetSelectedFaction = function()
  return __wow_reputation_state.selectedFaction or 0
end
C_Reputation.SetSelectedFaction = function(index)
  __wow_reputation_state.selectedFaction = tonumber(index) or 0
end
C_Reputation.GetWatchedFactionData = function()
  local faction = __wow_reputation_find_by_id(__wow_reputation_state.watchedFactionID)
    or __wow_reputation_find_visible(1)
  if faction == nil then
    return nil
  end
  local info = {}
  for key, value in pairs(faction) do
    info[key] = value
  end
  info.factionID = info.factionID or 0
  info.reaction = info.reaction or info.standing or 0
  info.currentReactionThreshold = info.currentReactionThreshold or 0
  info.nextReactionThreshold = info.nextReactionThreshold or info.topValue or 3000
  info.currentStanding = info.currentStanding or 0
  return info
end
C_Reputation.SetWatchedFactionByIndex = function(index)
  local faction = __wow_reputation_find_visible(index)
  __wow_reputation_state.watchedFactionID = faction and faction.factionID or 0
end
C_Reputation.SetWatchedFactionByID = function(factionID)
  __wow_reputation_state.watchedFactionID = tonumber(factionID) or 0
end
C_Reputation.CollapseFactionHeader = function(index)
  local header = __wow_reputation_find_header(index)
  if header then
    header.isCollapsed = true
  end
end
C_Reputation.ExpandFactionHeader = function(index)
  local header = __wow_reputation_find_header(index)
  if header then
    header.isCollapsed = false
  end
end
C_Reputation.CollapseAllFactionHeaders = function()
  for _, faction in ipairs(__wow_reputation_state.factions) do
    if faction.isHeader then
      faction.isCollapsed = true
    end
  end
end
C_Reputation.ExpandAllFactionHeaders = function()
  for _, faction in ipairs(__wow_reputation_state.factions) do
    if faction.isHeader then
      faction.isCollapsed = false
    end
  end
end
C_Reputation.GetReputationSortType = function()
  return __wow_reputation_state.sortType
end
C_Reputation.SetReputationSortType = function(sortType)
  __wow_reputation_state.sortType = tonumber(sortType) or 0
end
C_Reputation.AreLegacyReputationsShown = function()
  return __wow_reputation_state.showLegacy == true
end
C_Reputation.SetLegacyReputationsShown = function(shown)
  __wow_reputation_state.showLegacy = shown ~= false
end
C_Reputation.GetGuildFactionData = function()
  return __wow_reputation_normalize_faction({
    factionID = 1168,
    name = "Guild",
    description = "Guild reputation",
    reaction = 8,
    standing = 1000,
    bottom = 0,
    top = 1000,
    isHeader = false,
    isCollapsed = false,
    isChild = false,
  })
end
C_Reputation.IsAccountWideReputation = function(factionID)
  local faction = __wow_reputation_find_by_id(tonumber(factionID) or 0)
  return faction ~= nil and faction.isAccountWide == true
end
if C_Reputation.IsFactionParagonForCurrentPlayer == nil then
  C_Reputation.IsFactionParagonForCurrentPlayer = function()
    return false
  end
end
if C_Reputation.IsFactionParagon == nil then
  C_Reputation.IsFactionParagon = function()
    return false
  end
end
if C_Reputation.IsMajorFaction == nil then
  C_Reputation.IsMajorFaction = function()
    return false
  end
end
if C_Reputation.GetFactionParagonInfo == nil then
  C_Reputation.GetFactionParagonInfo = function()
    return nil
  end
end
C_Reputation.RequestFactionParagonPreloadRewardData = __wow_noop
C_Reputation.IsFactionActive = function()
  return true
end
C_Reputation.SetFactionActive = __wow_noop
C_Reputation.ToggleFactionAtWar = __wow_noop

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

-- Additional LFG helpers.
if GetLFGCategoryForID == nil then
  function GetLFGCategoryForID() return 0 end
end

-- Battle.net friends count: sim has no BNet connection.
if BNGetNumFriends == nil then
  function BNGetNumFriends() return 0, 0, 0, 0 end
end
if BNGetNumFriendInvites == nil then
  function BNGetNumFriendInvites() return 0 end
end
if BNGetFriendInfo == nil then
  function BNGetFriendInfo() return nil end
end
if Ambiguate == nil then
  function Ambiguate(fullName, context)
    if context == "none" then
      return fullName
    end
    return string.match(fullName, "^(.-)%-.+$") or fullName
  end
end
if AreTalentsLocked == nil then
  function AreTalentsLocked() return false end
end
if Sound_GameSystem_GetNumOutputDrivers == nil then
  function Sound_GameSystem_GetNumOutputDrivers() return 1 end
end
if Sound_GameSystem_GetOutputDriverNameByIndex == nil then
  function Sound_GameSystem_GetOutputDriverNameByIndex(index)
    if index == 0 then
      return "Silent Output Device"
    end
    return nil
  end
end
if Sound_GameSystem_GetNumInputDrivers == nil then
  function Sound_GameSystem_GetNumInputDrivers() return 1 end
end
if Sound_GameSystem_GetInputDriverNameByIndex == nil then
  function Sound_GameSystem_GetInputDriverNameByIndex(index)
    if index == 0 then
      return "Silent Input Device"
    end
    return nil
  end
end
if Sound_ChatSystem_GetNumOutputDrivers == nil then
  function Sound_ChatSystem_GetNumOutputDrivers() return 1 end
end
if Sound_ChatSystem_GetOutputDriverNameByIndex == nil then
  function Sound_ChatSystem_GetOutputDriverNameByIndex(index)
    if index == 0 then
      return "Silent Voice Output Device"
    end
    return nil
  end
end
if Sound_ChatSystem_GetNumInputDrivers == nil then
  function Sound_ChatSystem_GetNumInputDrivers() return 1 end
end
if Sound_ChatSystem_GetInputDriverNameByIndex == nil then
  function Sound_ChatSystem_GetInputDriverNameByIndex(index)
    if index == 0 then
      return "Silent Voice Input Device"
    end
    return nil
  end
end
if Sound_GameSystem_RestartSoundSystem == nil then
  function Sound_GameSystem_RestartSoundSystem() end
end

-- Friend list: sim has no social layer.
C_FriendList = C_FriendList or __wow_namespace()
if rawget(C_FriendList, "GetNumFriends") == nil then
  function C_FriendList.GetNumFriends() return 0 end
end
if rawget(C_FriendList, "GetNumOnlineFriends") == nil then
  function C_FriendList.GetNumOnlineFriends() return 0 end
end
if GetNumMacros == nil then
  function GetNumMacros() return 2, 1 end
end
if GetMacroInfo == nil then
  function GetMacroInfo(index)
    if index == 1 then
      return "Raid Beacon", "Interface\\Icons\\INV_Misc_QuestionMark", "/rw Stack on star"
    end
    if index == 121 then
      return "Crusader", "Interface\\Icons\\Spell_Holy_CrusaderAura", "/cast Crusader Aura"
    end
    return nil
  end
end
__wow_loose_macro_icons = {
  "INV_Misc_QuestionMark",
}
__wow_macro_icons = {
  "Spell_Holy_CrusaderAura",
}
__wow_loose_macro_item_icons = {
  "INV_Misc_Bag_08",
}
__wow_macro_item_icons = {
  "INV_Sword_04",
}
function __wow_append_icons(iconTable, icons)
  if type(iconTable) ~= "table" then
    iconTable = {}
  end
  for _, icon in ipairs(icons) do
    table.insert(iconTable, icon)
  end
  return iconTable
end
if GetLooseMacroIcons == nil then
  function GetLooseMacroIcons(iconTable)
    __wow_append_icons(iconTable, __wow_loose_macro_icons)
  end
end
if GetLooseMacroItemIcons == nil then
  function GetLooseMacroItemIcons(iconTable)
    __wow_append_icons(iconTable, __wow_loose_macro_item_icons)
  end
end
if GetMacroIcons == nil then
  function GetMacroIcons(iconTable)
    return __wow_append_icons(iconTable, __wow_macro_icons)
  end
end
if GetMacroItemIcons == nil then
  function GetMacroItemIcons(iconTable)
    return __wow_append_icons(iconTable, __wow_macro_item_icons)
  end
end

C_Macro = C_Macro or __wow_namespace()
if rawget(C_Macro, "GetNumMacros") == nil then
  function C_Macro.GetNumMacros() return 2, 1 end
end
if rawget(C_Macro, "GetMacroName") == nil then
  function C_Macro.GetMacroName(index)
    if index == 1 then
      return "Raid Beacon"
    end
    if index == 121 then
      return "Crusader"
    end
    return nil
  end
end
if rawget(C_Macro, "GetSelectedMacroIcon") == nil then
  function C_Macro.GetSelectedMacroIcon(index)
    if index == 121 then
      return "Interface\\Icons\\Spell_Holy_CrusaderAura"
    end
    return nil
  end
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
  GetSoulCurrencies = function()
    return {}
  end,
  GetAnimaInfo = function()
    return 0, 0
  end,
  CanDepositAnima = function()
    return false
  end,
  DepositAnima = function() end,
  EndInteraction = function() end,
  GetFeatures = function()
    return {}
  end,
  GetCurrentTalentTreeID = function()
    return 0
  end,
})

-- C_LevelLink.IsActionLocked is registered from Rust
-- (src/lua_api/globals/missing_surface/small_namespaces.rs), backed by
-- SimState::locked_action_slots. The _state table below still backs the
-- bootstrap-only IsSpellLocked implementation.
local __wow_level_link_state = type(C_LevelLink) == "table" and rawget(C_LevelLink, "_state") or nil
C_LevelLink = __wow_merge_namespace(C_LevelLink, {
  _state = __wow_level_link_state or {
    lockedSpells = {},
    lastSpellQuery = nil,
  },
})

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

-- C_GuildInfo.GetClubId / IsGuildOfficer / CanSpeakInGuildChat /
-- GetMOTD / SetMOTD / GetInfoText / SetInfoText are registered from Rust
-- (src/lua_api/globals/guild_info.rs), backed by SimState::world fields.
-- Merge the stub-namespace __index fallback so other unimplemented
-- C_GuildInfo members resolve to the no-op metamethod.
C_GuildInfo = __wow_merge_namespace(C_GuildInfo, {})
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
if HasLootSpecializations == nil then
  function HasLootSpecializations()
    return true
  end
end
if CanShowSetRoleButton == nil then
  function CanShowSetRoleButton()
    return false
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

-- Housing service flag stays in Rust, but the seeded housing/catalog UI
-- namespaces need enough state for the full Blizzard UI to load.
-- C_Housing.IsHousingServiceEnabled is registered from Rust
-- (src/lua_api/globals/housing.rs), backed by SimState::housing_service_enabled.
-- Admin: A_Admin.SetHousingServiceEnabled(b?).
-- Merge stub-namespace fallback so other unimplemented C_Housing members
-- resolve to the no-op metamethod.
C_Housing = __wow_merge_namespace(C_Housing, {})
C_HousingBasicMode = __wow_merge_namespace(C_HousingBasicMode, {
  IsPlacingNewDecor = function() return false end,
  IsDecorSelected = function() return C_HousingDecor.IsDecorSelected() end,
  GetSelectedDecorInfo = function() return C_HousingDecor.GetSelectedDecorInfo() end,
  IsHouseExteriorSelected = function() return false end,
  CommitDecorMovement = __wow_noop,
  CommitHouseExteriorPosition = __wow_noop,
  CancelActiveEditing = __wow_noop,
  FinishPlacingNewDecor = __wow_noop,
  StartPlacingNewDecor = __wow_noop,
  StartPlacingPreviewDecor = __wow_noop,
  IsGridSnapEnabled = function() return false end,
  SetGridSnapEnabled = __wow_noop,
  IsGridVisible = function() return false end,
  SetGridVisible = __wow_noop,
  IsFreePlaceEnabled = function() return true end,
  SetFreePlaceEnabled = __wow_noop,
})
C_HousingExpertMode = __wow_merge_namespace(C_HousingExpertMode, {
  IsDecorSelected = function() return C_HousingDecor.IsDecorSelected() end,
  IsHouseExteriorSelected = function() return false end,
  GetSelectedDecorInfo = function() return C_HousingDecor.GetSelectedDecorInfo() end,
  CancelActiveEditing = __wow_noop,
  GetPrecisionSubmode = function() return 0 end,
  SetPrecisionSubmode = __wow_noop,
  GetPrecisionSubmodeRestriction = function() return 0 end,
  ResetPrecisionChanges = __wow_noop,
})
C_HousingLayout = __wow_merge_namespace(C_HousingLayout, {
  GetSpentPlacementBudget = function() return 0 end,
  GetRoomPlacementBudget = function() return 10 end,
  HasRoomPlacementBudget = function() return false end,
  HasAnySelections = function() return false end,
  GetSelectedFloorplan = function() return nil end,
  SelectFloorplan = __wow_noop,
  DeselectFloorplan = __wow_noop,
  GetViewedFloor = function() return 0 end,
  SetViewedFloor = __wow_noop,
  GetNumFloors = function() return 1 end,
  AnyRoomsOnFloor = function() return false end,
  IsDraggingRoom = function() return false end,
  StartDrag = __wow_noop,
  StopDrag = __wow_noop,
  StopDraggingRoom = __wow_noop,
  CancelActiveLayoutEditing = __wow_noop,
  IsBaseRoom = function() return false end,
  GetSelectedDoor = function() return nil end,
  HasValidConnection = function() return false end,
  RemoveRoom = __wow_noop,
  RotateRoom = __wow_noop,
})
C_HousingDecor = __wow_merge_namespace(C_HousingDecor, {
  CancelActiveEditing = __wow_noop,
  CommitDecorMovement = __wow_noop,
  EnterPreviewState = function() __wow_housing_decor_state.preview = true end,
  ExitPreviewState = function() __wow_housing_decor_state.preview = false end,
  GetAllPlacedDecor = function()
    local placed = {}
    for _, info in ipairs(__wow_housing_decor_state.placedDecor) do
      placed[#placed + 1] = __wow_housing_clone_table(info)
    end
    return placed
  end,
  GetDecorHyperlink = function(decorID)
    local name = __wow_housing_decor_name_by_id[decorID]
    if not name then
      return nil
    end
    return string.format("|cff66bbff|Hhousingdecor:%d|h[%s]|h|r", decorID, name)
  end,
  GetDecorIcon = function(decorID)
    return __wow_housing_decor_icon_by_id[decorID] or 0
  end,
  GetDecorInstanceInfoForGUID = function(decorGUID)
    return __wow_housing_decor_info_by_guid[decorGUID] and __wow_housing_clone_table(__wow_housing_decor_info_by_guid[decorGUID]) or nil
  end,
  GetDecorName = function(decorID)
    return __wow_housing_decor_name_by_id[decorID] or ""
  end,
  GetHoveredDecorInfo = function()
    local decorGUID = __wow_housing_decor_state.hoveredDecorGUID
    return decorGUID and __wow_housing_decor_info_by_guid[decorGUID] and __wow_housing_clone_table(__wow_housing_decor_info_by_guid[decorGUID]) or nil
  end,
  GetHoveredDecorDebugInfo = function()
    return C_HousingDecor.GetHoveredDecorInfo()
  end,
  GetMaxPlacementBudget = function() return 100 end,
  GetNumDecorPlaced = function() return #__wow_housing_decor_state.placedDecor end,
  GetNumPreviewDecor = function() return __wow_housing_decor_state.preview and 1 or 0 end,
  GetPreviewDyesOnSelectedDecor = function() return {} end,
  GetRecentlyUsedDyes = function() return {} end,
  GetSelectedDecorInfo = function()
    local decorGUID = __wow_housing_decor_state.selectedDecorGUID
    return decorGUID and __wow_housing_decor_info_by_guid[decorGUID] and __wow_housing_clone_table(__wow_housing_decor_info_by_guid[decorGUID]) or nil
  end,
  GetSpentPlacementBudget = function() return 20 end,
  HasMaxPlacementBudget = function() return false end,
  IsDecorSelected = function() return __wow_housing_decor_state.selectedDecorGUID ~= nil end,
  IsGridVisible = function() return __wow_housing_decor_state.gridVisible end,
  IsHouseExteriorHovered = function() return false end,
  IsHoveringDecor = function() return __wow_housing_decor_state.hoveredDecorGUID ~= nil end,
  IsModeDisabledForPreviewState = function() return false end,
  IsPreviewState = function() return __wow_housing_decor_state.preview end,
  RemovePlacedDecorEntry = __wow_noop,
  RemoveSelectedDecor = __wow_noop,
  SetGridVisible = function(visible) __wow_housing_decor_state.gridVisible = not not visible end,
  SetPlacedDecorEntryHovered = function(decorGUID, hovered)
    __wow_housing_decor_state.hoveredDecorGUID = hovered and decorGUID or nil
  end,
  SetPlacedDecorEntrySelected = function(decorGUID, selected)
    __wow_housing_decor_state.selectedDecorGUID = selected and decorGUID or nil
  end,
})
C_HousingCustomizeMode = __wow_merge_namespace(C_HousingCustomizeMode, {
  CancelActiveEditing = __wow_noop,
  GetHoveredRoomComponentInfo = function() return nil end,
  GetRecentlyUsedThemeSets = function() return { 1 } end,
  GetRecentlyUsedWallpapers = function() return { 1 } end,
  GetSelectedDecorInfo = function()
    return __wow_housing_clone_table(__wow_housing_customize_mode_selected_decor)
  end,
  GetSelectedRoomComponentInfo = function() return nil end,
  GetThemeSetInfo = function(themeSetID)
    return __wow_housing_theme_set_names[themeSetID]
  end,
  GetWallpapersForRoomComponentType = function(_type)
    return { { roomComponentTextureRecID = 1, name = "Sunspire Plaster" } }
  end,
  IsDecorSelected = function() return true end,
  IsHoveringRoomComponent = function() return false end,
  IsRoomComponentSelected = function() return false end,
  ApplyThemeToSelectedRoomComponent = __wow_noop,
  ApplyWallpaperToSelectedRoomComponent = __wow_noop,
  ApplyWallpaperToAllWalls = __wow_noop,
  ApplyThemeToRoom = __wow_noop,
  ClearTargetRoomComponent = __wow_noop,
  CommitDyesForSelectedDecor = function() return true end,
  GetNumDyesToSpendOnSelectedDecor = function() return 0 end,
  GetNumDyesToRemoveOnSelectedDecor = function() return 0 end,
  GetPreviewDyesOnSelectedDecor = function() return {} end,
  GetRecentlyUsedDyes = function() return {} end,
  RoomComponentSupportsVariant = function() return false end,
  SetGridSnapEnabled = __wow_noop,
  SetGridVisible = __wow_noop,
  SetRoomComponentCeilingType = __wow_noop,
  SetRoomComponentDoorType = __wow_noop,
  SetFreePlaceEnabled = __wow_noop,
})
C_HousingNeighborhood = __wow_merge_namespace(C_HousingNeighborhood, {
  CanReturnAfterVisitingHouse = function() return false end,
  CancelInviteToNeighborhood = __wow_noop,
  DemoteToResident = __wow_noop,
  GetCornerstoneHouseInfo = function()
    return __wow_housing_neighborhood_state.houseInfo and __wow_housing_clone_table(__wow_housing_neighborhood_state.houseInfo) or nil
  end,
  GetCornerstoneNeighborhoodInfo = function()
    return __wow_housing_neighborhood_state.neighborhoodInfo and __wow_housing_clone_table(__wow_housing_neighborhood_state.neighborhoodInfo) or nil
  end,
  GetCornerstonePurchaseMode = function() return 0 end,
  GetCurrentNeighborhoodTextureSuffix = function() return "dawnmeadow" end,
  GetDiscountedMovePrice = function() return 0 end,
  GetMoveCooldownTime = function() return 0 end,
  GetNeighborhoodMapData = function() return {} end,
  GetNeighborhoodName = function() return "Dawnmeadow" end,
  GetNeighborhoodPlotName = function(plotID) return string.format("Plot %s", tostring(plotID)) end,
  GetPreviousHouseIdentifier = function() return "Sunspire Retreat" end,
  HasPermissionToPurchase = function() return false end,
  InvitePlayerToNeighborhood = __wow_noop,
  IsNeighborhoodManager = function() return false end,
  IsNeighborhoodOwner = function() return true end,
  IsPlayerInOtherPlayersPlot = function() return false end,
  IsPlotAvailableForPurchase = function() return false end,
  IsPlotOwnedByPlayer = function() return true end,
  OnBulletinBoardClosed = __wow_noop,
  OnCornerstoneClosed = function()
    __wow_housing_neighborhood_state.houseInfo = nil
    __wow_housing_neighborhood_state.neighborhoodInfo = nil
  end,
  PromoteToManager = __wow_noop,
  RequestNeighborhoodInfo = __wow_noop,
  RequestNeighborhoodRoster = __wow_noop,
  RequestPendingNeighborhoodInvites = __wow_noop,
  SetDesiredNeighborhoodType = __wow_noop,
  TransferNeighborhoodOwnership = __wow_noop,
  TryEvictPlayer = __wow_noop,
  TryMoveHouse = __wow_noop,
  TryPurchasePlot = __wow_noop,
})
C_HouseExterior = __wow_merge_namespace(C_HouseExterior, {
  CancelActiveExteriorEditing = __wow_noop,
  GetCoreFixtureOptionsInfo = function(coreFixtureType)
    if coreFixtureType == Enum.HousingFixtureType.Base then
      return __wow_housing_copy_core_fixture_info(__wow_housing_exterior_state.baseFixtureInfo)
    elseif coreFixtureType == Enum.HousingFixtureType.Roof then
      return __wow_housing_copy_core_fixture_info(__wow_housing_exterior_state.roofFixtureInfo)
    end
    return nil
  end,
  GetCurrentHouseExteriorSize = function() return __wow_housing_exterior_state.selectedSize end,
  GetCurrentHouseExteriorType = function() return __wow_housing_exterior_state.selectedExteriorType, __wow_housing_exterior_state.selectedExteriorTypeName end,
  GetHouseExteriorSizeOptions = function()
    return {
      selectedSize = __wow_housing_exterior_state.selectedSize,
      options = {
        { size = Enum.HousingFixtureSize and Enum.HousingFixtureSize.Medium or 3, name = "Medium" },
        { size = Enum.HousingFixtureSize and Enum.HousingFixtureSize.Large or 4, name = "Large" },
      },
    }
  end,
  GetHouseExteriorTypeOptions = function()
    return {
      selectedExteriorType = __wow_housing_exterior_state.selectedExteriorType,
      options = {
        { houseExteriorTypeID = 1, name = "Sunspire Cottage" },
        { houseExteriorTypeID = 2, name = "Sunspire Manor" },
      },
    }
  end,
  GetSelectedFixturePointInfo = function()
    return __wow_housing_exterior_state.selectedFixturePoint and __wow_housing_clone_table(__wow_housing_exterior_state.selectedFixturePoint) or nil
  end,
  GetHoveredFixtureDebugInfo = function() return nil end,
  HasHoveredFixture = function() return false end,
  HasSelectedFixturePoint = function() return true end,
  IsAnyDecorAttachedToCoreFixture = function(coreFixtureType)
    return coreFixtureType == Enum.HousingFixtureType.Base
  end,
  IsAnyDecorAttachedToDoor = function() return true end,
  IsAnyDecorAttachedToHouseExterior = function() return true end,
  IsAnyDecorAttachedToSelectedFixturePoint = function() return true end,
  IsExteriorDecorHidden = function() return __wow_housing_exterior_state.decorHidden end,
  RemoveFixtureFromSelectedPoint = __wow_noop,
  SelectCoreFixtureOption = __wow_noop,
  SelectFixtureOption = __wow_noop,
  SetExteriorDecorHidden = function(decorHidden)
    __wow_housing_exterior_state.decorHidden = not not decorHidden
  end,
  SetHouseExteriorSize = __wow_noop,
  SetHouseExteriorType = __wow_noop,
})
C_HousingCatalog = __wow_merge_namespace(C_HousingCatalog, {
  CreateCatalogSearcher = function()
    return __wow_housing_make_catalog_searcher()
  end,
  DeletePreviewCartDecor = __wow_noop,
  DestroyEntry = __wow_noop,
  GetAllFilterTagGroups = function() return {} end,
  GetAllVariantInfosForEntry = function(entryID)
    local entry_id = type(entryID) == "table" and entryID.recordID or entryID
    local variants = __wow_housing_seeded_variants[entry_id]
    if not variants then
      return {}
    end
    local results = {}
    for variantID, _variant in pairs(variants) do
      results[#results + 1] = __wow_housing_copy_variant_info(entry_id, variantID)
    end
    table.sort(results, function(lhs, rhs)
      return (lhs.variantID or 0) < (rhs.variantID or 0)
    end)
    return results
  end,
  GetBundleInfo = function(bundleCatalogShopProductID)
    return __wow_housing_copy_bundle_info(bundleCatalogShopProductID)
  end,
  GetCartSizeLimit = function() return 20 end,
  GetCatalogCategoryInfo = function(categoryID)
    if categoryID == __wow_housing_all_category_id then
      return { ID = categoryID, orderIndex = 0, name = "All", icon = nil, subcategoryIDs = { 1001, 1002 }, anyStoredEntries = true }
    elseif categoryID == 101 then
      return { ID = categoryID, orderIndex = 1, name = "Featured", icon = nil, subcategoryIDs = { 1001 }, anyStoredEntries = true }
    elseif categoryID == 102 then
      return { ID = categoryID, orderIndex = 2, name = "Decor", icon = nil, subcategoryIDs = { 1001, 1002 }, anyStoredEntries = true }
    end
    return nil
  end,
  GetCatalogEntryInfo = function(entryVariantID)
    local entry_id = type(entryVariantID) == "table" and entryVariantID.recordID or entryVariantID
    return __wow_housing_copy_entry_info(entry_id)
  end,
  GetCatalogEntryInfoByItem = function(itemInfo)
    local item_id = type(itemInfo) == "table" and (itemInfo.itemID or itemInfo.id) or itemInfo
    return __wow_housing_copy_entry_info(item_id)
  end,
  GetCatalogEntryInfoByRecordID = function(entryType, recordID)
    if entryType ~= __wow_housing_entry_type then
      return nil
    end
    return __wow_housing_copy_entry_info(recordID)
  end,
  GetCatalogEntryRefundTimeStampByRecordID = function() return nil end,
  GetCatalogEntryVariantInfo = function(entryID, variantID)
    local entry_id = type(entryID) == "table" and entryID.recordID or entryID
    local variant_id = type(entryID) == "table" and entryID.variantIdentifier or variantID or 1
    return __wow_housing_copy_variant_info(entry_id, variant_id)
  end,
  GetCatalogSubcategoryInfo = function(subcategoryID)
    if subcategoryID == 1001 then
      return { ID = 1001, orderIndex = 1, parentCategoryID = 102, name = "Seating", icon = nil, anyStoredEntries = true }
    elseif subcategoryID == 1002 then
      return { ID = 1002, orderIndex = 2, parentCategoryID = 102, name = "Lighting", icon = nil, anyStoredEntries = true }
    end
    return nil
  end,
  GetDecorMaxOwnedCount = function() return 99 end,
  GetDecorTotalOwnedCount = function() return 2, 0 end,
  GetDestroyableInstanceCount = function(entryVariantID)
    local entry_id = type(entryVariantID) == "table" and entryVariantID.recordID or entryVariantID
    local variant_id = type(entryVariantID) == "table" and entryVariantID.variantIdentifier or 1
    local variant = __wow_housing_seeded_variants[entry_id] and __wow_housing_seeded_variants[entry_id][variant_id]
    return variant and variant.numStored or 0
  end,
  GetFeaturedBundles = function()
    local featured = { __wow_housing_copy_bundle_info(5001) }
    return featured
  end,
  GetFeaturedSmallProducts = function()
    return __wow_housing_copy_featured_small_products()
  end,
  GetMarketInfoForDecor = function(decorID)
    return __wow_housing_copy_market_info(decorID)
  end,
  HasFeaturedEntries = function() return true end,
  HousingMarketActionAddToCart = function(productID)
    local market = __wow_housing_seeded_market_state[productID]
    if not market then
      return false
    end
    market.cartCount = market.cartCount + 1
    return true
  end,
  HousingMarketActionClearCart = function()
    for _decorID, market in pairs(__wow_housing_seeded_market_state) do
      market.cartCount = 0
    end
  end,
  HousingMarketActionRemoveFromCart = function(productID)
    local market = __wow_housing_seeded_market_state[productID]
    if not market then
      return false
    end
    market.cartCount = 0
    return true
  end,
  HousingMarketActionViewBundle = function(bundleProductID)
    local bundle = __wow_housing_seeded_bundle_state[bundleProductID]
    if not bundle then
      return false
    end
    bundle.wasViewed = true
    return true
  end,
  HousingMarketActionViewInStore = function(productID)
    local market = __wow_housing_seeded_market_state[productID]
    if not market then
      return false
    end
    market.wasViewedInStore = true
    return true
  end,
  IsPreviewCartItemShown = function(decorGUID)
    return __wow_housing_preview_cart_state[decorGUID] or false
  end,
  PromotePreviewDecor = function(decorID, previewDecorGUID)
    __wow_housing_preview_cart_state[previewDecorGUID] = true
    return true
  end,
  RequestHousingMarketInfoRefresh = __wow_noop,
  RequestHousingMarketRefundInfo = __wow_noop,
  SearchCatalogCategories = function(_searchParams)
    return { __wow_housing_all_category_id, 101, 102 }
  end,
  SearchCatalogSubcategories = function(_searchParams)
    return { 1001, 1002 }
  end,
  SetPreviewCartItemShown = function(decorGUID, shown)
    __wow_housing_preview_cart_state[decorGUID] = not not shown
  end,
  IsProductIncludedInAnyBundle = function(productID)
    return productID == 20031 or productID == 20032
  end,
  GetProductAvailabilityTimeRemainingSecs = function() return 1 end,
})
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
  GetHighestPrioritySuperTrackingType = function() return nil end,
  ClearAllSuperTracked = __wow_noop,
  ClearSuperTrackedContent = __wow_noop,
  ClearSuperTrackedMapPin = __wow_noop,
  GetSuperTrackedMapPin = function() return end,
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
C_Macro = C_Macro or __wow_namespace()
local __wow_native_has_action = rawget(_G, "HasAction")
local __wow_native_get_action_texture = rawget(_G, "GetActionTexture")
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
    if type(__wow_native_has_action) == "function" then
      return __wow_native_has_action(slot)
    end
    return false
  end,
  IsPressHoldReleaseSpell = function() return false end,
  GetActionTexture = function(slot)
    if type(__wow_native_get_action_texture) == "function" then
      return __wow_native_get_action_texture(slot)
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
local __wow_secure_transfer_state = type(C_SecureTransfer) == "table" and rawget(C_SecureTransfer, "_state") or nil
C_SecureTransfer = __wow_merge_namespace(C_SecureTransfer, {
  _state = __wow_secure_transfer_state or {
    shouldShowTradeOfferWarning = false,
    tradePartner = nil,
    mailInfo = {
      target = "",
      sendMoney = 0,
    },
    housingPurchaseCost = 0,
    housingPurchaseQuantity = 0,
    housingVCPurchaseProductID = 0,
    acceptTradeCount = 0,
    sendMailCount = 0,
    completeHousingPurchaseCount = 0,
    completeHousingVCPurchaseCount = 0,
    cancelCount = 0,
    lastAction = nil,
  },
})
if rawget(C_SecureTransfer, "GetMailInfo") == nil then
  function C_SecureTransfer.GetMailInfo()
    local mailInfo = C_SecureTransfer._state.mailInfo or {}
    return {
      target = tostring(mailInfo.target or ""),
      sendMoney = tonumber(mailInfo.sendMoney) or 0,
    }
  end
end
if rawget(C_SecureTransfer, "GetTradePartner") == nil then
  function C_SecureTransfer.GetTradePartner()
    return C_SecureTransfer._state.tradePartner
  end
end
if rawget(C_SecureTransfer, "ShouldShowTradeOfferWarning") == nil then
  function C_SecureTransfer.ShouldShowTradeOfferWarning()
    return C_SecureTransfer._state.shouldShowTradeOfferWarning == true
  end
end
if rawget(C_SecureTransfer, "GetHousingPurchaseCost") == nil then
  function C_SecureTransfer.GetHousingPurchaseCost()
    return tonumber(C_SecureTransfer._state.housingPurchaseCost) or 0
  end
end
if rawget(C_SecureTransfer, "GetHousingPurchaseQuantity") == nil then
  function C_SecureTransfer.GetHousingPurchaseQuantity()
    return tonumber(C_SecureTransfer._state.housingPurchaseQuantity) or 0
  end
end
if rawget(C_SecureTransfer, "GetHousingVCPurchaseProductID") == nil then
  function C_SecureTransfer.GetHousingVCPurchaseProductID()
    return tonumber(C_SecureTransfer._state.housingVCPurchaseProductID) or 0
  end
end
if rawget(C_SecureTransfer, "AcceptTrade") == nil then
  function C_SecureTransfer.AcceptTrade()
    C_SecureTransfer._state.acceptTradeCount = (tonumber(C_SecureTransfer._state.acceptTradeCount) or 0) + 1
    C_SecureTransfer._state.lastAction = "AcceptTrade"
  end
end
if rawget(C_SecureTransfer, "SendMail") == nil then
  function C_SecureTransfer.SendMail()
    C_SecureTransfer._state.sendMailCount = (tonumber(C_SecureTransfer._state.sendMailCount) or 0) + 1
    C_SecureTransfer._state.lastAction = "SendMail"
  end
end
if rawget(C_SecureTransfer, "CompleteHousingPurchase") == nil then
  function C_SecureTransfer.CompleteHousingPurchase()
    C_SecureTransfer._state.completeHousingPurchaseCount =
      (tonumber(C_SecureTransfer._state.completeHousingPurchaseCount) or 0) + 1
    C_SecureTransfer._state.lastAction = "CompleteHousingPurchase"
  end
end
if rawget(C_SecureTransfer, "CompleteHousingVCPurchase") == nil then
  function C_SecureTransfer.CompleteHousingVCPurchase()
    C_SecureTransfer._state.completeHousingVCPurchaseCount =
      (tonumber(C_SecureTransfer._state.completeHousingVCPurchaseCount) or 0) + 1
    C_SecureTransfer._state.lastAction = "CompleteHousingVCPurchase"
  end
end
if rawget(C_SecureTransfer, "Cancel") == nil then
  function C_SecureTransfer.Cancel()
    C_SecureTransfer._state.cancelCount = (tonumber(C_SecureTransfer._state.cancelCount) or 0) + 1
    C_SecureTransfer._state.lastAction = "Cancel"
  end
end
if type(UIFrameManager) ~= "table" then
  UIFrameManager = __wow_namespace()
end
if type(UIFrameManager_ManagedFrameMixin) ~= "table" then
  UIFrameManager_ManagedFrameMixin = __wow_namespace()
end
local __wow_ui_frame_manager_registered_frames = {}
local __wow_ui_frame_manager_registered_frame_type_to_frames = {}
local function __wow_ui_frame_manager_ensure_state()
  if type(UIFrameManager) == "table" and UIFrameManager.registeredFrameTypeToFrames ~= __wow_ui_frame_manager_registered_frame_type_to_frames then
    UIFrameManager.registeredFrameTypeToFrames = __wow_ui_frame_manager_registered_frame_type_to_frames
  end
end
if rawget(UIFrameManager, "OnLoad") == nil then
  function UIFrameManager:OnLoad()
    __wow_ui_frame_manager_ensure_state()
    if type(self.RegisterEvent) == "function" then
      self:RegisterEvent("FRAME_MANAGER_UPDATE_ALL")
      self:RegisterEvent("FRAME_MANAGER_UPDATE_FRAME")
    end
  end
end
if rawget(UIFrameManager, "OnEvent") == nil then
  function UIFrameManager:OnEvent(event, ...)
    __wow_ui_frame_manager_ensure_state()
    if event == "FRAME_MANAGER_UPDATE_ALL" then
      for frameType, frames in pairs(__wow_ui_frame_manager_registered_frame_type_to_frames) do
        for frame in pairs(frames) do
          frame:UpdateFrameState(C_FrameManager.GetFrameVisibilityState(frameType))
        end
      end
      return
    end
    local frameType, show = ...
    local frames = __wow_ui_frame_manager_registered_frame_type_to_frames[frameType]
    if not frames then
      return
    end
    for frame in pairs(frames) do
      frame:UpdateFrameState(show)
    end
  end
end
if rawget(UIFrameManager, "RegisterFrameForFrameType") == nil then
  function UIFrameManager:RegisterFrameForFrameType(frame, frameType)
    __wow_ui_frame_manager_ensure_state()
    if __wow_ui_frame_manager_registered_frames[frame] then
      return
    end
    local frames = __wow_ui_frame_manager_registered_frame_type_to_frames[frameType]
    if frames == nil then
      frames = {}
      __wow_ui_frame_manager_registered_frame_type_to_frames[frameType] = frames
    end
    frames[frame] = true
    __wow_ui_frame_manager_registered_frames[frame] = true
    frame:UpdateFrameState(C_FrameManager.GetFrameVisibilityState(frameType))
  end
end
if rawget(UIFrameManager_ManagedFrameMixin, "OnLoad") == nil then
  function UIFrameManager_ManagedFrameMixin:OnLoad()
    UIFrameManager:RegisterFrameForFrameType(self, self.frameType)
  end
end
if rawget(UIFrameManager_ManagedFrameMixin, "UpdateFrameState") == nil then
  function UIFrameManager_ManagedFrameMixin:UpdateFrameState(show)
    self:SetShown(show)
  end
end
if C_ProfSpecs == nil then
  C_ProfSpecs = __wow_namespace()
end
if rawget(C_ProfSpecs, "ShouldShowSpecTab") == nil then
  function C_ProfSpecs.ShouldShowSpecTab() return true end
end
local __wow_reincarnation_state = {
  active = false,
  character = nil,
}
C_Reincarnation = __wow_merge_namespace(C_Reincarnation, {
  _state = __wow_reincarnation_state,
})
if rawget(C_Reincarnation, "IsReincarnating") == nil then
  function C_Reincarnation.IsReincarnating()
    return C_Reincarnation._state.active == true
  end
end
if rawget(C_Reincarnation, "GetReincarnatingCharacter") == nil then
  function C_Reincarnation.GetReincarnatingCharacter()
    return C_Reincarnation._state.character
  end
end
if rawget(C_Reincarnation, "StartReincarnation") == nil then
  function C_Reincarnation.StartReincarnation(character)
    if C_Reincarnation._state.active then
      return false
    end
    if character ~= nil and type(character) ~= "table" then
      return false
    end
    local guid = character and tostring(character.guid or "") or "reincarnation-guid"
    local name = character and tostring(character.name or "") or "Reincarnating Character"
    C_Reincarnation._state.active = true
    C_Reincarnation._state.character = {
      guid = guid,
      name = name,
    }
    return true
  end
end
if rawget(C_Reincarnation, "StopReincarnation") == nil then
  function C_Reincarnation.StopReincarnation()
    if not C_Reincarnation._state.active then
      return false
    end
    C_Reincarnation._state.active = false
    C_Reincarnation._state.character = nil
    return true
  end
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
        description = "",
        rootIconID = 0,
        highlights = {},
      }
    end
    return nil
  end
end
if rawget(C_ProfSpecs, "GetSpecTabInfo") == nil then
  function C_ProfSpecs.GetSpecTabInfo()
    return {
      enabled = false,
      errorReason = "",
    }
  end
end
if rawget(C_ProfSpecs, "GetCurrencyInfoForSkillLine") == nil then
  function C_ProfSpecs.GetCurrencyInfoForSkillLine(skillLineID)
    if tonumber(skillLineID) == 164 then
      return {
        numAvailable = 0,
        currencyName = "",
      }
    end
    return nil
  end
end
if rawget(C_ProfSpecs, "SkillLineHasSpecialization") == nil then
  function C_ProfSpecs.SkillLineHasSpecialization(_skillLineID)
    return false
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
      local inventoryID = C_Container.ContainerIDToInventoryID and C_Container.ContainerIDToInventoryID(bagID)
      local portraitTexture = inventoryID and GetInventoryItemTexture("player", inventoryID)
      texture:SetTexture(portraitTexture)
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

local function __wow_make_transmog_location(slotName, slotID, transmogType, modification)
  local location = {
    slotName = tostring(slotName or ""),
    slotID = tonumber(slotID) or 0,
    transmogType = tonumber(transmogType) or 0,
    modification = tonumber(modification) or 0,
  }

  function location:IsAppearance()
    return true
  end

  function location:IsIllusion()
    return false
  end

  function location:IsEitherHand()
    return self.slotName == "MAINHANDSLOT" or self.slotName == "SECONDHANDSLOT"
  end

  function location:IsSecondary()
    return self.slotName == "SECONDHANDSLOT"
  end

  function location:IsMainHand()
    return self.slotName == "MAINHANDSLOT"
  end

  function location:GetSlotName()
    return self.slotName
  end

  function location:IsEqual(other)
    return type(other) == "table"
      and self.slotName == other.slotName
      and self.slotID == other.slotID
      and self.transmogType == other.transmogType
      and self.modification == other.modification
  end

  return location
end

TransmogUtil = TransmogUtil or __wow_namespace()
if rawget(TransmogUtil, "GetTransmogLocation") == nil then
  function TransmogUtil.GetTransmogLocation(slotName, transmogType, modification)
    return __wow_make_transmog_location(slotName, 0, transmogType, modification)
  end
end
if rawget(TransmogUtil, "CreateTransmogLocation") == nil then
  function TransmogUtil.CreateTransmogLocation(slotID, transmogType, modification)
    return __wow_make_transmog_location("", slotID, transmogType, modification)
  end
end
if rawget(TransmogUtil, "GetBestItemModifiedAppearanceID") == nil then
  function TransmogUtil.GetBestItemModifiedAppearanceID(_itemID)
    return nil
  end
end

C_TransmogSets = C_TransmogSets or __wow_namespace()
if rawget(C_TransmogSets, "GetBaseSetID") == nil then
  function C_TransmogSets.GetBaseSetID(_setID)
    return 0
  end
end
if rawget(C_TransmogSets, "GetVariantSets") == nil then
  function C_TransmogSets.GetVariantSets(_setID)
    return {}
  end
end
if rawget(C_TransmogSets, "GetSetInfo") == nil then
  function C_TransmogSets.GetSetInfo(_setID)
    return {
      setID = 0,
      name = "",
      collected = false,
    }
  end
end
if rawget(C_TransmogSets, "GetSetPrimaryAppearances") == nil then
  function C_TransmogSets.GetSetPrimaryAppearances(_setID)
    return {}
  end
end
if rawget(C_TransmogSets, "GetBaseSets") == nil then
  function C_TransmogSets.GetBaseSets()
    return {}
  end
end
if rawget(C_TransmogSets, "GetAllSets") == nil then
  function C_TransmogSets.GetAllSets()
    return {}
  end
end
if rawget(C_TransmogSets, "GetUsableSets") == nil then
  function C_TransmogSets.GetUsableSets()
    return {}
  end
end
if rawget(C_TransmogSets, "HasAvailableSets") == nil then
  function C_TransmogSets.HasAvailableSets()
    return false
  end
end
if rawget(C_TransmogSets, "IsBaseSetCollected") == nil then
  function C_TransmogSets.IsBaseSetCollected(_setID)
    return false
  end
end
if rawget(C_TransmogSets, "GetSourcesForSlot") == nil then
  function C_TransmogSets.GetSourcesForSlot(_setID, _slotID)
    return {}
  end
end
if rawget(C_TransmogSets, "GetAllSetAppearancesByID") == nil then
  function C_TransmogSets.GetAllSetAppearancesByID(_setID)
    return {}
  end
end

C_MountJournal = __wow_merge_namespace(C_MountJournal, {
  IsUsingDefaultFilters = function() return true end,
  SetDefaultFilters = function() end,
  ClearRecentFanfares = function() end,
  GetDynamicFlightModeSpellID = function() return 0 end,
  GetMountEquipmentUnlockLevel = function() return 0 end,
  IsDragonridingUnlocked = function() return false end,
  GetNumDisplayedMounts = function() return 0 end,
  GetDisplayedMountID = function(_index) return nil end,
  GetNumMounts = function() return 0 end,
  GetMountIDs = function() return {} end,
  GetMountInfoByID = function(_mountID) return nil, nil, nil, false end,
  GetMountInfoExtraByID = function(_mountID) return nil, nil, nil, nil, nil, nil, nil, nil, nil end,
  GetMountLink = function(_mountID) return nil end,
  GetMountUsabilityByID = function(_mountID) return false, false, false end,
  IsItemMountEquipment = function(_itemLocation) return false end,
  IsMountEquipmentApplied = function() return false end,
  GetIsFavorite = function(_mountID) return false end,
  SetIsFavorite = function(_mountID, _favorite) end,
  NeedsFanfare = function(_mountID) return false end,
  ClearFanfare = function(_mountID) end,
  Dismiss = function() end,
  ApplyMountEquipment = function(_mountID) end,
  AreMountEquipmentEffectsSuppressed = function() return false end,
  GetAppliedMountEquipmentID = function() return 0 end,
  Pickup = function(_mountID) end,
  PickupDynamicFlightMode = function() end,
  SwapDynamicFlightMode = function() end,
  SetSearch = function(_text) end,
  SetCollectedFilterSetting = function(_filter, _value) return true end,
  GetCollectedFilterSetting = function(_filter) return true end,
  IsTypeChecked = function(_filterIndex) return true end,
  SetTypeFilter = function(_filterIndex, _value) end,
  IsValidTypeFilter = function(_filterIndex) return true end,
  IsSourceChecked = function(_filterIndex) return true end,
  SetSourceFilter = function(_filterIndex, _value) end,
  SetAllSourceFilters = function(_value) end,
  IsValidSourceFilter = function(_filterIndex) return true end,
})

C_PetJournal = __wow_merge_namespace(C_PetJournal, {
  IsUsingDefaultFilters = function() return true end,
  SetDefaultFilters = function() end,
  ClearRecentFanfares = function() end,
  GetSummonBattlePetCooldown = function() return 0, 0, false end,
  PetNeedsFanfare = function() return false end,
  GetNumPets = function() return 0, 0 end,
  GetPetInfoByIndex = function(_index) return nil, nil, false end,
  GetPetInfoByPetID = function(_petID) return nil end,
  GetPetInfoBySpeciesID = function(_speciesID) return nil, nil, nil end,
  GetPetStats = function(_petID) return 0, 0, 0, 0, 0 end,
  GetPetSummonInfo = function(_petID) return false, nil, nil end,
  GetPetSortParameter = function() return LE_SORT_BY_NAME end,
  SetPetSortParameter = function(_parameter) end,
  IsFilterChecked = function(_filterIndex) return true end,
  SetFilterChecked = function(_filterIndex, _value) end,
  IsPetTypeChecked = function(_filterIndex) return true end,
  SetPetTypeFilter = function(_filterIndex, _value) end,
  IsPetSourceChecked = function(_filterIndex) return true end,
  SetPetSourceChecked = function(_filterIndex, _value) end,
  SetAllPetTypesChecked = function(_value) end,
  SetAllPetSourcesChecked = function(_value) end,
  SetSearchFilter = function(_text) end,
  IsFindBattleEnabled = function() return false end,
  IsJournalUnlocked = function() return true end,
  SetCustomName = function(_petID, _name) end,
  CagePetByID = function(_petID) end,
  ReleasePetByID = function(_petID) end,
  ClearFanfare = function(_petID) end,
  ClearHoveredBattlePet = function() end,
  GetNumPetSources = function() return 0 end,
  GetBattlePetLink = function(_speciesID, _level, _breed, _quality, _maxHealth, _power, _speed) return nil end,
  HasFavoritePets = function() return false end,
  PetIsFavorite = function(_petID) return false end,
  PetCanBeReleased = function(_petID) return false end,
  PetIsHurt = function(_petID) return false end,
  PetIsLockedForConvert = function(_petID) return false end,
  PetIsRevoked = function(_petID) return false end,
  PetIsSlotted = function(_petID) return false end,
  PetIsSummonable = function(_petID) return false end,
  PetIsTradable = function(_petID) return false end,
  PickupPet = function(_petID) end,
  PickupSummonRandomPet = function() end,
  SetAbility = function(_slot, _abilityID) end,
  SetFavorite = function(_petID, _favorite) end,
  SummonPetByGUID = function(_petID) end,
  SummonRandomPet = function() end,
})

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

C_LootJournal = __wow_merge_namespace(C_LootJournal, {
  GetItemSets = function(_classID, _specID)
    return {}
  end,
  GetItemSetItems = function(_setID)
    return {}
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

local __wow_perks_activities_state = rawget(_G, "__wow_perks_activities_state")
if type(__wow_perks_activities_state) ~= "table" then
  __wow_perks_activities_state = {
    trackedIDs = {},
    removeCount = 0,
    lastRemovedID = nil,
    activityInfoByID = {},
    chatLinkByID = {},
    activitiesInfo = nil,
    allTags = nil,
    pendingCompletion = nil,
  }
  rawset(_G, "__wow_perks_activities_state", __wow_perks_activities_state)
end

C_PerksActivities = __wow_merge_namespace(C_PerksActivities, {
  _state = __wow_perks_activities_state,
  AddTrackedPerksActivity = function(id)
    local state = C_PerksActivities._state
    local trackedIDs = state.trackedIDs
    if type(trackedIDs) ~= "table" then
      trackedIDs = {}
      state.trackedIDs = trackedIDs
    end
    table.insert(trackedIDs, tonumber(id) or id)
  end,
  ClearPerksActivitiesPendingCompletion = function()
    local state = C_PerksActivities._state
    state.pendingCompletion = { pendingIDs = {} }
  end,
  GetAllPerksActivityTags = function()
    local state = C_PerksActivities._state
    if type(state.allTags) == "table" then
      return state.allTags
    end
    return { tagName = {} }
  end,
  GetPerksActivitiesInfo = function()
    local state = C_PerksActivities._state
    if type(state.activitiesInfo) == "table" then
      return state.activitiesInfo
    end
    return {
      activePerksMonth = 1,
      displayMonthName = "",
      secondsRemaining = 0,
      activities = {},
      thresholds = {},
    }
  end,
  GetPerksActivitiesPendingCompletion = function()
    local state = C_PerksActivities._state
    if type(state.pendingCompletion) == "table" then
      return state.pendingCompletion
    end
    return { pendingIDs = {} }
  end,
  GetPerksActivityChatLink = function(_id)
    local state = C_PerksActivities._state
    local info = state.chatLinkByID and state.chatLinkByID[tonumber(_id) or _id]
    return info or ""
  end,
  GetPerksActivityInfo = function(_id)
    local state = C_PerksActivities._state
    return state.activityInfoByID and state.activityInfoByID[tonumber(_id) or _id] or nil
  end,
  GetPerksUIThemePrefix = function()
    return ""
  end,
  GetTrackedPerksActivities = function()
    local state = C_PerksActivities._state
    return { trackedIDs = state.trackedIDs or {} }
  end,
  RemoveTrackedPerksActivity = function(id)
    local state = C_PerksActivities._state
    local trackedIDs = state.trackedIDs
    if type(trackedIDs) ~= "table" then
      trackedIDs = {}
      state.trackedIDs = trackedIDs
    end

    local targetID = tonumber(id) or id
    for index = #trackedIDs, 1, -1 do
      if tonumber(trackedIDs[index]) == targetID then
        table.remove(trackedIDs, index)
        state.removeCount = (tonumber(state.removeCount) or 0) + 1
        state.lastRemovedID = targetID
        return true
      end
    end
    return false
  end,
})

local __wow_store_glue_state = rawget(_G, "__wow_store_glue_state")
if type(__wow_store_glue_state) ~= "table" then
  __wow_store_glue_state = {
    disconnectOnLogout = false,
    vasProductReady = false,
    purchaseStateByGuid = {},
    requestedQueueGuids = {},
    requestCharacterQueueTimeCount = 0,
    updateVASPurchaseStatesCount = 0,
    lastRequestedQueueGuid = nil,
  }
  rawset(_G, "__wow_store_glue_state", __wow_store_glue_state)
end

local function __wow_store_glue_state_table()
  local state = __wow_store_glue_state
  if type(state.purchaseStateByGuid) ~= "table" then
    state.purchaseStateByGuid = {}
  end
  if type(state.requestedQueueGuids) ~= "table" then
    state.requestedQueueGuids = {}
  end
  return state
end

C_StoreGlue = __wow_merge_namespace(C_StoreGlue, {
  _state = __wow_store_glue_state_table(),
  GetDisconnectOnLogout = function()
    return __wow_store_glue_state_table().disconnectOnLogout == true
  end,
  GetVASProductReady = function()
    return __wow_store_glue_state_table().vasProductReady == true
  end,
  GetVASPurchaseStateInfo = function(guid)
    local state = __wow_store_glue_state_table()
    local record = state.purchaseStateByGuid[tostring(guid)] or state.purchaseStateByGuid[guid]
    if type(record) ~= "table" then
      return 0, 0, nil
    end
    return tonumber(record.purchaseState) or 0, tonumber(record.productID) or 0, record.result
  end,
  RequestCharacterQueueTime = function(guid)
    local state = __wow_store_glue_state_table()
    table.insert(state.requestedQueueGuids, guid)
    state.requestCharacterQueueTimeCount = (tonumber(state.requestCharacterQueueTimeCount) or 0) + 1
    state.lastRequestedQueueGuid = guid
    return true
  end,
  UpdateVASPurchaseStates = function()
    local state = __wow_store_glue_state_table()
    state.updateVASPurchaseStatesCount = (tonumber(state.updateVASPurchaseStatesCount) or 0) + 1
    return true
  end,
})

local __wow_video_options_state = rawget(_G, "__wow_video_options_state")
if type(__wow_video_options_state) ~= "table" then
  __wow_video_options_state = {
    defaultGameWindowSize = { x = 1920, y = 1080 },
    currentGameWindowSize = { x = 1920, y = 1080 },
    availableGameWindowSizes = {},
    gxAdapterInfo = {},
    setGameWindowSizeCount = 0,
    lastSetWindowSize = nil,
  }
  rawset(_G, "__wow_video_options_state", __wow_video_options_state)
end

local function __wow_video_options_state_table()
  local state = __wow_video_options_state
  if type(state.defaultGameWindowSize) ~= "table" then
    state.defaultGameWindowSize = { x = 1920, y = 1080 }
  end
  if type(state.currentGameWindowSize) ~= "table" then
    state.currentGameWindowSize = __wow_copy_table(state.defaultGameWindowSize)
  end
  if type(state.availableGameWindowSizes) ~= "table" then
    state.availableGameWindowSizes = {}
  end
  if type(state.gxAdapterInfo) ~= "table" then
    state.gxAdapterInfo = {}
  end
  return state
end

local function __wow_copy_window_size(size)
  if type(size) ~= "table" then
    return { x = 0, y = 0 }
  end
  return {
    x = tonumber(size.x) or 0,
    y = tonumber(size.y) or 0,
  }
end

local function __wow_copy_window_sizes(sizes)
  local copied = {}
  if type(sizes) ~= "table" then
    return copied
  end
  for index, size in ipairs(sizes) do
    copied[index] = __wow_copy_window_size(size)
  end
  return copied
end

local function __wow_copy_adapter_info(adapters)
  local copied = {}
  if type(adapters) ~= "table" then
    return copied
  end
  for index, adapter in ipairs(adapters) do
    copied[index] = __wow_copy_table(adapter)
  end
  return copied
end

C_VideoOptions = __wow_merge_namespace(C_VideoOptions, {
  _state = __wow_video_options_state_table(),
  GetDefaultGameWindowSize = function()
    local state = __wow_video_options_state_table()
    return __wow_copy_window_size(state.defaultGameWindowSize)
  end,
  GetCurrentGameWindowSize = function()
    local state = __wow_video_options_state_table()
    return __wow_copy_window_size(state.currentGameWindowSize)
  end,
  GetGameWindowSizes = function()
    return __wow_copy_window_sizes(__wow_video_options_state_table().availableGameWindowSizes)
  end,
  GetGxAdapterInfo = function()
    return __wow_copy_adapter_info(__wow_video_options_state_table().gxAdapterInfo)
  end,
  IsSpellVisualDensitySystemSupported = function()
    return false
  end,
  SetGameWindowSize = function(width, height)
    local state = __wow_video_options_state_table()
    state.currentGameWindowSize = {
      x = tonumber(width) or 0,
      y = tonumber(height) or 0,
    }
    state.lastSetWindowSize = __wow_copy_window_size(state.currentGameWindowSize)
    state.setGameWindowSizeCount = (tonumber(state.setGameWindowSizeCount) or 0) + 1
    return true
  end,
})

local __wow_combat_log_namespace_state = rawget(_G, "__wow_combat_log_state")
if type(__wow_combat_log_namespace_state) ~= "table" then
  __wow_combat_log_namespace_state = {
    currentEntry = 0,
    numEntries = 0,
    retentionTime = 300,
    filteredEventsEnabled = false,
    messageLimit = 300,
    entries = {},
    currentIndex = nil,
    createdMessages = {},
  }
  rawset(_G, "__wow_combat_log_state", __wow_combat_log_namespace_state)
end

local function __wow_combat_log_state_table()
  local state = __wow_combat_log_namespace_state
  if type(state.entries) ~= "table" then
    state.entries = {}
  end
  if type(state.createdMessages) ~= "table" then
    state.createdMessages = {}
  end
  return state
end

local function __wow_combat_log_count_entries(state)
  local entries = state.entries
  if type(entries) ~= "table" then
    return tonumber(state.numEntries) or 0
  end
  return #entries
end

local function __wow_combat_log_current_entry(state)
  local entries = state.entries
  if type(entries) ~= "table" or #entries == 0 then
    return nil
  end
  local index = state.currentIndex
  if type(index) ~= "number" or index < 1 or index > #entries then
    index = #entries
  end
  return entries[index], index
end

local function __wow_combat_log_clear_entries(state)
  state.entries = {}
  state.currentIndex = nil
  state.currentEntry = 0
  state.numEntries = 0
end

local function __wow_combat_log_store_message(state, message, red, green, blue, order)
  local entry = {
    message = tostring(message or ""),
    red = tonumber(red) or 0,
    green = tonumber(green) or 0,
    blue = tonumber(blue) or 0,
    order = order,
  }
  local newest = Enum.CombatLogMessageOrder and Enum.CombatLogMessageOrder.Newest
  if order == newest then
    table.insert(state.createdMessages, 1, entry)
  else
    table.insert(state.createdMessages, entry)
  end
end

C_CombatLog = __wow_merge_namespace(C_CombatLog, {
  _state = __wow_combat_log_state_table(),
  GetEntryCount = function()
    return __wow_combat_log_count_entries(__wow_combat_log_state_table())
  end,
  GetCurrentEventInfo = function()
    local entry = __wow_combat_log_current_entry(__wow_combat_log_state_table())
    if entry == nil then
      return nil
    end
    return unpack(entry)
  end,
  ShouldShowCurrentEntry = function()
    return __wow_combat_log_count_entries(__wow_combat_log_state_table()) > 0
  end,
  GetEntryRetentionTime = function()
    return __wow_combat_log_state_table().retentionTime
  end,
  SetEntryRetentionTime = function(retentionTime)
    __wow_combat_log_state_table().retentionTime = tonumber(retentionTime) or 0
  end,
  AreFilteredEventsEnabled = function()
    return __wow_combat_log_state_table().filteredEventsEnabled == true
  end,
  SetFilteredEventsEnabled = function(enabled)
    __wow_combat_log_state_table().filteredEventsEnabled = enabled == true
  end,
  GetMessageLimit = function()
    return __wow_combat_log_state_table().messageLimit or 300
  end,
  SetMessageLimit = function(limit)
    __wow_combat_log_state_table().messageLimit = tonumber(limit) or 0
  end,
  ClearEntries = function()
    __wow_combat_log_clear_entries(__wow_combat_log_state_table())
  end,
  ApplyFilterSettings = function(_settings)
  end,
  RefilterEntries = function()
  end,
})

C_CombatLogSecure = __wow_merge_namespace(C_CombatLogSecure, {
  _state = __wow_combat_log_state_table(),
  GetEntryCount = function()
    return __wow_combat_log_count_entries(__wow_combat_log_state_table())
  end,
  GetCurrentEntryInfo = function()
    local entry = __wow_combat_log_current_entry(__wow_combat_log_state_table())
    if entry == nil then
      return nil
    end
    return unpack(entry)
  end,
  SeekToNewestEntry = function()
    local state = __wow_combat_log_state_table()
    local count = __wow_combat_log_count_entries(state)
    if count == 0 then
      return false
    end
    state.currentIndex = count
    return true
  end,
  SeekToPreviousEntry = function()
    local state = __wow_combat_log_state_table()
    local count = __wow_combat_log_count_entries(state)
    if count == 0 then
      return false
    end
    local index = state.currentIndex or count
    if index <= 1 then
      return false
    end
    state.currentIndex = index - 1
    return true
  end,
  CreateCombatLogMessage = function(message, red, green, blue, order)
    __wow_combat_log_store_message(__wow_combat_log_state_table(), message, red, green, blue, order)
    return true
  end,
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
    value = __wow_make_color(1, 1, 1, 1)
  elseif __wow_is_color_constant_key(key) then
    value = __wow_make_color(1, 1, 1, 1)
  elseif key == "PLAYER_FACTION_COLOR_HORDE" then
    value = __wow_make_color(1, 0.1, 0.1, 1)
  elseif key == "PLAYER_FACTION_COLOR_ALLIANCE" then
    value = __wow_make_color(0.2, 0.4, 1, 1)
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
if type(CreateAnchor) ~= "function" then
  function CreateAnchor(point, relativeTo, relativePoint, x, y)
    return {
      point = point,
      relativeTo = relativeTo,
      relativePoint = relativePoint or point,
      x = x or 0,
      y = y or 0,
    }
  end
end

if type(GetFinalNameFromTextureKit) ~= "function" then
  function GetFinalNameFromTextureKit(formatString, textureKit)
    if type(formatString) ~= "string" then
      return nil
    end
    if textureKit == nil or textureKit == "" then
      return (formatString:gsub("%%s_?", ""):gsub("_$", ""))
    end
    return formatString:gsub("%%s", textureKit)
  end
end

if type(SetClampedTextureRotation) ~= "function" then
  function SetClampedTextureRotation(texture, rotation)
    if texture and type(texture.SetRotation) == "function" then
      texture:SetRotation(rotation or 0)
    end
  end
end

if type(CopyValuesAsKeys) ~= "function" then
  function CopyValuesAsKeys(values)
    local result = {}
    if type(values) ~= "table" then
      return result
    end
    for _, value in pairs(values) do
      result[value] = true
    end
    return result
  end
end

if type(GetMicroIconForRole) ~= "function" then
  function GetMicroIconForRole(role)
    if type(role) ~= "string" then
      return "roleicon"
    end
    return "roleicon-" .. role:lower()
  end
end

if type(PingSystemInitializer) ~= "function" then
  function PingSystemInitializer(_category)
  end
end

if type(SecondsFormatter) ~= "table" then
  SecondsFormatter = {
    Abbreviation = { None = 0 },
    Interval = { Minutes = 60 },
  }
end

if type(SecondsFormatterMixin) ~= "table" then
  SecondsFormatterMixin = {}
end

if type(SecondsFormatterMixin.Init) ~= "function" then
  function SecondsFormatterMixin:Init(secondsPerUnit, abbreviation, displayZero)
    self.secondsPerUnit = secondsPerUnit
    self.abbreviation = abbreviation
    self.displayZero = displayZero
  end
end

if type(SecondsFormatterMixin.SetStripIntervalWhitespace) ~= "function" then
  function SecondsFormatterMixin:SetStripIntervalWhitespace(strip)
    self.stripIntervalWhitespace = strip
  end
end

if type(SecondsFormatterMixin.GetStripIntervalWhitespace) ~= "function" then
  function SecondsFormatterMixin:GetStripIntervalWhitespace()
    return self.stripIntervalWhitespace
  end
end

if type(SecondsFormatterMixin.SetConvertToLower) ~= "function" then
  function SecondsFormatterMixin:SetConvertToLower(convertToLower)
    self.convertToLower = convertToLower
  end
end

if type(SecondsFormatterMixin.SetDefaultAbbreviation) ~= "function" then
  function SecondsFormatterMixin:SetDefaultAbbreviation(defaultAbbreviation)
    self.defaultAbbreviation = defaultAbbreviation
  end
end

if type(SecondsFormatterMixin.SetApproximationSeconds) ~= "function" then
  function SecondsFormatterMixin:SetApproximationSeconds(approximationSeconds)
    self.approximationSeconds = approximationSeconds
  end
end

if type(SecondsFormatterMixin.SetCanRoundUpLastUnit) ~= "function" then
  function SecondsFormatterMixin:SetCanRoundUpLastUnit(roundUpLastUnit)
    self.roundUpLastUnit = roundUpLastUnit
  end
end

if type(SecondsFormatterMixin.SetCanRoundUpIntervals) ~= "function" then
  function SecondsFormatterMixin:SetCanRoundUpIntervals(roundUpIntervals)
    self.roundUpIntervals = roundUpIntervals
  end
end

if type(SecondsFormatterMixin.GetDesiredUnitCount) ~= "function" then
  function SecondsFormatterMixin:SetDesiredUnitCount(unitCount)
    self.unitCount = unitCount
  end

  function SecondsFormatterMixin:GetDesiredUnitCount(_seconds)
    return 1
  end
end

if type(SecondsFormatterMixin.SetMinInterval) ~= "function" then
  function SecondsFormatterMixin:SetMinInterval(interval)
    self.minInterval = interval
  end
end

if type(SecondsFormatterMixin.GetMinInterval) ~= "function" then
  function SecondsFormatterMixin:GetMinInterval(_seconds)
    return SecondsFormatter.Interval.Minutes
  end
end

if type(SecondsFormatterMixin.Format) ~= "function" then
  function SecondsFormatterMixin:Format(seconds)
    return tostring(seconds or 0)
  end
end

if type(CallbackRegistryMixin) ~= "table" then
  CallbackRegistryMixin = {}
end

if type(CallbackRegistryMixin.OnLoad) ~= "function" then
  function CallbackRegistryMixin:OnLoad()
    self.__callbacks = self.__callbacks or {}
    self.Event = self.Event or {}
  end
end

if type(CallbackRegistryMixin.SetUndefinedEventsAllowed) ~= "function" then
  function CallbackRegistryMixin:SetUndefinedEventsAllowed(allowed)
    self.__allowUndefinedEvents = not not allowed
  end
end

if type(CallbackRegistryMixin.GenerateCallbackEvents) ~= "function" then
  function CallbackRegistryMixin:GenerateCallbackEvents(events)
    self:OnLoad()
    if type(events) ~= "table" then
      return
    end
    for _, eventName in ipairs(events) do
      self.Event[eventName] = eventName
    end
  end
end

if type(CallbackRegistryMixin.RegisterCallback) ~= "function" then
  function CallbackRegistryMixin:RegisterCallback(eventName, callback, owner)
    self:OnLoad()
    if type(callback) ~= "function" then
      return nil
    end
    local callbacks = self.__callbacks[eventName]
    if callbacks == nil then
      callbacks = {}
      self.__callbacks[eventName] = callbacks
    end
    local handle = { callback = callback, owner = owner }
    callbacks[#callbacks + 1] = handle
    return handle
  end
end

if type(CallbackRegistryMixin.UnregisterCallback) ~= "function" then
  function CallbackRegistryMixin:UnregisterCallback(eventName, ownerOrHandle)
    local callbacks = self.__callbacks and self.__callbacks[eventName]
    if callbacks == nil then
      return
    end
    for index = #callbacks, 1, -1 do
      local entry = callbacks[index]
      if entry == ownerOrHandle or entry.owner == ownerOrHandle then
        table.remove(callbacks, index)
      end
    end
  end
end

if type(CallbackRegistryMixin.TriggerEvent) ~= "function" then
  function CallbackRegistryMixin:TriggerEvent(eventName, ...)
    local callbacks = self.__callbacks and self.__callbacks[eventName]
    if callbacks == nil then
      return
    end
    for _, entry in ipairs(callbacks) do
      if entry.owner ~= nil then
        entry.callback(entry.owner, ...)
      else
        entry.callback(...)
      end
    end
  end
end

if type(EventRegistry) ~= "table" then
  EventRegistry = CreateFromMixins(CallbackRegistryMixin)
  EventRegistry:OnLoad()
end

if type(EventRegistry.RegisterFrameEventAndCallback) ~= "function" then
  function EventRegistry:RegisterFrameEventAndCallback(eventName, callback, owner)
    return self:RegisterCallback(eventName, callback, owner)
  end
end

if type(CVarCallbackRegistry) ~= "table" then
  CVarCallbackRegistry = CreateFromMixins(CallbackRegistryMixin)
  CVarCallbackRegistry:OnLoad()
end

if type(CVarCallbackRegistry.SetCVarCachable) ~= "function" then
  function CVarCallbackRegistry:SetCVarCachable(name)
    self.__cvars = self.__cvars or {}
    self.__cvars[name] = true
  end
end

if type(ProxyConvertableMixin.Init) ~= "function" then
  function ProxyConvertableMixin:Init(proxy, proxies, permitOverwrite)
    self.proxy = proxy or self
    if proxies and type(proxies.AddProxy) == "function" then
      proxies:AddProxy(self, permitOverwrite)
    end
    self.__proxy_tags = self.__proxy_tags or {}
    return self.__proxy_tags
  end
end

if type(ProxyConvertableMixin.ToProxy) ~= "function" then
  function ProxyConvertableMixin:ToProxy()
    return self.proxy or self
  end
end

if type(ProxyUtil.CreateProxyDirectory) ~= "function"
  or type(ProxyUtil.CreateProxyDirectory().AddProxy) ~= "function"
then
  function ProxyUtil.CreateProxyDirectory()
    local proxies = {
      __private_by_public = setmetatable({}, { __mode = "k" }),
      __public_by_private = setmetatable({}, { __mode = "k" }),
    }

    function proxies:AddProxy(object, _permitOverwrite)
      local public = object and type(object.ToProxy) == "function" and object:ToProxy() or object
      if public ~= nil then
        self.__private_by_public[public] = object
        self.__public_by_private[object] = public
      end
    end

    function proxies:RemoveProxy(public)
      local private = self.__private_by_public[public]
      self.__private_by_public[public] = nil
      if private ~= nil then
        self.__public_by_private[private] = nil
      end
    end

    function proxies:ToPrivate(public)
      return self.__private_by_public[public] or public
    end

    function proxies:ToPublic(private)
      return self.__public_by_private[private] or private
    end

    return proxies
  end
end

if type(GetAppropriateTopLevelParent) ~= "function" then
  __wow_root_ui_parent = rawget(_G, "UIParent")
  __wow_alternate_top_level_parent = nil

  function SetAlternateTopLevelParent(parent)
    __wow_alternate_top_level_parent = parent
    if type(EventRegistry) == "table" and type(EventRegistry.TriggerEvent) == "function" then
      EventRegistry:TriggerEvent("UI.AlternateTopLevelParentChanged", parent)
    end
  end

  function ClearAlternateTopLevelParent()
    __wow_alternate_top_level_parent = nil
    if type(EventRegistry) == "table" and type(EventRegistry.TriggerEvent) == "function" then
      EventRegistry:TriggerEvent("UI.AlternateTopLevelParentChanged")
    end
  end

  function GetAppropriateTopLevelParent(optionalExcludedParent)
    if __wow_alternate_top_level_parent
      and type(__wow_alternate_top_level_parent.IsShown) == "function"
      and __wow_alternate_top_level_parent:IsShown()
      and (not optionalExcludedParent or __wow_alternate_top_level_parent ~= optionalExcludedParent)
    then
      return __wow_alternate_top_level_parent
    end

    if __wow_root_ui_parent ~= nil and __wow_root_ui_parent ~= optionalExcludedParent then
      return __wow_root_ui_parent
    end

    return UIParent or GlueParent
  end

  function SetAppropriateTopLevelParent(frame)
    local parent = GetAppropriateTopLevelParent()
    if frame and parent and type(frame.SetParent) == "function" then
      frame:SetParent(parent)
    end
  end
end

if type(GetAppropriateTooltip) ~= "function" then
  function GetAppropriateTooltip()
    return UIParent and GameTooltip or GlueTooltip
  end
end

if type(BaseNineSliceDialogMixin) ~= "table" then
  BaseNineSliceDialogMixin = {}
end

if type(BaseNineSliceDialogMixin.OnShow) ~= "function" then
  function BaseNineSliceDialogMixin:OnShow()
  end
end

if type(BaseNineSliceDialogMixin.OnCloseClick) ~= "function" then
  function BaseNineSliceDialogMixin:OnCloseClick()
    if type(self.Hide) == "function" then
      self:Hide()
    end
  end
end

if type(CallbackRegistrantMixin) ~= "table" then
  CallbackRegistrantMixin = {}
end

if type(CallbackRegistrantMixin.AddEventMethodInternal) ~= "function" then
  function CallbackRegistrantMixin:AddEventMethodInternal(handlersTable, callbackRegistry, event, handlerMethod)
    local info = self:CreateEventRegistrationInfo(callbackRegistry, event, handlerMethod)
    table.insert(handlersTable, info)
    return info
  end
end

if type(CallbackRegistrantMixin.GetDynamicCallbackRegistrantHandlers) ~= "function" then
  function CallbackRegistrantMixin:GetDynamicCallbackRegistrantHandlers()
    self.callbackRegistrantHandlers = self.callbackRegistrantHandlers or {}
    return self.callbackRegistrantHandlers
  end
end

if type(CallbackRegistrantMixin.GetStaticCallbackRegistrantHandlers) ~= "function" then
  function CallbackRegistrantMixin:GetStaticCallbackRegistrantHandlers()
    self.staticCallbackRegistrantHandlers = self.staticCallbackRegistrantHandlers or {}
    return self.staticCallbackRegistrantHandlers
  end
end

if type(CallbackRegistrantMixin.CreateEventRegistrationInfo) ~= "function" then
  function CallbackRegistrantMixin:CreateEventRegistrationInfo(callbackRegistry, event, handlerMethod)
    return {
      callbackRegistry = callbackRegistry,
      event = event,
      handlerMethod = handlerMethod,
      registered = false,
    }
  end
end

if type(CallbackRegistrantMixin.RegisterFromRegistrationInfo) ~= "function" then
  function CallbackRegistrantMixin:RegisterFromRegistrationInfo(info)
    if info.registered then
      return
    end
    if type(info.callbackRegistry) ~= "table" or type(info.callbackRegistry.RegisterCallback) ~= "function" then
      return
    end
    info.callbackRegistry:RegisterCallback(info.event, info.handlerMethod, self)
    info.registered = true
  end
end

if type(CallbackRegistrantMixin.UnregisterFromRegistrationInfo) ~= "function" then
  function CallbackRegistrantMixin:UnregisterFromRegistrationInfo(info)
    if not info.registered then
      return
    end
    if type(info.callbackRegistry) == "table" and type(info.callbackRegistry.UnregisterCallback) == "function" then
      info.callbackRegistry:UnregisterCallback(info.event, self)
    end
    info.registered = false
  end
end

if type(CallbackRegistrantMixin.UnregisterAllInternal) ~= "function" then
  function CallbackRegistrantMixin:UnregisterAllInternal(handlersTable)
    for _, info in ipairs(handlersTable) do
      self:UnregisterFromRegistrationInfo(info)
    end
  end
end

if type(CallbackRegistrantMixin.AddStaticEventMethod) ~= "function" then
  function CallbackRegistrantMixin:AddStaticEventMethod(callbackRegistry, event, handlerMethod)
    local info = self:AddEventMethodInternal(self:GetStaticCallbackRegistrantHandlers(), callbackRegistry, event, handlerMethod)
    self:RegisterFromRegistrationInfo(info)
    return info
  end
end

if type(CallbackRegistrantMixin.AddDynamicEventMethod) ~= "function" then
  function CallbackRegistrantMixin:AddDynamicEventMethod(callbackRegistry, event, handlerMethod)
    local info = self:AddEventMethodInternal(self:GetDynamicCallbackRegistrantHandlers(), callbackRegistry, event, handlerMethod)
    if type(self.IsShown) == "function" and self:IsShown() then
      self:RegisterFromRegistrationInfo(info)
    end
    return info
  end
end

if type(CallbackRegistrantMixin.RemoveStaticEventMethod) ~= "function" then
  function CallbackRegistrantMixin:RemoveStaticEventMethod(callbackRegistry, event, _handlerMethod)
    local handlers = self:GetStaticCallbackRegistrantHandlers()
    for index, info in ipairs(handlers) do
      if info.callbackRegistry == callbackRegistry and info.event == event then
        self:UnregisterFromRegistrationInfo(info)
        table.remove(handlers, index)
        break
      end
    end
  end
end

if type(CallbackRegistrantMixin.UnregisterAllEventMethods) ~= "function" then
  function CallbackRegistrantMixin:UnregisterAllEventMethods()
    self:UnregisterAllInternal(self:GetDynamicCallbackRegistrantHandlers())
    self:UnregisterAllInternal(self:GetStaticCallbackRegistrantHandlers())
  end
end

if type(CallbackRegistrantMixin.OnShow) ~= "function" then
  function CallbackRegistrantMixin:OnShow()
    for _, info in ipairs(self:GetDynamicCallbackRegistrantHandlers()) do
      self:RegisterFromRegistrationInfo(info)
    end
  end
end

if type(CallbackRegistrantMixin.OnHide) ~= "function" then
  function CallbackRegistrantMixin:OnHide()
    self:UnregisterAllInternal(self:GetDynamicCallbackRegistrantHandlers())
  end
end

if type(SecondsFormatterConstants) ~= "table" then
  SecondsFormatterConstants = {
    ZeroApproximationThreshold = 0,
    ConvertToLower = true,
    DontConvertToLower = false,
    RoundUpLastUnit = true,
    DontRoundUpLastUnit = false,
    RoundUpIntervals = true,
    DontRoundUpIntervals = false,
  }
end

if type(SecondsFormatter.Abbreviation) ~= "table" then
  SecondsFormatter.Abbreviation = {}
end
SecondsFormatter.Abbreviation.None = SecondsFormatter.Abbreviation.None or 1
SecondsFormatter.Abbreviation.Truncate = SecondsFormatter.Abbreviation.Truncate or 2
SecondsFormatter.Abbreviation.OneLetter = SecondsFormatter.Abbreviation.OneLetter or 3

if type(SecondsFormatter.Interval) ~= "table" then
  SecondsFormatter.Interval = {}
end
SecondsFormatter.Interval.Seconds = SecondsFormatter.Interval.Seconds or 1
SecondsFormatter.Interval.Minutes = SecondsFormatter.Interval.Minutes or 2
SecondsFormatter.Interval.Hours = SecondsFormatter.Interval.Hours or 3
SecondsFormatter.Interval.Days = SecondsFormatter.Interval.Days or 4

if type(SecondsFormatterMixin.GetDefaultAbbreviation) ~= "function" then
  function SecondsFormatterMixin:GetDefaultAbbreviation()
    return self.defaultAbbreviation or SecondsFormatter.Abbreviation.None
  end
end

if type(SecondsFormatterMixin.GetApproximationSeconds) ~= "function" then
  function SecondsFormatterMixin:GetApproximationSeconds()
    return self.approximationSeconds or 0
  end
end

if type(SecondsFormatterMixin.CanRoundUpLastUnit) ~= "function" then
  function SecondsFormatterMixin:CanRoundUpLastUnit()
    return not not self.roundUpLastUnit
  end
end

if type(SecondsFormatterMixin.CanRoundUpIntervals) ~= "function" then
  function SecondsFormatterMixin:CanRoundUpIntervals()
    return not not self.roundUpIntervals
  end
end

if type(SecondsFormatterMixin.GetMinInterval) ~= "function" then
  function SecondsFormatterMixin:GetMinInterval(_seconds)
    return self.minInterval or SecondsFormatter.Interval.Seconds
  end
end

if type(rawget(_G, "Settings")) == "table" then
  __wow_prepare_global_assignment("Settings", rawget(_G, "Settings"))
end

SOUNDKIT = SOUNDKIT or {}
if SOUNDKIT.CATALOG_SHOP_SELECT_NAV_MENU == nil then
  SOUNDKIT.CATALOG_SHOP_SELECT_NAV_MENU = 303824
end
if SOUNDKIT.CATALOG_SHOP_SELECT_GENERIC_UI_BUTTON == nil then
  SOUNDKIT.CATALOG_SHOP_SELECT_GENERIC_UI_BUTTON = 303826
end

if GetItemLevelColor == nil then
  function GetItemLevelColor()
    return 1, 1, 1
  end
end

if ClearCursorHoveredItem == nil then
  function ClearCursorHoveredItem()
    return nil
  end
end

if SetCursorHoveredItem == nil then
  function SetCursorHoveredItem(_itemLocation)
    return nil
  end
end

if SetCursorHoveredItemTradeItem == nil then
  function SetCursorHoveredItemTradeItem(_enabled)
    return nil
  end
end

if UnitInSubgroup == nil then
  function UnitInSubgroup(unit)
    if unit == nil or unit == "player" then
      return false
    end
    return type(UnitInParty) == "function" and UnitInParty(unit) or false
  end
end

if GetNumGuildPerks == nil then
  function GetNumGuildPerks()
    return 0
  end
end

if RequestGuildRewards == nil then
  function RequestGuildRewards()
    return nil
  end
end

if GetGuildRenameRequired == nil then
  function GetGuildRenameRequired()
    return false
  end
end

if GetAvailableBandwidth == nil then
  function GetAvailableBandwidth()
    local bandwidthIn, bandwidthOut = GetNetStats()
    return math.max(tonumber(bandwidthIn) or 0, tonumber(bandwidthOut) or 0)
  end
end

C_Club = C_Club or __wow_namespace()
if rawget(C_Club, "GetClubStreamNotificationSettings") == nil then
  function C_Club.GetClubStreamNotificationSettings(_clubId)
    return {}
  end
end

C_WarbandScene = C_WarbandScene or __wow_namespace()
if rawget(C_WarbandScene, "SearchWarbandSceneEntries") == nil then
  function C_WarbandScene.SearchWarbandSceneEntries(_searchParams)
    return {}
  end
end

C_TransmogOutfitInfo = C_TransmogOutfitInfo or __wow_namespace()
if rawget(C_TransmogOutfitInfo, "GetTransmogOutfitSlotFromInventorySlot") == nil then
  function C_TransmogOutfitInfo.GetTransmogOutfitSlotFromInventorySlot(inventorySlot)
    local slot = tonumber(inventorySlot)
    if not slot or slot < 0 then
      return nil
    end
    return slot
  end
end
if rawget(C_TransmogOutfitInfo, "GetLinkedSlotInfo") == nil then
  function C_TransmogOutfitInfo.GetLinkedSlotInfo(_slot)
    return nil
  end
end
if rawget(C_TransmogOutfitInfo, "GetAllSlotLocationInfo") == nil then
  local __wow_inventory_slot_id_by_name = {
    HEADSLOT = 1,
    SHOULDERSLOT = 3,
    SHIRTSLOT = 4,
    CHESTSLOT = 5,
    WAISTSLOT = 6,
    LEGSSLOT = 7,
    FEETSLOT = 8,
    WRISTSLOT = 9,
    HANDSSLOT = 10,
    BACKSLOT = 15,
    MAINHANDSLOT = 16,
    SECONDARYHANDSLOT = 17,
    TABARDSLOT = 19,
  }

  local function __wow_make_outfit_slot_info(slotName, transmogType, collectionType, isSecondary)
    local slotID = __wow_inventory_slot_id_by_name[slotName] or 0
    if type(GetInventorySlotInfo) == "function" then
      local ok, rawSlotID = pcall(GetInventorySlotInfo, slotName)
      if ok then
        slotID = tonumber(rawSlotID) or slotID
      end
    end

    return {
      slot = math.max(slotID - 1, 0),
      type = transmogType,
      collectionType = collectionType,
      slotName = slotName,
      isSecondary = isSecondary and true or false,
    }
  end

  function C_TransmogOutfitInfo.GetAllSlotLocationInfo()
    local transmogType = Enum and Enum.TransmogType or {}
    local appearanceType = transmogType.Appearance or 0
    local illusionType = transmogType.Illusion or 1

    local collectionType = Enum and Enum.TransmogCollectionType or {}
    local noneType = collectionType.None or 0
    local appearanceSlotInfo = {
      __wow_make_outfit_slot_info("HEADSLOT", appearanceType, collectionType.Head or 1, false),
      __wow_make_outfit_slot_info("SHOULDERSLOT", appearanceType, collectionType.Shoulder or 2, false),
      __wow_make_outfit_slot_info("BACKSLOT", appearanceType, collectionType.Back or 3, false),
      __wow_make_outfit_slot_info("CHESTSLOT", appearanceType, collectionType.Chest or 4, false),
      __wow_make_outfit_slot_info("SHIRTSLOT", appearanceType, collectionType.Shirt or 5, false),
      __wow_make_outfit_slot_info("TABARDSLOT", appearanceType, collectionType.Tabard or 6, false),
      __wow_make_outfit_slot_info("WRISTSLOT", appearanceType, collectionType.Wrist or 7, false),
      __wow_make_outfit_slot_info("HANDSSLOT", appearanceType, collectionType.Hands or 8, false),
      __wow_make_outfit_slot_info("WAISTSLOT", appearanceType, collectionType.Waist or 9, false),
      __wow_make_outfit_slot_info("LEGSSLOT", appearanceType, collectionType.Legs or 10, false),
      __wow_make_outfit_slot_info("FEETSLOT", appearanceType, collectionType.Feet or 11, false),
      __wow_make_outfit_slot_info("MAINHANDSLOT", appearanceType, noneType, false),
      __wow_make_outfit_slot_info("SECONDARYHANDSLOT", appearanceType, noneType, false),
      __wow_make_outfit_slot_info("SHOULDERSLOT", appearanceType, collectionType.Shoulder or 2, true),
    }

    local illusionSlotInfo = {
      __wow_make_outfit_slot_info("MAINHANDSLOT", illusionType, noneType, false),
      __wow_make_outfit_slot_info("SECONDARYHANDSLOT", illusionType, noneType, false),
    }

    return appearanceSlotInfo, illusionSlotInfo
  end
end
