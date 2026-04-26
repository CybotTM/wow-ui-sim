//! Integration tests for the `C_MajorFactions` Renown surface
//! (`src/c_api/c_major_factions.rs`) plus the `C_Reputation.IsMajorFaction`
//! and `C_Reputation.IsAccountWideReputation` lookups added to
//! `src/lua_api/globals/faction_probes.rs`.

use wow_ui_sim::lua_api::{MajorFactionData, RenownLevelInfo, WowLuaEnv};

fn sample_faction(faction_id: i64) -> MajorFactionData {
    MajorFactionData {
        faction_id,
        name: "Dream Wardens".to_string(),
        expansion_filter: 9,
        renown_level: 7,
        renown_reputation_earned: 1_500,
        renown_level_threshold: 2_500,
        is_unlocked: true,
        unlock_description: Some("Reach Honored to unlock.".to_string()),
        celebration_sound_kit: 12345,
        renown_fanfare_sound_kit_id: 67890,
        texture_kit: "majorfactions_dreamwardens".to_string(),
    }
}

fn sample_renown_levels(faction_id: i64) -> Vec<RenownLevelInfo> {
    vec![
        RenownLevelInfo {
            faction_id,
            level: 1,
            locked: false,
            is_milestone: false,
            is_capstone: false,
        },
        RenownLevelInfo {
            faction_id,
            level: 5,
            locked: false,
            is_milestone: true,
            is_capstone: false,
        },
        RenownLevelInfo {
            faction_id,
            level: 10,
            locked: true,
            is_milestone: true,
            is_capstone: true,
        },
    ]
}

#[test]
fn get_major_faction_data_returns_nil_when_unknown() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_MajorFactions.GetMajorFactionData(2507) == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_major_faction_data_returns_state_row() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .major_factions
        .insert(2507, sample_faction(2507));
    env.exec(
        r#"
        local data = C_MajorFactions.GetMajorFactionData(2507)
        name = data.name
        renown = data.renownLevel
        threshold = data.renownLevelThreshold
        unlocked = data.isUnlocked
        unlockDesc = data.unlockDescription
        textureKit = data.textureKit
        factionID = data.factionID
        expansion = data.expansionFilter
        earned = data.renownReputationEarned
        celebration = data.celebrationSoundKit
        fanfare = data.renownFanfareSoundKitID
    "#,
    )
    .unwrap();
    let name: String = env.eval("return name").unwrap();
    let renown: f64 = env.eval("return renown").unwrap();
    let threshold: f64 = env.eval("return threshold").unwrap();
    let unlocked: bool = env.eval("return unlocked").unwrap();
    let unlock_desc: String = env.eval("return unlockDesc").unwrap();
    let texture_kit: String = env.eval("return textureKit").unwrap();
    let faction_id: f64 = env.eval("return factionID").unwrap();
    let expansion: f64 = env.eval("return expansion").unwrap();
    let earned: f64 = env.eval("return earned").unwrap();
    let celebration: f64 = env.eval("return celebration").unwrap();
    let fanfare: f64 = env.eval("return fanfare").unwrap();
    assert_eq!(name, "Dream Wardens");
    assert!((renown - 7.0).abs() < 1e-6);
    assert!((threshold - 2_500.0).abs() < 1e-6);
    assert!(unlocked);
    assert_eq!(unlock_desc, "Reach Honored to unlock.");
    assert_eq!(texture_kit, "majorfactions_dreamwardens");
    assert!((faction_id - 2_507.0).abs() < 1e-6);
    assert!((expansion - 9.0).abs() < 1e-6);
    assert!((earned - 1_500.0).abs() < 1e-6);
    assert!((celebration - 12_345.0).abs() < 1e-6);
    assert!((fanfare - 67_890.0).abs() < 1e-6);
}

#[test]
fn get_major_faction_data_unlock_description_can_be_nil() {
    let env = WowLuaEnv::new().expect("env");
    let mut faction = sample_faction(2507);
    faction.unlock_description = None;
    env.state().borrow_mut().major_factions.insert(2507, faction);
    let nil: bool = env
        .eval("return C_MajorFactions.GetMajorFactionData(2507).unlockDescription == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_renown_levels_returns_empty_table_for_unknown_faction() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return #C_MajorFactions.GetRenownLevels(2507)")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_renown_levels_returns_state_sequence() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .major_faction_renown_levels
        .insert(2507, sample_renown_levels(2507));
    env.exec(
        r#"
        local levels = C_MajorFactions.GetRenownLevels(2507)
        count = #levels
        firstLevel = levels[1].level
        firstMilestone = levels[1].isMilestone
        secondLevel = levels[2].level
        secondMilestone = levels[2].isMilestone
        capLevel = levels[3].level
        capLocked = levels[3].locked
        capCapstone = levels[3].isCapstone
        capFaction = levels[3].factionID
    "#,
    )
    .unwrap();
    let count: f64 = env.eval("return count").unwrap();
    let first_level: f64 = env.eval("return firstLevel").unwrap();
    let first_milestone: bool = env.eval("return firstMilestone").unwrap();
    let second_level: f64 = env.eval("return secondLevel").unwrap();
    let second_milestone: bool = env.eval("return secondMilestone").unwrap();
    let cap_level: f64 = env.eval("return capLevel").unwrap();
    let cap_locked: bool = env.eval("return capLocked").unwrap();
    let cap_capstone: bool = env.eval("return capCapstone").unwrap();
    let cap_faction: f64 = env.eval("return capFaction").unwrap();
    assert!((count - 3.0).abs() < 1e-6);
    assert!((first_level - 1.0).abs() < 1e-6);
    assert!(!first_milestone);
    assert!((second_level - 5.0).abs() < 1e-6);
    assert!(second_milestone);
    assert!((cap_level - 10.0).abs() < 1e-6);
    assert!(cap_locked);
    assert!(cap_capstone);
    assert!((cap_faction - 2_507.0).abs() < 1e-6);
}

#[test]
fn is_major_faction_is_false_when_unregistered() {
    let env = WowLuaEnv::new().expect("env");
    let result: bool = env
        .eval("return C_Reputation.IsMajorFaction(2507)")
        .unwrap();
    assert!(!result);
}

#[test]
fn is_major_faction_reads_state_table() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .major_factions
        .insert(2507, sample_faction(2507));
    let result: bool = env
        .eval("return C_Reputation.IsMajorFaction(2507)")
        .unwrap();
    assert!(result);
    let other: bool = env
        .eval("return C_Reputation.IsMajorFaction(2511)")
        .unwrap();
    assert!(!other);
}

#[test]
fn is_account_wide_reputation_is_false_by_default() {
    let env = WowLuaEnv::new().expect("env");
    let result: bool = env
        .eval("return C_Reputation.IsAccountWideReputation(2507)")
        .unwrap();
    assert!(!result);
}

#[test]
fn is_account_wide_reputation_reads_state_set() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .account_wide_reputation_factions
        .insert(2507);
    let listed: bool = env
        .eval("return C_Reputation.IsAccountWideReputation(2507)")
        .unwrap();
    let unlisted: bool = env
        .eval("return C_Reputation.IsAccountWideReputation(2511)")
        .unwrap();
    assert!(listed);
    assert!(!unlisted);
}

#[test]
fn reputation_bar_helper_uses_renown_max_level() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .major_faction_renown_levels
        .insert(2507, sample_renown_levels(2507));
    env.exec(
        r#"
        local levels = C_MajorFactions.GetRenownLevels(2507)
        maxLevel = levels[#levels].level
    "#,
    )
    .unwrap();
    let max_level: f64 = env.eval("return maxLevel").unwrap();
    assert!((max_level - 10.0).abs() < 1e-6);
}
