//! Tests for achievements: earn/remove via admin API, query via GetAchievementInfo.

use wow_ui_sim::event::EventArg;
use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn test_achievement_not_completed_by_default() {
    let env = env();
    let completed: bool = env
        .eval("local _, _, _, c = GetAchievementInfo(6); return c")
        .unwrap();
    assert!(!completed);
}

#[test]
fn test_earn_achievement() {
    let env = env();
    env.exec("A_Admin.SetAchievementEarned(6, true)").unwrap();
    let completed: bool = env
        .eval("local _, _, _, c = GetAchievementInfo(6); return c")
        .unwrap();
    assert!(completed);
}

#[test]
fn test_remove_achievement() {
    let env = env();
    env.exec("A_Admin.SetAchievementEarned(6, true)").unwrap();
    env.exec("A_Admin.SetAchievementEarned(6, false)").unwrap();
    let completed: bool = env
        .eval("local _, _, _, c = GetAchievementInfo(6); return c")
        .unwrap();
    assert!(!completed);
}

#[test]
fn test_has_achievement_false_by_default() {
    let env = env();
    let has: bool = env.eval("return A_Admin.HasAchievement(999)").unwrap();
    assert!(!has);
}

#[test]
fn test_has_achievement_true_after_earning() {
    let env = env();
    env.exec("A_Admin.SetAchievementEarned(999, true)").unwrap();
    let has: bool = env.eval("return A_Admin.HasAchievement(999)").unwrap();
    assert!(has);
}

#[test]
fn test_multiple_achievements_independent() {
    let env = env();
    env.exec("A_Admin.SetAchievementEarned(100, true)").unwrap();
    env.exec("A_Admin.SetAchievementEarned(200, true)").unwrap();
    let (a, b, c): (bool, bool, bool) = env
        .eval(
            "return A_Admin.HasAchievement(100), \
                    A_Admin.HasAchievement(200), \
                    A_Admin.HasAchievement(300)",
        )
        .unwrap();
    assert!(a);
    assert!(b);
    assert!(!c);
}

#[test]
fn test_achievement_info_returns_id() {
    let env = env();
    let id: i64 = env
        .eval("local id = GetAchievementInfo(42); return id")
        .unwrap();
    assert_eq!(id, 42);
}

// ============================================================================
// Achievement Categories
// ============================================================================

#[test]
fn test_get_category_list_returns_nine_categories() {
    let env = env();
    let count: i32 = env.eval("return #GetCategoryList()").unwrap();
    assert_eq!(count, 9);
}

#[test]
fn test_get_category_info_general() {
    let env = env();
    let (name, parent): (String, i32) = env
        .eval("local n, p = GetCategoryInfo(92); return n, p")
        .unwrap();
    assert_eq!(name, "General");
    assert_eq!(parent, -1);
}

#[test]
fn test_get_category_info_feats_of_strength() {
    let env = env();
    let name: String = env.eval("local n = GetCategoryInfo(81); return n").unwrap();
    assert_eq!(name, "Feats of Strength");
}

#[test]
fn test_get_category_info_unknown_returns_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("local n = GetCategoryInfo(99999); return n == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_get_category_num_achievements_returns_three_values() {
    let env = env();
    let (total, completed, incomplete): (i32, i32, i32) =
        env.eval("return GetCategoryNumAchievements(92)").unwrap();
    assert_eq!(total, 6); // 6 General achievements (Level 10-80)
    assert_eq!(completed, 0);
    assert_eq!(incomplete, 6);
}

#[test]
fn test_achievement_info_returns_real_name() {
    let env = env();
    let name: String = env
        .eval("local _, name = GetAchievementInfo(6); return name")
        .unwrap();
    assert_eq!(name, "Level 10");
}

#[test]
fn test_achievement_info_explore_elwynn() {
    let env = env();
    let (name, points): (String, i32) = env
        .eval("local _, name, pts = GetAchievementInfo(776); return name, pts")
        .unwrap();
    assert_eq!(name, "Explore Elwynn Forest");
    assert_eq!(points, 10);
}

#[test]
fn test_achievement_info_full_signature() {
    let env = env();
    env.exec("A_Admin.SetAchievementEarned(776, true)").unwrap();
    let (
        id,
        name,
        points,
        completed,
        month,
        day,
        year,
        desc,
        flags,
        icon,
        reward,
        is_guild,
        was_earned,
    ): (
        i32,
        String,
        i32,
        bool,
        i32,
        i32,
        i32,
        String,
        i32,
        i32,
        String,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local id, name, pts, comp, m, d, y, desc, fl, ic, rw, ig, we = GetAchievementInfo(776)
            return id, name, pts, comp, m, d, y, desc, fl, ic, rw, ig, we
            "#,
        )
        .unwrap();
    assert_eq!(id, 776);
    assert_eq!(name, "Explore Elwynn Forest");
    assert_eq!(points, 10);
    assert!(completed);
    assert_eq!(month, 1);
    assert_eq!(day, 15);
    assert_eq!(year, 2025);
    assert_eq!(
        desc,
        "Explore Elwynn Forest, revealing the covered areas of the world map."
    );
    assert_eq!(flags, 0);
    assert_eq!(icon, 236809);
    assert_eq!(reward, "");
    assert!(!is_guild);
    assert!(was_earned);
}

#[test]
fn test_category_num_updates_with_earned() {
    let env = env();
    env.exec("A_Admin.SetAchievementEarned(6, true)").unwrap();
    let (total, completed, incomplete): (i32, i32, i32) =
        env.eval("return GetCategoryNumAchievements(92)").unwrap();
    assert_eq!(total, 6);
    assert_eq!(completed, 1);
    assert_eq!(incomplete, 5);
}

#[test]
fn test_has_completed_any_achievement_is_available_for_ui_gating() {
    let env = env();
    let completed_any: bool = env.eval("return HasCompletedAnyAchievement()").unwrap();
    assert!(completed_any);
}

#[test]
fn test_can_show_achievement_ui_is_enabled() {
    let env = env();
    let can_show: bool = env.eval("return CanShowAchievementUI()").unwrap();
    assert!(can_show);
}

#[test]
fn test_get_achievement_category_returns_seeded_category() {
    let env = env();
    let (general, exploration, pvp): (i32, i32, i32) = env
        .eval(
            r#"
            return GetAchievementCategory(6),
                   GetAchievementCategory(776),
                   GetAchievementCategory(558)
            "#,
        )
        .unwrap();
    assert_eq!(general, 92);
    assert_eq!(exploration, 97);
    assert_eq!(pvp, 95);
}

#[test]
fn test_previous_and_next_achievement_follow_category_order() {
    let env = env();
    let (prev, next_id, next_complete): (i32, i32, bool) = env
        .eval(
            r#"
            local prev = GetPreviousAchievement(7)
            local next_id, completed = GetNextAchievement(7)
            return prev, next_id, completed
            "#,
        )
        .unwrap();
    assert_eq!(prev, 6);
    assert_eq!(next_id, 8);
    assert!(!next_complete);
}

#[test]
fn test_previous_and_next_achievement_handle_category_edges_and_completion() {
    let env = env();
    env.exec("A_Admin.SetAchievementEarned(8, true)").unwrap();
    let (prev_is_nil, next_id, next_complete, last_next_is_nil): (bool, i32, bool, bool) = env
        .eval(
            r#"
            local prev = GetPreviousAchievement(6)
            local next_id, completed = GetNextAchievement(7)
            local last_next = GetNextAchievement(11)
            return prev == nil, next_id, completed, last_next == nil
            "#,
        )
        .unwrap();
    assert!(prev_is_nil);
    assert_eq!(next_id, 8);
    assert!(next_complete);
    assert!(last_next_is_nil);
}

#[test]
fn test_latest_completed_achievements_returns_completed_ids() {
    let env = env();
    env.exec(
        r#"
        A_Admin.SetAchievementEarned(776, true)
        A_Admin.SetAchievementEarned(6, true)
        "#,
    )
    .unwrap();
    let (count, first, second): (i32, i32, i32) = env
        .eval(
            r#"
            local ids = GetLatestCompletedAchievements(false)
            return #ids, ids[1], ids[2]
            "#,
        )
        .unwrap();
    assert_eq!(count, 2);
    assert_eq!(first, 6);
    assert_eq!(second, 776);
}

#[test]
fn test_get_achievement_guild_rep_defaults_to_nil_for_non_guild_achievements() {
    let env = env();
    let is_nil: bool = env.eval("return GetAchievementGuildRep(6) == nil").unwrap();
    assert!(is_nil);
}

// ============================================================================
// GetAchievementCriteriaInfo
// ============================================================================

#[test]
fn test_achievement_num_criteria() {
    let env = env();
    let count: i32 = env.eval("return GetAchievementNumCriteria(948)").unwrap();
    assert_eq!(count, 5); // Ambassador: 5 factions
}

#[test]
fn test_achievement_criteria_info_returns_name() {
    let env = env();
    let name: String = env
        .eval("return GetAchievementCriteriaInfo(948, 1)")
        .unwrap();
    assert_eq!(name, "Exalted with Stormwind");
}

#[test]
fn test_achievement_criteria_completed_tracks_earned() {
    let env = env();
    env.exec("A_Admin.SetAchievementEarned(513, true)").unwrap();
    let (name, completed, qty, req): (String, bool, i32, i32) = env
        .eval(
            r#"
            local n, _, c, q, r = GetAchievementCriteriaInfo(513, 1)
            return n, c, q, r
            "#,
        )
        .unwrap();
    assert_eq!(name, "Honorable kills");
    assert!(completed);
    assert_eq!(qty, 100);
    assert_eq!(req, 100);
}

#[test]
fn test_achievement_criteria_nil_for_invalid_index() {
    let env = env();
    let is_nil: bool = env
        .eval("return GetAchievementCriteriaInfo(6, 99) == nil")
        .unwrap();
    assert!(is_nil);
}

// ============================================================================
// A_Admin.EarnAchievement
// ============================================================================

#[test]
fn test_earn_achievement_sets_earned_and_fires_event() {
    let env = env();
    let completed: bool = env
        .eval(
            r#"
            A_Admin.EarnAchievement(6)
            local _, _, _, c = GetAchievementInfo(6)
            return c
            "#,
        )
        .unwrap();
    let event_fired = {
        let state = env.state();
        let state = state.borrow();
        state.events.pending().iter().any(|event| {
            event.name == "ACHIEVEMENT_EARNED"
                && matches!(event.args.as_slice(), [EventArg::Number(id)] if *id == 6.0)
        })
    };
    assert!(completed);
    assert!(event_fired);
}
