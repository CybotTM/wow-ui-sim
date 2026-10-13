# DiscreteProgressSteps bounded source gate

Scope: commit `42f206f66`, `<REPO>`, input/test-only stage. All eight inspected Rust files matched their committed text. No implementation edits, runtime execution, hashing, compilation, delegation, deployment, or other operations performed. Verification and rust-readability skills read; readability audited manually, without running metric tools.

**Overall: SOURCE-ONLY / NOT GREEN. Behavioral proof: UNRUN.** Neither executed RED nor GREEN nor compilation is established here. Direct formatting and source inspection cannot establish native parity or successful cached consumption.

## Command proof

- Direct `rustfmt --check --edition 2024 --config skip_children=true` with exactly the eight changed Rust files: **exit 0**, empty stdout/stderr. `skip_children=true` prevents recursive formatting checks outside the explicit list.
- `git diff --check 42f206f66^ 42f206f66`: **exit 0**, empty stdout/stderr.
- Bounded added-link checks: **8/8 PASS**, including both new checkpoint anchors. Existing unrelated links excluded.

Full command output: `commands.json`. Link results: `doc-links.json`. File existence, line counts, commit-text comparisons, and marker counts: `source-scope.json`. No TODO/FIXME/HACK/XXX or allow attributes in these eight files.

## Coverage matrix

| Requirement | Source evidence | Proof level / gap |
|---|---|---|
| Complete supplied DTO, map, public export | `discrete_record.rs:4-24`: 19 fields, seven f64, seven u32 enums, four Strings, one bool; no DTO Default. `inputs.rs:50`: keyed map. `ui_widget_manager.rs:5-6`: module/export. | SOURCE inspected; Lua projection absent. |
| Cached schema / visualization19 / catalog kit | Retail and PTR cached `UIWidgetManagerDocumentation.lua:1352-1376` list identical 19 nonnil fields. Cached shared documentation assigns type19. `data/atlas.rs:5498-5507,5958-5959` contains all ten enabled/disabled step atlases, bar128x128 and rune40x40. | SOURCE metadata only; native/render/extraction unknown. |
| Test discovery and five initial boundaries | `tests/integration.rs:1` invokes generator macro. `crates/test-harness/build.rs:29-65,441-477` discovers top-level .rs and does not exclude this module; `:124-177` follows explicit child path attributes. `tests/ui_widget_manager_discrete.rs:4-12` declares all four children. | Generated local artifact contains parent path/module at lines6191-6192; not compiled-test-list proof. Security requires retail-12-1-0; parent requires Retail/PTR. |
| Snapshot / numbers / environment ownership | Three query cases at `queries.rs:6,82,121`. Each seeds a complete record then calls `assert_initial_snapshot` before subsequent work. `fixtures.rs:49-83` asserts function, supplied table, every field type/value, and rejects extra fields. Later assertions cover fresh copies, retained snapshots, replacement/removal, empty independent state, fractional/negative/large/signed-zero IDs, non-number rejection. | Intended behavioral assertions; UNRUN. |
| Genuine host-secret path | `security.rs:7-21` creates actual host secret NUMBER wrappers, pushes before global publication, then pops. Initial ordinary snapshot at `:27` precedes security helper installation. Closure taint stamped at `:63`, caller/denial/public recovery asserted thereafter. | SOURCE path inspected; security behavior UNRUN. |
| Actual cached consumer and lifecycle | `consumer.rs:264-275`: initial getter/record boundary before membership/cache/atlas/XML prerequisites. Cache closure helper receives active profile cache and empty overrides at `:66-79`; no replacements of vendor behavior or atlas catalogs. Actual registered UPDATE_UI_WIDGET, UPDATE_ALL_UI_WIDGETS and OnUpdate paths used. | Intended consumer assertions; UNRUN and downstream boundary not reached by this gate. |
| Excluded producers remain absent | Commit only adds record/map/export and tests/docs. Existing `queries.rs:10-22` registers other supplied visualizations; `feed.rs:11-29,153-175` has no DiscreteProgressSteps variant/storage. No new getter implementation, registration, or host-feed producer. | Intentional stage boundary, not an implementation defect. |

Generated artifact inspected: `target/debug/build/wow-ui-sim-test-harness-a0dc779095698bb1/out/integration_tests.rs`. Its inclusion alone does not prove successful compilation, test discovery by a running executable, or execution of five cases.

## Security grounding

Read dependency source from `.cargo/git/checkouts/rilua-fd5a0715e46b5888/842e4d3/src/`, matching the declared dependency source path, without any hash gate. `table_security.rs:233-238` allocates secret userdata containing `Val::Num` and requires rooting before another GC safe point. Test push/global/pop sequence meets that source contract; existing `tests/ui_widget_manager_security.rs:66-81` uses the same sequence. `lib.rs:456-470` interns the global key and stores the wrapper in the globals table. `stdlib/taint.rs:148-161` stamps closure taint; `vm/execute.rs:645-710` installs it in the call frame. `table_security.rs:401-427` checks secure/non-revoked context before returning a secret payload, with the asserted `untainted caller` message. Existing `ui_widget_manager/queries.rs:25-31` unwraps before accepting a numeric ID. This grounds intended assertions, not a claim that the absent discrete producer already applies that helper.

Event payload returned by eval is immediately passed to `WowLuaEnv::fire_event_with_args`; `env_events.rs:245-259` pushes arguments before dispatch allocations and restores the stack afterward. No intervening Lua allocation in the new test helper was identified. The eval expect-message is not independent rooting evidence.

## Cached consumer oracle

Cached `Blizzard_UIWidgetTemplateDiscreteProgressSteps.lua:124-193` clamps for consumption only, computes step geometry, applies kits, and filters hidden records through its adapter at lines3-10. XML supplies 140x140 outer/128x128 Steps and five marker arrays. Test oracle does not replace these functions:

- Range60/current40/steps4: two complete, three markers, first angle108, circular remaining0.5.
- Current52/steps6: four complete, five markers, adjusted53.8, circular remaining0.316, first angle84.
- Current16/steps2: zero complete, one marker, adjusted15.4, circular remaining0.828, first angle180.

These constants agree with cached adjustment/anchor arithmetic and `Blizzard_UIWidgetTemplateBase.lua:1436-1459`. Cached `Blizzard_FrameXMLUtil/Cooldown.lua:15-19` pauses, defaults to100 seconds, and sets start from remaining percentage; test checks milliseconds and paused state. Catalog-backed atlas names/dimensions are asserted on actual frame textures, not injected values.

Lifecycle assertions cover incremental same-frame update, hidden release and stopped animations, pooled restore with changed geometry, record removal, membership refresh/removal/restore, active pool counts, timer absence, unregister/event removal/manager-map removal, and no post-unregister reacquisition. Cached manager `:46-73,252-331` grounds event/layout/registration mechanics. Direct input seeding and dispatched events deliberately do NOT prove typed host-feed acceptance.

## Readability and scoped findings

Changed Rust inspected for line clarity, naming/effects, branching/nesting, state accumulation, warning suppression, duplication, parameter overload, and fixture/test responsibility boundaries. Supplied literals are concrete behavioral fixtures; oracle literals have arithmetic/source explanations. Existing unrelated helper implementations were not made readability repair targets. No in-scope blocking hypothesis was established; no fixes proposed or applied. Intentional producer absence, conditional fifth case, and unreached cached prerequisites are recorded gaps, not speculative hardening findings.

Docs explicitly preserve unrun RED/GREEN, absent producer, direct-seeding limitation, and unknown native/catalog/render parity. Broader compiler and behavioral acceptance belong to the caller's separate async work; its outcome was not inspected or adopted.
