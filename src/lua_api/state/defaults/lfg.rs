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
        lfg_activity_group(295, 2, "The War Within Mythic+", 1, LFG_FILTER_PVE),
        lfg_activity_group(296, 2, "The War Within Heroic Dungeons", 2, LFG_FILTER_PVE),
        lfg_activity_group(320, 3, "The War Within Raids", 1, LFG_FILTER_PVE),
        lfg_activity_group(350, 4, "Arenas", 1, LFG_FILTER_PVP),
        lfg_activity_group(360, 9, "Rated Battlegrounds", 1, LFG_FILTER_PVP),
        lfg_activity_group(400, 6, "Custom — World Content", 1, LFG_FILTER_PVE),
    ]
}

const LFG_FILTER_PVE: u32 = 1;
const LFG_FILTER_PVP: u32 = 2;
const LFG_MIN_LEVEL_TWW: i32 = 80;
const LFG_MIN_LEVEL_PVP: i32 = 70;

#[derive(Clone, Copy)]
struct LfgActivitySeed {
    activity_id: u32,
    group_id: u32,
    category_id: i32,
    full_name: &'static str,
    short_name: &'static str,
    min_level: i32,
    max_players: i32,
    filters: u32,
    order_index: i32,
    use_honor_level: bool,
    difficulty_id: i32,
    allow_cross_faction: bool,
    is_current_raid_activity: bool,
}

impl From<LfgActivitySeed> for LfgActivityInfo {
    fn from(seed: LfgActivitySeed) -> Self {
        Self {
            activity_id: seed.activity_id,
            group_id: seed.group_id,
            category_id: seed.category_id,
            full_name: seed.full_name.into(),
            short_name: seed.short_name.into(),
            min_level: seed.min_level,
            max_players: seed.max_players,
            item_level: 0,
            filters: seed.filters,
            display_type: 0,
            order_index: seed.order_index,
            use_honor_level: seed.use_honor_level,
            difficulty_id: seed.difficulty_id,
            allow_cross_faction: seed.allow_cross_faction,
            is_current_raid_activity: seed.is_current_raid_activity,
        }
    }
}

fn lfg_activity_group(
    group_id: u32,
    category_id: i32,
    name: &str,
    order_index: i32,
    filters: u32,
) -> LfgActivityGroupInfo {
    LfgActivityGroupInfo {
        group_id,
        category_id,
        name: name.into(),
        order_index,
        filters,
    }
}

pub(in crate::lua_api::state) fn default_lfg_activities() -> Vec<LfgActivityInfo> {
    mythic_plus_activity_seeds()
        .into_iter()
        .chain(raid_activity_seeds())
        .chain(world_content_activity_seeds())
        .chain(pvp_activity_seeds())
        .map(LfgActivityInfo::from)
        .collect()
}

fn mythic_plus_activity_seeds() -> [LfgActivitySeed; 3] {
    [
        pve_activity(1195, 295, 2, "Mists of Tirna Scithe (M+)", "MoTS M+", 5, 1),
        pve_activity(1188, 295, 2, "The Stonevault (M+)", "Stonevault M+", 5, 2),
        pve_activity(
            1190,
            295,
            2,
            "Ara-Kara, City of Echoes (M+)",
            "Ara-Kara M+",
            5,
            3,
        ),
    ]
}

fn raid_activity_seeds() -> [LfgActivitySeed; 2] {
    [
        raid_activity(1296, "Nerub-ar Palace (Heroic)", "Nerub-ar HC", 1, 15, true),
        raid_activity(1295, "Nerub-ar Palace (Normal)", "Nerub-ar N", 2, 14, true),
    ]
}

fn world_content_activity_seeds() -> [LfgActivitySeed; 2] {
    [
        pve_activity(
            1350,
            400,
            6,
            "World Boss — Aggregation of Horrors",
            "World Boss",
            40,
            1,
        ),
        pve_activity(
            1700,
            400,
            6,
            "World Quests — Khaz Algar",
            "World Quests",
            5,
            2,
        ),
    ]
}

fn pvp_activity_seeds() -> [LfgActivitySeed; 3] {
    [
        pvp_activity(491, 350, 4, "2v2 Arena Skirmish", "2v2 Arena", 2, 1),
        pvp_activity(492, 350, 4, "3v3 Arena Skirmish", "3v3 Arena", 3, 2),
        pvp_activity(493, 360, 9, "Rated Battlegrounds", "RBG", 10, 1),
    ]
}

fn pve_activity(
    activity_id: u32,
    group_id: u32,
    category_id: i32,
    full_name: &'static str,
    short_name: &'static str,
    max_players: i32,
    order_index: i32,
) -> LfgActivitySeed {
    LfgActivitySeed {
        activity_id,
        group_id,
        category_id,
        full_name,
        short_name,
        min_level: LFG_MIN_LEVEL_TWW,
        max_players,
        filters: LFG_FILTER_PVE,
        order_index,
        use_honor_level: false,
        difficulty_id: 0,
        allow_cross_faction: true,
        is_current_raid_activity: false,
    }
}

fn pvp_activity(
    activity_id: u32,
    group_id: u32,
    category_id: i32,
    full_name: &'static str,
    short_name: &'static str,
    max_players: i32,
    order_index: i32,
) -> LfgActivitySeed {
    LfgActivitySeed {
        activity_id,
        group_id,
        category_id,
        full_name,
        short_name,
        min_level: LFG_MIN_LEVEL_PVP,
        max_players,
        filters: LFG_FILTER_PVP,
        order_index,
        use_honor_level: true,
        difficulty_id: 0,
        allow_cross_faction: false,
        is_current_raid_activity: false,
    }
}

fn raid_activity(
    activity_id: u32,
    full_name: &'static str,
    short_name: &'static str,
    order_index: i32,
    difficulty_id: i32,
    is_current_raid_activity: bool,
) -> LfgActivitySeed {
    LfgActivitySeed {
        difficulty_id,
        is_current_raid_activity,
        ..pve_activity(activity_id, 320, 3, full_name, short_name, 20, order_index)
    }
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
