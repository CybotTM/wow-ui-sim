# Zone ability spell icon

`C_ZoneAbility.GetZoneAbilityIcon(zoneAbilitySpellID)` must query the existing spell catalog. Cached Retail `ZoneAbilityDocumentation.lua` declares a required number argument, nilable numeric result and `SecretArguments = "AllowedWhenUntainted"`. Current legacy producer returns a placeholder string; replacement belongs in [C API architecture](../lua-api.md).

## What it must do

- [ ] Known Fireball spell 133 returns exactly one ordinary numeric value matching catalog `icon_file_data_id`, not a texture path.
- [ ] A valid u32 absent from the catalog returns exactly one nil. Fixture 4,000,000,000 has absence asserted. This is explicit simulator policy consistent with nilable output, not native-observed miss behavior.
- [ ] Preserve existing ordinary input rejection: missing/nil, non-numbers including numeric strings, non-integral/non-finite numbers and numbers outside u32 range fail explicitly. This preserves current extraction, not native parity.
- [ ] Authenticate original secret arguments before numeric extraction/lookup: native host-secret known and absent IDs work untainted and fail tainted; plain known numeric IDs remain permitted tainted. Preserve wrapper identity/secrecy and caller taint across success, rejection and subsequent untainted reuse. Input secrecy does not imply secret output propagation.

## How it works

- [Existing spell API](../lua-api.md).
- [Cache provenance](../updating-blizzard-ui-to-a-new-patch.md).

## Implementation inventory

- `src/lua_api/globals/missing_surface/zone_ability.rs`: existing query, still placeholder.
- `src/lua_bridge/from_stack.rs`: existing exact integer/u32 extraction.
- `data/spells.rs`: existing generated catalog; not edited.

## Tests asserting this spec

`tests/zone_ability_icon.rs`: four staged cases for positive lookup, modeled miss, ordinary rejection and actual host-secret caller policy. Secret case uses existing `retail-12-0-5` native fixture gate. All remain uncompiled/unexecuted, not behavioral RED.

## Known gaps (current cycle)

- [ ] Compile/execute staged tests and distinguish behavioral RED from fixture barriers.
- [ ] Replace the string placeholder with one C_API-owned producer and obtain independent behavior/startup/check proof.

## Out of scope

Native miss/malformed-input behavior and native parity remain unknown. Active-ability membership, zone lifecycle, external addon caller acceptance and catalog/generated-data edits are not covered. Secret tests assert observable authentication failures, not internal lookup calls.
