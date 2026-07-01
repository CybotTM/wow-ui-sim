BootstrapOrderProbeDB = BootstrapOrderProbeDB or {}
BootstrapOrderProbeDB.events = BootstrapOrderProbeDB.events or {}

local function record(label)
    table.insert(BootstrapOrderProbeDB.events, label)
    print("[BootstrapOrderProbe] " .. label)
end

BootstrapOrderProbe_B_NormalSeen = (BootstrapOrderProbe_B_NormalSeen or 0) + 1
record("B normal file #" .. BootstrapOrderProbe_B_NormalSeen)
