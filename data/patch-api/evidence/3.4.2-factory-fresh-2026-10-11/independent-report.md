# Independent Wrath factory artifact verification

Verified: 2026-10-11T00:33:04.455843+00:00

## Verdict: PASS — saved receipt admission, bounded scope

**Six factory controls passed; 155 observations exactly reproduce saved historical GREEN. Publication coverage remains 72 matches / 83 pinned mismatches. Native behavior and full-profile acceptance remain unproven.**

Read and followed `/home/osso/AgentConfig/skills/verify/SKILL.md`, artifact mode, as assigned independent verifier. No tests, builds, process execution, operations, delegation, or repository edits. Only Pyrun/read and private report/evidence writes. This verifies the already captured execution, not a replay.

Fresh root: `/home/osso/.local/state/wow-ui-sim/verification/wrath-factory-fresh-current/20261011T003109Z`. Reference artifact/build root: `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/20261010T235452Z/wrath`. Historical GREEN: `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/3.4.2-factory-2026-10-09`. Evidence: `independent-evidence.json`.

## Execution and exact selectors

Recorded argv (not executed by verifier):
```json
{
  "argv": [
    "/usr/bin/timeout",
    "90",
    "/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/20261010T235452Z/wrath/integration-test",
    "patch_3_4_2_factory::",
    "--nocapture",
    "--test-threads=1"
  ],
  "artifact_unchanged": true,
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "ended": "2026-10-11T00:31:11.310385+00:00",
  "environment_overrides": {
    "WOW_SIM_P342_FACTORY_OUT": "/home/osso/.local/state/wow-ui-sim/verification/wrath-factory-fresh-current/20261011T003109Z/observations.json",
    "WOW_SIM_P342_SUCCESSOR_CONTROL_OUT": "/home/osso/.local/state/wow-ui-sim/verification/wrath-factory-fresh-current/20261011T003109Z/successor-control.json"
  },
  "exit": 0,
  "scope": "Bare Wrath simulator factory publication/absence controls; not API semantics/native or full-profile acceptance",
  "source_equal": true,
  "started": "2026-10-11T00:31:10.099440+00:00"
}
```

| Exact selector | Result | Covered control |
|---|---|---|
| `patch_3_4_2_factory::bare_wrath_factories_probe_literal_widget_owners` | ok (1 case) | 18 literal widget owner rows produce object-method probes; this does not assert all methods exist |
| `patch_3_4_2_factory::cvar_defaults_remain_separate_from_publication` | ok (1 case) | Added CVars split (14 absent, 11 matching defaults, 1 published differing default) |
| `patch_3_4_2_factory::foreign_successors_do_not_remove_wrath_publication` | ok (1 case) | Foreign retail/mists removals do not supersede synthetic own-line function; own true, default/value null, no superseded_by; return 342 asserted |
| `patch_3_4_2_factory::patch_3_4_2_exact_factory_publication_gaps` | ok (1 case) | 155 rows; exact false-ID set equals 83 reviewed known gaps |
| `patch_3_4_2_factory::wrath_event_acceptance_is_nondiscriminating` | ok (1 case) | Nonsense P342_NOT_A_NATIVE_EVENT and removed TWITTER_POST_RESULT both register; permissiveness confirmed |
| `patch_3_4_2_factory::wrath_factory_rejects_retail_inventory` | ok (1 case) | Expected panic only: retail inventory rejected before aliases/measurement |

Exact full stdout:
```text

running 6 tests
test patch_3_4_2_factory::bare_wrath_factories_probe_literal_widget_owners ... ok
test patch_3_4_2_factory::cvar_defaults_remain_separate_from_publication ... ok
test patch_3_4_2_factory::foreign_successors_do_not_remove_wrath_publication ... ok
test patch_3_4_2_factory::patch_3_4_2_exact_factory_publication_gaps ... ok
test patch_3_4_2_factory::wrath_event_acceptance_is_nondiscriminating ... ok
test patch_3_4_2_factory::wrath_factory_rejects_retail_inventory - should panic ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 8029 filtered out; finished in 1.14s
```

Recorded wall window: 1.210945s; libtest 1.14s. Single thread; 90s bound; exit 0. Six selected, zero failed/ignored/measured, 8,029 filtered; 8,035 discovered in aggregate. These are not 8,035 passing tests.

Full stderr inspected: 18 widget lines, 15 CVar-default diagnostics, startup traces, and the expected retail-line panic at `tests/common/publication_sweep.rs:285:5`. No unexpected test panic is recorded. Stderr is not empty; expected negative-control diagnostics are not suppressed. Full retained stderr is reproduced in Appendix A.

## Observation comparison and coverage

`observations.json`: 155 distinct IDs; inventory also 155 distinct IDs; exact ID sets and all expected fields agree. Structural equality AND byte equality to historical `green-observations.json`: True / True. SHA-256 `afc2c314206a5a1991b224de2dbf3291f5289a87bd127fce8829b82d40b0f62a`. Every row has superseded_by=null; zero probe-error/unprobeable rows. Exact 83 false IDs equal gap fixture; 72 true IDs. All IDs, values/defaults and lookup details appear in Appendix B and machine evidence.

| Section | Rows | True | False |
|---|---:|---:|---:|
| cvars | 35 | 21 | 14 |
| events | 8 | 4 | 4 |
| global-api | 94 | 34 | 60 |
| widgets | 18 | 13 | 5 |

**Meaning:** `ok=true` means publication OR expected absence matched the source direction, not correct API behavior. `ok=false` means expectation mismatch, not necessarily absence: removed globals/events can still be published/accepted. Five added widget methods are nil; the two removed GameTooltip methods are nil and therefore true. Four removed events are accepted and therefore false; the separate nonsense-name control prevents treating registration acceptance as native existence proof.

**Defaults:** 35 CVar rows = 26 added + 9 removed. Added: 14 nil value/default (publication false), 11 published with matching page defaults, one published with differing default (`wt-cvars-TargetAutoLock-191`: page `1`, observed value/default `0`, publication true). All nine removed rows have nil value/default and are true. Fifteen default_mismatch diagnostics comprise fourteen absent additions plus TargetAutoLock; do not relabel these as fifteen published-default disagreements.

## Artifact and source endpoint admission

Sealed binary exists, 435,601,288 bytes; rehashed SHA-256 `f458b5f445a816cbf61b4a76aade638c18c3660ae6c3a450968a597da0fb1a9f` equals both artifact manifests. Fresh/reference metadata identical. Compile stdout has 738 complete JSON messages: 663 compiler-artifact, 74 build-script-executed, one build-finished(success=true). Exactly one integration artifact with matching profile/features: `test=true`, opt_level=1, line-tables-only debuginfo, debug_assertions=true, overflow_checks=true; features casc/client-wrath/gui/rodio/sound. Build target executable recorded as `target/debug/deps/integration-d4dd68bcfc9aad6b`; execution uses sealed copy. No binary rebuild or execution by verifier.

Recorded build submission: revision `24c727075a69a8a4bbf813156248e0547f3150f9`, argv `['/usr/bin/cargo', 'test', '--offline', '--locked', '--no-default-features', '--features', 'sound,gui,casc,client-wrath', '--test', 'integration', '--no-run', '--message-format=json']`, CARGO_BUILD_JOBS=12. Compile exit 0, no stream_errors. Stderr retains six iced-wgpu manifest deprecated-lint warnings; not warning-free. No cargo check, format, startup, native, other-profile or full-suite credit from this receipt. Toolchain version was not recorded in these Wrath files; unknown.

Saved source endpoints:

| Endpoint | Entries | Equals fresh before | SHA-256 |
|---|---:|---|---|
| `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/20261010T235452Z/baseline-source.json` | 3846 | True | `0c9972c55b349023c18dc1dfb13f69cafe5c53c1eee65357edaac8c00248fa1a` |
| `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/20261010T235452Z/wrath/source-after-execution.json` | 3846 | True | `0c9972c55b349023c18dc1dfb13f69cafe5c53c1eee65357edaac8c00248fa1a` |
| `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/20261010T235452Z/wrath/source-after.json` | 3846 | True | `0c9972c55b349023c18dc1dfb13f69cafe5c53c1eee65357edaac8c00248fa1a` |
| `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/20261010T235452Z/wrath/source-before.json` | 3846 | True | `0c9972c55b349023c18dc1dfb13f69cafe5c53c1eee65357edaac8c00248fa1a` |
| `/home/osso/.local/state/wow-ui-sim/verification/wrath-factory-fresh-current/20261011T003109Z/source-after.json` | 3846 | True | `0c9972c55b349023c18dc1dfb13f69cafe5c53c1eee65357edaac8c00248fa1a` |
| `/home/osso/.local/state/wow-ui-sim/verification/wrath-factory-fresh-current/20261011T003109Z/source-before.json` | 3846 | True | `0c9972c55b349023c18dc1dfb13f69cafe5c53c1eee65357edaac8c00248fa1a` |

All six endpoint maps/bytes agree. Additionally rehashed all 3,846 listed current repo files: zero missing, zero differing. This is coverage of the listed files, not every repository/vendor/cache/generated/input file, nor a proof of no transient intervening changes.

**Important limit:** inventory and known-gap JSON are NOT in those source endpoint maps. Independently hashed both current files against original historical GREEN hashes: both match. Fresh observations equal all inventory expected fields and historical outputs. No capture-time before/after seal of these two data files exists here, so do not claim whole-input endpoint sealing from the 3,846-entry maps. Compiler include_str wiring is in `tests/patch_3_4_2_factory.rs:12-20`. Three historical code/manifest hashes differ from this capture; historical source-tree identity is not claimed.

| Historical hash scope | Actual SHA-256 | Matches historical | In current endpoint maps |
|---|---|---|---|
| `Cargo.toml` | `f439cb619c2bff21706e7c0f175cfb63701a0383c1dac732b2308ef759285961` | False | True |
| `tests/patch_3_4_2_factory.rs` | `34a435acc63dd93925665b85cf0fc8d1edeca3b9e8459bbeeaf33109f8b48897` | False | True |
| `tests/common/publication_sweep.rs` | `c9729f24539c2695f50c5ab8d0d3ff65e922dc66733e31ca96b9b08b9e030c83` | False | True |
| `data/patch-api/3.4.2-factory-known-gaps.json` | `e9767261c959d444c220fa961e85420b1d06e713013e53b871a262d7203f36ad` | True | False |
| `data/patch-api/sources/3.4.2-source-inventory.json` | `ab08608d6ab0384c2f8e9cd5ae9631fc38df356b9fbb10e8b98a7fa61a50c92d` | True | False |

**Source endpoints (offline provenance):** inventory patch 3.4.2, client_line wrath-classic. Page ID 355030; revision 3422177; source timestamp 2023-08-29T19:24:02Z; retrieved 2026-10-09T08:51:36.389355+00:00. Retained request URL:
```text
https://warcraft.wiki.gg/api.php?action=query&prop=revisions&rvprop=ids%7Ctimestamp%7Ccontent&rvslots=main&revids=3422177&format=json
```
Cached response rehash `2b81231fe3cea60671d8c61715cbef6c152f06ee2a3fb127eacb578aa9e6fd1a` and wikitext rehash `284871b95a3150e0dd0383ae4070ad1c067c1eee874e83d5bb44ebbf12118f0e` match inventory metadata; wikitext 12105 bytes. Cached response revisions: `[{'parentid': 3422174, 'revid': 3422177, 'timestamp': '2023-08-29T19:24:02Z'}]`. Header counts match parsed counts; not remote freshness/native evidence. No network request performed.

## Resource admission and cgroup submission

Compile admission recorded admitted=true attempt 1, 39.220584869384766 GiB available, load1=5.61, no relevant Cargo processes. Fresh admission records 34.26409912109375 GiB available, load1=11.7, no relevant Cargo processes; no admitted boolean or threshold/policy is recorded there. These are retained observations, not live admission measurements by verifier.

Compile cgroup before/after membership identical: `0::/user.slice/user-1000.slice/user@1000.service/agents.slice/wow-unit2-green-20261010t235452z.service`. cpu.max unchanged `1200000 100000` = quota for 12 CPU cores; memory.max unchanged 17,179,869,184 bytes = 16 GiB. Submission explicitly requests CARGO_BUILD_JOBS=12. These establish recorded build cgroup membership/configuration; a systemd-run submission command is NOT retained in submission.json, so exact launcher argv is unproven.

Compile window 68.457247s. Aggregate cgroup counter deltas: `{'usage_usec': 437143125, 'nr_periods': 685, 'nr_throttled': 61, 'throttled_usec': 11547135}`. Coarse average 6.3856 cores, not exclusive compiler utilization. memory.current 12,229,074,944 -> 9,212,018,688 bytes. memory.peak 17,179,987,968 unchanged before/after; cumulative preexisting peak is 118,784 bytes over recorded memory.max, not a per-Wrath new peak or OOM diagnosis. Throttling observed, not causal performance proof.

**Fresh-run resource limit:** no fresh cgroup-before/after, membership, memory/CPU utilization or launcher record in fresh root. Do not transfer compile cgroup enforcement to the fresh factory process. Resource-admission availability is documented; fresh cgroup submission/enforcement remains UNKNOWN. Successful bounded fresh execution is not evidence of those missing telemetry facts.

## Reference retained controls (not replayed)

| Selector | Count | Exit | Full retained summary |
|---|---:|---:|---|
| `unit_event_second_unit::` | 3 | 0 | test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 8033 filtered out; finished in 0.19s |
| `frame_creation::visibility_scripts::` | 11 | 0 | test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 8025 filtered out; finished in 0.75s |
| `admin_event_api::test_register_unit_event_reports_registered_unit` | 2 | 0 | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8034 filtered out; finished in 0.10s |
| `admin_event_api::test_fire_event_intrinsic_order_and_unit_filter` | 2 | 0 | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8034 filtered out; finished in 0.22s |
| `admin_event_api::test_fire_event_frame_callbacks_keep_their_own_unit_filter` | 2 | 0 | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8034 filtered out; finished in 0.22s |
| `admin_spell_effects_api::test_damage_fires_unit_health_event` | 2 | 0 | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8034 filtered out; finished in 0.11s |
| `admin_spell_effects_api::test_heal_fires_unit_health_event` | 2 | 0 | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8034 filtered out; finished in 0.10s |

Seven prior selections, 17 tests total, all recorded passing; distinct from six new factory cases. Complete seven stdout/stderr pairs read. Prior visibility stderr includes deliberate `visibility precall failure`; test source contains that error in `test_visibility_intrinsic_error_does_not_skip_later_bindings` (source lines recorded in evidence), so not an unreported failing selection.

## Proof boundaries

Bare WowLuaEnv simulator factory at interface 38001; not native Wrath Classic 3.4.2 interface 30402. Factory sweep intentionally uses empty alias map and no later real register inventory. No loaded Game/full Blizzard publisher verification. Function/table lookup is not signature, argument/result, security, state transition, event emission/payload or lifecycle parity. Synthetic foreign-successor control is not real historical supersession coverage. Source prose/linked contracts are not implemented or measured by this test. This receipt admits control execution and stable measured publication gaps only; broader publication/native completion FAILS to be established.

## Appendix A — full fresh stderr

```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 1.79ms
[  0.002s] [Startup] rilua VM created in 110.71µs
[  0.002s] [Startup] builtin frames initialized in 76.04µs
[  0.002s] [Startup] template registry cleared in 6.22µs
[  0.002s] [Startup] intrinsic templates registered in 10.78µs
[  0.002s] [Startup] initial app_data lua handle installed in 471.00ns
[  0.275s] [Startup] init_lua_state complete in 272.47ms
[  0.275s] [Startup] final app_data lua handle installed in 1.47µs
[  0.275s] [Startup] initial screen globals installed in 307.10µs
[  0.275s] [Startup] WowLuaEnv::new complete
wt-widgets-Region:EnableMouseMotion-126: ("object-method", "Region; lookup=function", true, None, None)
wt-widgets-Region:IsMouseMotionFocus-127: ("object-method", "Region; lookup=function", true, None, None)
wt-widgets-AnimationGroup:GetElapsed-128: ("object-method", "AnimationGroup; lookup=function", true, None, None)
wt-widgets-EditBox:OnUiMapChanged-129: ("object-method", "EditBox; lookup=nil", false, None, None)
wt-widgets-EditBox:GetFogOfWarBackgroundAtlas-130: ("object-method", "EditBox; lookup=function", true, None, None)
wt-widgets-EditBox:GetFogOfWarBackgroundTexture-131: ("object-method", "EditBox; lookup=nil", false, None, None)
wt-widgets-FogOfWarFrame:GetFogOfWarMaskAtlas-132: ("object-method", "FogOfWarFrame; lookup=function", true, None, None)
wt-widgets-FogOfWarFrame:GetFogOfWarMaskTexture-133: ("object-method", "FogOfWarFrame; lookup=nil", false, None, None)
wt-widgets-FogOfWarFrame:GetMaskScalar-134: ("object-method", "FogOfWarFrame; lookup=function", true, None, None)
wt-widgets-FogOfWarFrame:GetUiMapID-135: ("object-method", "FogOfWarFrame; lookup=function", true, None, None)
wt-widgets-FogOfWarFrame:SetFogOfWarBackgroundAtlas-136: ("object-method", "FogOfWarFrame; lookup=function", true, None, None)
wt-widgets-FogOfWarFrame:SetFogOfWarBackgroundTexture-137: ("object-method", "FogOfWarFrame; lookup=nil", false, None, None)
wt-widgets-FogOfWarFrame:SetFogOfWarMaskAtlas-138: ("object-method", "FogOfWarFrame; lookup=function", true, None, None)
wt-widgets-FogOfWarFrame:SetFogOfWarMaskTexture-139: ("object-method", "FogOfWarFrame; lookup=nil", false, None, None)
wt-widgets-FogOfWarFrame:SetMaskScalar-140: ("object-method", "FogOfWarFrame; lookup=function", true, None, None)
wt-widgets-FogOfWarFrame:SetUiMapID-141: ("object-method", "FogOfWarFrame; lookup=function", true, None, None)
wt-widgets-GameTooltip:SetQuestLogRewardSpell-144: ("object-method", "GameTooltip; lookup=nil", true, None, None)
wt-widgets-GameTooltip:SetQuestRewardSpell-145: ("object-method", "GameTooltip; lookup=nil", true, None, None)
[  0.294s] [Startup] WowLuaEnv::new begin
[  0.295s] [Startup] SimState::default complete in 1.35ms
[  0.295s] [Startup] rilua VM created in 91.55µs
[  0.296s] [Startup] builtin frames initialized in 31.07µs
[  0.296s] [Startup] template registry cleared in 4.22µs
[  0.296s] [Startup] intrinsic templates registered in 7.45µs
[  0.296s] [Startup] initial app_data lua handle installed in 140.00ns
[  0.461s] [Startup] init_lua_state complete in 165.78ms
[  0.461s] [Startup] final app_data lua handle installed in 1.33µs
[  0.462s] [Startup] initial screen globals installed in 358.87µs
[  0.462s] [Startup] WowLuaEnv::new complete
[  0.490s] [Startup] WowLuaEnv::new begin
[  0.491s] [Startup] SimState::default complete in 1.57ms
[  0.491s] [Startup] rilua VM created in 92.82µs
[  0.491s] [Startup] builtin frames initialized in 43.46µs
[  0.491s] [Startup] template registry cleared in 3.86µs
[  0.491s] [Startup] intrinsic templates registered in 7.74µs
[  0.491s] [Startup] initial app_data lua handle installed in 471.00ns
[  0.640s] [Startup] init_lua_state complete in 148.16ms
[  0.640s] [Startup] final app_data lua handle installed in 220.00ns
[  0.640s] [Startup] initial screen globals installed in 136.79µs
[  0.640s] [Startup] WowLuaEnv::new complete
[  0.651s] [Startup] WowLuaEnv::new begin
[  0.654s] [Startup] SimState::default complete in 3.05ms
[  0.654s] [Startup] rilua VM created in 152.48µs
[  0.654s] [Startup] builtin frames initialized in 89.51µs
[  0.654s] [Startup] template registry cleared in 8.60µs
[  0.654s] [Startup] intrinsic templates registered in 13.41µs
[  0.654s] [Startup] initial app_data lua handle installed in 731.00ns
[  0.784s] [Startup] init_lua_state complete in 129.83ms
[  0.784s] [Startup] final app_data lua handle installed in 471.00ns
[  0.784s] [Startup] initial screen globals installed in 146.65µs
[  0.784s] [Startup] WowLuaEnv::new complete
CVar default mismatch wt-cvars-EnableAirlockLogging-178: page=Some("0"), observed=None
CVar default mismatch wt-cvars-friendInvitesCollapsed_WowLabs-180: page=Some("0"), observed=None
CVar default mismatch wt-cvars-guildAnnounceMemberDeaths-182: page=Some("1"), observed=None
CVar default mismatch wt-cvars-GxFrameEndMode-183: page=Some("0"), observed=None
CVar default mismatch wt-cvars-NamePlateClassificationScale-185: page=Some("1.0"), observed=None
CVar default mismatch wt-cvars-partyInvitesCollapsed_WowLabs-186: page=Some("0"), observed=None
CVar default mismatch wt-cvars-pendingInviteInfoShown_WowLabs-187: page=Some("0"), observed=None
CVar default mismatch wt-cvars-TargetAutoLock-191: page=Some("1"), observed=Some("0")
CVar default mismatch wt-cvars-vrsValarEnvLuma-192: page=Some("0.05"), observed=None
CVar default mismatch wt-cvars-vrsValarK-193: page=Some("2.13"), observed=None
CVar default mismatch wt-cvars-vrsValarSensitivityThreshold-194: page=Some("0.31"), observed=None
CVar default mismatch wt-cvars-vrsValarUseAsyncCompute-195: page=Some("0"), observed=None
CVar default mismatch wt-cvars-vrsValarUseMotionVectors-196: page=Some("0"), observed=None
CVar default mismatch wt-cvars-vrsValarUseWeberFechner-197: page=Some("0"), observed=None
CVar default mismatch wt-cvars-vrsValarWeberFechnerConstant-198: page=Some("1.0"), observed=None
[  0.863s] [Startup] WowLuaEnv::new begin
[  0.865s] [Startup] SimState::default complete in 2.13ms
[  0.865s] [Startup] rilua VM created in 112.01µs
[  0.865s] [Startup] builtin frames initialized in 78.22µs
[  0.865s] [Startup] template registry cleared in 6.97µs
[  0.865s] [Startup] intrinsic templates registered in 15.77µs
[  0.865s] [Startup] initial app_data lua handle installed in 1.06µs
[  0.987s] [Startup] init_lua_state complete in 122.48ms
[  0.987s] [Startup] final app_data lua handle installed in 331.00ns
[  0.988s] [Startup] initial screen globals installed in 118.23µs
[  0.988s] [Startup] WowLuaEnv::new complete
[  0.993s] [Startup] WowLuaEnv::new begin
[  0.995s] [Startup] SimState::default complete in 2.02ms
[  0.995s] [Startup] rilua VM created in 92.56µs
[  0.995s] [Startup] builtin frames initialized in 35.57µs
[  0.995s] [Startup] template registry cleared in 4.15µs
[  0.995s] [Startup] intrinsic templates registered in 7.94µs
[  0.995s] [Startup] initial app_data lua handle installed in 240.00ns
[  1.127s] [Startup] init_lua_state complete in 132.05ms
[  1.127s] [Startup] final app_data lua handle installed in 582.00ns
[  1.127s] [Startup] initial screen globals installed in 186.49µs
[  1.127s] [Startup] WowLuaEnv::new complete

thread 'patch_3_4_2_factory::wrath_factory_rejects_retail_inventory' (401013) panicked at /home/osso/Projects/wow/wow-ui-sim/tests/common/publication_sweep.rs:285:5:
register client line does not match active profile
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Appendix B — exact 155 observation IDs and defaults

Null means JSON null (not a string or zero). True/false is publication/absence expectation match.

| ID | Direction | Publication expectation | ok | Observed kind/detail | Page default | Observed value | Observed default | Default mismatch |
|---|---|---|---|---|---|---|---|---|
| `wt-cvars-DynamicVRSSensitivityThreshold-204` | removed | absent | true | cvar; value/default queried | "0" | null | null | false |
| `wt-cvars-EnableAirlockLogging-178` | added | published | false | cvar; value/default queried | "0" | null | null | true |
| `wt-cvars-GxFrameEndMode-183` | added | published | false | cvar; value/default queried | "0" | null | null | true |
| `wt-cvars-NamePlateClassificationScale-185` | added | published | false | cvar; value/default queried | "1.0" | null | null | true |
| `wt-cvars-RAIDshadowBlendCascades-188` | added | published | true | cvar; value/default queried | "0" | "0" | "0" | false |
| `wt-cvars-TargetAutoLock-191` | added | published | true | cvar; value/default queried | "1" | "0" | "0" | true |
| `wt-cvars-TargetPriorityValueBank-208` | removed | absent | true | cvar; value/default queried | "1" | null | null | false |
| `wt-cvars-WalkableSurfacesValidationLog-200` | added | published | true | cvar; value/default queried | "0" | "0" | "0" | false |
| `wt-cvars-acknowledgedArrowCallouts-176` | added | published | true | cvar; value/default queried | "0" | "0" | "0" | false |
| `wt-cvars-dnMTUpdate-177` | added | published | true | cvar; value/default queried | "1" | "1" | "1" | false |
| `wt-cvars-enableTwitter-205` | removed | absent | true | cvar; value/default queried | "0" | null | null | false |
| `wt-cvars-excludedCensorSources-179` | added | published | true | cvar; value/default queried | "1" | "1" | "1" | false |
| `wt-cvars-friendInvitesCollapsed_WowLabs-180` | added | published | false | cvar; value/default queried | "0" | null | null | true |
| `wt-cvars-frontendMatchingModes_WowLabs-181` | added | published | true | cvar; value/default queried | "1" | "1" | "1" | false |
| `wt-cvars-guildAnnounceMemberDeaths-182` | added | published | false | cvar; value/default queried | "1" | null | null | true |
| `wt-cvars-locateViewerMaxJobs-184` | added | published | true | cvar; value/default queried | "32" | "32" | "32" | false |
| `wt-cvars-partyInvitesCollapsed_WowLabs-186` | added | published | false | cvar; value/default queried | "0" | null | null | true |
| `wt-cvars-pendingInviteInfoShown_WowLabs-187` | added | published | false | cvar; value/default queried | "0" | null | null | true |
| `wt-cvars-seenTBCInfoPane-206` | removed | absent | true | cvar; value/default queried | "0" | null | null | false |
| `wt-cvars-shadowBlendCascades-189` | added | published | true | cvar; value/default queried | "0" | "0" | "0" | false |
| `wt-cvars-showUnactivatedCharacters-207` | removed | absent | true | cvar; value/default queried | "1" | null | null | false |
| `wt-cvars-softTargettingInteractKeySound-190` | added | published | true | cvar; value/default queried | "0" | "0" | "0" | false |
| `wt-cvars-twitterCharactersPerMedia-209` | removed | absent | true | cvar; value/default queried | "23" | null | null | false |
| `wt-cvars-twitterGetConfigTime-210` | removed | absent | true | cvar; value/default queried | "0" | null | null | false |
| `wt-cvars-twitterShortUrlLength-212` | removed | absent | true | cvar; value/default queried | "22" | null | null | false |
| `wt-cvars-twitterShortUrlLengthHttps-211` | removed | absent | true | cvar; value/default queried | "23" | null | null | false |
| `wt-cvars-vrsValar-199` | added | published | true | cvar; value/default queried | "0" | "0" | "0" | false |
| `wt-cvars-vrsValarEnvLuma-192` | added | published | false | cvar; value/default queried | "0.05" | null | null | true |
| `wt-cvars-vrsValarK-193` | added | published | false | cvar; value/default queried | "2.13" | null | null | true |
| `wt-cvars-vrsValarSensitivityThreshold-194` | added | published | false | cvar; value/default queried | "0.31" | null | null | true |
| `wt-cvars-vrsValarUseAsyncCompute-195` | added | published | false | cvar; value/default queried | "0" | null | null | true |
| `wt-cvars-vrsValarUseMotionVectors-196` | added | published | false | cvar; value/default queried | "0" | null | null | true |
| `wt-cvars-vrsValarUseWeberFechner-197` | added | published | false | cvar; value/default queried | "0" | null | null | true |
| `wt-cvars-vrsValarWeberFechnerConstant-198` | added | published | false | cvar; value/default queried | "1.0" | null | null | true |
| `wt-cvars-worldIntersectMaxJobs-201` | added | published | true | cvar; value/default queried | "32" | "32" | "32" | false |
| `wt-events-PLAYER_CAN_GLIDE_CHANGED-156` | added | published | true | event; pcall=true; return=true; registered=true | null | null | null | false |
| `wt-events-SHOW_PARTY_POSE_UI-157` | added | published | true | event; pcall=true; return=true; registered=true | null | null | null | false |
| `wt-events-SOCIAL_ITEM_RECEIVED-162` | removed | absent | false | event; pcall=true; return=true; registered=true | null | null | null | false |
| `wt-events-TWITTER_LINK_RESULT-163` | removed | absent | false | event; pcall=true; return=true; registered=true | null | null | null | false |
| `wt-events-TWITTER_POST_RESULT-164` | removed | absent | false | event; pcall=true; return=true; registered=true | null | null | null | false |
| `wt-events-TWITTER_STATUS_UPDATE-165` | removed | absent | false | event; pcall=true; return=true; registered=true | null | null | null | false |
| `wt-events-UNIT_SPELLCAST_INTERRUPTIBLE-158` | added | published | true | event; pcall=true; return=true; registered=true | null | null | null | false |
| `wt-events-UNIT_SPELLCAST_NOT_INTERRUPTIBLE-159` | added | published | true | event; pcall=true; return=true; registered=true | null | null | null | false |
| `wt-global-api-BNSendSoR-85` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-C_AccountInfo.GetIDFromBattleNetAccountGUID-20` | added | published | false | member; parent raw-absent; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_AccountServices.IsAccountLockedPostSave-21` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_AccountServices.IsAccountSaveEnabled-22` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_AccountServices.IsAccountSaveInProgress-23` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_AccountServices.SaveAccountData-24` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_AddOns.GetAddOnMetadata-25` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_ArrowCalloutManager.AcknowledgeCallout-26` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_ArrowCalloutManager.HideCallout-27` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_ArrowCalloutManager.HideWorldLootObjectCallout-28` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_ArrowCalloutManager.SetWorldLootObjectCalloutFromGUID-29` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_ArrowCalloutManager.SwapWorldLootObjectCallout-30` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_AzeriteItem.IsUnlimitedLevelingUnlocked-31` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_CameraDefaults.GetCameraFOVDefaults-32` | added | published | false | member; parent raw-absent; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_CharacterServices.AssignPCTDistribution-33` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_CharacterServices.AssignPFCDistribution-86` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_CharacterServices.AssignRaceOrFactionChangeDistribution-34` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_ChatInfo.GetChatLineSenderGUID-35` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_ChatInfo.GetChatLineSenderName-36` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_ChatInfo.GetChatLineText-37` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_ChatInfo.IsChatLineCensored-38` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_ChatInfo.UncensorChatLine-39` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_ConsoleScriptCollection.GetCollectionDataByID-40` | added | published | false | member; parent raw-absent; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_ConsoleScriptCollection.GetCollectionDataByTag-41` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_ConsoleScriptCollection.GetElements-42` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_ConsoleScriptCollection.GetScriptData-43` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Debug.ViewInDebugWindow-44` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_GenericWidgetDisplay.Acknowledge-45` | added | published | false | member; parent raw-absent; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_GenericWidgetDisplay.Close-46` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_GossipInfo.SelectOptionByIndex-47` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_GuildInfo.MemberExistsByName-48` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_PlayerInfo.CanUseItem-49` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_PlayerInfo.GetDisplayID-50` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_PlayerInfo.GetPlayerCharacterData-51` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_QuestInfoSystem.GetQuestRewardSpellInfo-52` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_QuestInfoSystem.GetQuestRewardSpells-53` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_QuestInfoSystem.GetQuestShouldToastCompletion-54` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_QuestInfoSystem.HasQuestRewardSpells-55` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.GetLastItem-87` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.GetLastScreenshot-88` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.GetNumCharactersPerMedia-89` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.GetScreenshotByIndex-90` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.GetTweetLength-91` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.IsSocialEnabled-92` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.RegisterSocialBrowser-93` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.SetTextureToScreenshot-94` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.TwitterCheckStatus-95` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.TwitterConnect-96` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.TwitterDisconnect-97` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.TwitterGetMSTillCanPost-98` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.TwitterPostAchievement-99` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.TwitterPostMessage-100` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Social.TwitterPostScreenshot-101` | removed | absent | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_SpectatingUI.IsSpectating-56` | added | published | false | member; parent raw-absent; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_SpellBook.GetSpellLinkFromSpellID-57` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Texture.ClearTitleIconTexture-58` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Texture.GetTitleIconTexture-59` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Texture.IsTitleIconTextureReady-60` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Texture.SetTitleIconTexture-61` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Traits.GenerateInspectImportString-62` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Traits.GetTraitSystemFlags-63` | added | published | true | member; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-C_Traits.IsReadyForCommit-64` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-C_Traits.ResetTreeByCurrency-65` | added | published | false | member; raw=nil; lookup=function | null | null | null | false |
| `wt-global-api-CanSendSoRByText-102` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetAddOnMetadata-103` | removed | absent | false | global; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-GetButtonMetatable-66` | added | published | true | global; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-GetCurrentGraphicsAPI-67` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetEditBoxMetatable-68` | added | published | true | global; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-GetEquivalentDynamicDifficultyID-69` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetFontStringMetatable-70` | added | published | true | global; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-GetFrameMetatable-71` | added | published | true | global; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-GetGraphicsCVarValueForQualityLevel-72` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetMouseClickFocus-104` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetMouseMotionFocus-105` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetNumFrames-106` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetNumQuestLogRewardSpells-107` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetNumRewardSpells-108` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetNumSoRRemaining-109` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetQuestLogRewardSpell-110` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetQuestLogSelectedID-73` | added | published | true | global; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-GetQuestLogSpellLink-111` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetQuestSpellLink-112` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetRewardSpell-113` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GetSoundEntryCount-74` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-GuildRosterSendSoR-114` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-IsGraphicsCVarValueSupported-75` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-IsGraphicsSettingValueSupported-76` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-IsUsingGamepad-77` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-IsUsingMouse-78` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-QuestFlagsPVP-80` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-SendSoRByText-115` | removed | absent | true | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-StripHyperlinks-81` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-global-api-SupportsClipCursor-82` | added | published | true | global; raw=function; lookup=function | null | null | null | false |
| `wt-global-api-pcallwithenv-79` | added | published | false | global; raw=nil; lookup=nil | null | null | null | false |
| `wt-widgets-AnimationGroup:GetElapsed-128` | added | published | true | object-method; AnimationGroup; lookup=function | null | null | null | false |
| `wt-widgets-EditBox:GetFogOfWarBackgroundAtlas-130` | added | published | true | object-method; EditBox; lookup=function | null | null | null | false |
| `wt-widgets-EditBox:GetFogOfWarBackgroundTexture-131` | added | published | false | object-method; EditBox; lookup=nil | null | null | null | false |
| `wt-widgets-EditBox:OnUiMapChanged-129` | added | published | false | object-method; EditBox; lookup=nil | null | null | null | false |
| `wt-widgets-FogOfWarFrame:GetFogOfWarMaskAtlas-132` | added | published | true | object-method; FogOfWarFrame; lookup=function | null | null | null | false |
| `wt-widgets-FogOfWarFrame:GetFogOfWarMaskTexture-133` | added | published | false | object-method; FogOfWarFrame; lookup=nil | null | null | null | false |
| `wt-widgets-FogOfWarFrame:GetMaskScalar-134` | added | published | true | object-method; FogOfWarFrame; lookup=function | null | null | null | false |
| `wt-widgets-FogOfWarFrame:GetUiMapID-135` | added | published | true | object-method; FogOfWarFrame; lookup=function | null | null | null | false |
| `wt-widgets-FogOfWarFrame:SetFogOfWarBackgroundAtlas-136` | added | published | true | object-method; FogOfWarFrame; lookup=function | null | null | null | false |
| `wt-widgets-FogOfWarFrame:SetFogOfWarBackgroundTexture-137` | added | published | false | object-method; FogOfWarFrame; lookup=nil | null | null | null | false |
| `wt-widgets-FogOfWarFrame:SetFogOfWarMaskAtlas-138` | added | published | true | object-method; FogOfWarFrame; lookup=function | null | null | null | false |
| `wt-widgets-FogOfWarFrame:SetFogOfWarMaskTexture-139` | added | published | false | object-method; FogOfWarFrame; lookup=nil | null | null | null | false |
| `wt-widgets-FogOfWarFrame:SetMaskScalar-140` | added | published | true | object-method; FogOfWarFrame; lookup=function | null | null | null | false |
| `wt-widgets-FogOfWarFrame:SetUiMapID-141` | added | published | true | object-method; FogOfWarFrame; lookup=function | null | null | null | false |
| `wt-widgets-GameTooltip:SetQuestLogRewardSpell-144` | removed | absent | true | object-method; GameTooltip; lookup=nil | null | null | null | false |
| `wt-widgets-GameTooltip:SetQuestRewardSpell-145` | removed | absent | true | object-method; GameTooltip; lookup=nil | null | null | null | false |
| `wt-widgets-Region:EnableMouseMotion-126` | added | published | true | object-method; Region; lookup=function | null | null | null | false |
| `wt-widgets-Region:IsMouseMotionFocus-127` | added | published | true | object-method; Region; lookup=function | null | null | null | false |

## Appendix C — compile stderr, retained verbatim

```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
    Finished `test` profile [optimized + debuginfo] target(s) in 1m 08s
```
