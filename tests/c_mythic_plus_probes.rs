//! Tests for `C_MythicPlus` probes backed by `SimState.mythic_plus`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{MythicPlusAffix, MythicPlusRun, MythicPlusWeeklyBest};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ── GetCurrentAffixes ────────────────────────────────────────────────────────

#[test]
fn get_current_affixes_returns_seeded_tyrannical() {
    let env = env();
    let (count, id, season_id): (i32, i32, i32) = env
        .eval(
            r#"
            local a = C_MythicPlus.GetCurrentAffixes()
            return #a, a[1].id, a[1].seasonID
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(id, 9, "default affix is Tyrannical (id=9)");
    assert_eq!(season_id, 14);
}

#[test]
fn get_current_affixes_reflects_mutation() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.mythic_plus.current_affixes = vec![
            MythicPlusAffix {
                id: 9,
                season_id: 14,
            },
            MythicPlusAffix {
                id: 10,
                season_id: 14,
            },
        ];
    }
    let count: i32 = env
        .eval("return #C_MythicPlus.GetCurrentAffixes()")
        .unwrap();
    assert_eq!(count, 2);
}

// ── GetCurrentSeason ─────────────────────────────────────────────────────────

#[test]
fn get_current_season_returns_14() {
    let env = env();
    let season: i32 = env.eval("return C_MythicPlus.GetCurrentSeason()").unwrap();
    assert_eq!(season, 14);
}

// ── C_ChallengeMode map probes ────────────────────────────────────────────────

#[test]
fn challenge_mode_get_map_table_returns_iterable_table() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local maps = C_ChallengeMode.GetMapTable()
            for _ in ipairs(maps) do
            end
            return type(maps) == "table"
            "#,
        )
        .unwrap();
    assert!(result);
}

#[test]
fn affix_info_returns_name_description_and_icon() {
    let env = env();
    let (global_name, challenge_name, tww_name, pulsar_name, description_type, icon): (
        String,
        String,
        String,
        String,
        String,
        i32,
    ) = env
        .eval(
            r#"
            local globalName, description, icon = GetAffixInfo(9)
            local challengeName = C_ChallengeMode.GetAffixInfo(9)
            local twwName = GetAffixInfo(148)
            local pulsarName = GetAffixInfo(162)
            return globalName, challengeName, twwName, pulsarName, type(description), icon
            "#,
        )
        .unwrap();
    assert_eq!(global_name, "Tyrannical");
    assert_eq!(challenge_name, "Tyrannical");
    assert_eq!(tww_name, "Xal'atath's Bargain: Ascendant");
    assert_eq!(pulsar_name, "Xal'atath's Bargain: Pulsar");
    assert_eq!(description_type, "string");
    assert_ne!(icon, 0);
}

// ── GetLastWeeklyChest ───────────────────────────────────────────────────────

#[test]
fn get_last_weekly_chest_returns_nothing_by_default() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local function count_rets(...)
                return select('#', ...)
            end
            return count_rets(C_MythicPlus.GetLastWeeklyChest())
            "#,
        )
        .unwrap();
    assert_eq!(count, 0, "no weekly chest tracked by default");
}

// ── GetRunHistory ─────────────────────────────────────────────────────────────

#[test]
fn get_run_history_empty_by_default() {
    let env = env();
    let count: i32 = env
        .eval("return #C_MythicPlus.GetRunHistory(false, false, false)")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn get_run_history_returns_seeded_runs() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.mythic_plus.run_history = vec![MythicPlusRun {
            map_challenge_mode_id: 399,
            level: 10,
            completed: true,
            season: 14,
            run_score: 150.0,
            this_week: true,
            duration_sec: 1800,
        }];
    }
    let (count, map_id, level, completed): (i32, i32, i32, bool) = env
        .eval(
            r#"
            local runs = C_MythicPlus.GetRunHistory(false, false, false)
            local r = runs[1]
            return #runs, r.mapChallengeModeID, r.level, r.completed
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(map_id, 399);
    assert_eq!(level, 10);
    assert!(completed);
}

// ── GetSeasonBestAffixScoreInfoForMap ─────────────────────────────────────────

#[test]
fn get_season_best_affix_score_info_returns_nothing_by_default() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local function count_rets(...)
                return select('#', ...)
            end
            return count_rets(C_MythicPlus.GetSeasonBestAffixScoreInfoForMap(399))
            "#,
        )
        .unwrap();
    assert_eq!(
        count, 0,
        "no season best data by default (mayreturnnothing)"
    );
}

// ── GetWeeklyChestRewardLevel ─────────────────────────────────────────────────

#[test]
fn get_weekly_chest_reward_level_returns_four_zeros() {
    let env = env();
    let (a, b, c, d): (i32, i32, i32, i32) = env
        .eval("return C_MythicPlus.GetWeeklyChestRewardLevel()")
        .unwrap();
    assert_eq!((a, b, c, d), (0, 0, 0, 0));
}

// ── GetOwnedKeystoneLevel ─────────────────────────────────────────────────────

#[test]
fn get_owned_keystone_level_returns_nothing_when_zero() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local function count_rets(...)
                return select('#', ...)
            end
            return count_rets(C_MythicPlus.GetOwnedKeystoneLevel())
            "#,
        )
        .unwrap();
    assert_eq!(count, 0, "no key = mayreturnnothing");
}

#[test]
fn get_owned_keystone_level_returns_level_when_set() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.mythic_plus.owned_keystone_level = 15;
    }
    let level: i32 = env
        .eval("return C_MythicPlus.GetOwnedKeystoneLevel()")
        .unwrap();
    assert_eq!(level, 15);
}

// ── GetWeeklyBestForMap ───────────────────────────────────────────────────────

#[test]
fn get_weekly_best_for_map_returns_nothing_when_no_data() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local function count_rets(...)
                return select('#', ...)
            end
            return count_rets(C_MythicPlus.GetWeeklyBestForMap(399))
            "#,
        )
        .unwrap();
    assert_eq!(count, 0, "no weekly best = mayreturnnothing");
}

#[test]
fn get_weekly_best_for_map_returns_seeded_data() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.mythic_plus.weekly_best_per_map.insert(
            399,
            MythicPlusWeeklyBest {
                map_challenge_mode_id: 399,
                level: 12,
                duration_sec: 1500,
                score: 180.0,
            },
        );
    }
    let (duration, level): (i32, i32) = env
        .eval("return C_MythicPlus.GetWeeklyBestForMap(399)")
        .unwrap();
    assert_eq!(duration, 1500);
    assert_eq!(level, 12);
}

// ── IsMythicPlusActive ────────────────────────────────────────────────────────

#[test]
fn is_mythic_plus_active_false_by_default() {
    let env = env();
    let active: bool = env
        .eval("return C_MythicPlus.IsMythicPlusActive()")
        .unwrap();
    assert!(!active);
}

#[test]
fn is_mythic_plus_active_toggle() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.mythic_plus.is_active = true;
    }
    let active: bool = env
        .eval("return C_MythicPlus.IsMythicPlusActive()")
        .unwrap();
    assert!(active);
}

// ── IsWeeklyRewardAvailable ───────────────────────────────────────────────────

#[test]
fn is_weekly_reward_available_false_by_default() {
    let env = env();
    let available: bool = env
        .eval("return C_MythicPlus.IsWeeklyRewardAvailable()")
        .unwrap();
    assert!(!available);
}

#[test]
fn is_weekly_reward_available_toggle() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.mythic_plus.is_weekly_reward_available = true;
    }
    let available: bool = env
        .eval("return C_MythicPlus.IsWeeklyRewardAvailable()")
        .unwrap();
    assert!(available);
}
