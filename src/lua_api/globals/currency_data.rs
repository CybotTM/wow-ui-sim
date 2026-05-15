//! Currency data for the WoW UI simulator.
//!
//! Provides a static list of currencies with quantities for the token frame UI.
//! The list is hierarchical: headers group currencies into categories.

/// A currency entry in the currency list.
pub struct CurrencyEntry {
    pub currency_id: i32,
    pub name: &'static str,
    pub quantity: i32,
    pub max_quantity: i32,
    pub icon_file_id: u32,
    pub quality: i32,
    pub is_header: bool,
    pub is_header_expanded: bool,
    pub depth: i32,
    pub is_discovered: bool,
    pub is_show_in_backpack: bool,
}

const fn header(name: &'static str) -> CurrencyEntry {
    CurrencyEntry {
        currency_id: 0,
        name,
        quantity: 0,
        max_quantity: 0,
        icon_file_id: 0,
        quality: 0,
        is_header: true,
        is_header_expanded: true,
        depth: 0,
        is_discovered: true,
        is_show_in_backpack: false,
    }
}

const fn currency(
    currency_id: i32,
    name: &'static str,
    quantity: i32,
    max_quantity: i32,
    icon_file_id: u32,
    quality: i32,
) -> CurrencyEntry {
    CurrencyEntry {
        currency_id,
        name,
        quantity,
        max_quantity,
        icon_file_id,
        quality,
        is_header: false,
        is_header_expanded: false,
        depth: 1,
        is_discovered: true,
        is_show_in_backpack: false,
    }
}

const fn watched(mut c: CurrencyEntry) -> CurrencyEntry {
    c.is_show_in_backpack = true;
    c
}

/// Static currency list (headers + entries).
static CURRENCY_LIST: &[CurrencyEntry] = &[
    header("The War Within"),
    watched(currency(2245, "Valorstones", 1847, 0, 5868905, 3)),
    currency(2806, "Weathered Harbinger Crest", 42, 90, 5868904, 2),
    currency(2807, "Carved Harbinger Crest", 15, 90, 5868906, 3),
    currency(2809, "Runed Harbinger Crest", 3, 90, 5868907, 4),
    watched(currency(3089, "Resonance Crystals", 620, 4000, 3528287, 3)),
    header("Player vs. Player"),
    watched(currency(1792, "Honor", 4350, 15000, 1140617, 0)),
    currency(1602, "Conquest", 880, 0, 1140616, 0),
    header("Miscellaneous"),
    currency(1191, "Valor", 0, 0, 5868908, 0),
    currency(1813, "Reservoir Anima", 23500, 35000, 3528287, 0),
    currency(1767, "Stygia", 140, 0, 134418, 0),
    currency(824, "Garrison Resources", 0, 0, 1042294, 0),
    currency(1101, "Oil", 0, 0, 1391724, 0),
];

/// Additional live currency IDs referenced by popular addons but not shown in
/// the small default currency list. Names are enough for addon startup paths
/// that sort or label historical currency IDs.
static SUPPLEMENTAL_CURRENCY_NAMES: &[(i32, &str)] = &[
    (81, "Epicurean Award"),
    (515, "Darkmoon Prize Ticket"),
    (2588, "Riders of Azeroth Badge"),
    (3363, "Community Coupons"),
    (241, "Champion's Seal"),
    (391, "Tol Barad Commendation"),
    (416, "Mark of the World Tree"),
    (402, "Ironpaw Token"),
    (697, "Elder Charm of Good Fortune"),
    (738, "Lesser Charm of Good Fortune"),
    (752, "Mogu Rune of Fate"),
    (776, "Warforged Seal"),
    (777, "Timeless Coin"),
    (789, "Bloody Coin"),
    (823, "Apexis Crystal"),
    (824, "Garrison Resources"),
    (994, "Seal of Tempered Fate"),
    (1101, "Oil"),
    (1129, "Seal of Inevitable Fate"),
    (1149, "Sightless Eye"),
    (1155, "Ancient Mana"),
    (1166, "Timewarped Badge"),
    (1220, "Order Resources"),
    (1226, "Nethershards"),
    (1273, "Seal of Broken Fate"),
    (1275, "Curious Coin"),
    (1299, "Brawler's Gold"),
    (1314, "Lingering Soul Fragment"),
    (1342, "Legionfall War Supplies"),
    (1501, "Writhing Essence"),
    (1508, "Veiled Argunite"),
    (1533, "Wakening Essence"),
    (1710, "Seafarer's Dubloon"),
    (1580, "Seal of Wartorn Fate"),
    (1587, "War Supplies"),
    (1716, "Honorbound Service Medal"),
    (1717, "7th Legion Service Medal"),
    (1718, "Titan Residuum"),
    (1721, "Prismatic Manapearl"),
    (1719, "Corrupted Memento"),
    (1755, "Coalescing Visions"),
    (1803, "Echoes of Ny'alotha"),
    (1754, "Argent Commendation"),
    (1191, "Valor"),
    (1602, "Conquest"),
    (1792, "Honor"),
    (1822, "Renown"),
    (1767, "Stygia"),
    (1828, "Soul Ash"),
    (1810, "Redeemed Soul"),
    (1813, "Reservoir Anima"),
    (1816, "Sinstone Fragments"),
    (1819, "Medallion of Service"),
    (1820, "Infused Ruby"),
    (1885, "Grateful Offering"),
    (1889, "Adventure Campaign Progress"),
    (1904, "Tower Knowledge"),
    (1906, "Soul Cinders"),
    (1931, "Cataloged Research"),
    (1977, "Stygian Ember"),
    (1979, "Cyphers of the First Ones"),
    (2009, "Cosmic Flux"),
    (2000, "Motes of Fate"),
    (2003, "Dragon Isles Supplies"),
    (2245, "Flightstones"),
    (2123, "Bloody Tokens"),
    (2797, "Trophy of Strife"),
    (2045, "Dragon Glyph Embers"),
    (2118, "Elemental Overflow"),
    (2122, "Storm Sigil"),
    (2409, "Whelpling Crest Fragment Tracker [DNT]"),
    (2410, "Drake Crest Fragment Tracker [DNT]"),
    (2411, "Wyrm Crest Fragment Tracker [DNT]"),
    (2412, "Aspect Crest Fragment Tracker [DNT]"),
    (
        2413,
        "10.1 Professions - Personal Tracker - S2 Spark Drops (Hidden)",
    ),
    (2533, "Renascent Shadowflame"),
    (2594, "Paracausal Flakes"),
    (2650, "Emerald Dewdrop"),
    (2651, "Seedbloom"),
    (2777, "Dream Infusion"),
    (2796, "Renascent Dream"),
    (2706, "Whelpling's Dreaming Crest"),
    (2707, "Drake's Dreaming Crest"),
    (2708, "Wyrm's Dreaming Crest"),
    (2709, "Aspect's Dreaming Crest"),
    (
        2774,
        "10.2 Professions - Personal Tracker - S3 Spark Drops (Hidden)",
    ),
    (2657, "Mysterious Fragment"),
    (2912, "Renascent Awakening"),
    (2806, "Whelpling's Awakened Crest"),
    (2807, "Drake's Awakened Crest"),
    (2809, "Wyrm's Awakened Crest"),
    (2812, "Aspect's Awakened Crest"),
    (
        2800,
        "10.2.6 Professions - Personal Tracker - S4 Spark Drops (Hidden)",
    ),
    (
        3010,
        "10.2.6 Rewards - Personal Tracker - S4 Dinar Drops (Hidden)",
    ),
    (2778, "Bronze"),
    (3089, "Residual Memories"),
    (2803, "Undercoin"),
    (2815, "Resonance Crystals"),
    (3056, "Kej"),
    (3008, "Valorstones"),
    (2813, "Harmonized Silk"),
    (2914, "Weathered Harbinger Crest"),
    (2915, "Carved Harbinger Crest"),
    (2916, "Runed Harbinger Crest"),
    (2917, "Gilded Harbinger Crest"),
    (
        3023,
        "11.0 Professions - Personal Tracker - S1 Spark Drops (Hidden)",
    ),
    (3100, "Bronze Celebration Token"),
    (3090, "Flame-Blessed Iron"),
    (3218, "Empty Kaja'Cola Can"),
    (3220, "Vintage Kaja'Cola Can"),
    (3226, "Market Research"),
    (3116, "Essence of Kaja'mite"),
    (3107, "Weathered Undermine Crest"),
    (3108, "Carved Undermine Crest"),
    (3109, "Runed Undermine Crest"),
    (3110, "Gilded Undermine Crest"),
    (
        3132,
        "11.1 Professions - Personal Tracker - S2 Spark Drops (Hidden)",
    ),
    (3149, "Displaced Corrupted Mementos"),
    (3278, "Ethereal Strands"),
    (3303, "Untethered Coin"),
    (3356, "Untainted Mana-Crystals"),
    (3269, "Ethereal Voidsplinter"),
    (3284, "Weathered Ethereal Crest"),
    (3286, "Carved Ethereal Crest"),
    (3288, "Runed Ethereal Crest"),
    (3290, "Gilded Ethereal Crest"),
    (3141, "Starlight Spark Dust"),
    (3319, "Twilight's Blade Insignia"),
    (3316, "Voidlight Marl"),
    (3376, "Shard of Dundun"),
    (3377, "Unalloyed Abundance"),
    (3379, "Brimming Arcana"),
    (3385, "Luminous Dust"),
    (3392, "Remnant of Anguish"),
    (3400, "Uncontaminated Void Sample"),
    (3373, "Angler Pearls"),
    (3393, "Illusionary Coin"),
    (3405, "Field Accolade"),
    (3256, "Artisan Alchemist's Moxie"),
    (3257, "Artisan Blacksmith's Moxie"),
    (3258, "Artisan Enchanter's Moxie"),
    (3259, "Artisan Engineer's Moxie"),
    (3260, "Artisan Herbalist's Moxie"),
    (3261, "Artisan Scribe's Moxie"),
    (3262, "Artisan Jewelcrafter's Moxie"),
    (3263, "Artisan Leatherworker's Moxie"),
    (3264, "Artisan Miner's Moxie"),
    (3265, "Artisan Skinner's Moxie"),
    (3266, "Artisan Tailor's Moxie"),
    (3028, "Restored Coffer Key"),
    (3310, "Coffer Key Shards"),
    (3212, "Radiant Spark Dust"),
    (3378, "Dawnlight Manaflux"),
    (3383, "Adventurer Dawncrest"),
    (3341, "Veteran Dawncrest"),
    (3343, "Champion Dawncrest"),
    (3345, "Hero Dawncrest"),
    (3347, "Myth Dawncrest"),
    (3418, "Nebulous Voidcore"),
];

/// Number of items in the currency list.
pub fn currency_list_size() -> i32 {
    CURRENCY_LIST.len() as i32
}

/// Get a currency list entry by 1-based index.
pub fn get_currency_list_entry(index: i32) -> Option<&'static CurrencyEntry> {
    CURRENCY_LIST.get((index - 1) as usize)
}

/// Get currency info by currency ID.
pub fn get_currency_by_id(currency_id: i32) -> Option<&'static CurrencyEntry> {
    CURRENCY_LIST
        .iter()
        .find(|c| !c.is_header && c.currency_id == currency_id)
}

/// Backpack (watched) currencies, returned as (index, entry) pairs.
pub fn backpack_currencies() -> impl Iterator<Item = &'static CurrencyEntry> {
    CURRENCY_LIST
        .iter()
        .filter(|c| c.is_show_in_backpack && !c.is_header)
}

/// Build the initial `SimState.currency_info` map by projecting each
/// non-header `CurrencyEntry` into a `CurrencyInfo`. Non-seeded fields
/// (weekly caps, transfer metadata, etc.) default to 0 / false so the
/// map still drives `C_CurrencyInfo.GetCurrencyInfo` for the commonly-
/// referenced ids in `CURRENCY_LIST`.
pub fn seeded_currency_info_map()
-> std::collections::HashMap<i32, crate::lua_api::state::CurrencyInfo> {
    use crate::lua_api::state::CurrencyInfo;
    let mut currencies = CURRENCY_LIST
        .iter()
        .filter(|c| !c.is_header)
        .map(|c| {
            (
                c.currency_id,
                CurrencyInfo {
                    currency_id: c.currency_id,
                    name: c.name.to_string(),
                    icon_file_id: c.icon_file_id,
                    quantity: c.quantity,
                    max_quantity: c.max_quantity,
                    quality: c.quality,
                    is_show_in_backpack: c.is_show_in_backpack,
                    discovered: c.is_discovered,
                    currency_list_depth: c.depth,
                    ..CurrencyInfo::default()
                },
            )
        })
        .collect::<std::collections::HashMap<_, _>>();

    for &(currency_id, name) in SUPPLEMENTAL_CURRENCY_NAMES {
        currencies
            .entry(currency_id)
            .or_insert_with(|| CurrencyInfo {
                currency_id,
                name: name.to_string(),
                discovered: true,
                ..CurrencyInfo::default()
            });
    }

    currencies
}
