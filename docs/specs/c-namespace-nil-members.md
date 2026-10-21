# Missing C namespace members

The October 20, 2026 removal decision makes unmodeled `C_*` members nil, rather than manufacturing callable functions. Runtime namespace attachment lives in `src/lua_api/env_init/runtime_surface_bootstrap.lua`; explicit registrations remain authoritative.

## What it must do

- [ ] Unknown members remain nil under raw lookup, ordinary lookup, repeated lookup and `type`, without appearing in enumeration, across seven client profiles.
- [ ] Explicit seeded defaults, state and actions remain unchanged.
- [ ] Missing-member diagnostics remain available without publishing a value; strict and retired members remain nil.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/env_init/runtime_surface_bootstrap.lua` — shared nil diagnostic lookup.
- `src/lua_api/workarounds/temporary/housing_catalog_state.lua` — explicit housing defaults only.
- `src/lua_api/workarounds/temporary/{game_rules_namespace_fallback,guild_info_namespace_fallback,secure_transfer_state}.rs` — explicit surfaces and nil diagnostics, never arbitrary member fabrication.

## Tests asserting this spec

- `tests/c_namespace_nil_contract.rs` — lookup/enumeration and explicit registration contract, profile independent.
- `tests/nil_symbol_access.rs` — attributed, deduplicated nil observations.
- Embedded tests in the three Rust modules above — preserved state/actions and nil expectations.

## Known gaps (current cycle)

- [ ] Capture actual stock startup errors on all seven profiles and Retail startup tests; map original consumers to M1–M5/T6. F0 must report new failures, not mask them.

## Out of scope

- Implementing the missing backing models or introducing workarounds. Follow-up batches own those gaps.
