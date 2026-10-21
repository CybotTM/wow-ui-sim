# Missing C namespace members

The October 20, 2026 removal decision makes unmodeled `C_*` members nil, rather than manufacturing callable functions. Runtime namespace attachment lives in `src/lua_api/env_init/runtime_surface_bootstrap.lua`; explicit registrations remain authoritative.

## What it must do

- [x] Unknown members remain nil under raw lookup, ordinary lookup, repeated lookup and `type`, without appearing in enumeration, across seven client profiles.
- [x] Explicit seeded defaults, state and actions remain unchanged.
- [x] Missing-member diagnostics remain available without publishing a value; strict and retired members remain nil.

## How it works

- [Namespace lookup and diagnostics](../wiki/systems/addon-loading.md#missing-c-members-f0-2026-10-20)

## Implementation inventory

- `src/lua_api/env_init/runtime_surface_bootstrap.lua` — shared nil diagnostic lookup.
- `src/lua_api/workarounds/temporary/housing_catalog_state.lua` — explicit housing defaults only.
- `src/lua_api/workarounds/temporary/{game_rules_namespace_fallback,guild_info_namespace_fallback,secure_transfer_state}.rs` — explicit surfaces and nil diagnostics, never arbitrary member fabrication.

## Tests asserting this spec

- `tests/c_namespace_nil_contract.rs` — lookup/enumeration and explicit registration contract, profile independent.
- `tests/nil_symbol_access.rs` — attributed, deduplicated nil observations.
- `tests/namespace_member_absence.rs` and `tests/classic_startup_namespaces.rs` — strict, ordinary and constructed namespace absence.
- `tests/guild_info_probes.rs` and `tests/housing.rs` — unknown-member nil expectations alongside unchanged explicit-state checks.
- Embedded tests in the three Rust modules above — preserved state/actions and nil expectations.

## Known gaps (current cycle)

- [x] Capture stock startup errors on all seven profiles and Retail startup tests without relaxing assertions.
- [ ] Identify the erased callee behind Forever's anonymous timer-error family. Supplemental diagnostics locate its original callback at `Blizzard_SharedXMLBase/FunctionUtil.lua:87`; its `C_*` root remains unknown and is explicitly null under T6.

### Frozen F0 proof

Source anchor `0953c9b58`; `8c60cf5bb` changes documentation only. All seven builds use `sound,gui,casc,client-<profile>`; non-Retail disables default features. Each integration ELF binds this worktree's test source, and each stock executable's Cargo artifact binds its client profile. Surface, diagnostic, strict/constructor and embedded explicit-state subsets pass. Retail `startup_namespaces` passes 4/4; `startup_warnings` passes 24/26, with `test_no_warnings_on_startup` and `test_secure_env_annotated_files_load_cleanly` still failing.

| Profile | Stock unique errors | Occurrences |
|---|---:|---:|
| Retail | 108 | 185 |
| PTR | 108 | 188 |
| Wrath | 0 | 0 |
| Mists | 66 | 90 |
| Era | 52 | 70 |
| Anniversary | 52 | 70 |
| Forever | 208 | 224 |

The private F0 handoff's `gap-list.json` preserves every stock error, original consumer location and M1–M5/T6 routing, plus Retail assertion diagnostics. Alias/cascade attributions include original-source evidence and explicitly label inference. The two anonymous normal/wrapped timer rows retain a null member rather than an invented root. Committed baselines remain unchanged; comparisons use those empty baselines, not a fresh pre-removal run. Original persisted CVars retain their initial stat and hash.

**F0 alone is not mergeable.** Follow-up batches must restore clean stock startup and the two failing Retail tests without reinstating arbitrary member fabrication.

## Out of scope

- Implementing the missing backing models or introducing workarounds. Follow-up batches own those gaps.
