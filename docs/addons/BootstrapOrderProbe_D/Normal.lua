BootstrapOrderProbeDB = BootstrapOrderProbeDB or {}
BootstrapOrderProbeDB.events = BootstrapOrderProbeDB.events or {}

local function record(label)
    table.insert(BootstrapOrderProbeDB.events, label)
    print("[BootstrapOrderProbe] " .. label)
end

BootstrapOrderProbe_D_NormalSeen = (BootstrapOrderProbe_D_NormalSeen or 0) + 1
record("D non-LoD normal file #" .. BootstrapOrderProbe_D_NormalSeen)

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", function()
    print("[BootstrapOrderProbe] D bootstrap seen=", BootstrapOrderProbe_D_BootstrapSeen, "normal seen=", BootstrapOrderProbe_D_NormalSeen)
    print("[BootstrapOrderProbe] D order snapshot=", table.concat(BootstrapOrderProbeDB.events, " -> "))
end)
