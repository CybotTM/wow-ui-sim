# Widget / atlas PTR saved-evidence audit

**PASS — bounded saved PTR evidence only.** No parent-goal or native-WoW completion claim.

## Evidence boundary

Epoch: `<PRIVATE_VERIFICATION_ROOT>/ui-widget-bars-hud-green-current/20261013T102802192152Z`.

Compiled source: `34ef9235dfd724ea2ffb7256cd879b2a6b38334a`; saved metadata identifies docs-only revision over production `5bd208dba`; profile **PTR**. Source relationship is a saved declaration, not an independently inspected diff. Reused `source-39ce8969a` bind appears in launcher argv; no other epoch was inspected. Source parity receives no runtime credit.

Only saved text/JSON and artifact stat metadata were inspected. No rerun, build, check, binary/library hash, delegation, tracked edit, source inspection, or live service query. Full text inventory, exact argv, admissions, receipts, observed stats, and full runtime streams are preserved in `proof.json`.

## Runtime coverage

| Saved group | Unique pass | Parent filtered | Proof |
|---|---:|---:|---|
| `ui_widget_manager_bars::` | 2 | 10139 | PTR runtime PASS |
| `ui_widget_manager_hud_selections::` | 3 | 10138 | PTR runtime PASS |
| `ui_widget_manager_state::` | 6 | 10135 | PTR runtime PASS |
| `ui_widget_manager_security::` | 1 | 10140 | PTR runtime PASS |
| `ui_widget_manager_lifecycle::` | 2 | 10139 | PTR runtime PASS; child summaries deduplicated |
| `set_atlas_arguments::` | 2 | 10139 | PTR runtime PASS |
| `texture_atlas_regressions::` | 10 | 10131 | PTR runtime PASS |

**26 unique tests; 0 failed, ignored, or measured.** Each parent accounts for 10141 available tests, not 10141 executed tests. All seven saved launcher results are exit 0 and their full `--wait` launcher streams say `code=exited, status=0/SUCCESS`.

Lifecycle stdout contains two child `1 passed` summaries and a final parent `2 passed` summary. Count the final parent once: nine raw summaries would incorrectly total 28; deduplication yields 26. Child names repeat the parent's two lifecycle identities, not two additional tests. Module filters are not `--exact` test selectors; exactness here means saved argv equality and exact parent counts.

Three Retail-only bar/HUD consumer tests remain excluded: **0 PTR credit**. Their names are not supplied by this epoch. Passing PTR query/state/lifecycle tests does not establish those consumers or native parity.

## One saved normal PTR startup

`startup-enum-diagnostic` has one process-start record, 283 Blizzard addons, no third-party addons or SavedVariables, and CASC/sound disabled. It is normal simulator startup with post-startup diagnostic Lua, not a native client run or a consumer-specific rendering test.

Full stderr: **219 lines / 14696 bytes**. Full stdout: **30 lines / 1798 bytes**: 23 unique enum records, each `number`, expected value, `true`; percentages `0.2` and `0.1`; four atlas lookups returning `table`; trailing `[]`. Diagnostic Lua argv matches all 29 diagnostic output records. No unexpected diagnostics precede the explicit final stderr summary:

```text
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```

Saved launcher exit **0**, full launcher `status=0/SUCCESS`, terminal `Result=success` / `ExecMainStatus=0`. **`[]` alone is not clean-startup proof**; conclusion uses both complete streams and launcher evidence. Atlas tables establish lookup availability only, not visuals, asset extraction, native behavior, or excluded Retail consumers.

## Compile / provenance / admission

Full compile stdout: **741 JSON records / 574600 bytes**: 665 compiler artifacts, 75 build-script records, one final `build-finished` with `success=true`; no Rust compiler-message records. Both sealed executable artifacts exactly match records in that full stream. Features contain `client-ptr`, not `client-retail`; `retail-12-*` epoch features do not select the Retail client. Saved Cargo argv explicitly uses `--no-default-features --features sound,gui,casc,client-ptr`.

Compile submission exit 0 alone is insufficient: its argv is asynchronous, without `--wait`. Saved compile terminal supplies `Result=success`, `SubState=exited`, `ExecMainStatus=0`, exit timestamp `Tue 2026-10-13 05:31:11 CDT`. Compile stderr finishes in 3m 09s. Six `iced-wgpu-patched/Cargo.toml` deprecated manifest lint-key warnings are retained, not called warning-free:

```text
large-enum-variant -> large_enum_variant
map-entry -> map_entry
match-wildcard-for-single-variants -> match_wildcard_for_single_variants
redundant-closure-for-method-calls -> redundant_closure_for_method_calls
trivially-copy-pass-by-ref -> trivially_copy_pass_by_ref
type-complexity -> type_complexity
```

Compile admission and eight runtime admissions recompute consistently from saved raw counters: half-clean-file/slab reclaim, projected memory, parent high/max bound, recorded host availability, and pressure <5. Compile worker limits 8GiB/6GiB, CPU 800%; runtime 4GiB/3GiB, CPU 200%. Exact runtime wrapper argv includes `systemd-run --user --wait`, `agents.slice`, control-group kill mode, separate output files, 86-second TERM timeout with 2-second kill grace, read-only bwrap binds, network isolation, and the sealed PTR executable. No fresh resource-state claim.

Both saved receipts state PASS, matching digests and before/after stats. Seal/receipt digest strings agree; **no digest was recomputed**. Observed `(device, inode, bytes, mtime_ns)` equals seal plus all three receipt stat snapshots:

```text
integration: (66306, 140810419, 469979728, 1791887471339258977)
wow-sim:     (66306, 140810420, 415272136, 1791887415886827204)
```

Runtime terminal snapshots are inactive/dead with blank exit timestamps and unset peaks; they are not standalone execution proof. Full exact-unit `--wait` launcher streams plus recorded launcher return codes and test/diagnostic streams provide bounded saved execution evidence. No parent/native completion assertion follows.
