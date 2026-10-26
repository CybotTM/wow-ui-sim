# Era and Anniversary engraving availability

The [availability contract](../../specs/era-engraving-availability.md) replaces the temporary constant-false query with the shared environment-local supplied reader. Missing state is an explicit error, not evidence that engraving is disabled.

## Root cause and selection

The temporary Classic startup defaults installed `IsEngravingEnabled() return false` only on Era/Anniversary. Pinned declarations on both profiles declare a bool result but establish no inactive default. The existing [Mists engraving inputs](mists-engraving-inputs.md) already represent availability as `Option<bool>`; the shared C API registrar now publishes that same reader on all three profiles through `client_profile::ACTIVE`. Only Mists continues to publish the other six modeled members.

Both Era and Anniversary select vanilla/classic game types (`src/toc/mod.rs`), Family Classic and Game Vanilla. Both pinned `Blizzard_CharacterFrame.toc:18,22` select Vanilla PaperDoll Lua/XML. `Vanilla/PaperDollFrame.xml:716,722` creates RuneFrameControlButton and binds original OnLoad; `PaperDollFrame.lua:950–954` reads availability and hides the control on false. Settings selects `Classic/InterfaceOverrides.lua` through its Classic TOC and reads availability at136. Both have a real conditional LoD EngravingUI entry at PaperDollFrame.lua935–948; that full system is not implemented by this slice.

## Evidence and limits

`tests/era_engraving.rs` loads the selected original addon closure with availability supplied before XML loading. Two selectors exercise opposite initial values, transitions, independent environments, named unset errors and recovery through the XML-bound OnLoad. Era producer-free `1aba02ad7` rejects true at the original control (both selectors exit101); the settings preservation attempt completes without errors with supplied false. Further profile/GREEN/stock/compiler evidence is pending.

Do not interpret the settings preservation attempt as full-settings behavior proof. Bags and full LoD EngravingUI remain excluded, as do the other25 declared C_Engraving members on these two profiles. Compiler checks are not runtime parity proof for other profiles. No stock input is supplied merely to suppress a newly exposed error.

## Sources

- Era and Anniversary pinned `Blizzard_APIDocumentationGenerated/EngravingInfoDocumentation.lua:179–186` — non-null bool result, not default policy.
- Each profile's selected `Blizzard_CharacterFrame/Vanilla/PaperDollFrame.lua:935–954` and XML716–724 — real control and LoD entry.
- [Availability spec](../../specs/era-engraving-availability.md) — bounded contract and exclusions.

## See Also

- [Mists engraving](mists-engraving-inputs.md) — existing shared input model.
- [Consumer selection and RED proof](consumer-selection-and-red-proof.md) — applicability and discriminating selectors.
