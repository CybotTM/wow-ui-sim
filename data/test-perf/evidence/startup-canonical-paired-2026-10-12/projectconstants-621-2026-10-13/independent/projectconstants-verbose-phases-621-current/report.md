# Source621 ProjectConstants saved-artifact audit

## Verdict and scope
PASS: saved output audited fully; normal startup reports zero observed Lua errors. NOT a clean compatibility surface: 185 nil observations and 88 missing requirements. Prior timed-fixture performance failure remains FAIL; cause unresolved.

Read/followed `<HOME>/AgentConfig/skills/verify/SKILL.md` as the assigned saved-artifact verifier; no delegation. No new runtime, build, binary read/hash, tracked edit, fix, or threshold change. Only private proof files written. Revision from saved metadata: `621649e56d8418d21c4539b26d787ad40c73431f` (retail). Source semantics read from the saved epoch `source/`, not current checkout.

Artifact: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-removal-green-20261013T193523870575Z/runtime-projectconstants-verbose-phases`.
Command: saved normal `wow-sim --no-addons --no-saved-vars lua-errors`; `WOW_SIM_VERBOSE=1`, `WOW_SIM_CASC=0`, `WOW_SIM_NO_SOUND=1`; systemd wait + timeout/bwrap argv retained verbatim in proof. No claim that this reproduces native CASC/audio runtime.

## Complete output classification
Read all 173324 stderr bytes / 2663 lines through contiguous bounded reads (no clipped body). Stdout exactly `[]\n` (3 bytes). Saved launcher exit 0; full wait result:

```text
Running as unit: &amp;lt;PRIVATE_INVOCATION_06&amp;gt;.service; invocation ID: e11a4e2f678e42f7bd287dc304017591
          Finished with result: success
Main processes terminated with: code=exited, status=0/SUCCESS
               Service runtime: 10.404s
             CPU time consumed: 8.166s
                   Memory peak: 2.1G (swap: 0B)
```

291 addon status records: 1573 Lua and 677 XML reported; zero reported warnings. Nil observations: 185 across 38 addon records. Missing requirements: 88 across 26 records. These are sums of loader diagnostic vector lengths, not counts of globally unique symbols or runtime error occurrences. Explicit load failures, failure/warning detail lines, panics, instruction-budget success/error messages, and timeout diagnostics: none in this normal artifact. Names such as ScriptErrors/UIErrorsFrame are file/addon names, not failures; `seterrorhandler restored` is informational. Cache `replay_fail=0` / `store_fail=0` are zero counters, not failures.

Final exact summary:
```text
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```

Source `src/bin/wow_sim/addon_loading.rs:156–194` prints counts independently of final Lua errors; detailed `[failure]`, `[nil-observation]`, `[missing-requirement]` text requires `WOW_SIM_DEBUG_NIL_GLOBALS`. No such detail text exists here. Therefore symbol names/attributions behind the 185/88 counts are unknown from this saved output; no invented diagnostics and no claim of zero missing APIs. `src/loader/addon/nil_symbol_reports.rs:90–128,179–201` deduplicates per environment/kind within the addon, filters resolved ordinary globals, classifies C_* namespaces/methods as missing requirements and other nil accesses as observations. Neither category is automatically a thrown Lua error.

## Exact ProjectConstants phase/frame evidence
stderr:42:
```text
Blizzard_ProjectConstants loaded: 1 Lua, 0 XML, 0 warnings, 0 nil observations, 0 missing requirements (737.2ms: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=736.2ms [compile=736.1ms call=63.2µs] frames=0)
```

| Field | Observed |
|---|---|
| independently counted total | 737.2ms |
| xmlproc / frame exec_lua / lifecycle / layers | 0.0ns / 0.0ns / 0.0ns / 0.0ns |
| Lua total | 736.2ms |
| compile/cache-load elapsed | 736.1ms |
| call elapsed | 63.2µs |
| Lua / XML / frames | 1 / 0 / 0 |
| warnings / nil observations / missing requirements | 0 / 0 / 0 |

`LoadTiming::total()` (`src/loader/mod.rs:371–381`) sums I/O + XML parse + XML processing + Lua compile/call + SavedVariables. It is not an outer addon wall stopwatch or exclusive CPU measurement. The printed rounded total exceeds printed Lua time by roughly 1.0ms; exact unprinted I/O/SV components cannot be recovered from this line. `lua_file.rs:138–185` times `load_cached_or_compile_for_chunk` and function preparation/call with `Instant::elapsed`, accumulating compile+call into Lua total. “compile” includes content-key work, cache lookup/initialization and bytecode reload on hits—not proof of source compilation. `bytecode_cache.rs:193–287` can load/index the pack lazily inside lookup; this establishes possible attribution scope, not the measured cause of 736.1ms. XML processing subtracts already-counted child buckets (`xml_file.rs:56–73`); frame subfields are not extra totals. Zero XML/frame fields exclude measured frame construction as this addon's observed dominant bucket.

Exact aggregate cache/frame context (not a per-file causal proof):
```text
[  6.345s] Blizzard addons loaded in 5.51s (io=351.46ms xml=467.53ms xmlproc=2.96s frames⊂xmlproc=2.94s lua=1.17s [compile=989.59ms call=182.68ms] gc=deferred, bytecode cache: 1569/1569 hits (100%, lookup_miss=0 replay_fail=0 stored=0 store_fail=0))
  frame breakdown: setup=1.44s finalize=3.34s (2482 frames)
  setup: code_build=17.87ms exec_lua=1.42s props=16.03ms
  finalize: layers=93.38ms (1151 tex, 689 fs) anim=25.02ms button=53.12ms lifecycle=1.25s (1761 fires)
```
1569/1569 cache hits and zero lookup misses/replay/store failures describe this run's aggregate cached-file path; 1573 reported Lua files is a different loader count. Do not infer identical cache residency, pack initialization timing, host scheduling, or VM/preloader state in the prior fixture.

## Prior 1.42s fixture: distinction only
Saved `runtime-perf-owner-localization/result.json`: exit 101, one failed test; parent/child repeated summaries are not two independent test cases. Exact stderr retained below and in proof:
```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.059s] [Startup] SimState::default complete in 55.65ms
[  0.061s] [Startup] rilua VM created in 1.83ms
[  0.062s] [Startup] builtin frames initialized in 1.04ms
[  0.062s] [Startup] template registry cleared in 218.88µs
[  0.062s] [Startup] intrinsic templates registered in 149.21µs
[  0.062s] [Startup] initial app_data lua handle installed in 158.99µs
[  0.419s] [Startup] init_lua_state complete in 356.90ms
[  0.419s] [Startup] final app_data lua handle installed in 781.00ns
[  0.419s] [Startup] initial screen globals installed in 235.37µs
[  0.419s] [Startup] WowLuaEnv::new complete
heaviest Blizzard addon loads: Blizzard_ProjectConstants=1.42s, Blizzard_UIPanels_Game=485.19ms, Blizzard_ActionBar=480.05ms, Blizzard_GroupFinder=360.31ms, Blizzard_CompactRaidFrames=335.43ms, Blizzard_UnitFrame=239.34ms, Blizzard_SharedXML=179.57ms, Blizzard_Communities=166.22ms

thread 'addon_loading_perf::heaviest_blizzard_addons_stay_within_per_addon_load_budgets' (1745203) panicked at <REPO>/tests/addon_loading_perf.rs:167:13:
heaviest Blizzard addon loading regressed:
Blizzard_ProjectConstants became a new heavy-addon outlier at 1.42s; add an explicit budget entry
heaviest: [Blizzard_ProjectConstants=1.42s, Blizzard_UIPanels_Game=485.19ms, Blizzard_ActionBar=480.05ms, Blizzard_GroupFinder=360.31ms, Blizzard_CompactRaidFrames=335.43ms, Blizzard_UnitFrame=239.34ms, Blizzard_SharedXML=179.57ms, Blizzard_Communities=166.22ms]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'addon_loading_perf::heaviest_blizzard_addons_stay_within_per_addon_load_budgets' (1745197) panicked at <REPO>/tests/common/timeout_reexec.rs:184:9:
timeout child for `addon_loading_perf::heaviest_blizzard_addons_stay_within_per_addon_load_budgets` failed (exit code 101)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
The failed test calls `load_timed_game_addons_with_saved_vars()` (`tests/addon_loading_perf.rs:150–152`); `tests/perf/addon_loading.rs:26–71` creates a SavedVariables manager and loads each discovered addon with it, storing `result.timing.total()`. `tests/perf/base_game.rs:6–20` builds its game environment at 1024×768. The normal executable follows startup discovery/bootstrap at 1600×1200, disables SavedVariables loading, and still explicitly loads the read-only EditMode cache from WTF (stderr:24–36). No equivalence of caches, scheduling, preloader state, or SavedVariables conditions is established. 737.2ms normal total versus 1.42s fixture total is descriptive only: no same-cause claim, performance-test pass, threshold repair, or regression resolution.

## TOP20 emitter reachability
Global “emitter absent from source” finding REJECTED. Saved source contains `src/bin/wow_sim/addon_loading/lua_file_profile.rs:1–22`, which prints `Slowest Lua files:` and `.take(20)`, sorted by compile+call elapsed. It is invoked by `print_load_summary` (`addon_loading.rs:704–714`). This normal no-addons/non-test path returns at `load_third_party_addons` (`addon_loading.rs:253–255`) before that call; stderr:2574 confirms `Addon loading disabled`. Thus missing TOP20 output is expected here, not globally absent instrumentation. Recording separately requires `WOW_SIM_PROFILE_LUA_FILES` (`loader/mod.rs:75–126`); WOW_SIM_VERBOSE alone does not enable it. No new run/flag change performed.

## Receipt reuse / preservation
Recorded binary stat tuple matches prior receipt and current artifact stat: `{"st_dev": 66306, "st_ino": 142479845, "st_mtime_ns": 1791920284944505512, "st_size": 408320640}`. Stat-only reuse; zero new hash reads, no binary content read. This is metadata continuity, not fresh cryptographic identity proof. Full actual stderr, prior failure stderr, stdout, argv, launcher, and saved-source excerpts are preserved unredacted in private `proof.json`; no public staging requested.

## All nonzero diagnostic status lines (exact)
Counts are compatibility diagnostics, not hidden runtime failures. This appendix includes every status with nonzero nil observations or missing requirements:
```text
56: Blizzard_Fonts_Shared loaded: 2 Lua, 11 XML, 0 warnings, 4 nil observations, 0 missing requirements (28.4ms: xmlproc=13.9ms exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=542.3µs [compile=291.9µs call=250.4µs] frames=0)
139: Blizzard_PrintHandler loaded: 1 Lua, 0 XML, 0 warnings, 1 nil observations, 0 missing requirements (282.2µs: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=138.2µs [compile=42.0µs call=96.2µs] frames=0)
169: Blizzard_Narration loaded: 6 Lua, 0 XML, 0 warnings, 7 nil observations, 0 missing requirements (1.3ms: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=805.7µs [compile=303.2µs call=502.4µs] frames=0)
601: Blizzard_SharedXML loaded: 294 Lua, 73 XML, 0 warnings, 7 nil observations, 1 missing requirements (135.3ms: xmlproc=23.8ms exec_lua=14.2ms lifecycle=1.8ms layers=318.4µs lua=65.3ms [compile=29.3ms call=35.9ms] frames=16)
624: Blizzard_FrameXMLBase loaded: 10 Lua, 3 XML, 0 warnings, 1 nil observations, 0 missing requirements (4.8ms: xmlproc=117.3µs exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=2.4ms [compile=967.4µs call=1.4ms] frames=0)
726: Blizzard_FrameXMLUtil loaded: 80 Lua, 2 XML, 0 warnings, 4 nil observations, 3 missing requirements (13.4ms: xmlproc=38.2µs exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=9.1ms [compile=5.7ms call=3.5ms] frames=0)
760: Blizzard_EditMode loaded: 13 Lua, 4 XML, 0 warnings, 7 nil observations, 0 missing requirements (48.5ms: xmlproc=36.7ms exec_lua=24.8ms lifecycle=8.7ms layers=682.7µs lua=6.3ms [compile=3.9ms call=2.4ms] frames=66)
774: Blizzard_GameTooltip loaded: 6 Lua, 1 XML, 0 warnings, 2 nil observations, 0 missing requirements (40.4ms: xmlproc=36.9ms exec_lua=34.4ms lifecycle=1.9ms layers=63.5µs lua=2.2ms [compile=1.4ms call=831.2µs] frames=9)
795: Blizzard_PrivateAurasUI loaded: 4 Lua, 2 XML, 0 warnings, 1 nil observations, 1 missing requirements (6.7ms: xmlproc=3.5ms exec_lua=2.0ms lifecycle=1.3ms layers=2.7µs lua=2.1ms [compile=845.2µs call=1.3ms] frames=4)
925: Blizzard_StaticPopup_Game loaded: 5 Lua, 2 XML, 0 warnings, 2 nil observations, 1 missing requirements (31.0ms: xmlproc=25.1ms exec_lua=23.6ms lifecycle=665.4µs layers=175.9µs lua=3.8ms [compile=2.9ms call=949.2µs] frames=8)
1107: Blizzard_FrameXML loaded: 59 Lua, 50 XML, 0 warnings, 5 nil observations, 5 missing requirements (162.5ms: xmlproc=95.3ms exec_lua=51.3ms lifecycle=14.9ms layers=12.6ms lua=13.9ms [compile=8.3ms call=5.6ms] frames=151)
1120: Blizzard_StoreUI loaded: 14 Lua, 3 XML, 0 warnings, 1 nil observations, 0 missing requirements (116.0ms: xmlproc=104.3ms exec_lua=18.8ms lifecycle=80.2ms layers=2.0ms lua=4.3ms [compile=2.9ms call=1.4ms] frames=64)
1216: Blizzard_UIPanels_Game loaded: 48 Lua, 38 XML, 0 warnings, 5 nil observations, 2 missing requirements (442.7ms: xmlproc=362.1ms exec_lua=270.7ms lifecycle=57.5ms layers=9.9ms lua=17.7ms [compile=12.1ms call=5.7ms] frames=344)
1224: Blizzard_GuildControlUI loaded: 3 Lua, 1 XML, 0 warnings, 1 nil observations, 1 missing requirements (15.6ms: xmlproc=12.2ms exec_lua=6.6ms lifecycle=3.3ms layers=843.9µs lua=566.9µs [compile=362.9µs call=204.1µs] frames=33)
1257: Blizzard_ChatFrameBase loaded: 27 Lua, 3 XML, 0 warnings, 2 nil observations, 2 missing requirements (85.4ms: xmlproc=62.2ms exec_lua=40.4ms lifecycle=20.7ms layers=46.3µs lua=15.9ms [compile=4.4ms call=11.5ms] frames=23)
1282: Blizzard_ChatFrame loaded: 12 Lua, 9 XML, 0 warnings, 7 nil observations, 1 missing requirements (52.5ms: xmlproc=36.0ms exec_lua=22.2ms lifecycle=8.8ms layers=875.0µs lua=5.1ms [compile=3.3ms call=1.8ms] frames=97)
1312: Blizzard_Communities loaded: 26 Lua, 22 XML, 0 warnings, 1 nil observations, 0 missing requirements (162.9ms: xmlproc=134.5ms exec_lua=99.2ms lifecycle=26.9ms layers=2.6ms lua=7.2ms [compile=5.6ms call=1.6ms] frames=101)
1319: Blizzard_MicroMenu loaded: 4 Lua, 2 XML, 0 warnings, 4 nil observations, 0 missing requirements (16.0ms: xmlproc=10.4ms exec_lua=5.2ms lifecycle=3.6ms layers=942.1µs lua=1.4ms [compile=1.1ms call=233.1µs] frames=22)
1373: Blizzard_ActionBar loaded: 28 Lua, 20 XML, 0 warnings, 12 nil observations, 2 missing requirements (436.3ms: xmlproc=409.6ms exec_lua=21.2ms lifecycle=380.5ms layers=482.2µs lua=4.8ms [compile=3.1ms call=1.6ms] frames=27)
1459: Blizzard_UnitFrame loaded: 46 Lua, 37 XML, 0 warnings, 11 nil observations, 1 missing requirements (293.6ms: xmlproc=240.3ms exec_lua=82.4ms lifecycle=148.5ms layers=1.9ms lua=12.0ms [compile=7.2ms call=4.8ms] frames=83)
1475: Blizzard_EnvironmentCleanup loaded: 1 Lua, 0 XML, 0 warnings, 3 nil observations, 0 missing requirements (1.4ms: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=642.0µs [compile=465.5µs call=176.5µs] frames=0)
1480: Blizzard_TimeManager loaded: 3 Lua, 1 XML, 0 warnings, 2 nil observations, 0 missing requirements (13.5ms: xmlproc=8.3ms exec_lua=3.7ms lifecycle=2.9ms layers=855.8µs lua=1.2ms [compile=418.8µs call=746.0µs] frames=19)
1501: Blizzard_BoostTutorial loaded: 11 Lua, 4 XML, 0 warnings, 1 nil observations, 2 missing requirements (12.7ms: xmlproc=4.4ms exec_lua=2.2ms lifecycle=444.3µs layers=1.2ms lua=4.0ms [compile=1.7ms call=2.3ms] frames=5)
1518: Blizzard_SecureTransferUI loaded: 2 Lua, 1 XML, 0 warnings, 2 nil observations, 0 missing requirements (7.8ms: xmlproc=6.3ms exec_lua=4.8ms lifecycle=216.9µs layers=419.3µs lua=442.8µs [compile=262.1µs call=180.7µs] frames=7)
1535: Blizzard_AccountStore loaded: 6 Lua, 5 XML, 0 warnings, 1 nil observations, 0 missing requirements (17.6ms: xmlproc=12.1ms exec_lua=9.1ms lifecycle=1.8ms layers=234.1µs lua=1.3ms [compile=732.6µs call=530.9µs] frames=10)
1783: Blizzard_UIWidgets loaded: 40 Lua, 39 XML, 0 warnings, 2 nil observations, 0 missing requirements (99.5ms: xmlproc=4.6ms exec_lua=2.9ms lifecycle=525.0µs layers=6.1µs lua=7.0ms [compile=3.9ms call=3.1ms] frames=5)
1824: Blizzard_ObjectiveTracker loaded: 20 Lua, 20 XML, 0 warnings, 2 nil observations, 0 missing requirements (50.1ms: xmlproc=23.6ms exec_lua=16.3ms lifecycle=3.0ms layers=1.6ms lua=5.8ms [compile=3.8ms call=2.1ms] frames=37)
1874: Blizzard_ClientSceneVisManager loaded: 1 Lua, 1 XML, 0 warnings, 0 nil observations, 2 missing requirements (691.5µs: xmlproc=286.2µs exec_lua=135.7µs lifecycle=121.9µs layers=942.0ns lua=44.8µs [compile=24.3µs call=20.4µs] frames=1)
1879: Blizzard_ContentTracking loaded: 2 Lua, 1 XML, 0 warnings, 0 nil observations, 1 missing requirements (1.7ms: xmlproc=48.6µs exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=643.7µs [compile=215.1µs call=428.6µs] frames=0)
1895: Blizzard_CombatAudioAlerts loaded: 1 Lua, 1 XML, 0 warnings, 0 nil observations, 3 missing requirements (3.6ms: xmlproc=1.4ms exec_lua=257.0µs lifecycle=1.0ms layers=4.2µs lua=1.7ms [compile=1.6ms call=158.2µs] frames=1)
1932: Blizzard_FriendsFrame loaded: 7 Lua, 4 XML, 0 warnings, 7 nil observations, 0 missing requirements (114.6ms: xmlproc=98.1ms exec_lua=73.1ms lifecycle=14.8ms layers=5.7ms lua=7.0ms [compile=4.5ms call=2.5ms] frames=53)
1939: Blizzard_RaidFrame loaded: 3 Lua, 3 XML, 0 warnings, 2 nil observations, 0 missing requirements (19.3ms: xmlproc=14.4ms exec_lua=7.3ms lifecycle=5.8ms layers=97.5µs lua=1.1ms [compile=585.0µs call=561.9µs] frames=21)
1946: Blizzard_CompactRaidFrames loaded: 4 Lua, 2 XML, 0 warnings, 1 nil observations, 2 missing requirements (263.2ms: xmlproc=258.4ms exec_lua=5.3ms lifecycle=250.6ms layers=786.4µs lua=1.7ms [compile=1.0ms call=679.2µs] frames=29)
1980: Blizzard_CooldownViewer loaded: 18 Lua, 8 XML, 0 warnings, 39 nil observations, 0 missing requirements (47.8ms: xmlproc=26.1ms exec_lua=17.9ms lifecycle=6.6ms layers=247.7µs lua=8.2ms [compile=4.3ms call=3.8ms] frames=24)
2030: Blizzard_Deprecated loaded: 5 Lua, 0 XML, 0 warnings, 0 nil observations, 1 missing requirements (1.0ms: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=375.0µs [compile=203.3µs call=171.7µs] frames=0)
2043: Blizzard_DeprecatedCombatLog loaded: 1 Lua, 0 XML, 0 warnings, 0 nil observations, 4 missing requirements (3.4ms: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=837.5µs [compile=136.3µs call=701.2µs] frames=0)
2049: Blizzard_DeprecatedGuildScript loaded: 1 Lua, 0 XML, 0 warnings, 0 nil observations, 3 missing requirements (191.2µs: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=118.3µs [compile=20.6µs call=97.7µs] frames=0)
2053: Blizzard_DeprecatedItemScript loaded: 1 Lua, 0 XML, 0 warnings, 0 nil observations, 33 missing requirements (767.5µs: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=683.7µs [compile=48.8µs call=634.9µs] frames=0)
2061: Blizzard_DeprecatedPetInfo loaded: 1 Lua, 0 XML, 0 warnings, 0 nil observations, 2 missing requirements (92.3µs: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=53.2µs [compile=8.2µs call=45.0µs] frames=0)
2063: Blizzard_DeprecatedPvpScript loaded: 1 Lua, 0 XML, 0 warnings, 0 nil observations, 3 missing requirements (109.5µs: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=74.1µs [compile=12.8µs call=61.3µs] frames=0)
2144: Blizzard_GameMenu loaded: 2 Lua, 1 XML, 0 warnings, 1 nil observations, 0 missing requirements (4.8ms: xmlproc=2.7ms exec_lua=1.9ms lifecycle=551.0µs layers=34.7µs lua=454.4µs [compile=245.6µs call=208.9µs] frames=4)
2171: Blizzard_GroupFinder loaded: 11 Lua, 9 XML, 0 warnings, 4 nil observations, 3 missing requirements (131.4ms: xmlproc=102.6ms exec_lua=70.1ms lifecycle=17.8ms layers=5.4ms lua=10.2ms [compile=5.4ms call=4.8ms] frames=228)
2196: Blizzard_HousingTemplates loaded: 9 Lua, 7 XML, 0 warnings, 14 nil observations, 0 missing requirements (7.6ms: xmlproc=923.5µs exec_lua=53.9µs lifecycle=51.9µs layers=383.1µs lua=3.4ms [compile=2.3ms call=1.1ms] frames=1)
2230: Blizzard_Tutorials loaded: 14 Lua, 1 XML, 0 warnings, 0 nil observations, 3 missing requirements (6.6ms: xmlproc=1.0ms exec_lua=557.7µs lifecycle=352.4µs layers=501.0ns lua=4.1ms [compile=2.1ms call=1.9ms] frames=1)
2275: Blizzard_MainMenuBarBagButtons loaded: 3 Lua, 1 XML, 0 warnings, 7 nil observations, 0 missing requirements (13.5ms: xmlproc=11.7ms exec_lua=6.1ms lifecycle=3.9ms layers=3.6µs lua=762.8µs [compile=341.4µs call=421.4µs] frames=8)
2363: Blizzard_PetBattleUI loaded: 3 Lua, 2 XML, 0 warnings, 0 nil observations, 4 missing requirements (28.7ms: xmlproc=22.4ms exec_lua=12.2ms lifecycle=2.6ms layers=4.6ms lua=1.5ms [compile=777.2µs call=701.7µs] frames=37)
2426: Blizzard_SocialUI loaded: 3 Lua, 2 XML, 0 warnings, 10 nil observations, 2 missing requirements (68.0ms: xmlproc=60.6ms exec_lua=41.5ms lifecycle=18.1ms layers=267.2µs lua=1.8ms [compile=1.4ms call=438.1µs] frames=5)
2463: Blizzard_UIParent loaded: 1 Lua, 0 XML, 0 warnings, 1 nil observations, 0 missing requirements (144.7µs: xmlproc=0.0ns exec_lua=0.0ns lifecycle=0.0ns layers=0.0ns lua=29.3µs [compile=16.8µs call=12.5µs] frames=0)
2477: Blizzard_WorldMap loaded: 7 Lua, 5 XML, 0 warnings, 1 nil observations, 0 missing requirements (15.9ms: xmlproc=11.7ms exec_lua=2.2ms lifecycle=8.6ms layers=162.6µs lua=1.4ms [compile=989.1µs call=402.6µs] frames=5)
```
