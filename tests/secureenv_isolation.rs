//! End-to-end fenv isolation tests for secureenv.
//!
//! Secureenv is a shallow copy of `_G` retargeted onto the function
//! environment of chunks marked `[LoadIntoEnvironment secure]`. Writes
//! from secure code to env-level bindings must stay in secureenv — they
//! must not replicate to `_G`, because `_G` is what insecure addons see.
//! These tests exercise the property via the real load path (`mark_secure`
//! → `call_function`).
//!
//! Probe variable names use a lowercase prefix on purpose: our `_G`
//! metatable synthesises uppercase-only names into their own string
//! (so undefined `FOO_BAR` reads `"FOO_BAR"` instead of `nil`), which
//! defeats the "absent from _G" half of these assertions. Mixed-case
//! names skip that fallback.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn secure_primitive_write_stays_inside_secureenv() {
    let env = env();

    // Before the secure chunk runs, the probe name must be absent from
    // both environments. This pins the starting point so the post-run
    // assertions prove the write landed exactly in secureenv.
    let (initial_in_g, initial_in_secureenv): (String, String) = env
        .eval(
            r#"
            return type(rawget(_G, "secureenvPrimitiveProbe")),
                   type(rawget(__secureenv, "secureenvPrimitiveProbe"))
            "#,
        )
        .unwrap();
    assert_eq!(initial_in_g, "nil");
    assert_eq!(initial_in_secureenv, "nil");

    // Run a chunk under secureenv that rebinds a primitive global.
    // Because the chunk's fenv is secureenv, the assignment lands in
    // secureenv's hash, not in _G.
    env.exec_rilua_secure(
        r#"
            secureenvPrimitiveProbe = 42
        "#,
    )
    .unwrap();

    // Verify the split: _G stays nil, secureenv now holds 42.
    let (g_type, secureenv_type, secureenv_value): (String, String, f64) = env
        .eval(
            r#"
            return type(rawget(_G, "secureenvPrimitiveProbe")),
                   type(rawget(__secureenv, "secureenvPrimitiveProbe")),
                   rawget(__secureenv, "secureenvPrimitiveProbe")
            "#,
        )
        .unwrap();
    assert_eq!(g_type, "nil", "_G must not have been mutated");
    assert_eq!(secureenv_type, "number");
    assert!((secureenv_value - 42.0).abs() < f64::EPSILON);
}

#[test]
fn secure_string_write_is_not_visible_in_global_env() {
    let env = env();

    env.exec_rilua_secure(
        r#"
            secureenvStringProbe = "only-in-secureenv"
        "#,
    )
    .unwrap();

    let (g_probe_type, secureenv_probe): (String, String) = env
        .eval(
            r#"
            return type(rawget(_G, "secureenvStringProbe")),
                   tostring(rawget(__secureenv, "secureenvStringProbe"))
            "#,
        )
        .unwrap();
    assert_eq!(g_probe_type, "nil");
    assert_eq!(secureenv_probe, "only-in-secureenv");
}

#[test]
fn insecure_chunk_still_lands_in_global_env() {
    // Contrast test: the same assignment from a non-secure chunk DOES
    // land on _G. Keeps the first test honest — if mark_secure were a
    // no-op, the two writes would behave identically.
    let env = env();

    env.exec(
        r#"
            insecureProbe = "landed-on-_G"
        "#,
    )
    .unwrap();

    let on_g: String = env
        .eval(r#"return tostring(rawget(_G, "insecureProbe"))"#)
        .unwrap();
    assert_eq!(on_g, "landed-on-_G");
}

#[test]
fn shared_table_mutation_propagates_both_ways() {
    // The one legitimate cross-env write: mutating a table that both
    // envs already reference. Since shallow copy shares table refs,
    // this assignment should be visible from _G as well.
    let env = env();
    env.exec(r#"secureenvSharedContainer = {}"#).unwrap();

    // Secure chunk reads secureenvSharedContainer via __index fallback to
    // _G (it's the same table reference) and mutates a field on it.
    env.exec_rilua_secure(
        r#"
            secureenvSharedContainer.filled = "from-secure"
        "#,
    )
    .unwrap();

    let from_g: String = env
        .eval(r#"return tostring(rawget(_G, "secureenvSharedContainer").filled)"#)
        .unwrap();
    assert_eq!(
        from_g, "from-secure",
        "mutations to a pre-existing shared table should be visible from _G"
    );
}
