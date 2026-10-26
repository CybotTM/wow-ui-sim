# Active loss of control

## Contract

A new simulated world has no active loss-of-control effects. Supplied entries are ordered per unit. Player queries and explicit player-unit queries read the same entries; other units are independent. Removing entries returns counts to zero and indexed queries to nil. `A_Admin.AddLossOfControl(unit, data)` appends a declared record and returns its one-based index. `A_Admin.ClearLossOfControl(unit)` removes that unit's entries. Fixture input does not synthesize native events or infer effects from auras.

Entries follow `Blizzard_APIDocumentationGenerated/LossOfControlDocumentation.lua` in each pinned runtime cache:

| Profile | Queries | `LossOfControlData` fields |
| --- | --- | --- |
| Retail | 11–81 | 151–161 |
| PTR | 11–81 | 151–161 |
| Mists | 11–77 | 146–156 |
| Era | 11–77 | 146–156 |
| Anniversary | 11–77 | 146–156 |
| Forever | 11–82 | 151–161 |

All six declare numeric `iconTexture`, optional numeric `startTime`, `timeRemaining`, `duration`, and `auraInstanceID`, required string `locType` and `displayText`, and numeric `spellID`, `lockoutSchool`, `priority`, and `displayType`. All six declare `GetActiveLossOfControlDuration` as an optional `LuaDurationObject`, not a numeric number of seconds. Duration objects derive from the supplied timing fields. No timing is invented when these fields are absent.

Wrath has no pinned generated API documentation or FrameXML consumer for this namespace. It must not publish `C_LossOfControl`.

`LossOfControlInfo` models spell cooldowns, not active effect records: its `modRate`, `isActive`, and `shouldReplaceNormalCooldown` fields do not fit the active-effect declaration. Active effects require a distinct record.

## Consumer proof

The original selected `Blizzard_FrameXML/LossOfControlFrame.lua` `LossOfControlMixin:UpdateDisplay` branches on absent/present data. It must hide for the empty world, show a supplied effect, then hide after removal. The test must run unchanged on the seeded producer and corrected producer. No vendor code is replaced.
