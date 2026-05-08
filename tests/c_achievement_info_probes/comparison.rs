use super::env;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{AchievementComparisonData, AchievementInfo};

fn seed_comparison_data(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.achievement_comparison_data.earned.insert(6);
    state.achievement_comparison_data.earned.insert(42);
    state.achievement_comparison_data.earned.insert(558);
    state
        .achievement_comparison_data
        .completion_dates
        .insert(6, (3, 7, 2024));
    state
        .achievement_comparison_data
        .statistics
        .insert(128, "8888".into());
}

#[test]
fn set_achievement_comparison_unit_records_unit_and_fires_event() {
    let env = env();
    let ok: bool = env
        .eval(r#"return SetAchievementComparisonUnit("party1")"#)
        .unwrap();
    assert!(ok);
    let state = env.state();
    let state = state.borrow();
    assert_eq!(state.achievement_comparison_unit.as_deref(), Some("party1"));
    let event_fired = state
        .events
        .pending()
        .iter()
        .any(|event| event.name == "INSPECT_ACHIEVEMENT_READY");
    assert!(event_fired, "INSPECT_ACHIEVEMENT_READY must be queued");
}

#[test]
fn set_achievement_comparison_unit_returns_false_for_empty_string() {
    let env = env();
    let ok: bool = env
        .eval(r#"return SetAchievementComparisonUnit("")"#)
        .unwrap();
    assert!(!ok);
    assert!(env.state().borrow().achievement_comparison_unit.is_none());
}

#[test]
fn clear_achievement_comparison_unit_resets_state() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievement_comparison_unit = Some("party1".into());
    }
    env.eval::<()>("ClearAchievementComparisonUnit()").unwrap();
    assert!(env.state().borrow().achievement_comparison_unit.is_none());
}

#[test]
fn get_achievement_comparison_info_returns_false_when_no_unit_selected() {
    let env = env();
    seed_comparison_data(&env);
    let (completed, month_is_nil, day_is_nil, year_is_nil): (bool, bool, bool, bool) = env
        .eval(
            r#"
            local c, m, d, y = GetAchievementComparisonInfo(6)
            return c, m == nil, d == nil, y == nil
            "#,
        )
        .unwrap();
    assert!(!completed);
    assert!(month_is_nil);
    assert!(day_is_nil);
    assert!(year_is_nil);
}

#[test]
fn get_achievement_comparison_info_returns_seeded_date_for_earned() {
    let env = env();
    seed_comparison_data(&env);
    {
        let mut state = env.state().borrow_mut();
        state.achievement_comparison_unit = Some("party1".into());
    }
    let (completed, month, day, year): (bool, i32, i32, i32) =
        env.eval("return GetAchievementComparisonInfo(6)").unwrap();
    assert!(completed);
    assert_eq!(month, 3);
    assert_eq!(day, 7);
    assert_eq!(year, 2024);
}

#[test]
fn get_achievement_comparison_info_returns_false_for_unearned() {
    let env = env();
    seed_comparison_data(&env);
    {
        let mut state = env.state().borrow_mut();
        state.achievement_comparison_unit = Some("party1".into());
    }
    let (completed, month_is_nil): (bool, bool) = env
        .eval(
            r#"
            local c, m = GetAchievementComparisonInfo(948)
            return c, m == nil
            "#,
        )
        .unwrap();
    assert!(!completed);
    assert!(month_is_nil);
}

#[test]
fn get_comparison_achievement_points_returns_zero_when_no_unit() {
    let env = env();
    seed_comparison_data(&env);
    let total: i32 = env.eval("return GetComparisonAchievementPoints()").unwrap();
    assert_eq!(total, 0);
}

#[test]
fn get_comparison_achievement_points_sums_seeded_earned_achievements() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievements.insert(
            6,
            AchievementInfo {
                achievement_id: 6,
                name: "Level 10".into(),
                points: 10,
                description: String::new(),
                flags: 0,
                icon: 0,
                reward_text: String::new(),
                is_guild: false,
                is_statistic: false,
                reward_item_id: None,
            },
        );
        state.achievements.insert(
            42,
            AchievementInfo {
                achievement_id: 42,
                name: "Explore Eastern Kingdoms".into(),
                points: 25,
                description: String::new(),
                flags: 0,
                icon: 0,
                reward_text: String::new(),
                is_guild: false,
                is_statistic: false,
                reward_item_id: None,
            },
        );
        state.achievement_comparison_unit = Some("party1".into());
        state.achievement_comparison_data.earned.insert(6);
        state.achievement_comparison_data.earned.insert(42);
    }
    let total: i32 = env.eval("return GetComparisonAchievementPoints()").unwrap();
    assert_eq!(total, 35);
}

#[test]
fn get_comparison_category_num_achievements_counts_seeded_earned() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievement_comparison_unit = Some("party1".into());
        state.achievement_comparison_data.earned.insert(6);
        state.achievement_comparison_data.earned.insert(7);
    }
    let general_completed: i32 = env
        .eval("return GetComparisonCategoryNumAchievements(92)")
        .unwrap();
    assert_eq!(general_completed, 2);
}

#[test]
fn get_comparison_category_num_achievements_returns_zero_when_no_unit() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievement_comparison_data.earned.insert(6);
    }
    let general_completed: i32 = env
        .eval("return GetComparisonCategoryNumAchievements(92)")
        .unwrap();
    assert_eq!(general_completed, 0);
}

#[test]
fn get_comparison_statistic_returns_seeded_quantity() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievement_comparison_unit = Some("party1".into());
        state
            .achievement_comparison_data
            .statistics
            .insert(128, "8888".into());
    }
    let quantity: String = env.eval("return GetComparisonStatistic(128)").unwrap();
    assert_eq!(quantity, "8888");
}

#[test]
fn get_comparison_statistic_returns_nil_when_no_unit() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state
            .achievement_comparison_data
            .statistics
            .insert(128, "8888".into());
    }
    let is_nil: bool = env
        .eval("return GetComparisonStatistic(128) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn comparison_data_struct_can_be_default_constructed() {
    let data = AchievementComparisonData::default();
    assert!(data.earned.is_empty());
    assert!(data.completion_dates.is_empty());
    assert!(data.statistics.is_empty());
}
