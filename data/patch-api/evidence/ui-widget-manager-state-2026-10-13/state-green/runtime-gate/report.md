# Bounded runtime gate: FAIL

Compiled revision: `67775404d`. Existing compile evidence: `../compile.stdout`, `../compile.stderr`, `../terminal-systemd.txt`. Compile terminal `ExecMainCode=1`, `ExecMainStatus=0`; six inherited iced_wgpu manifest deprecation warnings, zero compiler-message diagnostics. Complete parsed diagnostics retained in `compiler-diagnostics-full.json`.

| Scope | Passed/expected | Process exit | Observed proof |
|---|---:|---:|---|
| ui_widget_manager_state:: | 6/6 | 0 | Supplied membership, host-feed ordering, exact IDs, complete fresh isolated snapshots, absent IDs |
| ui_widget_manager_lifecycle:: | 0/1 | 101 | Real cached UI reached unregister-cleanup assertion; not prerequisite failure |
| ui_widget_manager_security:: | 1/1 | 0 | Tainted-secret denial and public isolated snapshot recovery |
| c_ui_widget_manager_button_header:: | 9/9 | 0 | Cached provider hidden filtering; field projection, argument security, fresh isolated records |
| path_control_point:: | 6/6 | 0 | Cached smooth XML, declared point order/mutation, None playback, Catmull-Rom geometry, stop/finish offset restoration |
| p1200_rest_catalogs::p1200_rest_prey_missing --exact | 1/1 | 0 | Only prey test selected: supplied widget snapshot and active-quest existence |
| Normal startup lua-errors | 0 unique / 0 total Lua errors | 0 | JSON []; all 219 stderr lines inspected; no warnings or AnimSetup diagnostics |

Integration total: **23/24**. Lifecycle child and parent summaries describe the same single failing test, not two tests. Failure: `tests/ui_widget_manager_lifecycle.rs:135:33`, phase `unregister cleans active pools events manager and timer`, Lua assertion failed. Output does not identify which assertion within that phase. No fix, rerun, or broader completion claim.

## Evidence and isolation

- Every process has `<label>.argv.json`, `.admission.json`, `.stdout`, `.stderr`, `.launch.txt`, `.terminal.txt`, and `.report.json`; future-controller processes also retain `.observations.json`.
- Artifacts authenticated once against `../artifact-seals.json`; immediate `.auth-receipt.json` uses `st_ino`, size, mtime_ns, device, and SHA-256. `deps/` aliases are hardlinks; `.alias-receipt.json` verifies identical stats without hashing again.
- Runtime source is frozen `source-67775404d/`, extracted from that revision, read-only mounted over the repo. Current staged tests/docs were not used. Existing target remains read-only for runtime dependencies.
- Per-process fresh admission in agents.slice: MemoryMax=4G, MemoryHigh=3G, CPUQuota=200%; reclaim-aware projection and host availability/pressure retained. Bwrap root read-only, private /tmp, preserved HOME, network isolated, CASC=0, sound disabled. Startup no-addons/no-saved-vars; alias parent `deps` routes CVar persistence into private /tmp.
- Runtime timeout 86 seconds with 2-second kill grace. Initial state controller blocked on `--wait` plus `RemainAfterExit=yes` after tests finished. Parent retained exact terminal zero in `state.main-retained-terminal.txt`, then stopped only that completed service to release launcher; `state.controller-recovery.txt` records recovery. State was not rerun. Later workers submit without --wait and inspect completion bounded to 92 seconds, preserving exact process terminals.

No tracked edits, Cargo/build/check, production operations, delegated work, or repeated unaffected 46-query suite. Bounded verification is finished; lifecycle requirement remains failing.
