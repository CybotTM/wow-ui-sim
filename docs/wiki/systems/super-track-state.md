# SuperTrack state

CF-2 stages environment-owned tracked targets and quest-log-owned highlight rows for the original Blizzard_QuestSuperTracking consumer. Source edited only; runtime acceptance is pending. [Contract](../../specs/super-track-state.md).

## Ownership and publication

`SimState.super_track` holds `SuperTrackState { quest_id, content, map_pin, vignette }`; every field defaults to None. `c_api::c_super_track` owns the API endpoints. `QuestLogState.additional_highlights` holds supplied QuestAdditionalHighlights rows keyed by quest ID; no parallel quest log or fabricated row exists. `c_api::c_quest_log` projects the five fields in documented order.

`c_api/mod.rs::supports_super_tracking` selects Retail/PTR/Forever once at runtime, alongside the existing C_QuestLog registration. Modeled target installers were removed from temporary defaults and world-service defaults. Only the unsupported highest-priority nil query remains temporary, with retirement tied to native priority modeling. It is not inferred priority behavior.

Mutators commit state, release the borrow, then synchronously dispatch SUPER_TRACKING_CHANGED. The original manager refreshes its cache and publishes Supertracking.OnChanged. Per-category replacement, clear(quest setter 0), and clear-all are modeled without inventing cross-target exclusivity. Equal-write notifications and synchronous timing are simulator policies, not native evidence.

## Consumer selection and observations

Retail/PTR select the generic Blizzard_FrameXMLUtil TOC, Family=Mainline/Game=Standard; Forever selects Family=Mainline/Game=Camelot. All permit the mainline-gated QuestSuperTracking entry. Mists/Era/Anniversary reject this entry; no Wrath chain is established. Tests resolve the profile-selected TOC, check the selected consumer entry, retain its source hash in output, load dependencies and the complete original chunk through the loader, and observe manager notifications. They never replace producer closures or rewrite vendor Lua.

The stock selector also exercises content transitions: startup absence alone cannot reject constant nil. Active-quest selectors supply two different highlight tuples, assert all cached fields and map helpers, then clear stale outputs. Matching/nonmatching quest events and offer auto-tracking use the original event handlers. Task-only classification cannot be varied independently of the static world catalog and is explicitly held.

## Evidence bounds

Untracked quest nil is a **documented-optional representation**, not native sentinel parity. Unknown/0 highlight output is a **native evidence gap**: an absent supplied row raises an explicit simulator error, never an invented tuple. No native timing, deduplication, exclusivity or priority assertion is made.

Frozen Retail stock receipt: 36 unique messages / 42 occurrences. Only the QuestSuperTracking :52 error and its ScriptErrors wrapper (count 2 each) are attributed here. Conditional expected remainder: 34 unique / 38 occurrences, with those exact two messages removed and every other message/count unchanged. :56 is latent, not part of the 36; any new :56 error fails acceptance. This is an expectation, not an executed result.

## Sources

- [Contract and staged tests](../../specs/super-track-state.md)
- [M4 world inputs](../../specs/c-factory-m4-world-inputs.md) — historical absent-service policy, superseded for these targets.
- `Blizzard_FrameXMLUtil/Mainline/Blizzard_QuestSuperTracking.lua` and generated SuperTrackManager/QuestLog documentation in each profile cache — original consumer and nullable signatures; no vendor edits.

## See Also

- [[lua-api]] — C API subsystem boundary.
- [[c-factory-startup-followup]] — factory-removal residuals, not a clean-startup claim.
