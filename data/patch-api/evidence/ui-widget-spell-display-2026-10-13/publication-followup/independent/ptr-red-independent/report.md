# Saved actual PTR SpellDisplay RED

**Verification PASS: genuine saved RED. Behavioral FAIL/RED; not GREEN or acceptance.**

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-ptr-red-retry1-20261013T182259502151Z`
Recorded source `5ad0ecfcb0742a06e70f2ac053c05c90f4329064`; user classification docs/data over passive `85b5494d2`. Saved metadata, not fresh source authentication.

## Runtime evidence

Complete stdout/stderr/argv/result/admission/launcher read. **0 PASS, 9 FAIL**, 0 ignored, 0 measured, 10161 filtered out, **1.73s**. Saved launcher `--wait` exit101; main status101; exit-code result; service1.984s. Allowed runtime admission. Not timeout or admission failure. Unit name contains `green`; observed result, not label, governs.

All nine failures at `<REPO>/tests/widget_discrete/fixtures.rs:42:33` (shared panic wrapper):

> discrete [SpellDisplay supplied table boundary]: Lua error: missing supplied SpellDisplay record

Saved input-source report locates callable guard then supplied-record result-table gate at `tests/widget_spell_display/fixtures.rs:64–74`, before expected snapshot construction. Actual diagnostic matches that result gate. **Exact nil and return arity not established.** No claim about current getter implementation.

## Coverage and ordering

Basis: `<PRIVATE_VERIFICATION_ROOT>/spell-display-input-source-current/report.md` and `source-inventory.txt`, read completely. These record old passive candidate revision85b, not currently changing canonical source. User supplied5ad docs/data-over-passive relationship; neither source archive/tree authenticated.

| Case (prefix ui_widget_manager_spell_display::) | Saved call site under tests/widget_spell_display/ | Later behavior UNREACHED |
|---|---|---|
| consumer::spell_display_attached_player_context_separate_from_world_loot_policy | consumer.rs:191–192 → helper:15 | attached context/cached construction |
| consumer::spell_display_cached_activation_update_hidden_restore_remove | consumer.rs:106–107 → helper:15 | cached lifecycle |
| consumer::spell_display_zero_spell_polling_uses_real_on_update_without_feed_notification | consumer.rs:209–210 → helper:15 | OnUpdate/no-notification |
| feed::spell_display_feed_commits_before_events_retains_visualization_on_membership_removal | feed.rs:46–51 | set/event observers/commit/removal |
| feed::spell_display_feed_rejects_type_mismatch_unknown_set_without_mutation_or_notification | feed.rs:114–118 | event observers/rejection/no-mutation |
| queries::spell_display_complete_nested_fresh_replace_delete | queries.rs:7–11 | complete fields/freshness/replace/delete |
| queries::spell_display_same_id_environment_isolation | queries.rs:103–106 | second environment/isolation |
| queries::spell_display_strict_numeric_ids_without_integer_range_inference | queries.rs:73–76 | special IDs/rejection |
| security::spell_display_real_secret_ids_allow_untainted_deny_tainted_recover | security.rs:9–21 | VM security registration/host secrets/taint recovery |

Every case: observed FAIL at common supplied-record result boundary after callable guard. This is **one common gap, not nine proven downstream defects**. Host record seeded first according to saved source report. Consumer helper performs cache-path/base-environment preparation first, but gate:15 precedes supplied set:17, intrinsic templates:18, cached addon load:19, container/registration. Feed gates precede set/event observers. Security gate:12 precedes VM security registration:16 and host secret construction:21. VM creation itself succeeds in runtime startup logs; later VM-secret/security setup is unreached. Complete snapshots, numeric probes, events, consumer/cached lifecycle, GREEN/native acceptance remain unproven.

## Separate compile and receipt evidence

Complete saved compile JSON parsed: **741 records** (665 compiler-artifact, 75 build-script-executed, 1 successful build-finished), **zero compiler-message records**. Full saved stderr inspected: **six distinct iced-wgpu-patched manifest deprecations**, plus aggregate six-warning summary (not seventh warning). Terminal SubState=exited, Result=success, ExecMainStatus=0. No-run compile is setup proof, not behavioral acceptance. Full diagnostics retained in proof.json.

Existing new authentication receipt read; executable **stat only**, all four fields equal receipt before/after: `{"st_dev": 66306, "st_ino": 142103913, "st_mtime_ns": 1791916133031045263, "st_size": 470748552}`. Prior receipt claims PASS/one hash; no executable read/hash/reauth performed here. Metadata continuity does not newly authenticate contents, source or external dependency/cache provenance.

Initial PTR epoch `spell-display-ptr-red-20261013T181858205721Z`: admission denied, only admission.json, **zero launches**; not behavioral RED. Retry compile/runtime admission allowed and actual nine-case failure is separate.

## Restrictions and private persistence

No runtime/build/hash/reauth/delegation, sourcearchive/tree/current-source reads, or tracked edits. Main retains all operations/production work; recorded RED source unchanged by this verifier.

Only absolute private outputs (directory0700, files0600), full readback equality and existence checked:
- `<PRIVATE_VERIFICATION_ROOT>/spell-display-ptr-genuine-red-current/report.md`
- `<PRIVATE_VERIFICATION_ROOT>/spell-display-ptr-genuine-red-current/proof.json`
