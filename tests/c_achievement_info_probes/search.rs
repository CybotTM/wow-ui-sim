use super::env;
use wow_ui_sim::lua_api::state::{AchievementInfo, AchievementSearchState};

#[test]
fn achievement_search_state_default_is_empty() {
    let state = AchievementSearchState::default();
    assert!(state.query.is_empty());
    assert_eq!(state.progress, 0);
    assert_eq!(state.size, 0);
    assert!(state.filtered_ids.is_empty());
}

#[test]
fn search_progress_and_size_default_to_zero() {
    let env = env();
    let (progress, size, count): (i32, i32, i32) = env
        .eval(
            r#"
            return GetAchievementSearchProgress(),
                   GetAchievementSearchSize(),
                   GetNumFilteredAchievements()
            "#,
        )
        .unwrap();
    assert_eq!(progress, 0);
    assert_eq!(size, 0);
    assert_eq!(count, 0);
}

#[test]
fn set_achievement_search_string_returns_true_for_sync_finish() {
    let env = env();
    let finished: bool = env
        .eval(r#"return SetAchievementSearchString("level")"#)
        .unwrap();
    assert!(
        finished,
        "synchronous impl always reports full search finished"
    );
}

#[test]
fn set_achievement_search_string_filters_seeded_achievements_by_substring() {
    let env = env();
    env.eval::<bool>(r#"return SetAchievementSearchString("level")"#)
        .unwrap();
    let count: i32 = env.eval("return GetNumFilteredAchievements()").unwrap();
    assert!(
        count > 0,
        "default seeded set contains 'Level 10'/'Level 20'/..."
    );
}

#[test]
fn set_achievement_search_string_is_case_insensitive() {
    let env = env();
    let (lower, upper): (i32, i32) = env
        .eval(
            r#"
            SetAchievementSearchString("LEVEL")
            local upper = GetNumFilteredAchievements()
            SetAchievementSearchString("level")
            local lower = GetNumFilteredAchievements()
            return lower, upper
            "#,
        )
        .unwrap();
    assert_eq!(lower, upper);
    assert!(lower > 0);
}

#[test]
fn set_achievement_search_string_clears_results_for_empty_query() {
    let env = env();
    env.eval::<bool>(r#"return SetAchievementSearchString("level")"#)
        .unwrap();
    env.eval::<bool>(r#"return SetAchievementSearchString("")"#)
        .unwrap();
    let count: i32 = env.eval("return GetNumFilteredAchievements()").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn search_progress_matches_size_after_sync_search() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievements.insert(
            777,
            AchievementInfo {
                achievement_id: 777,
                name: "Custom Searchable".into(),
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
    env.eval::<bool>(r#"return SetAchievementSearchString("Custom Searchable")"#)
        .unwrap();
    let (progress, size): (i32, i32) = env
        .eval("return GetAchievementSearchProgress(), GetAchievementSearchSize()")
        .unwrap();
    assert_eq!(progress, 1);
    assert_eq!(size, 1);
}

#[test]
fn get_filtered_achievement_id_returns_seeded_id_at_index() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievements.insert(
            777,
            AchievementInfo {
                achievement_id: 777,
                name: "Unique Marker".into(),
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
    env.eval::<bool>(r#"return SetAchievementSearchString("Unique Marker")"#)
        .unwrap();
    let id: i32 = env.eval("return GetFilteredAchievementID(1)").unwrap();
    assert_eq!(id, 777);
}

#[test]
fn get_filtered_achievement_id_returns_nil_for_out_of_range_index() {
    let env = env();
    env.eval::<bool>(r#"return SetAchievementSearchString("level")"#)
        .unwrap();
    let (zero_is_nil, past_end_is_nil): (bool, bool) = env
        .eval(
            r#"
            local count = GetNumFilteredAchievements()
            return GetFilteredAchievementID(0) == nil,
                   GetFilteredAchievementID(count + 1) == nil
            "#,
        )
        .unwrap();
    assert!(zero_is_nil, "1-indexed: index 0 must yield nil");
    assert!(past_end_is_nil);
}

#[test]
fn search_results_are_sorted_by_id() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        for (id, name) in [(900, "Marker B"), (800, "Marker A"), (1000, "Marker C")] {
            state.achievements.insert(
                id,
                AchievementInfo {
                    achievement_id: id,
                    name: name.into(),
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
    }
    env.eval::<bool>(r#"return SetAchievementSearchString("Marker")"#)
        .unwrap();
    let (first, second, third): (i32, i32, i32) = env
        .eval(
            r#"
            return GetFilteredAchievementID(1),
                   GetFilteredAchievementID(2),
                   GetFilteredAchievementID(3)
            "#,
        )
        .unwrap();
    assert_eq!(first, 800);
    assert_eq!(second, 900);
    assert_eq!(third, 1000);
}

#[test]
fn search_full_loop_renders_preview_results() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.achievements.insert(
            555,
            AchievementInfo {
                achievement_id: 555,
                name: "Preview Hit".into(),
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
    }
    let (count, name): (i32, String) = env
        .eval(
            r#"
            SetAchievementSearchString("Preview Hit")
            local n = GetNumFilteredAchievements()
            local id = GetFilteredAchievementID(1)
            local _, name = C_AchievementInfo.GetAchievementInfo(id)
            return n, name
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(name, "Preview Hit");
}
