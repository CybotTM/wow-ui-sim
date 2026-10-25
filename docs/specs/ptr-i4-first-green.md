# PTR I4 first bounded runtime repair

## Scope

PTR 12.1.7.70493 publishes only two formerly missing endpoints in this packet. Other profiles preserve their surfaces, including Forever's existing graveyard query. Profile selection uses `client_profile::ACTIVE`.

## Supplied-state contract

- `C_Spell.GetItemCooldown(itemID)` accepts a numeric item ID and returns one nullable `SpellCooldownInfo`. A missing `SimState.item_cooldowns` entry returns nil; a supplied record projects its exact start and duration. No equipment-slot, spell-ID or bag-slot alias is used.
- The record uses the existing simulator cooldown policy: enabled and unmodified playback (`isEnabled=true`, `modRate=1`), with `isActive` derived from nonzero start and duration. Optional category, recovery and GCD fields remain absent because the item model does not supply them. Each query returns a separate table.
- `CanPortGraveyard()` returns exactly one boolean from `player.can_port_graveyard`. State changes and independent environments must be visible without leaking. No death, map, capability or graveyard policy is inferred from other fields.

## Evidence and native-unknown limits

The authenticated PTR APIDocumentation declares `GetItemCooldown` at `SpellDocumentation.lua:139–153`, its record at `SpellSharedDocumentation.lua:19–34`, and the global at `PlayerScriptDocumentation.lua:87–93`. Original selected consumers are `Blizzard_CollectionTemplates.lua:37` and `GhostFrame.lua:4`.

The item documentation says "Returns nil if the item is not found or on cooldown". That sentence does not establish how supplied simulator cooldown records map to native eligibility. This packet's record projection and missing-record policy are an explicit shape-only simulator contract, not native cooldown correctness. Enabled/rate fields retain existing simulator policy, not measured native constants. Graveyard availability is an explicit supplied input; its seeded false value is not a native capability default. Secret argument handling and native restrictions remain unproved.

## Verification

`tests/ptr_i4_green1.rs` checks concrete supplied records, changed/removed records, return shapes and environment isolation. A producer-only RED child must remove only the two new registrations while keeping these and the I4 test bytes unchanged.

The existing I4 discovery and preservation selectors remain unchanged. They still require the other 167 missing shapes and three blocked consumers; this bounded repair cannot make those aggregate selectors pass. Persist all observations and report the two original-consumer transitions separately, without an aggregate GREEN claim.

Stock PTR startup compares master against this repair with identical pinned sources and guarded CVar copies. Every added error key requires an original emitter line and the predecessor failure that masked it. PTR/Retail all-target checks and formatting are separate acceptance evidence.

## Related documentation

- [I4 runtime contract](patch-12-1-7-inherited-runtime.md)
- [I4 source investigation](../wiki/investigations/patch-12-1-7-api-audit.md)
- [Existing item cooldown model](item-cooldowns.md)
- [Existing graveyard query](forever-graveyard-port.md)
