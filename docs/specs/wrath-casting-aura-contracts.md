# Wrath casting, channel and aura projections

Wrath legacy globals project existing cast, spell-catalog and aura state without changing Retail projections. Source lives in `src/lua_api/globals/utility_system_spell/spell_api.rs`, `spellcast_events.rs`, `cast_completion.rs` and `globals/auras.rs`. [System notes](../wiki/systems/lua-api.md#wrath-casting-and-aura-projections) describe the adapter boundary.

The 12340 static-source findings relayed on 2026-10-21 are authoritative for the native shapes below. Original cached FrameXML consumers provide simulator integration evidence, not live-client verification.

## What it must do

### Casting and channel queries

- [x] Active `UnitCastingInfo` returns nine values: name, subtext, display text, icon filename, start/end milliseconds, trade-skill boolean, numeric cast count, and non-interruptible boolean.
- [x] Active `UnitChannelInfo` returns eight values: the same first seven fields followed by the non-interruptible boolean, without a cast-count slot.
- [x] Missing/expired casts, unavailable units and absent spell records return zero values. Non-string unit arguments raise the named usage error.
- [x] Preserve supplied timing, empty icon strings and catalog subtext. Original `CastingBarFrame_OnEvent` must consume the legacy positions without boolean-time arithmetic errors.

### Produced cast events

- [x] Existing Wrath START, DELAYED, FAILED and STOP producers publish unit token, spell name, subtext and numeric cast count. Completion retains that projection after clearing cast state.
- [ ] The channel event projection uses count zero; the missing-record event contains only the unit token.
- [x] Quiet admin failure must not be reinterpreted as native interruption. The original bar ignores the quiet notification and consumes its ordinary STOP.

### Legacy aura reads

- [ ] Found `UnitAura`, `UnitBuff` and `UnitDebuff` records return eleven values, with string rank, string-or-nil icon, numeric count, seconds-based timing, optional caster, `1`/nil stealability/consolidation flags and numeric spell ID. Misses return zero values.
- [x] Non-positive duration projects duration/expiration as zero. Party fallback records project count one, zero timing and absent caster/stealability.
- [x] Accept numeric-string indices and one-based indexing. Default polarity is HELPFUL; UnitBuff/UnitDebuff force their corresponding polarity. Filter spelling is case-insensitive with spaces or pipes separating components.
- [x] Preserve Retail outputs and all sixteen unchanged modern-only Wrath report rows; do not hide them behind new gates.
- [ ] The original BuffFrame consumer receives a filename string from its Texture:GetTexture, not a file ID. Wrath returns one stored filename string or one nil; Retail remains file-ID-first. Do not normalize case or separators to force identity.

## How it works

- [Projection notes](../wiki/systems/lua-api.md#wrath-casting-and-aura-projections)
- [Native action fixture](../../tests/fixtures/wrath-action-callbacks.md)
- [Spell and aura model](../lua-api.md)

## Implementation inventory

- `src/lua_api/globals/utility_system_spell/spell_api.rs`: profile-specific positional cast/channel snapshots and argument validation.
- `src/lua_api/spellcast_events.rs`: event payload projection over existing spell metadata and cast identifiers.
- `src/lua_api/cast_completion.rs`: completion notifications after removing the active cast.
- `src/lua_api/globals/auras.rs`: legacy argument/filter selection and local tuple-field projection; C_UnitAuras tables and Retail paths remain unchanged.
- `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`: isolated absent-consolidation policy; the old nil rank workaround is removed in favor of catalog metadata.
- `src/lua_api/frame/methods/widgets/texture/atlas.rs`: runtime Wrath getter projection returns stored filename rather than the independently resolved numeric ID.

## Tests asserting this spec

`tests/wrath_casting_aura_contracts.rs` covers public tuples, expiration, event payloads, numeric-string inputs and original CastingBarFrame/TargetFrame consumers. `tests/common/wrath_callback_assertions.rs` retains native action, casting and buff callbacks selected by the original F05 rows.

Bound pre-producer evidence: merged baseline `6e54c3602` executes Wrath37 as 16 PASS/21 FAIL (all sixteen report-only rows fail), Retail37 as 37 PASS and both stock Lua-error arrays as empty. Fixture revision `254c42ad2` executes twelve exact cases as 1 PASS/11 FAIL, including all seven new contracts RED. Follow-up `fc67e8727` executes its four changed cases as two corrected controls PASS and two cast/channel contracts RED. Every counted case has a positive completed-test count. At `20f35943a`, all seven new contracts pass, Retail37 stays 37 PASS, both stock arrays remain empty and Wrath check/formatter pass. Wrath37 is 20 PASS/17 FAIL: sixteen report-only rows plus an introduced BuffFrame icon regression (UnitBuff filename versus GetTexture number 135893). Fixture `19c9a906b` reproduces that exact original-consumer failure at its explicit filename-type assertion before the getter correction; one exact completed test FAIL.

Getter authority is the laptop's independently checked, reconstructed 12340 source report received 2026-10-23, not native execution. A named texture returns exactly one filename string without extension, otherwise one nil; never a number. Cache spelling can come from an earlier case-insensitive load and aliases can substitute paths. Slash/backslash equality is not promised. The bounded correction returns the stored path unchanged; fresh extensionless BuffFrame identity is tested, not unconditional latest-setter byte echo.

Final producer revision `14aed640c0629a41138e0c809a7ad56b87738170`: Wrath original37 = 21 PASS/16 unchanged report-only FAIL; native21 and all seven added contracts PASS. The original BuffFrame filename-type/identity assertions PASS. Retail original37 before/after = 37 PASS/0 FAIL with identical named-case statuses. One final Retail edit-mode attempt failed in mandatory prefork `conformance::bounded_workers` setup (2000ms worker timeouts), before its named case; one bounded exact retry on the same sealed revision passed. Both setup failure and retry receipts are retained; no harness or assertion was changed.

Both profiles' stock Lua-error arrays remain empty before/after. Final Wrath `cargo check` and `cargo fmt --all -- --check` pass; check has zero warnings. All counted cases have positive completed-test counts, all Retail successes include the exact named success, and child-only current CVar copies leave the persisted original stat/hash unchanged. Unchecked requirements below retain unexercised branches, not whole-batch parity claims.

## Known gaps (current cycle)

- [ ] Produced channel-event zero-count and missing-record event branches lack direct producer coverage; incoming channel transport remains unmodeled.
- [ ] Full eleven-value UnitDebuff projection and absent-texture one-nil getter arity are not independently pinned by this bounded suite; existing shared serializer code is not substituted for coverage.

## Out of scope

- Native channel transport, incoming-unit cast state and failure-reason input are absent. Supplied channel notifications exercise the original consumer; native reason 40/41 interruption is not simulated by the admin quiet boolean.
- Trade-skill/non-interruptible flags have no supplied input. Special cast display overrides and the condition selecting localized CHANNELING text remain gaps; supplied spell name is the INFERRED display policy without those inputs.
- The spell catalog supplies subtext but is not a separately authenticated 12340 rank catalog. Unknown aura-catalog rank uses an INFERRED empty string. Consolidation has no backing input; INFERRED opt-out uses the native absent flag, nil.
- PLAYER/RAID matching projects existing classification fields (INFERRED). CANCELABLE/NOT_CANCELABLE matching has no backing state and remains unmodeled; recognition of filter spelling is not proof of matching semantics. Non-decimal/index-edge coercions and conflicting filter components are not covered by this batch.
- Texture setter extension stripping, cache/alias fidelity and preservation of earlier cache spelling are not changed: the reproduced consumer supplies an extensionless filename. A mismatch beyond the getter return type requires separate evidence, not forced normalization.
- Pet aura backing data, modern-only panels and unrelated widget/producer bugs are not repaired by this batch. Such failures require separate receipts rather than relaxed assertions or compatibility fallbacks.
