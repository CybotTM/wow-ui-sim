# Mists global strings

## Contract

- `client-mists` publishes the 19,914 unique tags from the pinned enUS build `5.5.4.70268` GlobalStrings DB2 (FDID 1394440). Other profiles retain their existing generated table.
- Preserve source text, format tokens, empty strings, quotes and multiline CSV fields. Runtime uses the existing WoW escape resolver.
- Absent tags remain absent: `OPTION_TOOLTIP_SHOW_ENEMY_CASTBARS` is not in this build. Do not consult another profile's table or invent localized text.
- Both Lua string registration and XML fontstring lookup use the same selected table.
- Existing independently registered numeric/constants behaviour is unchanged.

## Source and reproduction

Snapshot: `data/db2/mists-5.5.4.70268/GlobalStrings.csv`. The adjacent `global-strings-provenance.json` pins CK, EK, decrypted DB2, CSV and generator hashes.

```text
python3 tools/gen_mists_global_strings.py
python3 tools/gen_mists_global_strings.py --db2 <verified-decrypted-globalstrings.db2>
```

The first command regenerates the PHF from the committed multiline-safe CSV. The second first verifies the pinned DB2 SHA256 and content-key MD5, then decodes both WDC5 sections and writes the CSV. BLTE acquisition/decryption is not performed by this generator; its input must be the complete verified decrypted payload, including the second section. The generator is scoped to this build's three-field, 12-byte-record, palette-flags layout, not a general DB2 decoder.

## Proof scope

`tests/mists_global_strings.rs` loads the runtime-selected original Classic Settings closure. It checks the real `showTargetCastbar` setting uses `Show Enemy Cast Bar` (ID51586), advances beyond original `InterfaceOverrides.lua:131`, and leaves the absent tooltip nil. Unsupplied engraving at original line138 is a separate boundary, not a localization repair. No full SettingsRegistrar pass or visible-panel interaction is claimed.

Generator fixtures cover two decrypted sections, relative string pointers, empty-string sentinels, multiline/quoted format text, and duplicate rejection before replacing the output.
