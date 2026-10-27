# Wrath F10 spellbook and talent fixture

F10 keeps the 50 original modern rows unchanged and adds ten separate cached-Wrath consumer cases. A native analogue never substitutes for an original row. The [bounded results](../../baselines/wrath-fixture-f10.md) own counts; this page owns failure interpretation.

## Consumer boundary

The existing Wrath-only parent loads original FrameXML in startup order. The fixture supplies known IDs6603/635/19750, full-ranks mode and original `ToggleSpellBook`; TalentUI loads separately from its native TOC and calls `PlayerTalentFrame_Toggle`. Spellbook/TalentUI source digests bind the cached consumers. No consumer/API is replaced by the fixture.

Cached frame names, geometry, visibility, tooltip and talent expectations are **INFERRED**. Laptop12340 panel declarations remain **UNCONFIRMED**; simulator execution is not a native capture. The [existing tab contract](../../specs/wrath-spellbook-tabs.md) only authenticates the bounded tab/full/compact API semantics, not populated spell rendering or talent metadata.

## Observed failure boundaries

- The populated name case fails at its authored `GetSpellName` assertion oracle after original open. Do not claim independent original-consumer RED for that absent global from this receipt.
- The icon remains ancestor-hidden and native texture lookup is nil. Static source diagnosis is consistent with `SpellButton_UpdateButton`'s nil-texture hide branch: the available pre-12 wrapper lacks book-slot mapping, while the separate Rust legacy book-slot binding is Mists-only. Active runtime global-function provenance and exclusive dispatch/layout cause were not instrumented. Known-spell views derive on read; do not invent stale-cache repair.
- Original `SpellButton_OnEnter` reaches `SpellBookFrame.lua:335`, where GameTooltip `SetSpell` is nil.
- The original talent toggle previously reached nil `GetGlyphSocketInfo` at GlyphUI:65 and nil `GetUnspentTalentPoints` at TalentUI:499. Current diagnostic execution confirms the globals themselves were nil; the xpcall traceback label did not establish a callable wrapper. The [supplied-input contract](../../specs/wrath-glyph-talent-inputs.md) removes those two callback boundaries on Wrath. Two byte-identical consumer selectors fail at those exact calls on producer-free RED and pass on GREEN with distinct supplied values. The talent selector explicitly asserts the next original failure: nil `GetTalentTabInfo` at `FrameXML/TalentFrameBase.lua:118`. The visible-talent assertion remains unproven. Inspected and pet talent state is a separate follow-up; until supplied, those point queries return scenario-default zero.
- Hidden talent root, loaded addon, nonzero spell-button rectangle, page label and second-toggle closure pass. Those passes do not establish visible spell content, tooltip behavior or talent data completeness.

The glyph/talent repair is limited to supplied socket and point state. Wrath and Retail stock remain 0/0 on RED and GREEN. Seven all-target checks have zero compiler warnings; PTR was checked before a subsequent Wrath-only change, with its selected source unchanged. Spellbook and talent-tab producer repairs remain separate. No fallback, modern-mixin shim, tolerance change, assertion softening, suppression, ignore or new hiding gate is justified by these failures.

## Target/proof boundaries

Retail keeps seven original rows in existing `prefork_full_ui`; the other43 are ordinary integration registrations. A zero-selected exact integration invocation for those seven is INVALID and retained, not credited. Correct target execution binds ELF test paths to the candidate worktree and executes every original named case positively before/after. One after-run conformance setup failure occurred before the requested test; its INVALID attempt remains retained and one admitted exact retry passed. This is not a repeated actual test failure. No test moves or new gates are needed.

## Sources

- [F10 source/results inventory](../../baselines/wrath-fixture-f10.md) — exact native mapping and original modern rows.
- [Wrath spellbook contract](../../specs/wrath-spellbook-tabs.md) — bounded authenticated API and unmodeled metadata.
- `tests/common/wrath_spell_panels_f10.rs` and `tests/wrath_spellbook_tabinfo.rs` — original consumer loading and unchanged assertions.
- `src/c_api/c_spell.rs`, `src/lua_api/workarounds/temporary/legacy_spell_globals.rs` — registration/slot interpretation evidence; not runtime dispatch instrumentation.

## See Also

- [Lua API architecture](../systems/lua-api.md#wrath-single-class-spellbook-projection) — existing known-spell projection.
- [Prefork harness](../systems/prefork-test-harness.md) — original Retail target ownership.
