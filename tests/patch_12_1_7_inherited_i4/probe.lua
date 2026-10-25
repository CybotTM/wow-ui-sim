-- Observations are not native expectations. Inherited removal is never an oracle.
local function rawPath(path)
    local value = _G;
    for part in path:gmatch("[^.]+") do
        if type(value) ~= "table" then return nil; end
        value = rawget(value, part);
    end
    return value;
end

local function encode(value)
    if value == nil then return "nil"; end
    local text = tostring(value);
    return type(value) .. ":" .. #text .. ":" .. text;
end

if section == "cvars" then
    if symbol == "dumpSmallAlloc" then
        -- Catalog only: never invoke this diagnostic command.
        for _, entry in ipairs(C_Console.GetAllCommands()) do
            if entry.command == symbol then
                return "commandType=" .. encode(entry.commandType);
            end
        end
        return "command-absent";
    end
    return "value=" .. encode(C_CVar.GetCVar(symbol))
        .. ";default=" .. encode(C_CVar.GetCVarDefault(symbol))
        .. ";scope=native-unknown";
end

if section == "events" then
    local frame = CreateFrame("Frame");
    local ok = pcall(frame.RegisterEvent, frame, symbol);
    local registered = frame:IsEventRegistered(symbol) == true;
    frame:UnregisterAllEvents();
    return "register-call=" .. tostring(ok) .. ";registered=" .. tostring(registered)
        .. ";payload-and-lifecycle=native-unknown";
end

if section == "widgets" then
    local owner, method = symbol:match("^([^:]+):([^:]+)$");
    local receiver;
    if owner == "ScriptRegion" then
        receiver = CreateFrame("Button", nil, UIParent);
    elseif owner == "Minimap" then
        -- The original XML-selected instance, not CreateFrame('Minimap').
        receiver = rawget(_G, "Minimap");
    else
        return "unsupported-3D-domain;native-publication=unknown";
    end
    if receiver == nil then return "receiver-unavailable"; end
    return type(receiver[method]);
end

return type(rawPath(symbol));
