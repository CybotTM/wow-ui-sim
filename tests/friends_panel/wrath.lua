-- Native 12340 FriendsFrame.lua owns the legacy list, not a modern ScrollBox.
ToggleFriendsFrame(1)
FriendsList_Update()
assert(FriendsFrame:IsShown() and FriendsListFrame:IsShown(), "native friends panel must open")
local total, online = GetNumFriends()
local onlineNames = {}
local offlineName
local offlineConnected
for index = 1, total do
    local name, _, _, _, connected = GetFriendInfo(index)
    assert(type(name) == "string", "native friend row must retain its identity")
    if connected then
        table.insert(onlineNames, name)
    else
        offlineName = name
        offlineConnected = connected
    end
end
table.sort(onlineNames)
return table.concat({ total, online, table.concat(onlineNames, "\n"),
    assert(offlineName, "native offline friend row"), tostring(offlineConnected) }, "|")
