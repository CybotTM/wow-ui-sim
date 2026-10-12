# Edit Mode layout selection

Current Retail/PTR startup must preserve the active-layout index supplied by the character cache in Blizzard's combined preset-plus-custom selection space. The unchanged Blizzard manager prepends Modern/Classic presets without offsetting `activeLayout`. In the observed cache, index2 therefore selects Classic, not custom Widescreen. User WTF data must not be changed or orientation forced to hide a selection error.

## What it must do

- [ ] Import provided account/character cache files before whole unchanged Game UI startup; preserve the character's active index through preset insertion. Native cache index2 selects Classic with horizontal Micro Menu geometry, even when the second custom layout is vertical Widescreen.
- [ ] Preserve saved layout content, names/identifiers and existing named environment/snapshot override selection and precedence. Resolve an explicit saved-layout name to the combined selection space; do not reinterpret native character indices as custom-list positions.
- [ ] Keep selection, cache decoding and original consumer behavior distinct. No original WTF writes, vendor patches, forced orientation or guessed new numeric-override contract.

## How it works

- [Edit Mode investigation](../wiki/investigations/editmode-layout.md)
- [Anchor replay contract](edit-mode-initial-anchors.md)

## Implementation inventory

- `src/saved_variables.rs`: existing file import into the modeled cache API.
- `src/lua_api/workarounds/temporary/edit_mode_cache_defaults.rs`: existing Lua-owned cache state, active selection and explicit override resolution.
- `src/lua_api/workarounds/editmode/setup_layout_info.lua`: manager initialization currently incorrectly offsets native indices; repair pending.

## Tests asserting this spec

- [edit_mode_cache_startup.rs](../../tests/edit_mode_cache_startup.rs): fresh environment, isolated real-format file inputs, import before whole Game loading, original startup, selected Classic and public Micro Menu geometry. Not compiled or executed yet.
- [edit_mode_api/cache.rs](../../tests/edit_mode_api/cache.rs): existing parser and named override controls; low-level cache proof is not whole consumer proof.

## Known gaps (current cycle)

- [ ] Genuine fresh-startup RED before selection production changes.
- [ ] Correct model/helper index handling, then focused GREEN and selected named-override preservation.
- [ ] Corrected live GUI and intended-user acceptance; diagnostic runtime mismatch is not fixed/native parity proof.

## Out of scope

Rewriting user layouts, native pixel/account parity, new invalid-index policies, new numeric configuration semantics and unrelated anchor/renderer redesign.
