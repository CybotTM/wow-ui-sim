//! 12340 enUS DBC-backed catalog; native ordering is not subclass-ID order.
#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn auction_original_380_returns_native_class_order() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        auctionUpdates = 0
        local watcher = CreateFrame('Frame')
        watcher:RegisterEvent('AUCTION_ITEM_LIST_UPDATE')
        watcher:SetScript('OnEvent', function() auctionUpdates = auctionUpdates + 1 end)
        function AuctionFrameBrowse_InitClasses(...)
            classNames = {...}; classCount = select('#', ...)
        end
    "#,
    )
    .unwrap();
    env.exec(&original_lines("Blizzard_AuctionUI", 380, 380))
        .unwrap();
    env.exec(r#"
        assert(classCount == 12)
        assert(table.concat(classNames, '|') ==
            'Weapon|Armor|Container|Consumable|Glyph|Trade Goods|Projectile|Quiver|Recipe|Gem|Miscellaneous|Quest')
        assert(select('#', GetAuctionItemClasses(false, {})) == 12)
        assert(auctionUpdates == 0)
    "#).unwrap();
}

#[test]
fn auction_subclasses_preserve_dbc_order_and_display_flags() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local consumables = {GetAuctionItemSubClasses('4')}
        assert(#consumables == 9, 'expected nine consumable names, got '..#consumables)
        -- Physical subclass IDs: 0,5,1,2,3,7,6,4,8, not sorted IDs.
        assert(table.concat(consumables, '|') ==
            'Consumable|Food & Drink|Potion|Elixir|Flask|Bandage|Item Enhancement|Scroll|Other')
        local armor = {GetAuctionItemSubClasses(2)}
        assert(table.concat(armor, '|') ==
            'Miscellaneous|Cloth|Leather|Mail|Plate|Shields|Librams|Idols|Totems|Sigils')
        local weapons = {GetAuctionItemSubClasses(1)}
        assert(table.concat(weapons, '|') ==
            'One-Handed Axes|Two-Handed Axes|Bows|Guns|One-Handed Maces|Two-Handed Maces|Polearms|One-Handed Swords|Two-Handed Swords|Staves|Fist Weapons|Miscellaneous|Daggers|Thrown|Crossbows|Wands|Fishing Poles')
    "#).unwrap();
}

#[test]
fn auction_inventory_pairs_match_native_masks_and_unfound_ordinal() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local tokens = {
            'INVTYPE_HEAD', 'INVTYPE_NECK', 'INVTYPE_SHOULDER', 'INVTYPE_BODY',
            'INVTYPE_CHEST', 'INVTYPE_WAIST', 'INVTYPE_LEGS', 'INVTYPE_FEET',
            'INVTYPE_WRIST', 'INVTYPE_HAND', 'INVTYPE_FINGER', 'INVTYPE_TRINKET',
            'INVTYPE_CLOAK', 'INVTYPE_HOLDABLE',
        }
        local function pack(...) return {n=select('#', ...), ...} end
        local function check(class, subclass, available)
            local actual = pack(GetAuctionInvTypes(class, subclass))
            assert(actual.n == 28, tostring(actual.n))
            local enabled = {}
            for _, token in ipairs(available) do enabled[token] = true end
            for i, token in ipairs(tokens) do
                assert(actual[2*i-1] == token)
                local expected = enabled[token] and 1 or nil
                assert(actual[2*i] == expected, token..': '..tostring(actual[2*i]))
            end
        end
        check(2, 1, {'INVTYPE_HEAD', 'INVTYPE_NECK', 'INVTYPE_BODY',
            'INVTYPE_FINGER', 'INVTYPE_TRINKET', 'INVTYPE_HOLDABLE'})
        local regular = {'INVTYPE_HEAD', 'INVTYPE_SHOULDER', 'INVTYPE_CHEST',
            'INVTYPE_WAIST', 'INVTYPE_LEGS', 'INVTYPE_FEET', 'INVTYPE_WRIST', 'INVTYPE_HAND'}
        check(2, 3, regular)
        -- An unfound ordinal intentionally falls through; it is not an error/empty result.
        check(2, 999, regular)
        local cloth = {'INVTYPE_HEAD', 'INVTYPE_SHOULDER', 'INVTYPE_CHEST',
            'INVTYPE_WAIST', 'INVTYPE_LEGS', 'INVTYPE_FEET', 'INVTYPE_WRIST',
            'INVTYPE_HAND', 'INVTYPE_CLOAK'}
        check('2', '2', cloth)
        check(1, 999, tokens)
        assert(select('#', GetAuctionInvTypes(1, 1)) == 0)
        assert(select('#', GetAuctionInvTypes(2, 6)) == 0)
    "#,
    )
    .unwrap();
}

#[test]
fn auction_numeric_validation_and_zero_returns() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local function usage(f, expected, ...)
            local ok, err = pcall(f, ...)
            assert(not ok and string.find(err, expected, 1, true), tostring(err))
        end
        local subUsage = 'Usage: GetAuctionItemSubClasses(index)'
        local invUsage = 'Usage: GetAuctionInvTypes(classIndex, subClassIndex)'
        usage(GetAuctionItemSubClasses, subUsage)
        for _, bad in ipairs({false, true, {}, 'bad'}) do
            usage(GetAuctionItemSubClasses, subUsage, bad)
            usage(GetAuctionInvTypes, invUsage, bad, 1)
            usage(GetAuctionInvTypes, invUsage, 1, bad)
        end
        usage(GetAuctionInvTypes, invUsage, 1)
        usage(GetAuctionInvTypes, invUsage, 12, false)
        for _, class in ipairs({0, -1, 13, 999, '12'}) do
            assert(select('#', GetAuctionItemSubClasses(class)) == 0)
            assert(select('#', GetAuctionInvTypes(class, 1)) == 0)
        end
    "#,
    )
    .unwrap();
}
