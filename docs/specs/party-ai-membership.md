# Supplied party AI membership

`UnitInPartyIsAI(unit)` must expose explicit GUID classification intersected with the live active-party representation. Only empty `SimState.party_ai_guids` is integrated; the existing producer stays unchanged pending genuine RED. Tests target `client-retail`/`client-ptr`; backing state uses their shared `retail-12-0-0` feature floor. Integration is uncompiled/unexecuted; this is current modeled behavior, not a historical 10.2.5 emulator.

## What it must do

### Supplied membership

- [ ] Initialize an independent empty GUID set per environment; supplying a GUID creates no unit, roster member, target, dungeon or follower.
- [ ] Resolve existing modeled identity and return true only if its GUID is both supplied and currently part of the active party representation. Unknown/absent/nonparty identities are false even when supplied; ordinary unsupplied members are false.
- [ ] Same-GUID aliases agree; matching display names do not create equality. Retargeting does not transfer classification; retained focus remains classified only while its GUID belongs to the live roster.
- [ ] Query live activity/roster each time; inactivation/removal clears effective membership immediately even with retained snapshots/metadata. Explicit set removal is reversible; environments remain independent.
- [ ] Follower-dungeon state alone neither supplies nor spawns AI; supplied active-party AI does not require an instance. Classification does not remodel human/player identity.

### Accepted inferred policies

- [ ] Include an explicitly supplied existing player GUID while the party is active, following current UnitInParty's player inclusion. Unsupplied/solo/inactive player is false. This is an inferred simulator policy, not native AI-player proof.
- [ ] Use the existing active-party representation, without a new roster-size exclusion or party/raid taxonomy. Tests use two roster entries; raid-token/conversion behavior is not claimed.
- [ ] Preserve nil/missing -> one false, strict string-or-nil inputs, ignored extra arguments and one plain nonnil boolean. No number coercion/token normalization or new result-secret policy.
- [ ] Authenticate genuine VM secret inputs for untainted callers before state lookup, then enforce string/nil validation; reject tainted secret callers while preserving wrappers and caller taint. Ordinary tainted queries remain usable; authentic wrong-type secret numbers error.
- [ ] Actual cached CompactUnitFrame name consumer, with a real FontString and real query, adds follower prefix only for supplied live AI in follower-dungeon context; ordinary members/context and removed retained aliases lose the prefix.

## How it works

- [Unit GUID presence](unit-guid-presence.md).
- [Unit identity and retained snapshots](unit-identity-equality.md).
- [Lua API overview](../lua-api.md).

## Implementation inventory

- `src/lua_api/state/sim_state.rs`: public `party_ai_guids: HashSet<String>` fixture field.
- `src/lua_api/state.rs`: independent empty initialization.
- `src/lua_api/globals/group_queries_relationships.rs`: existing non-C global producer; unchanged in this stage.

## Tests asserting this spec

[tests/party_ai_membership.rs](../../tests/party_ai_membership.rs): nine unexecuted cases for supplied/ordinary/player classification, aliases and equal-name controls, absent/nonparty identities, live inactivation/removal, reversible independent state, dungeon independence, nullable/plain type policy, authentic host-secret security, and cached real name consumer.

Consumer fixture loads unchanged contiguous CompactUnitFrame_UpdateName/IsTapDenied definitions from the active profile cache. Only visibility fixture input and localization sentinel are supplied; AI getter, walk-in/spectating predicates and actual FontString remain real. Walk-in TRUE is unmodeled and excluded. No vendor/cache changes or query overrides.

## Known gaps (current cycle)

- [ ] Obtain compiled behavioral RED; field/import/cache/fixture failures are not behavioral RED. No tests/builds ran during integration.
- [ ] Implement membership/authentication after genuine RED, then obtain bounded GREEN.
- [ ] Authentication discrepancy: cached current Retail UnitDocumentation.lua:1579–1592 declares AllowedWhenUntainted, nullable UnitToken, nonnil bool; the draft observed the current false producer extracting Option<String> without decoding genuine wrappers. Preserving documented authentication requires repair, not preservation of that stub. This is not historical 10.2.5 secret-policy evidence.
- [ ] Native/PTR parity and slot lifetimes remain unverified. PartyMember has no GUID; current identity is slot-derived, so regrowth can reuse a GUID. No epoch/generation or slot-reuse contract is introduced.

Source limits: draft inspected cached Retail API/CompactUnitFrame sources and repository identity/group helpers, not native execution. [10.2.5 publication source](../../data/patch-api/sources/10.2.5-api-changes.wikitext) row 108 establishes publication only. Current UnitInParty is token-spelling based and cannot directly satisfy GUID-alias requirements; pet/vehicle lack represented GUIDs. Supplied AI is distinct from npc_follower_guids.

## Out of scope

New Admin commands, AI entity generation, party/human remodeling, raid identity expansion, metadata pruning, lifetime redesign, walk-in behavior, UI event wiring, nameplate lifecycle, other profiles, native probes, vendor/config/WTF changes. No producer or registration implementation in this stage.
