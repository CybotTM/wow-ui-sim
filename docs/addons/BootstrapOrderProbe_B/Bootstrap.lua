BootstrapOrderProbeDB = BootstrapOrderProbeDB or {}
BootstrapOrderProbeDB.events = BootstrapOrderProbeDB.events or {}

local function record(label)
    table.insert(BootstrapOrderProbeDB.events, label)
    print("[BootstrapOrderProbe] " .. label)
end

BootstrapOrderProbe_B_BootstrapSeen = (BootstrapOrderProbe_B_BootstrapSeen or 0) + 1
record("B bootstrap file #" .. BootstrapOrderProbe_B_BootstrapSeen)
