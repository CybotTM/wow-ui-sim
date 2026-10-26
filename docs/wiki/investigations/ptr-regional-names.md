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

## GREEN and preservation

Producer revision `77db393c3` passes the same exact selector: both regional and surname policy transitions, actual player/nonplayer names, one-value returns, unknown-policy rejection, and environment isolation. The existing CanPortGraveyard supplied-state and original PTR GhostFrame selectors pass unchanged. Forever 70532's original NameUtil transitions/default/arity and graveyard projection also pass on a byte-identical private cache copy.

All cases have one-test listings, zero ignored tests, fresh byte-exact current CVar copies, and unchanged original CVar stat/hash guards. PTR/Forever all-target checks pass with only the three inherited `fast_handler_late_bind.rs` dead-code warnings; Retail passes with zero warnings. Cargo JSON and plain stderr were inspected. Formatting passes.

## Stock changes and unmasked successors

PTR's saved 296 keys/410 occurrences becomes 272/348. All 13 UnitFrame155 primary keys/30 occurrences disappear: 26 raw keys/60 occurrences including ScriptErrors echoes. There are 30 removed and 6 added raw keys, with no counts changed on retained keys:

- PlayerFrame initializes `feedbackText` at `Mainline/PlayerFrame.lua:51` after `UnitFrame_Initialize` returns. Its former CombatFeedback nil pair disappears (6 occurrences).
- PlayerFrame now reaches `RegisterEvent("PLAYER_PVP_FLAG_CHANGED")` at `PlayerFrame.lua:73`, after the repaired name call in `UnitFrame_Initialize`; the unsupported event adds one primary/reporter pair.
- `PLAYER_ENTERING_WORLD` now passes `UnitFrame_Update` at `PlayerFrame.lua:130`, reaches `PlayerFrame_UpdateArt` and `PlayerFrame_ToPlayerArt`, then `BuffFrame:Update()` at line 696. `BuffFrameMixin:UpdateAuras` at `BuffFrame.lua:809` reaches missing `C_Item.GetWeaponEnchantInfo` at line 766. This adds one primary/reporter pair; UnitFrame's stored-event wrapper is at lines 998–1000.
- The existing OverrideActionBarExpBar `GameRulesUtil.lua:144` / `C_GameRules.IsWoWHack` pair changes only its generated stored-invoker hash (`aa210eb0546614a3`→`1157f9ecb24367c6`). The full error and traceback are otherwise byte-identical; this is not a new missing API.

These successors remain out of scope. No GameRules, event-catalog, weapon-enchant, RecentAllies, or housing repair is included. Forever's stock map stays exactly 26 keys/26 occurrences with no additions, removals or count changes. Both profiles still exit 1 for known stock errors; neither is clean startup.

## Sources

- [Contract and test inventory](../../specs/regional-unique-names.md).
- [TOC selection](../../addon-loading-pipeline.md).
- [Consumer selection and RED rules](consumer-selection-and-red-proof.md).

## See Also

- [[patch-12-1-7-api-audit]] — PTR publication boundaries.
