# API Coverage

Current state of WoW API implementation across C_* namespaces, LE_* constants, and global functions.

## Coverage Summary

- **C_* namespaces**: ~97% coverage (~3700 of ~3807 methods used by BlizzardUI)
- **LE_* constants**: ~99% — only 8 missing, all legacy/Classic-era
- **Enum namespaces**: ~99% — 11 missing after scanner false-positive fix, mostly low-usage

## Implementation Layers

Three layers in priority order (hand-written wins via `is_nil()` guard):

1. **Hand-written Rust** (`c_*.rs`, `*_api.rs`) — real logic, state-backed
2. **Hand-written stubs** (`c_stubs_api*.rs`, `c_misc_api*.rs`) — hardcoded correct defaults
3. **Auto-generated** (`generated_stubs.rs`, ~19K lines) — catch-all nil/false/0 returns

Additionally, ~25 global (non-C_*) stubs have been upgraded from auto-generated nil to correct return values (player API, unit API, system API, etc.).

## Well-Implemented Namespaces

| Namespace | Notes |
|-----------|-------|
| C_Timer | After, NewTicker, NewTimer — fully functional |
| C_Item | GetItemInfo, GetItemInfoInstant, item class/subclass lookups |
| C_Container | GetContainerItemID/Link/Info, HasContainerItem |
| C_Map | GetAreaInfo, GetMapInfo, GetWorldPosFromMapPos |
| C_QuestLog | GetNumQuestLogEntries, GetInfo, quest ID lookups |
| C_EditMode | GetLayouts, GetAccountSettings |
| C_Reputation | GetFactionDataByID/Index, GetNumFactions |
| C_ColorOverrides | GetColorForQuality, GetDefaultColorForQuality |
| C_TransmogCollection | PlayerHasTransmog variants |

## Missing APIs (107 methods total)

All missing methods are in niche or expansion-specific systems:

**Store/Token** (39 methods) — C_StoreSecure, C_WowTokenSecure — glue-screen login/purchase flows, not needed for in-game addon testing.

**Housing** (20 methods) — C_HousingCatalog, C_HouseExterior, C_HousingCustomizeMode, C_HousingDecor, C_HousingNeighborhood — new TWW expansion system.

**Delves/Scenarios** (12 methods) — C_DelvesUI, C_ScenarioInfo — expansion-specific content.

**PvP** (6 methods) — C_PvP world PvP area and lockout map queries.

**Miscellaneous** (30 methods) — guild MOTD, transmog outfit management, talent loadout switching, aura data provider management, 1-2 methods each across ~15 namespaces.

## Missing Namespaces (16 total)

Only 16 C_* namespaces from BlizzardUI are completely unregistered. All are either glue-screen (login/character creation) or removed systems:

- **Glue-screen**: C_CharacterCreation (194 refs), C_RealmList, C_StoreGlue, C_PaidServices, C_WowTokenGlue, C_ConfigurationWarnings, C_SocialContractGlue
- **Removed**: C_Reforge (pre-WoD), C_GlyphInfo (pre-Legion), C_ArtifactRelicForgeUI (Legion)
- **Other**: C_LiveEvent, C_MapInternal, C_Barbershop

## Stub Methodology

To implement a stub: find it in layer 1/2/3, replace the body with proper return values matching `Blizzard_APIDocumentationGenerated/*.lua`. The signature audit in `docs/c-api-signature-audit.md` documents the exact parameter/return types for high-priority namespaces (C_Spell, C_SpellBook, C_CVar, C_PlayerInfo, C_ClassColor, C_ChatInfo, C_CurrencyInfo, etc.).

Dead stubs (duplicates of hand-written implementations) were cleaned up: 854 removed from `generated_stubs.rs`, reducing it from ~22K to ~19K lines.

## Sources

- [FUTURE.md](../../../FUTURE.md) — stub implementation tasks and methodology
- [docs/c-api-signature-audit.md](../../c-api-signature-audit.md) — official API signatures (Patch 12.0.1)
- [docs/c-api-stub-audit.md](../../c-api-stub-audit.md) — current implementation status per namespace

## See Also

- [[cli-commands]] — `audit-api` command for live gap reports
- [[architecture-overview]] — three-layer implementation architecture
