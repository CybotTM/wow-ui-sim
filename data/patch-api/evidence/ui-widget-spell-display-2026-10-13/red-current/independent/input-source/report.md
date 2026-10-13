# SpellDisplay passive input/test/spec source verification

Date: 2026-10-13
Canonical repository: <REPO>
Revision: 85b5494d2cc60086b3eaf94793d585eb8cf7fd7f
Comparison: a97e6b476..85b5494d2
Private proof directory: <PRIVATE_VERIFICATION_ROOT>/spell-display-input-source-current/

## Verdict

**PASS — bounded passive-input/test/spec source stage and permitted formatting/whitespace checks.**
**Readability: one helper naming/side-effect finding, below.** No compiler, test execution, genuine RED/GREEN, runtime, native, rendered, or parent acceptance is established. Nine tests are source-wired, not nine passing tests. No getter/query serializer producer exists in this change; intended initial result failure is not an observed execution failure.

Initial git status contains only untracked `.code-index.db` and `verification/`; no tracked modifications reported. These pre-existing paths were untouched. Eleven changed paths match the supplied inventory, all present and substantive. Exact sizes/line counts and zero TODO/FIXME/HACK/XXX markers per path are in source-inventory.txt. No new suppression or empty stub bodies in changed Rust. Passive DTO declarations are intentional, not fake implementations.

## Source contract coverage

| Boundary | Evidence | Proof level |
|---|---|---|
| Complete passive DTOs | src/c_api/ui_widget_manager/spell_display_record.rs:4–21 nested16, :24–39 top14. Strings/bools/numbers/enums match cached fields; no Default DTO or retained Lua Val | Source PASS |
| Empty environment-local SpellDisplay map | inputs.rs:47–61 derives Default; :54 HashMap field. lua_api/state/sim_state.rs:160 owns UiWidgetInputs; lua_api/state.rs:314 uses seed_demo_world_quest_widgets; fixtures.rs:6–22 starts default and seeds only other maps | Source PASS; no runtime isolation claim |
| Type13 feed and ordering | feed.rs:17 variant, :31 type13, :65 validation, :68–76 set lookup/upsert/supply before :78 event; :167–169 map insertion. Existing synchronous notification route reused. Validation precedes mutation; missing set returns before upsert. Membership removal :82–102 does not remove visualization | Source PASS; event behavior unexecuted |
| Passive module wiring / no query producer | ui_widget_manager.rs:14/18 module and public DTO exports. src SpellDisplay search finds declarations/feed/map only, no GetSpellDisplayVisualizationInfo implementation/registration or serializer. Range diff adds no such producer | Source PASS for absence of new producer, not absence of existing generic compatibility surface |
| Generated integration wiring | tests/integration.rs:1 invokes macro; crates/test-harness/src/lib.rs:7 includes generated macro. crates/test-harness/build.rs:29–66 enumerates top-level tests, generates module entries; :441–449 discovers .rs; :462–476 excludes other named files, not ui_widget_manager_spell_display.rs. New root module :2 gates Retail/PTR, :3–14 reaches all five children and shared discrete fixtures | Generator-source PASS; no old generated artifact inspected or fresh compilation claimed |

## Actual primary cached contracts

Read actual files for BOTH profiles under `<HOME>/.cache/wow-ui-sim/blizzard-ui/{retail,ptr}/AddOns/`:

- Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua:353–366 declares numeric nonnil widgetID, `SecretArguments = "AllowedWhenUntainted"`, one nullable SpellDisplayVisualizationInfo return. :1746–1765 declares exactly14 top-level fields. :2138–2160 declares UIWidgetSpellInfo exactly16 nested fields, all nonnil. No RGBA objects; borderColor/tint are enum fields. Rust nested name WidgetSpellInfoRecord is a host DTO, not the literal documented structure name.
- Blizzard_APIDocumentationGenerated/UIWidgetManagerSharedDocumentation.lua:148–149 declares SpellDisplay13 / DoubleStateIconRow14 in each profile. Fixture's “BOTH cached Retail/PTR declarations” refers to profiles; do not reinterpret it as requiring the same enum in both Documentation and SharedDocumentation files.
- ManagerDocumentation :1030 None animation0 and :1086 Large icon2. Blizzard_UIWidgets/Blizzard_UIWidgetTemplateBase.lua:409 maps Large to36, :589–590 sets icon size from that map. :229–233 GetOutAnim returns animation only for Fade; :257–267 AnimOut otherwise invokes completion. This grounds fixture out_anim_type0 and icon_size_type2→36 only; no universal immediate-removal/animation assertion.
- Blizzard_UIWidgets/Blizzard_UIWidgetTemplateSpellDisplay.lua:1–9 cached getter adapter/registration, :30–43 actual nested Setup/geometry, :99–112 polling. Actual base Setup :505–562 consumes text, stacks, earned mark and dimensions; :644 onward uses tint enum. These are read-only vendor references, not simulator producer implementations.

Line-numbered excerpts retained in cached-contracts.txt and nested-contracts.txt. No cached files changed. Local primary sources satisfied this source audit; no external research or runtime proof used.

## All nine first supplied-record boundaries

fixtures.rs:64–107 asserts getter function then first result table at :70–74 BEFORE constructing expected snapshot/assertion helper. Complete recursive snapshot checks top14+nested16, rejects extras and preserves explicit false. Initial host record is seeded before assertion.

| Test file / case | First result gate precedes |
|---|---|
| queries.rs:7 freshness/replace/delete | :11 before copy/mutation/replacement/deletion |
| queries.rs:73 numeric IDs | :76 before special IDs and reject probes |
| queries.rs:103 environment isolation | :106 before second environment creation/query |
| feed.rs:46 committed event/removal | :49 before set/event observer at :50–51 |
| feed.rs:114 rejected feed | :117 before set/observer and invalid feed calls |
| security.rs:9 real secret policy | :12 before VM security registration at :16 and host secret values at :21 |
| consumer.rs:106 lifecycle | :107 calls consumer_env(false); helper :15 result gate before :17 supplied set, :18 intrinsic templates, :19 cached addon load, container and registration |
| consumer.rs:191 attached context | :192 same helper ordering with true |
| consumer.rs:209 zero polling | :210 same helper ordering with false |

Consumer helper resolves cache path and creates base environment before gate; this is required environment preparation, not loading cached widget construction. tests/common/blizzard_addon_closure.rs:74–89 new environment helper only constructs environment, sets screen and addon base paths. No cached addon closure loaded there. Failure could still occur during preparation; source ordering cannot guarantee compiler/environment success or prove actual RED. Later assertions, callbacks, pool state, secret behavior, and enum checks remain unexecuted.

Consumer tests use typed host feed, real template registration/events/OnUpdate, no direct Setup or provider/pool/vendor overrides. Attached-player context and no-event zero-spell polling are separately asserted. World-loot and animation generalization excluded. Spec explicitly marks requirements incomplete and records nine tests unexecuted.

## Changed Rust readability audit

Manual changed-line audit of ten Rust files; no extra metrics commands run under the user's command restriction. Test code (including helpers) assessed against test-size allowance; explicit DTO literals and exact-field snapshot fixture retained as single coherent contracts rather than split solely on data-line count. No observed >200-line test body, deep Rust nesting, complex changed Rust boolean expressions, five primitive parameters, warning suppression, duplicated production functions, or mutating iterator anti-pattern.

[SIDE_EFFECT] tests/widget_spell_display/consumer.rs:11 — consumer_env()
Problem: neutral environment noun hides cached addon loading, synchronous set notification, Lua container creation/registration, and OnUpdate dispatch.
Suggestion: use an effect-revealing name such as load_and_register_spell_container_env; preserve first result-gate ordering. No edit authorized or performed. This readability finding does not invalidate the bounded source ordering/contract checks and is not a runtime failure.

## Commands and full diagnostics

Exact argv, cwd, returncodes and COMPLETE stdout/stderr: commands.json and proof.txt. Separate captures command-1.json (range diff), command-2.json (rustfmt), command-3.json (whitespace). Initial three git observations reconstructed from their already-read tool output, not rerun.

1. `git rev-parse HEAD` — exit0, requested HEAD confirmed.
2. `git status --short` — exit0, two pre-existing untracked paths above.
3. `git diff --name-status a97e6b476..85b5494d2` — exit0, exactly11 paths.
4. `git diff a97e6b476..85b5494d2 -- src/c_api/ui_widget_manager.rs src/c_api/ui_widget_manager/feed.rs src/c_api/ui_widget_manager/inputs.rs` — exit0; complete diff in proof.txt.
5. Scoped rustfmt command below — exit0; stdout empty, stderr empty, no diagnostics.
6. `git diff --check a97e6b476..85b5494d2` — exit0; stdout empty, stderr empty, no diagnostics.

```text
rustfmt --check --edition 2024 --config skip_children=true src/c_api/ui_widget_manager.rs src/c_api/ui_widget_manager/feed.rs src/c_api/ui_widget_manager/inputs.rs src/c_api/ui_widget_manager/spell_display_record.rs tests/ui_widget_manager_spell_display.rs tests/widget_spell_display/consumer.rs tests/widget_spell_display/feed.rs tests/widget_spell_display/fixtures.rs tests/widget_spell_display/queries.rs tests/widget_spell_display/security.rs
```

Incidental discovery/Pyrun errors and recovery are retained without suppression in tool-diagnostics.txt. Permitted acceptance checks each ran once. No Cargo build/check/test, runtime invocation, old artifact/hash work, delegation, commits, tracked writes or production mutation. Only absolute private proof/report files written. Report and all proof files read back before handoff.
