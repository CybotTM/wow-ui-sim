# Wrath F10 spellbook and talent fixture

F10 keeps the 50 original modern rows unchanged and adds ten separate cached-Wrath consumer cases. A native analogue never substitutes for an original row. The [bounded results](../../baselines/wrath-fixture-f10.md) own counts; this page owns failure interpretation.

## Consumer boundary

The existing Wrath-only parent loads original FrameXML in startup order. The fixture supplies known IDs6603/635/19750, full-ranks mode and original `ToggleSpellBook`; TalentUI loads separately from its native TOC and calls `PlayerTalentFrame_Toggle`. Spellbook/TalentUI source digests bind the cached consumers. No consumer/API is replaced by the fixture.

Cached frame names, geometry, visibility, tooltip and talent expectations are **INFERRED**. Laptop12340 panel declarations remain **UNCONFIRMED**; simulator execution is not a native capture. The [existing tab contract](../../specs/wrath-spellbook-tabs.md) only authenticates the bounded tab/full/compact API semantics, not populated spell rendering or talent metadata.

## Observed failure boundaries

- The populated name case fails at its authored `GetSpellName` assertion oracle after original open. Do not claim independent original-consumer RED for that absent global from this receipt.
- The icon remains ancestor-hidden and native texture lookup is nil. Static source diagnosis is consistent with `SpellButton_UpdateButton`'s nil-texture hide branch: the available pre-12 wrapper lacks book-slot mapping, while the separate Rust legacy book-slot binding is Mists-only. Active runtime global-function provenance and exclusive dispatch/layout cause were not instrumented. Known-spell views derive on read; do not invent stale-cache repair.
- Original `SpellButton_OnEnter` reaches `SpellBookFrame.lua:335`, where GameTooltip `SetSpell` is nil.
- Original talent toggle enters GlyphUI/TalentUI callbacks. `GetGlyphSocketInfo` and `GetUnspentTalentPoints` wrappers encounter a nil target; do not call the wrapper globals themselves nil. The callback-error assertion fails before the final visible-talent-button assertion, leaving visible talent backing unproven.
- Hidden talent root, loaded addon, nonzero spell-button rectangle, page label and second-toggle closure pass. Those passes do not establish visible spell content, tooltip behavior or talent data completeness.

Producer repair remains excluded. No fallback, modern-mixin shim, tolerance change, assertion softening, suppression, ignore or new hiding gate is justified by these failures.

## Target/proof boundaries

Retail keeps seven original rows in existing `prefork_full_ui`; the other43 are ordinary integration registrations. A zero-selected exact integration invocation for those seven is INVALID and retained, not credited. Correct target selection must bind ELF test paths to the candidate worktree and execute each original named case positively before/after. No test moves or new gates are needed.

## Sources

- [F10 source/results inventory](../../baselines/wrath-fixture-f10.md) — exact native mapping and original modern rows.
- [Wrath spellbook contract](../../specs/wrath-spellbook-tabs.md) — bounded authenticated API and unmodeled metadata.
- `tests/common/wrath_spell_panels_f10.rs` and `tests/wrath_spellbook_tabinfo.rs` — original consumer loading and unchanged assertions.
- `src/c_api/c_spell.rs`, `src/lua_api/workarounds/temporary/legacy_spell_globals.rs` — registration/slot interpretation evidence; not runtime dispatch instrumentation.

## See Also

- [Lua API architecture](../systems/lua-api.md#wrath-single-class-spellbook-projection) — existing known-spell projection.
- [Prefork harness](../systems/prefork-test-harness.md) — original Retail target ownership.
