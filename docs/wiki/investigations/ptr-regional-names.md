# PTR regional names and surname visibility

The original PTR UnitFrame name branch needs two state-backed queries in order. The [regional-name contract](../../specs/regional-unique-names.md) keeps regional naming separate from supplied surname visibility and from native-default inference.

## Source selection and failure boundaries

PTR wowt 12.1.7.70493 `Blizzard_APIDocumentationGenerated/UnitDocumentation.lua:536` declares `RegionalUniqueNamesEnabled()` returning one boolean. `PlayerInfoDocumentation.lua:358` declares `C_PlayerInfo.ShouldDisplaySurname()` returning one boolean.

`Blizzard_UnitFrame.toc:26` selects `[Family]/UnitFrame.lua`; PTR's family is Mainline and its game is Standard (`src/toc/mod.rs`). The Game dependency closure therefore installs the original Mainline `UnitFrame_Update`. `Blizzard_FrameXMLUtil.toc` selects `NameUtil.lua`, not its Camelot override. Tests check the original UnitFrame bytes and both loaded function sources.

At `UnitFrame.lua:155`, the regional query runs first. The surname query is reached only with regional naming enabled and `UnitIsUnit(self.unit, "player")` true. Missing regional publication therefore masks missing surname publication; the two nil-call failures require separate producer revisions to attribute them honestly.

## Model and fixture boundaries

The existing `PlayerState.regional_unique_names_enabled` is shared by PTR and Forever. `PlayerState.should_display_surname` is optional supplied input; the PTR C_PlayerInfo query rejects None explicitly. No CVar or native account-default policy is inferred. `CanPortGraveyard` remains a separate query in the same original registration gate.

The exact consumer selector supplies `Ada-Lovelace` for the player, `Mira-Stone` for a nonplayer target, and a separator fixture. Whole→first→whole→first→whole local-name outputs distinguish either constant boolean from the supplied transitions. The nonplayer keeps its full name even when surname policy is unknown. The fixture does not establish native separator publication. Forever preservation uses its own pinned 70532 cache.

## Sequential RED proof

The exact `ptr_regional_names::original_consumer::unit_frame_tracks_regional_and_surname_policy_transitions` selector has byte-identical tests on the RED and producer branches. Both exact one-test listings succeeded and both runs failed with exit101 at the original consumer:

1. `677821014`: regional=false, `UnitFrame.lua:155: attempt to call a nil value`; surname is short-circuited.
2. `ddbc800f8`: the regional producer is installed, the disabled-policy full-name assertion passes, then regional=true reaches `UnitFrame.lua:155: attempt to call field 'ShouldDisplaySurname' (a nil value)`.

Complete GREEN and preservation remain pending. The baseline UnitFrame155 family is 13 primary keys / 30 occurrences, or 26 raw keys / 60 occurrences including ScriptErrors echoes, within PTR's saved 296/410 map. New successors must remain explicit and must be traced to their original emitter and masking predecessor.

## Sources

- [Contract and test inventory](../../specs/regional-unique-names.md).
- [TOC selection](../../addon-loading-pipeline.md).
- [Consumer selection and RED rules](consumer-selection-and-red-proof.md).

## See Also

- [[patch-12-1-7-api-audit]] — PTR publication boundaries.
