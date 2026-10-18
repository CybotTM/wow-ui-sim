-- Verbatim consumer excerpts from the archived timer125 measurement addons.
-- Only fixture construction and assertions are simulator test code.
function RunEllesmereHookFixture()
local PP, ns = {}, {}
local EAB = { db = { profile = { showEquippedBorder = true } } }
local HIGHLIGHT_TEXTURES = { "Interface\\Buttons\\WHITE8x8" }
local EAB_VTABLE = {}
local _pixelSnapDisabled = setmetatable({}, { __mode = "k" })
-- EllesmereUI/EllesmereUI_PixelPerfect.lua:304-327
    function PP.DisablePixelSnap(obj)
        if not obj then return end
        if issecretvalue and issecretvalue(obj) then return end
        if issecrettable and issecrettable(obj) then return end
        if _pixelSnapDisabled[obj] then return end
        if obj.IsForbidden and obj:IsForbidden() then return end

        -- Textures and FontStrings expose SetSnapToPixelGrid directly
        local target = obj
        if not obj.SetSnapToPixelGrid and obj.GetStatusBarTexture then
            -- StatusBars need their inner texture unsnapped instead
            target = obj:GetStatusBarTexture()
            if type(target) ~= "table" or not target.SetSnapToPixelGrid then
                _pixelSnapDisabled[obj] = true
                return
            end
        end

        if target.SetSnapToPixelGrid then
            target:SetSnapToPixelGrid(false)
            target:SetTexelSnappingBias(0)
        end
        _pixelSnapDisabled[obj] = true
    end
-- EllesmereUI/EllesmereUI_PixelPerfect.lua:350-375
    local function WatchPixelSnap(frame, snap)
        if issecrettable and issecrettable(frame) then return end
        if (frame and not frame:IsForbidden()) and _pixelSnapDisabled[frame] and snap then
            _pixelSnapDisabled[frame] = nil
        end
    end

    local _hookedMetatables = {}
    local function HookPixelSnap(object)
        local mk = getmetatable(object)
        if not mk then return end
        mk = mk.__index
        if not mk or _hookedMetatables[mk] then return end

        if mk.SetSnapToPixelGrid or mk.SetStatusBarTexture or mk.SetColorTexture
           or mk.SetAtlas or mk.SetTexture then
            -- Rationale in the banner. CreateTexture is NOT hooked: hooksecurefunc
            -- passes the parent frame, not the new texture, so it would be a no-op.
            if mk.SetSnapToPixelGrid then hooksecurefunc(mk, "SetSnapToPixelGrid", WatchPixelSnap) end
            if mk.SetStatusBarTexture then hooksecurefunc(mk, "SetStatusBarTexture", PP.DisablePixelSnap) end
            if mk.SetColorTexture then hooksecurefunc(mk, "SetColorTexture", PP.DisablePixelSnap) end
            if mk.SetAtlas then hooksecurefunc(mk, "SetAtlas", PP.DisablePixelSnap) end
            if mk.SetTexture then hooksecurefunc(mk, "SetTexture", PP.DisablePixelSnap) end
            _hookedMetatables[mk] = true
        end
    end
-- EllesmereUIActionBars/EUI_ActionBars_ButtonArt.lua:67-74,81-135
local function SetSquareTexture(texture, texPath)
    if not texture then return end
    texture:SetAtlas(nil)
    texture:SetTexture(texPath)
    texture:SetTexCoord(0, 1, 0, 1)
    texture:ClearAllPoints()
    texture:SetAllPoints(texture:GetParent())
end
do
    local busy = false
    local IsEquippedAction, GetActionTexture = C_ActionBar.IsEquippedAction, C_ActionBar.GetActionTexture

    -- Rarity of the equipped item an action holds, read from the slot that
    -- holds it (an item's quality can differ from its base record): an item
    -- action by its ID, anything else (a macro) by its icon.
    local function EquippedQuality(action)
        local aType, id = GetActionInfo(action)
        local isItem = aType == "item" and id
        local tex = not isItem and GetActionTexture(action)
        if not (isItem or tex) then return end
        for slot = 1, 19 do
            local hit
            if isItem then
                hit = GetInventoryItemID("player", slot) == id
            else
                hit = GetInventoryItemTexture("player", slot) == tex
            end
            if hit then return GetInventoryItemQuality("player", slot) end
        end
        if isItem then return C_Item.GetItemQualityByID(id) end
    end

    function ns.AB_EquippedBorderLook(bd, btn)
        if busy then return end
        busy = true
        SetSquareTexture(bd, HIGHLIGHT_TEXTURES[1])
        busy = false
        local a = btn and btn:GetAttribute("action")
        local q = a and IsEquippedAction(a) and EquippedQuality(a)
        if q then
            local r, g, b = C_Item.GetItemQualityColor(q)
            bd:SetVertexColor(r, g, b, 1)
        else
            bd:SetVertexColor(0, 1, 0, 0.5)
        end
        bd:SetAlpha(1)
    end

    -- The equipped-item Border from live slot contents, for every path that
    -- changes what a button holds (slot changes, page flips, bar reveals,
    -- setup). Our square buttons show it only with Show Equipped Item Color
    -- on; the Blizzard and Classic styles keep Blizzard's own look.
    function ns.AB_SyncEquippedBorder(btn, action)
        local bd = btn.Border
        if not bd then return end
        local fd = ns._eabFD[btn]
        local square = fd and fd.squared
        if square and not EAB.db.profile.showEquippedBorder then return end
        local on = action and IsEquippedAction(action) and true or false
        if square and on then ns.AB_EquippedBorderLook(bd, btn) end
        bd:SetShown(on)
    end
end
local host = CreateFrame("Frame")
local tex = host:CreateTexture()
HookPixelSnap(tex)
local td = {}
-- EllesmereUIBlizzardSkin/EllesmereUIBlizzardSkin_WindowPacks.lua:3540-3547
                local function ApplyCaret()
                    if td.inSet then return end
                    td.inSet = true
                    tex:SetAtlas("Azerite-PointingArrow", false)
                    tex:SetSize(14, 10)
                    tex:SetVertexColor(1, 1, 1, 1)
                    td.inSet = false
                end
hooksecurefunc(tex, "SetAtlas", ApplyCaret)
hooksecurefunc(tex, "SetTexture", ApplyCaret)
hooksecurefunc(tex, "SetTexCoord", ApplyCaret)
ApplyCaret()
local caretPaints = 0
hooksecurefunc(tex, "SetSize", function() caretPaints = caretPaints + 1 end)
local buttons = {}
for index = 1, 3 do
    local btn = CreateFrame("Button", nil, host)
    btn.Border = btn:CreateTexture()
    local fd = {}
-- EllesmereUIActionBars/EUI_ActionBars_ButtonArt.lua:558-570 (per-button border hooks)
    if btn.Border and not fd.borderHooked then
        local function BorderRefresh(self)
            if EAB.db.profile.showEquippedBorder then
                ns.AB_EquippedBorderLook(self, btn)
            else
                self:SetAlpha(0)
                EAB_VTABLE.HideRegionDeferred(self)
            end
        end
        hooksecurefunc(btn.Border, "SetAtlas", BorderRefresh)
        hooksecurefunc(btn.Border, "Show", BorderRefresh)
        fd.borderHooked = true
    end
    buttons[index] = btn
end
for _, btn in ipairs(buttons) do
    SetSquareTexture(btn.Border, HIGHLIGHT_TEXTURES[1])
end
return caretPaints
end
