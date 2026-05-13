//! Item-key round-trip coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const ITEM_KEY_ROUND_TRIP_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local itemID = 210935
local itemLevel = 70
local key = C_AuctionHouse.MakeItemKey(itemID, itemLevel)

expect(type(key) == "table", "MakeItemKey must return a table")
expect(key.itemID == itemID, "itemID must round-trip")
expect(key.itemLevel == itemLevel, "itemLevel must round-trip")
expect(key.itemSuffix == 0, "itemSuffix must default to 0")
expect(key.battlePetSpeciesID == 0, "battlePetSpeciesID must default to 0")

local fieldCount = 0
for fieldName in pairs(key) do
    fieldCount = fieldCount + 1
    expect(fieldName == "itemID" or
           fieldName == "itemLevel" or
           fieldName == "itemSuffix" or
           fieldName == "battlePetSpeciesID",
           "unexpected ItemKey field " .. tostring(fieldName))
end
expect(fieldCount == 4, "ItemKey must expose exactly 4 fields")

local info = C_AuctionHouse.GetItemKeyInfo(key)
expect(type(info) == "table", "GetItemKeyInfo must return a table")
if type(info) == "table" then
    expect(type(info.itemName) == "string" and info.itemName ~= "",
           "itemName must be populated")
    expect(type(info.iconFileID) == "number" and info.iconFileID > 0,
           "iconFileID must be populated")
    expect(type(info.quality) == "number" and info.quality >= 0,
           "quality must be populated")
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_item_key_round_trips_to_item_key_info() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(ITEM_KEY_ROUND_TRIP_PROBE_LUA)
                    .expect("AuctionHouse item-key round-trip probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` item-key round-trip failures:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` item-key round-trip probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
