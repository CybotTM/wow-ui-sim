# Mists pinned global strings

The selected Mists settings constructor used a foreign generated string table. Original `Blizzard_SettingsDefinitions_Frame/Classic/InterfaceOverrides.lua:131` passed nil for `SHOW_ENEMY_CASTBARS`; real `Blizzard_Settings_Shared/Blizzard_Setting.lua:34` rejected the setting name. The fix selects the full pinned catalog, not a one-label shim.

## Source and selection

The [contract](../../specs/mists-global-strings.md) and [provenance manifest](../../../data/db2/mists-5.5.4.70268/global-strings-provenance.json) bind the CSV and generated PHF to the verified complete decrypted DB2. WDC5's two physical sections share concatenated logical record/string spaces. Relative pointer zero means empty text; Flags indexes the palette. The decrypted second section is included. The generator uses Python's multiline-safe CSV algorithm, preserving quoted text and format tokens.

`client-mists` forwards the data-crate feature. The established `sim-data-profile` cfg_attr selection site chooses the Mists module; the root re-export is unchanged. Both Lua registration and XML fontstring lookup therefore use this table. Other profiles do not compile the Mists PHF. Missing tags do not consult another profile's table: the enemy-castbar tooltip is genuinely absent in this build.

## Executed bounded consumer proof

`tests/mists_global_strings.rs` checks the runtime-selected Classic TOC and original Lua provenance. One exact byte-identical selector fails at original Setting.lua:34 on the producer-selection RED sibling and passes with the Mists table. A non-aborting original-function line observer proves131 and138; no pcall, hook exit, or vendor replacement masks the native error. The real `showTargetCastbar` setting has the pinned name `Show Enemy Cast Bar` (ID51586) and Boolean type, and both following controls exist. Unsupplied engraving at original138 remains a separate boundary. This does not prove a full SettingsRegistrar pass or visible-panel interaction.

## Stock exposure and ungrounded seed gap

Paired stock changes6/12→8/14. All original six keys and their counts are unchanged. The two added count1 keys are original `ReputationBar.lua:148`'s nil `REPUTATION_STATUS_BAR_LABEL_ACCOUNT_WIDE` concatenation and its ScriptErrors reporter. The pinned DB2 lacks that tag; the foreign table's `(Warband)` value masked this path.

The unchanged temporary [reputation seed](../../../src/lua_api/workarounds/temporary/reputation_state.rs) supplies watched faction2590 and accountWide=true on Mists. These are ungrounded Mists inputs. Its existing exclusion covers only Era/Anniversary; this lane does not change the seed or invent a label.

The pinned Mists API surface explicitly declares `IsAccountWideReputation` at `ReputationInfoDocumentation.lua:49–59` and the `FactionData.isAccountWide` field at133–153. Therefore a claim that the account-wide concept is absent is not established. Native values need separate evidence. A read-only scan of all2,673 Lua/XML cache files finds exactly one literal label reference, the concatenation at148, and no Lua assignment or XML definition. All seven full-feature all-target checks pass with zero JSON/plain compiler warnings. This is compilation proof, not other profiles' full runtime localization parity.

## Sources

- [Contract and reproduction](../../specs/mists-global-strings.md)
- [Generator](../../../tools/gen_mists_global_strings.py)
- [Original-consumer test](../../../tests/mists_global_strings.rs)
- [Data selection](../../../crates/sim-data-profile/src/lib.rs)

## See Also

- [[consumer-selection-and-red-proof]] — selected consumer and causal RED requirements
- [[mists-engraving-inputs]] — separate supplied engraving state
