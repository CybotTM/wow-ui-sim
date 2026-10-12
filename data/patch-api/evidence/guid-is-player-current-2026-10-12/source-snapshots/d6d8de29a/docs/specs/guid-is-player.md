# GUID player identity query

Current Retail/PTR `C_PlayerInfo.GUIDIsPlayer` must project existing represented identity and player-kind facts, not GUID syntax or a native catalog. Cached `PlayerLocationDocumentation.lua` declares a required `WOWGUID`, one boolean result and `AllowedWhenUntainted`. Strict string/no-coercion and closed-world unknown handling below are simulator policy, not native validation evidence. See [API architecture](../lua-api.md).

## What it must do

- [ ] Authenticate the original secret argument before type validation or lookup; require an actual UTF-8 string and return exactly one boolean. Missing/nil/nonstring values error without coercion.
- [ ] Return true for exact local identity and currently active populated party identities; target/focus use their explicit player-kind facts. Text prefixes never classify input.
- [ ] Reflect live identity replacement, kind changes, removal, roster activation/population and independent environments without stale results.
- [ ] Return false for unrepresented strings, including player-looking GUIDs and empty strings: “not represented as a player in current model state,” not native non-player classification.
- [ ] Preserve authentic secret wrappers/caller taint: wrapped strings allowed when untainted, wrapped hits/misses/malformed values denied when tainted; ordinary strings remain usable.
- [ ] Load the ordinary ObjectAPI addon closure and exercise original whole `PlayerLocation:CreateFromGUID`/`PlayerLocationMixin:IsValid`, with vendor source provenance. Represented players yield true; misses retain the unchanged account operand's observed result, not a native account-type claim. No account mocks or sliced consumers.

## How it works

- [Lua/API architecture](../lua-api.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

- `src/c_api/c_player_info_guid.rs` — modeled query over current identity/kind facts; current profile registration in `src/c_api/mod.rs`/`registration.rs`.
- `src/lua_api/globals/unit_misc.rs` — existing represented identity projection.
- `src/lua_api/globals/unit_probes.rs` — existing target/focus player-kind facts.
- `src/lua_api/state_types/social.rs` — existing local identity constant.
- `tests/guid_is_player.rs` — direct state/security and whole vendor consumer regressions.

## Tests asserting this spec

`tests/guid_is_player.rs`: local/misses, target/focus replacement, kind/removal, party lifecycle, environment isolation, strict type, authentic secret boundary and two original ObjectAPI consumer cases. Initial Retail RED at `76f405e1d`:0/9 PASS, exit101, eight missing raw-provider guards and one original whole-consumer exact-true mismatch. Ordinary closure/provenance reached; downstream query/security/account diagnostics unreached. Compile/fmt0 and source unchanged independently verified; requirements remain unchecked pending GREEN.

## Known gaps (current cycle)

- [x] Genuine RED observed before producer; assertions unreached after missing-provider guards receive no behavioral credit.
- [ ] Implement and verify Retail/PTR direct/consumer/startup checks, preserve diagnostics and account predicate provenance.

## Out of scope

Native arbitrary-GUID classification/coercion/catalogs, prefix inference, new normal-session identities/seeding, account-type implementation, earlier-profile contracts, vendor/global overrides, native or full-handoff acceptance.
