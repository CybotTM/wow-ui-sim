# Adventure Guide Boss Icons

Adventure Guide boss buttons can render blank when `EJ_GetCreatureInfo` returns `0` for a missing creature icon fileDataID. Blizzard's boss button Lua expects `nil` for that slot so its `or "Interface\\EncounterJournal\\UI-EJ-BOSS-Default"` fallback is selected.

## Content

`EncounterBossButtonMixin:Init` uses the first creature from an encounter as the boss button image:

```lua
local bossImage = select(5, EJ_GetCreatureInfo(1, elementData.bossID)) or "Interface\\EncounterJournal\\UI-EJ-BOSS-Default"
self.creature:SetTexture(bossImage)
```

Lua treats `0` as truthy, so returning `0` from slot 5 bypasses the fallback and then `Texture:SetTexture(0)` clears the texture. The concrete regression was encounter `2773` (`Zekvir`), whose first creature row has `icon_file_id: 0`.

The simulator now maps zero Encounter Journal creature icon fileDataIDs to Lua `nil` while preserving non-zero fileDataIDs. Regression coverage lives in `tests/encounter_journal_creature_icons.rs`.

## Sources

- [encounter_journal.rs](../../../src/lua_api/globals/missing_surface/encounter_journal.rs) — `EJ_GetCreatureInfo` tuple construction
- [encounter_journal_creature_icons.rs](../../../tests/encounter_journal_creature_icons.rs) — fallback and non-zero round-trip coverage
- [Blizzard_EncounterJournal.lua](../../../vendor/wow-ui-source/Interface/AddOns/Blizzard_EncounterJournal/Mainline/Blizzard_EncounterJournal.lua) — boss button image selection

## See Also

- [[lua-api]] — Lua API tuple surfaces and globals
- [[texture-atlas]] — texture path/fileDataID rendering pipeline
