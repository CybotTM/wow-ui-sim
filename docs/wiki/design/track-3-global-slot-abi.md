# Track 3: Global-slot fast-path ABI (design)

Design for the `GETGLOBAL_SLOT` / `SETGLOBAL_SLOT` fast path that
replaces the `string_arena` table lookup on whitelisted globals with
a direct indexed read/write into a frozen slot vector. This is
sub-item 1 of Track 3 — the *design* side; sub-items 2-5 are the
bootstrap populator, compiler integration, bytecode cache version
bump, and proof/measurement.

## Motivation

After Track 2's hot-literal registry + handle threading, the
remaining `intern_string`-adjacent cost during Blizzard startup is
not the string interning itself — it's the `Table::get_str` bucket
walk that every global dereference pays against the frozen `_G`.
Every `return UIParent` or `C_AddOns.IsAddOnLoaded(...)` currently:

1. Intern the global name (ptr-cache hit after Track 1).
2. Hash the key for `_G`'s bucket lookup.
3. Walk the hash chain to find the `Val`.

Steps 2-3 are unavoidable with the normal Lua semantics. But for
names we *know* statically at bytecode-load time (because they're on
the whitelist), we can precompute a slot index during bootstrap and
have the bytecode read a Vec by index — O(1), no hashing, no chain
walk.

## Scope

**In scope for the first ABI version:**

- `_G` itself (slot 0 — the bootstrap table reference).
- `Enum`, `Constants` namespaces.
- The 95 `C_*` namespace tables in `HOT_NAMESPACES`.
- The 35 hot bare globals in `HOT_GLOBALS` (`UIParent`, `WorldFrame`,
  `Mixin`, `CreateFrame`, `GetTime`, `GetCVar`/`SetCVar`, etc.).

**Out of scope:**

- Method names / field keys (`SetPoint`, `__index`, etc.). Those are
  table-member lookups, not global lookups; they live in
  `HOT_FRAME_METHODS` / `HOT_METATABLE_KEYS` and stay on Track
  1/2's `intern_string_static` fast path.
- Dynamically-named globals (`_G[name]` where `name` is a variable).
  These flow through the normal string-arena lookup; the slot path
  only kicks in when the compiler statically sees a
  whitelisted name.
- Slot *allocation* (hash-consing new slots as code runs). The
  whitelist is frozen at bootstrap — adding entries requires bumping
  [`WHITELIST_VERSION`] and rebuilding the bytecode cache.

## Slot numbering

The slot index of an entry in the frozen vector is determined by its
category + position within that category:

    slot(entry) = category_base[entry.category] + entry.index

With `category_base` a compile-time table:

    _G          = 0
    HOT_GLOBALS = 1
    HOT_NAMESPACES = 1 + HOT_GLOBALS.len()

Total slot count = `1 + HOT_GLOBALS.len() + HOT_NAMESPACES.len()` =
1 + 35 + 97 = **133 slots** in ABI v1.

`_G` gets an explicit slot 0 because (a) it's referenced by multiple
Blizzard globals and (b) its `Val::Table` never changes (unlike the
other globals, which the `_G_live` shadow can mask — see next
section). Slot 0 being `_G` itself also makes it the canonical
fallback target for opcodes that hit "slot not resolved"
(`GETGLOBAL_SLOT 0` == `GETGLOBAL _G`).

## `_G_live` shadow handling

`env_init/freeze_globals.rs` already installs a mutable `_G_live`
shadow table after `freeze_table(_G)` so addons can override or add
global names without hitting the frozen-write error. The slot
fast-path must honor those overrides:

    slot_read(i) =
      let live_val = _G_live.get_str(slot_name[i])
      if live_val != Nil {
        return live_val                  // shadow override wins
      }
      return slot_vec[i]                 // direct slot read

The extra `_G_live.get_str` is a hash lookup, which seems to defeat
the whole point. Two options:

1. **Dirty bit per slot.** `_G_live` maintains a bitset indexed by
   slot; `_G_live[name] = value` flips the bit. Slot reads skip the
   `_G_live` lookup when the bit is clear. One conditional branch
   per read in the common (no-override) case.

2. **Shadow table is empty ⇒ skip.** `_G_live.table.len()` is an
   O(1) check. When the shadow has no entries, skip the check
   entirely. Once any addon writes to `_G_live`, every slot read
   pays the hash probe. Simpler, works because the common case
   during bootstrap + many addon loads is `_G_live` untouched.

ABI v1 specifies **option 2** (shadow-empty skip) because it avoids
tying the slot ABI to a dirty-bitset representation that might
evolve. Option 1 can be introduced later without bumping the ABI
version — it's a read-path optimization.

For writes: `SETGLOBAL_SLOT i <val>` always writes through the
shadow (`_G_live[slot_name[i]] = val`), never mutates the slot
vector. This preserves the freeze-gate invariant that the bootstrap
`_G` is immutable after freeze, and means slot-write semantics match
current `_G` write semantics exactly.

## ABI version

`hot_literals::WHITELIST_VERSION` is the canonical slot-ABI version.
Its current value of `1` is the first stable version. Bump rules:

- **Adding entries at the end** of any category slice bumps the
  total slot count but preserves every existing slot's index. Soft
  bump: increment `WHITELIST_VERSION`, old bytecode cache is
  forward-compatible (older indices still point at the same slots)
  but new indices need a recompile. Cache entries keyed on the old
  version can be loaded and extended with the new slots populated
  at the old slot-count boundary.

- **Reordering or removing entries** shifts indices. Hard bump:
  increment `WHITELIST_VERSION` by a larger step (e.g. +10) to make
  the break explicit, and invalidate every cached bytecode hash
  that carries the old version. Avoid if at all possible — it
  triggers a full recompile pass on next startup.

Sub-item 4 (cache version bump) wires `WHITELIST_VERSION` into the
bytecode cache key format so stale slot indexes can't silently
interpret against a new whitelist.

## Interaction with the rilua VM

Track 3 requires two rilua-side changes:

1. **New bytecode ops** `GETGLOBAL_SLOT` and `SETGLOBAL_SLOT` (or a
   single opcode with a read/write flag). Argument: u16 slot index
   (133 slots fit easily).

2. **Per-VM `slot_vec: Box<[Val]>`** populated by wow-ui-sim during
   bootstrap via a new `Gc::install_global_slots(values)` entry
   point. The vec stays alive for the life of the VM; freeze sets
   the slots to their final `Val` values, and opcode dispatch reads
   `state.gc.slot_vec[i]` directly.

Wow-ui-sim's job (sub-item 2) is to walk `HOT_GLOBALS` +
`HOT_NAMESPACES` at bootstrap, resolve each name's current `_G` value,
and pass the resolved vector to rilua.

## Compiler fast-path emission

During `patch_string_constants` (rilua's load-time proto rewrite pass),
whenever the compiler sees a `GETGLOBAL <str>` op whose constant
matches a whitelisted name, it rewrites to `GETGLOBAL_SLOT <idx>`.
The name lookup needs the `&[u8]` → `slot_idx` map built once per VM
(keyed on `WHITELIST_VERSION`).

Non-whitelisted `GETGLOBAL` ops stay untouched — they go through the
normal `Table::get_str` path. This keeps the optimization opt-in per
name, so addon-authored globals (`MyAddonFrame`, `MyAddonConfig`)
never hit the slot fast path (which is correct: those aren't on the
whitelist and the compiler has no way to know their slot).

## Bytecode cache key

Current cache key (in `loader::chunk_cache::tagged_hash`):

    hash(source_bytes) + hash(tag)

New key (sub-item 4):

    hash(source_bytes) + hash(tag) + hash(WHITELIST_VERSION)

This makes every cached bytecode invalidate when the version bumps,
so the slot indexes baked into the cached ops can never point at a
stale whitelist.

## Measurement plan (sub-item 5)

Add a `wow-cli startup-global-slot-stats` command (gated on the
existing `intern-stats` feature or a new one) that reports:

- Slot-lookup count (how often `GETGLOBAL_SLOT` fires).
- Fallback-lookup count (how often a whitelisted name was accessed
  via `GETGLOBAL` because the cache was stale — should be 0 after
  steady state).
- `_G_live`-override count (how often a slot read's shadow check
  fired).

Parity tests: a Lua probe that redefines a whitelisted global
(`Mixin = function() end`) and confirms the override is visible both
via `_G.Mixin` and via bytecode that was compiled against the frozen
slot. This catches the case where the shadow check is accidentally
skipped.

Startup wall-time comparison: release build, `--no-addons
--no-saved-vars`, before vs after the slot ABI lands. Rilua's
`intern-stats` feature is not on the critical path here (the slot
path bypasses `intern_string` entirely for whitelisted names), so
the comparison is plain wall-time.

## Open questions

- **Does rilua's existing `patch_string_constants` pass already
  preserve enough per-proto information to know which `GETGLOBAL` to
  rewrite?** Check before sub-item 3 starts.
- **What's the right moment to call `install_global_slots`?**
  Currently `register_globals` → `finalize_bootstrap_gc` →
  `freeze_globals_with_live_shadow`. The slot vector should be built
  from the post-freeze, pre-addon `_G` values, so right after
  `freeze_globals_with_live_shadow` is the natural place.
- **How does the slot fast-path interact with taint?** `issecure` /
  `issecurevariable` read the stack taint, which isn't affected by
  how the global was fetched. Should be transparent, but worth
  pinning a test for sub-item 5.
