# M5 housing, pets, bank and transmog startup inputs

M5 restores individually required startup inputs after [factory removal](c-namespace-nil-members.md). [Implementation notes](../wiki/systems/c-factory-startup-followup.md#m5-service-and-catalog-inputs) separate catalog-backed queries from named temporary service gaps.

## What it must do

- [ ] Retail/PTR owned-house requests are available before the housing dashboard loads and asynchronously emit `PLAYER_HOUSE_LIST_UPDATED` with an empty list while the authoritative ownership service is absent.
- [ ] Retail/PTR/Mists/Forever original pet action buttons remain disabled when swap/skip legality is unavailable. Named temporary predicates return false, not eligibility inferred from health alone.
- [ ] Those profiles' pet-icon query joins the battle pet's supplied species ID to existing pet-journal icon metadata; missing owner/slot/species metadata returns no icon.
- [ ] Forever purchased bank-tab metadata returns an empty table and player-bag bank capability returns false until their named models exist.
- [ ] Forever `C_TransmogSets.HasAnyValidSets()` reads supplied catalog `valid_for_character` flags independently of UI filters. Other profile API publication is unchanged.

## How it works

- [Service/capability defaults and catalog projections](../wiki/systems/c-factory-startup-followup.md#m5-service-and-catalog-inputs)

## Implementation inventory

- `src/c_api/c_pet_battles.rs` — species/catalog icon projection.
- `src/c_api/c_transmog_sets{.rs,/catalog.rs}` — Forever valid-set query and existing catalog.
- `src/c_api/{mod.rs,registration.rs}` and `src/lua_api/{state.rs,state/sim_state.rs}` — share catalog storage without publishing unrelated profile members.
- `src/lua_api/workarounds/temporary/{housing_owned_house_requests,pet_battle_action_legality_defaults,forever_bank_capability_defaults}.rs` — named missing ownership service, battle legality and bank metadata/capability systems.
- `src/lua_api/workarounds/temporary/housing_dashboard_preload.rs` — reuses the same owned-house request definition instead of a late-only duplicate.

## Tests asserting this spec

- `tests/cfactory_m5_services.rs` — complete original pet UI chunks, generated icon/housing/bank/set contracts and asynchronous ownership response.

## Known gaps (current cycle)

- [ ] New test-only revision RED and targeted GREEN; combined stock startup matrix.

## Out of scope

Wrath behavior; authoritative ownership, swap locks/turn legality, purchased bank-tab metadata and bank-type capability policies. No invented sample pets/houses/tabs, vendor changes or arbitrary member fabrication.
