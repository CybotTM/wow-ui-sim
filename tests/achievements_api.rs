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
    let name: String = env
        .eval("local n = GetCategoryInfo(81); return n")
        .unwrap();
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
    let (total, completed, incomplete): (i32, i32, i32) = env
        .eval("return GetCategoryNumAchievements(92)")
        .unwrap();
    assert_eq!(total, 0);
    assert_eq!(completed, 0);
    assert_eq!(incomplete, 0);
}
