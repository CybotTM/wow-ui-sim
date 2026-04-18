//! Sandbox-parity probes for dangerous globals.
//!
//! Pins the behavioural expectation for five classically-restricted
//! globals across both environments that run addon code:
//!
//! - `dofile`, `loadfile`, `require` — filesystem / module loaders.
//!   Blizzard strips these from the client environment; real WoW
//!   addons see `nil` for all three.
//! - `string.dump` — bytecode dumper. Blocking it is part of the
//!   restricted-execution policy because dumped bytecode can be
//!   loaded back and side-step fenv/taint.
//! - `math.randomseed` — not security-critical on its own but
//!   removed from the restricted surface because it mutates a
//!   process-global RNG shared with the rest of the UI; restricted
//!   code shouldn't perturb it.
//!
//! The current secureenv is a shallow copy of `_G` with an `__index`
//! fallback to `_G`. That means anything still present on `_G` is
//! reachable from secureenv too — the sandbox parity check has to
//! treat the two environments as a single attack surface.
//!
//! Results drive the follow-up decision recorded in PLAN.md: if a
//! probe surfaces a real leak, restore the missing `_G` cleanup.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn probe(env: &WowLuaEnv, name: &str) -> (String, String) {
    let script = format!(
        r#"
        return type(rawget(_G, "{name}")),
               type(rawget(__secureenv, "{name}"))
        "#
    );
    env.eval::<(String, String)>(&script)
        .expect("probe eval should succeed")
}

#[test]
fn dofile_is_absent_from_both_environments() {
    let env = env();
    let (g, secure) = probe(&env, "dofile");
    assert_eq!(g, "nil", "dofile leaks into _G");
    assert_eq!(secure, "nil", "dofile leaks into __secureenv");
}

#[test]
fn loadfile_is_absent_from_both_environments() {
    let env = env();
    let (g, secure) = probe(&env, "loadfile");
    assert_eq!(g, "nil", "loadfile leaks into _G");
    assert_eq!(secure, "nil", "loadfile leaks into __secureenv");
}

#[test]
fn require_is_absent_from_both_environments() {
    let env = env();
    let (g, secure) = probe(&env, "require");
    assert_eq!(g, "nil", "require leaks into _G");
    assert_eq!(secure, "nil", "require leaks into __secureenv");
}

#[test]
fn string_dump_is_absent_from_both_environments() {
    let env = env();
    let (g, secure): (String, String) = env
        .eval(
            r#"
            local g_string = rawget(_G, "string")
            local secure_string = rawget(__secureenv, "string")
            return type(g_string and g_string.dump),
                   type(secure_string and secure_string.dump)
            "#,
        )
        .unwrap();
    assert_eq!(g, "nil", "string.dump leaks into _G.string");
    assert_eq!(secure, "nil", "string.dump leaks into __secureenv.string");
}

#[test]
fn math_randomseed_is_absent_from_both_environments() {
    let env = env();
    let (g, secure): (String, String) = env
        .eval(
            r#"
            local g_math = rawget(_G, "math")
            local secure_math = rawget(__secureenv, "math")
            return type(g_math and g_math.randomseed),
                   type(secure_math and secure_math.randomseed)
            "#,
        )
        .unwrap();
    assert_eq!(g, "nil", "math.randomseed leaks into _G.math");
    assert_eq!(
        secure, "nil",
        "math.randomseed leaks into __secureenv.math"
    );
}
