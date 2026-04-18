# Track 2 sub-item 1: compiler / runtime `intern_string` audit

Inventory of every `state.gc.intern_string(...)` call site in the
`wow-ui-sim` crate, done as the entry point into Track 2 ("reuse
interned handles end-to-end in compiler / VM hot paths").

Rationale: Track 1 landed a pre-intern whitelist and registry, but the
bulk of intern traffic still flows through helpers that take raw
`&[u8]` / `&str` and content-hash them through `intern_string` on every
call. Track 2 converts those helpers to accept an already-interned
`GcRef<LuaString>` handle so callers in hot loops (globals lookup,
metatable access, event dispatch, path walking) stop repaying the
content-hash cost.

## Baseline numbers

From the Track 1 sub-item 4 measurement harness
(`wow-cli --features intern-stats startup-intern-stats`):

    total_calls=57304 unique_strings=49629

Top-7 (64+ calls) entries at HEAD:

    144 x ""                 # empty string — likely a default/fallback
    118 x "__fontFlags"      # FontString field sink
    118 x "__textColorR"     # ditto
    118 x "__fontHeight"     # ditto
    118 x "__textColorG"     # ditto
    118 x "__textColorB"     # ditto
    105 x "1"                # stringified index
     61 x "None"             # seed-value constant
     59 x "__justifyH"       # FontString field sink (repeats)
     59 x "__shadowColorR"   # ditto, 6x color/justify/name fields

`__fontFlags` / `__textColor*` / `__fontHeight` dominate because
`text_attribute_event::events.rs` and `callbacks.rs` intern
`field_name.as_bytes()` fresh per event / per dispatch.

## Progress note

The shared `__index` / `__newindex` metatable key path is now threaded
through the prewarmed hot-literal registry via `hot_metatable_key(...)`.
`methods.rs`, `globals/create_frame/helpers.rs`, `globals/security.rs`,
and `env_init/freeze_globals.rs` all reuse the registry handle when
bootstrap has already installed it, and fall back to the static cache
only in bootstrap-skipping tests.

## Call-site inventory (80 total, `src/` only)

Generated via:

    grep -rn "intern_string(" src/ \
        | grep -v "intern_string_static\|//\|intern_stats\|fn intern_string\|Repos/rilua"

### Static-literal sites (12) — direct conversions to `intern_string_static`

These pass a `b"..."` byte-string literal. Trivial migration target.

| Site | Literal |
|---|---|
| `frame/methods/text_attribute_event/events.rs:565` | `b"_s"` |
| `globals/create_frame/dropdown_api.rs:149` | `b"text"` |
| `globals/missing_surface/tooltip_info/builders.rs:18` | `b"CreateColor"` |
| `globals/missing_surface/tooltip_info/builders.rs:133` | `b"ITEM_BIND_ON_PICKUP"` |
| `globals/stubs/mod.rs:53` | `b"NONE"` |
| `globals/utility_system_spell/c_xml_util.rs:89` | `b"keyValues"` |
| `globals/utility_system_spell/table_util.rs:164` | `b"C_TableUtil"` |
| `globals/utility_system_spell/c_spec.rs:135` | `b"UIWidgetContainerMixin"` |
| `globals/utility_system_spell/c_addons.rs:132` | `b"ADDON_ACTIONS_BLOCKED"` |
| `timer_layout.rs:231` | `b"__id"` |
| `timer_layout.rs:405` | `b"C_Timer"` |
| `timer_layout.rs:431` | `b"__rilua_layout_fns"` |

Note: `C_TableUtil`, `C_Timer`, `ADDON_ACTIONS_BLOCKED` would benefit
from living in `HOT_NAMESPACES` / `HOT_GLOBALS` so the single shared
static intern cache entry covers them.

### Dynamic-string sites (68) — handle-threading candidates

The 68 sites that pass a `&str` / `&[u8]` computed at runtime are the
real Track 2 targets. Grouped by pattern and hotness:

#### Helper: `get_global` / `set_global_*` / `ensure_global_table` (~12 sites)

`globals/create_frame/helpers.rs:{16,32,95}` plus the
parallel `security.rs` / `freeze_globals.rs` copies. All wrap
`intern_string(name.as_bytes())` where `name: &str`.

Conversion shape: add `get_global_by_key(state, key: GcRef<LuaString>)`
and similar so callers holding a pre-interned handle (from
`HotLiteralHandles`) can skip the content-hash step. Existing
`&str` entry points wrap the new handle-based variant.

#### Path-segment walker: `resolve_table_path` (1 site, fan-out per addon XML)

`globals/create_frame/helpers.rs:168`. Interns each
`segment.as_bytes()` in a `path.split('.')` loop. Covered earlier by
Track 1 follow-up notes — conversion to handles needs a
`&[u8]` → `&'static [u8]` map over `HOT_NAMESPACES` plus a handle
vector, which is Track 3 territory. Leave as-is here.

#### Frame-field helpers: `set_frame_field` / `get_frame_field` / `table_get_str` (3 sites)

`globals/create_frame/helpers.rs:{74,95}` plus `helpers_shared.rs:105`.
Pattern: frame-fields table indexed by `&str`. Covered by a separate
follow-up once the field-name set is enumerated (`__fontFlags` /
`__textColor*` / etc. are already in `HOT_METATABLE_KEYS`).

#### Event dispatch: `text_attribute_event` (4 sites in events.rs / callbacks.rs)

`events.rs:{359,393}` + `callbacks.rs:69` — all intern
`event.as_bytes()` per dispatch. Events are a closed enumeration (one
handle per event name in the Event enum would suffice). High-volume
because every fired event hits at least one of these sites.

#### Button anchor / hierarchy: `shared.rs` (3 sites)

`frame/methods/button_anchor_hierarchy/shared.rs:{30,65,85}` — all
intern `name.as_bytes()` for parent-key lookups. Same story as the
frame-field helpers.

#### Font-strings / misc surface (~10 sites)

`globals/font_strings_collection/mod.rs:46`,
`globals/missing_surface/item_spell/helpers.rs:95`, voice_chat,
various `utility_system_spell/*` — each interns a name read from
parsed XML or from a SimState struct at dispatch time. Closed
enumeration in each case (font name list, item category list, etc.)
but the per-module sets are too large to hoist into a single
whitelist without more measurement.

#### `timer_layout.rs` and `methods.rs` plumbing (7 sites)

Layout-timer plumbing and the core `methods.rs` helpers. These are
one-shot or infrequent — low priority for handle threading.

#### `saved_variables.rs` / `taint.rs` / bridge glue (~6 sites)

Each already runs once at load / shutdown. Not in the hot loop.

### Top offenders by file

Aggregate count of `intern_string` calls per file (across both the
static-literal and dynamic buckets):

    9  src/lua_api/globals/utility_system_spell/mod.rs
    8  src/lua_api/script_helpers.rs
    7  src/lua_api/methods.rs
    7  src/lua_api/globals/stubs/mod.rs
    6  src/lua_api/timer_layout.rs
    5  src/lua_api/globals/create_frame/helpers.rs
    3  src/lua_api/globals/utility_system_spell/c_xml_util.rs
    3  src/lua_api/globals/lua_duration_object.rs
    3  src/lua_api/frame/methods/text_attribute_event/events.rs
    3  src/lua_api/frame/methods/button_anchor_hierarchy/shared.rs
    …

Several of these (`utility_system_spell/mod.rs`, `script_helpers.rs`,
`methods.rs`, `stubs/mod.rs`) are registration-time code and run once
per bootstrap; they're noisy in the count but not in per-addon
dispatch cost.

## Conversion priority for Track 2 sub-items 2–3

Hotness-ranked, based on the intern-stats top-N plus the audit buckets:

1. `text_attribute_event` event-name + field-name threading — clears
   the ~118× `__fontFlags`/`__textColor*`/`__fontHeight` top cluster.
2. Frame-field helpers (`get_frame_field` / `set_frame_field` /
   `helpers_shared::table_get_str`) — the same field-name set shows
   up here and in text_attribute_event.
3. `button_anchor_hierarchy::shared` parent-key lookups — 3 sites,
   lower volume but touch every frame at layout time.
4. Static-literal batch cleanup — the 12 `b"..."` sites in
   `timer_layout`, `stubs`, `utility_system_spell`, and
   `missing_surface/tooltip_info/builders` are trivial conversions
   to `intern_string_static` and remove them from the dynamic audit.
5. Path-segment walker (`resolve_table_path`) — punt to Track 3.

## Out of scope for Track 2

- The rilua VM's internal `intern_string` traffic from
  `patch_string_constants`, `compile_or_undump`, and bytecode load.
  The VM owns those; Track 2 scopes to `wow-ui-sim` callers only.
- `&str` ↔ `GcRef<LuaString>` conversions in the `lua_bridge` layer
  (`FromStack` / `IntoStack`). Those cross the Lua/Rust boundary and
  have their own lifetime constraints.
- Tests / fixtures — intern_string calls in `tests/` and
  `#[cfg(test)]` code are once-per-test and not in the hot path.
