# Assisted action identity and slot lookup

Explicit optional assisted action identity is independent of the next-cast recommendation. Input state lives in `src/lua_api/state/support_types.rs`; query producers remain unchanged. Six tests are staged, uncompiled and unexecuted. Architecture: [Lua API](../lua-api.md).

## What it must do

- [ ] Supplied `action_spell_id: Option<u32>` defaults to `None`. No identity inferred from recommendations or populated bars.
- [ ] Zero-argument `C_AssistedCombat.GetActionSpell()` returns the current numeric identity or nil, live and environment-local, without changing `next_cast_spell_id` or `GetNextCastSpell()`.
- [ ] Zero-argument `C_ActionBar.FindAssistedCombatActionButtons()` matches semantic spellID assignments against action identity, never next-cast recommendation.
- [ ] **Simulator policy:** matching sparse positive slots form a dense ascending list; absent identity or no matches returns nil. Native ordering and exact miss arity are unverified.
- [ ] Results are fresh snapshots reflecting current identity and assignments. Caller mutation cannot alter backing inputs or another result; retained results survive input changes.

Contract grounding: cached `Blizzard_APIDocumentationGenerated/ActionBarFrameDocumentation.lua` declares a zero-argument, potentially absent table of slot indices. `AssistedCombatDocumentation.lua` declares separate action and recommendation queries. Cached `Blizzard_ActionBar/Mainline/AssistedCombatManager.lua` reads these identities independently. These sources do not establish a recommendation or identity-selection algorithm.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

| Path | Role |
|---|---|
| `src/lua_api/state/support_types.rs` | Independent optional action input; derived default is absent. |
| `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs` | Existing environment-owned assisted state and default initialization. |
| `src/lua_api/globals/missing_surface/small_namespaces.rs` | Existing nil action getter and modeled next-cast getter, unchanged. |
| `src/c_api/c_action_bar_spell_slots.rs` | Existing direct-spell queries; assisted Find not implemented. |
| `tests/assisted_action_input.rs` | Six prospective behavioral tests, shared `retail-12-0-0` capability gate. |

## Tests asserting this spec

`tests/assisted_action_input.rs` stages live optional identity, independent environments, sparse ascending matches, absent/unmatched identity, fresh/live snapshots and recommendation independence. Fixtures use explicit semantic spellIDs and clear macros/outfits; map values are not universally spellIDs. The assertion helper declares Find calls but does not execute them during setup. Generated `--test integration` filter: `assisted_action_input::` (expected six cases). No executed RED or GREEN yet.

## Known gaps (current cycle)

- [ ] Compile and execute staged tests against unchanged producers; compilation or fixture failure is not behavioral RED.
- [ ] Implement minimal action getter and assisted slot query only after relevant behavioral failures.
- [ ] Execute bounded GREEN, required startup/API-registration checks and applicable profile checks.

## Out of scope

Macros, outfits, talent-ID reinterpretation, shadowed assignments, slot-zero validation and special-bar membership are not modeled by these semantic-slot fixtures. Recommendation synthesis, automatic identity selection, new admin/import producers, events and persistence are excluded. No native ordering, security, miss arity, cached-caller parity or broad-profile acceptance is claimed.
