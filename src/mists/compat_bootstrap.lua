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

-- LE_ITEM_QUALITY_*: pre-Cata legacy enum constants. Mists's
-- Blizzard_FrameXMLBase/Classic/Constants.lua uses these as table keys, so
-- any nil among them errors the file with "table index is nil" before later
-- constants like KEYRING_CONTAINER and NUM_BAG_SLOTS get defined, cascading
-- into EquipmentManager.lua's "for KEYRING_CONTAINER, NUM_BAG_SLOTS" loop.
if rawget(_G, "LE_ITEM_QUALITY_POOR") == nil then LE_ITEM_QUALITY_POOR = 0 end
if rawget(_G, "LE_ITEM_QUALITY_COMMON") == nil then LE_ITEM_QUALITY_COMMON = 1 end
if rawget(_G, "LE_ITEM_QUALITY_UNCOMMON") == nil then LE_ITEM_QUALITY_UNCOMMON = 2 end
if rawget(_G, "LE_ITEM_QUALITY_RARE") == nil then LE_ITEM_QUALITY_RARE = 3 end
if rawget(_G, "LE_ITEM_QUALITY_EPIC") == nil then LE_ITEM_QUALITY_EPIC = 4 end
if rawget(_G, "LE_ITEM_QUALITY_LEGENDARY") == nil then LE_ITEM_QUALITY_LEGENDARY = 5 end
if rawget(_G, "LE_ITEM_QUALITY_ARTIFACT") == nil then LE_ITEM_QUALITY_ARTIFACT = 6 end
if rawget(_G, "LE_ITEM_QUALITY_HEIRLOOM") == nil then LE_ITEM_QUALITY_HEIRLOOM = 7 end
if rawget(_G, "LE_ITEM_QUALITY_WOW_TOKEN") == nil then LE_ITEM_QUALITY_WOW_TOKEN = 8 end

-- Inventory slot constants used as table keys in Constants.lua line 189+.
if rawget(_G, "INVSLOT_MAINHAND") == nil then INVSLOT_MAINHAND = 16 end
if rawget(_G, "INVSLOT_OFFHAND") == nil then INVSLOT_OFFHAND = 17 end
if rawget(_G, "INVSLOT_RANGED") == nil then INVSLOT_RANGED = 18 end

-- Challenge medal constants (Constants.lua line 607+).
if rawget(_G, "CHALLENGE_MEDAL_BRONZE") == nil then CHALLENGE_MEDAL_BRONZE = 1 end
if rawget(_G, "CHALLENGE_MEDAL_SILVER") == nil then CHALLENGE_MEDAL_SILVER = 2 end
if rawget(_G, "CHALLENGE_MEDAL_GOLD") == nil then CHALLENGE_MEDAL_GOLD = 3 end

-- ─── DebugBarManager (mists debug overlay) ───────────────────────────────────

if rawget(_G, "DebugBarManager") == nil then
  DebugBarManager = setmetatable({}, {
    __index = function() return function() end end,
  })
end

-- ─── C_LootHistory namespace ─────────────────────────────────────────────────

-- C_Item.GetItemQualityColor: mists's UIParent.lua iterates qualities 0..8
-- and stuffs the (r,g,b) tuple into ITEM_QUALITY_COLORS. Returning nil for
-- the tuple causes nil arithmetic when CreateColor formats hex markup.
do
  local quality_colors = {
    [0] = { 0.62, 0.62, 0.62, "9d9d9d" },  -- Poor (gray)
    [1] = { 1.00, 1.00, 1.00, "ffffff" },  -- Common (white)
    [2] = { 0.12, 1.00, 0.00, "1eff00" },  -- Uncommon (green)
    [3] = { 0.00, 0.44, 0.87, "0070dd" },  -- Rare (blue)
    [4] = { 0.64, 0.21, 0.93, "a335ee" },  -- Epic (purple)
    [5] = { 1.00, 0.50, 0.00, "ff8000" },  -- Legendary (orange)
    [6] = { 0.90, 0.80, 0.50, "e6cc80" },  -- Artifact (light gold)
    [7] = { 0.00, 0.80, 1.00, "00ccff" },  -- Heirloom (light blue)
    [8] = { 0.00, 0.80, 1.00, "00ccff" },  -- Token (light blue)
  }

  C_Item = C_Item or {}
  -- Override unconditionally: the simulator's existing C_Item registration
  -- doesn't expose this method, and mists's UIParent.lua needs it during
  -- bootstrap. Even if a future C_Item stub appears, mists's hardcoded color
  -- table is fine for visual fidelity in 2D mode.
  function C_Item.GetItemQualityColor(quality)
    local row = quality_colors[quality] or quality_colors[1]
    return row[1], row[2], row[3], row[4]
  end

  -- Flat global GetItemQualityColor: simulator's nil-stub returns nothing;
  -- mists's UIParent.lua expects (r,g,b,hex). Always override under mists.
  function GetItemQualityColor(quality)
    local row = quality_colors[quality] or quality_colors[1]
    return row[1], row[2], row[3], row[4]
  end
end

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

-- ─── Phase 4.4b: action-bar / LFD / raid / paperdoll helpers ─────────────────

if rawget(_G, "GetActionCharges") == nil then
  function GetActionCharges(slot) return nil end
end

if rawget(_G, "GetExtraBarIndex") == nil then
  function GetExtraBarIndex() return nil end
end

if rawget(_G, "GetMultiCastBarIndex") == nil then
  function GetMultiCastBarIndex() return nil end
end

if rawget(_G, "IsAutoRepeatAction") == nil then
  function IsAutoRepeatAction(slot) return false end
end

if rawget(_G, "IsUsableAction") == nil then
  function IsUsableAction(slot) return true, false end
end

if rawget(_G, "GetLFDChoiceCollapseState") == nil then
  function GetLFDChoiceCollapseState() return false end
end

if rawget(_G, "GetNumRaidProfiles") == nil then
  function GetNumRaidProfiles() return 0 end
end

if rawget(_G, "GetPVPYesterdayStats") == nil then
  function GetPVPYesterdayStats() return 0, 0 end
end

if rawget(_G, "MoneyFrame_SetType") == nil then
  function MoneyFrame_SetType(self, t) end
end

if rawget(_G, "MoneyFrame_Update") == nil then
  function MoneyFrame_Update(self, value) end
end

if rawget(_G, "MoneyInputFrame_SetNextFocus") == nil then
  function MoneyInputFrame_SetNextFocus() end
end

if rawget(_G, "PaperDollItemSlotButton_Update") == nil then
  function PaperDollItemSlotButton_Update(self) end
end

if rawget(_G, "RefreshDebuffs") == nil then
  function RefreshDebuffs() end
end

if rawget(_G, "GetLFDChoiceEnabledState") == nil then
  function GetLFDChoiceEnabledState() return true end
end

if rawget(_G, "GetPVPLifetimeStats") == nil then
  function GetPVPLifetimeStats() return 0, 0, 0 end
end

if rawget(_G, "GetNumBankSlots") == nil then
  function GetNumBankSlots() return 0, 0 end
end

if rawget(_G, "IsAttackAction") == nil then
  function IsAttackAction(slot) return false end
end

if rawget(_G, "IsEquippedAction") == nil then
  function IsEquippedAction(slot) return false end
end

if rawget(_G, "IsConsumableAction") == nil then
  function IsConsumableAction(slot) return false end
end

if rawget(_G, "IsStackableAction") == nil then
  function IsStackableAction(slot) return false end
end

-- Additional action-slot probes called from Classic/ActionButton.lua's
-- ActionButton_Update hot path. Default returns mirror "no spell here":
if rawget(_G, "HasAction") == nil then
  function HasAction(slot) return false end
end
if rawget(_G, "HasZoneAbility") == nil then
  function HasZoneAbility() return false end
end
if rawget(_G, "IsItemAction") == nil then
  function IsItemAction(slot) return false end
end
if rawget(_G, "IsCurrentAction") == nil then
  function IsCurrentAction(slot) return false end
end
if rawget(_G, "IsAutoCastPetAction") == nil then
  function IsAutoCastPetAction(slot) return false end
end
if rawget(_G, "IsEnabledAutoCastPetAction") == nil then
  function IsEnabledAutoCastPetAction(slot) return false end
end
if rawget(_G, "IsSpellOverlayed") == nil then
  function IsSpellOverlayed() return false end
end
if rawget(_G, "IsBindingForGamePad") == nil then
  function IsBindingForGamePad() return false end
end
if rawget(_G, "GetActionInfo") == nil then
  function GetActionInfo(slot) return nil, nil, nil end
end
if rawget(_G, "GetActionTexture") == nil then
  function GetActionTexture(slot) return nil end
end
if rawget(_G, "GetActionText") == nil then
  function GetActionText(slot) return "" end
end
if rawget(_G, "IsActionInRange") == nil then
  function IsActionInRange(slot) return nil end
end
if rawget(_G, "UnitInPhase") == nil then
  function UnitInPhase(unit) return true end
end
if rawget(_G, "GetActionCount") == nil then
  function GetActionCount(slot) return 0 end
end
if rawget(_G, "GetActionCooldown") == nil then
  function GetActionCooldown(slot) return 0, 0, 0 end
end
if rawget(_G, "GetActionButtonForID") == nil then
  function GetActionButtonForID(id) return nil end
end
if rawget(_G, "GetCooldownDuration") == nil then
  function GetCooldownDuration() return 0 end
end
if rawget(_G, "GetMacroSpell") == nil then
  function GetMacroSpell(idx) return nil end
end
if rawget(_G, "GetSpellCharges") == nil then
  function GetSpellCharges(spell) return nil end
end
if rawget(_G, "GetNewActionHighlightMark") == nil then
  function GetNewActionHighlightMark() return false end
end
if rawget(_G, "GetOnBarHighlightMark") == nil then
  function GetOnBarHighlightMark() return false end
end
if rawget(_G, "GetLastZoneAbilitySpellTexture") == nil then
  function GetLastZoneAbilitySpellTexture() return nil end
end
if rawget(_G, "GetCVarValueBool") == nil then
  function GetCVarValueBool(name) return false end
end

if rawget(_G, "MoneyFrame_SetMaxDisplayWidth") == nil then
  function MoneyFrame_SetMaxDisplayWidth(self, w) end
end

if rawget(_G, "RequestRatedInfo") == nil then
  function RequestRatedInfo() end
end

-- ChatAlertFrame and MiniMapTrackingBackground are frame references. Mists's
-- Blizzard_SharedXML normally defines them as real frames; if it doesn't get
-- there, leaving them nil is preferable to a noopFrame proxy which conflicts
-- with the real frame definition (see wrath/compat_frame_proxies.lua note).
