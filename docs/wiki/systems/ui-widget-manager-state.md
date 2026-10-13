# UI Widget Manager State

Environment-local supplied UI widget sets, visualization snapshots, exact quest associations and synchronous host feeds form the core model. Production `0745fe84f` adds supplied bars and HUD selectors. The [contract](../../specs/ui-widget-manager-state.md) owns requirements; the [retained attempts](../../../data/patch-api/evidence/ui-widget-manager-state-2026-10-13/README.md#later-consumer-attempts) own current outcomes and historical failures. Historical test-only `b28a45f91` has HUD OnLoad2 PASS and bar initial visibility FAIL; later `5bd208dba` closes the bounded Retail consumer assertions below. No native or whole-goal acceptance follows.

## State and representation

`UiWidgetInputs` stores set records keyed by `WidgetId`, independent IconAndText records, and exact `(questID, setType)` associations. A set contains layout direction, vertical padding, and an ordered vector of explicit widget IDs, types, and optional unit tokens. Numeric keys preserve floating-point bits except equal signed zeroes; availability is not inferred from ID ranges.

Queries project fresh Lua tables from supplied state: unknown set metadata and IconAndText records return nil; unknown membership returns an empty table. IconAndText projects the complete supplied record, including tooltip, texture kits, timer, ordering, animation, scale, layout, and model-layer fields, without automatic field fillers. Existing complete ButtonHeader and Prey records remain separate backing inputs; core feeds support IconAndText type0, ButtonHeader type30, and feature-gated PreyHuntProgress type31. New feed integration adds `WidgetVisualization` StatusBar type2 and CaptureBar type1. This is not coverage of every visualization type.

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

The [contract proof ledger](../../specs/ui-widget-manager-state.md#tests-asserting-this-spec) retains historical query, cached-container, security, deferred-layout, and unit-notification failures. Prior core proof remains separate: two actual lifecycle cases PASS at `e887`, plus 23 unchanged cases reused from `67775404d`; not a fresh 25-case run and not proof of the new bars/HUD slice.

### Bars, HUD, and feed implementation scope

- `bars.rs` supplies full 35-field StatusBar and 22-field CaptureBar DTO snapshots, including a fresh ordered StatusBar partition array.
- `hud.rs` exposes four live environment-local `f64` selectors. Each query returns exactly one number; default zero is simulator model policy, not a proven native default.
- `queries::register` calls both modules. Feed integration supplies StatusBar type2 and CaptureBar type1; the actual bar-consumer adaptation uses host feeds rather than direct state mutation/manual events.
- The old PowerBar bootstrap is removed without fallback. Query5/core19 and normal-startup/default-check proofs are reused within their original source scopes, not rerun for later fixture corrections.

### Historical RED and current consumer boundary

The [eight-case RED ledger](../../specs/ui-widget-manager-state.md#next-slice-tests-and-proof-gaps) at `87fecfd31` separates two wrong-type bar snapshot failures, three TopCenter exactly-one-return failures, one supplied-record query failure after a cached closure, and two numeric-query failures before cached HUD OnLoad. Later bar lifecycle and HUD consumer assertions were not reached. Earlier four bwrap missing-target launch failures executed zero cases and remain separate.

Later `39ce8969a` and `174b86505` each retain three consumer failures. At `b28a45f91`, HUD2 pass actual cached OnLoad; bar assertion59 fails initial neutral-glow visibility. Subsequent incremental bar updates, hidden/restore, membership removal and unregister remain unreached. With the explicit [atlas application-result policy](../../specs/texture-atlas-application-result.md), later `5bd208dba` actual bar1/HUD2 pass, including incremental updates, zero-zone geometry, hidden/restore, removal and unregister. The linked [packet](../../../data/patch-api/evidence/ui-widget-manager-state-2026-10-13/README.md#atlas-application-result-red-and-retail-green) owns exact current receipts; preceding failures remain historical. Later actual PTR consumers pass separately at `1804d1c49`; the [PTR and owning checkpoint](../investigations/integrated-source-and-factory-proof-2026-10-09.md#separate-ptr-and-owning-checkpoints--2026-10-13) owns exact runtime and reuse boundaries. Native return parity, other-profile and full-parent acceptance remain unestablished.

No native catalog, server transport, interpolation parity, dynamic HUD rebinding or unit-context proof is established. Query freshness does not prove rebinding of loaded containers. Consumer and broader parent acceptance remain OPEN.

### DiscreteProgressSteps producer checkpoint — 2026-10-13

[Discrete contract](../../specs/ui-widget-discrete-progress-steps.md) owns current producer and exact proof ledger. Historical42 input stage physically fails five cases on missing supplied record before cache/atlas. Current source registers complete fresh snapshots and type19 host-feed storage; query/security direct inputs and actual cached consumer typed feeds remain distinct. New feed tests observe committed state inside real callbacks. [Current retained proof](../../../data/patch-api/evidence/ui-widget-discrete-progress-2026-10-13/README.md#retail-and-ptr-current-proof) separates actual Retail/PTR seven-case Discrete GREEN, five preservation cases per profile, fresh normal startup and default check. Source/root compatibility policy is not catalog, rendered/native/tutorial or full-parent acceptance; bounded GREEN text-retention audit PASS at `a6723200be40145ec352666b538998bf4035a608`, scoped by current packet; original RED and other packets not rechecked.

### DiscreteProgressSteps input-only checkpoint — 2026-10-13

[Discrete contract](../../specs/ui-widget-discrete-progress-steps.md) owns the complete19-field supplied-record/type19 slice. Historical `42f206f66` stage added DTO, empty keyed map and five direct-seeded query/security/actual-cache cases only. Getter, registration and typed feed were absent at that stage; initial compilation/RED were unrun. Later genuine RED and producer source are recorded above. Initial supplied-record assertions precede cache/atlas prerequisites; downstream lifecycle assertions are not evidence until reached. Existing `eyeofthejailer` atlas metadata grounds fixture names, not asset extraction/render/native parity. No vendor or demo catalog changes.

### TextWithState input-only checkpoint — 2026-10-13

Historical input-only stage `64a2a57b2` is superseded by producer source `2bb6cdde1`: [`text_state.rs`](../../../src/c_api/ui_widget_manager/text_state.rs) projects fresh complete21-field snapshots without input clamping; core queries publish through the Retail/PTR active-profile constant gate. The [contract proof ledger](../../specs/ui-widget-text-with-state.md#tests-asserting-this-spec) owns genuine supplied-record RED and unreached downstream boundaries. GREEN and parent acceptance remain pending.

## Sources

- [UIWidgetManager contract](../../specs/ui-widget-manager-state.md) — required behavior, staged tests, and historical RED boundary.
- [Manager module](../../../src/c_api/ui_widget_manager.rs) and [inputs](../../../src/c_api/ui_widget_manager/inputs.rs) — numeric identities, set/membership records, and complete supplied DTOs.
- [Queries](../../../src/c_api/ui_widget_manager/queries.rs), [feed](../../../src/c_api/ui_widget_manager/feed.rs), and [fixtures](../../../src/c_api/ui_widget_manager/fixtures.rs) — snapshot registration, synchronous producers, and bounded demo initialization.
- `src/c_api/ui_widget_manager/bars.rs` and `src/c_api/ui_widget_manager/hud.rs` — implemented producer paths; exact query/consumer proof boundaries remain separate.
- [Task quest lookup](../../../src/lua_api/globals/quest_surface/task_quest.rs) — stored quest/set association lookup.
- [Widget architecture](../../widget-system.md), [loading pipeline](../../addon-loading-pipeline.md), and [events](../../event-system.md) — existing frame, cached Lua consumer, and dispatch boundaries.

## See Also

- [[widget-system]] — ordinary frames are distinct from C_API records.
- [[addon-loading]] — cached Blizzard consumer loading.
- [[event-system]] — synchronous host notification boundary.
- [[lua-api]] — exposed API inventory.
