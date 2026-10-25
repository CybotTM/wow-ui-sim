-- Invoke original functions loaded by their selected TOC closure. No API mocks.
local function original(fn, suffix)
    assert(type(fn) == "function", "original consumer is not published: " .. suffix);
    local info = debug.getinfo(fn, "S");
    local source = info.source:gsub("\\", "/");
    assert(info.what == "Lua" and source:sub(-#suffix) == suffix,
        "consumer must come from selected original cache: " .. suffix);
end

if case == "smart-navigation-focus" then
    local suffix = "Blizzard_GamepadSmartNavigation/SmartNavigation.lua";
    original(SmartNavigationMixin.RunCurrentButtonEnterScript, suffix);
    original(SmartNavigationMixin.RunCurrentButtonLeaveScript, suffix);
    local button = CreateFrame("Button", nil, UIParent);
    button:EnableMouse(true);
    SmartNavigationMixin.RunCurrentButtonEnterScript({ currentButton = button });
    SmartNavigationMixin.RunCurrentButtonLeaveScript({ currentButton = button });
    -- No invented native focus/event order: only documented calls and receiver shape.
    return "original-focus-enter-exit-call-shape";
end

if case == "smart-navigation-mouse" then
    local suffix = "Blizzard_GamepadSmartNavigation/SmartNavigation.lua";
    original(SmartNavigationMixin.Click, suffix);
    original(SmartNavigationMixin.RightClick, suffix);
    assert(SmartNavigation, "original XML-selected navigator required");
    local button = CreateFrame("Button", nil, UIParent);
    button:EnableMouse(true);
    button:Show();
    SmartNavigation.currentButton = button;
    SmartNavigation.activeInfo = { frame = UIParent };
    assert(SmartNavigation_IsCurrentButtonClickable(), "click branch must be reached");
    SmartNavigationMixin.Click(SmartNavigation);
    SmartNavigationMixin.RightClick(SmartNavigation);
    return "original-mouse-down-up-call-shape;callback-order=native-unknown";
end

if case == "ghost-frame" then
    original(SetGhostFrameShown, "Blizzard_FrameXML/GhostFrame.lua");
    assert(GhostFrame, "original XML GhostFrame required");
    SetGhostFrameShown(true); -- Do not choose the native capability value.
    assert(type(CanPortGraveyard()) == "boolean", "documented non-null boolean");
    SetGhostFrameShown(false);
    assert(not GhostFrame:IsShown(), "false input remains hidden in original Lua");
    return "original-graveyard-query-shape;capability-value=native-unknown";
end

if case == "regional-name" then
    original(RecentAlliesUtil.GetBestDisplayNameForCharacter,
        "Blizzard_RecentAllies/Blizzard_RecentAlliesUtil.lua");
    assert(type(RegionalUniqueNamesEnabled()) == "boolean", "documented non-null boolean");
    local record = { name = "I4", fullName = "I4 Example" };
    local result = RecentAlliesUtil.GetBestDisplayNameForCharacter(record);
    assert(result == record.name or result == record.fullName,
        "original consumer must select a supplied name, without a native policy guess");
    return "original-name-selection-shape;policy=native-unknown";
end

if case == "collections-item-cooldown" then
    original(CollectionsSpellButton_UpdateCooldown,
        "Blizzard_Collections/Shared/Blizzard_CollectionTemplates.lua");
    local button = CreateFrame("Button", nil, UIParent);
    button.itemID = 6948; -- Concrete query input, not an assumed native item state.
    button.cooldown = CreateFrame("Cooldown", nil, button);
    CollectionsSpellButton_UpdateCooldown(button);
    local result = C_Spell.GetItemCooldown(button.itemID);
    assert(result == nil or type(result) == "table", "documented nullable record");
    if result ~= nil then
        assert(type(result.startTime) == "number" and type(result.duration) == "number");
        assert(type(result.isEnabled) == "boolean");
    end
    return "original-item-cooldown-nullable-shape;values=native-unknown";
end

if case == "retail-ghost-frame" then
    original(SetGhostFrameShown, "Blizzard_FrameXML/GhostFrame.lua");
    SetGhostFrameShown(true);
    assert(GhostFrame:IsShown(), "Retail original consumer has no PTR capability gate");
    SetGhostFrameShown(false);
    assert(not GhostFrame:IsShown());
    return "own-retail-original-ghost-visibility";
end

if case == "recent-allies-null-time" then
    original(RecentAlliesUtil.GetFormattedTime,
        "Blizzard_RecentAllies/Blizzard_RecentAlliesUtil.lua");
    assert(RecentAlliesUtil.GetFormattedTime(nil) == "");
    return "own-profile-original-null-time";
end

error("unknown original-consumer case: " .. tostring(case));
