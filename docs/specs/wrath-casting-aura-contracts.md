# Wrath casting, channel and aura projections

Wrath legacy globals project existing cast, spell-catalog and aura state without changing Retail projections. Source lives in `src/lua_api/globals/utility_system_spell/spell_api.rs`, `spellcast_events.rs`, `cast_completion.rs` and `globals/auras.rs`. [System notes](../wiki/systems/lua-api.md#wrath-casting-and-aura-projections) describe the adapter boundary.

The 12340 static-source findings relayed on 2026-10-21 are authoritative for the native shapes below. Original cached FrameXML consumers provide simulator integration evidence, not live-client verification.

## What it must do

### Casting and channel queries

- [ ] Active `UnitCastingInfo` returns nine values: name, subtext, display text, icon filename, start/end milliseconds, trade-skill boolean, numeric cast count, and non-interruptible boolean.
- [ ] Active `UnitChannelInfo` returns eight values: the same first seven fields followed by the non-interruptible boolean, without a cast-count slot.
- [ ] Missing/expired casts, unavailable units and absent spell records return zero values. Non-string unit arguments raise the named usage error.
- [ ] Preserve supplied timing, empty icon strings and catalog subtext. Original `CastingBarFrame_OnEvent` must consume the legacy positions without boolean-time arithmetic errors.

### Produced cast events

- [ ] Existing Wrath START, DELAYED, FAILED and STOP producers publish unit token, spell name, subtext and numeric cast count. Completion retains that projection after clearing cast state.
- [ ] The channel event projection uses count zero; the missing-record event contains only the unit token.
- [ ] Quiet admin failure must not be reinterpreted as native interruption. The original bar ignores the quiet notification and consumes its ordinary STOP.

### Legacy aura reads

- [ ] Found `UnitAura`, `UnitBuff` and `UnitDebuff` records return eleven values, with string rank, string-or-nil icon, numeric count, seconds-based timing, optional caster, `1`/nil stealability/consolidation flags and numeric spell ID. Misses return zero values.
- [ ] Non-positive duration projects duration/expiration as zero. Party fallback records project count one, zero timing and absent caster/stealability.
- [ ] Accept numeric-string indices and one-based indexing. Default polarity is HELPFUL; UnitBuff/UnitDebuff force their corresponding polarity. Filter spelling is case-insensitive with spaces or pipes separating components.
- [ ] Preserve Retail outputs and all sixteen unchanged modern-only Wrath report rows; do not hide them behind new gates.

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

## Tests asserting this spec

`tests/wrath_casting_aura_contracts.rs` covers public tuples, expiration, event payloads, numeric-string inputs and original CastingBarFrame/TargetFrame consumers. `tests/common/wrath_callback_assertions.rs` retains native action, casting and buff callbacks selected by the original F05 rows.

Bound pre-producer evidence: merged baseline `6e54c3602` executes Wrath37 as 16 PASS/21 FAIL (all sixteen report-only rows fail), Retail37 as 37 PASS and both stock Lua-error arrays as empty. Fixture revision `254c42ad2` executes twelve exact cases as 1 PASS/11 FAIL, including all seven new contracts RED. Follow-up `fc67e8727` executes its four changed cases as two corrected controls PASS and two cast/channel contracts RED. Every counted case has a positive completed-test count. Producer GREEN and final preservation proof remain pending.

## Known gaps (current cycle)

- [ ] Record producer GREEN, original Wrath37 after counts, paired Retail37 preservation, empty stock arrays, formatter and Wrath profile-check evidence before checking requirements above.

## Out of scope

- Native channel transport, incoming-unit cast state and failure-reason input are absent. Supplied channel notifications exercise the original consumer; native reason 40/41 interruption is not simulated by the admin quiet boolean.
- Trade-skill/non-interruptible flags have no supplied input. Special cast display overrides and the condition selecting localized CHANNELING text remain gaps; supplied spell name is the INFERRED display policy without those inputs.
- The spell catalog supplies subtext but is not a separately authenticated 12340 rank catalog. Unknown aura-catalog rank uses an INFERRED empty string. Consolidation has no backing input; INFERRED opt-out uses the native absent flag, nil.
- PLAYER/RAID matching projects existing classification fields (INFERRED). CANCELABLE/NOT_CANCELABLE matching has no backing state and remains unmodeled; recognition of filter spelling is not proof of matching semantics. Non-decimal/index-edge coercions and conflicting filter components are not covered by this batch.
- Pet aura backing data, modern-only panels and unrelated widget/producer bugs are not repaired by this batch. Such failures require separate receipts rather than relaxed assertions or compatibility fallbacks.
