-- Mists-only global stubs.
--
-- Loaded after `runtime_surface_bootstrap.lua` under `client-mists`. Every
-- entry uses `if rawget(_G, "X") == nil then ... end` so a real definition
-- from a mists Blizzard_* addon (loaded later) takes precedence.
--
-- Mists is post-Cataclysm (5.4) so it doesn't need wrath's Lua-5.0 string
-- aliases or the SetBackdrop frame proxies. The function stubs here address
-- specifically the 46 unique missing globals in mists's lua-errors baseline.

-- ─── Pre-Cata leftover globals that mists kept but retail removed ────────────

if rawget(_G, "DropCursorMoney") == nil then
  function DropCursorMoney() end
end

if rawget(_G, "FillLocalizedClassList") == nil then
  function FillLocalizedClassList(t) return t end
end

if rawget(_G, "GetActionBarPage") == nil then
  function GetActionBarPage() return 1 end
end

if rawget(_G, "GetActionBarToggles") == nil then
  function GetActionBarToggles()
    return false, false, false, false, false, false
  end
end

if rawget(_G, "GetComboPoints") == nil then
  function GetComboPoints() return 0 end
end

if rawget(_G, "GetCurrentArenaSeasonUsesTeams") == nil then
  function GetCurrentArenaSeasonUsesTeams() return false end
end

if rawget(_G, "GetQuestLogSelection") == nil then
  function GetQuestLogSelection() return 0 end
end

if rawget(_G, "GetQuestLogTitle") == nil then
  function GetQuestLogTitle(idx) return nil end
end

if rawget(_G, "GetQuestLogPortraitGiver") == nil then
  function GetQuestLogPortraitGiver() return nil end
end

if rawget(_G, "GetQuestLogPushable") == nil then
  function GetQuestLogPushable() return false end
end

if rawget(_G, "GetQuestTagInfo") == nil then
  function GetQuestTagInfo(idx) return nil end
end

if rawget(_G, "GetQuestTimers") == nil then
  function GetQuestTimers() return nil end
end

if rawget(_G, "GetRuneType") == nil then
  function GetRuneType() return 1 end
end

if rawget(_G, "GetTabardCreationCost") == nil then
  function GetTabardCreationCost() return 0 end
end

if rawget(_G, "GetRaidProfileOption") == nil then
  function GetRaidProfileOption() return nil end
end

if rawget(_G, "GuildControlGetRank") == nil then
  function GuildControlGetRank() return nil end
end

if rawget(_G, "HasExtraActionBar") == nil then
  function HasExtraActionBar() return false end
end

if rawget(_G, "HasKey") == nil then
  function HasKey() return false end
end

if rawget(_G, "HasLoadedCUFProfiles") == nil then
  function HasLoadedCUFProfiles() return false end
end

if rawget(_G, "IsCommunitiesUIDisabledByTrialAccount") == nil then
  function IsCommunitiesUIDisabledByTrialAccount() return false end
end

if rawget(_G, "IsInGlobalEnvironment") == nil then
  -- Note: real implementation returns true only when running in the addon
  -- shared environment. Returning false is the safer default — true would
  -- enable code paths that check it as a guard.
  function IsInGlobalEnvironment() return false end
end

if rawget(_G, "IsKeyRingEnabled") == nil then
  function IsKeyRingEnabled() return false end
end

if rawget(_G, "IsRaidMarkerActive") == nil then
  function IsRaidMarkerActive() return false end
end

if rawget(_G, "LFD_IsEmpowered") == nil then
  function LFD_IsEmpowered() return true end
end

-- SetGuildRosterSelection: deliberately not stubbed. A no-op stub causes
-- mists's GuildRoster UI to enter an infinite loop — it calls Set then expects
-- a subsequent Get to return the new index; with a no-op stub the index stays
-- unchanged and the calling code retries forever, hanging the bootstrap.
-- The nil reference produces a clean Lua error instead, which is better than
-- a hang. Implement with real state if a wrath/mists guild test ever needs it.

if rawget(_G, "SetSelectedSkill") == nil then
  function SetSelectedSkill() end
end

-- ─── Helpers and utilities ───────────────────────────────────────────────────

if rawget(_G, "AddLuaErrorHandler") == nil then
  function AddLuaErrorHandler() end
end

if rawget(_G, "AreHighResTexturesAvailable") == nil then
  function AreHighResTexturesAvailable() return true end
end

if rawget(_G, "CreateForbiddenFrame") == nil then
  -- Real mists creates a frame with the forbidden flag set. Returning a plain
  -- table is enough for callers that store it without exercising frame methods.
  function CreateForbiddenFrame()
    return {}
  end
end

if rawget(_G, "FCF_StripChatMsg") == nil then
  function FCF_StripChatMsg(msg) return msg end
end

if rawget(_G, "ChatFrame_ImportAllListsToHash") == nil then
  function ChatFrame_ImportAllListsToHash() end
end

if rawget(_G, "GetDisplayedAllyFrames") == nil then
  function GetDisplayedAllyFrames() return nil end
end

if rawget(_G, "SecureMixin") == nil then
  -- Real mists copies fields from mixins into the target while preserving
  -- security state. For the stub, plain shallow merge works.
  function SecureMixin(target, ...)
    for i = 1, select("#", ...) do
      local mixin = select(i, ...)
      if type(mixin) == "table" then
        for k, v in pairs(mixin) do
          target[k] = v
        end
      end
    end
    return target
  end
end

-- ─── Money frame OnLoad helpers ──────────────────────────────────────────────
-- Mists's Blizzard_FrameXML defines these but our load order may invoke the
-- XML OnLoad before the lua side registers them. Safe no-ops.

if rawget(_G, "MoneyFrame_OnLoad") == nil then
  function MoneyFrame_OnLoad(self) end
end

if rawget(_G, "SmallMoneyFrame_OnLoad") == nil then
  function SmallMoneyFrame_OnLoad(self) end
end

if rawget(_G, "MoneyInputFrame_SetCompact") == nil then
  function MoneyInputFrame_SetCompact() end
end

if rawget(_G, "MoneyInputFrame_SetOnValueChangedFunc") == nil then
  function MoneyInputFrame_SetOnValueChangedFunc() end
end

if rawget(_G, "MoneyInputFrame_SetPreviousFocus") == nil then
  function MoneyInputFrame_SetPreviousFocus() end
end

if rawget(_G, "PaperDollItemSlotButton_OnLoad") == nil then
  function PaperDollItemSlotButton_OnLoad(self) end
end

if rawget(_G, "PaperDollItemSlotButton_OnShow") == nil then
  function PaperDollItemSlotButton_OnShow(self) end
end

if rawget(_G, "UIParent_OnLoad") == nil then
  function UIParent_OnLoad(self) end
end

-- ─── Constants ───────────────────────────────────────────────────────────────

if rawget(_G, "NUM_LE_ITEM_QUALITYS") == nil then
  NUM_LE_ITEM_QUALITYS = 8
end

-- ─── DebugBarManager (mists debug overlay) ───────────────────────────────────

if rawget(_G, "DebugBarManager") == nil then
  DebugBarManager = setmetatable({}, {
    __index = function() return function() end end,
  })
end

-- ─── C_LootHistory namespace ─────────────────────────────────────────────────

if rawget(_G, "C_LootHistory") == nil then
  C_LootHistory = {
    GetItem = function() return nil end,
    GetNumItems = function() return 0 end,
    GetPlayerInfo = function() return nil end,
    GiveMasterLoot = function() end,
    SetExpiration = function() end,
    CanMasterLoot = function() return false end,
  }
end
