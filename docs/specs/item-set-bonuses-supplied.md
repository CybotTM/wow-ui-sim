# Supplied item-set bonuses

`C_Item.GetSetBonusesForSpecializationByItemID(specID, itemID)` must project an explicit environment-local mapping to ordered spell IDs. C_API ownership lives in `src/c_api/item_set_bonuses.rs`; the producer is implemented after genuine behavioral RED. Central registration wiring belongs to main. Tests target `client-retail`/`client-ptr`; backing state uses their shared `retail-12-0-0` feature floor. GREEN remains pending; implementation is not native acceptance.

Accepted draft evidence: cached Retail `~/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/Blizzard_APIDocumentationGenerated/ItemDocumentation.lua:1087–1101` declares required numeric `specID`, `itemID`, `MayReturnNothing = true`, `SecretArguments = "AllowedWhenUntainted"`, and nonnil table `itemSetSpellIDs` with numeric inner type. Supplied mapping, ordering/duplicates, fresh allocation and signed32 matching below are accepted simulator policies, not native execution or historical-build evidence. PTR gating does not establish parity.

## What it must do

### Supplied mapping

- [ ] Default empty; use the complete `(specID, itemID)` pair, without equipment/specialization/threshold/ownership/catalog derivation.
- [ ] Missing pairs evaluate to nil without specifying zero-versus-one-nil return arity; supplied empty lists return an empty table.
- [ ] Preserve exact IDs, order and duplicates; no sorting, substitution, deduplication or catalog validation.
- [ ] Return a fresh table even for empty rows; mutation affects neither simultaneous/later results nor supplier state.
- [ ] Keep mappings environment-local and unchanged by successful/rejected queries.

### Inputs and security

- [ ] Authenticate both original selectors before extracting/validating either or reading state; invalid plain first selectors and misses must not bypass secret-second authentication.
- [ ] Missing/nil and nonnumeric types error explicitly; numeric strings and item links are not numbers.
- [ ] Finite integral signed32 selectors match supplied keys; fractional/nonfinite/out-of-range numbers miss without truncation/saturation. This follows the existing supplied-relation convention, not a native range declaration. No positivity/catalog constraint.
- [ ] Untainted callers may decode genuine VM secrets before numeric validation. Tainted callers reject secrets before type/read work, including malformed or missed payloads, while ordinary numbers remain usable. Preserve wrappers, unwrappability restrictions and caller taint.

## How it works

- [Lua API overview](../lua-api.md).
- [Existing supplied-relation contract](mount-item-relations.md).

## Implementation inventory

- `src/c_api/item_set_bonuses.rs`: `ItemSetBonuses = HashMap<(i32, i32), Vec<i32>>`; producer and `pub(crate) register(state)` ensure `C_Item` and publish only `GetSetBonusesForSpecializationByItemID`. Both original arguments undergo VM secret authentication before strict numeric parsing or lookup. Exact signed32 selectors query cloned supplied rows; missing pairs return nil. Supplied rows become fresh stack-rooted sequence tables, preserving order/duplicates and releasing the state borrow before VM allocation. Pending GREEN.
- `src/c_api/mod.rs`: public alias re-export, no registration change.
- `src/lua_api/state/sim_state.rs`: public `item_set_bonuses` field.
- `src/lua_api/state.rs`: independent empty initialization.
- `src/c_api/item_spell/c_item.rs`: existing namespace boundary, unchanged in this stage.
- Central producer wiring is main-owned; no shared registration, state, other modules or tests changed by this implementation.

## Tests asserting this spec

[tests/item_set_bonuses_supplied.rs](../../tests/item_set_bonuses_supplied.rs): nine behavioral tests covering absent pairs; exact order/duplicates; supplied empty rows; mutation/fresh tables; independent environments; signed32 endpoints and invalid numeric misses; required-number validation; secure secrets; tainted permission-before-validation/miss and recovery. Main supplied genuine 76 RED evidence: one absence test passed; eight supplied/public-type/security tests failed at actual behavioral assertions. No tests were run in this implementation stage; bounded GREEN remains pending with main. All supplied spec/item/spell relationships are artificial fixtures, not native relationships.

## Known gaps (current cycle)

- [x] Obtain compiled behavioral RED; main supplied 76 RED evidence, not unresolved symbols/compile errors.
- [x] Implement producer after RED, within the C_API module only.
- [ ] Main wires central registration and obtains bounded GREEN; requirement checkboxes above remain pending that proof.
- [ ] Native numeric-domain, list semantics, return arity and Retail/PTR parity remain unverified.

## Out of scope

Equipment/active-bonus thresholds/catalog inference, normal-session population, Lua supplier/Admin commands, fallback, native probes, other profiles, vendor/config/WTF changes. Shared wiring/state/modules/tests, commits, builds/checks/runtime/final gates, operations and delegation are excluded from this implementation stage.
