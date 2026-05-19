use super::*;
use std::collections::HashSet;

/// Every category entry must be non-empty and unique within its
/// category. Catches copy-paste duplicates when the list grows.
#[test]
fn every_category_has_unique_nonempty_entries() {
    for (name, slice) in [
        ("HOT_GLOBALS", HOT_GLOBALS),
        ("HOT_NAMESPACES", HOT_NAMESPACES),
        ("HOT_FRAME_METHODS", HOT_FRAME_METHODS),
        ("HOT_METATABLE_KEYS", HOT_METATABLE_KEYS),
        ("HOT_LOADER_SENTINELS", HOT_LOADER_SENTINELS),
    ] {
        let mut seen: HashSet<&[u8]> = HashSet::new();
        for entry in slice {
            assert!(!entry.is_empty(), "{name} contains an empty byte slice",);
            assert!(
                seen.insert(entry),
                "{name} contains duplicate entry {:?}",
                std::str::from_utf8(entry).unwrap_or("<non-utf8>"),
            );
        }
    }
}

/// Confirms [`HOT_LITERAL_COUNT`] stays in sync with the sum of the
/// per-category slices. Compile-time-ish; catches drift if a new
/// category is added without updating the total.
#[test]
fn count_equals_sum_of_categories() {
    let sum = HOT_GLOBALS.len()
        + HOT_NAMESPACES.len()
        + HOT_FRAME_METHODS.len()
        + HOT_METATABLE_KEYS.len()
        + HOT_LOADER_SENTINELS.len();
    assert_eq!(HOT_LITERAL_COUNT, sum);
}

#[test]
fn version_is_nonzero() {
    assert!(WHITELIST_VERSION >= 1);
}

/// End-to-end bootstrap: install the registry on a fresh VM and
/// confirm each category's handles decode back to the source bytes.
/// Pins the invariant that the static intern cache survives the
/// prewarm step.
#[test]
fn registry_install_produces_handles_that_decode_to_source_bytes() {
    use rilua::{Lua, LuaApiMut};

    let mut lua = Lua::new().expect("fresh rilua VM");
    let handles = HotLiteralRegistry::install(lua.state_mut());

    assert_eq!(handles.len(), HOT_LITERAL_COUNT);

    // Spot-check one entry from each category — full roundtrip for
    // every entry in `every_handle_decodes_to_its_source_bytes`.
    let checks: &[(&'static [u8], GcRef<LuaString>)] = &[
        (HOT_GLOBALS[0], handles.global(0)),
        (HOT_NAMESPACES[0], handles.namespace(0)),
        (HOT_FRAME_METHODS[0], handles.frame_method(0)),
        (HOT_METATABLE_KEYS[0], handles.metatable_key(0)),
        (HOT_LOADER_SENTINELS[0], handles.loader_sentinel(0)),
    ];
    for (expected, handle) in checks {
        let s = lua
            .state_mut()
            .gc
            .string_arena
            .get(*handle)
            .expect("interned string alive");
        assert_eq!(s.data(), *expected);
    }
}

/// Full roundtrip: every position in every category slice must decode
/// back to its source bytes via the arena lookup.
#[test]
fn every_handle_decodes_to_its_source_bytes() {
    use rilua::{Lua, LuaApiMut};

    let mut lua = Lua::new().expect("fresh rilua VM");
    let handles = HotLiteralRegistry::install(lua.state_mut());

    let state = lua.state_mut();
    let categories: &[(&'static str, &[&[u8]], &[GcRef<LuaString>])] = &[
        ("globals", HOT_GLOBALS, &handles.globals),
        ("namespaces", HOT_NAMESPACES, &handles.namespaces),
        ("frame_methods", HOT_FRAME_METHODS, &handles.frame_methods),
        (
            "metatable_keys",
            HOT_METATABLE_KEYS,
            &handles.metatable_keys,
        ),
        (
            "loader_sentinels",
            HOT_LOADER_SENTINELS,
            &handles.loader_sentinels,
        ),
    ];
    for (name, src, refs) in categories {
        assert_eq!(src.len(), refs.len(), "{name} length mismatch");
        for (i, (bytes, r)) in src.iter().zip(refs.iter()).enumerate() {
            let s = state
                .gc
                .string_arena
                .get(*r)
                .unwrap_or_else(|| panic!("{name}[{i}] interned string missing"));
            assert_eq!(s.data(), *bytes, "{name}[{i}] byte mismatch");
        }
    }
}

/// Pins each named index constant to its expected source byte slice.
/// Catches drift when a category is reordered without bumping
/// WHITELIST_VERSION and updating the index constants.
#[test]
fn named_indexes_map_to_expected_slice_entries() {
    use metatable_idx as mi;

    let metatable_pairs: &[(usize, &'static [u8])] = &[
        (mi::INDEX, b"__index"),
        (mi::NEWINDEX, b"__newindex"),
        (mi::TOSTRING, b"__tostring"),
        (mi::GC, b"__gc"),
        (mi::EQ, b"__eq"),
        (mi::LT, b"__lt"),
        (mi::LE, b"__le"),
        (mi::ADD, b"__add"),
        (mi::SUB, b"__sub"),
        (mi::MUL, b"__mul"),
        (mi::DIV, b"__div"),
        (mi::MOD, b"__mod"),
        (mi::POW, b"__pow"),
        (mi::UNM, b"__unm"),
        (mi::CONCAT, b"__concat"),
        (mi::LEN, b"__len"),
        (mi::CALL, b"__call"),
        (mi::METATABLE, b"__metatable"),
        (mi::RILUA_FRAME_MT, b"__rilua_frame_mt"),
        (mi::RILUA_FRAME_REFS, b"__rilua_frame_refs"),
        (mi::SIM_PRINT, b"__sim_print"),
        (mi::SECUREENV, b"__secureenv"),
        (mi::CVARS, b"__cvars"),
        (mi::ORIGINAL_STRING_FORMAT, b"__original_string_format"),
    ];

    for (index, expected) in metatable_pairs {
        assert_eq!(HOT_METATABLE_KEYS[*index], *expected);
    }

    assert_eq!(
        HOT_METATABLE_KEYS[metatable_idx::RILUA_FRAME_MT],
        b"__rilua_frame_mt"
    );

    assert_eq!(
        HOT_FRAME_METHODS[frame_method_idx::SET_TEXT],
        FRAME_METHOD_SET_TEXT.as_bytes()
    );

    // Loader sentinels: every named index must match the corresponding
    // `&str` constant via `.as_bytes()`.
    use loader_sentinel_idx as lsi;
    let pairs: &[(usize, &'static str)] = &[
        (
            lsi::TEMPLATE_INLINE_FUNCTION_NOARGS,
            TEMPLATE_INLINE_FUNCTION_NOARGS,
        ),
        (
            lsi::TEMPLATE_INLINE_FUNCTION_SELF_ID,
            TEMPLATE_INLINE_FUNCTION_SELF_ID,
        ),
        (
            lsi::TEMPLATE_INLINE_FUNCTION_EVENT_VARARGS,
            TEMPLATE_INLINE_FUNCTION_EVENT_VARARGS,
        ),
        (
            lsi::TEMPLATE_INLINE_FUNCTION_BUTTON,
            TEMPLATE_INLINE_FUNCTION_BUTTON,
        ),
        (
            lsi::TEMPLATE_INLINE_FUNCTION_ELAPSED,
            TEMPLATE_INLINE_FUNCTION_ELAPSED,
        ),
        (
            lsi::TEMPLATE_INLINE_FUNCTION_SELF_STRING,
            TEMPLATE_INLINE_FUNCTION_SELF_STRING,
        ),
        (
            lsi::TEMPLATE_INLINE_FUNCTION_STRING_ARG,
            TEMPLATE_INLINE_FUNCTION_STRING_ARG,
        ),
        (
            lsi::TEMPLATE_INLINE_FUNCTION_GLOBAL_ARG,
            TEMPLATE_INLINE_FUNCTION_GLOBAL_ARG,
        ),
        (
            lsi::TEMPLATE_INLINE_FUNCTION_TWO_GLOBAL_ARGS,
            TEMPLATE_INLINE_FUNCTION_TWO_GLOBAL_ARGS,
        ),
        (
            lsi::TEMPLATE_GLOBAL_METHOD_HANDLER,
            TEMPLATE_GLOBAL_METHOD_HANDLER,
        ),
    ];
    for (idx, expected) in pairs {
        assert_eq!(
            HOT_LOADER_SENTINELS[*idx],
            expected.as_bytes(),
            "loader_sentinel_idx {idx} drift",
        );
    }
}

/// Second call to `install` on the same VM must return equivalent
/// handles (same arena pointers), exercising rilua's static intern
/// cache hit path.
#[test]
fn second_install_returns_same_handles_via_cache_hit() {
    use rilua::{Lua, LuaApiMut};

    let mut lua = Lua::new().expect("fresh rilua VM");
    let first = HotLiteralRegistry::install(lua.state_mut());
    let second = HotLiteralRegistry::install(lua.state_mut());

    for i in 0..HOT_GLOBALS.len() {
        assert_eq!(
            first.global(i),
            second.global(i),
            "global[{i}] handle differs between installs",
        );
    }
    for i in 0..HOT_NAMESPACES.len() {
        assert_eq!(
            first.namespace(i),
            second.namespace(i),
            "namespace[{i}] handle differs between installs",
        );
    }
}

/// Regression: handles must survive a full GC cycle triggered after
/// install. Rilua's `intern_string_static` registers each cache entry
/// as a GC root; this test pins that contract from the wow-ui-sim side.
/// If the GC ever collects a static-cache entry, the decode below
/// would panic on a missing / freed arena slot.
#[test]
fn handles_survive_full_gc_cycle() {
    use rilua::{Lua, LuaApiMut};

    let mut lua = Lua::new().expect("fresh rilua VM");
    let handles = HotLiteralRegistry::install(lua.state_mut());

    // Force a complete mark-and-sweep cycle. `intern_string_static`'s
    // mid-cycle protection is exercised implicitly since the install
    // above runs outside GC, but the full cycle here exercises the
    // `mark_gc_roots` path that preserves the static cache.
    lua.gc_collect().expect("full gc");

    // Every handle in every category must still decode to its source bytes.
    let state = lua.state_mut();
    let categories: &[(&'static str, &[&[u8]], &[GcRef<LuaString>])] = &[
        ("globals", HOT_GLOBALS, &handles.globals),
        ("namespaces", HOT_NAMESPACES, &handles.namespaces),
        ("frame_methods", HOT_FRAME_METHODS, &handles.frame_methods),
        (
            "metatable_keys",
            HOT_METATABLE_KEYS,
            &handles.metatable_keys,
        ),
        (
            "loader_sentinels",
            HOT_LOADER_SENTINELS,
            &handles.loader_sentinels,
        ),
    ];
    for (name, src, refs) in categories {
        for (i, (bytes, r)) in src.iter().zip(refs.iter()).enumerate() {
            let s = state
                .gc
                .string_arena
                .get(*r)
                .unwrap_or_else(|| panic!("{name}[{i}] handle dangling after gc_collect"));
            assert_eq!(
                s.data(),
                *bytes,
                "{name}[{i}] handle decoded to wrong bytes after gc_collect",
            );
        }
    }
}

/// Regression: installing through the real wow-ui-sim bootstrap path
/// (`WowLuaEnv::new`) populates `WowLuaAppData.hot_literals` with a
/// full-length `HotLiteralHandles`. Pins the invariant that the
/// `register_bootstrap_globals` prewarm step actually ran.
#[test]
fn bootstrap_populates_app_data_hot_literals() {
    use crate::lua_api::WowLuaEnv;
    use crate::lua_api::env::WowLuaAppData;
    use rilua::LuaApi;

    let env = WowLuaEnv::new().expect("fresh wow lua env");
    let lua = env.lua.borrow();
    let app = lua
        .state()
        .app_data::<WowLuaAppData>()
        .expect("app data present");
    let handles = app
        .hot_literals
        .as_ref()
        .expect("hot_literals populated by bootstrap prewarm");
    assert_eq!(handles.len(), HOT_LITERAL_COUNT);
}

/// The shared metatable accessor should return the bootstrap prewarmed
/// handle when app_data already holds the registry.
#[test]
fn hot_metatable_key_prefers_bootstrap_handle() {
    use crate::lua_api::WowLuaEnv;
    use rilua::{LuaApi, LuaApiMut};

    let env = WowLuaEnv::new().expect("fresh wow lua env");
    let mut lua = env.rilua_mut();
    for index in [
        metatable_idx::RILUA_FRAME_MT,
        metatable_idx::RILUA_FRAME_REFS,
    ] {
        let expected = {
            let state = lua.state();
            state
                .app_data::<crate::lua_api::env::WowLuaAppData>()
                .and_then(|app| app.hot_literals.as_ref())
                .expect("hot_literals populated")
                .metatable_key(index)
        };
        let actual = hot_metatable_key(lua.state_mut(), index);
        assert_eq!(actual, expected);
    }
}

/// The shared metatable accessor must still fall back when no bootstrap
/// registry has been installed yet.
#[test]
fn hot_metatable_key_falls_back_without_bootstrap_registry() {
    use rilua::{Lua, LuaApiMut};

    let mut lua = Lua::new().expect("fresh rilua VM");
    for index in [
        metatable_idx::RILUA_FRAME_MT,
        metatable_idx::RILUA_FRAME_REFS,
    ] {
        let expected = {
            let state = lua.state_mut();
            state.gc.intern_string_static(HOT_METATABLE_KEYS[index])
        };
        let actual = hot_metatable_key(lua.state_mut(), index);
        assert_eq!(actual, expected);
    }
}
