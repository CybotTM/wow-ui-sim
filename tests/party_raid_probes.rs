//! Integration tests for the party/raid probe additions to
//! `src/lua_api/globals/group_queries.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

fn set_party_active(env: &WowLuaEnv, active: bool) {
    env.state().borrow_mut().party_group_active = active;
}

// ── Defaults ──────────────────────────────────────────────────────────────────

#[test]
fn probes_default_to_false_when_not_in_group() {
    let env = env();
    // The default sim seeds a party roster — simulate "no active group"
    // by flipping party_group_active off.
    set_party_active(&env, false);
    let tuple: (bool, bool, bool, bool, bool) = env
        .eval(
            "return IsInGroup(), IsInRaid(), IsPartyLFG(), IsGroupLeader(), IsEveryoneAssistant()",
        )
        .unwrap();
    assert_eq!(tuple, (false, false, false, false, false));
}

// ── IsInGroup / IsInRaid ─────────────────────────────────────────────────────

#[test]
fn is_in_group_true_when_party_active_with_members() {
    let env = env();
    set_party_active(&env, true);
    let b: bool = env.eval("return IsInGroup()").unwrap();
    assert!(b);
}

#[test]
fn is_in_raid_true_when_six_or_more_members() {
    let env = env();
    // Pad party up to raid threshold.
    {
        let mut st = env.state().borrow_mut();
        st.party_group_active = true;
        let template = st.party_members[0].clone();
        while st.party_members.len() < 6 {
            st.party_members.push(template.clone());
        }
    }
    let b: bool = env.eval("return IsInRaid()").unwrap();
    assert!(b);
}

// ── IsPartyLFG ────────────────────────────────────────────────────────────────

#[test]
fn is_party_lfg_reads_sim_state_flag() {
    let env = env();
    env.state().borrow_mut().is_party_lfg = true;
    let b: bool = env.eval("return IsPartyLFG()").unwrap();
    assert!(b);
}

// ── IsGroupLeader ─────────────────────────────────────────────────────────────

#[test]
fn is_group_leader_requires_active_group() {
    let env = env();
    // Flag says no group — default leader is the player but in solo we report false.
    set_party_active(&env, false);
    let b: bool = env.eval("return IsGroupLeader()").unwrap();
    assert!(!b);
}

#[test]
fn is_group_leader_true_when_player_leads_active_party() {
    let env = env();
    set_party_active(&env, true);
    // Default party_leader_index is None → player leads.
    let b: bool = env.eval("return IsGroupLeader()").unwrap();
    assert!(b);
}

#[test]
fn is_group_leader_false_when_party_member_leads() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.party_group_active = true;
        st.party_leader_index = Some(0);
    }
    let b: bool = env.eval("return IsGroupLeader()").unwrap();
    assert!(!b);
}

// ── IsEveryoneAssistant ───────────────────────────────────────────────────────

#[test]
fn is_everyone_assistant_reads_sim_state_flag() {
    let env = env();
    env.state().borrow_mut().everyone_assistant = true;
    let b: bool = env.eval("return IsEveryoneAssistant()").unwrap();
    assert!(b);
}
