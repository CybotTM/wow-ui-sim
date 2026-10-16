# Supplied quest background material

## Contract

Retail/Forever `GetQuestBackgroundMaterial()` takes no arguments and returns one
nilable string. Explicit per-environment quest-ID materials in
`QuestLogState.background_materials` project through the current selected quest ID.
No selected/supplied material returns nil, not an invented material name. Existing
abandon/turn-in transitions remove the consumed quest's supplied material.

The original cached `QuestFrame_GetMaterial` retains responsibility for selecting
Parchment when nil or when contrast mode requests it. No Blizzard function is
replaced and no texture/material catalog is invented.

## Grounding

[Warcraft Wiki API documentation](https://warcraft.wiki.gg/wiki/API:GetQuestBackgroundMaterial)
(inspected 2026-10-16; page edited 2026-08-10) specifies `material =
GetQuestBackgroundMaterial()` and `string?`, with nil selecting the default Parchment.
This legacy global has no entry in the inspected cached generated Retail API docs.
Cached Retail `Blizzard_UIPanels_Game/Mainline/QuestFrame.lua:602-610` independently
shows the original nil-to-Parchment consumer branch. The corresponding Forever
consumer uses the same query.

## Implementation and proof

`quest_surface/legacy_globals.rs` reads the existing rich-state-owned supplied map;
`quest_surface/register.rs` publishes only in the same Retail/Forever scope as QL-2.
No simulator admin mutation endpoint is added.

`tests/quest_background_material.rs` loads the original cached consumer and verifies
unsupplied Parchment, one nil return, supplied Stone, selection changes, environment
isolation and abandonment cleanup. Passive map ingress was included in test commit
`4e5e1efd0`; `fe529b241` adds its empty seeded initializer. The first build's E0063
compile error is not RED. Repaired saved `fe529b241` produced two real Retail
assertion failures: None versus Parchment and None versus supplied Stone. GREEN and
full progress/reward dialog acceptance remain pending; unrelated missing consumer
APIs are separate blockers, not evidence for new defaults.

See [QL-2 dialog contract](quest-readiness-dialog.md).
