-- Supplied widget graph; original QuestInfo functions and methods are never replaced.
assert(QUEST_DESCRIPTION_GRADIENT_CPS == 70)
assert(QUEST_DESCRIPTION_GRADIENT_LENGTH == 30)
QuestInfoFrame = CreateFrame("Frame", nil, UIParent)
QuestInfoFrame.acceptButton = CreateFrame("Button", nil, QuestInfoFrame)
QuestInfoFadingFrame = CreateFrame("Frame", nil, QuestInfoFrame)
QuestInfoFadingFrame:Show()
QuestInfoDescriptionText = QuestInfoFrame:CreateFontString(nil, "ARTWORK")
-- Seven repetitions of five scalars; markup and UTF-8 bytes are not positions.
QuestInfoDescriptionText:SetText("|cffff0000" .. string.rep("Hé H\n", 7) .. "|r")

local function assert_pending(progress)
    assert(QuestInfoFadingFrame.fading == 1, "fade must remain pending")
    assert(QuestInfoFadingFrame.fadingProgress == progress, "exact progress")
    assert(QuestInfoFadingFrame:GetAlpha() == 0, "dependent frame still transparent")
    assert(not QuestInfoFrame.acceptButton:IsEnabled(), "Accept still disabled")
    assert(QuestInfoDescriptionText:HasAlphaGradient(), "gradient remains installed")
end

local function assert_complete(progress)
    assert(QuestInfoFadingFrame.fading == nil, "completion clears fading flag")
    assert(QuestInfoFadingFrame.fadingProgress == progress, "completion progress")
    assert(QuestInfoFadingFrame:IsShown(), "dependent frame remains shown")
    assert(QuestInfoFadingFrame:GetAlpha() == 1, "dependent frame revealed")
    assert(QuestInfoFrame.acceptButton:IsEnabled(), "Accept enabled")
    assert(QuestInfoDescriptionText:HasAlphaGradient(), "completion retains gradient")
    QuestInfoFadingFrame_OnUpdate(QuestInfoFadingFrame, 0.25)
    assert(QuestInfoFadingFrame.fadingProgress == progress, "completed callback is idle")
    assert(QuestInfoFadingFrame.fading == nil)
    assert(QuestInfoFadingFrame:GetAlpha() == 1)
    assert(QuestInfoFrame.acceptButton:IsEnabled())
end

function RunWrathGradualGradient()
    QUEST_FADING_DISABLE = "0"
    assert(QuestInfo_ShowFadingFrame() == QuestInfoFadingFrame)
    assert_pending(0)
    QuestInfoFadingFrame_OnUpdate(QuestInfoFadingFrame, 0.25)
    assert_pending(17.5)
    QuestInfoFadingFrame_OnUpdate(QuestInfoFadingFrame, 0.125)
    assert_pending(26.25)
    -- Explicit configuration transition: retain gradual text progress but select
    -- immediate dependent reveal for the completing tick. Fixed "0" schedules
    -- UIFrameFadeIn instead; that scheduler is outside this T4 test.
    QUEST_FADING_DISABLE = "1"
    QuestInfoFadingFrame_OnUpdate(QuestInfoFadingFrame, 0.125)
    assert_complete(35)
end

function RunWrathInstantGradient()
    QUEST_FADING_DISABLE = "1"
    assert(QuestInfo_ShowFadingFrame() == QuestInfoFadingFrame)
    assert_pending(1024) -- Original initializer selects this branch, not the test.
    QuestInfoFadingFrame_OnUpdate(QuestInfoFadingFrame, 0.25)
    assert_complete(1041.5)
end
