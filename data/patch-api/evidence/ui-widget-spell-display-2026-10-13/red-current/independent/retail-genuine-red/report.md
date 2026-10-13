# Saved actual Retail SpellDisplay RED

**Verification PASS — saved genuine RED observed. Behavioral FAIL/RED; not GREEN or acceptance.**

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-red-20261013T181246353729Z`
Recorded source: `85b5494d2cc60086b3eaf94793d585eb8cf7fd7f` (label, not new source authentication).

## Runtime

Complete runtime `stdout`, `stderr`, `argv.json`, `launcher.txt`, `result.json`, `admission.json` read. **9 cases, 0 PASS, 9 FAIL; 0 ignored, 0 measured, 10984 filtered out; 1.55s**. Saved `systemd-run --wait` exit **101**, main status101, exit-code result; service runtime1.622s. Not timeout/admission denial.

All nine panics at `<REPO>/tests/widget_discrete/fixtures.rs:42:33`:

> discrete [SpellDisplay supplied table boundary]: Lua error: missing supplied SpellDisplay record

That location is the shared panic wrapper; saved SpellDisplay report locates result assertion at `tests/widget_spell_display/fixtures.rs:64–74`. Diagnostic does **not** establish exact nil, return arity or serializer semantics. Unit `wow-speak-retail-green-runtime-20261013t182110471661z` contains `green`; actual failure outcome is authoritative.

## Nine seeded-record/result gates

Saved source basis: `<PRIVATE_VERIFICATION_ROOT>/spell-display-input-source-current/report.md` and sibling `source-inventory.txt` read in full, recording the same revision. Previously saved source-stage report and source-inventory.txt recording 85b5494d2; not fresh source/archive/tree authentication. Report says host record seeded before assert_initial_snapshot at fixtures.rs:64–74; callable guard then initial result-table assertion before expected-snapshot construction. Actual matching logs reach that result boundary. Consumer helper prepares cache path/base environment first, but result gate line15 precedes supplied set17, templates18, cached addon load19 and container/registration. Feed gates precede set/event observer; security gate precedes VM registration16 and host secret construction21.

| Case | Observed gate | Saved source call site | Later assertions UNREACHED |
|---|---|---|---|
| consumer::spell_display_attached_player_context_separate_from_world_loot_policy | FAIL, supplied-record result gate | tests/widget_spell_display/consumer.rs:191–192 → helper:15 | attached context/cached load |
| consumer::spell_display_cached_activation_update_hidden_restore_remove | FAIL, supplied-record result gate | tests/widget_spell_display/consumer.rs:106–107 → helper:15 | lifecycle/cache/cached load |
| consumer::spell_display_zero_spell_polling_uses_real_on_update_without_feed_notification | FAIL, supplied-record result gate | tests/widget_spell_display/consumer.rs:209–210 → helper:15 | OnUpdate/no-feed-notification/cached load |
| feed::spell_display_feed_commits_before_events_retains_visualization_on_membership_removal | FAIL, supplied-record result gate | tests/widget_spell_display/feed.rs:46–51 | set/event observer and commit/removal |
| feed::spell_display_feed_rejects_type_mismatch_unknown_set_without_mutation_or_notification | FAIL, supplied-record result gate | tests/widget_spell_display/feed.rs:114–118 | set/event observer and rejection/no-mutation |
| queries::spell_display_complete_nested_fresh_replace_delete | FAIL, supplied-record result gate | tests/widget_spell_display/queries.rs:7–11 | complete snapshot/freshness/replace/delete |
| queries::spell_display_same_id_environment_isolation | FAIL, supplied-record result gate | tests/widget_spell_display/queries.rs:103–106 | second environment/isolation |
| queries::spell_display_strict_numeric_ids_without_integer_range_inference | FAIL, supplied-record result gate | tests/widget_spell_display/queries.rs:73–76 | special IDs/numeric rejection probes |
| security::spell_display_real_secret_ids_allow_untainted_deny_tainted_recover | FAIL, supplied-record result gate | tests/widget_spell_display/security.rs:9–21 | table-security registration/secret construction/taint recovery |

Ordering uses saved source assertions plus actual matching diagnostics, not newly authenticated source. Consumer cache-path/base-environment preparation occurs before gate, **cached addon construction/load does not**. Later consumer/cache/numeric/security/feed assertions remain **UNREACHED**. Common initial failure does not prove nine independent downstream defects.

## Compilation — separate setup proof

Complete compile.stdout parsed: **741 JSON records**: 665 compiler-artifact, 75 build-script-executed, 1 build-finished success=true; **0 compiler-message records**. Full stderr: **six distinct manifest deprecations**; aggregate summary not seventh warning. Test profile finished4m49s. Terminal `SubState=exited`, `Result=success`, `ExecMainStatus=0`.

```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim v0.1.0 (<REPO>)
   Compiling wow-ui-sim-test-harness v0.1.0 (<REPO>/crates/test-harness)
    Finished `test` profile [optimized + debuginfo] target(s) in 4m 49s
```

No-run compile success is not behavioral RED/GREEN. Build source archive/tree **NOT AUTHENTICATED**, neither read nor hashed. External libraries/cache/settings unauthenticated.

## Setup/admission/authentication distinctions

- Retail compile admission allowed; saved compile launcher0, terminal0. Retail runtime admission allowed, 4GiB max/3GiB high; actual result failure101 follows admission/setup.
- PTR `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-ptr-red-20261013T181858205721Z` denied admission (`allowed=false`); only admission.json exists, **0 launches**. Not Retail failure or observed PTR RED.
- Existing receipt/seal **stat-only**, contents not read. Copied executable metadata matches saved runtime receipt_stat_reused in all four fields; this proves metadata continuity, not new cryptographic authentication. No hash/reauth.
- Saved auth-report.md claims prior executable authentication only. Its pre-runtime RED UNEXECUTED statement is superseded by actual runtime logs, not altered. Source archive/tree remains unauthenticated.

## Private outputs and restrictions

No runtime/build/reauth/hash/delegation, source archive/tree reads, tracked edits or producer changes. Main owns ops. Only these absolute private files written (directory0700, files0600), then complete content readback compared:

- `<PRIVATE_VERIFICATION_ROOT>/spell-display-retail-genuine-red-current/report.md`
- `<PRIVATE_VERIFICATION_ROOT>/spell-display-retail-genuine-red-current/proof.json`

Proof contains each case, full compile diagnostics, admission distinctions and stat-only metadata. Initial generation failed `NameError: collections` before writes; explicit import fixed report generation only. No operational action followed.
