# C_* Stub Implementation Plan

Turn stubbed C_* functions from returning nil/false/0 into returning properly-shaped data.

Source of truth: `Interface/BlizzardUI/Blizzard_APIDocumentationGenerated/*.lua` and `warcraft.wiki.gg`.

## Related Investigation Notes

- Character-select startup hitch investigation and current perf findings:
  [docs/character-select-performance.md](/syncthing/Sync/Projects/wow/wow-ui-sim/docs/character-select-performance.md)

## How stubs work today

Three layers, in priority order:
1. **Hand-written Rust** — `c_misc_api_core.rs`, `c_misc_api_game.rs`, `c_misc_api_ui.rs`, `spell_api.rs`, etc.
2. **Hand-written stubs** — `c_stubs_api.rs`, `c_stubs_api_extra.rs`, etc.
3. **Auto-generated** — `generated_stubs.rs` (22K lines, `is_nil()` guard so hand-written wins)

To "implement a stub" means: find the function in layer 1/2/3, replace its body with proper return values matching the Blizzard API docs in `docs/c-api-signature-audit.md`.

## Tasks

Each task is self-contained. Do one at a time. Run `cargo build --bin wow-sim` after each.

### Task 1: C_Spell — GetSpellInfo returns SpellInfo table

**File:** `src/lua_api/globals/spell_api.rs`
**What:** `C_Spell.GetSpellInfo(spellIdentifier)` currently returns nil for unknown spells. For known spells (from spellbook_data), return a SpellInfo table: `{ name, iconID, castTime, minRange, maxRange, spellID, originalIconID }`. Check `SpellDocumentation.lua` for the full SpellInfo structure.
**Also:** `GetSpellName(id)` should return the spell name string. `GetSpellTexture(id)` should return `(iconID, iconID)`. `DoesSpellExist(id)` should return true for known spells. `IsSpellDataCached(id)` should return true for known spells.
**Test:** `wow-sim --no-addons --no-saved-vars run-tests Wowless` should not regress.

### Task 2: C_Spell — Boolean query stubs

**File:** `src/lua_api/globals/spell_api.rs`
**What:** These C_Spell functions should return proper defaults instead of nil:
- `GetSpellCooldown(id)` → `{ startTime=0, duration=0, isEnabled=true, modRate=1 }` (not on cooldown)
- `GetSpellCharges(id)` → nil (no charges by default, which is correct)
- `IsSpellUsable(id)` → `(true, false)` for known spells
- `IsSpellHarmful(id)` → false, `IsSpellHelpful(id)` → false
- `IsSpellPassive(id)` → check spellbook_data `is_passive` field
- `GetSpellPowerCost(id)` → empty table (no cost data)
- `GetSpellSubtext(id)` → nil
- `GetSpellDescription(id)` → nil (no description data)
- `GetSpellLink(id)` → `"|cff71d5ff|Hspell:{id}|h[{name}]|h|r"` for known spells

### Task 3: C_SpellBook — GetSpellBookItemInfo returns proper table

**File:** `src/lua_api/globals/spell_api.rs`
**What:** `C_SpellBook.GetSpellBookItemInfo(slot, bank)` should return a SpellBookItemInfo table with fields: `{ name, subName, actionID, spellID, iconID, itemType, isPassive, isOffSpec }`. Data comes from `spellbook_data.rs`.
**Also:** `GetSpellBookItemType` should return `(itemType, actionID, spellID)`. `GetSpellBookItemName` should return `(name, subName)`.

### Task 4: C_CVar — Use real CVar defaults

**File:** `src/lua_api/globals/cvar_api.rs`
**What:** `GetCVarDefault(name)` currently exists. Verify that common CVars used by BlizzardUI have sensible defaults in the CVar store. Key ones:
- `nameplateShowFriendlyNPCs` → "0"
- `nameplateShowEnemies` → "1"
- `Sound_EnableSFX` → "1"
- `showTutorials` → "0"
- `profanityFilter` → "0"
Check `GetCVarInfo` returns the right 7-tuple shape.

### Task 5: C_CurrencyInfo — GetCurrencyInfo returns CurrencyInfo table

**File:** `src/lua_api/globals/c_misc_api_core.rs` (or wherever C_CurrencyInfo is registered)
**What:** `GetCurrencyInfo(type)` should return a CurrencyInfo table for known currencies: `{ name, description, currencyTypesID, iconFileID, quantity, maxQuantity, isDiscovered, ... }`. Data source: `currency_data.rs` already has currency entries.
**Also:** `GetCurrencyListInfo(index)` should use the same data. `GetCurrencyListSize()` should return the list length. `GetBackpackCurrencyInfo(index)` should return tracked currencies.
**Also:** `GetCoinTextureString(amount, fontHeight)` — format gold/silver/copper with texture escapes.

### Task 6: C_PlayerInfo — Return proper structures

**File:** `src/lua_api/globals/c_misc_api_core.rs`
**What:** Several C_PlayerInfo functions should return proper values:
- `GetAlternateFormInfo()` → `(false, false)` (no worgen/dracthyr form)
- `GetPlayerCharacterData()` → table with `{ name, level, className, raceName, ... }` from SimState
- `HasVisibleInvSlot(slot)` → `true` for equipment slots 1-19
- `GetNativeDisplayID()` → same as `GetDisplayID()`
- `IsDisplayRaceNative()` → `true`

### Task 7: C_ClassColor — Return real class colors

**File:** `src/lua_api/globals/c_misc_api_ui.rs`
**What:** `GetClassColor(className)` should return a CreateColor-compatible table with real WoW class colors. Static data:
- WARRIOR: 0.78, 0.61, 0.43
- PALADIN: 0.96, 0.55, 0.73
- HUNTER: 0.67, 0.83, 0.45
- ROGUE: 1.00, 0.96, 0.41
- PRIEST: 1.00, 1.00, 1.00
- DEATHKNIGHT: 0.77, 0.12, 0.23
- SHAMAN: 0.00, 0.44, 0.87
- MAGE: 0.25, 0.78, 0.92
- WARLOCK: 0.53, 0.53, 0.93
- MONK: 0.00, 1.00, 0.60
- DRUID: 1.00, 0.49, 0.04
- DEMONHUNTER: 0.64, 0.19, 0.79
- EVOKER: 0.20, 0.58, 0.50

Verify it returns a table with `r, g, b` fields and `GetRGB()`/`GetRGBA()` methods (CreateColor pattern).

### Task 8: C_ChatInfo — Basic chat infrastructure

**File:** `src/lua_api/globals/c_misc_api_core.rs`
**What:**
- `GetGeneralChannelID()` → `1` (should already be in generated_stubs, verify)
- `GetColorForChatType(chatType)` → return color table for known types (SAY=white, YELL=red, WHISPER=pink, GUILD=green, PARTY=blue, RAID=orange)
- `GetNumReservedChatWindows()` → `1`
- `ReplaceIconAndGroupExpressions(input)` → return input unchanged (no icon replacement in sim)
- `IsAddonMessagePrefixRegistered(prefix)` → `false`
- `RegisterAddonMessagePrefix(prefix)` → `0` (success)

### Task 9: C_SpecializationInfo — Return real spec data

**File:** `src/lua_api/globals/c_misc_api_ui.rs`
**What:** `GetSpecializationInfo(specIndex)` should return real data from SimState:
- `(specID, name, description, iconID, role, primaryStat, pointsSpent, background, 0, true)`
- Use the player's class/spec from state
**Also:** `GetNumSpecializationsForClassID(classID)` → return proper count (most classes have 3-4 specs).
**Also:** `GetAllSelectedPvpTalentIDs()` → empty table.

### Task 10: C_ChallengeMode — Return M+ map data

**File:** `src/lua_api/globals/c_misc_api_game.rs`
**What:** `GetMapTable()` should return current M+ dungeon IDs (static list). `GetMapUIInfo(mapID)` should return `(name, id, timeLimit, texture, bgTexture, mapID)` for known dungeons. `GetAffixInfo(affixID)` should return `(name, description, iconFileID)` for known affixes.
**Data:** Hardcode current season dungeon pool. Can get IDs from wowhead or wiki.

### Task 11: C_PartyInfo — Proper return shapes

**File:** `src/lua_api/globals/c_misc_api_core.rs`
**What:**
- `CanFormCrossFactionParties()` → `true`
- `GetActiveCategories()` → `{1}` (LE_PARTY_CATEGORY_HOME)
- `GetMinLevel()` → `1`
- `AllowedToDoPartyConversion(toRaid)` → `true`
- `GetLootMethod()` → `("personalloot", nil, nil)` (modern WoW default)

### Task 12: C_GossipInfo — Proper empty state

**File:** `src/lua_api/globals/c_misc_api_ui.rs`
**What:** Verify these return correct types (not nil where table expected):
- `GetOptions()` → empty table (not nil)
- `GetActiveQuests()` → empty table
- `GetAvailableQuests()` → empty table
- `GetText()` → `""` (empty string, not nil)
- `GetNumActiveQuests()` → `0`
- `GetNumAvailableQuests()` → `0`
- `ForceGossip()` → `false`

## How to work through this

1. Pick a task
2. Read the relevant source file
3. Read the matching `*Documentation.lua` from `Blizzard_APIDocumentationGenerated/`
4. Implement the changes
5. `cargo build --bin wow-sim` — must compile
6. `wow-sim --no-addons --no-saved-vars run-tests Wowless` — must not regress
7. Commit with descriptive message
8. Move to next task
