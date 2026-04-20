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

## Sources

- [tests/blizzard_ui_unit.rs](../../../tests/blizzard_ui_unit.rs) — unit-lane example
- [tests/addon_coverage.rs](../../../tests/addon_coverage.rs) — addon-bootstrap lane example
- [docs/wiki/reference/addon-compatibility.md](addon-compatibility.md) — broader addon compatibility context

## See Also

- [[addon-compatibility]] — addon loading, load order, and bootstrap coverage context
- [[development-phases]] — broader test/coverage roadmap
