//! Tests for `C_AchievementInfo` probes backed by
//! `SimState.achievements` + `WorldState.earned_achievements`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{AchievementGuildRep, AchievementInfo, AchievementStatistic};

#[path = "c_achievement_info_probes/comparison.rs"]
mod comparison;
#[path = "c_achievement_info_probes/search.rs"]
mod search;

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
fn get_achievement_link_returns_full_chat_link_for_seeded_id() {
    let env = env();
    let link: String = env.eval("return GetAchievementLink(6)").unwrap();
    assert_eq!(
        link,
        "|cffffff00|Hachievement:6:Player-1-00000001:1:1:15:2025:0:0:0:0|h[Level 10]|h|r"
    );
}

#[test]
fn get_achievement_link_returns_nil_for_unknown_id() {
    let env = env();
    let is_nil: bool = env
        .eval("return GetAchievementLink(999999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn get_achievement_link_uses_state_seeded_name() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievements.insert(
            12345,
            AchievementInfo {
                achievement_id: 12345,
                name: "Custom Title".into(),
                points: 5,
                description: String::new(),
                flags: 0,
                icon: 0,
                reward_text: String::new(),
                is_guild: false,
                is_statistic: false,
                reward_item_id: None,
            },
        );
    }
    let link: String = env.eval("return GetAchievementLink(12345)").unwrap();
    assert!(
        link.contains("[Custom Title]"),
        "expected bracketed name in link, got {link}"
    );
    assert!(link.contains("Hachievement:12345:"));
}

#[test]
fn get_achievement_guild_rep_returns_defaults_for_unseeded_id() {
    let env = env();
    let (requires_rep, has_rep, rep_level_is_nil): (bool, bool, bool) = env
        .eval(
            r#"
            local r, h, lvl = GetAchievementGuildRep(6)
            return r, h, lvl == nil
            "#,
        )
        .unwrap();
    assert!(!requires_rep);
    assert!(!has_rep);
    assert!(rep_level_is_nil);
}

#[test]
fn get_achievement_guild_rep_returns_seeded_values_when_unlocked() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievement_guild_rep.insert(
            2336,
            AchievementGuildRep {
                requires_rep: true,
                has_rep: true,
                rep_level: Some(8),
            },
        );
    }
    let (requires_rep, has_rep, rep_level): (bool, bool, i32) =
        env.eval("return GetAchievementGuildRep(2336)").unwrap();
    assert!(requires_rep);
    assert!(has_rep);
    assert_eq!(rep_level, 8);
}

#[test]
fn get_achievement_guild_rep_signals_locked_when_player_lacks_rep() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievement_guild_rep.insert(
            2336,
            AchievementGuildRep {
                requires_rep: true,
                has_rep: false,
                rep_level: Some(7),
            },
        );
    }
    let (requires_rep, has_rep, rep_level): (bool, bool, i32) =
        env.eval("return GetAchievementGuildRep(2336)").unwrap();
    assert!(requires_rep);
    assert!(!has_rep);
    assert_eq!(rep_level, 7);
}

#[test]
fn get_achievement_guild_rep_returns_three_values() {
    let env = env();
    let nret: i32 = env
        .eval("return select('#', GetAchievementGuildRep(6))")
        .unwrap();
    assert_eq!(nret, 3);
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

#[test]
fn get_statistic_returns_nil_and_false_for_unseeded_id() {
    let env = env();
    let (quantity_is_nil, is_counter): (bool, bool) = env
        .eval(
            r#"
            local q, c = GetStatistic(6)
            return q == nil, c
            "#,
        )
        .unwrap();
    assert!(quantity_is_nil);
    assert!(!is_counter);
}

#[test]
fn get_statistic_returns_seeded_quantity_and_counter_flag() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievement_statistics.insert(
            128,
            AchievementStatistic {
                quantity: "1234".into(),
                is_counter: true,
            },
        );
    }
    let (quantity, is_counter): (String, bool) = env.eval("return GetStatistic(128)").unwrap();
    assert_eq!(quantity, "1234");
    assert!(is_counter);
}

#[test]
fn get_statistic_returns_non_counter_for_string_stats() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievement_statistics.insert(
            132,
            AchievementStatistic {
                quantity: "1h 23m".into(),
                is_counter: false,
            },
        );
    }
    let (quantity, is_counter): (String, bool) = env.eval("return GetStatistic(132)").unwrap();
    assert_eq!(quantity, "1h 23m");
    assert!(!is_counter);
}

#[test]
fn get_statistic_two_arg_form_signals_skip() {
    let env = env();
    let (quantity_is_nil, skip, id_is_nil): (bool, bool, bool) = env
        .eval(
            r#"
            local q, s, id = GetStatistic(92, 1)
            return q == nil, s, id == nil
            "#,
        )
        .unwrap();
    assert!(quantity_is_nil);
    assert!(
        skip,
        "two-arg form must signal skip until categories are seeded"
    );
    assert!(id_is_nil);
}

#[test]
fn get_statistic_falls_back_to_dashes_when_unseeded() {
    let env = env();
    let displayed: String = env
        .eval(
            r#"
            local q = GetStatistic(99999)
            if not q then q = "--" end
            return q
            "#,
        )
        .unwrap();
    assert_eq!(displayed, "--", "addon's nil-fallback must yield '--'");
}

#[test]
fn get_guild_achievement_num_members_returns_zero_for_unseeded_id() {
    let env = env();
    let count: i32 = env
        .eval("return GetGuildAchievementNumMembers(4860)")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn get_guild_achievement_num_members_returns_seeded_count() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state
            .guild_achievement_members
            .insert(4860, vec!["Thrall".into(), "Jaina".into(), "Anduin".into()]);
    }
    let count: i32 = env
        .eval("return GetGuildAchievementNumMembers(4860)")
        .unwrap();
    assert_eq!(count, 3);
}

#[test]
fn get_guild_achievement_members_is_no_op() {
    let env = env();
    let nret: i32 = env
        .eval("return select('#', GetGuildAchievementMembers(4860))")
        .unwrap();
    assert_eq!(nret, 0, "async stub must return zero values");
}

#[test]
fn get_guild_achievement_member_info_returns_seeded_name_at_index() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state
            .guild_achievement_members
            .insert(4860, vec!["Thrall".into(), "Jaina".into(), "Anduin".into()]);
    }
    let (first, second, third): (String, String, String) = env
        .eval(
            r#"
            return GetGuildAchievementMemberInfo(4860, 1),
                   GetGuildAchievementMemberInfo(4860, 2),
                   GetGuildAchievementMemberInfo(4860, 3)
            "#,
        )
        .unwrap();
    assert_eq!(first, "Thrall");
    assert_eq!(second, "Jaina");
    assert_eq!(third, "Anduin");
}

#[test]
fn get_guild_achievement_member_info_returns_nil_for_out_of_range_index() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state
            .guild_achievement_members
            .insert(4860, vec!["Thrall".into()]);
    }
    let (zero_is_nil, past_end_is_nil): (bool, bool) = env
        .eval(
            r#"
            return GetGuildAchievementMemberInfo(4860, 0) == nil,
                   GetGuildAchievementMemberInfo(4860, 9) == nil
            "#,
        )
        .unwrap();
    assert!(zero_is_nil, "1-indexed: index 0 must yield nil");
    assert!(past_end_is_nil);
}

#[test]
fn get_guild_achievement_member_info_returns_nil_for_unseeded_achievement() {
    let env = env();
    let is_nil: bool = env
        .eval("return GetGuildAchievementMemberInfo(99999, 1) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn guild_member_tooltip_loop_renders_seeded_roster() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.guild_achievement_members.insert(
            4860,
            vec![
                "Thrall".into(),
                "Jaina".into(),
                "Anduin".into(),
                "Sylvanas".into(),
            ],
        );
    }
    let joined: String = env
        .eval(
            r#"
            local id = 4860
            local n = GetGuildAchievementNumMembers(id)
            local names = {}
            for i = 1, n do
                names[i] = GetGuildAchievementMemberInfo(id, i)
            end
            return table.concat(names, ",")
            "#,
        )
        .unwrap();
    assert_eq!(joined, "Thrall,Jaina,Anduin,Sylvanas");
}

#[test]
fn focused_achievement_defaults_to_none() {
    let env = env();
    assert!(env.state().borrow().focused_achievement.is_none());
}

#[test]
fn set_focused_achievement_records_id() {
    let env = env();
    env.eval::<()>("SetFocusedAchievement(2186)").unwrap();
    assert_eq!(env.state().borrow().focused_achievement, Some(2186));
}

#[test]
fn set_focused_achievement_returns_no_values() {
    let env = env();
    let nret: i32 = env
        .eval("return select('#', SetFocusedAchievement(6))")
        .unwrap();
    assert_eq!(nret, 0);
}

#[test]
fn set_focused_achievement_overwrites_previous_id() {
    let env = env();
    env.eval::<()>("SetFocusedAchievement(6)").unwrap();
    env.eval::<()>("SetFocusedAchievement(42)").unwrap();
    assert_eq!(env.state().borrow().focused_achievement, Some(42));
}
