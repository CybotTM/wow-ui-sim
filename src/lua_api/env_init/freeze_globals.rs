//! Freeze `_G` (and its shared metatable) after bootstrap, redirect
//! future writes to a mutable `_G_live` shadow table.
//!
//! Layout after install:
//!
//! - `_G` — frozen. Reads hit the same slots as before; direct writes
//!   raise "attempt to modify a frozen table". Its metatable is the
//!   new proxy (see below).
//! - `_G_live` — mutable shadow table, pinned so sweep never collects
//!   it. Carries all writes that happen after freeze.
//! - Proxy metatable — pinned. `__index` = `_G_live`, so reads of
//!   missing-in-`_G` keys fall through. `__newindex` = `_G_live`, so
//!   writes to missing-in-`_G` keys land in the shadow.
//!
//! Writes to keys that already exist in `_G` still go through
//! `Table::raw_set` → the frozen gate → Lua runtime error. That is
//! intentional: it flags any bootstrap-registered global being
//! overwritten mid-addon-load so callers can migrate the write to
//! `_G_live` or unfreeze explicitly.
//!
//! Ordering relative to other init steps:
//!
//! 1. Finish all bootstrap writes to `_G`.
//! 2. `finalize_bootstrap_gc` drops transients so they aren't pinned.
//! 3. This function: create shadow + proxy, freeze `_G`, attach proxy.
//!
//! The shadow + proxy must be created BEFORE `freeze_table(_G)` so
//! they can be pinned first (otherwise the freeze walk would make
//! them immutable — the opposite of what we want). They are attached
//! to `_G` AFTER freeze so the freeze walk does not follow them.

use crate::lua_api::hot_literals::{hot_metatable_key, metatable_idx};
use rilua::vm::table::Table;
use rilua::{LuaApiMut, Val};

const G_LIVE_REGISTRY_KEY: &str = "__rilua_g_live";
const G_PROXY_METATABLE_REGISTRY_KEY: &str = "__rilua_g_proxy_mt";
const SECUREENV_LIVE_REGISTRY_KEY: &str = "__rilua_secureenv_live";
const SECUREENV_PROXY_METATABLE_REGISTRY_KEY: &str = "__rilua_secureenv_proxy_mt";

pub(super) fn freeze_globals_with_live_shadow(lua: &mut rilua::Lua) -> crate::Result<()> {
    freeze_root_with_shadow(
        lua,
        FreezeTarget::Global,
        G_LIVE_REGISTRY_KEY,
        G_PROXY_METATABLE_REGISTRY_KEY,
    )?;
    if let Some(secureenv_ref) = find_secureenv(lua) {
        freeze_root_with_shadow(
            lua,
            FreezeTarget::Secureenv(secureenv_ref),
            SECUREENV_LIVE_REGISTRY_KEY,
            SECUREENV_PROXY_METATABLE_REGISTRY_KEY,
        )?;
    }
    Ok(())
}

fn find_secureenv(lua: &mut rilua::Lua) -> Option<rilua::vm::gc::arena::GcRef<Table>> {
    match LuaApiMut::get_global_val(lua, "__secureenv") {
        Val::Table(r) => Some(r),
        _ => None,
    }
}

enum FreezeTarget {
    Global,
    Secureenv(rilua::vm::gc::arena::GcRef<Table>),
}

fn freeze_root_with_shadow(
    lua: &mut rilua::Lua,
    target: FreezeTarget,
    live_key: &'static str,
    proxy_key: &'static str,
) -> crate::Result<()> {
    let state = lua.state_mut();

    let live = state.gc.alloc_table(Table::new());
    state.gc.pin_object(Val::Table(live));

    let proxy_mt = build_proxy_metatable(state, live);
    state.gc.pin_object(Val::Table(proxy_mt));

    // Freeze the root and its existing transitively-reachable children
    // BEFORE attaching the proxy, so the freeze walk does not follow
    // into live / proxy_mt.
    let root = match target {
        FreezeTarget::Global => state.global,
        FreezeTarget::Secureenv(r) => r,
    };
    let stats = state.gc.freeze_table(root);
    tracing::debug!(
        target = ?match target {
            FreezeTarget::Global => "_G",
            FreezeTarget::Secureenv(_) => "__secureenv",
        },
        tables = stats.tables,
        closures = stats.closures,
        userdata = stats.userdata,
        upvalues = stats.upvalues,
        "freeze_table done"
    );

    attach_proxy_metatable(state, root, proxy_mt);
    stash_shadow_refs_in_registry(state, live_key, live, proxy_key, proxy_mt);
    Ok(())
}

fn build_proxy_metatable(
    state: &mut rilua::vm::state::LuaState,
    g_live: rilua::vm::gc::arena::GcRef<Table>,
) -> rilua::vm::gc::arena::GcRef<Table> {
    let mt = state.gc.alloc_table(Table::new());
    let index_key = hot_metatable_key(state, metatable_idx::INDEX);
    let newindex_key = hot_metatable_key(state, metatable_idx::NEWINDEX);
    let mt_table = state
        .gc
        .tables
        .get_mut(mt)
        .expect("freshly allocated proxy metatable must be live");
    let _ = mt_table.raw_set(
        Val::Str(index_key),
        Val::Table(g_live),
        &state.gc.string_arena,
    );
    let _ = mt_table.raw_set(
        Val::Str(newindex_key),
        Val::Table(g_live),
        &state.gc.string_arena,
    );
    mt
}

fn attach_proxy_metatable(
    state: &mut rilua::vm::state::LuaState,
    g_ref: rilua::vm::gc::arena::GcRef<Table>,
    proxy_mt: rilua::vm::gc::arena::GcRef<Table>,
) {
    // set_metatable is a direct field write on Table and bypasses the
    // raw_set frozen guard, so this works on the already-frozen _G.
    if let Some(g_table) = state.gc.tables.get_mut(g_ref) {
        g_table.set_metatable(Some(proxy_mt));
    }
}

fn stash_shadow_refs_in_registry(
    state: &mut rilua::vm::state::LuaState,
    live_key: &'static str,
    live: rilua::vm::gc::arena::GcRef<Table>,
    proxy_key: &'static str,
    proxy_mt: rilua::vm::gc::arena::GcRef<Table>,
) {
    use crate::lua_api::methods::registry_set;
    registry_set(state, live_key, Val::Table(live));
    registry_set(state, proxy_key, Val::Table(proxy_mt));
}
