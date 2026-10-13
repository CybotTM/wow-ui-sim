# TextWithState bounded source gate

Date: 2026-10-13. Canonical checkout: `<REPO>`.

**Outcome: source wiring/data/format PASS; spec consistency FAIL (one concrete wording contradiction). Readability threshold findings below. No behavioral acceptance.**

Rust/source scope: `bb5b7e1c7..64a2a57b2`, 11 files: 10 Rust files and one spec. Additional doc scope: the three wiki pointer additions in `7a8e6cfd7`. `proof.json` retains exact argv, cwd, revision/scope, exit codes, complete stdout/stderr, revision source snapshots, cached declarations/templates, generated macro contents and read-only audit results. No output is replaced by a summary in that ledger.

## Proof matrix

| Boundary | Result | Evidence / limit |
|---|---|---|
| EXIST / SUBSTANTIVE | PASS | All 11 scoped files obtained with `git show 64a2a57b2:<path>`, exit 0. DTO has 21 explicit fields; no default DTO or placeholder producer. Scoped working-source diff against `64a2a57b2` was empty. |
| Passive input wiring | PASS, source only | `ui_widget_manager.rs:14-16` declares/exports DTO. `inputs.rs:53` adds `HashMap<WidgetId, TextWithStateRecord>`. `feed.rs:15,28,164-165` accepts type8 and inserts the complete record. `update_ui_widget` validates the type, requires the set, upserts membership/inserts visualization before notifying; removal only removes membership. Environment state owns the inputs; demo seeding starts from `UiWidgetInputs::default()` and never fills this map. |
| Getter | INTENTIONALLY ABSENT | `queries.rs::register` adds no TextWithState projection/registration. This is the authorized input stage, not an orphaned implementation finding. Compatibility getter publication/result behavior was not executed. |
| Integration discovery | PASS, source/artifact only | `Cargo.toml` has `autotests=false` and explicit `integration` target. `tests/integration.rs:1` invokes `integration_tests!()`. `crates/test-harness/build.rs:29-65,441-482` discovers top-level Rust modules; this file is not excluded. Generated `target/debug/build/wow-ui-sim-test-harness-a0dc779095698bb1/out/integration_tests.rs:6215-6216` includes the actual canonical TextWithState test module. The other existing harness artifact lacks it; neither artifact's compilation/revision is certified. No standalone TextWithState Cargo target is asserted. |
| Staged cases | PASS, inventory only | Seven concrete `#[test]` declarations: queries3, feed2, consumer1, security1. Explicit child-module paths and shared `discrete_fixtures::assert_lua` resolve. Top-level gate is Retail/PTR; security follows `retail-12-1-0`, enabled by both current profile bundles. No test discovery/run was executed. |
| DTO/data | PASS, declarations only | All 21 Rust fields map in order to both cached declarations and the 21 expected Lua fixture keys. Types preserve strings, enum integers, f64 numeric fields and bool; every cached field is non-nil. Both getter declarations specify `AllowedWhenUntainted`. Fixtures are supplied policy, not native records. |
| Consumer staging | PASS, source only | Uses existing isolated Game environment/closure loader, real container constructor, host updates and real OnUpdate before count checks. Initial supplied getter assertion precedes addon loading; security initial getter assertion precedes secret-wrapper setup. Feed observer checks a supplied table during notification and immediately checks callback success. Rejected feed case is not claimed as missing-getter RED. |
| Anti-pattern scan | PASS | Zero newly added TODO/FIXME/HACK/XXX or `#[allow(...)]` markers. Existing generated harness suppressions are outside this diff. |
| Direct formatting | PASS | `rustfmt --check --edition 2024 --config skip_children=true <all 10 changed Rust paths>`: exit 0, empty stdout/stderr. Full argv in ledger. |
| Project formatting | PASS | `cargo fmt --check`: run once, exit 0, empty stdout/stderr. Wiki-only addition does not invalidate Rust formatting proof; no rerun. |
| Added wiki pointers | PASS | All five new relative links resolve; both anchor links match `### TextWithState input-only checkpoint — 2026-10-13` and slug `textwithstate-input-only-checkpoint--2026-10-13`. Index/log/system wording remains input-only/unverified and references `64a2a57b2`, not completed producer/runtime acceptance. |

## Applicable findings

**[SPEC] `docs/specs/ui-widget-text-with-state.md:14`**

Problem: sentence says negative padding/order values “never enter the consumer fixture.” Counterexample: `tests/widget_text_state/consumer.rs::update_hide_restore` sets `record.bottom_padding = -3.75`, supplies that record to the consumer, and checks geometry with `-3.75`. Negative padding intentionally enters the staged consumer fixture.

Suggested correction: restrict query-only wording to negative order and other query-only discriminators; describe negative consumer padding as a clamp test. No source/test change requested or made.

**[LENGTH] `tests/widget_text_state/consumer.rs:33` — `register_consumer()`**

56 body lines, exceeding the skill's literal 30-line non-test-function threshold. Registration/fixture-definition and first activation are separate named phases available for a small extraction. This is a test helper, mostly embedded Lua; not evidence of a behavioral failure.

**[LENGTH] `tests/widget_text_state/consumer.rs:92` — `update_hide_restore()`**

49 body lines. Update, hide and restore are explicit sequential phases. Small phase extraction is possible if applying the non-test-helper threshold. Not a runtime blocker; no redesign recommended.

**[LENGTH] `tests/widget_text_state/fixtures.rs:52` — `assert_initial_snapshot()`**

35 body lines, primarily one Lua fixture/assertion literal. Threshold exceeded, but splitting a single coherent literal has limited practical value; defer rather than invent an abstraction. No behavioral counterexample.

`rust-code-analysis-cli` is unavailable on PATH. Complexity/nesting audit was manual, not a metrics pass. No additional applicable complexity, warning-suppression, side-effect or parameter-overload finding identified in changed Rust. `#[test]` bodies stay below the 200-line test threshold.

## Rejected / unproven claims

The prior negative-padding finding is rejected: actual expectation is `textHeight + Clamp(padding, 0, textHeight - 1)`, matching both cached templates' `Setup`. The spec contradiction above does **not** revive that false test finding.

Getter absence is authorized. Rust formatting cannot establish compilation. Source discovery and existing generated macro cannot establish executed tests. Cached schema/template reads cannot establish native admissibility or real consumer acceptance.

Behavioral RED, GREEN, builds, cargo check, runtime, hashes, operations, delegation and parent acceptance were not run. No canonical source/config/vendor/cache edits, commits, deployment or fixes. Only requested `verification/text-state-input-source/report.md` and `proof.json` written. Parent acceptance remains OPEN.
