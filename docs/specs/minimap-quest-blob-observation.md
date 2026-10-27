# Supplied minimap quest-blob observation

Retail 12.1.5.70448, PTR 12.1.7.70493 and Forever 1.60.1.70532 declare `C_Minimap.IsInsideQuestBlob(questID)` in their pinned `Blizzard_APIDocumentationGenerated/MinimapDocumentation.lua` at lines 151–163, 157–169 and 151–163 respectively. Each declares a required number, a non-nil boolean result and `SecretArguments = "AllowedWhenUntainted"`.

The simulator accepts per-quest boolean observations through `A_Admin.SetMinimapQuestBlobInside(questID, inside)`. The query reads the supplied observation. For an unsupplied quest it returns false as a simulator scenario default; native containment and its default are unknown. This input does not derive geometry, emit events or alter map rendering.

The original `QuestObjectiveItemButtonMixin:CheckUpdateInsideBlob` at `Blizzard_ObjectiveTrackerShared.lua:130–134` consumes the result. Tests exercise this consumer with both supplied values.
