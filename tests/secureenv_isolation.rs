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
fn two_secure_chunks_share_one_secureenv() {
    // Blizzard's restricted addon loads several secure files in sequence
    // (RestrictedEnvironment, RestrictedExecution, etc.). They cooperate
    // by writing to secureenv-scope globals that later files read. That
    // only works if every secure chunk resolves globals through the same
    // secureenv table.
    let env = env();

    // First secure chunk defines a binding that cannot exist in _G
    // (per the fenv-isolation test above) and only lives in secureenv.
    env.exec_rilua_secure(
        r#"
            secureenvSharedBetweenChunks = "defined-by-first-chunk"
        "#,
    )
    .unwrap();

    // Second secure chunk reads the same name through its own fenv and
    // republishes the result under a separate key so we can pull it out
    // from the registry-stored secureenv without trusting _G.
    env.exec_rilua_secure(
        r#"
            secureenvSharedBetweenChunksCopy = secureenvSharedBetweenChunks
        "#,
    )
    .unwrap();

    let (first_binding, copy_binding, mt_index_is_genv): (String, String, bool) = env
        .eval(
            r#"
            return tostring(rawget(__secureenv, "secureenvSharedBetweenChunks")),
                   tostring(rawget(__secureenv, "secureenvSharedBetweenChunksCopy")),
                   (getmetatable(__secureenv) and getmetatable(__secureenv).__index == _G) or false
            "#,
        )
        .unwrap();

    assert_eq!(
        first_binding, "defined-by-first-chunk",
        "first chunk's write must be present in secureenv"
    );
    assert_eq!(
        copy_binding, "defined-by-first-chunk",
        "second chunk must see the first chunk's binding through the shared secureenv"
    );
    assert!(
        mt_index_is_genv,
        "secureenv's metatable should still fall back to _G (no accidental rebuild)"
    );
}

#[test]
fn exec_maybe_secure_true_matches_exec_rilua_secure() {
    // The --exec-lua-secure CLI flag routes exec-lua code through
    // `exec_maybe_secure(code, true)`. Confirm that path is a pure
    // pass-through to `exec_rilua_secure`: a write lands in secureenv,
    // not in _G.
    let env = env();

    env.exec_maybe_secure(r#"execMaybeSecureProbe = "via-flag""#, true)
        .unwrap();

    let (g_probe_type, secureenv_value): (String, String) = env
        .eval(
            r#"
            return type(rawget(_G, "execMaybeSecureProbe")),
                   tostring(rawget(__secureenv, "execMaybeSecureProbe"))
            "#,
        )
        .unwrap();
    assert_eq!(g_probe_type, "nil");
    assert_eq!(secureenv_value, "via-flag");
}

#[test]
fn exec_maybe_secure_false_matches_exec() {
    // Without the flag, exec-lua routes through `exec_maybe_secure(code,
    // false)`, which must behave like plain `exec` — writes land on _G.
    let env = env();

    env.exec_maybe_secure(r#"execMaybeSecureInsecure = "plain""#, false)
        .unwrap();

    let on_g: String = env
        .eval(r#"return tostring(rawget(_G, "execMaybeSecureInsecure"))"#)
        .unwrap();
    assert_eq!(on_g, "plain");
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

/// Read a global through a function whose fenv is secureenv, mirroring how
/// a `[LoadIntoEnvironment secure]` chunk resolves names.
const SECURE_READ_PROBE: &str = r#"
    local result
    local probe = function() result = %NAME% end
    debug.setfenv(probe, __secureenv)
    probe()
    return result
"#;

#[test]
fn late_global_is_visible_to_secure_via_index_fallback() {
    // CLAIM A: a global registered on _G AFTER secureenv was created is not
    // in secureenv's own slot, so a secure read falls through __index to _G
    // and sees it. This is the "_G.MyAddonGlobal = 6 is linked" behavior.
    let env = env();

    // Insecure write → lands on _G (genv), absent from secureenv's own slot.
    env.exec(r#"lateGlobalProbe = 6"#).unwrap();

    let (in_secureenv_own_slot, secure_sees): (String, f64) = env
        .eval(
            r#"
            return type(rawget(__secureenv, "lateGlobalProbe")),
                   (function()
                       local result
                       local probe = function() result = lateGlobalProbe end
                       debug.setfenv(probe, __secureenv)
                       probe()
                       return result
                   end)()
            "#,
        )
        .unwrap();

    assert_eq!(
        in_secureenv_own_slot, "nil",
        "late global must NOT be in secureenv's own slot"
    );
    assert_eq!(
        secure_sees, 6.0,
        "secure code must see the late _G global through __index fallback"
    );
}

#[test]
fn late_global_link_is_live_not_snapshot() {
    // CLAIM B: the fall-through link is live. Re-binding the _G global to a
    // new value must be visible to a subsequent secure read — it is not
    // frozen at first access.
    let env = env();

    env.exec(r#"liveLinkProbe = 6"#).unwrap();
    let first: f64 = env
        .eval(&SECURE_READ_PROBE.replace("%NAME%", "liveLinkProbe"))
        .unwrap();
    assert_eq!(first, 6.0);

    // Re-bind on _G (insecure), then read again through secureenv.
    env.exec(r#"liveLinkProbe = 7"#).unwrap();
    let second: f64 = env
        .eval(&SECURE_READ_PROBE.replace("%NAME%", "liveLinkProbe"))
        .unwrap();
    assert_eq!(
        second, 7.0,
        "secure read must track the live _G value, not a snapshot"
    );
}

#[test]
fn secure_write_severs_the_link_to_global_env() {
    // CLAIM C: once secure code writes the name, the value lands in
    // secureenv's own slot, which shadows the __index fallback. From then on
    // the secure side is decoupled from _G — exactly the "frozen" behavior,
    // produced on demand rather than at creation.
    let env = env();

    env.exec(r#"severProbe = 10"#).unwrap();

    // Secure write → secureenv own slot.
    env.exec_rilua_secure(r#"severProbe = 99"#).unwrap();

    // Insecure re-bind on _G afterwards.
    env.exec(r#"severProbe = 11"#).unwrap();

    let (secure_sees, g_value): (f64, f64) = env
        .eval(
            r#"
            local result
            local probe = function() result = severProbe end
            debug.setfenv(probe, __secureenv)
            probe()
            return result, rawget(_G, "severProbe")
            "#,
        )
        .unwrap();

    assert_eq!(
        secure_sees, 99.0,
        "after a secure write, secure reads its own slot, not _G"
    );
    assert_eq!(g_value, 11.0, "_G keeps its own independent value");
}

#[test]
fn global_copied_at_creation_is_frozen_against_later_g_rebind() {
    // CLAIM D: a primitive global present at secureenv creation was copied
    // into secureenv's own slot. Re-binding it on _G later does NOT change
    // the secure read — the copy is decoupled. Contrast with CLAIM A/B where
    // the name was absent at creation and stays live.
    let env = env();

    // Discover a primitive global that exists in BOTH envs' own slots with
    // equal value (i.e. it was copied at creation). pairs() walks raw keys,
    // so it ignores the __index metatable and yields only copied keys.
    let key: String = env
        .eval(
            r#"
            for k, v in pairs(__secureenv) do
                if (type(v) == "number" or type(v) == "string")
                   and type(k) == "string"
                   and rawget(_G, k) == v then
                    return k
                end
            end
            return ""
            "#,
        )
        .unwrap();
    assert!(
        !key.is_empty(),
        "expected at least one copied primitive global at creation"
    );

    // Rebind the _G slot to a sentinel string the original could not equal.
    let probe = format!(
        r#"
            local sentinel = "__frozen_sentinel__"
            rawset(_G, {key:?}, sentinel)
            local result
            local probe = function() result = rawget(__secureenv, {key:?}) end
            probe()
            return tostring(result), tostring(rawget(_G, {key:?})), tostring(result == sentinel)
            "#,
    );
    let (secure_value, g_value, secure_equals_sentinel): (String, String, String) =
        env.eval(&probe).unwrap();

    assert_eq!(g_value, "__frozen_sentinel__", "_G rebind must have applied");
    assert_eq!(
        secure_equals_sentinel, "false",
        "secureenv's copied value (key {key}) must NOT follow the _G rebind"
    );
    assert_ne!(secure_value, "__frozen_sentinel__");
}

#[test]
fn secureenv_reads_replaced_soundkit_from_global_env() {
    let env = env();

    env.exec(
        r#"
            SOUNDKIT = { UI_IG_STORE_WINDOW_OPEN_BUTTON = 39512 }
        "#,
    )
    .unwrap();

    let sound_id: f64 = env
        .eval(
            r#"
            local result
            local probe = function()
                result = SOUNDKIT.UI_IG_STORE_WINDOW_OPEN_BUTTON
            end
            debug.setfenv(probe, __secureenv)
            probe()
            return result
            "#,
        )
        .unwrap();

    assert_eq!(sound_id, 39512.0);
}
