BootstrapOrderProbeDB = BootstrapOrderProbeDB or {}
BootstrapOrderProbeDB.events = BootstrapOrderProbeDB.events or {}

local function record(label)
    table.insert(BootstrapOrderProbeDB.events, label)
    print("[BootstrapOrderProbe] " .. label)
end

BootstrapOrderProbe_D_BootstrapSeen = (BootstrapOrderProbe_D_BootstrapSeen or 0) + 1
record("D non-LoD bootstrap file #" .. BootstrapOrderProbe_D_BootstrapSeen)
