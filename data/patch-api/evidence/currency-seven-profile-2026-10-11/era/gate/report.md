# Era retry2 saved-artifact audit

**PASS — Era only: 16/16 named cases, 0 failed, 0 ignored, saved exit 0.**

Lane: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/era`
Epoch revision: `842098a4e497c76bbf86e813a2e15e12600178b3`; observed current HEAD: `9245bdddfa1d558771153b1626313c896b31e160`.

## Evidence
- stdout: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/era/currency.stdout`
- Exact summary: `test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 8012 filtered out; finished in 1.13s`
- Exit receipt: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/era/execution.json`; compile exit 0: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/era/compile-result.json`.
- Streams authenticate all 16 names against current test declarations and execution receipt; stderr contains 16 environment starts/completions, no detected panic/error/vendor-load markers.
- Cargo JSON: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/era/compile.stdout`; exactly one integration test executable artifact; features `casc, client-era, gui, rodio, sound`. Sole client feature: `client-era`; build-finished success true.
- Seal hashed once: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/era/integration-test`, 433487728 bytes, SHA-256 `3217f31dbca619b0f980b630c2c8d3650306a0c4c4b61d6c4e9c594219721188`; matches artifact receipt and executed path.
- Eight inputs match parent epoch scope, compile-before, execution-after, current files, and source `1dcc59c07`; scoped equality does not imply whole-tree identity.

## Eight-input scope
| Absolute path | SHA-256 | Epoch / before / after / current / source1dcc |
|---|---|---|
| `/home/osso/Projects/wow/wow-ui-sim/Cargo.lock` | `5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1` | PASS / PASS / PASS / PASS / PASS |
| `/home/osso/Projects/wow/wow-ui-sim/Cargo.toml` | `91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9` | PASS / PASS / PASS / PASS / PASS |
| `/home/osso/Projects/wow/wow-ui-sim/build.rs` | `cdebff6b6dec0e19e1124e734b2603432e2c47a7b956f9a4568ae412de182741` | PASS / PASS / PASS / PASS / PASS |
| `/home/osso/Projects/wow/wow-ui-sim/src/c_api/item_spell/c_currency.rs` | `3ad4a0b1bdb62cdb9c408fbe06ae850f2ca57800957da2eec30c689500e30204` | PASS / PASS / PASS / PASS / PASS |
| `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/globals/currency_data.rs` | `5ba0a32cca4b7df3bf95acb3fd2d87e14ab754c4d772429d756da5a2c3ec1c1a` | PASS / PASS / PASS / PASS / PASS |
| `/home/osso/Projects/wow/wow-ui-sim/src/mists/compat_bootstrap.lua` | `f70b2b24970b7d906a69e401f1ed87e3c56a6195b96b9ca5f564f00ef4623ea8` | PASS / PASS / PASS / PASS / PASS |
| `/home/osso/Projects/wow/wow-ui-sim/tests/currency_info_probes.rs` | `55d4df850f110436df482606ea32c832e36c0b88e46de80db556eaa676afec35` | PASS / PASS / PASS / PASS / PASS |
| `/home/osso/Projects/wow/wow-ui-sim/tests/mists_currency_list.rs` | `20882151b1434af4de470a4e062809a1ce38d02ee0d604d35879618eb7c50366` | PASS / PASS / PASS / PASS / PASS |

## Named cases
- `currency_info_probes::get_backpack_currency_info_reflects_successive_existing_currency_updates` — ok
- `currency_info_probes::get_backpack_currency_info_returns_nothing_when_existing_currency_row_is_removed` — ok
- `currency_info_probes::get_basic_currency_info_returns_display_info_for_saved_instances_hover` — ok
- `currency_info_probes::get_currency_container_info_returns_display_info_with_passed_quantity` — ok
- `currency_info_probes::get_currency_container_info_returns_nothing_for_unknown_id` — ok
- `currency_info_probes::get_currency_info_covers_saved_instances_currency_ids` — ok
- `currency_info_probes::get_currency_info_exposes_all_retail_fields` — ok
- `currency_info_probes::get_currency_info_from_link_parses_currency_hyperlink` — ok
- `currency_info_probes::get_currency_info_from_link_returns_nothing_for_non_currency_link` — ok
- `currency_info_probes::get_currency_info_from_link_returns_nothing_for_unknown_currency_id` — ok
- `currency_info_probes::get_currency_info_matches_new_saved_instances_currency_metadata` — ok
- `currency_info_probes::get_currency_info_reflects_sim_state_mutation` — ok
- `currency_info_probes::get_currency_info_returns_nothing_for_unknown_id` — ok
- `currency_info_probes::get_currency_info_returns_seeded_valorstones_row` — ok
- `currency_info_probes::get_currency_list_info_reflects_successive_existing_currency_updates` — ok
- `currency_info_probes::get_currency_list_info_returns_nil_when_existing_currency_row_is_removed` — ok

## Preserved historical Era epochs
- `/home/osso/.local/state/wow-ui-sim/verification/currency-other-profiles-current/20261011T074905Z`: RESOURCE_BLOCKED, 0 executed cases; 5 attempts, none admitted. Unmodified.
- `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-current/20261011T080007Z`: RESOURCE_BLOCKED, 0 executed cases; 5 attempts, none admitted. Unmodified.

## Limits
Anniversary and Wowforever remain pending/not audited here. No parent/matrix completion claim. No rerun, build, delegation, vendor loading/acquisition, cwd change, or native/profile startup acceptance.
Saved receipt records OS exit 0; streams independently corroborate results, not process exit provenance.
Compile stderr retains six iced-wgpu manifest-key deprecation warnings; no clean-warning claim.
Full structured evidence: `/home/osso/.local/state/wow-ui-sim/verification/currency-retry2-era-final/report.json`.
