//! Tests for A_Admin Great Vault API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetVaultActivity / C_WeeklyRewards.GetActivities
// ============================================================================

#[test]
fn test_set_vault_activity_appears_in_get_activities() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetVaultActivity(2, 1, 8, 5, 10)
            local activities = C_WeeklyRewards.GetActivities()
            return #activities
            "#,
        )
        .unwrap();
    assert!(count > 0);
}

#[test]
fn test_set_vault_activity_filter_by_type() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetVaultActivity(2, 1, 8, 5, 10)
            local activities = C_WeeklyRewards.GetActivities(2)
            return #activities
            "#,
        )
        .unwrap();
    assert!(count > 0);
}

#[test]
fn test_set_vault_activity_filter_excludes_other_types() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetVaultActivity(2, 1, 8, 5, 10)
            local activities = C_WeeklyRewards.GetActivities(1)
            return #activities
            "#,
        )
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_set_vault_activity_progress_field() {
    let env = env();
    let progress: i32 = env
        .eval(
            r#"
            A_Admin.SetVaultActivity(2, 1, 8, 5, 10)
            local activities = C_WeeklyRewards.GetActivities(2)
            return activities[1].progress
            "#,
        )
        .unwrap();
    assert_eq!(progress, 5);
}

#[test]
fn test_set_vault_activity_threshold_field() {
    let env = env();
    let threshold: i32 = env
        .eval(
            r#"
            A_Admin.SetVaultActivity(2, 1, 8, 5, 10)
            local activities = C_WeeklyRewards.GetActivities(2)
            return activities[1].threshold
            "#,
        )
        .unwrap();
    assert_eq!(threshold, 8);
}

#[test]
fn test_set_vault_activity_level_field() {
    let env = env();
    let level: i32 = env
        .eval(
            r#"
            A_Admin.SetVaultActivity(2, 1, 8, 5, 10)
            local activities = C_WeeklyRewards.GetActivities(2)
            return activities[1].level
            "#,
        )
        .unwrap();
    assert_eq!(level, 10);
}

#[test]
fn test_set_vault_activity_updates_existing_entry() {
    let env = env();
    let (count, progress): (i32, i32) = env
        .eval(
            r#"
            A_Admin.SetVaultActivity(2, 1, 8, 3, 10)
            A_Admin.SetVaultActivity(2, 1, 8, 7, 10)
            local activities = C_WeeklyRewards.GetActivities(2)
            return #activities, activities[1].progress
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(progress, 7);
}

#[test]
fn test_set_vault_activity_multiple_types() {
    let env = env();
    let total: i32 = env
        .eval(
            r#"
            A_Admin.SetVaultActivity(1, 1, 3, 2, 450)
            A_Admin.SetVaultActivity(2, 1, 8, 5, 470)
            A_Admin.SetVaultActivity(3, 1, 25, 20, 440)
            local activities = C_WeeklyRewards.GetActivities()
            return #activities
            "#,
        )
        .unwrap();
    assert_eq!(total, 3);
}

// ============================================================================
// SetVaultRewards / HasAvailableRewards / CanClaimRewards
// ============================================================================

#[test]
fn test_set_vault_rewards_both_true() {
    let env = env();
    let (has, can): (bool, bool) = env
        .eval(
            r#"
            A_Admin.SetVaultRewards(true, true)
            return C_WeeklyRewards.HasAvailableRewards(), C_WeeklyRewards.CanClaimRewards()
            "#,
        )
        .unwrap();
    assert!(has);
    assert!(can);
}

#[test]
fn test_set_vault_rewards_has_true_cannot_claim() {
    let env = env();
    let (has, can): (bool, bool) = env
        .eval(
            r#"
            A_Admin.SetVaultRewards(true, false)
            return C_WeeklyRewards.HasAvailableRewards(), C_WeeklyRewards.CanClaimRewards()
            "#,
        )
        .unwrap();
    assert!(has);
    assert!(!can);
}

#[test]
fn test_set_vault_rewards_false_clears_both() {
    let env = env();
    let (has, can): (bool, bool) = env
        .eval(
            r#"
            A_Admin.SetVaultRewards(true, true)
            A_Admin.SetVaultRewards(false)
            return C_WeeklyRewards.HasAvailableRewards(), C_WeeklyRewards.CanClaimRewards()
            "#,
        )
        .unwrap();
    assert!(!has);
    assert!(!can);
}

#[test]
fn test_set_vault_rewards_has_true_can_claim_defaults_to_has() {
    let env = env();
    let (has, can): (bool, bool) = env
        .eval(
            r#"
            A_Admin.SetVaultRewards(true)
            return C_WeeklyRewards.HasAvailableRewards(), C_WeeklyRewards.CanClaimRewards()
            "#,
        )
        .unwrap();
    assert!(has);
    assert!(can);
}

// ============================================================================
// ClearVault
// ============================================================================

#[test]
fn test_clear_vault_has_available_rewards_returns_false() {
    let env = env();
    let got: bool = env
        .eval(
            r#"
            A_Admin.SetVaultActivity(2, 1, 8, 5, 10)
            A_Admin.SetVaultRewards(true, true)
            A_Admin.ClearVault()
            return C_WeeklyRewards.HasAvailableRewards()
            "#,
        )
        .unwrap();
    assert!(!got);
}

#[test]
fn test_clear_vault_can_claim_rewards_returns_false() {
    let env = env();
    let got: bool = env
        .eval(
            r#"
            A_Admin.SetVaultRewards(true, true)
            A_Admin.ClearVault()
            return C_WeeklyRewards.CanClaimRewards()
            "#,
        )
        .unwrap();
    assert!(!got);
}

#[test]
fn test_clear_vault_get_activities_returns_empty() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetVaultActivity(2, 1, 8, 5, 10)
            A_Admin.ClearVault()
            local activities = C_WeeklyRewards.GetActivities()
            return #activities
            "#,
        )
        .unwrap();
    assert_eq!(count, 0);
}

// ============================================================================
// Default state
// ============================================================================

#[test]
fn test_vault_has_no_rewards_by_default() {
    let env = env();
    let got: bool = env
        .eval("return C_WeeklyRewards.HasAvailableRewards()")
        .unwrap();
    assert!(!got);
}

#[test]
fn test_vault_get_activities_empty_by_default() {
    let env = env();
    let count: i32 = env.eval("return #C_WeeklyRewards.GetActivities()").unwrap();
    assert_eq!(count, 0);
}
