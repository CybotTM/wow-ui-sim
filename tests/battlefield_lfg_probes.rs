//! Integration tests for `src/lua_api/globals/battlefield_lfg_probes.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::BattlefieldStatus;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── GetBattlefieldStatus ──────────────────────────────────────────────────────

#[test]
fn get_battlefield_status_reports_none_when_queue_empty() {
    let env = env();
    let (status, map_name): (String, String) = env
        .eval("return select(1, GetBattlefieldStatus(1)), select(2, GetBattlefieldStatus(1))")
        .unwrap();
    assert_eq!(status, "none");
    assert!(map_name.is_empty());
}

#[test]
fn get_battlefield_status_returns_queued_entry_for_matching_index() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.battlefield_queue.status = BattlefieldStatus::Queued;
        st.battlefield_queue.index = 1;
        st.battlefield_queue.name = "Warsong Gulch".into();
    }
    let (status, map_name): (String, String) = env
        .eval(
            r#"
            local status, mapName = GetBattlefieldStatus(1)
            return status, mapName
            "#,
        )
        .unwrap();
    assert_eq!(status, "queued");
    assert_eq!(map_name, "Warsong Gulch");
}

#[test]
fn get_battlefield_status_none_for_mismatched_index() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.battlefield_queue.status = BattlefieldStatus::Queued;
        st.battlefield_queue.index = 1;
        st.battlefield_queue.name = "Warsong Gulch".into();
    }
    let status: String = env
        .eval("return select(1, GetBattlefieldStatus(2))")
        .unwrap();
    assert_eq!(status, "none", "other indexes should report an empty queue");
}

#[test]
fn get_battlefield_status_returns_nine_values() {
    let env = env();
    let arity: i32 = env
        .eval("return select('#', GetBattlefieldStatus(1))")
        .unwrap();
    assert_eq!(arity, 9);
}

// ── GetBattlefieldInstanceRunTime ─────────────────────────────────────────────

#[test]
fn get_battlefield_instance_run_time_zero() {
    let env = env();
    let ms: i32 = env.eval("return GetBattlefieldInstanceRunTime()").unwrap();
    assert_eq!(ms, 0);
}

// ── GetNumBattlegroundEntries ─────────────────────────────────────────────────

#[test]
fn get_num_battleground_entries_zero_when_no_queue() {
    let env = env();
    let n: i32 = env.eval("return GetNumBattlegroundEntries()").unwrap();
    assert_eq!(n, 0);
}

#[test]
fn get_num_battleground_entries_one_when_queued() {
    let env = env();
    env.state().borrow_mut().battlefield_queue.status = BattlefieldStatus::Queued;
    let n: i32 = env.eval("return GetNumBattlegroundEntries()").unwrap();
    assert_eq!(n, 1);
}

#[test]
fn get_num_battleground_types_reports_no_listed_battlegrounds() {
    let env = env();
    let n: i32 = env.eval("return GetNumBattlegroundTypes()").unwrap();
    assert_eq!(n, 0);
}

// ── GetLFGDungeonInfo / Mode / NumEncounters ──────────────────────────────────

#[test]
fn get_lfg_dungeon_info_nil() {
    let env = env();
    let v: Option<String> = env.eval("return GetLFGDungeonInfo(1)").unwrap();
    assert_eq!(v, None);
}

#[test]
fn get_lfg_mode_returns_two_nils() {
    let env = env();
    let (a, b): (Option<String>, Option<String>) = env.eval("return GetLFGMode(1)").unwrap();
    assert_eq!(a, None);
    assert_eq!(b, None);
}

#[test]
fn get_lfg_random_cooldown_expiration_is_inert() {
    let env = env();
    let expiration: i32 = env.eval("return GetLFGRandomCooldownExpiration()").unwrap();
    assert_eq!(expiration, 0);
}

#[test]
fn get_lfg_dungeon_num_encounters_zero() {
    let env = env();
    let n: i32 = env.eval("return GetLFGDungeonNumEncounters(1)").unwrap();
    assert_eq!(n, 0);
}
