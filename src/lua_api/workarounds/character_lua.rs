pub(super) const GETGLOBAL_HELPER_LUA: &str = r#"
local function __wow_getglobal(name)
    return getglobal(name)
end
_G.__wow_panel_getglobal = __wow_getglobal
"#;

pub(super) const CHARACTER_CREATE_DEFAULTS_WORKAROUND_LUA: &str = r#"
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

pub(super) const CHARACTER_FRAME_SURFACE_REFRESH_WORKAROUND_LUA: &str = r#"
local function get_character_panel_slot_buttons()
    local slotFrameNames = {
        "CharacterHeadSlot",
        "CharacterNeckSlot",
        "CharacterShoulderSlot",
        "CharacterBackSlot",
        "CharacterChestSlot",
        "CharacterShirtSlot",
        "CharacterTabardSlot",
        "CharacterWristSlot",
        "CharacterHandsSlot",
        "CharacterWaistSlot",
        "CharacterLegsSlot",
        "CharacterFeetSlot",
        "CharacterFinger0Slot",
        "CharacterFinger1Slot",
        "CharacterTrinket0Slot",
        "CharacterTrinket1Slot",
        "CharacterMainHandSlot",
        "CharacterSecondaryHandSlot",
    }
    local buttons = {}
    for _, frameName in ipairs(slotFrameNames) do
        local button = _G[frameName]
        if type(button) == "table" then
            table.insert(buttons, button)
        end
    end
    return buttons
end

if type(CharacterFrame) == "table"
    and type(CharacterFrame.GetScript) == "function"
    and type(CharacterFrame.SetScript) == "function" then
    local existing_wrapper = rawget(_G, "__wow_character_frame_onshow_wrapper")
    if CharacterFrame:GetScript("OnShow") ~= existing_wrapper then
        local original_on_show = CharacterFrame:GetScript("OnShow")
        if type(original_on_show) ~= "function" then
            return
        end
        local wrapper = function(self, ...)
            original_on_show(self, ...)
            if type(self.UpdateTitle) == "function" then
                self:UpdateTitle()
            end
            if type(PaperDollItemSlotButton_Update) == "function" then
                for _, button in ipairs(get_character_panel_slot_buttons()) do
                    PaperDollItemSlotButton_Update(button)
                end
            end
        end
        CharacterFrame:SetScript("OnShow", wrapper)
        rawset(_G, "__wow_character_frame_onshow_wrapper", wrapper)
    end
end

if type(CharacterFrame) == "table" and type(CharacterFrame.RefreshDisplay) == "function" then
    local existing_wrapper = rawget(_G, "__wow_character_frame_refresh_display_wrapper")
    if CharacterFrame.RefreshDisplay ~= existing_wrapper then
        local original_refresh_display = CharacterFrame.RefreshDisplay
        local wrapper = function(self, ...)
            original_refresh_display(self, ...)
            if type(self.UpdateTitle) == "function" then
                self:UpdateTitle()
            end
            if type(PaperDollItemSlotButton_Update) == "function" then
                for _, button in ipairs(get_character_panel_slot_buttons()) do
                    PaperDollItemSlotButton_Update(button)
                end
            end
        end
        CharacterFrame.RefreshDisplay = wrapper
        rawset(_G, "__wow_character_frame_refresh_display_wrapper", wrapper)
    end
end

if type(CharacterFrame) == "table" then
    if type(CharacterFrame.RefreshDisplay) == "function" then
        CharacterFrame:RefreshDisplay()
    elseif type(CharacterFrame.UpdateTitle) == "function" then
        CharacterFrame:UpdateTitle()
    end
end

if type(PaperDollItemSlotButton_Update) == "function" then
    for _, button in ipairs(get_character_panel_slot_buttons()) do
        PaperDollItemSlotButton_Update(button)
    end
end

if type(CharacterFrame) == "table"
    and CharacterFrame.TitleContainer
    and CharacterFrame.TitleContainer.TitleText
    and type(CharacterFrame.TitleContainer.TitleText.SetText) == "function" then
    CharacterFrame.TitleContainer.TitleText:SetText(UnitPVPName("player"))
end

for _, button in ipairs(get_character_panel_slot_buttons()) do
    if type(button.icon) == "table" then
        local textureName = GetInventoryItemTexture("player", button:GetID())
        if textureName ~= nil then
            button.icon:SetTexture(textureName)
        elseif button.backgroundTextureName ~= nil then
            button.icon:SetTexture(button.backgroundTextureName)
        end
    end
end
"#;
