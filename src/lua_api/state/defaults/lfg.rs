use super::*;

pub(in crate::lua_api::state) fn default_lfg_category_info()
-> std::collections::HashMap<i32, LfgCategoryInfo> {
    let cat = |name: &str, order: i32, allow_cross_faction: bool| -> LfgCategoryInfo {
        LfgCategoryInfo {
            name: name.into(),
            order,
            separate_recommended: false,
            prefer_current_area: false,
            allow_cross_faction,
            auto_choose_activity: false,
            show_playstyle_dropdown: false,
        }
    };
    let mut map = std::collections::HashMap::new();
    map.insert(2, cat("Dungeons", 1, true));
    map.insert(3, cat("Raids", 2, true));
    map.insert(4, cat("Arenas", 3, false));
    map.insert(6, cat("Custom", 4, true));
    map.insert(9, cat("Battlegrounds", 5, false));
    map
}

pub(in crate::lua_api::state) fn default_lfg_activity_groups() -> Vec<LfgActivityGroupInfo> {
    vec![
        LfgActivityGroupInfo {
            group_id: 295,
            category_id: 2,
            name: "The War Within Mythic+".into(),
            order_index: 1,
            filters: 1, // PvE
        },
        LfgActivityGroupInfo {
            group_id: 296,
            category_id: 2,
            name: "The War Within Heroic Dungeons".into(),
            order_index: 2,
            filters: 1,
        },
        LfgActivityGroupInfo {
            group_id: 320,
            category_id: 3,
            name: "The War Within Raids".into(),
            order_index: 1,
            filters: 1,
        },
        LfgActivityGroupInfo {
            group_id: 350,
            category_id: 4,
            name: "Arenas".into(),
            order_index: 1,
            filters: 2, // PvP
        },
        LfgActivityGroupInfo {
            group_id: 360,
            category_id: 9,
            name: "Rated Battlegrounds".into(),
            order_index: 1,
            filters: 2,
        },
        LfgActivityGroupInfo {
            group_id: 400,
            category_id: 6,
            name: "Custom — World Content".into(),
            order_index: 1,
            filters: 1,
        },
    ]
}

pub(in crate::lua_api::state) fn default_lfg_activities() -> Vec<LfgActivityInfo> {
    let pve = |activity_id: u32,
               group_id: u32,
               category_id: i32,
               full_name: &str,
               short_name: &str,
               max_players: i32,
               order_index: i32,
               difficulty_id: i32,
               is_current_raid: bool|
     -> LfgActivityInfo {
        LfgActivityInfo {
            activity_id,
            group_id,
            category_id,
            full_name: full_name.into(),
            short_name: short_name.into(),
            min_level: 80,
            max_players,
            item_level: 0,
            filters: 1, // PvE
            display_type: 0,
            order_index,
            use_honor_level: false,
            difficulty_id,
            allow_cross_faction: true,
            is_current_raid_activity: is_current_raid,
        }
    };
    let pvp = |activity_id: u32,
               group_id: u32,
               category_id: i32,
               full_name: &str,
               short_name: &str,
               max_players: i32,
               order_index: i32|
     -> LfgActivityInfo {
        LfgActivityInfo {
            activity_id,
            group_id,
            category_id,
            full_name: full_name.into(),
            short_name: short_name.into(),
            min_level: 70,
            max_players,
            item_level: 0,
            filters: 2, // PvP
            display_type: 0,
            order_index,
            use_honor_level: true,
            difficulty_id: 0,
            allow_cross_faction: false,
            is_current_raid_activity: false,
        }
    };
    vec![
        pve(
            1195,
            295,
            2,
            "Mists of Tirna Scithe (M+)",
            "MoTS M+",
            5,
            1,
            0,
            false,
        ),
        pve(
            1188,
            295,
            2,
            "The Stonevault (M+)",
            "Stonevault M+",
            5,
            2,
            0,
            false,
        ),
        pve(
            1190,
            295,
            2,
            "Ara-Kara, City of Echoes (M+)",
            "Ara-Kara M+",
            5,
            3,
            0,
            false,
        ),
        pve(
            1296,
            320,
            3,
            "Nerub-ar Palace (Heroic)",
            "Nerub-ar HC",
            20,
            1,
            15,
            true,
        ),
        pve(
            1295,
            320,
            3,
            "Nerub-ar Palace (Normal)",
            "Nerub-ar N",
            20,
            2,
            14,
            true,
        ),
        pve(
            1350,
            400,
            6,
            "World Boss — Aggregation of Horrors",
            "World Boss",
            40,
            1,
            0,
            false,
        ),
        pve(
            1700,
            400,
            6,
            "World Quests — Khaz Algar",
            "World Quests",
            5,
            2,
            0,
            false,
        ),
        pvp(491, 350, 4, "2v2 Arena Skirmish", "2v2 Arena", 2, 1),
        pvp(492, 350, 4, "3v3 Arena Skirmish", "3v3 Arena", 3, 2),
        pvp(493, 360, 9, "Rated Battlegrounds", "RBG", 10, 1),
    ]
}

pub(in crate::lua_api::state) fn default_lfd_dungeons() -> Vec<LfdDungeonInfo> {
    let d = |dungeon_id: i32,
             name: &str,
             type_id: i32,
             subtype_id: i32,
             min_level: i32,
             max_level: i32,
             rec_level: i32,
             max_players: i32,
             expansion_level: i32,
             texture_filename: &str,
             description: &str,
             is_random: bool,
             is_follower: bool|
     -> LfdDungeonInfo {
        LfdDungeonInfo {
            dungeon_id,
            name: name.into(),
            type_id,
            subtype_id,
            min_level,
            max_level,
            rec_level,
            min_rec_level: rec_level,
            max_rec_level: rec_level,
            expansion_level,
            group_id: 0,
            texture_filename: texture_filename.into(),
            difficulty: 0,
            max_players,
            description: description.into(),
            is_holiday: false,
            min_players: 1,
            map_name: name.into(),
            min_gear: 0,
            is_scaling_dungeon: false,
            is_random,
            is_follower_dungeon: is_follower,
        }
    };
    vec![
        // Header row (negative id). is_random=false: in retail data,
        // headers are pure categories — only positive-id "random heroic
        // dungeon" entries carry is_random=true.
        d(
            -1,
            "Random Heroic Dungeons",
            6,
            1,
            80,
            80,
            80,
            5,
            10,
            "",
            "",
            false,
            false,
        ),
        // Random heroic dungeon entry (positive id, is_random=true).
        // GetRandomDungeonBestChoice returns this id; LFG_UPDATE_RANDOM_INFO
        // selects it as the default choice. Without a positive-id random
        // entry, GetRandomDungeonBestChoice returns nil, which breaks the
        // LFDQueueFrame_SetType("specific") fallback only when the header
        // itself is mistakenly marked random.
        d(
            999,
            "Random Heroic Dungeon",
            6,
            2,
            80,
            80,
            80,
            5,
            10,
            "Interface/LFGFRAME/UI-LFG-BACKGROUND-HEROIC",
            "A random heroic dungeon.",
            true,
            false,
        ),
        // War Within dungeons
        d(
            1201,
            "Ara-Kara, City of Echoes",
            2,
            2,
            70,
            80,
            80,
            5,
            10,
            "Interface/LFGFRAME/UI-LFG-DUNGEON-ARAKARA",
            "A spider-city dungeon.",
            false,
            false,
        ),
        d(
            1202,
            "City of Threads",
            2,
            2,
            70,
            80,
            80,
            5,
            10,
            "Interface/LFGFRAME/UI-LFG-DUNGEON-CITYOFTHREADS",
            "Nerubian city.",
            false,
            true,
        ),
        d(
            1203,
            "Mists of Tirna Scithe",
            2,
            2,
            60,
            80,
            80,
            5,
            10,
            "Interface/LFGFRAME/UI-LFG-DUNGEON-MISTSOFTIRNA",
            "A fae forest dungeon.",
            false,
            false,
        ),
        d(
            1204,
            "The Stonevault",
            2,
            2,
            70,
            80,
            80,
            5,
            10,
            "Interface/LFGFRAME/UI-LFG-DUNGEON-STONEVAULT",
            "Earthen vault.",
            false,
            true,
        ),
        d(
            1205,
            "Grim Batol",
            2,
            2,
            15,
            80,
            80,
            5,
            10,
            "Interface/LFGFRAME/UI-LFG-DUNGEON-GRIMBATOL",
            "Ancient dragon bastion.",
            false,
            false,
        ),
        d(
            1206,
            "The Dawnbreaker",
            2,
            2,
            70,
            80,
            80,
            5,
            10,
            "Interface/LFGFRAME/UI-LFG-DUNGEON-DAWNBREAKER",
            "Arathi warship.",
            false,
            false,
        ),
        d(
            1207,
            "Darkflame Cleft",
            2,
            2,
            70,
            80,
            80,
            5,
            10,
            "Interface/LFGFRAME/UI-LFG-DUNGEON-DARKFLAMECLEFT",
            "A torch-lit cavern.",
            false,
            true,
        ),
        d(
            1208,
            "The Rookery",
            2,
            2,
            70,
            80,
            80,
            5,
            10,
            "Interface/LFGFRAME/UI-LFG-DUNGEON-ROOKERY",
            "Stormrook fortress.",
            false,
            false,
        ),
    ]
}

// Seed the `SimState.auction_browse_results` list with two
// representative Browse-tab rows (a crafting mat and a gear piece).
