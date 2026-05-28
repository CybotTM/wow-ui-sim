//! Integration tests for `src/lua_api/globals/real/player_probes.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── IsLoggedIn ────────────────────────────────────────────────────────────────

#[test]
fn is_logged_in_reads_sim_state() {
    let env = env();
    // Default is false in SimState::new, but set it explicitly for clarity.
    env.state().borrow_mut().is_logged_in = false;
    let b: bool = env.eval("return IsLoggedIn()").unwrap();
    assert!(!b);
    env.state().borrow_mut().is_logged_in = true;
    let b: bool = env.eval("return IsLoggedIn()").unwrap();
    assert!(b);
}

// ── IsMenuOpen ────────────────────────────────────────────────────────────────

#[test]
fn is_menu_open_reads_sim_state() {
    let env = env();
    let b: bool = env.eval("return IsMenuOpen()").unwrap();
    assert!(!b);
    env.state().borrow_mut().menu_open = true;
    let b: bool = env.eval("return IsMenuOpen()").unwrap();
    assert!(b);
}

// ── IsXPUserDisabled ──────────────────────────────────────────────────────────

#[test]
fn is_xp_user_disabled_reads_sim_state() {
    let env = env();
    let b: bool = env.eval("return IsXPUserDisabled()").unwrap();
    assert!(!b);
    env.state().borrow_mut().xp_disabled = true;
    let b: bool = env.eval("return IsXPUserDisabled()").unwrap();
    assert!(b);
}

// ── PlayerCanTeleport ─────────────────────────────────────────────────────────

#[test]
fn player_can_teleport_defaults_true() {
    let env = env();
    let b: bool = env.eval("return PlayerCanTeleport()").unwrap();
    assert!(
        b,
        "default retail behaviour is that the player can teleport"
    );
}

#[test]
fn player_can_teleport_false_when_flag_off() {
    let env = env();
    env.state().borrow_mut().can_teleport = false;
    let b: bool = env.eval("return PlayerCanTeleport()").unwrap();
    assert!(!b);
}

// ── PlayerHasHearthstone ──────────────────────────────────────────────────────

#[test]
fn player_has_hearthstone_defaults_true() {
    let env = env();
    let b: bool = env.eval("return PlayerHasHearthstone()").unwrap();
    assert!(b);
}

#[test]
fn player_has_hearthstone_false_when_flag_off() {
    let env = env();
    env.state().borrow_mut().has_hearthstone = false;
    let b: bool = env.eval("return PlayerHasHearthstone()").unwrap();
    assert!(!b);
}

#[test]
fn player_probe_globals_live_under_real_globals_boundary() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/player_probes.rs").exists(),
        "player probe globals are modeled through SimState and belong under globals::real",
    );
    assert!(
        std::path::Path::new("src/lua_api/globals/real/player_probes.rs").exists(),
        "player probe globals should stay classified as real modeled Lua globals",
    );
}
