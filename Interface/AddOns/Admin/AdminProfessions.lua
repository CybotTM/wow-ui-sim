-- AdminProfessions.lua
-- Adds a "Grant Reagents" button to the ProfessionsFrame schematic form so
-- the player can drop the currently displayed recipe's required materials
-- into their backpack with one click. Backed by `A_Admin.SeedReagentsForRecipe`.

local BUTTON_NAME = "WowSimAdminGrantReagentsButton"

local function FireBagRefreshEvents()
    if FireEvent then
        for bag = 0, 4 do
            FireEvent("BAG_UPDATE", bag)
        end
        FireEvent("BAG_UPDATE_DELAYED")
    end
end

local function GrantReagentsForCurrentRecipe(schematicForm)
    local info = schematicForm and schematicForm.currentRecipeInfo
    local recipeID = info and info.recipeID
    if not recipeID then
        print("[Admin] No recipe selected.")
        return
    end
    if not (A_Admin and A_Admin.SeedReagentsForRecipe) then
        print("[Admin] A_Admin.SeedReagentsForRecipe unavailable.")
        return
    end
    local ok = A_Admin.SeedReagentsForRecipe(recipeID, 1)
    if ok then
        FireBagRefreshEvents()
        print(string.format("[Admin] Granted reagents for recipe %d.", recipeID))
    else
        print(string.format("[Admin] Recipe %d not found in profession_data.", recipeID))
    end
end

local function CreateGrantButton(schematicForm)
    if _G[BUTTON_NAME] then return _G[BUTTON_NAME] end

    local btn = CreateFrame("Button", BUTTON_NAME, schematicForm, "UIPanelButtonTemplate")
    btn:SetSize(120, 22)
    btn:SetText("Grant Reagents")

    -- Anchor under the Track Recipe checkbox (top-right area of the form).
    if schematicForm.TrackRecipeCheckbox then
        btn:SetPoint("TOPRIGHT", schematicForm.TrackRecipeCheckbox, "BOTTOMRIGHT", 0, -4)
    else
        btn:SetPoint("TOPRIGHT", schematicForm, "TOPRIGHT", -10, -10)
    end

    btn:SetScript("OnClick", function()
        GrantReagentsForCurrentRecipe(schematicForm)
    end)

    return btn
end

local function TryAttachButton()
    local frame = _G.ProfessionsFrame
    local schematicForm = frame and frame.CraftingPage and frame.CraftingPage.SchematicForm
    if not schematicForm then return false end
    CreateGrantButton(schematicForm)
    return true
end

-- ProfessionsFrame is on-demand: it only exists once Blizzard_Professions has loaded.
local loader = CreateFrame("Frame")
loader:RegisterEvent("ADDON_LOADED")
loader:RegisterEvent("PLAYER_LOGIN")
loader:SetScript("OnEvent", function(self, event, addonName)
    if event == "ADDON_LOADED" and addonName ~= "Blizzard_Professions" then
        return
    end
    if TryAttachButton() then
        self:UnregisterAllEvents()
    end
end)

-- In case Blizzard_Professions has already loaded by the time this runs.
TryAttachButton()
