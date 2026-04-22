//! Tests for `C_AchievementInfo` probes backed by
//! `SimState.achievements` + `WorldState.earned_achievements`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::AchievementInfo;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn is_valid_achievement_returns_true_for_seeded_ids() {
    let env = env();
    let (level_ten, explore_ek, invalid): (bool, bool, bool) = env
        .eval(
            r#"
            return C_AchievementInfo.IsValidAchievement(6),
                   C_AchievementInfo.IsValidAchievement(42),
                   C_AchievementInfo.IsValidAchievement(999999)
            "#,
        )
        .unwrap();
    assert!(level_ten);
    assert!(explore_ek);
    assert!(!invalid);
}

#[test]
fn get_reward_item_id_returns_seeded_reward_or_nil() {
    let env = env();
    let (has_tabard, no_reward_is_nil, unknown_is_nil): (i32, bool, bool) = env
        .eval(
            r#"
            local tabard = C_AchievementInfo.GetRewardItemID(558)
            return tabard,
                   C_AchievementInfo.GetRewardItemID(6) == nil,
                   C_AchievementInfo.GetRewardItemID(999999) == nil
            "#,
        )
        .unwrap();
    assert_eq!(has_tabard, 43155);
    assert!(no_reward_is_nil, "Level 10 has no item reward");
    assert!(unknown_is_nil);
}

#[test]
fn get_achievement_info_returns_full_fifteen_tuple() {
    let env = env();
    let (id, name, points, completed, description, icon, is_guild, is_statistic): (
        i32,
        String,
        i32,
        bool,
        String,
        i32,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local id, name, points, completed, _, _, _, description,
                  _, icon, _, isGuild, _, _, isStatistic =
                C_AchievementInfo.GetAchievementInfo(6)
            return id, name, points, completed, description, icon,
                   isGuild, isStatistic
            "#,
        )
        .unwrap();
    assert_eq!(id, 6);
    assert_eq!(name, "Level 10");
    assert_eq!(points, 10);
    assert!(!completed, "not earned by default");
    assert_eq!(description, "Reach Level 10.");
    assert_eq!(icon, 236562);
    assert!(!is_guild);
    assert!(!is_statistic);
}

#[test]
fn get_achievement_info_reflects_earned_flag() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.earned_achievements.insert(6);
    }

    let (completed, was_earned_by_me, earned_by): (bool, bool, String) = env
        .eval(
            r#"
            local _, _, _, completed, _, _, _, _, _, _, _, _, wasEarned,
                  earnedBy, _ = C_AchievementInfo.GetAchievementInfo(6)
            return completed, wasEarned, earnedBy
            "#,
        )
        .unwrap();
    assert!(completed);
    assert!(was_earned_by_me);
    assert_eq!(earned_by, "player");
}

#[test]
fn get_achievement_info_returns_nothing_for_unknown_id() {
    let env = env();
    let nret: i32 = env
        .eval("return select('#', C_AchievementInfo.GetAchievementInfo(999999))")
        .unwrap();
    assert_eq!(nret, 0);
}

#[test]
fn is_valid_achievement_reflects_sim_state_mutation() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievements.insert(
            777,
            AchievementInfo {
                achievement_id: 777,
                name: "Custom Achievement".into(),
                points: 20,
                description: "Runtime seeded.".into(),
                flags: 0,
                icon: 0,
                reward_text: String::new(),
                is_guild: false,
                is_statistic: false,
                reward_item_id: Some(12345),
            },
        );
    }

    let (valid, reward, name): (bool, i32, String) = env
        .eval(
            r#"
            return C_AchievementInfo.IsValidAchievement(777),
                   C_AchievementInfo.GetRewardItemID(777),
                   (select(2, C_AchievementInfo.GetAchievementInfo(777)))
            "#,
        )
        .unwrap();
    assert!(valid);
    assert_eq!(reward, 12345);
    assert_eq!(name, "Custom Achievement");
}
