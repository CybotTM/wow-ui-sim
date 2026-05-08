//! Tests for `C_LFGInfo` probes backed by `SimState.lfg_category_info`
//! and `SimState.lfg_active_categories`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::LfgCategoryInfo;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn can_player_use_lfd_returns_true_by_default() {
    let env = env();
    let (can_use, failure_reason): (bool, Option<String>) =
        env.eval("return C_LFGInfo.CanPlayerUseLFD()").unwrap();
    assert!(can_use, "CanPlayerUseLFD should return true by default");
    assert!(
        failure_reason.is_none(),
        "failure reason should be nil when allowed"
    );
}

#[test]
fn can_player_use_group_finder_returns_true_by_default() {
    let env = env();
    let (can_use, failure_reason): (bool, Option<String>) = env
        .eval("return C_LFGInfo.CanPlayerUseGroupFinder()")
        .unwrap();
    assert!(
        can_use,
        "CanPlayerUseGroupFinder gates the LFG micro menu button"
    );
    assert!(
        failure_reason.is_none(),
        "failure reason should be nil when group finder is allowed"
    );
}

#[test]
fn get_lfg_category_info_returns_dungeons_for_category_2() {
    let env = env();
    let (name, order): (String, i32) = env
        .eval("local info = C_LFGInfo.GetLFGCategoryInfo(2) return info.name, info.order")
        .unwrap();
    assert_eq!(name, "Dungeons");
    assert_eq!(order, 1);
}

#[test]
fn get_lfg_category_info_returns_raids_for_category_3() {
    let env = env();
    let (name, order): (String, i32) = env
        .eval("local info = C_LFGInfo.GetLFGCategoryInfo(3) return info.name, info.order")
        .unwrap();
    assert_eq!(name, "Raids");
    assert_eq!(order, 2);
}

#[test]
fn get_lfg_category_info_returns_nil_for_unknown_id() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_LFGInfo.GetLFGCategoryInfo(999) == nil")
        .unwrap();
    assert!(is_nil, "unknown category id should return nil");
}

#[test]
fn get_system_panel_data_returns_available() {
    let env = env();
    let (available, available_and_enabled): (bool, bool) = env
        .eval(
            r#"
            local data = C_LFGInfo.GetSystemPanelData()
            return data.isAvailable, data.isAvailableAndEnabled
            "#,
        )
        .unwrap();
    assert!(available);
    assert!(available_and_enabled);
}

#[test]
fn is_lfg_mode_active_returns_false_for_inactive_category() {
    let env = env();
    let active: bool = env
        .eval("return C_LFGInfo.IsLFGModeActiveForCategory(2)")
        .unwrap();
    assert!(!active, "no categories active by default");
}

#[test]
fn is_lfg_mode_active_reflects_state_mutation() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.lfg_active_categories.insert(2);
    }
    let (cat2, cat3): (bool, bool) = env
        .eval(
            r#"
            return C_LFGInfo.IsLFGModeActiveForCategory(2),
                   C_LFGInfo.IsLFGModeActiveForCategory(3)
            "#,
        )
        .unwrap();
    assert!(cat2, "category 2 was inserted into active set");
    assert!(!cat3, "category 3 was not inserted");
}

#[test]
fn get_lfg_category_info_reflects_state_mutation() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.lfg_category_info.insert(
            5,
            LfgCategoryInfo {
                name: "Scenarios".into(),
                order: 3,
                separate_recommended: false,
                prefer_current_area: false,
                allow_cross_faction: false,
                auto_choose_activity: false,
                show_playstyle_dropdown: false,
            },
        );
    }
    let (name, order): (String, i32) = env
        .eval("local info = C_LFGInfo.GetLFGCategoryInfo(5) return info.name, info.order")
        .unwrap();
    assert_eq!(name, "Scenarios");
    assert_eq!(order, 3);
}

#[test]
fn get_lfg_category_info_returns_state_flags() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.lfg_category_info.insert(
            9,
            LfgCategoryInfo {
                name: "Flagged".into(),
                order: 9,
                separate_recommended: true,
                prefer_current_area: true,
                allow_cross_faction: true,
                auto_choose_activity: true,
                show_playstyle_dropdown: true,
            },
        );
    }
    let flags: (bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            local info = C_LFGInfo.GetLFGCategoryInfo(9)
            return info.separateRecommended,
                   info.preferCurrentArea,
                   info.allowCrossFaction,
                   info.autoChooseActivity,
                   info.showPlaystyleDropdown
            "#,
        )
        .unwrap();
    assert_eq!(flags, (true, true, true, true, true));
}
