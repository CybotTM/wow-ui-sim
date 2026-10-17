-- Characterization only: explicitly loaded Classic/QuestFrame + Wrath/QuestInfo
-- from the Mists cache. This does not prove automatic Mists source selection.
assert(QUEST_DESCRIPTION_GRADIENT_CPS == 40)
assert(QUEST_DESCRIPTION_GRADIENT_LENGTH == 30)
assert(type(SOUNDKIT.IG_WRITE_QUEST) == "number", "real sound-kit prerequisite")
QuestFrameDetailPanel = CreateFrame("Frame", nil, UIParent)
QuestDetailScrollChildFrame = CreateFrame("Frame", nil, QuestFrameDetailPanel)
QuestFrameAcceptButton = CreateFrame("Button", nil, QuestFrameDetailPanel)
QuestInfoDescriptionText = QuestDetailScrollChildFrame:CreateFontString(nil, "ARTWORK")
QuestInfoDescriptionText:SetText("|cffff0000" .. string.rep("Hé H\n", 7) .. "|r")
local objectives = CreateFrame("Frame", nil, QuestDetailScrollChildFrame)
local rewards = CreateFrame("Frame", nil, QuestDetailScrollChildFrame)
objectives:Show()
rewards:Show()
QuestDetailScrollChildFrame.alphaDependentText = { objectives, rewards }

local function prime_callback(progress)
    -- Supplied post-OnShow state. The original OnShow/QuestInfo_Display tree is
    -- NOT executed or claimed; the original hide helper IS executed unchanged.
    QuestInfo_HideAlphaDependentText(QuestDetailScrollChildFrame)
    QuestFrameAcceptButton:Disable()
    QuestFrameDetailPanel.fading = 1
    QuestFrameDetailPanel.fadingProgress = progress
    QuestInfoDescriptionText:SetAlphaGradient(0, QUEST_DESCRIPTION_GRADIENT_LENGTH)
end

local function assert_pending(progress)
    assert(QuestFrameDetailPanel.fading == 1)
    assert(QuestFrameDetailPanel.fadingProgress == progress)
    assert(objectives:GetAlpha() == 0 and rewards:GetAlpha() == 0)
    assert(not QuestFrameAcceptButton:IsEnabled())
    assert(QuestInfoDescriptionText:HasAlphaGradient())
end

local function assert_complete(progress)
    assert(QuestFrameDetailPanel.fading == nil, "completion clears fading flag")
    assert(QuestFrameDetailPanel.fadingProgress == progress)
    assert(objectives:IsShown() and rewards:IsShown())
    assert(objectives:GetAlpha() == 1 and rewards:GetAlpha() == 1,
        "original show helper reveals both dependents")
    assert(QuestFrameAcceptButton:IsEnabled(), "Accept enabled")
    assert(QuestInfoDescriptionText:HasAlphaGradient(), "stored gradient retained")
    QuestFrameDetailPanel_OnUpdate(QuestFrameDetailPanel, 0.25)
    assert(QuestFrameDetailPanel.fadingProgress == progress, "completed callback is idle")
    assert(QuestFrameDetailPanel.fading == nil)
    assert(objectives:GetAlpha() == 1 and rewards:GetAlpha() == 1)
    assert(QuestFrameAcceptButton:IsEnabled())
end

function RunMistsGradualGradientCharacterization()
    SetCVar("instantQuestText", "0")
    assert(GetCVar("instantQuestText") == "0")
    prime_callback(0)
    assert_pending(0)
    QuestFrameDetailPanel_OnUpdate(QuestFrameDetailPanel, 0.25)
    assert_pending(10)
    QuestFrameDetailPanel_OnUpdate(QuestFrameDetailPanel, 0.5)
    assert_pending(30)
    -- Select the immediate helper only on the completing tick, as in Wrath.
    SetCVar("instantQuestText", "1")
    assert(GetCVar("instantQuestText") == "1")
    QuestFrameDetailPanel_OnUpdate(QuestFrameDetailPanel, 0.125)
    assert_complete(35)
end

function RunMistsInstantGradientCharacterization()
    SetCVar("instantQuestText", "1")
    assert(GetCVar("instantQuestText") == "1")
    prime_callback(1024) -- Supplied initializer convention, NOT OnShow branch proof.
    assert_pending(1024)
    QuestFrameDetailPanel_OnUpdate(QuestFrameDetailPanel, 0.25)
    assert_complete(1034)
end
