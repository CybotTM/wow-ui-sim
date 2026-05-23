//! Default data for WorldState and PlayerState collections.
//!
//! Separated from state_types.rs to keep type definitions concise.

use super::state_types::*;
use std::collections::HashMap;

pub(super) fn default_equipped_items() -> HashMap<i32, EquippedItem> {
    let e = |item_id| EquippedItem {
        item_id,
        enchant_id: 0,
        gem_ids: [0, 0, 0],
    };
    [
        (1, e(211993)),  // Head: Entombed Seraph's Casque
        (2, e(230637)),  // Neck: Astral Gladiator's Amulet
        (3, e(211991)),  // Shoulder: Entombed Seraph's Plumes
        (5, e(211996)),  // Chest: Entombed Seraph's Breastplate
        (6, e(211990)),  // Waist: Entombed Seraph's Waistguard
        (7, e(211992)),  // Legs: Entombed Seraph's Greaves
        (8, e(211995)),  // Feet: Entombed Seraph's Sabatons
        (9, e(211989)),  // Wrist: Entombed Seraph's Shackles
        (10, e(211994)), // Hands: Entombed Seraph's Castigation
        (11, e(225748)), // Ring1: Seal of the Silent Vigil
        (12, e(215135)), // Ring2: Ring of Earthen Craftsmanship
        (13, e(218715)), // Trinket1: Forged Gladiator's Emblem
        (14, e(236914)), // Trinket2: Unbound Vision Journal
        (15, e(211988)), // Back: Entombed Seraph's Greatcloak
        (16, e(229181)), // MainHand: Ordained Forge Maul
    ]
    .into_iter()
    .collect()
}

pub(super) fn default_mounts() -> Vec<MountData> {
    let m = |mount_id, name: &str, spell_id, icon, collected, mount_type| MountData {
        mount_id,
        name: name.to_string(),
        spell_id,
        icon,
        is_collected: collected,
        is_usable: collected,
        mount_type,
    };
    vec![
        m(6, "Brown Horse", 458, 132261, true, 230), // Ground
        m(18, "Swift Palomino", 23338, 132261, true, 230), // Ground
        m(69, "Swift Stormsaber", 23338, 132226, true, 230), // Ground
        m(107, "Ashes of Al'ar", 40192, 132238, true, 248), // Flying
        m(219, "Invincible", 72286, 132226, true, 248), // Flying
        m(280, "Traveler's Tundra Mammoth", 61425, 236241, true, 230), // Ground (vendor)
        m(376, "Sandstone Drake", 93326, 656576, true, 248), // Flying (2-person)
        m(440, "Grand Expedition Yak", 122708, 646372, true, 230), // Ground (vendor)
        m(678, "Llothien Prowler", 171851, 1394955, true, 230), // Ground
        m(
            1039,
            "Mighty Caravan Brutosaur",
            264058,
            2399241,
            false,
            230,
        ), // Ground (AH mount)
    ]
}

pub(super) fn default_pets() -> Vec<PetData> {
    let mut id = 0u32;
    let p = |id: &mut u32,
             species: u32,
             name: &str,
             icon: u32,
             pet_type: i32,
             level: i32,
             quality: i32,
             collected: bool| {
        *id += 1;
        PetData {
            pet_id: format!("BattlePet-0-{:08X}", *id),
            species_id: species,
            name: name.to_string(),
            icon,
            pet_type,
            level,
            quality,
            is_collected: collected,
        }
    };
    vec![
        p(&mut id, 39, "Mechanical Squirrel", 132932, 9, 25, 3, true), // Mechanical
        p(&mut id, 87, "Phoenix Hatchling", 132837, 3, 25, 3, true),   // Elemental
        p(&mut id, 68, "Cat", 132576, 7, 1, 1, true),                  // Beast
        p(&mut id, 254, "Lil' Ragnaros", 134153, 3, 25, 4, true),      // Elemental (legendary)
        p(&mut id, 1266, "Xu-Fu, Cub of Xuen", 648459, 7, 25, 4, true), // Beast (legendary)
        p(&mut id, 630, "Clockwork Gnome", 425954, 9, 25, 3, true),    // Mechanical
        p(&mut id, 846, "Anubisath Idol", 607552, 10, 25, 3, true),    // Humanoid
        p(&mut id, 40, "Bombay Cat", 132576, 7, 1, 1, true),           // Beast
        p(&mut id, 1395, "Iron Starlette", 971374, 9, 25, 3, true),    // Mechanical
        p(&mut id, 2403, "Pocopoc", 4038816, 9, 25, 4, false),         // Mechanical (not collected)
    ]
}

pub(super) fn default_toys() -> Vec<ToyData> {
    let t = |item_id, name: &str, icon, collected| ToyData {
        item_id,
        name: name.to_string(),
        icon,
        is_collected: collected,
        is_usable: collected,
    };
    vec![
        t(166779, "Hearthstone Game Table", 2823166, true),
        t(13379, "Piccolo of the Flaming Fire", 134208, true),
        t(34480, "Romantic Picnic Basket", 236571, true),
        t(33927, "Brewfest Pony Keg", 132790, true),
        t(119210, "Hearthstone Board", 134457, true),
        t(69227, "Fool's Gold", 134112, true),
        t(88589, "Cremating Torch", 135432, true),
        t(86575, "Foxicopter Controller", 463485, true),
        t(104324, "Foot Ball", 620832, true),
        t(187421, "Ashen Liniment", 1500929, false),
    ]
}

pub(super) fn default_warband_scenes() -> Vec<WarbandSceneData> {
    let s = |warband_scene_id,
             name: &str,
             description: &str,
             source: &str,
             quality,
             texture_kit: &str,
             is_collected,
             is_favorite,
             has_fanfare,
             source_type| WarbandSceneData {
        warband_scene_id,
        name: name.to_string(),
        description: description.to_string(),
        source: source.to_string(),
        quality,
        texture_kit: texture_kit.to_string(),
        is_collected,
        is_favorite,
        has_fanfare,
        source_type,
    };

    vec![
        s(
            1,
            "Harbor Camp",
            "A windswept camp overlooking the harbor.",
            "Rewarded from the opening Warband questline.",
            2,
            "campcollection-bg-image1",
            true,
            true,
            false,
            1,
        ),
        s(
            2,
            "Evergreen Grove",
            "A quiet glade with druidic furnishings.",
            "Purchased from the Trading Post.",
            3,
            "campcollection-bg-image2",
            true,
            false,
            false,
            2,
        ),
        s(
            3,
            "Storm Camp",
            "A fortified site built for foul weather.",
            "Complete a seasonal event to unlock.",
            3,
            "campcollection-bg-image3",
            false,
            false,
            false,
            3,
        ),
        s(
            4,
            "Dragonflight Perch",
            "A high cliffside camp with dragonriding access.",
            "Earned from a meta achievement.",
            4,
            "campcollection-bg-image4",
            false,
            false,
            false,
            4,
        ),
    ]
}

/// Default transmog appearances: ~5 per armor slot + weapon slots.
///
/// Category IDs from Enum.TransmogCollectionType:
///   Head=1, Shoulder=2, Back=3, Chest=4, Shirt=5, Tabard=6, Wrist=7,
///   Hands=8, Waist=9, Legs=10, Feet=11, OneHSword=14, Staff=23, Shield=18
pub(super) fn default_transmog_appearances() -> Vec<TransmogAppearance> {
    let mut sid = 0i32; // auto-increment source_id
    let mut vid = 0i32; // auto-increment visual_id
    let a = |sid: &mut i32, vid: &mut i32, cat: i32, item: i32, collected: bool, src: i32| {
        *sid += 1;
        *vid += 1;
        TransmogAppearance {
            source_id: *sid,
            visual_id: *vid,
            category_id: cat,
            item_id: item,
            is_collected: collected,
            source_type: src,
            item_mod_id: 0,
        }
    };
    // Source types: JournalEncounter=1, Quest=2, Vendor=3, WorldDrop=4
    vec![
        // Head (cat 1)
        a(&mut sid, &mut vid, 1, 31110, true, 1), // Helm of Wrath
        a(&mut sid, &mut vid, 1, 34333, true, 1), // Onslaught Greathelm
        a(&mut sid, &mut vid, 1, 44006, true, 2), // Titan-forged Plate Helm
        a(&mut sid, &mut vid, 1, 77154, true, 1), // Colossal Dragonplate Helmet
        a(&mut sid, &mut vid, 1, 99999, false, 4), // Uncollected head piece
        // Shoulder (cat 2)
        a(&mut sid, &mut vid, 2, 30055, true, 1), // Pauldrons of the Fallen Champion
        a(&mut sid, &mut vid, 2, 34392, true, 1), // Onslaught Shoulderblades
        a(&mut sid, &mut vid, 2, 50853, true, 1), // Boneguard Commander's Pauldrons
        a(&mut sid, &mut vid, 2, 77166, true, 1), // Backbreaker Spaulders
        a(&mut sid, &mut vid, 2, 99998, false, 3), // Uncollected shoulders
        // Back (cat 3)
        a(&mut sid, &mut vid, 3, 27878, true, 1), // Cloak of the Pit Stalker
        a(&mut sid, &mut vid, 3, 34241, true, 1), // Cloak of Unforgivable Sin
        a(&mut sid, &mut vid, 3, 65108, true, 2), // Wrap of Unity
        a(&mut sid, &mut vid, 3, 77098, true, 4), // Cape of Vaulted Secrets
        a(&mut sid, &mut vid, 3, 99997, false, 4), // Uncollected cloak
        // Chest (cat 4)
        a(&mut sid, &mut vid, 4, 30129, true, 1), // Breastplate of the Lightbinder
        a(&mut sid, &mut vid, 4, 34215, true, 1), // Onslaught Breastplate
        a(&mut sid, &mut vid, 4, 50078, true, 1), // Thassarian's Battleplate
        a(&mut sid, &mut vid, 4, 77171, true, 1), // Colossal Dragonplate Battleplate
        a(&mut sid, &mut vid, 4, 99996, false, 3), // Uncollected chest
        // Shirt (cat 5)
        a(&mut sid, &mut vid, 5, 16059, true, 3), // Common Brown Shirt
        a(&mut sid, &mut vid, 5, 4330, true, 3),  // Stylish Red Shirt
        // Tabard (cat 6)
        a(&mut sid, &mut vid, 6, 5976, true, 3), // Guild Tabard
        // Wrist (cat 7)
        a(&mut sid, &mut vid, 7, 30067, true, 1), // Bracers of Maliciousness
        a(&mut sid, &mut vid, 7, 34431, true, 1), // Wristguards of Tranquil Thought
        a(&mut sid, &mut vid, 7, 50611, true, 1), // Bracers of Dark Reckoning
        a(&mut sid, &mut vid, 7, 77162, true, 4), // Dragonbone Wargreaves
        a(&mut sid, &mut vid, 7, 99995, false, 4), // Uncollected bracers
        // Hands (cat 8)
        a(&mut sid, &mut vid, 8, 30113, true, 1), // Gloves of the Fallen Defender
        a(&mut sid, &mut vid, 8, 34342, true, 1), // Handguards of the Dawn
        a(&mut sid, &mut vid, 8, 50610, true, 1), // Gauntlets of Overexposure
        a(&mut sid, &mut vid, 8, 77160, true, 1), // Colossal Dragonplate Gauntlets
        a(&mut sid, &mut vid, 8, 99994, false, 2), // Uncollected gloves
        // Waist (cat 9)
        a(&mut sid, &mut vid, 9, 30034, true, 1), // Belt of One-Hundred Deaths
        a(&mut sid, &mut vid, 9, 34485, true, 1), // Girdle of the Fearless
        a(&mut sid, &mut vid, 9, 50069, true, 1), // Belt of Broken Bones
        a(&mut sid, &mut vid, 9, 77164, true, 4), // Gorge Stalker Belt
        a(&mut sid, &mut vid, 9, 99993, false, 3), // Uncollected belt
        // Legs (cat 10)
        a(&mut sid, &mut vid, 10, 30126, true, 1), // Leggings of the Fallen
        a(&mut sid, &mut vid, 10, 34180, true, 1), // Onslaught Legguards
        a(&mut sid, &mut vid, 10, 50063, true, 1), // Legguards of Lost Hope
        a(&mut sid, &mut vid, 10, 77155, true, 1), // Colossal Dragonplate Legguards
        a(&mut sid, &mut vid, 10, 99992, false, 2), // Uncollected legs
        // Feet (cat 11)
        a(&mut sid, &mut vid, 11, 30032, true, 1), // Red Riding Boots
        a(&mut sid, &mut vid, 11, 34567, true, 1), // Boots of the Protector
        a(&mut sid, &mut vid, 11, 50607, true, 1), // Treads of Impending Resurrection
        a(&mut sid, &mut vid, 11, 77159, true, 4), // Mosshide Treads
        a(&mut sid, &mut vid, 11, 99991, false, 4), // Uncollected boots
        // One-Hand Sword (cat 14)
        a(&mut sid, &mut vid, 14, 28573, true, 1), // Despair
        a(&mut sid, &mut vid, 14, 34247, true, 1), // Apolyon, the Soul-Render
        a(&mut sid, &mut vid, 14, 50070, true, 1), // Glorenzelg, High-Blade of the Silver Hand
        a(&mut sid, &mut vid, 14, 77188, true, 1), // Gurthalak, Voice of the Deeps
        a(&mut sid, &mut vid, 14, 99990, false, 1), // Uncollected sword
        // Staff (cat 23)
        a(&mut sid, &mut vid, 23, 30723, true, 1), // Staff of Infinite Mysteries
        a(&mut sid, &mut vid, 23, 34182, true, 1), // Grand Magister's Staff of Torrents
        a(&mut sid, &mut vid, 23, 50731, true, 4), // Archus, Greatstaff of Antonidas
        a(&mut sid, &mut vid, 23, 77196, true, 1), // Ti'tahk, the Steps of Time
        a(&mut sid, &mut vid, 23, 99989, false, 4), // Uncollected staff
        // Shield (cat 18)
        a(&mut sid, &mut vid, 18, 28606, true, 1), // Shield of Impenetrable Darkness
        a(&mut sid, &mut vid, 18, 34185, true, 1), // Sword Breaker's Bulwark
        a(&mut sid, &mut vid, 18, 50729, true, 1), // Icecrown Glacial Wall
        a(&mut sid, &mut vid, 18, 77167, true, 4), // Blackhorn's Mighty Bulwark
        a(&mut sid, &mut vid, 18, 99988, false, 3), // Uncollected shield
    ]
}

pub(super) fn default_heirlooms() -> Vec<HeirloomData> {
    let h = |id, name: &str, loc: &str, icon, lvl, src: &str, min, max| HeirloomData {
        item_id: id,
        name: name.into(),
        equip_loc: loc.into(),
        icon,
        upgrade_level: lvl,
        source: src.into(),
        min_level: min,
        max_level: max,
    };
    vec![
        h(
            122245,
            "Burnished Helm of Might",
            "INVTYPE_HEAD",
            133071,
            6,
            "Vendor",
            1,
            50,
        ),
        h(
            122355,
            "Polished Breastplate of Valor",
            "INVTYPE_CHEST",
            132633,
            6,
            "Vendor",
            1,
            50,
        ),
        h(
            122356,
            "Polished Spaulders of Valor",
            "INVTYPE_SHOULDER",
            132633,
            6,
            "Vendor",
            1,
            50,
        ),
        h(
            122382,
            "Preened Ironfeather Shoulders",
            "INVTYPE_SHOULDER",
            135033,
            6,
            "Vendor",
            1,
            50,
        ),
        h(
            122384,
            "Tattered Dreadmist Robe",
            "INVTYPE_CHEST",
            132673,
            6,
            "Vendor",
            1,
            50,
        ),
        h(
            122247,
            "Burnished Legplates of Might",
            "INVTYPE_LEGS",
            133071,
            6,
            "Vendor",
            1,
            50,
        ),
        h(
            122250,
            "Mystical Kilt of Elements",
            "INVTYPE_LEGS",
            134188,
            6,
            "Vendor",
            1,
            50,
        ),
        h(
            122266,
            "Balanced Heartseeker",
            "INVTYPE_WEAPON",
            135274,
            6,
            "Vendor",
            1,
            50,
        ),
        h(
            122389,
            "Bloodied Arcanite Reaper",
            "INVTYPE_2HWEAPON",
            135277,
            6,
            "Vendor",
            1,
            50,
        ),
        h(
            122390,
            "Dignified Headmaster's Charge",
            "INVTYPE_2HWEAPON",
            135146,
            6,
            "Vendor",
            1,
            50,
        ),
        h(
            187997,
            "Eternal Amulet of the Redeemed",
            "INVTYPE_NECK",
            133280,
            0,
            "Vendor",
            1,
            50,
        ),
    ]
}

pub(super) fn default_premade_listings() -> Vec<PremadeListing> {
    use std::collections::HashMap;

    /// `Enum.LFGEntryGeneralPlaystyle` codes as named constants — the
    /// raw numbers are otherwise opaque in the seed table.
    const PLAYSTYLE_LEARNING: i32 = 1;
    const PLAYSTYLE_FUN_RELAXED: i32 = 2;
    const PLAYSTYLE_FUN_SERIOUS: i32 = 3;

    /// Faction id encoding for `leaderFactionGroup` (`PLAYER_FACTION_GROUP`):
    /// 0 = Horde, 1 = Alliance.
    const FACTION_HORDE: i32 = 0;
    const FACTION_ALLIANCE: i32 = 1;

    let mut id = 0u32;
    let l = |id: &mut u32,
             name: &str,
             comment: &str,
             leader: &str,
             activity: u32,
             tanks: i32,
             healers: i32,
             damagers: i32,
             max: i32,
             playstyle: i32,
             faction: i32| {
        *id += 1;
        PremadeListing {
            search_result_id: *id,
            name: name.to_string(),
            comment: comment.to_string(),
            leader_name: leader.to_string(),
            activity_id: activity,
            num_members: tanks + healers + damagers,
            max_members: max,
            voice_chat: String::new(),
            auto_accept: false,
            is_delisted: false,
            party_guid: format!("Party-3-0000-1234-{:08X}", *id),
            tanks,
            healers,
            damagers,
            no_role: 0,
            classes_by_role: HashMap::new(),
            general_playstyle: playstyle,
            cross_faction_listing: false,
            leader_faction_group: faction,
            num_bnet_friends: 0,
            num_char_friends: 0,
            num_guild_mates: 0,
        }
    };
    vec![
        l(
            &mut id,
            "+15 Mists chill run",
            "Know mechanics, 2.5k io",
            "Thrallx",
            1195,
            1,
            1,
            1,
            5,
            PLAYSTYLE_FUN_SERIOUS,
            FACTION_HORDE,
        ),
        l(
            &mut id,
            "+12 Siege weekly",
            "Weekly key, all welcome",
            "Jainavx",
            1188,
            0,
            1,
            1,
            5,
            PLAYSTYLE_FUN_RELAXED,
            FACTION_ALLIANCE,
        ),
        l(
            &mut id,
            "Nerub-ar Palace HC fresh",
            "AOTC prog, be geared",
            "Anduin",
            1296,
            2,
            3,
            7,
            20,
            PLAYSTYLE_FUN_SERIOUS,
            FACTION_ALLIANCE,
        ),
        l(
            &mut id,
            "Nerub-ar Palace N learn",
            "Learning run, patient",
            "Sylvanas",
            1295,
            2,
            2,
            4,
            20,
            PLAYSTYLE_LEARNING,
            FACTION_HORDE,
        ),
        l(
            &mut id,
            "World Boss — Aggregation",
            "Quick kill, summon up",
            "Khadgar",
            1350,
            3,
            4,
            11,
            40,
            PLAYSTYLE_FUN_RELAXED,
            FACTION_ALLIANCE,
        ),
        l(
            &mut id,
            "2v2 Arena chill",
            "Just capping",
            "Garrosh",
            491,
            0,
            0,
            1,
            2,
            PLAYSTYLE_FUN_RELAXED,
            FACTION_HORDE,
        ),
        l(
            &mut id,
            "RBG yolo",
            "Casual RBG, no rage",
            "Velen",
            493,
            1,
            2,
            4,
            10,
            PLAYSTYLE_FUN_RELAXED,
            FACTION_ALLIANCE,
        ),
        l(
            &mut id,
            "WQ group Ringing Deeps",
            "Doing WQs together",
            "Malfurion",
            1700,
            0,
            0,
            3,
            5,
            PLAYSTYLE_FUN_RELAXED,
            FACTION_ALLIANCE,
        ),
    ]
}
