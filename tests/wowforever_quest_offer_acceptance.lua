-- Fresh WowLuaEnv, client-wowforever; no Blizzard UI handlers.
-- No state injection, API replacement, event listener, or fabricated metadata.
A_Admin.OpenMultiQuestNpc()
local offers = C_GossipInfo.GetAvailableQuests()
assert(#offers == 1 and GetNumAvailableQuests() == 1,
    "BLOCKER: expected the existing multi-NPC's single public offer")
local offer = offers[1]
local id, title, level = offer.questID, offer.title, offer.questLevel
assert(type(id) == "number" and id > 0, "BLOCKER: no public offer ID")
assert(type(title) == "string" and title ~= "", "BLOCKER: no public title")
assert(type(level) == "number", "BLOCKER: no public offer level")
local _, _, _, _, legacyID = GetAvailableQuestInfo(1)
assert(legacyID == id and GetAvailableTitle(1) == title,
    "BLOCKER: legacy offer index does not match public gossip metadata")

local function observe()
    local index = C_QuestLog.GetLogIndexForQuestID(id)
    local info
    if index ~= nil then info = C_QuestLog.GetInfo(index) end
    local count = C_QuestLog.GetNumQuestWatches()
    local ids, occurrences, watchLogIndex = {}, 0, nil
    for watchIndex = 1, count do
        local watchedID = C_QuestLog.GetQuestIDForQuestWatchIndex(watchIndex)
        ids[watchIndex] = tostring(watchedID)
        if watchedID == id then
            occurrences = occurrences + 1
            watchLogIndex = GetQuestIndexForWatch(watchIndex)
        end
    end
    return {
        onQuest = C_QuestLog.IsOnQuest(id), index = index,
        infoID = info and info.questID, title = info and info.title,
        level = info and info.level, difficulty = info and info.difficultyLevel,
        world = info and info.isWorldQuest,
        watchType = C_QuestLog.GetQuestWatchType(id), count = count,
        ids = table.concat(ids, ","), occurrences = occurrences,
        watchLogIndex = watchLogIndex,
    }
end

local function describe(stage, value)
    return stage .. " onQuest=" .. tostring(value.onQuest)
        .. " index=" .. tostring(value.index) .. " infoID=" .. tostring(value.infoID)
        .. " title=" .. tostring(value.title) .. " level=" .. tostring(value.level)
        .. " difficulty=" .. tostring(value.difficulty) .. " world=" .. tostring(value.world)
        .. " watchType=" .. tostring(value.watchType) .. " watchCount=" .. tostring(value.count)
        .. " watchIDs=" .. value.ids .. " occurrences=" .. tostring(value.occurrences)
        .. " watchLogIndex=" .. tostring(value.watchLogIndex)
end

local before = observe()
assert(before.onQuest == false and before.index == nil,
    "BLOCKER: seeded offer is already a rich quest-log member")
assert(before.watchType == nil and before.occurrences == 0,
    "BLOCKER: seeded offer already watched")
SelectAvailableQuest(1) -- Legacy API takes offer index, NOT quest ID.
local selectedID = GetQuestID()
assert(selectedID == id, "BLOCKER: public selection did not select the offered ID")
ConfirmAcceptQuest()
local accepted = observe()
local added = C_QuestLog.AddQuestWatch(id) -- Explicit watch; no auto-watch assumption.
local watched = observe()

local acceptedRich = accepted.onQuest == true and accepted.infoID == id
    and accepted.title == title and accepted.level == level
    and accepted.difficulty == level and accepted.world == false
local watchedRich = watched.onQuest == true and watched.infoID == id
    and watched.title == title and watched.level == level
    and watched.difficulty == level and watched.world == false
local needsWatch = accepted.watchType == nil
local expectedType, expectedCount = accepted.watchType, accepted.count
if needsWatch then
    expectedType, expectedCount = 1, accepted.count + 1 -- Explicit Add uses Manual.
end
local watchCoherent = added == needsWatch and watched.watchType == expectedType
    and watched.occurrences == 1 and watched.count == expectedCount
    and watched.watchLogIndex == watched.index
local report = "offerID=" .. tostring(id) .. " title=" .. title
    .. " offerLevel=" .. tostring(level) .. " questInfoID=" .. tostring(offer.questInfoID)
    .. " selectedID=" .. tostring(selectedID)
    .. "\n" .. describe("before", before)
    .. "\n" .. describe("accepted", accepted)
    .. "\naddWatch=" .. tostring(added)
    .. "\n" .. describe("watched", watched)
return id, acceptedRich and watchedRich and watchCoherent, report
