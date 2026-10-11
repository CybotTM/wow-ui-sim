# Independent saved-artifact verification

Epoch: `/home/osso/.local/state/wow-ui-sim/verification/four-profile-stock-startup-current/20261011T003325Z`

**Capture provenance PASS. Completion 1/4. Clean startup not established.**

All receipts/outcome present. No replay/build/operations/delegation/polling; reads and private reports only. Worker, parent submission/request/completed controller, profile receipts, and results/outcome cross-checked. `outcome.json` independent_admission remains `PENDING` (not a passed acceptance gate).

| Profile | Build exit | Completion exit / marker | Lua exit / unique / occurrences | Cache admission |
|---|---:|---|---|---|
| wrath | 0 | 1 / 0 | 1 / unassessed / unassessed | absent (main observation) |
| mists | 0 | 0 / 1 | 1 / 2 / 2 | present (main observation) |
| era | 0 | 1 / 0 | 1 / unassessed / unassessed | absent (main observation) |
| anniversary | 0 | 1 / 0 | 1 / unassessed / unassessed | absent (main observation) |

## Completion versus cleanliness

Mists completion marker occurs exactly once at `mists/completion.stderr:285`, after post-event workarounds and before font/layout/tree work. Completion exit 0. Logged login, world-enter and post-login events complete; `FIRST_FRAME_RENDERED` here is a CLI event, not evidence of a GUI render. Lua-errors exit 1: `mists/lua-errors.stderr:234-238` reports FAILED, 2 unique / 2 occurrences, both attributed to Blizzard_UIPanels_Game, none unattributed. Completion stderr also records both errors; exit 0 does not imply clean Lua.

Wrath/Era/Anniversary: both commands exit 1, no marker and empty Lua stdout. Each records `Blizzard UI source missing; syncing Blizzard UI from CASC` followed by `Could not find a World of Warcraft installation.` Empty Lua stdout is **not** 0 errors: startup was blocked before the stock Blizzard load. Missing profile caches are main direct observations, consistent with logs; no historical cache snapshot in receipts.

### Exact Mists JSON signatures (count 1 each)

```text
Failed to create frame __Blizzard_UIPanels_Game_20279: Lua error: layer children on __Blizzard_UIPanels_Game_20279: Action[SetPoint] failed because[Cannot anchor to itself]: attempted from: Frame:SetPoint.
stack traceback:
	(string):1: in main chunk
```
SHA256: `51cd82b07de713c51329fdab1589674781fdf9b545d9a0efcd3cd01a7add7604`

```text
Lua Error: Failed to create frame __Blizzard_UIPanels_Game_20279: Lua error: layer children on __Blizzard_UIPanels_Game_20279: Action[SetPoint] failed because[Cannot anchor to itself]: attempted from: Frame:SetPoint.
stack traceback:
	(string):1: in main chunk
[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:59: in function 'GetErrorData'
[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:72: in function <[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:71>
```
SHA256: `a0cf068565fba737f47e4c387327a035a1ffca098772305c532550bd9a1b1d0e`

## Binaries, compiler JSON and source

Recorded revision: `47d54d918c85d95454d5980259cc23943f93fe12`. Four normal `dev` non-test artifacts: opt_level=1, debuginfo=line-tables-only, debug_assertions=true, overflow_checks=true. `casc, gui, rodio, sound` plus matching `client-*`; Mists additionally `prefork-full-ui`. No `fast-build` or `default` feature in target artifact JSON. Features/profile/size/hash agree with artifact receipts. Binary hashing performed once per sealed binary; declared source files hashed once.

- `wrath/wow-sim-normal`: 398443064 bytes, SHA256 `178627ad5fabe6f3c1bfc6adb5c4ee2b2edf82a67461997b854f97820757c12a`.
- `mists/wow-sim-normal`: 398790704 bytes, SHA256 `2c4e9b4cd3b4384117224025ac9869092bea5145bca641d6e10a16b29a057f85`.
- `era/wow-sim-normal`: 398405840 bytes, SHA256 `3444e7d28140d9ce83319ae26f11d1e21227924c77d099e6648e6cadef4c90df`.
- `anniversary/wow-sim-normal`: 398196712 bytes, SHA256 `6a7a2af08560ea5eda2db80a79062721a16661a6d55cd5fb062110f6c1c103a3`.

3846 declared file hashes (1653 src, 2189 tests, four Cargo/build/config files) match baseline, all 12 saved source maps, and one current source hashing pass; no missing or mismatching files. Dependencies/vendor/data/runtime caches not covered.

Per build: 653 fresh artifacts, 0 nonfresh, 74 build-script-executed records, 1 successful build-finished; zero compiler-message warnings; six manifest deprecations plus one summary warning line (not seven distinct warnings). Across four: 2612 fresh artifacts, 24 repeated manifest warning occurrences, six distinct manifest warnings.
Six distinct deprecated manifest lint names: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity. All in iced-wgpu-patched/Cargo.toml. Warning text retained verbatim in evidence. `build-script-executed` records do not independently prove fresh build-script execution. Saved Cargo/rustc version receipts absent; exact compiler versions unproven.

## Timing and resource admission

| Profile | Cargo reported seconds | Submission→compile seconds | Lua-errors seconds | Completion seconds |
|---|---:|---:|---:|---:|
| wrath | 0.76s | 0.852145 | 0.331272 | 0.341922 |
| mists | 0.30s | 0.377244 | 4.893915 | 4.023537 |
| era | 0.19s | 0.234412 | 0.163279 | 0.141023 |
| anniversary | 0.17s | 0.208569 | 0.182246 | 0.231117 |

All command receipts finish under the 90s wrapper, none exit 124. This is coarse saved wall time only: 653/653 fresh artifacts per build, Mists 1094/1094 bytecode hits in both captures; not cold-build or cold-start proof. Mists reported Blizzard load 3.47s (Lua-errors) / 2.41s (completion); completion login 91.56ms, world-enter 278.83ms, post-login 111.85ms. Logged last post-event milestone 3.553s, marker untimestamped.

Resource observations independently recomputed against worker predicate: MemAvailable ≥6GiB, load1 <24, no relevant Cargo; all four admitted on attempt 1. Historical snapshots only, not current resource guarantees. Resource admission is separate from missing cache admission.

- wrath: 33.982098GiB available, load1=14.03, relevant Cargo=[]; compile cgroup CPU delta 523917µs, throttled-period delta 0.
- mists: 32.320316GiB available, load1=14.67, relevant Cargo=[]; compile cgroup CPU delta 377525µs, throttled-period delta 0.
- era: 32.036289GiB available, load1=14.9, relevant Cargo=[]; compile cgroup CPU delta 235407µs, throttled-period delta 0.
- anniversary: 32.190807GiB available, load1=14.19, relevant Cargo=[]; compile cgroup CPU delta 209423µs, throttled-period delta 0.

Both saved cgroup snapshots per profile match launcher service, cpu.max `1200000 100000` (12 CPU-equivalent quota) and memory.max `17179869184` (16GiB). Snapshots bracket compile only. memory.peak is cumulative in the shared service, not per-profile startup peak. Runtime logs say 2 CPU cores; quota is an upper bound, not proof of actual parallelism or affinity. Full counters preserved in evidence.

## Scope limits

- No cargo -V/rustc -Vv receipt: exact compiler versions not independently established by saved compiler JSON.
- Resource admission is not cache admission. Main direct observed caches: Wrath/Era/Anniversary absent; Mists present (caller evidence, not an independent historical filesystem snapshot).
- Cgroup snapshots bracket compile only; shared service cumulative counters and peaks are not isolated runtime measures. Runtime says Resource limits: 2 CPU core(s), unlike service quota capacity of 12; effective affinity/runtime concurrency not proven.
- Wall times and logged milestones are single-capture coarse observations, not benchmarks, cold-cache performance, native startup timing, or 90s successful completion guarantees.
- Not all-profile/native/GUI/third-party/assets/full-suite acceptance.
- WOW_SIM_CASC=0 is recorded, not proof all code paths disable CASC: completion logs WowFontSystem::new begin casc=true; failed profiles explicitly attempt CASC UI sync. Mists used read-only EditMode cache and 1094/1094 bytecode cache hits in each capture; not cold or stateless startup.

Full runtime stdout/stderr, compiler record counts/target artifact, warning text, JSON signatures, source hash map, file fingerprints and receipt comparisons: `independent-verification-evidence.json`. Summary: `independent-verification-report.json`.
