# Wrath minimap, XML and attack-query contracts

Wrath 12340 behavior supplied by laptop batch 3 (2026-10-18); original Minimap Lua is the zoom/ping consumer. Loader tests use controlled collision/failure inputs. Other client profiles keep their existing behavior.

## What it must do

- [ ] M3: SetZoom truncates numeric input to signed integer, converts to unsigned, then upper-clamps to 5; six levels. Changed and unchanged calls emit no MINIMAP_UPDATE_ZOOM. Original button handlers update endpoint enablement.
- [ ] M2: Every player-present PingLocation publishes MINIMAP_PING with unit player, including solo/repeated and nonzero calls. Center and incomplete pairs use the player's normalized origin (0,0), reaching the original centered-ping consumer. Nonzero raw-coordinate preservation is an explicitly temporary missing-transform workaround; tests assert event/unit, not x/y. No-player lifecycle is not established by these fixtures.
- [ ] X2: Missing/malformed included XML fails before that document's children; parent siblings and TOC tail continue. Lua include and external Script failures do not abort later XML siblings.
- [ ] X1: Virtual frame template names are case-insensitive. Existing untainted declaration wins; tainted+tainted keeps first; untainted incoming replaces tainted. Non-replacement warns. Warmed inheritance/lifecycle lookups select the same winner under every spelling.
- [ ] G1 ready subset: Empty one-based action slot and slot zero return exactly one nil, not false. Occupied slots temporarily return one nil as non-attack because spell effect-type data is absent; this is not a real attack classifier and does not change C_ActionBar or other profiles.

## How it works

- [Minimap system](../minimap-system.md)
- [Addon loading](../addon-loading-pipeline.md)
- [XML templates](../xml-template-system.md)

## Implementation inventory

- `src/lua_api/frame/methods/map_frames.rs` — zoom and local-ping entry points.
- `src/lua_api/frame/methods/map_frames/wrath_ping.rs` — player-present local publication.
- `src/lua_api/workarounds/temporary/wrath_minimap_ping_coordinates.rs` — explicitly raw nonzero coordinates pending transform modeling.
- `src/loader/xml_file.rs` — document failure boundaries.
- `src/xml/template.rs` — template precedence and cache invalidation.
- `src/lua_api/globals/action_bar_api/` — legacy attack query registration.

## Tests asserting this spec

`tests/batch3_wrath_minimap.rs`, `tests/batch3_wrath_include.rs`, `tests/batch3_wrath_templates.rs`, `tests/batch3_wrath_attack.rs`.

## Known gaps (current cycle)

- [ ] M2 nonzero minimap view->world transform (dimensions/radius/rotation), normalization, grouped networking, no-player lifecycle and native receive/fade remain unmodeled/unverified. Temporary raw positions are not normalized-coordinate evidence; retirement condition: model the minimap view->world transform.
- [ ] M3 indoor/outdoor zoom, CVar synchronization and init/switch publication are held: the minimap-mode source, an authenticated indoor/outdoor mutation input and initialization timing are not specified. Do not couple this to IsIndoors/IsOutdoors or invent a first-tick init event; those existing probes remain unchanged.
- [ ] IsAttackAction attack classification needs spell effect-type data. data/spells.rs carries implicit targets, not effect types; gen_spells.rs extracts targets only. Once effect types are generated, classify first effect ==78. Synthetic occupied-policy controls are not authenticated native spell-ID classification.
- [ ] X2 exact diagnostic count/order and inline compile-error continuation remain unproved.

## Out of scope

Fonts have separate duplicate rules. Other registries, non-ASCII name folding, UseAction and contextual container actions have no contracts in this bounded batch.
