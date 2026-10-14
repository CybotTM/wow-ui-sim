-- Simulator policy inferred from cached original consumers, not native declarations.
-- Prefix this whole chunk with: local store_group_mode = "records" (Retail/PTR)
-- or: local store_group_mode = "ids" (Mists). No StoreFrame required.
local mode = store_group_mode
assert(mode == "records" or mode == "ids", "explicit store_group_mode required")
local groups = C_StoreSecure.GetProductGroups()
assert(type(groups) == "table", "GetProductGroups must return a list")
assert(#groups == 1, "fresh demo must expose exactly one group")
local first = groups[1]
local groupID
if mode == "ids" then
    -- Deliberately first: today's record-list producer must fail before lookups.
    assert(type(first) == "number", "Mists GetProductGroups[1] must be numeric")
    groupID = first
else
    assert(type(first) == "table", "Retail/PTR GetProductGroups[1] must be a record")
    assert(type(first.groupID) == "number", "record.groupID must be numeric")
    assert(first.parentGroupID == 0, "featured record must retain parentGroupID=0")
    groupID = first.groupID
end
assert(groupID == 501, "fresh featured group must be 501")
local groupInfo = C_StoreSecure.GetProductGroupInfo(groupID)
assert(type(groupInfo) == "table", "returned group ID must resolve group info")
assert(groupInfo.groupName == "Featured", "group 501 must be Featured")
local products = C_StoreSecure.GetProducts(groupID)
assert(type(products) == "table", "returned group ID must resolve product list")
assert(#products == 1, "fresh featured group must contain one entry")
assert(products[1] == 1003, "featured entry must be 1003")
local entry = C_StoreSecure.GetEntryInfo(products[1])
assert(type(entry) == "table", "returned entry ID must resolve entry info")
assert(entry.entryID == 1003, "entry info must retain entry ID")
assert(entry.productID == 2003, "entry info must retain product ID")
assert(type(entry.sharedData) == "table", "entry shared data must exist")
assert(entry.sharedData.name == "Apprentice Rider Bundle", "demo entry name must match")
return true
