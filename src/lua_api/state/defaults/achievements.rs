use super::*;

/// Seed the `SimState.achievements` table with a handful of the
/// commonly-referenced retail achievement ids. Unknown ids are
/// treated as invalid by `IsValidAchievement`.
pub(in crate::lua_api::state) fn default_achievements() -> HashMap<i32, AchievementInfo> {
    [
        achievement_level_ten(),
        achievement_level_twenty(),
        achievement_level_thirty(),
        achievement_level_forty(),
        achievement_level_fifty(),
        achievement_level_sixty(),
        achievement_explore_elwynn_forest(),
        achievement_explore_eastern_kingdoms(),
        achievement_veteran_of_the_alliance(),
    ]
    .into_iter()
    .map(|a| (a.achievement_id, a))
    .collect()
}

fn achievement_level_ten() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 6,
        name: "Level 10".into(),
        points: 10,
        description: "Reach Level 10.".into(),
        flags: 0,
        icon: 236562,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_level_twenty() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 7,
        name: "Level 20".into(),
        points: 10,
        description: "Reach Level 20.".into(),
        flags: 0,
        icon: 236563,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_level_thirty() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 8,
        name: "Level 30".into(),
        points: 10,
        description: "Reach Level 30.".into(),
        flags: 0,
        icon: 236563,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_level_forty() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 9,
        name: "Level 40".into(),
        points: 10,
        description: "Reach Level 40.".into(),
        flags: 0,
        icon: 236565,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_level_fifty() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 10,
        name: "Level 50".into(),
        points: 10,
        description: "Reach Level 50.".into(),
        flags: 0,
        icon: 236565,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_level_sixty() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 11,
        name: "Level 60".into(),
        points: 10,
        description: "Reach Level 60.".into(),
        flags: 0,
        icon: 236567,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_explore_eastern_kingdoms() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 42,
        name: "Explore Eastern Kingdoms".into(),
        points: 30,
        description: "Explore Eastern Kingdoms, revealing the covered areas of the world map."
            .into(),
        flags: 0,
        icon: 236541,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_explore_elwynn_forest() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 776,
        name: "Explore Elwynn Forest".into(),
        points: 10,
        description: "Explore Elwynn Forest, revealing the covered areas of the world map.".into(),
        flags: 0,
        icon: 236809,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_veteran_of_the_alliance() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 558,
        name: "Veteran of the Alliance".into(),
        points: 25,
        description: "Earn 100 honorable kills in a single battleground.".into(),
        flags: 0,
        icon: 236412,
        reward_text: "Tabard reward".into(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: Some(43155),
    }
}

// Seed the `SimState.area_pois` table with one permanent and one
// time-limited POI so tests can exercise both the nil and the
// number return paths of `GetAreaPOISecondsLeft`.
