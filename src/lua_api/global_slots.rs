//! Track 3 sub-item 2: frozen slot vector + `_G_live`-aware read path.
//!
//! Populates a `Box<[Val]>` at bootstrap (after `register_globals` and
//! after the optional `freeze_globals_with_live_shadow`) by walking the
//! Track 1 whitelist and resolving each name's current value from `_G`.
//! The vector's index layout is fixed by the ABI design in
//! `docs/wiki/design/track-3-global-slot-abi.md`:
//!
//!   - slot 0              → `_G`
//!   - slots 1..=35        → [`HOT_GLOBALS`] (35 entries)
//!   - slots 36..=132      → [`HOT_NAMESPACES`] (97 entries)
//!
//! Total slot count in ABI v1: `1 + HOT_GLOBALS.len() + HOT_NAMESPACES.len()`
//! (= 133 for `WHITELIST_VERSION == 1`).
//!
//! The read path [`read_slot`] checks `_G_live` first. In the common
//! case (shadow empty — true through the whole freeze-disabled bootstrap
//! and through most addon loads when freeze *is* enabled), the check is
//! `array_len() == 0 && hash_size() == 0` on the shadow table, then the
//! frozen slot value is returned directly. When the shadow has entries,
//! fall back to a hashed `_G_live.name` lookup — if that's non-nil, the
//! addon-author override wins; otherwise the frozen slot value is returned.
//!
//! This module stands up the populator + read path so later Track 3
//! sub-items can build on it:
//!
//!   - Sub-item 3 will teach the rilua compiler to rewrite
//!     `GETGLOBAL <name>` → `GETGLOBAL_SLOT <idx>` for whitelisted
//!     names and have the VM dispatch through [`read_slot`].
//!   - Sub-item 4 bumps the bytecode cache key on
//!     [`WHITELIST_VERSION`] so stale slot indexes cannot interpret
//!     against a new whitelist.
//!   - Sub-item 5 adds parity tests and a startup perf comparison.

use crate::lua_api::hot_literals::{HOT_GLOBALS, HOT_NAMESPACES, WHITELIST_VERSION};
use rilua::Val;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::string::LuaString;

/// Registry key for the `_G_live` shadow table installed by
/// `freeze_globals_with_live_shadow`. Mirrors the constant declared
/// in `env_init::freeze_globals` (kept private there).
const G_LIVE_REGISTRY_KEY: &[u8] = b"__rilua_g_live";

/// Total slot count in the current ABI. Ties directly to
/// [`WHITELIST_VERSION`] so bumping the version + the whitelist
/// entries is a single coordinated change.
pub const SLOT_COUNT: usize = 1 + HOT_GLOBALS.len() + HOT_NAMESPACES.len();

/// Slot index for `_G` itself. Always 0 by ABI.
pub const SLOT_G: usize = 0;

/// First slot reserved for [`HOT_GLOBALS`] entries. Slot
/// `GLOBALS_BASE + i` is `HOT_GLOBALS[i]`.
pub const GLOBALS_BASE: usize = 1;

/// First slot reserved for [`HOT_NAMESPACES`] entries. Slot
/// `NAMESPACES_BASE + i` is `HOT_NAMESPACES[i]`.
pub const NAMESPACES_BASE: usize = GLOBALS_BASE + HOT_GLOBALS.len();

/// The populated slot vector, owned by the wow-ui-sim app-data.
///
/// Built once during bootstrap by [`install`] and stashed on
/// `WowLuaAppData.global_slots`. The `GcRef<...>` values inside each
/// `Val::Table` reference tables that are already pinned by bootstrap
/// (frozen `_G` or namespace tables reachable from it), so the slot
/// vector stays valid for the life of the VM without additional rooting.
#[derive(Clone)]
pub struct GlobalSlotTable {
    values: Box<[Val]>,
    /// Pre-interned key per slot. Slot 0 (`_G`) has no name; its entry
    /// is the g_live_key sentinel, never read because [`read_slot`]
    /// short-circuits on `idx == 0`.
    name_keys: Box<[GcRef<LuaString>]>,
    /// Pre-interned `"__rilua_g_live"` registry key. Cached once so the
    /// read path never calls `intern_string_static` (which needs `&mut`).
    g_live_key: GcRef<LuaString>,
    /// Whitelist version the slot indexes were computed against.
    /// Sub-item 4 wires this into the bytecode cache key so stale
    /// cached ops can't interpret against a newer version.
    version: u32,
}

impl GlobalSlotTable {
    /// Current slot count. Always equals [`SLOT_COUNT`] for the
    /// version this table was built against.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Whitelist version the slot indexes were computed against.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Raw slot read (no `_G_live` shadow check). Intended for the
    /// future `GETGLOBAL_SLOT` opcode dispatch when the compiler has
    /// already proven the shadow is empty, or when measurement tooling
    /// wants the frozen value.
    pub fn raw(&self, idx: usize) -> Val {
        self.values[idx]
    }
}

/// Walk the whitelist against the current `_G` table and populate a
/// fresh [`GlobalSlotTable`]. Safe to call multiple times — each call
/// re-resolves against whatever `_G` currently points at (though
/// post-freeze the values are fixed, so repeat calls should produce
/// identical results).
///
/// The resolution is intentionally cheap: `Table::get_str` against
/// the frozen `_G` is an O(1) hashed lookup, but it happens exactly
/// once per slot at bootstrap, not per access. Nil entries (names in
/// the whitelist that don't exist in `_G`) are stored as `Val::Nil`
/// — the slot read path will then fall back to the `_G_live`-aware
/// path which gives the addon-author-defined value if any.
pub fn install(state: &mut LuaState) -> GlobalSlotTable {
    let mut values: Vec<Val> = Vec::with_capacity(SLOT_COUNT);
    let mut name_keys: Vec<GcRef<LuaString>> = Vec::with_capacity(SLOT_COUNT);

    // Pre-intern the `_G_live` registry key so the read path can look
    // up the shadow without needing `&mut state`.
    let g_live_key = state.gc.intern_string_static(G_LIVE_REGISTRY_KEY);

    // Slot 0: `_G` itself. The VM exposes the global table as a
    // `GcRef<Table>` on `state.global`; wrap it as `Val::Table`.
    values.push(Val::Table(state.global));
    name_keys.push(g_live_key); // sentinel — never read for slot 0.

    // Slots for HOT_GLOBALS then HOT_NAMESPACES, in the whitelist
    // order. Each resolves `_G[name]` via `intern_string_static` +
    // `get_str` — one-shot cost per bootstrap.
    let global_ref = state.global;
    for &name_bytes in HOT_GLOBALS.iter().chain(HOT_NAMESPACES.iter()) {
        let key = state.gc.intern_string_static(name_bytes);
        let val = state
            .gc
            .tables
            .get(global_ref)
            .map(|g| g.get_str(key, &state.gc.string_arena))
            .unwrap_or(Val::Nil);
        values.push(val);
        name_keys.push(key);
    }

    debug_assert_eq!(values.len(), SLOT_COUNT);
    debug_assert_eq!(name_keys.len(), SLOT_COUNT);
    GlobalSlotTable {
        values: values.into_boxed_slice(),
        name_keys: name_keys.into_boxed_slice(),
        g_live_key,
        version: WHITELIST_VERSION,
    }
}

/// Read a slot, honoring the `_G_live` shadow if it has any entries.
///
/// Walk:
///   1. Slot 0 (`_G`) bypasses the shadow — the frozen global table is
///      the shadow's `__index` target, so there's no override path.
///   2. Resolve the shadow table via the pre-interned
///      `__rilua_g_live` registry key. If absent (freeze never ran) or
///      empty (common case through most of startup), return the raw
///      slot value directly.
///   3. Otherwise look the slot's pre-interned name key up on the
///      shadow; return the override if non-nil, else the raw slot.
pub fn read_slot(state: &LuaState, slots: &GlobalSlotTable, idx: usize) -> Val {
    if idx == SLOT_G {
        return slots.raw(idx);
    }

    let Some(live_ref) = lookup_g_live(state, slots.g_live_key) else {
        return slots.raw(idx);
    };
    let Some(live_table) = state.gc.tables.get(live_ref) else {
        return slots.raw(idx);
    };
    if live_table.array_len() == 0 && live_table.hash_size() == 0 {
        return slots.raw(idx);
    }

    let key = slots.name_keys[idx];
    let live_val = live_table.get_str(key, &state.gc.string_arena);
    if live_val != Val::Nil {
        return live_val;
    }
    slots.raw(idx)
}

/// Look up the `_G_live` shadow table via the registry, using the
/// pre-interned key cached on [`GlobalSlotTable`]. Returns `None` if
/// the registry entry is absent or not a table (e.g. in unit tests that
/// skip `freeze_globals_with_live_shadow`).
fn lookup_g_live(
    state: &LuaState,
    g_live_key: GcRef<LuaString>,
) -> Option<rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>> {
    let registry = state.gc.tables.get(state.registry)?;
    match registry.get_str(g_live_key, &state.gc.string_arena) {
        Val::Table(r) => Some(r),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_api::WowLuaEnv;
    use crate::lua_api::env::WowLuaAppData;
    use rilua::{Lua, LuaApi, LuaApiMut};

    #[test]
    fn slot_count_matches_whitelist_size() {
        assert_eq!(SLOT_COUNT, 1 + HOT_GLOBALS.len() + HOT_NAMESPACES.len());
        assert!(SLOT_COUNT >= 133, "ABI v1 has at least 133 slots");
    }

    #[test]
    fn slot_bases_match_abi_v1() {
        assert_eq!(SLOT_G, 0);
        assert_eq!(GLOBALS_BASE, 1);
        assert_eq!(NAMESPACES_BASE, 1 + HOT_GLOBALS.len());
    }

    #[test]
    fn install_populates_g_at_slot_zero() {
        let mut lua = Lua::new().expect("fresh rilua VM");
        let slots = install(lua.state_mut());
        assert_eq!(slots.len(), SLOT_COUNT);
        assert_eq!(slots.version(), WHITELIST_VERSION);
        match slots.raw(SLOT_G) {
            Val::Table(r) => assert_eq!(r, lua.state().global),
            other => panic!("expected Val::Table(_G), got {other:?}"),
        }
    }

    #[test]
    fn bootstrap_populates_global_slots_on_app_data() {
        let env = WowLuaEnv::new().expect("fresh wow lua env");
        let lua = env.lua.borrow();
        let app = lua
            .state()
            .app_data::<WowLuaAppData>()
            .expect("app data present");
        let slots = app
            .global_slots
            .as_ref()
            .expect("global_slots populated by bootstrap");
        assert_eq!(slots.len(), SLOT_COUNT);
        assert_eq!(slots.version(), WHITELIST_VERSION);
    }

    #[test]
    fn read_slot_returns_g_for_slot_zero() {
        let env = WowLuaEnv::new().expect("fresh wow lua env");
        let lua = env.lua.borrow();
        let state = lua.state();
        let app = state.app_data::<WowLuaAppData>().expect("app data");
        let slots = app.global_slots.as_ref().expect("global_slots");
        match read_slot(state, slots, SLOT_G) {
            Val::Table(r) => assert_eq!(r, state.global),
            other => panic!("expected _G table, got {other:?}"),
        }
    }

    #[test]
    fn read_slot_surfaces_shadow_entry_for_slot_that_was_nil_at_install() {
        // The current freeze gate (`env_init::freeze_globals`) rejects
        // writes to existing `_G` keys with "attempt to modify a frozen
        // table" — only keys that were MISSING at freeze time flow
        // through `__newindex` → `_G_live`. The slot read path must
        // surface that shadow entry, and this test pins that contract
        // without relying on the freeze pipeline (which requires
        // `WOW_SIM_FREEZE_GLOBALS=1` + a full bootstrap).
        //
        // Build the scenario by hand on a fresh rilua VM:
        //   1. Install slots against an empty `_G` — `Mixin` resolves to
        //      `Val::Nil` in the slot vector.
        //   2. Alloc a `_G_live` table, register it under
        //      `__rilua_g_live`, and raw-set the `Mixin` key to a
        //      sentinel value on the shadow.
        //   3. Read the slot. Expect the sentinel — the shadow-lookup
        //      branch of `read_slot`.
        use rilua::vm::table::Table;

        let mut lua = Lua::new().expect("fresh rilua VM");
        let state = lua.state_mut();
        let slots = install(state);
        let mixin_idx = HOT_GLOBALS
            .iter()
            .position(|&name| name == b"Mixin")
            .map(|i| GLOBALS_BASE + i)
            .expect("Mixin is in HOT_GLOBALS");
        assert_eq!(
            slots.raw(mixin_idx),
            Val::Nil,
            "Mixin should be Nil in slot vector before any addon load"
        );

        let live_ref = state.gc.alloc_table(Table::new());
        let live_key = state.gc.intern_string_static(G_LIVE_REGISTRY_KEY);
        let mixin_key = state.gc.intern_string_static(b"Mixin");
        let sentinel = state.gc.intern_string_static(b"shadow-sentinel");
        let registry_ref = state.registry;
        let strings_ptr: *const _ = &state.gc.string_arena;
        if let Some(live_table) = state.gc.tables.get_mut(live_ref) {
            // SAFETY: immutable borrow of `string_arena` while we hold
            // a mutable borrow of the live table. No arena mutation
            // happens through this borrow — it's only used for hashing
            // the string key during `raw_set`.
            let strings = unsafe { &*strings_ptr };
            let _ = live_table.raw_set(Val::Str(mixin_key), Val::Str(sentinel), strings);
        }
        if let Some(reg_table) = state.gc.tables.get_mut(registry_ref) {
            let strings = unsafe { &*strings_ptr };
            let _ = reg_table.raw_set(Val::Str(live_key), Val::Table(live_ref), strings);
        }

        match read_slot(state, &slots, mixin_idx) {
            Val::Str(s_ref) => {
                let s = state.gc.string_arena.get(s_ref).expect("live string");
                assert_eq!(s.data(), b"shadow-sentinel");
            }
            other => panic!("expected shadow sentinel, got {other:?}"),
        }
    }
}
