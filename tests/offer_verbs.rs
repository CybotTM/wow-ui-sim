//! Integration tests for `src/lua_api/globals/offer_verbs.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

fn fired(env: &WowLuaEnv, name: &str) -> bool {
    env.state()
        .borrow()
        .events
        .pending()
        .iter()
        .any(|e| e.name == name)
}

// ── AcceptDuel / DeclineDuel ──────────────────────────────────────────────────

#[test]
fn accept_duel_consumes_offer_and_fires_duel_finished() {
    let env = env();
    env.state().borrow_mut().pending_duel = Some("Arthas".into());
    env.exec("AcceptDuel()").unwrap();
    assert!(env.state().borrow().pending_duel.is_none());
    assert!(fired(&env, "DUEL_FINISHED"));
}

#[test]
fn accept_duel_without_offer_is_silent() {
    let env = env();
    assert!(env.state().borrow().pending_duel.is_none());
    env.exec("AcceptDuel()").unwrap();
    assert!(!fired(&env, "DUEL_FINISHED"));
}

#[test]
fn decline_duel_consumes_offer_and_fires_duel_finished() {
    let env = env();
    env.state().borrow_mut().pending_duel = Some("Arthas".into());
    env.exec("DeclineDuel()").unwrap();
    assert!(env.state().borrow().pending_duel.is_none());
    assert!(fired(&env, "DUEL_FINISHED"));
}

// ── AcceptResurrect / DeclineResurrect ────────────────────────────────────────

#[test]
fn accept_resurrect_clears_dead_since_and_fires_player_alive() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.pending_resurrect = Some("Tyrande".into());
        st.player.health = 0;
    }
    env.exec("AcceptResurrect()").unwrap();
    let st = env.state().borrow();
    assert!(st.pending_resurrect.is_none());
    assert_eq!(st.player.health, st.player.health_max);
    drop(st);
    assert!(fired(&env, "PLAYER_ALIVE"));
}

#[test]
fn decline_resurrect_clears_offer_silently() {
    let env = env();
    env.state().borrow_mut().pending_resurrect = Some("Tyrande".into());
    env.exec("DeclineResurrect()").unwrap();
    assert!(env.state().borrow().pending_resurrect.is_none());
    assert!(!fired(&env, "PLAYER_ALIVE"));
}

// ── RetrieveCorpse ────────────────────────────────────────────────────────────

#[test]
fn retrieve_corpse_revives_player_and_fires_events() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.corpse_available = true;
        st.player.health = 0;
    }
    env.exec("RetrieveCorpse()").unwrap();
    let st = env.state().borrow();
    assert!(!st.corpse_available);
    assert_eq!(st.player.health, st.player.health_max);
    drop(st);
    assert!(fired(&env, "PLAYER_ALIVE"));
    assert!(fired(&env, "CORPSE_IN_RANGE"));
}

#[test]
fn retrieve_corpse_without_corpse_is_noop() {
    let env = env();
    env.exec("RetrieveCorpse()").unwrap();
    assert!(!fired(&env, "PLAYER_ALIVE"));
}

// ── ResurrectGetOfferer ───────────────────────────────────────────────────────

#[test]
fn resurrect_get_offerer_returns_pending_name() {
    let env = env();
    env.state().borrow_mut().pending_resurrect = Some("Malfurion".into());
    let name: String = env.eval("return ResurrectGetOfferer()").unwrap();
    assert_eq!(name, "Malfurion");
}

#[test]
fn resurrect_get_offerer_returns_nil_when_no_offer() {
    let env = env();
    let is_nil: bool = env.eval("return ResurrectGetOfferer() == nil").unwrap();
    assert!(is_nil);
}
