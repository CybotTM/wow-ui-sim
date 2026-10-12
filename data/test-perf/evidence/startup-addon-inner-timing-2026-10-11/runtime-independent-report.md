# returnedLoadResult — runtime-independent observation gate

**Verdict: OBSERVATIONAL_COHERENCE_PASS. Observation only; not full-goal completion.**

## Reused evidence
First authentication reused from `/home/osso/.local/state/wow-ui-sim/verification/startup-addon-inner-timing-current/compile-independent-auth.json`: **349156608 bytes**, SHA256 **5c0d33ddbe6fee24d061efd742b6de2e3bd96faa0443cf866dca4f23e4c91a66**; one prior payload read, no new hash/read. Compile/source report and full proof reused first, including byte identity at `cfcede4a0ec641c311599a3e03205a3851bc1b23`, typed field contract, manual readability, and format exit 0. `preflight-output-gap.json` remains historical: no independent source-preflight PASS fabricated. Source bucket definitions reused from saved `startup-addon-inner-timing-boundary-current.md`; no fresh source audit.

## Actual request and terminal evidence
Request: `/home/osso/.local/state/wow-ui-sim/verification/startup-addon-inner-timing-current/runtime-request-20261012T052154358712Z.jsonl`. Inspected as data only: sole expression dispatch is `print(submit_runtime(...))` for this epoch; compile definitions are inert. Submission invokes the authenticated path, not Cargo. Unit `wow-startup-inner-observation-20261012t052154361130z`; saved admission 2026-10-12T05:21:54.375854+00:00: load **21.35**, available RAM **29.787315369 GiB**, relevant Cargo **[]**, admitted. Saved terminal 2026-10-12T05:22:29.040588+00:00: Result=success, ExecMainStatus=0, active/exited (RemainAfterExit), ControlGroup empty. Saved process observation: remaining probe processes **[]**. Main-result matches independent parsing: **221 JSON = 220 load_result + 1 final summary**, complete=true, rowcount=220. No fresh guard, process query, poll, or execution.

## Exact coverage
Full raw stdout **173030 bytes / 221 lines** and stderr **643 bytes / 11 lines** read without slicing; both full texts retained in proof. All JSON lines parse; terminal summary is last; no observed stream record truncation. Each of **220** rows has exact root/group keys, nonempty expected/actual, consecutive ordinal **1–220**, two nonnegative file counts, **8 base + 10 XML/frame + 4 creation + 6 cache + 3 diagnostic fields**. **7260 numeric field values** checked (excludes ordinal). Schema/type/negative failures: **0**. Discovery keys unique: **220**. Returned name differs from expected key in **73** rows; preserve pairs, not equality assertions. Duplicate returned names: `{"Blizzard Saved Sets": 2}`. No independent TOC/title/manifest pairing audit; those display-like strings alone do not establish a wrong addon. All identities/file counts and raw rows retained in proof.

Source-defined `total = io + xml_parse + xml_process + lua_exec + savedvars`: **220/220 equal**, mismatch count **0**. Reused contract supports `lua_exec = lua_compile + lua_call`: **220/220 equal**, mismatch count **0**. Never sum lua_exec together with compile/call or sum XML/frame children as exclusive cost. Public total is an accounting duration, not measured walltime; nested work may belong to an enclosing invocation.

## Aggregate counts
| Group | Observed aggregate |
|---|---|
| Files | Lua 1501; XML 677 |
| counts | fontstring 689; frame 2482; lifecycle_fire 1761; texture 1151 |
| cache | hits 1497; lookupmiss 0; misses 0; replayfail 0; store_failure 0; store_success 0 |
| diagnostics | missing_requirements 90; nil_symbol_observations 185; warnings 0 |


Diagnostics nonzero on **49 rows**: missing_requirements: 27 rows, nil_symbol_observations: 38 rows, warnings: 0 rows. These are returned collection lengths, not unique errors. Complete=true/no returned hard error is distinct from diagnostic-free execution. Diagnostic contents/severities and cap/filter behavior are not exposed by these counts. Full per-addon diagnostic counts retained in proof.

Zero-file rows: ordinal 108 `Blizzard_ClientSavedVariables` (total 50 ns); ordinal 169 `Blizzard_FullscreenBrowser` (total 40 ns). Retained successful results; **not automatically labeled skipped**. Read-only mode is reused setup-contract evidence; zero store_success/store_failure counters are observed returned counters, not cache-tree immutability/native proof. The 1501 returned Lua-file count and 1497 cache-hit count have different contracts; no inferred missing files, exact coverage, or causal explanation.

## Actual bucket totals ranked
Within each group: descending nanoseconds. Alias/parent/detail fields stay separate; none are combined into an exclusive-cost sum.

| Group/bucket | Total ns | Total ms |
|---|---:|---:|
| base.total | 4111631541 | 4111.631541 |
| base.xml_process | 2224790049 | 2224.790049 |
| base.lua_exec | 891450284 | 891.450284 |
| base.lua_compile | 645984880 | 645.984880 |
| base.io | 504566205 | 504.566205 |
| base.xml_parse | 490765220 | 490.765220 |
| base.lua_call | 245465404 | 245.465404 |
| base.savedvars | 59783 | 0.059783 |
| xml_frame.xml_frame_finalize | 2750970854 | 2750.970854 |
| xml_frame.xml_frame_create | 2207029055 | 2207.029055 |
| xml_frame.xml_frame_setup | 1047204997 | 1047.204997 |
| xml_frame.frame_exec_lua | 1035075140 | 1035.075140 |
| xml_frame.frame_lifecycle | 992904728 | 992.904728 |
| xml_frame.frame_layer_children | 58984602 | 58.984602 |
| xml_frame.frame_button | 27860454 | 27.860454 |
| xml_frame.frame_anim | 18257533 | 18.257533 |
| xml_frame.frame_code_build | 14246984 | 14.246984 |
| xml_frame.frame_apply_props | 11737510 | 11.737510 |

**Overlap finding:** `xml_frame_finalize` totals **2750970854 ns**, exceeding `xml_frame_create` **2207029055 ns** (and `xml_process` **2224790049 ns**). No reused contract establishes a child≤parent invariant between these fields. Report literal scopes; no exclusive subtraction, invented invariant failure, or causal explanation.

## Addon ranks from this run
Top 3 for every returned timing bucket; rank uses ordinal ties. Expected discovery keys shown; proof keeps ordinal/actual names and complete 220-row ranking for each bucket. Total/exec/compile/call rankings are separate aliases/components; XML/frame rankings overlap. These are invocation observations, not exclusive addon ownership or root cause.

| Bucket | Top observed addons (ns) |
|---|---|
| base.io | 1. [206] `Blizzard_SharedMapDataProviders` 95547310; 2. [11] `Blizzard_SharedXML` 38095244; 3. [218] `Blizzard_WorldMap` 20758591 |
| base.lua_call | 1. [11] `Blizzard_SharedXML` 37205567; 2. [29] `Blizzard_CatalogShopSharedUtil` 25030482; 3. [5] `Blizzard_SharedXMLBase` 22288133 |
| base.lua_compile | 1. [1] `Blizzard_ProjectConstants` 411517810; 2. [7] `Blizzard_Menu` 35134689; 3. [11] `Blizzard_SharedXML` 33147124 |
| base.lua_exec | 1. [1] `Blizzard_ProjectConstants` 411581020; 2. [11] `Blizzard_SharedXML` 70352691; 3. [7] `Blizzard_Menu` 37766328 |
| base.savedvars | 1. [120] `Blizzard_Console` 3477; 2. [8] `Blizzard_Colors` 1784; 3. [56] `Blizzard_QuickKeybind` 1072 |
| base.total | 1. [1] `Blizzard_ProjectConstants` 411698562; 2. [58] `Blizzard_UIPanels_Game` 325770696; 3. [68] `Blizzard_ActionBar` 264135249 |
| base.xml_parse | 1. [206] `Blizzard_SharedMapDataProviders` 52851390; 2. [53] `Blizzard_FrameXML` 42360663; 3. [11] `Blizzard_SharedXML` 35829979 |
| base.xml_process | 1. [58] `Blizzard_UIPanels_Game` 275029345; 2. [68] `Blizzard_ActionBar` 252937410; 3. [119] `Blizzard_CompactRaidFrames` 156472034 |
| xml_frame.frame_anim | 1. [53] `Blizzard_FrameXML` 7357509; 2. [58] `Blizzard_UIPanels_Game` 2591285; 3. [74] `Blizzard_UnitFrame` 1227649 |
| xml_frame.frame_apply_props | 1. [58] `Blizzard_UIPanels_Game` 1344927; 2. [53] `Blizzard_FrameXML` 1022610; 3. [172] `Blizzard_GroupFinder` 769246 |
| xml_frame.frame_button | 1. [58] `Blizzard_UIPanels_Game` 3495889; 2. [53] `Blizzard_FrameXML` 2787437; 3. [172] `Blizzard_GroupFinder` 1895757 |
| xml_frame.frame_code_build | 1. [58] `Blizzard_UIPanels_Game` 1641246; 2. [53] `Blizzard_FrameXML` 1312959; 3. [172] `Blizzard_GroupFinder` 782717 |
| xml_frame.frame_exec_lua | 1. [58] `Blizzard_UIPanels_Game` 184846670; 2. [65] `Blizzard_Communities` 66368394; 3. [172] `Blizzard_GroupFinder` 58224170 |
| xml_frame.frame_layer_children | 1. [53] `Blizzard_FrameXML` 10212116; 2. [191] `Blizzard_PVPMatch` 7446189; 3. [58] `Blizzard_UIPanels_Game` 4941184 |
| xml_frame.frame_lifecycle | 1. [68] `Blizzard_ActionBar` 240321823; 2. [119] `Blizzard_CompactRaidFrames` 152128845; 3. [74] `Blizzard_UnitFrame` 101264376 |
| xml_frame.xml_frame_create | 1. [58] `Blizzard_UIPanels_Game` 274607899; 2. [68] `Blizzard_ActionBar` 252914195; 3. [119] `Blizzard_CompactRaidFrames` 156466404 |
| xml_frame.xml_frame_finalize | 1. [58] `Blizzard_UIPanels_Game` 339756801; 2. [172] `Blizzard_GroupFinder` 316175720; 3. [68] `Blizzard_ActionBar` 251209738 |
| xml_frame.xml_frame_setup | 1. [58] `Blizzard_UIPanels_Game` 186238386; 2. [65] `Blizzard_Communities` 66841211; 3. [172] `Blizzard_GroupFinder` 59016943 |

## Errors, truncation, panic, collector boundaries
Saved stderr contains only 11 environment-startup timing lines, no visible panic/error. No returned_error record; saved process exit 0 and final summary complete. This exercises successful observation/output only: environment failure, first returned hard error, partial-prefix output, write failure, timeout, and panic paths are **not exercised**. Full-stream syntax/completion establishes no visible saved-record truncation, not exhaustiveness of internal logging. Diagnostic collection filtering/caps/omissions and log-collector loss limits were not measured; zero warnings is only the returned collection length. Tool display truncation during analysis was distinct from runtime capture: full streams were read and included in proof; saved analysis output used to recover display-limited totals.

```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.027s] [Startup] SimState::default complete in 26.64ms
[  0.029s] [Startup] rilua VM created in 2.17ms
[  0.030s] [Startup] builtin frames initialized in 995.45µs
[  0.031s] [Startup] template registry cleared in 620.51µs
[  0.031s] [Startup] intrinsic templates registered in 139.20µs
[  0.031s] [Startup] initial app_data lua handle installed in 1.12µs
[  0.366s] [Startup] init_lua_state complete in 335.28ms
[  0.367s] [Startup] final app_data lua handle installed in 340.00ns
[  0.367s] [Startup] initial screen globals installed in 240.61µs
[  0.367s] [Startup] WowLuaEnv::new complete
```

## Provenance and exclusions
Frozen/private/tracked/committed byte identity and typed compilation reused, not re-established. Library producer revision `262987b1a65f049d5c3e158b18a8eb73cc22e4ba`; saved Cargo compiler-artifact record/stat matches reused for wow_ui_sim and serde_json. No current library payload/stat/hash proof, hermetic build proof, or frozen acceptance-library proof. Configuration and returned cache counters do not establish cache-tree or native WoW behavior. No old-run paired comparison; old **220 phase / 1497 file DONE** scope untouched. No root-cause, repair, startup-event/tick, or acceptance-case claim. No build/run/check/format/guard/poll/source audit/delegation/network/ops performed. Only the two explicitly owned report/proof paths written.

## Owned outputs
- `/home/osso/.local/state/wow-ui-sim/verification/startup-addon-inner-timing-current/runtime-independent-report.md`
- `/home/osso/.local/state/wow-ui-sim/verification/startup-addon-inner-timing-current/runtime-independent-proof.json`
