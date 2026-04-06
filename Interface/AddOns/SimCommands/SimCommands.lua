SimCommands = {}
SimCommands.commands = {}

function SimCommands:Register(name, description, action, category)
    table.insert(self.commands, {
        name = name,
        description = description or "",
        action = action,
        category = category or "General",
    })
end

function SimCommands:GetCommands()
    return self.commands
end

-- Filter commands by substring match on name or description (case-insensitive).
function SimCommands:Filter(query)
    if not query or query == "" then
        return self.commands
    end
    local q = query:lower()
    local results = {}
    for _, cmd in ipairs(self.commands) do
        if cmd.name:lower():find(q, 1, true)
            or cmd.description:lower():find(q, 1, true) then
            table.insert(results, cmd)
        end
    end
    return results
end

---------------------------------------------------------------------------
-- UI: command palette frame
---------------------------------------------------------------------------

local MAX_VISIBLE_ROWS = 12
local ROW_HEIGHT = 24
local PANEL_WIDTH = 420
local PANEL_HEIGHT = 42 + ROW_HEIGHT * MAX_VISIBLE_ROWS  -- editbox + rows

local function CreatePaletteFrame()
    local frame = CreateFrame("Frame", "SimCommandsFrame", UIParent)
    frame:SetSize(PANEL_WIDTH, PANEL_HEIGHT)
    frame:SetPoint("CENTER", 0, 100)
    frame:SetFrameStrata("DIALOG")
    frame:SetFrameLevel(500)
    frame:Hide()
    frame:EnableMouse(true)

    -- Dark background
    local bg = frame:CreateTexture(nil, "BACKGROUND")
    bg:SetAllPoints()
    bg:SetColorTexture(0.1, 0.1, 0.1, 0.92)

    -- Border
    local border = frame:CreateTexture(nil, "BORDER")
    border:SetPoint("TOPLEFT", -1, 1)
    border:SetPoint("BOTTOMRIGHT", 1, -1)
    border:SetColorTexture(0.4, 0.4, 0.4, 1)

    -- Inner background (covers border interior)
    local inner = frame:CreateTexture(nil, "ARTWORK")
    inner:SetAllPoints()
    inner:SetColorTexture(0.1, 0.1, 0.1, 0.92)

    return frame
end

local function CreateSearchBox(parent)
    local box = CreateFrame("EditBox", "SimCommandsSearchBox", parent)
    box:SetSize(PANEL_WIDTH - 16, 24)
    box:SetPoint("TOP", 0, -8)
    box:SetAutoFocus(false)
    box:SetFontObject(GameFontNormal or "GameFontNormal")
    box:SetTextInsets(8, 8, 0, 0)

    local boxBg = box:CreateTexture(nil, "BACKGROUND")
    boxBg:SetAllPoints()
    boxBg:SetColorTexture(0.15, 0.15, 0.15, 1)

    return box
end

local function CreateRow(parent, index)
    local row = CreateFrame("Button", nil, parent)
    row:SetSize(PANEL_WIDTH - 8, ROW_HEIGHT)
    row:SetPoint("TOPLEFT", parent, "TOPLEFT", 4, -(36 + (index - 1) * ROW_HEIGHT))

    local highlight = row:CreateTexture(nil, "HIGHLIGHT")
    highlight:SetAllPoints()
    highlight:SetColorTexture(0.3, 0.3, 0.5, 0.4)

    row.nameText = row:CreateFontString(nil, "OVERLAY")
    row.nameText:SetFontObject(GameFontNormal or "GameFontNormal")
    row.nameText:SetPoint("LEFT", 8, 0)
    row.nameText:SetJustifyH("LEFT")
    row.nameText:SetWidth(PANEL_WIDTH - 24)
    row.nameText:SetWordWrap(false)

    row.command = nil
    return row
end

---------------------------------------------------------------------------
-- Palette controller
---------------------------------------------------------------------------

local palette  -- the Frame
local searchBox
local rows = {}
local filteredCommands = {}

local function RefreshRows()
    filteredCommands = SimCommands:Filter(searchBox and searchBox:GetText() or "")
    for i = 1, MAX_VISIBLE_ROWS do
        local row = rows[i]
        local cmd = filteredCommands[i]
        if cmd then
            local label = cmd.name
            if cmd.description ~= "" then
                label = label .. "  |cff888888" .. cmd.description .. "|r"
            end
            row.nameText:SetText(label)
            row.command = cmd
            row:Show()
        else
            row.nameText:SetText("")
            row.command = nil
            row:Hide()
        end
    end
end

local function InitUI()
    if palette then return end

    palette = CreatePaletteFrame()
    searchBox = CreateSearchBox(palette)

    for i = 1, MAX_VISIBLE_ROWS do
        local row = CreateRow(palette, i)
        row:SetScript("OnClick", function(self)
            if self.command and self.command.action then
                palette:Hide()
                self.command.action()
            end
        end)
        rows[i] = row
    end

    searchBox:SetScript("OnTextChanged", function()
        RefreshRows()
    end)

    searchBox:SetScript("OnEscapePressed", function(self)
        self:ClearFocus()
        palette:Hide()
    end)

    searchBox:SetScript("OnEnterPressed", function(self)
        -- Execute first matching command
        if filteredCommands[1] and filteredCommands[1].action then
            palette:Hide()
            filteredCommands[1].action()
        end
    end)

    -- Handle CTRL-P while the search box has focus (EditBox focus blocks keybind dispatch)
    searchBox:SetScript("OnKeyDown", function(self, key)
        if key == "CTRL-P" then
            palette:Hide()
        end
    end)

    palette:SetScript("OnShow", function()
        searchBox:SetText("")
        searchBox:SetFocus()
        RefreshRows()
    end)

    palette:SetScript("OnHide", function()
        searchBox:ClearFocus()
    end)
end

function SimCommands:Toggle()
    InitUI()
    if palette:IsShown() then
        palette:Hide()
    else
        palette:Show()
    end
end

function SimCommands:Show()
    InitUI()
    palette:Show()
end

function SimCommands:Hide()
    if palette then palette:Hide() end
end

function SimCommands:IsShown()
    return palette and palette:IsShown() or false
end

---------------------------------------------------------------------------
-- Minimap button
---------------------------------------------------------------------------

local function CreateMinimapButton()
    local btn = CreateFrame("Button", "SimCommandsMinimapButton", Minimap or UIParent)
    btn:SetSize(28, 28)
    btn:SetPoint("BOTTOMLEFT", Minimap or UIParent, "BOTTOMLEFT", 2, 2)
    btn:SetFrameStrata("MEDIUM")
    btn:SetFrameLevel(8)

    local bg = btn:CreateTexture(nil, "BACKGROUND")
    bg:SetAllPoints()
    bg:SetColorTexture(0.2, 0.2, 0.3, 0.85)

    local border = btn:CreateTexture(nil, "BORDER")
    border:SetPoint("TOPLEFT", -1, 1)
    border:SetPoint("BOTTOMRIGHT", 1, -1)
    border:SetColorTexture(0.5, 0.5, 0.6, 1)

    local inner = btn:CreateTexture(nil, "ARTWORK")
    inner:SetAllPoints()
    inner:SetColorTexture(0.2, 0.2, 0.3, 0.85)

    local label = btn:CreateFontString(nil, "OVERLAY")
    label:SetFontObject(GameFontNormalSmall or "GameFontNormalSmall")
    label:SetPoint("CENTER", 0, 0)
    label:SetText("Sim")

    local hl = btn:CreateTexture(nil, "HIGHLIGHT")
    hl:SetAllPoints()
    hl:SetColorTexture(0.4, 0.4, 0.6, 0.3)

    btn:SetScript("OnClick", function()
        SimCommands:Toggle()
    end)

    return btn
end

-- Create the minimap button at load time (does not depend on palette UI)
CreateMinimapButton()

---------------------------------------------------------------------------
-- Input prompt dialog
---------------------------------------------------------------------------

local promptFrame, promptInput, promptLabel

local function CreatePromptDialog()
    if promptFrame then return end

    promptFrame = CreateFrame("Frame", "SimCommandsPrompt", UIParent)
    promptFrame:SetSize(300, 80)
    promptFrame:SetPoint("CENTER", 0, 150)
    promptFrame:SetFrameStrata("DIALOG")
    promptFrame:SetFrameLevel(600)
    promptFrame:EnableMouse(true)
    promptFrame:Hide()

    local bg = promptFrame:CreateTexture(nil, "BACKGROUND")
    bg:SetAllPoints()
    bg:SetColorTexture(0.1, 0.1, 0.1, 0.95)

    local border = promptFrame:CreateTexture(nil, "BORDER")
    border:SetPoint("TOPLEFT", -1, 1)
    border:SetPoint("BOTTOMRIGHT", 1, -1)
    border:SetColorTexture(0.4, 0.4, 0.4, 1)

    local inner = promptFrame:CreateTexture(nil, "ARTWORK")
    inner:SetAllPoints()
    inner:SetColorTexture(0.1, 0.1, 0.1, 0.95)

    promptLabel = promptFrame:CreateFontString(nil, "OVERLAY")
    promptLabel:SetFontObject(GameFontNormal or "GameFontNormal")
    promptLabel:SetPoint("TOP", 0, -10)

    promptInput = CreateFrame("EditBox", "SimCommandsPromptInput", promptFrame)
    promptInput:SetSize(260, 24)
    promptInput:SetPoint("BOTTOM", 0, 14)
    promptInput:SetAutoFocus(false)
    promptInput:SetFontObject(GameFontNormal or "GameFontNormal")
    promptInput:SetTextInsets(8, 8, 0, 0)
    local inputBg = promptInput:CreateTexture(nil, "BACKGROUND")
    inputBg:SetAllPoints()
    inputBg:SetColorTexture(0.15, 0.15, 0.15, 1)

    promptInput:SetScript("OnEscapePressed", function(self)
        self:ClearFocus()
        promptFrame:Hide()
    end)
end

--- Show an input prompt. `callback(text)` is called when the user presses Enter.
--- Pass `numeric = true` in opts for number-only input.
function SimCommands:Prompt(label, callback, opts)
    CreatePromptDialog()
    promptLabel:SetText(label)
    promptInput:SetText("")
    promptInput:SetNumeric(opts and opts.numeric or false)
    promptInput:SetScript("OnEnterPressed", function(self)
        local text = self:GetText()
        self:ClearFocus()
        promptFrame:Hide()
        if callback then callback(text) end
    end)
    promptFrame:Show()
    promptInput:SetFocus()
end
