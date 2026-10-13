# UI Widget Manager State

Source checkpoint `67775404d` implements environment-local supplied UI widget sets, visualization snapshots, exact quest associations, and synchronous host feeds. Implementation is not GREEN: current validation remains pending. The [contract](../../specs/ui-widget-manager-state.md) owns requirements and test evidence; this page describes the bounded source model, not native catalog or transport parity.

## State and representation

`UiWidgetInputs` stores set records keyed by `WidgetId`, independent IconAndText records, and exact `(questID, setType)` associations. A set contains layout direction, vertical padding, and an ordered vector of explicit widget IDs, types, and optional unit tokens. Numeric keys preserve floating-point bits except equal signed zeroes; availability is not inferred from ID ranges.

Queries project fresh Lua tables from supplied state: unknown set metadata and IconAndText records return nil; unknown membership returns an empty table. IconAndText projects the complete supplied record, including tooltip, texture kits, timer, ordering, animation, scale, layout, and model-layer fields, without automatic field fillers. Existing complete ButtonHeader and Prey records remain separate backing inputs; the feed supports IconAndText type0, ButtonHeader type30, and feature-gated PreyHuntProgress type31. This is not coverage of every visualization type.

A C_API widget record is data, **not a Rust `Frame`**. Blizzard's cached Lua manager consumes that data and owns actual template selection, frame pools, activation, updates, and removal. Resulting ordinary frames participate in the existing [widget architecture](../../widget-system.md), [loading pipeline](../../addon-loading-pipeline.md), and [event dispatch](../../event-system.md). The producer does not replace the manager with Rust-rendered widget consumers.

## Exact demo associations

Initialization preserves the existing twelve demo world quests: three explicitly stored set types per quest (Tooltip, BehindIcon, AdventureMapDetails), totaling 36 sets and associations. The initializer derives the previous fixture IDs and supplies one IconAndText member per set with the `Worldquest` texture kit. Query-time lookup uses stored associations, not numeric thresholds or synthesized ranges. These are demo fixtures, not a native quest/widget catalog.

## Synchronous host feeds

`WowLuaEnv` exposes three source-implemented operations:

- `replace_ui_widget_set`: replace or remove a set, then synchronously publish `UPDATE_ALL_UI_WIDGETS`; independent visualization records survive.
- `update_ui_widget`: validate type and supplied-set existence, update an existing membership in place or append a new member, store the visualization, then synchronously publish `UPDATE_UI_WIDGET` with supplied identity/type/unit-token fields. Type mismatches and unknown sets fail before mutation or notification.
- `remove_ui_widget`: remove matching membership and publish `UPDATE_ALL_UI_WIDGETS` only when membership changed; return false for absent sets/members. Visualization snapshots remain independently queryable.

State is committed before notification. Full reconciliation and incremental consumer behavior belong to the cached Blizzard manager; source wiring alone does not prove their runtime lifecycle.

## Proof boundary and remaining surface

The contract records genuine cached-container RED at `6a852c322`: registration could not find supplied set417. Its host-secret query also failed at the old numeric extractor; later lifecycle/recovery assertions were unreached. `67775404d` supplies the model and feed, but current state/lifecycle/security validation is pending—not a successful replay of those assertions.

HUD set selections, unit feeds, and StatusBar/CaptureBar visualization queries remain unimplemented in this bounded model. No claim is made that temporary partial-load no-op behavior was replaced, that full autoload works, or that native DB2 catalogs, server wire protocols, all widget types, or native parity are covered. Verification is not complete.

## Sources

- [UIWidgetManager contract](../../specs/ui-widget-manager-state.md) — required behavior, staged tests, and historical RED boundary.
- [Manager module](../../../src/c_api/ui_widget_manager.rs) and [inputs](../../../src/c_api/ui_widget_manager/inputs.rs) — numeric identities, set/membership records, and complete supplied DTOs.
- [Queries](../../../src/c_api/ui_widget_manager/queries.rs), [feed](../../../src/c_api/ui_widget_manager/feed.rs), and [fixtures](../../../src/c_api/ui_widget_manager/fixtures.rs) — snapshot projection, synchronous producers, and bounded demo initialization at `67775404d`.
- [Task quest lookup](../../../src/lua_api/globals/quest_surface/task_quest.rs) — stored quest/set association lookup.
- [Widget architecture](../../widget-system.md), [loading pipeline](../../addon-loading-pipeline.md), and [events](../../event-system.md) — existing frame, cached Lua consumer, and dispatch boundaries.

## See Also

- [[widget-system]] — ordinary frames are distinct from C_API records.
- [[addon-loading]] — cached Blizzard consumer loading.
- [[event-system]] — synchronous host notification boundary.
- [[lua-api]] — exposed API inventory.
