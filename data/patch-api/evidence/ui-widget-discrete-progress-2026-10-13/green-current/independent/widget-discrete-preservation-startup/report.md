# Saved-evidence audit: discrete preservation + startup

Evidence root: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/runtime-preservation`
Output root: `<REPO>/verification/widget-discrete-preservation-startup`
Revision: `50f6f0ea894d0bbe2f281887ef36d25e0bbbf313`.

## Preservation: PASS — 5/5 unique
Bars 1; HUD 2; core lifecycle 2. Five child duplicate summaries excluded; no double counting.

### bars-consumer
Admission: allowed=True; UTC=2026-10-13T12:22:34.272900+00:00; projected=31614642020; host_available=31960576000; worker_max=4294967296; worker_high=3221225472.
Saved launcher exit: 0; `--wait` present: True. Actual saved termination:
```text
Running as unit: wow-discrete-bars-consumer-20261013t122234273021z.service
          Finished with result: success
Main processes terminated with: code=exited, status=0/SUCCESS
               Service runtime: 5.822s
             CPU time consumed: 5.258s
                   Memory peak: 1.5G (swap: 0B)
```
Expected unique: 1; cases: ui_widget_manager_bars_consumer::actual_cached_status_and_capture_bar_lifecycle.
Parent summary: ('ok', '1', '0', '0', '0', '10973'); child duplicate summaries excluded: 1.
Full argv (exact JSON array):
```json
[
  "systemd-run",
  "--user",
  "--wait",
  "--unit=wow-discrete-bars-consumer-20261013t122234273021z",
  "--slice=agents.slice",
  "-p",
  "MemoryMax=4G",
  "-p",
  "MemoryHigh=3G",
  "-p",
  "CPUQuota=200%",
  "-p",
  "KillMode=control-group",
  "-p",
  "StandardOutput=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/runtime-preservation/bars-consumer.stdout",
  "-p",
  "StandardError=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/runtime-preservation/bars-consumer.stderr",
  "--working-directory=<REPO>",
  "/usr/bin/timeout",
  "--signal=TERM",
  "--kill-after=2",
  "86",
  "/usr/bin/bwrap",
  "--die-with-parent",
  "--unshare-net",
  "--ro-bind",
  "/",
  "/",
  "--ro-bind",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/source",
  "<REPO>",
  "--ro-bind",
  "<REPO>/target",
  "<REPO>/target",
  "--tmpfs",
  "/tmp",
  "--dev",
  "/dev",
  "--proc",
  "/proc",
  "--chdir",
  "<REPO>",
  "--setenv",
  "HOME",
  "<HOME>",
  "--setenv",
  "WOW_SIM_CASC",
  "0",
  "--setenv",
  "WOW_SIM_NO_SOUND",
  "1",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/artifacts/deps/integration-9f07e4a55f2bbbf7",
  "ui_widget_manager_bars_consumer::",
  "--nocapture",
  "--test-threads=1"
]
```
Full saved stdout:
```text

running 1 test
test ui_widget_manager_bars_consumer::actual_cached_status_and_capture_bar_lifecycle ... 
running 1 test
test ui_widget_manager_bars_consumer::actual_cached_status_and_capture_bar_lifecycle ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10973 filtered out; finished in 5.51s

ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10973 filtered out; finished in 5.57s
```
Full stderr inspected: 730 bytes; stdout/stderr saved result byte sizes agree: True.

### hud-consumers
Admission: allowed=True; UTC=2026-10-13T12:22:40.108868+00:00; projected=31461309804; host_available=32192643072; worker_max=4294967296; worker_high=3221225472.
Saved launcher exit: 0; `--wait` present: True. Actual saved termination:
```text
Running as unit: wow-discrete-hud-consumers-20261013t122240108966z.service
          Finished with result: success
Main processes terminated with: code=exited, status=0/SUCCESS
               Service runtime: 4.813s
             CPU time consumed: 4.558s
                   Memory peak: 1.4G (swap: 0B)
```
Expected unique: 2; cases: ui_widget_manager_hud_containers::cached_standard_hud_default_zero_leaves_no_active_widget_set, ui_widget_manager_hud_containers::cached_standard_hud_onload_consumes_preseeded_selection_and_records.
Parent summary: ('ok', '2', '0', '0', '0', '10972'); child duplicate summaries excluded: 2.
Full argv (exact JSON array):
```json
[
  "systemd-run",
  "--user",
  "--wait",
  "--unit=wow-discrete-hud-consumers-20261013t122240108966z",
  "--slice=agents.slice",
  "-p",
  "MemoryMax=4G",
  "-p",
  "MemoryHigh=3G",
  "-p",
  "CPUQuota=200%",
  "-p",
  "KillMode=control-group",
  "-p",
  "StandardOutput=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/runtime-preservation/hud-consumers.stdout",
  "-p",
  "StandardError=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/runtime-preservation/hud-consumers.stderr",
  "--working-directory=<REPO>",
  "/usr/bin/timeout",
  "--signal=TERM",
  "--kill-after=2",
  "86",
  "/usr/bin/bwrap",
  "--die-with-parent",
  "--unshare-net",
  "--ro-bind",
  "/",
  "/",
  "--ro-bind",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/source",
  "<REPO>",
  "--ro-bind",
  "<REPO>/target",
  "<REPO>/target",
  "--tmpfs",
  "/tmp",
  "--dev",
  "/dev",
  "--proc",
  "/proc",
  "--chdir",
  "<REPO>",
  "--setenv",
  "HOME",
  "<HOME>",
  "--setenv",
  "WOW_SIM_CASC",
  "0",
  "--setenv",
  "WOW_SIM_NO_SOUND",
  "1",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/artifacts/deps/integration-9f07e4a55f2bbbf7",
  "ui_widget_manager_hud_containers::",
  "--nocapture",
  "--test-threads=1"
]
```
Full saved stdout:
```text

running 2 tests
test ui_widget_manager_hud_containers::cached_standard_hud_default_zero_leaves_no_active_widget_set ... 
running 1 test
test ui_widget_manager_hud_containers::cached_standard_hud_default_zero_leaves_no_active_widget_set ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10973 filtered out; finished in 2.40s

ok
test ui_widget_manager_hud_containers::cached_standard_hud_onload_consumes_preseeded_selection_and_records ... 
running 1 test
test ui_widget_manager_hud_containers::cached_standard_hud_onload_consumes_preseeded_selection_and_records ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10973 filtered out; finished in 2.25s

ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 10972 filtered out; finished in 4.77s
```
Full stderr inspected: 1467 bytes; stdout/stderr saved result byte sizes agree: True.

### core-lifecycle
Admission: allowed=True; UTC=2026-10-13T12:22:44.935278+00:00; projected=33168410708; host_available=30831284224; worker_max=4294967296; worker_high=3221225472.
Saved launcher exit: 0; `--wait` present: True. Actual saved termination:
```text
Running as unit: wow-discrete-core-lifecycle-20261013t122244935422z.service
          Finished with result: success
Main processes terminated with: code=exited, status=0/SUCCESS
               Service runtime: 3.684s
             CPU time consumed: 3.618s
                   Memory peak: 1G (swap: 0B)
```
Expected unique: 2; cases: ui_widget_manager_lifecycle::cached_ui_widget_container_filters_supplied_unit_updates, ui_widget_manager_lifecycle::cached_ui_widget_container_refreshes_removes_and_unregisters_supplied_widgets.
Parent summary: ('ok', '2', '0', '0', '0', '10972'); child duplicate summaries excluded: 2.
Full argv (exact JSON array):
```json
[
  "systemd-run",
  "--user",
  "--wait",
  "--unit=wow-discrete-core-lifecycle-20261013t122244935422z",
  "--slice=agents.slice",
  "-p",
  "MemoryMax=4G",
  "-p",
  "MemoryHigh=3G",
  "-p",
  "CPUQuota=200%",
  "-p",
  "KillMode=control-group",
  "-p",
  "StandardOutput=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/runtime-preservation/core-lifecycle.stdout",
  "-p",
  "StandardError=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/runtime-preservation/core-lifecycle.stderr",
  "--working-directory=<REPO>",
  "/usr/bin/timeout",
  "--signal=TERM",
  "--kill-after=2",
  "86",
  "/usr/bin/bwrap",
  "--die-with-parent",
  "--unshare-net",
  "--ro-bind",
  "/",
  "/",
  "--ro-bind",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/source",
  "<REPO>",
  "--ro-bind",
  "<REPO>/target",
  "<REPO>/target",
  "--tmpfs",
  "/tmp",
  "--dev",
  "/dev",
  "--proc",
  "/proc",
  "--chdir",
  "<REPO>",
  "--setenv",
  "HOME",
  "<HOME>",
  "--setenv",
  "WOW_SIM_CASC",
  "0",
  "--setenv",
  "WOW_SIM_NO_SOUND",
  "1",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/artifacts/deps/integration-9f07e4a55f2bbbf7",
  "ui_widget_manager_lifecycle::",
  "--nocapture",
  "--test-threads=1"
]
```
Full saved stdout:
```text

running 2 tests
test ui_widget_manager_lifecycle::cached_ui_widget_container_filters_supplied_unit_updates ... 
running 1 test
test ui_widget_manager_lifecycle::cached_ui_widget_container_filters_supplied_unit_updates ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10973 filtered out; finished in 1.78s

ok
test ui_widget_manager_lifecycle::cached_ui_widget_container_refreshes_removes_and_unregisters_supplied_widgets ... 
running 1 test
test ui_widget_manager_lifecycle::cached_ui_widget_container_refreshes_removes_and_unregisters_supplied_widgets ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10973 filtered out; finished in 1.73s

ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 10972 filtered out; finished in 3.66s
```
Full stderr inspected: 1284 bytes; stdout/stderr saved result byte sizes agree: True.

### normal-startup
Admission: allowed=True; UTC=2026-10-13T12:22:48.633528+00:00; projected=32015834512; host_available=31163539456; worker_max=4294967296; worker_high=3221225472.
Saved launcher exit: 0; `--wait` present: True. Actual saved termination:
```text
Running as unit: wow-discrete-normal-startup-20261013t122248633681z.service
          Finished with result: success
Main processes terminated with: code=exited, status=0/SUCCESS
               Service runtime: 7.984s
             CPU time consumed: 5.266s
                   Memory peak: 1.5G (swap: 0B)
```
Expected unique: 0; cases: startup, not test cases.
Parent summary: None; child duplicate summaries excluded: 0.
Full argv (exact JSON array):
```json
[
  "systemd-run",
  "--user",
  "--wait",
  "--unit=wow-discrete-normal-startup-20261013t122248633681z",
  "--slice=agents.slice",
  "-p",
  "MemoryMax=4G",
  "-p",
  "MemoryHigh=3G",
  "-p",
  "CPUQuota=200%",
  "-p",
  "KillMode=control-group",
  "-p",
  "StandardOutput=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/runtime-preservation/normal-startup.stdout",
  "-p",
  "StandardError=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/runtime-preservation/normal-startup.stderr",
  "--working-directory=<REPO>",
  "/usr/bin/timeout",
  "--signal=TERM",
  "--kill-after=2",
  "86",
  "/usr/bin/bwrap",
  "--die-with-parent",
  "--unshare-net",
  "--ro-bind",
  "/",
  "/",
  "--ro-bind",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/source",
  "<REPO>",
  "--ro-bind",
  "<REPO>/target",
  "<REPO>/target",
  "--tmpfs",
  "/tmp",
  "--dev",
  "/dev",
  "--proc",
  "/proc",
  "--chdir",
  "<REPO>",
  "--setenv",
  "HOME",
  "<HOME>",
  "--setenv",
  "WOW_SIM_CASC",
  "0",
  "--setenv",
  "WOW_SIM_NO_SOUND",
  "1",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/artifacts/deps/wow-sim",
  "--no-addons",
  "--no-saved-vars",
  "--exec-lua",
  "local get=C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo; assert(type(get)=='function'); assert(select('#',get(760301))==1); assert(get(760301)==nil); print('DISCRETE_EMPTY_STATE',type(get),select('#',get(760301)),type(get(760301)))",
  "lua-errors"
]
```
Full saved stdout:
```text
DISCRETE_EMPTY_STATE	function	1	nil
[]
```
Full stderr inspected: 14686 bytes; stdout/stderr saved result byte sizes agree: True.

## Normal startup: PASS
One separately launched saved process, PID 656349; 291 Blizzard addons loaded. Actual `--exec-lua` diagnostic:
```lua
local get=C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo; assert(type(get)=='function'); assert(select('#',get(760301))==1); assert(get(760301)==nil); print('DISCRETE_EMPTY_STATE',type(get),select('#',get(760301)),type(get(760301)))
```
Probe reached; exact stdout:
```text
DISCRETE_EMPTY_STATE	function	1	nil
```
Function exists; new empty query 760301 returns exactly one nil. Diagnostic neither missing nor unreached.
Entire stderr inspected: 14686 bytes / 219 lines. Zero actual errors, warnings, or AnimSetup diagnostics. Keyword hits are restoration names and zero-error summary only.
```text
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```
`[]` and exit 0 were not used alone as proof. Scope: no addons/saved variables; CASC/sound disabled per exact argv.

## Existing receipts versus current stat-only
- integration: current `{"st_dev": 66306, "st_ino": 139893679, "st_mtime_ns": 1791893757924232364, "st_size": 470878000}`; matches saved before=True, after=True. Receipt: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/receipt-integration.json`. Saved authentication PASS is historical, not repeated.
- wow-sim: current `{"st_dev": 66306, "st_ino": 139893681, "st_mtime_ns": 1791893660569693455, "st_size": 408464016}`; matches saved before=True, after=True. Receipt: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/receipt-wow-sim.json`. Saved authentication PASS is historical, not repeated.

## Boundaries
Seven new GREEN cases remain pending independent audit 3674. Earlier Header/Prey/core-query evidence is reuse, not fresh counts. Neither group is included in these 5 preservation cases.
No binary/library reads, hashes, authentication, reruns, builds, checks, live operations, delegation, or tracked edits. Freshness here means current artifact stat equals saved receipts, not a new runtime execution or content authentication.
Full admissions, launchers, argv, result records, stdout, ENTIRE stderr, evidence file stats, and historical receipts are embedded in `proof.json`.
