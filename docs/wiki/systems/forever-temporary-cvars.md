# Forever temporary CVars

The [CF-2 contract](../../specs/c-factory-cf2-tts.md#forever-temporary-cvars) separates effective session values from persisted CVar overrides. This is bounded to the original Forever 1.60.1.70532 input callbacks, not a native default/reset policy.

## State and registration

`CVarStorage` owns one temporary map, separate from its persisted `overrides`. `get` reads temporary, persisted, then default values. `set_temporary` and `remove_temporary` never call persistence; ordinary `set` saves only persisted overrides. Each environment creates its own temporary map. `read_current_cvar` delegates to this same storage, so Rust and Lua reads agree.

`src/c_api/forever_startup.rs` registers both operations once under its existing Forever `ACTIVE` gate. Its former separate overlay is removed. The pinned generated `CVarDocumentation.lua:171` declares `SetTempCVar(name, nullable cstring)`; `:114` declares `RemoveTempCVar(name)`. Both have no returns. Number inputs are required by the original consumer. Nil inputs raise an explicit unmodelled-case error; no release/reset/empty-value policy is invented.

## Original consumer selection and listener identity

Pinned `Blizzard_Gamepad/Blizzard_Gamepad.toc:3–5` selects the SharedXML dependency and the `camelot` game type; line 11 selects `Core.lua`. `Core.lua:4/6` sets/removes temporary values. Its helper callers at lines 15–41 supply only non-nil constants. `Blizzard_GamepadTargeting/TargetLogic.lua:102` supplies only 0 or 1; its TOC selects `camelot`, depends on Gamepad, and loads `TargetLogic.lua` at line 7. These are the only two non-documentation SetTempCVar call sites in the pinned cache.

`Blizzard_SharedXML/Blizzard_SharedXML.toc:35` selects `[Family]/InputUtil.lua`, resolving to Mainline and initializing the InputUtil table. Line 36 selects the shared `InputUtil.lua`, which creates a forbidden listener at lines 28–30 and installs its handler at line 75. There are two listeners because the simulator's `loader/addon.rs::maybe_replay_blizzard_lua_in_secure_env` executes the same library files again in the secure environment. This is the existing [SharedXML replay behavior](../investigations/ptr-input-interface-style.md#user-decision-two-environments-2026-10-25), not a second TOC entry for the generic source or a native-parity finding.

The restored fixture cannot use public EnumerateFrames: forbidden listeners are excluded. It reads the actual registry frame cache, authenticates original handler provenance, and selects exactly one listener whose function environment matches the original InputUtil registration function. It also authenticates that listener's actual callback table contains the original Core Gamepad Init closure. The other environment's listener must not own that registration. Driving that other original listener must leave the persisted/effective observations unchanged. No listener or callback is replaced.

## Proof state

Test-first source `076e82674` reaches the original listener's effective-value assertion: stored 2 versus expected temporary 1. Rilua's pinned `secureexecuterange` continues after callback errors without reporting them; this is an observable-state RED, not a captured Core.lua:4 traceback. The original TargetLogic selector independently fails at line 102 calling the missing SetTempCVar. Explicit nil-error RED fails on the absent endpoint. GREEN `70e4d148b` passes all three exact selectors, plus the migrated legacy Core-removal test. Each has exact single-test discovery and a protected current-byte CVar fixture. Temporary callbacks leave the saved file byte-identical; an ordinary write during the live overlay saves only underlying values; a second environment observes persisted 2 rather than temporary 1. The legacy fixture also proves later persisted changes to 3 and 4 are visible after removal, rejecting stale constant overrides. At check source `6f50f3002`, canonical all-target checks pass on Forever, Retail, PTR, Wrath, Mists, Era and Anniversary with zero Cargo JSON or plain compiler warnings. Shared CVarStorage changes require all seven; publication remains Forever-only. Formatting passes. Fresh protected stock remains byte-identical at 26/26→26/26, with no added/removed/changed keys. All 4488 pinned UI file hashes remain unchanged. Later changes are documentation only.

Initial historical-fixture failures were not producer RED: public enumeration hid forbidden frames, and then the one-global-listener assumption failed on normal/secure replay. The fixture now authenticates the exact Gamepad-owning listener instead of accepting either listener. Every original lifecycle value assertion is retained.

The historical nil shape test remains deferred. A laptop report examined 1.60.1.69977 and found only the same two non-nil callers, but made no native nil-value observation. The 70532 cache was independently searched and authenticated here; neither source inspection establishes native nil semantics.

## Sources

- [CF-2 contract](../../specs/c-factory-cf2-tts.md#forever-temporary-cvars)
- [Integration inventory and unchanged mixed-file deferral](../investigations/cf2-tts-integration.md)
- [Original-consumer fixture](../../../tests/cf2_gamepad_temp_cvars.rs)
- [CVar storage](../../../src/cvars.rs) and [Forever endpoints](../../../src/c_api/forever_startup.rs)
- [Normal/secure replay](../../../src/loader/addon.rs)

## See Also

- [[ptr-input-interface-style]] — existing two-environment replay boundary
- [[lua-api]] — environment and state ownership
