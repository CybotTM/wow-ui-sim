//! Tests for A_Admin spec and talent API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetSpec / GetSpecialization
// ============================================================================

#[test]
fn test_set_spec_readable_via_get_specialization() {
    let env = env();
    let spec: i32 = env
        .eval(
            r#"
            A_Admin.SetSpec(3)
            return GetSpecialization()
            "#,
        )
        .unwrap();
    assert_eq!(spec, 3);
}

#[test]
fn test_set_spec_one() {
    let env = env();
    let spec: i32 = env
        .eval(
            r#"
            A_Admin.SetSpec(1)
            return GetSpecialization()
            "#,
        )
        .unwrap();
    assert_eq!(spec, 1);
}

#[test]
fn test_set_spec_two() {
    let env = env();
    let spec: i32 = env
        .eval(
            r#"
            A_Admin.SetSpec(2)
            return GetSpecialization()
            "#,
        )
        .unwrap();
    assert_eq!(spec, 2);
}

#[test]
fn test_set_spec_overrides_previous() {
    let env = env();
    let spec: i32 = env
        .eval(
            r#"
            A_Admin.SetSpec(1)
            A_Admin.SetSpec(4)
            return GetSpecialization()
            "#,
        )
        .unwrap();
    assert_eq!(spec, 4);
}

// ============================================================================
// SetTalentRank
// ============================================================================

// Node 100734 is a regular (type 0) node in the Paladin class talent tree (tree 994).
// It has no spec-set condition, so it is visible regardless of active spec.
// configID 1 is always valid (returned by C_Traits.GetConfigIDBySystemID).
const PALADIN_NODE_ID: i32 = 100734;

#[test]
fn test_set_talent_rank_does_not_error() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetTalentRank(100734, 2)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

#[test]
fn test_set_talent_rank_reflected_in_get_node_info() {
    let env = env();
    let ranks: i32 = env
        .eval(&format!(
            r#"
            A_Admin.SetTalentRank({node}, 2)
            local info = C_Traits.GetNodeInfo(1, {node})
            return info.ranksPurchased
            "#,
            node = PALADIN_NODE_ID,
        ))
        .unwrap();
    assert_eq!(ranks, 2);
}

#[test]
fn test_set_talent_rank_zero_clears_ranks() {
    let env = env();
    let ranks: i32 = env
        .eval(&format!(
            r#"
            A_Admin.SetTalentRank({node}, 3)
            A_Admin.SetTalentRank({node}, 0)
            local info = C_Traits.GetNodeInfo(1, {node})
            return info.ranksPurchased
            "#,
            node = PALADIN_NODE_ID,
        ))
        .unwrap();
    assert_eq!(ranks, 0);
}

#[test]
fn test_set_talent_rank_updates_current_rank() {
    let env = env();
    let (current, active): (i32, i32) = env
        .eval(&format!(
            r#"
            A_Admin.SetTalentRank({node}, 1)
            local info = C_Traits.GetNodeInfo(1, {node})
            return info.currentRank, info.activeRank
            "#,
            node = PALADIN_NODE_ID,
        ))
        .unwrap();
    assert_eq!(current, 1);
    assert_eq!(active, 1);
}

// ============================================================================
// SetTalentSelection
// ============================================================================

#[test]
fn test_set_talent_selection_does_not_error() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetTalentSelection(100754, 122583)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

// ============================================================================
// ResetTalents
// ============================================================================

#[test]
fn test_reset_talents_clears_node_ranks() {
    let env = env();
    let ranks: i32 = env
        .eval(&format!(
            r#"
            A_Admin.SetTalentRank({node}, 3)
            A_Admin.ResetTalents()
            local info = C_Traits.GetNodeInfo(1, {node})
            return info.ranksPurchased
            "#,
            node = PALADIN_NODE_ID,
        ))
        .unwrap();
    assert_eq!(ranks, 0, "ResetTalents should clear all node ranks to 0");
}

#[test]
fn test_reset_talents_does_not_error() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetTalentRank(100734, 2)
            A_Admin.SetTalentRank(100754, 1)
            A_Admin.ResetTalents()
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

#[test]
fn test_reset_talents_allows_re_setting_ranks() {
    let env = env();
    let ranks: i32 = env
        .eval(&format!(
            r#"
            A_Admin.SetTalentRank({node}, 3)
            A_Admin.ResetTalents()
            A_Admin.SetTalentRank({node}, 1)
            local info = C_Traits.GetNodeInfo(1, {node})
            return info.ranksPurchased
            "#,
            node = PALADIN_NODE_ID,
        ))
        .unwrap();
    assert_eq!(ranks, 1);
}
