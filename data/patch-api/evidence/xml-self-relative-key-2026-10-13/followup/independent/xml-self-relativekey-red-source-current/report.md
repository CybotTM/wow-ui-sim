# Bounded source/spec/readability verification

Revision: 8175014bcbc5ade058f6b80a675959704c58492d. Scoped files match committed revision (git diff and scoped git status empty).

**Verdict: source coverage established; one script-prerequisite gap. Compile/runtime/GENUINE RED NOT VERIFIED.** Parent-owned Mists no-run admission and integration runtime remain pending; no producer implementation was assessed or changed.

## Artifact checks

[EXIST] PASS — tests/xml_self_relative_key.rs: 181 lines; docs/specs/xml-self-relative-key.md: 41 lines. Both read completely.
[SUBSTANTIVE] PASS — two behavioral tests, parsed cached AST registration, real Lua dependency execution, exact identities/points/offsets and complete accumulator assertions; no mock or stripped AST.
[WIRED] PASS (source only) — Cargo.toml:9 disables Cargo autotests; Cargo.toml:178-180 declares integration. tests/integration.rs:1 invokes integration_tests!; crates/test-harness/build.rs:29-66 emits discovered modules; :441-477 includes this top-level file without exclusion. File-level client-mists cfg gates both cases. Separate Cargo target claim is wrong: these are modules in primary --test integration. Source contains two tests matching xml_self_relative_key; no runtime selected-count claim.
[ANTI-PATTERN] PASS — TODO/FIXME/HACK/XXX: 0 each in scoped files; no suppressions, empty error handling, or commented-out code observed.

## Literal-policy coverage matrix

| Requirement | Source evidence | Proof level |
|---|---|---|
| Full cached item-display AST with dependencies | test:7-50 parses XML, clones complete named frame/texture definitions; :57-79 executes complete ModelSceneMixin.lua and registers two ModelScene templates plus five exact texture dependencies | Source only |
| Normal CreateFrame inheritance/layers/finalization | test:95-103 calls real CreateFrame with ProductChoiceItemDisplayTemplate; runtime_loader_effects.rs:12-23 enters production xml_layer_batch | Source only |
| Icon self-relativeKey becomes parent card | test:111-117 asserts distinct identity, parent key, 63x63 size and one CENTER point targeting card at (0,15); cached ProductChoice.xml:66-69 authors disputed self reference | Assertion exists; expected failure not observed |
| Later IconBorder targets sibling and subtrees complete | test:118-129 asserts sibling target at (0,-5), decorations, ModelScene OnLoad state, PreviewButton inherited texture and Covers children; :133-137 requires entire error accumulator empty | Source only |
| Direct Lua atomic self rejection and distinct success | test:148-164 snapshots both points and verifies every tuple after pcall rejection; :165-177 checks exact replacement and untouched second point | Source only |

## In-scope setup finding

**Script prerequisite gap — tests/xml_self_relative_key.rs:57-79, 125-129.** Spec line 7 explicitly requires preserving scripts. Cached ProductChoice.xml:127-128 references ProductChoiceItemDisplay_OnMouseDown and ProductChoiceItemDisplay_OnMouseUp; their real definitions are cached Classic/ProductChoice.lua:257-263. Test loads only ModelSceneMixin.lua, not ProductChoice.lua, and does not assert those callbacks' presence/identity. WowLuaEnv::new (src/lua_api/env.rs:75-104) initializes builtins/runtime rather than loading ProductChoice addon sources; source search found no simulator definitions for these two globals. Actual fast binding (template_chain/builder_function_family/plain.rs:20-25) installs build_missing_global_handler when a named function is nil, so an empty accumulator at creation does not prove real ProductChoice callbacks survived. This is a source-established dependency/coverage gap, NOT evidence of failure before Icon: handlers are mouse callbacks, not mandatory creation-time OnLoad calls. Parent should load the real cached ProductChoice Lua dependency and assert real script binding if retaining the literal script-preservation claim; do not mock it, strip AST, alter vendor code, or change producer scope.

ModelSceneMixin.lua was read completely (565 lines). Its top-level extra dependencies are CreateFromMixins calls, already supplied by shared_bootstrap.lua:88-94. ModelScene OnLoad:5-13 initializes cameras/tagToActor/tagToCamera and only calls SetAllowOverlappedModels when explicitly requested; these two registered templates do not request that flag. No CameraRegistry/actor templates need adding merely because unused methods reference them. PreviewButton's click-only ModelPreviewFrame_ShowModel dependency is not executed at creation; no unrelated interaction requirements added.

## Rust readability

[SIDE_EFFECT] tests/xml_self_relative_key.rs:7 — parse_cached_xml()
Problem: name describes parsing but helper reads filesystem via parse_xml_file.
Suggestion: read_and_parse_cached_xml makes I/O explicit. Low priority; no behavioral blocker.

No other Rust readability violations found manually: helpers <=30 body lines, tests <200, nesting bounded, descriptive names, contextual read errors, no lint suppressions. Lua assertion conjunctions encode exact point tuples, not excessive Rust branch complexity. No formatter was necessary or run.

## Proof ledger / exclusions

Read-only git commands through cli.git(...).cwd('<REPO>'): show scoped commit; rev-parse HEAD; diff 8175014bc -- scoped files; status --short -- scoped files. All returned without stderr; head exact above and scoped diff/status empty. Direct file reads/searches provide cited source evidence. Marker/line counts computed from current scoped files.

No cargo build/check/test/fmt, runtime execution, XML producer edits, tracked edits, hashing/archive/auth operations, delegation, Bash, or cwd switching. No genuine RED, compile success, selected runtime test counts, or source-binary equivalence claimed. Earlier separate-target stage-report statement is superseded by inspected integration discovery. Parent owns validation and changes.
