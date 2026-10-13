# Supplied exact-link item stats

`C_Item.GetItemStats(itemLink)` and `C_Item.GetItemStatDelta(itemLink1, itemLink2)` must expose environment-local supplied numeric maps. Empty fixture state lives in `src/c_api/item_stats.rs`; producers are deferred pending genuine behavioral RED. Tests target `client-retail`/`client-ptr`; backing state uses their shared `retail-12-0-0` feature floor. Integration is uncompiled and unexecuted.

Cached Retail evidence recorded by the accepted draft: `~/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/Blizzard_APIDocumentationGenerated/ItemDocumentation.lua:965–995` declares required `cstring` arguments, `MayReturnNothing = true`, `SecretArguments = "AllowedWhenUntainted"`, and `LuaValueVariant` named `statTable`. Numeric maps, fresh allocation, arithmetic, and missing-link rules below are accepted simulator assumptions, not native facts established by that declaration. No native/PTR parity proof.

## What it must do

### Supplied state and queries

- [ ] Initialize each environment with empty supplied maps; direct Rust population suffices, without new host Lua commands.
- [ ] Match complete strings exactly; distinct full links for one item ID remain distinct. Unknown strings, bare IDs/links, changed names or whitespace yield nil unless supplied independently.
- [ ] Return exact string-keyed numeric values, including explicit zeros, in fresh tables; caller mutation cannot affect other results or later queries.
- [ ] Compute first minus second over the union of supplied keys, treating missing keys as zero. Preserve positive/negative/fractional differences; omit exactly zero differences. Identical supplied links yield an empty table.
- [ ] Return nil if either delta operand is absent, including two identical absent strings. Missing-result tests do not constrain zero-versus-one-nil return arity.

### Original-argument security and validation

- [ ] Authenticate all original required arguments before parsing/type validation; an invalid plain first operand cannot mask a secret second operand's denial.
- [ ] Untainted callers may unwrap genuine VM secrets, then apply ordinary required-string validation, lookup and arithmetic.
- [ ] Tainted callers must reject secrets before lookup/type validation, including malformed/missing payloads; retain wrappers and caller taint, without payload disclosure. Permission errors identify `untainted caller`.
- [ ] Missing/nil and non-string plain values, including numbers, must error explicitly with the required string contract. Tainted callers may query ordinary strings; no blanket addon denial.

## How it works

- [Lua API overview](../lua-api.md).
- [Runtime architecture](../wiki/systems/lua-api.md).

## Implementation inventory

- `src/c_api/item_stats.rs`: public `ItemStatsState { by_link: HashMap<String, HashMap<String, f64>> }`, empty default; no producer.
- `src/c_api/mod.rs`: C_API model re-export, no registration change.
- `src/lua_api/state/sim_state.rs`: public `item_stats` fixture field.
- `src/lua_api/state.rs`: independent empty initialization.
- `src/c_api/item_spell/c_item.rs`: existing C_Item registration boundary; unchanged in this stage.

## Tests asserting this spec

[tests/item_stats_supplied.rs](../../tests/item_stats_supplied.rs): nine unexecuted behavioral tests. Covers exact supplied maps; bidirectional union delta; misses in either position; zero omission/empty delta; fresh results; environment isolation; required-string errors; secure genuine-secret validation; tainted permission-before-validation and public-call recovery. Fixtures use same item ID with distinct full links and concrete fractional/zero stats.

## Known gaps (current cycle)

- [ ] Obtain compiled behavioral RED; absent fields/imports or excluded cases are not RED. This integration ran no builds/tests.
- [ ] Implement C_API producer/registration only after genuine RED, then obtain bounded GREEN.
- [ ] Native catalog/arithmetic/error wording/return arity/output security and PTR equivalence remain unverified.

## Out of scope

Catalog/cache seeding, equipment/item-level/scaling inference, canonicalization/item-ID parsing, supplied output-table arguments, Admin commands, fallbacks, native probes and other profiles. Non-finite stat values, embedded-NUL handling and extra-argument policy are unspecified. No vendor/config/WTF edits.
