# Anniversary saved-artifact audit

Model lane: **PASS** — 16/16 named cases; saved execution and compile receipts exit 0.
Source lane: **BLOCKED**. BCC, AllowLoadGameType, native catalog: **unknown**. Pure supplied-state model only.

Lane: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/anniversary`
Saved build revision: `842098a4e497c76bbf86e813a2e15e12600178b3`
Source comparison: `1dcc59c0788d0e09ee3d02516123df965ba8dc63`

## Authentication
- named_cases: PASS
- stdout_summary: PASS
- execution_receipt_exit_zero: PASS
- cargo_json: PASS
- sealed_binary: PASS
- eight_inputs: PASS

Cargo JSON: 741 records; one integration executable; sole client feature `client-anniversary`; successful build-finished.
Sealed binary: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/anniversary/integration-test`
SHA-256 (one streaming pass): `ddbb00ea425d434ab2b0009415c787b5912e50acd7fb0af4a519455907963333`; 433285152 bytes.
Stdout: `test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 8012 filtered out; finished in 1.45s`

## Named cases
- `currency_info_probes::get_backpack_currency_info_reflects_successive_existing_currency_updates`: ok
- `currency_info_probes::get_backpack_currency_info_returns_nothing_when_existing_currency_row_is_removed`: ok
- `currency_info_probes::get_basic_currency_info_returns_display_info_for_saved_instances_hover`: ok
- `currency_info_probes::get_currency_container_info_returns_display_info_with_passed_quantity`: ok
- `currency_info_probes::get_currency_container_info_returns_nothing_for_unknown_id`: ok
- `currency_info_probes::get_currency_info_covers_saved_instances_currency_ids`: ok
- `currency_info_probes::get_currency_info_exposes_all_retail_fields`: ok
- `currency_info_probes::get_currency_info_from_link_parses_currency_hyperlink`: ok
- `currency_info_probes::get_currency_info_from_link_returns_nothing_for_non_currency_link`: ok
- `currency_info_probes::get_currency_info_from_link_returns_nothing_for_unknown_currency_id`: ok
- `currency_info_probes::get_currency_info_matches_new_saved_instances_currency_metadata`: ok
- `currency_info_probes::get_currency_info_reflects_sim_state_mutation`: ok
- `currency_info_probes::get_currency_info_returns_nothing_for_unknown_id`: ok
- `currency_info_probes::get_currency_info_returns_seeded_valorstones_row`: ok
- `currency_info_probes::get_currency_list_info_reflects_successive_existing_currency_updates`: ok
- `currency_info_probes::get_currency_list_info_returns_nil_when_existing_currency_row_is_removed`: ok

## Eight scoped inputs
Before / after / current / source-1dcc SHA-256 agree for each listed input.
- `/home/osso/Projects/wow/wow-ui-sim/Cargo.lock`: PASS; `5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1`
- `/home/osso/Projects/wow/wow-ui-sim/Cargo.toml`: PASS; `91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9`
- `/home/osso/Projects/wow/wow-ui-sim/build.rs`: PASS; `cdebff6b6dec0e19e1124e734b2603432e2c47a7b956f9a4568ae412de182741`
- `/home/osso/Projects/wow/wow-ui-sim/src/c_api/item_spell/c_currency.rs`: PASS; `3ad4a0b1bdb62cdb9c408fbe06ae850f2ca57800957da2eec30c689500e30204`
- `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/globals/currency_data.rs`: PASS; `5ba0a32cca4b7df3bf95acb3fd2d87e14ab754c4d772429d756da5a2c3ec1c1a`
- `/home/osso/Projects/wow/wow-ui-sim/src/mists/compat_bootstrap.lua`: PASS; `f70b2b24970b7d906a69e401f1ed87e3c56a6195b96b9ca5f564f00ef4623ea8`
- `/home/osso/Projects/wow/wow-ui-sim/tests/currency_info_probes.rs`: PASS; `55d4df850f110436df482606ea32c832e36c0b88e46de80db556eaa676afec35`
- `/home/osso/Projects/wow/wow-ui-sim/tests/mists_currency_list.rs`: PASS; `20882151b1434af4de470a4e062809a1ce38d02ee0d604d35879618eb7c50366`

## Evidence
- `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/anniversary/compile-submission.json`
- `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/anniversary/compile-result.json`
- `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/anniversary/compile.stdout`
- `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/anniversary/compile.stderr`
- `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/anniversary/artifact.json`
- `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/anniversary/execution.json`
- `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/anniversary/currency.stdout`
- `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/anniversary/currency.stderr`
- `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/scope.json`

## Limits
Saved compile.stderr contains six iced_wgpu manifest lint-key deprecation warnings; not warning-free.
- Saved receipts/logs internally agree; not cryptographic proof of historical execution or complete build-input provenance.
- Eight scoped inputs match; unlisted dependencies, fixtures and build inputs are not authenticated by this comparison.
- Execution exit 0 is authenticated from the saved receipt, not a new process execution.
No rerun, build, delegation, vendor sync/loading, cwd change or source mutation; only report outputs written.
No source-lane reconfirmation; BLOCKED is preserved, not re-evaluated.
