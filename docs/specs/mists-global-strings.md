# Mists global strings

Only Mists' catalog selection changes. It publishes the complete enUS `5.5.4.70268` GlobalStrings catalog from [the pinned snapshot](../../data/db2/mists-5.5.4.70268/global-strings-provenance.json). The [investigation](../wiki/investigations/mists-global-strings.md) describes selection, generation and the accepted stock exposure gap.

## What it must do

- [x] Generate all19,914 unique pinned tags, including both decrypted DB2 sections; preserve empty strings, multiline text, quotes and format tokens. Full DB2/CSV field agreement and byte-exact PHF regeneration passed generation acceptance; recurring generator fixtures cover the two-section/multiline/empty-pointer cases.
- [x] Reject a non-pinned DB2 before replacing CSV/PHF output; reject duplicate CSV tags rather than choosing a winner. Duplicate output-preservation and invalid string-pointer fixtures pass; the wrong-DB2 acceptance probe preserves both outputs.
- [x] Let selected original Mists AdjustDisplaySettings register `showTargetCastbar` with `Show Enemy Cast Bar` (ID51586), Boolean type, and both following controls; observe original131/138 without modifying or stopping the consumer.
- [x] Keep absent `OPTION_TOOLTIP_SHOW_ENEMY_CASTBARS` nil. Do not consult a foreign table or invent text.

## How it works

- [Selection, source and generation](../wiki/investigations/mists-global-strings.md#source-and-selection)
- [Original-consumer proof](../wiki/investigations/mists-global-strings.md#executed-bounded-consumer-proof)
- [Accepted stock exposure and ungrounded seed](../wiki/investigations/mists-global-strings.md#stock-exposure-and-ungrounded-seed-gap)

## Implementation inventory

- `Cargo.toml`, `crates/sim-data-profile/Cargo.toml`, `crates/sim-data-profile/src/lib.rs` — forward the Mists feature and select one compiled table for both existing consumers.
- `crates/sim-data-profile/data/global_strings_mists.rs` — generated PHF catalog.
- `data/db2/mists-5.5.4.70268/` — multiline-safe CSV and CK/EK/DB2/CSV/generator hash manifest.
- `tools/gen_mists_global_strings.py` — pinned decrypted DB2/CSV generation, scoped to this build's layout.

## Tests asserting this spec

- `tests/mists_global_strings.rs` — selected original consumer, concrete setting, absent tooltip and next engraving boundary.
- `tools/tests/test_gen_mists_global_strings.py` — two sections, multiline/quoted tokens, empty-pointer semantics, invalid offsets and duplicate rejection.

Generation acceptance also compared every real DB2/CSV field, rejected a wrong payload and regenerated the committed PHF byte-exactly. Reproduce the PHF with `python3 tools/gen_mists_global_strings.py`; add `--db2 <verified-decrypted-globalstrings.db2>` to regenerate the CSV from the hash-pinned complete payload. BLTE acquisition/decryption is outside the generator.

## Known gaps (current cycle)

- [ ] Obtain native evidence for the temporary Retail reputation seed's ungrounded Mists inputs. The account-wide query is declared in the pinned Mists API surface; its native values are not established. This lane accepts the attributed6/12→8/14 stock delta without changing the seed or adding a label.

## Out of scope

- Repairing engraving or the reputation model: separate backing-state/native-evidence work.
- A full SettingsRegistrar pass, visible-panel interaction, rendering parity or other profiles' full runtime localization parity: not established by this constructor boundary or compilation checks.
- Other profiles' catalog changes or a foreign-table fallback: this feature selects Mists data only.
