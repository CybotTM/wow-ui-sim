-- Zero-reward renderer case; production GetLFGDungeonRewards stays installed.
LFDQueueFrame = CreateFrame('Frame')
LFDQueueFrameBackground = LFDQueueFrame:CreateTexture()
LFDQueueFrameRandomScrollFrameChildFrame = CreateFrame('Frame', 'LFDQueueFrameRandomScrollFrameChildFrame')
local p = LFDQueueFrameRandomScrollFrameChildFrame
for _, key in ipairs({'rewardsLabel','rewardsDescription','title','description','pugDescription','moneyLabel','xpLabel','xpAmount'}) do
    p[key] = p:CreateFontString()
end
p.moneyFrame = CreateFrame('Frame'); p.spacer = CreateFrame('Frame')
NUM_LFD_RANDOM_REWARD_FRAMES = 0
LFD_HOLIDAY_REWARD_EXPLANATION1 = 'Holiday rewards'
LFD_HOLIDAY_REWARD_EXPLANATION2 = 'Holiday completed'
LFD_RANDOM_REWARD_EXPLANATION1 = 'Random rewards'
LFD_RANDOM_REWARD_EXPLANATION2 = 'Random completed'
LFG_TYPE_RANDOM_DUNGEON = 'Random dungeon'; LFD_RANDOM_EXPLANATION = 'Random description'
