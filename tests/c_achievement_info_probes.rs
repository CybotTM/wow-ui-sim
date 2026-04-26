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

#[test]
fn set_portrait_texture_writes_default_path_and_returns_true() {
    let env = env();
    let (ok, path): (bool, String) = env
        .eval(
            r#"
            local f = CreateFrame("Frame", nil, UIParent)
            local tex = f:CreateTexture(nil, "ARTWORK")
            local result = C_AchievementInfo.SetPortraitTexture(tex, "player")
            return result, tex:GetTexture()
            "#,
        )
        .unwrap();
    assert!(ok);
    assert_eq!(path, "Interface\\Icons\\Achievement_Character_Default");
}

#[test]
fn set_portrait_texture_works_without_unit_argument() {
    let env = env();
    let (ok, path): (bool, String) = env
        .eval(
            r#"
            local f = CreateFrame("Frame", nil, UIParent)
            local tex = f:CreateTexture(nil, "ARTWORK")
            local result = C_AchievementInfo.SetPortraitTexture(tex)
            return result, tex:GetTexture()
            "#,
        )
        .unwrap();
    assert!(ok);
    assert_eq!(path, "Interface\\Icons\\Achievement_Character_Default");
}

#[test]
fn get_num_completed_achievements_counts_seeded_categories() {
    let env = env();
    let (total, completed): (i32, i32) = env.eval("return GetNumCompletedAchievements()").unwrap();
    assert!(total > 0, "ACHIEVEMENT_CATEGORIES should expose seeded ids");
    assert_eq!(completed, 0, "no achievements earned by default");
}

#[test]
fn get_num_completed_achievements_reflects_world_earned() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.earned_achievements.insert(6);
        state.world.earned_achievements.insert(42);
        state.world.earned_achievements.insert(1017);
    }
    let (total, completed): (i32, i32) = env.eval("return GetNumCompletedAchievements()").unwrap();
    assert!(total >= 3);
    assert_eq!(completed, 3, "earned ids in seeded categories should count");
}

#[test]
fn get_num_completed_achievements_guild_branch_uses_guild_categories() {
    let env = env();
    let (account_total, guild_total): (i32, i32) = env
        .eval(
            r#"
            local at = GetNumCompletedAchievements(false)
            local gt = GetNumCompletedAchievements(true)
            return at, gt
            "#,
        )
        .unwrap();
    assert!(account_total > 0);
    assert_eq!(
        guild_total, 0,
        "GUILD_CATEGORIES has no seeded achievement ids"
    );
}

#[test]
fn get_num_completed_achievements_ignores_unseeded_earned_ids() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.earned_achievements.insert(99999);
    }
    let (_total, completed): (i32, i32) = env.eval("return GetNumCompletedAchievements()").unwrap();
    assert_eq!(
        completed, 0,
        "earning an id outside ACHIEVEMENT_CATEGORIES does not bump the completed count"
    );
}

#[test]
fn get_total_achievement_points_starts_at_zero() {
    let env = env();
    let total: i32 = env.eval("return GetTotalAchievementPoints()").unwrap();
    assert_eq!(total, 0, "no achievements earned yet");
}

#[test]
fn get_total_achievement_points_sums_seeded_points() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.earned_achievements.insert(6);
        state.world.earned_achievements.insert(42);
    }
    let total: i32 = env.eval("return GetTotalAchievementPoints()").unwrap();
    let level_ten_points = env
        .state()
        .borrow()
        .achievements
        .get(&6)
        .map(|info| info.points)
        .unwrap_or(0);
    let explore_ek_points = env
        .state()
        .borrow()
        .achievements
        .get(&42)
        .map(|info| info.points)
        .unwrap_or(0);
    assert_eq!(total, level_ten_points + explore_ek_points);
}

#[test]
fn get_total_achievement_points_guild_branch_uses_guild_categories() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.earned_achievements.insert(6);
    }
    let (account, guild): (i32, i32) = env
        .eval(
            r#"
            return GetTotalAchievementPoints(false), GetTotalAchievementPoints(true)
            "#,
        )
        .unwrap();
    assert!(account > 0, "earning id 6 raises account-view points");
    assert_eq!(
        guild, 0,
        "GUILD_CATEGORIES has no seeded ids, so guild-view points stay zero"
    );
}

#[test]
fn get_total_achievement_points_ignores_unseeded_earned_ids() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.earned_achievements.insert(99999);
    }
    let total: i32 = env.eval("return GetTotalAchievementPoints()").unwrap();
    assert_eq!(
        total, 0,
        "earning an id outside ACHIEVEMENT_CATEGORIES contributes no points"
    );
}

#[test]
fn set_portrait_texture_overwrites_prior_atlas_and_color() {
    let env = env();
    let (ok, path, atlas_is_nil): (bool, String, bool) = env
        .eval(
            r#"
            local f = CreateFrame("Frame", nil, UIParent)
            local tex = f:CreateTexture(nil, "ARTWORK")
            tex:SetColorTexture(1, 0, 0, 1)
            local result = C_AchievementInfo.SetPortraitTexture(tex, "target")
            return result, tex:GetTexture(), tex:GetAtlas() == nil
            "#,
        )
        .unwrap();
    assert!(ok);
    assert_eq!(path, "Interface\\Icons\\Achievement_Character_Default");
    assert!(atlas_is_nil);
}
