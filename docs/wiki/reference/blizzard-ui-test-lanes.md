# Blizzard UI Test Lanes

Blizzard UI coverage is split into two lanes so the test intent matches the
startup model being exercised.

## Content

The unit lane is for isolated helpers and pure component behavior that can run
without loading a full Blizzard addon stack. In this repo, that lane lives in
[`tests/blizzard_ui_unit.rs`](../../../tests/blizzard_ui_unit.rs).

The addon-bootstrap lane is for behaviors that only exist after Blizzard
addons have loaded and their startup work has run. In this repo, that lane
lives in [`tests/addon_coverage.rs`](../../../tests/addon_coverage.rs).

The split is deliberate:

- unit tests should stay fast and narrowly scoped
- addon-bootstrap tests should verify load-order-sensitive behavior
- the heavier bootstrap path should not become the default for every regression

## First Smoke Targets

The first addon-bootstrap smoke targets are pinned to four representative
closure shapes:

- `combat_log` — mostly-functional single-addon surface via `Blizzard_CombatLog`
- `panel_templates` — template-heavy shared UI surface via `Blizzard_UIPanelTemplates`
- `world_map` — layout-heavy map canvas via `Blizzard_WorldMap`
- `settings_panel` — multi-addon flow via `Blizzard_SettingsDefinitions_Frame`

These targets are defined in
[`tests/common/blizzard_addon_manifest.rs`](../../../tests/common/blizzard_addon_manifest.rs)
and loaded through
[`tests/common/blizzard_addon_harness.rs`](../../../tests/common/blizzard_addon_harness.rs)
before being exercised by
[`tests/blizzard_addon_smoke_targets.rs`](../../../tests/blizzard_addon_smoke_targets.rs).

## Sources

- [tests/blizzard_ui_unit.rs](../../../tests/blizzard_ui_unit.rs) — unit-lane example
- [tests/addon_coverage.rs](../../../tests/addon_coverage.rs) — addon-bootstrap lane example
- [docs/wiki/reference/addon-compatibility.md](addon-compatibility.md) — broader addon compatibility context

## See Also

- [[addon-compatibility]] — addon loading, load order, and bootstrap coverage context
- [[development-phases]] — broader test/coverage roadmap
