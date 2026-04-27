//! Integration tests for the `C_ArtifactUI` artifact-bar surface registered
//! in `src/c_api/c_artifact_ui.rs`.

use wow_ui_sim::lua_api::{ArtifactInfo, WowLuaEnv};

fn sample_artifact() -> ArtifactInfo {
    ArtifactInfo {
        item_id: 128_910,
        alt_item_id: 128_911,
        name: "Ashbringer".to_string(),
        icon: "Interface/Icons/inv_sword_2h_artifactashbringer_d_01".to_string(),
        total_xp: 12_500,
        points_spent: 7,
        quality: 6,
        artifact_appearance_id: 41,
        appearance_mod_id: 0,
        item_appearance_id: 0,
        alt_item_appearance_id: 0,
        alt_on_top: false,
        tier: 2,
        maxed: false,
        disabled: false,
        category: 1,
    }
}

#[test]
fn equipped_artifact_item_id_is_nil_when_no_artifact() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_ArtifactUI.GetEquippedArtifactItemID() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn equipped_artifact_item_id_returns_state_value() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().equipped_artifact = Some(sample_artifact());
    let id: f64 = env
        .eval("return C_ArtifactUI.GetEquippedArtifactItemID()")
        .unwrap();
    assert!((id - 128_910.0).abs() < 1e-6);
}

#[test]
fn equipped_artifact_info_returns_thirteen_values() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().equipped_artifact = Some(sample_artifact());
    env.exec(
        "id, alt, name, icon, xp, points, quality, appID, modID, itemAppID, altAppID, altTop, tier = C_ArtifactUI.GetEquippedArtifactInfo()",
    )
    .unwrap();
    let id: f64 = env.eval("return id").unwrap();
    let name: String = env.eval("return name").unwrap();
    let icon: String = env.eval("return icon").unwrap();
    let xp: f64 = env.eval("return xp").unwrap();
    let points: f64 = env.eval("return points").unwrap();
    let alt_top: bool = env.eval("return altTop").unwrap();
    let tier: f64 = env.eval("return tier").unwrap();
    assert!((id - 128_910.0).abs() < 1e-6);
    assert_eq!(name, "Ashbringer");
    assert!(icon.contains("artifactashbringer"));
    assert!((xp - 12_500.0).abs() < 1e-6);
    assert!((points - 7.0).abs() < 1e-6);
    assert!(!alt_top);
    assert!((tier - 2.0).abs() < 1e-6);
}

#[test]
fn equipped_artifact_info_returns_no_values_when_unequipped() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_ArtifactUI.GetEquippedArtifactInfo() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn is_equipped_artifact_maxed_reads_flag() {
    let env = WowLuaEnv::new().expect("env");
    let none_default: bool = env
        .eval("return C_ArtifactUI.IsEquippedArtifactMaxed()")
        .unwrap();
    assert!(!none_default);

    let mut artifact = sample_artifact();
    artifact.maxed = true;
    env.state().borrow_mut().equipped_artifact = Some(artifact);
    let maxed: bool = env
        .eval("return C_ArtifactUI.IsEquippedArtifactMaxed()")
        .unwrap();
    assert!(maxed);
}

#[test]
fn is_equipped_artifact_disabled_reads_flag() {
    let env = WowLuaEnv::new().expect("env");
    let none_default: bool = env
        .eval("return C_ArtifactUI.IsEquippedArtifactDisabled()")
        .unwrap();
    assert!(!none_default);

    let mut artifact = sample_artifact();
    artifact.disabled = true;
    env.state().borrow_mut().equipped_artifact = Some(artifact);
    let disabled: bool = env
        .eval("return C_ArtifactUI.IsEquippedArtifactDisabled()")
        .unwrap();
    assert!(disabled);
}

#[test]
fn get_cost_for_point_at_rank_returns_zero_when_unset() {
    let env = WowLuaEnv::new().expect("env");
    let cost: f64 = env
        .eval("return C_ArtifactUI.GetCostForPointAtRank(5, 1)")
        .unwrap();
    assert!(cost.abs() < 1e-6);
}

#[test]
fn get_cost_for_point_at_rank_reads_state_table() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut state = env.state().borrow_mut();
        state.artifact_point_costs.insert((5, 1), 1_000);
        state.artifact_point_costs.insert((5, 2), 5_000);
    }
    let tier_one: f64 = env
        .eval("return C_ArtifactUI.GetCostForPointAtRank(5, 1)")
        .unwrap();
    let tier_two: f64 = env
        .eval("return C_ArtifactUI.GetCostForPointAtRank(5, 2)")
        .unwrap();
    let missing_rank: f64 = env
        .eval("return C_ArtifactUI.GetCostForPointAtRank(6, 2)")
        .unwrap();
    assert!((tier_one - 1_000.0).abs() < 1e-6);
    assert!((tier_two - 5_000.0).abs() < 1e-6);
    assert!(missing_rank.abs() < 1e-6);
}

#[test]
fn xp_reward_target_info_is_nil_when_no_artifact_equipped() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_ArtifactUI.GetArtifactXPRewardTargetInfo(1) == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn xp_reward_target_info_returns_name_and_icon_when_category_matches() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().equipped_artifact = Some(sample_artifact());
    env.exec("name, icon = C_ArtifactUI.GetArtifactXPRewardTargetInfo(1)")
        .unwrap();
    let name: String = env.eval("return name").unwrap();
    let icon: String = env.eval("return icon").unwrap();
    assert_eq!(name, "Ashbringer");
    assert!(icon.contains("artifactashbringer"));
}

#[test]
fn xp_reward_target_info_is_nil_when_category_mismatches() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().equipped_artifact = Some(sample_artifact());
    let nil: bool = env
        .eval("return C_ArtifactUI.GetArtifactXPRewardTargetInfo(99) == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn xp_reward_target_info_uses_state_category_value() {
    let env = WowLuaEnv::new().expect("env");
    let mut artifact = sample_artifact();
    artifact.category = 7;
    env.state().borrow_mut().equipped_artifact = Some(artifact);
    let nil_for_one: bool = env
        .eval("return C_ArtifactUI.GetArtifactXPRewardTargetInfo(1) == nil")
        .unwrap();
    let name_for_seven: String = env
        .eval("return (C_ArtifactUI.GetArtifactXPRewardTargetInfo(7))")
        .unwrap();
    assert!(nil_for_one);
    assert_eq!(name_for_seven, "Ashbringer");
}

#[test]
fn artifact_bar_helper_consumes_costs() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut state = env.state().borrow_mut();
        state.equipped_artifact = Some(sample_artifact());
        state.artifact_point_costs.insert((7, 2), 3_000);
        state.artifact_point_costs.insert((8, 2), 6_000);
        state.artifact_point_costs.insert((9, 2), 12_000);
    }
    env.exec(
        r#"
        function ArtifactBarGetNumArtifactTraitsPurchasableFromXP(pointsSpent, artifactXP, artifactTier)
            local numPoints = 0
            local xpForNextPoint = C_ArtifactUI.GetCostForPointAtRank(pointsSpent, artifactTier)
            while artifactXP >= xpForNextPoint and xpForNextPoint > 0 do
                artifactXP = artifactXP - xpForNextPoint
                pointsSpent = pointsSpent + 1
                numPoints = numPoints + 1
                xpForNextPoint = C_ArtifactUI.GetCostForPointAtRank(pointsSpent, artifactTier)
            end
            return numPoints, artifactXP, xpForNextPoint
        end
        local _, _, _, _, totalXP, points, _, _, _, _, _, _, tier = C_ArtifactUI.GetEquippedArtifactInfo()
        np, leftover, nextCost = ArtifactBarGetNumArtifactTraitsPurchasableFromXP(points, totalXP, tier)
    "#,
    )
    .unwrap();
    let np: f64 = env.eval("return np").unwrap();
    let leftover: f64 = env.eval("return leftover").unwrap();
    let next_cost: f64 = env.eval("return nextCost").unwrap();
    assert!((np - 2.0).abs() < 1e-6);
    assert!((leftover - 3_500.0).abs() < 1e-6);
    assert!((next_cost - 12_000.0).abs() < 1e-6);
}
