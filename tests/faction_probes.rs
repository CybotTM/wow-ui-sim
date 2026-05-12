//! Integration tests for `src/lua_api/globals/faction_probes.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::FactionEntry;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

fn seed_factions(env: &WowLuaEnv) {
    let mut st = env.state().borrow_mut();
    st.factions = vec![kirin_tor_entry(), wyrmrest_accord_entry()];
}

fn kirin_tor_entry() -> FactionEntry {
    FactionEntry {
        faction_id: 1090,
        name: "Kirin Tor".into(),
        description: "The archmages of Dalaran.".into(),
        standing: 8, // Exalted
        bottom: 21_000,
        top: 42_999,
        earned: 32_000,
        at_war: false,
        can_toggle_at_war: false,
        is_header: false,
        is_collapsed: false,
        has_rep: true,
        is_watched: false,
        is_child: false,
        has_bonus_rep_gain: false,
        can_be_lfg_bonus: false,
    }
}

fn wyrmrest_accord_entry() -> FactionEntry {
    FactionEntry {
        faction_id: 1091,
        name: "The Wyrmrest Accord".into(),
        description: "Defenders of Wyrmrest Temple.".into(),
        standing: 6, // Honored
        bottom: 3_000,
        top: 8_999,
        earned: 5_500,
        at_war: true,
        can_toggle_at_war: true,
        is_header: false,
        is_collapsed: false,
        has_rep: true,
        is_watched: false,
        is_child: false,
        has_bonus_rep_gain: true,
        can_be_lfg_bonus: false,
    }
}

fn fired(env: &WowLuaEnv, name: &str) -> bool {
    env.state()
        .borrow()
        .events
        .pending()
        .iter()
        .any(|e| e.name == name)
}

// ── GetFactionInfoByID ────────────────────────────────────────────────────────

#[test]
fn get_faction_info_by_id_returns_nothing_for_unknown_faction() {
    let env = env();
    let v: Option<String> = env.eval("return GetFactionInfoByID(999999)").unwrap();
    assert_eq!(v, None);
}

#[test]
fn get_faction_info_by_id_returns_sixteen_values_for_known_faction() {
    let env = env();
    seed_factions(&env);
    let arity: i32 = env
        .eval("return select('#', GetFactionInfoByID(1091))")
        .unwrap();
    assert_eq!(arity, 16);
}

#[test]
fn get_faction_info_by_id_fields_match_state() {
    let env = env();
    seed_factions(&env);
    let (name, standing, bottom, top, earned, at_war, has_bonus, faction_id): (
        String,
        i32,
        i64,
        i64,
        i64,
        bool,
        bool,
        i32,
    ) = env
        .eval(
            r#"
            local name, _desc, standing, bottom, top, earned,
                  atWar, _canToggle, _isHeader, _isCollapsed,
                  _hasRep, _isWatched, _isChild, factionID,
                  hasBonus = GetFactionInfoByID(1091)
            return name, standing, bottom, top, earned, atWar, hasBonus, factionID
            "#,
        )
        .unwrap();
    assert_eq!(name, "The Wyrmrest Accord");
    assert_eq!(standing, 6);
    assert_eq!(bottom, 3_000);
    assert_eq!(top, 8_999);
    assert_eq!(earned, 5_500);
    assert!(at_war);
    assert!(has_bonus);
    assert_eq!(faction_id, 1091);
}

// ── GetGuildFactionInfo ───────────────────────────────────────────────────────

#[test]
fn get_guild_faction_info_reads_world_guild_name() {
    let env = env();
    // Seeded world has "Heroes of Azeroth" guild.
    let (name, standing, bottom, top, earned): (String, i32, i64, i64, i64) = env
        .eval(
            r#"
            local name, _desc, standing, bottom, top, earned = GetGuildFactionInfo()
            return name, standing, bottom, top, earned
            "#,
        )
        .unwrap();
    assert_eq!(name, "Heroes of Azeroth");
    assert_eq!(standing, 8);
    assert_eq!(bottom, 0);
    assert_eq!(top, 1_000);
    assert_eq!(earned, 1_000);
}

#[test]
fn get_guild_faction_info_returns_nothing_when_guildless() {
    let env = env();
    env.state().borrow_mut().world.guild_name = None;
    let v: Option<String> = env.eval("return GetGuildFactionInfo()").unwrap();
    assert_eq!(v, None);
}

// ── GetSelectedFaction / SetSelectedFaction ───────────────────────────────────

#[test]
fn get_selected_faction_defaults_zero() {
    let env = env();
    let idx: i32 = env.eval("return GetSelectedFaction()").unwrap();
    assert_eq!(idx, 0);
}

#[test]
fn set_selected_faction_clamps_and_fires_event() {
    let env = env();
    seed_factions(&env);
    env.exec("SetSelectedFaction(2)").unwrap();
    let idx: i32 = env.eval("return GetSelectedFaction()").unwrap();
    assert_eq!(idx, 2);
    assert!(fired(&env, "UPDATE_FACTION"));
}

#[test]
fn set_selected_faction_clamps_to_range() {
    let env = env();
    seed_factions(&env);
    env.exec("SetSelectedFaction(99)").unwrap();
    let idx: i32 = env.eval("return GetSelectedFaction()").unwrap();
    assert_eq!(idx, 2, "out-of-range index clamps to factions.len()");
}

// ── SetWatchedFaction ─────────────────────────────────────────────────────────

#[test]
fn set_watched_faction_flips_is_watched_flag() {
    let env = env();
    seed_factions(&env);
    env.exec("SetWatchedFaction(1)").unwrap();
    {
        let st = env.state().borrow();
        assert_eq!(st.watched_faction_index, 1);
        assert!(st.factions[0].is_watched);
        assert!(!st.factions[1].is_watched);
    }
    assert!(fired(&env, "UPDATE_FACTION"));
}

#[test]
fn set_watched_faction_zero_clears_all_flags() {
    let env = env();
    seed_factions(&env);
    env.state().borrow_mut().factions[1].is_watched = true;
    env.state().borrow_mut().watched_faction_index = 2;
    env.exec("SetWatchedFaction(0)").unwrap();
    let st = env.state().borrow();
    assert_eq!(st.watched_faction_index, 0);
    assert!(st.factions.iter().all(|f| !f.is_watched));
}

#[test]
fn get_watched_faction_info_returns_legacy_status_bar_tuple() {
    let env = env();
    seed_factions(&env);
    env.exec("SetWatchedFaction(2)").unwrap();

    let (name, standing, bottom, top, earned, faction_id): (String, i32, i64, i64, i64, i32) = env
        .eval(
            r#"
            local name, standing, bottom, top, earned, factionID = GetWatchedFactionInfo()
            return name, standing, bottom, top, earned, factionID
            "#,
        )
        .unwrap();

    assert_eq!(name, "The Wyrmrest Accord");
    assert_eq!(standing, 6);
    assert_eq!(bottom, 3_000);
    assert_eq!(top, 8_999);
    assert_eq!(earned, 5_500);
    assert_eq!(faction_id, 1091);
}
