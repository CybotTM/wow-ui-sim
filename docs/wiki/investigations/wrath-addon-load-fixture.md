# Wrath addon-load fixture ordering

Coverage fixtures omitted the profile's top-level FrameXML TOC. Production loads it before discovering Blizzard addons. Restoring that same prerequisite is a fixture correction, not a producer shim or a change to error budgets.

## Root cause

`addon_coverage_lod::load_startup_blizzard_ui` and the force-load case now call `client_profile::blizzard_ui_framexml_toc()` after resetting templates and before Blizzard addon discovery/loading. Profiles without a top-level TOC retain their existing path.

The cached Wrath core publishes MoneyFrame helpers, scroll/check-button templates, dropdown Lua, SparkleFrame, UIMenus, combat-log constants and chat textures. Omitting those producers caused downstream original-addon failures. TimeManager supplies a numeric hour to the original dropdown's FontString; the incomplete fixture instead reached the simulator's strict string-only dropdown bootstrap.

## Bounded evidence

Source-bound before receipts execute one test per case: Retail all-addon plus16 LoD shards pass; Wrath all-addon, prior-runtime, TimeManager and shards5,8–16 fail. The exact original TimeManager case fails again after the [harness source-binding fix](../systems/prefork-test-harness.md#shared-target-source-binding--2026-10-20), confirming that relocation repair alone does not fix fixture ordering.

FrameXML-corrected Wrath GREEN and Retail preservation remain pending. No errors are filtered, no baseline counts are increased, and no vendor or API producer behavior is changed by this correction.

## Native limits and laptop questions

Cached core TOC Interface30300 and its consumers are source evidence, not native3.4 acceptance. These native3.4 claims remain **INFERRED** pending laptop evidence:

- Does core FrameXML publish these prerequisites before Blizzard LoD addons?
- Does original TimeManager pass its numeric hour unchanged through dropdown Lua to FontString text `1`?

Residual failures must be traced after the corrected fixture executes; missing-core symptoms alone do not justify API shims.

## Sources

- [Production startup](../../../src/bin/wow_sim/addon_loading.rs) — optional profile FrameXML precedes Blizzard addon iteration.
- [Profile TOC lookup](../../../src/client_profile.rs) — profile-cache top-level TOC discovery.
- [LoD coverage](../../../tests/addon_coverage_lod.rs) — original runtime consumers and concrete TimeManager assertion.
- [Force-load coverage](../../../tests/addon_coverage.rs) — per-addon collected-error accounting.

## See Also

- [Addon loading](../../addon-loading-pipeline.md) — loader behavior.
- [Harness source binding](../systems/prefork-test-harness.md#shared-target-source-binding--2026-10-20) — separate stale-source and zero-selection failure.
