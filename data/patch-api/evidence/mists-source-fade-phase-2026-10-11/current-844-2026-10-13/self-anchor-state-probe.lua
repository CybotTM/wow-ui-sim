local hit
local frame = EnumerateFrames()
while frame do
    local name = frame:GetDebugName()
    if name and name:find("Blizzard_UIPanels_Game_20279", 1, true) then
        assert(hit == nil, "ambiguous generated frame identity")
        hit = frame
    end
    frame = EnumerateFrames(frame)
end
assert(hit, "failed generated frame not enumerable")
local parent = hit:GetParent()
print("SELF_ANCHOR_FRAME", hit:GetDebugName(), hit:GetName(), hit:GetObjectType(), hit:GetParentKey(), parent and parent:GetDebugName())
if parent then
    for key, value in pairs(parent) do
        if value == hit then
            print("SELF_ANCHOR_PARENT_KEY", tostring(key))
        end
    end
end
for _, key in ipairs({"Background", "Name", "SubTitle", "Shadow", "Icon", "IconBorder"}) do
    local child = hit[key]
    print("SELF_ANCHOR_CHILD", key, type(child), child and child:GetDebugName(), child and child:GetObjectType(), child and child:GetParentKey(), child and child:GetParent() == hit)
    if child then
        print("SELF_ANCHOR_CHILD_SIZE", key, child:GetSize())
        for index = 1, child:GetNumPoints() do
            local point, relative, relativePoint, x, y = child:GetPoint(index)
            print("SELF_ANCHOR_CHILD_POINT", key, index, point, relative and relative:GetDebugName(), relative == child, relative == hit, relativePoint, x, y)
        end
    end
end
print("SELF_ANCHOR_STATE_PROBE_DONE")
