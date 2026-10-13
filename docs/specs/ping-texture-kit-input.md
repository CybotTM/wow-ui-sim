# Host-supplied ping texture kits

`C_Ping.GetTextureKitForType(type)` must project a whole host-supplied type-to-string mapping, not an invented native catalog. Input/export/default and six tests are staged; getter/registrar unchanged, tests uncompiled/unexecuted. Architecture: [Lua API](../lua-api.md).

## What it must do

- [ ] Return exactly one string equal to the host entry, with no normalization, inference or substitute kit.
- [ ] Each environment starts with empty input; whole-map replacement changes future queries without affecting another environment.
- [ ] **Simulator miss policy, not native unknown-type behavior:** unmapped types error explicitly, identifying `unmapped ping type <type>` and `host-supplied mapping required`; no nil/empty/guessed-kit fallback or cached stale entry.
- [ ] **Simulator key policy:** accept finite integral numbers representable as the supplied map's `i32` keys; reject missing/nil, strings, booleans, tables, fractional/non-finite and out-of-range values before lookup. No native coercion, enum membership or native-range claim.
- [ ] Authenticate secret inputs before ordinary validation/lookup using `AllowedWhenUntainted`: an untainted secret numeric key may select a mapped entry; tainted secrets (including unmapped and malformed payloads) are denied without leaking/declassifying input. Public keys remain usable by tainted callers. Returned mapped strings are public by simulator policy; caller taint is preserved.
- [ ] Actual cached `PingManager:ShowPingSpot(type,posX,posY)` consumes the real query and passes exact `Ping_SpotGlw_<kit>_In`/`_Out` atlas names with use-size true to both sinks.

Cached `PingManagerDocumentation.lua` declares required PingSubjectType, non-nil textureKit and AllowedWhenUntainted secret arguments. It does not provide mappings or miss behavior. Arbitrary synthetic keys/kits in tests are host fixtures, not native pairs. Secret-input authentication has prospective tests but still needs executed behavioral evidence before producer completion; no native security parity claimed by this stage.

## How it works

- [Lua API architecture](../lua-api.md)
- [Addon loading](../addon-loading-pipeline.md)

## Implementation inventory

| Path | Role |
|---|---|
| `src/c_api/c_ping_texture_kits.rs`, `src/c_api/mod.rs` | Input map/export only; no query or registrar. |
| `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs` | Environment input/default under retail-12-0-0. |
| `tests/ping_texture_kits.rs` | Six prospective tests selected on Retail/PTR, generated integration module. |

## Tests asserting this spec

Generated `--test integration` filter `ping_texture_kits::` (expected six): configured exact strings/live replacement/isolation; empty/absent/cleared input error policy; invalid public key matrix; untainted actual VM secret numeric input; tainted actual VM secret numeric/unmapped/fractional/string denial plus public-input recovery; actual cached ShowPingSpot atlas arguments through controlled pool/frame/animation sinks. Secret fixtures assert query publication before setup. Consumer asserts real query before cache discovery or vendor loading. Sink spies prove call arguments only, not rendering/assets/lifecycle. All six UNCOMPILED/UNRUN; cache/setup failures are not query RED.

## Known gaps (current cycle)

- [ ] Compile and obtain executed behavioral RED before implementing getter.
- [x] Stage source-backed ordinary/secret argument policy tests; execution remains pending.
- [ ] Implement minimal getter and prove GREEN plus applicable startup/profile checks.
- [ ] Verify cached consumer prerequisite sufficiency in execution.

## Out of scope

Native/default/enum-complete mappings, GetDefaultPingOptions DTOs, auto-discovery, atlas existence/rendering, send/wheel/animation lifecycle, native coercion/ranges/secrecy and full-profile/native acceptance. Existing cooldown/default-options providers remain unchanged.
