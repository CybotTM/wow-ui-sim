//! Tests for achievements: earn/remove via admin API, query via GetAchievementInfo.

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
    let (completed, event_fired): (bool, bool) = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("ACHIEVEMENT_EARNED")
            f:SetScript("OnEvent", function(self, event, id)
                if event == "ACHIEVEMENT_EARNED" and id == 6 then fired = true end
            end)
            A_Admin.EarnAchievement(6)
            local _, _, _, c = GetAchievementInfo(6)
            return c, fired
            "#,
        )
        .unwrap();
    assert!(completed);
    assert!(event_fired);
}
