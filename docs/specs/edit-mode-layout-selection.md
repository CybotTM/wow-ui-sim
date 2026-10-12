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
- `src/lua_api/workarounds/editmode/setup_layout_info.lua`: manager initialization preserves the active index while prepending presets; explicit override conversion belongs to cache import. Bounded Retail GREEN below; PTR runtime unproved.

## Tests asserting this spec

- [edit_mode_cache_startup.rs](../../tests/edit_mode_cache_startup.rs): fresh environment, isolated real-format file inputs, import before whole Game loading, original startup, selected Classic and public Micro Menu geometry. Native selection genuinely failed at `653e829f1`. At `7d9d9f621`, Retail native Classic/horizontal and four explicit Widescreen/vertical variants completed in two test names; parent/child recaps count once.
- [edit_mode_api/cache.rs](../../tests/edit_mode_api/cache.rs): 10/10 Retail cache/parser/env/snapshot controls PASS at `7d9d9f621`; low-level cache proof is not whole consumer proof.

## Known gaps (current cycle)

- [x] Genuine fresh-startup RED at `653e829f1`: selected-layout assertion failed after real file import/full startup; geometry unreached. Enter3 GREEN and separate strict publication stale-row FAIL do not establish selection GREEN.
- [x] Corrected source and bounded Retail startup2/cache10 GREEN; corrected normal startup Classic329×40/zero Lua errors, fmt/default check PASS. [Packet](../../data/patch-api/evidence/edit-mode-enter-2026-10-12/README.md). PTR runtime remains unproved.
- [ ] Current-artifact owned GUI query observed Classic/horizontal329×40 at native1920×1080 window; user subsequently accepted layout. Keyboard initially failed until native window focus was assigned; user then confirmed chat and `/command` worked. Automatic focus setup gate pending. Native/PTR/pixel parity remains unproved; configured90s expiry was replaced by user-authorized stop-after-test lifecycle. Public packet retention/link/privacy gate PASS through `b1c7c4348`; [gate](../../data/patch-api/evidence/edit-mode-enter-2026-10-12/public-retention-gate.json).

## Out of scope

Rewriting user layouts, native pixel/account parity, new invalid-index policies, new numeric configuration semantics and unrelated anchor/renderer redesign.
