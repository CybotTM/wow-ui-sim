# Source 41cb full suite — terminal FAIL

Exact source: `41cbf5005d4fec8dba98538e425e6d8fd7f0f5c3`. Original asynchronous submission remains historical; terminal artifacts record October 12, 2026, 18:26:49–18:59:39 CDT. Independent artifact verification PASS means retained results reconcile—not that the suite passed.

| Scope | Passed | Failed | Skipped |
|---|---:|---:|---:|
| Integration | 10,796 | 0 | 19 |
| Prefork primary | 2,324 | 0 | 0 |
| Prefork auxiliary | 4 | 0 | 0 |
| Library | 1,999 | 2 | 0 |
| Total invocations | 15,123 | 2 | 19 |

Integration/prefork exits are zero; library exit is 100. Totals count invocations, not unique capabilities. Nineteen skip identities/reasons and exact per-step wall-clock boundaries are unavailable. Six existing manifest deprecations recur across steps; warnings remain in selected output.

The two library failures are `setup_layout_info_preserves_distinct_saved_action_bar_profile_settings` and `setup_layout_info_remaps_active_saved_layout_after_prepending_presets`: Lua nil-index errors in fixtures using the superseded custom-ordinal/layout-ID selection assumption. Test-only correction `b41b42fc1` uses combined positions 4/3 and checks native preset index 2 → Classic. Initial focused source review found the new preset control reused cached manager state; follow-up `ac170ecbf` resets that fixture. Runtime was resource-BLOCKED, zero tests invoked; no repaired-test PASS or restored production remapper.

## Retained evidence

- [Recorded result](result.json): exact terminal JSON.
- [Artifact report](artifact-report.md): commands, full-output accounting, failures and limitations.
- [Selected output](selected-output.log): summaries, warnings and exact failures; private checkout paths replaced by `[PRIVATE_PATH]`.
- [Excerpt provenance](excerpt-provenance.json): original stream hashes, inclusive line ranges and redacted-line identities; full raw log stays private.
- [Retention manifest](retention-manifest.json): four exact privacy-reviewed retained data streams, 23,290 bytes. Public-copy verification remains separate.

No test/build replay. This source predates new VertexColor tests/spec `1887ec9cf` and Edit Mode fixture repair `b41b42fc1`; no current-HEAD whole-suite acceptance. Older suite receipts and failures remain unchanged. Whole handoff and second patch-note pass **OPEN**.
