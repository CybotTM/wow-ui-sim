# Region alpha numeric RED artifact verification

Verdict: **observed RED / valid-baseline consistency only. Not a final gate.**

Read <HOME>/AgentConfig/skills/verify/SKILL.md. Direct artifact verification under caller prohibition on delegation; no test command executed. All 24 artifacts fully read; complete contents follow below and are embedded in proof.json. No rerun, build, authorization interaction, service mutation, Bash, delegation, binary/library/source-tree/public payload read/hash, current-HEAD, native-client or render proof.

## Existing normal-runtime evidence

| Profile | Recorded normal-binary source | Valid | Invalid |
|---|---|---|---|
| Retail | cd9f42756472b8f8d706fd3e592f1977b3d89868 | exit 0; COMPLETE valid; CLEAN, 0 errors | exit 1; first Frame rejection assertion at -0.25; no COMPLETE; 2 unique/2 diagnostic occurrences |
| PTR | a5397a2d60ebdd2dc813309671bfb26534e3621c | exit 0; COMPLETE valid; CLEAN, 0 errors | exit 1; first Frame rejection assertion at -0.25; no COMPLETE; 2 unique/2 diagnostic occurrences |

Valid runs complete Frame, Texture, FontString with 0, 0.5, 1: nine setter/value combinations and eighteen return-count/roundtrip assertions per profile, inferred from the complete fixture and final marker. No inherited-alpha assertion exists in the valid branch. This is not a Rust test count; result.expected_cases is 0 and summaries is empty in all runs.

Invalid runs create all three region objects and set parent alpha 0.5. Only Frame enters the invalid loop: its initial SetAlpha(0.625), effective-alpha baseline query and pcall SetAlpha(-0.25) precede the failing rejection assertion. pcall accepted is true by assertion/control-flow evidence. Neither subsequent stored-alpha nor effective-alpha preservation assertion is reached. No observed post-invalid stored/effective value, clamp semantics, or other invalid value result is claimed.

## Exact invalid coverage (same for Retail and PTR)

| Region | -0.25 | 1.25 | -(2 ^ -30) | 1 + (2 ^ -30) | -math.huge | math.huge | 0 / 0 |
|---|---|---|---|---|---|---|---|
| Frame | REACHED / rejection assertion FAIL | UNREACHED | UNREACHED | UNREACHED | UNREACHED | UNREACHED | UNREACHED |
| Texture | UNREACHED | UNREACHED | UNREACHED | UNREACHED | UNREACHED | UNREACHED | UNREACHED |
| FontString | UNREACHED | UNREACHED | UNREACHED | UNREACHED | UNREACHED | UNREACHED | UNREACHED |

The fixture contains **seven invalid numbers total**, not seven additional numbers after -0.25: six further Frame values, plus all seven Texture and all seven FontString values, are unreached (20/21 combinations per profile). Construction of Texture/FontString is reached; their invalid setup/setters/getters/assertions are not. These unexecuted cases are not promoted to RED, passing, or compatibility proof.

Failure text in both profiles: `SetAlpha must reject out-of-range numeric alpha: -0.25`. Runtime traceback is `(string):1`; do not reinterpret it as a precise source-file line. Each invalid stdout has the direct exec-lua diagnostic and Blizzard_ScriptErrors' diagnostic of that failure (GetErrorData line 59 and handler lines 71–72). They are two diagnostic entries, not two independently failing input cases. Full stderr includes both traces and the final FAILED / 2 unique / 2 occurrences / 0 attributed owners / 2 unattributed occurrences summary. Valid stderr includes complete startup/compatibility diagnostics and CLEAN / zero-error summary, not merely JSON `[]`.

## Fixture and provenance boundary

Input label: `tests/region_alpha_validation/ordinary_range.lua`. All four complete `--exec-lua` arguments were read in full; exact complete argv and script are retained. Only the initial `alphaContractMode = 'valid'` / `'invalid'` prefix differs. Shared body has actual SetAlpha/GetAlpha/GetEffectiveAlpha calls, no method overrides or getter spies. No source-tree read was used to assert byte identity against the named Git fixture blob.

Existing recorded argv invokes frozen normal wow-sim artifacts through timeout/bwrap, with source snapshot read-only bind, CASC=0, sound disabled, --no-addons, --no-saved-vars, --exec-lua complete script, lua-errors. These are historical launcher records, not commands run during verification. Original receipt stat fields are reproduced only; no independent binary read/hash/stat or fresh source-build provenance claim.

Git metadata confirms authored commit `135b03c13fc093d3c4211e403207ccca3419f62d`, subject `Add ordinary region-alpha numeric range contract tests`, adds spec, Rust test file and Lua fixture. Two new Rust cases are caller-provided authorship information: **NOT compiled or executed**, not proven by these runtime artifacts. Metadata-only diff-tree comparisons from both recorded binary-source revisions to authored commit show no change for `src/lua_api/frame/methods/core_state/alpha.rs` (exit 0, empty changed-path output). No current HEAD inspected. First four Git metadata probes failed because Pyrun command cwd was not the repository; explicit-cwd metadata probes succeeded. No runtime probe was attempted.

Artifact timestamps are recorded as October 14, 2026 UTC; Git authored time October 13, 2026 22:16:52 -0500 corresponds to October 14, 2026 03:16:52 UTC. These are recorded timestamps, not a claim about current local date.

## Full evidence

### retail valid

Directory: `<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-retail-green-retry2-20261014T020756467959Z/runtime-region-alpha-valid-red-20261014T031725746358Z`

#### admission.json (complete, 2418 bytes)

```text
{
  "utc": "2026-10-14T03:17:25.848071+00:00",
  "raw": {
    "memory.current": "29713391616\n",
    "memory.high": "42949672960\n",
    "memory.max": "45097156608\n",
    "memory.stat": "anon 14564413440\nfile 12228825088\nkernel 2823430144\nkernel_stack 5898240\npagetables 63610880\nsec_pagetables 0\npercpu 20775360\nsock 0\nvmalloc 0\nshmem 803905536\nzswap 688618078\nzswapped 2791460864\nzswap_incomp 19869696\nfile_mapped 929140736\nfile_dirty 20955136\nfile_writeback 18108416\nswapcached 654680064\nanon_thp 121634816\nfile_thp 65011712\nshmem_thp 0\ninactive_anon 7641497600\nactive_anon 7809212416\ninactive_file 10378493952\nactive_file 1046253568\nunevictable 0\nslab_reclaimable 2001450096\nslab_unreclaimable 30847640\nslab 2032297736\nhugetlb 0\nworkingset_refault_anon 913050934\nworkingset_refault_file 1171671253\nworkingset_activate_anon 90574240\nworkingset_activate_file 48938070\nworkingset_restore_anon 93459744\nworkingset_restore_file 115602286\nworkingset_nodereclaim 426322757\npgdemote_kswapd 0\npgdemote_direct 0\npgdemote_khugepaged 0\npgdemote_proactive 0\npgsteal_kswapd 12457927611\npgsteal_direct 8638355554\npgsteal_khugepaged 93420\npgsteal_proactive 0\npgscan_kswapd 13612909474\npgscan_direct 12107092579\npgscan_khugepaged 112894\npgscan_proactive 0\npgrefill 10149524892\npgpromote_success 0\npgscan 25720114947\npgsteal 21096376585\npswpin 75131418\npswpout 284192717\npgfault 45512717655\npgmajfault 1125593064\npgactivate 6441815995\npgdeactivate 0\npglazyfree 0\npglazyfreed 1\nswpin_zero 4468615\nswpout_zero 27719445\nzswpin 549665473\nzswpout 1799058401\nzswpwb 284620882\nthp_fault_alloc 4695841\nthp_collapse_alloc 4895\nthp_swpout 0\nthp_swpout_fallback 35555\nnuma_pages_migrated 0\nnuma_pte_updates 0\nnuma_hint_faults 0\n",
    "memory.pressure": "some avg10=1.57 avg60=1.74 avg300=2.47 total=6979434627\nfull avg10=0.67 avg60=0.86 avg300=1.94 total=6042506016\n",
    "cpu.pressure": "some avg10=11.97 avg60=13.78 avg300=10.34 total=27443133637\nfull avg10=1.20 avg60=1.44 avg300=0.80 total=2565553113\n",
    "io.pressure": "some avg10=11.66 avg60=9.75 avg300=11.18 total=17061446816\nfull avg10=6.38 avg60=5.26 avg300=8.09 total=12983509017\n"
  },
  "half_clean_file_and_slab_reclaim": 6229082680,
  "projected": 27779276232,
  "host_available": 28999147520,
  "allowed": true,
  "worker_max": 4294967296,
  "worker_high": 3221225472,
  "profile": "retail"
}
```

#### argv.json (complete, 3180 bytes)

```text
[
  "systemd-run",
  "--user",
  "--wait",
  "--unit=<UNIT_04>",
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
  "StandardOutput=file:<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-retail-green-retry2-20261014T020756467959Z/runtime-region-alpha-valid-red-20261014T031725746358Z/stdout",
  "-p",
  "StandardError=file:<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-retail-green-retry2-20261014T020756467959Z/runtime-region-alpha-valid-red-20261014T031725746358Z/stderr",
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
  "<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-retail-green-retry2-20261014T020756467959Z/source",
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
  "<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-retail-green-retry2-20261014T020756467959Z/artifacts/wow-sim",
  "--no-addons",
  "--no-saved-vars",
  "--exec-lua",
  "alphaContractMode = 'valid'\nlocal parent = CreateFrame(\"Frame\", nil, UIParent)\nparent:SetAlpha(0.5)\nlocal frame = CreateFrame(\"Frame\", nil, parent)\nlocal regions = { frame, frame:CreateTexture(), frame:CreateFontString() }\n\nif alphaContractMode == \"valid\" then\n    for _, region in ipairs(regions) do\n        for _, alpha in ipairs({ 0, 0.5, 1 }) do\n            assert(select(\"#\", region:SetAlpha(alpha)) == 0, \"SetAlpha returns no values\")\n            assert(region:GetAlpha() == alpha, \"valid alpha must roundtrip\")\n        end\n    end\nelseif alphaContractMode == \"invalid\" then\n    local nearBoundary = 2 ^ -30\n    local invalid = { -0.25, 1.25, -nearBoundary, 1 + nearBoundary, -math.huge, math.huge, 0 / 0 }\n    for _, region in ipairs(regions) do\n        region:SetAlpha(0.625)\n        local effectiveBefore = region:GetEffectiveAlpha()\n        for _, alpha in ipairs(invalid) do\n            local accepted = pcall(function() region:SetAlpha(alpha) end)\n            assert(not accepted, \"SetAlpha must reject out-of-range numeric alpha: \" .. tostring(alpha))\n            assert(region:GetAlpha() == 0.625, \"rejection must preserve stored alpha\")\n            assert(region:GetEffectiveAlpha() == effectiveBefore, \"rejection must preserve inherited alpha\")\n        end\n    end\nelse\n    error(\"unknown alpha contract mode\")\nend\nprint(\"REGION_ALPHA_CONTRACT_COMPLETE\", alphaContractMode)\n",
  "lua-errors"
]
```

#### launcher.txt (complete, 308 bytes)

```text
Running as unit: <UNIT_04>.service
          Finished with result: success
Main processes terminated with: code=exited, status=0/SUCCESS
               Service runtime: 11.555s
             CPU time consumed: 6.113s
                   Memory peak: 2.2G (swap: 0B)
```

#### stdout (complete, 40 bytes)

```text
REGION_ALPHA_CONTRACT_COMPLETE	valid
[]
```

#### stderr (complete, 14696 bytes)

```text
[  0.000s] [startup] wow-sim process started pid=<PID_01>
[  0.000s] Resource limits: 2 CPU core(s)
[  0.017s] [Startup] WowLuaEnv::new begin
[  0.128s] [Startup] SimState::default complete in 109.53ms
[  0.136s] [Startup] rilua VM created in 8.07ms
[  0.140s] [Startup] builtin frames initialized in 3.83ms
[  0.142s] [Startup] template registry cleared in 2.02ms
[  0.142s] [Startup] intrinsic templates registered in 11.17µs
[  0.142s] [Startup] initial app_data lua handle installed in 641.00ns
[  0.480s] [Startup] init_lua_state complete in 338.58ms
[  0.480s] [Startup] final app_data lua handle installed in 241.00ns
[  0.481s] [Startup] initial screen globals installed in 438.70µs
[  0.481s] [Startup] WowLuaEnv::new complete
[  0.481s] [Startup] screen size set to 1600x1200 in 434.13µs
[  0.481s] [Startup] WowFontSystem::new begin casc=false
[  0.813s] [Startup] font system fallback loaded in 331.74ms
[  0.815s] [Startup] cosmic font system built in 1.57ms
[  0.815s] [Startup] swash cache created in 418.33µs
[  0.815s] [Startup] WowFontSystem::new complete in 333.78ms
[  0.815s] [Startup] font system created in 333.79ms
[  0.819s] Sound disabled
[  0.832s] [Startup] begin stop GC
[  0.832s] [Startup] end stop GC in 281.00ns
[  0.832s] [Startup] begin configure saved variables
[  0.832s] SavedVariables loading disabled
[  0.832s] [Startup] end configure saved variables in 6.75µs
[  0.832s] [Startup] begin load keybindings
[  0.832s] [Startup] end load keybindings in 160.00ns
[  0.832s] [Startup] begin configure edit mode cache
[  0.834s] EditMode cache source (read-only): <CHARACTER_ID_01> @ <CHARACTER_01>
[  0.834s] [Startup] end configure edit mode cache in 2.43ms
[  0.834s] [Startup] begin read ServerSnapshot edit mode layout
[  0.834s] [Startup] end read ServerSnapshot edit mode layout in 30.00ns
[  0.834s] [Startup] begin load edit mode cache
[  0.846s] Loaded EditMode layout cache from WTF
[  0.846s] [Startup] end load edit mode cache in 11.49ms
[  0.846s] [Startup] begin load ServerSnapshot character state
[  0.846s] [Startup] end load ServerSnapshot character state in 130.00ns
[  0.846s] [Startup] begin load Blizzard addons
[  1.246s] Loading 291 Blizzard addons...
[  8.758s] Blizzard addons loaded in 7.91s (io=1.22s xml=813.00ms xmlproc=2.07s frames⊂xmlproc=2.06s lua=3.11s [compile=2.97s call=146.78ms] gc=deferred, bytecode cache: 1569/1569 hits (100%, lookup_miss=0 replay_fail=0 stored=0 store_fail=0))
  frame breakdown: setup=902.57ms finalize=2.55s (2482 frames)
  setup: code_build=10.67ms exec_lua=892.15ms props=10.12ms
  finalize: layers=60.88ms (1151 tex, 689 fs) anim=18.56ms button=82.62ms lifecycle=942.53ms (1761 fires)
[  8.758s] [Startup] end load Blizzard addons in 7.91s
[  8.758s] [Startup] begin prepare chat frame for third-party addons
[  8.759s] [Startup] end prepare chat frame for third-party addons in 894.78µs
[  8.759s] [Startup] begin apply post-Blizzard load workarounds
[  8.759s] [Startup] applying post-load workarounds
[  8.734s] [Startup] apply_post_load_workarounds: begin
[  8.734s] [Workarounds] starting patch_edit_mode_manager
[  8.734s] [Workarounds] finished patch_edit_mode_manager in 459.08µs
[  8.734s] [Workarounds] starting init_edit_mode_layout
[  8.734s] [EditMode] starting setup_layout_info
[  8.763s] [EditMode] finished setup_layout_info in 28.53ms
[  8.763s] [EditMode] starting apply_system_anchors
[  8.979s] [EditMode] finished apply_system_anchors in 216.30ms
[  8.979s] [EditMode] starting finalize_action_bar_positions
[  8.982s] [EditMode] finished finalize_action_bar_positions in 2.61ms
[  8.982s] [EditMode] starting invoke_anchor_changed_hooks
[  8.982s] [EditMode] finished invoke_anchor_changed_hooks in 552.97µs
[  8.982s] [EditMode] starting fix_action_bar_nan_size
[  8.982s] [EditMode] finished fix_action_bar_nan_size in 57.37µs
[  8.982s] [EditMode] starting fix_action_bar_scale
[  8.982s] [EditMode] finished fix_action_bar_scale in 6.09µs
[  8.982s] [Workarounds] finished init_edit_mode_layout in 248.13ms
[  8.982s] [Workarounds] starting patch_ui_parent_panel_toggles
[  8.983s] [Workarounds] finished patch_ui_parent_panel_toggles in 322.85µs
[  8.983s] [Workarounds] starting patch_uiparent_onupdate_worklists
[  8.983s] [Workarounds] finished patch_uiparent_onupdate_worklists in 145.31µs
[  8.983s] [Workarounds] starting init_chat_type_colors
[  8.983s] [Workarounds] finished init_chat_type_colors in 158.94µs
[  8.983s] [Workarounds] starting patch_chat_voice_button_surface
[  8.983s] [Workarounds] finished patch_chat_voice_button_surface in 153.81µs
[  8.983s] [Workarounds] starting patch_item_socketing_tooltips
[  8.983s] [Workarounds] finished patch_item_socketing_tooltips in 84.44µs
[  8.983s] [Workarounds] starting patch_character_select_selected_name
[  8.983s] [Workarounds] finished patch_character_select_selected_name in 45.78µs
[  8.983s] [Workarounds] starting patch_character_create_defaults
[  8.984s] [Workarounds] finished patch_character_create_defaults in 537.05µs
[  8.984s] [Workarounds] starting patch_character_frame_title_refresh
[  9.003s] [Workarounds] finished patch_character_frame_title_refresh in 18.80ms
[  9.003s] [Workarounds] starting patch_vignette_pin_template
[  9.003s] [Workarounds] finished patch_vignette_pin_template in 145.52µs
[  9.003s] [Workarounds] starting patch_fog_of_war_pin_mixin
[  9.003s] [Workarounds] finished patch_fog_of_war_pin_mixin in 391.73µs
[  9.003s] [Workarounds] starting patch_map_exploration_pin_mixin
[  9.017s] [Workarounds] finished patch_map_exploration_pin_mixin in 13.83ms
[  9.017s] [Workarounds] starting patch_map_canvas_data_provider_attachment
[  9.017s] [Workarounds] finished patch_map_canvas_data_provider_attachment in 100.35µs
[  9.017s] [Workarounds] starting ensure_adventure_map_frame_surface
[  9.018s] [Workarounds] finished ensure_adventure_map_frame_surface in 643.98µs
[  9.018s] [Workarounds] starting patch_action_bar_button_event_fanout
[  9.018s] [Workarounds] finished patch_action_bar_button_event_fanout in 135.48µs
[  9.018s] [Workarounds] starting patch_paging_controls_page_text
[  9.018s] [Workarounds] finished patch_paging_controls_page_text in 52.07µs
[  9.018s] [Workarounds] starting patch_talent_edge_frame_level_sync
[  9.018s] [Workarounds] finished patch_talent_edge_frame_level_sync in 39.18µs
[  9.018s] [Workarounds] starting patch_catalog_shop_product_card_defaults
[  9.018s] [Workarounds] finished patch_catalog_shop_product_card_defaults in 82.11µs
[  9.018s] [Workarounds] starting patch_game_time_defaults
[  9.018s] [Workarounds] finished patch_game_time_defaults in 9.25µs
[  9.018s] [Workarounds] starting patch_tooltip_nineslice_surface
[  9.018s] [Workarounds] finished patch_tooltip_nineslice_surface in 62.99µs
[  9.018s] [Workarounds] starting patch_container_frame_token_tracker
[  9.018s] [Workarounds] finished patch_container_frame_token_tracker in 56.79µs
[  9.018s] [Workarounds] starting patch_achievement_display_set_achievements
[  9.018s] [Workarounds] finished patch_achievement_display_set_achievements in 11.12µs
[  9.018s] [Workarounds] starting patch_housing_dashboard_preload
[  9.018s] [Workarounds] finished patch_housing_dashboard_preload in 79.98µs
[  9.018s] [Workarounds] starting patch_lfg_lock_list
[  9.019s] [Workarounds] finished patch_lfg_lock_list in 49.82µs
[  9.019s] [Workarounds] starting patch_auction_house_browse_results_event
[  9.019s] [Workarounds] finished patch_auction_house_browse_results_event in 93.37µs
[  9.019s] [Workarounds] starting patch_auction_house_search_context_aliases
[  9.019s] [Workarounds] finished patch_auction_house_search_context_aliases in 45.12µs
[  9.019s] [Workarounds] starting patch_auth_challenge_frame_parent
[  9.019s] [Workarounds] finished patch_auth_challenge_frame_parent in 88.33µs
[  9.019s] [Workarounds] starting patch_settings_surface_defaults
[  9.019s] [Workarounds] finished patch_settings_surface_defaults in 484.12µs
[  9.019s] [Workarounds] starting patch_settings_canvas_layout_visibility
[  9.019s] [Workarounds] finished patch_settings_canvas_layout_visibility in 153.92µs
[  9.019s] [Startup] apply_post_load_workarounds: workarounds complete
[  9.111s] [Startup] apply_post_load_workarounds: globals restored
[  9.111s] [Startup] apply_post_load_workarounds: patch 12.1 compatibility restored
[  9.111s] [Startup] apply_post_load_workarounds: seterrorhandler restored
[  9.137s] [Startup] post-load workarounds complete in 377.34ms
[  9.137s] [Startup] end apply post-Blizzard load workarounds in 377.34ms
[  9.137s] [Startup] begin load third-party addons
[  9.137s] Addon loading disabled
[  9.137s] [Startup] end load third-party addons in 1.70µs
[  9.137s] [Startup] begin sync addon names to Lua
[  9.137s] [Startup] end sync addon names to Lua in 379.09µs
[  9.137s] [Startup] applying post-load workarounds
[  9.112s] [Startup] apply_post_load_workarounds: begin
[  9.112s] [Workarounds] starting patch_edit_mode_manager
[  9.112s] [Workarounds] finished patch_edit_mode_manager in 447.35µs
[  9.112s] [Workarounds] starting init_edit_mode_layout
[  9.112s] [EditMode] starting setup_layout_info
[  9.139s] [EditMode] finished setup_layout_info in 26.45ms
[  9.139s] [EditMode] starting apply_system_anchors
[  9.308s] [EditMode] finished apply_system_anchors in 169.84ms
[  9.308s] [EditMode] starting finalize_action_bar_positions
[  9.311s] [EditMode] finished finalize_action_bar_positions in 2.75ms
[  9.311s] [EditMode] starting invoke_anchor_changed_hooks
[  9.316s] [EditMode] finished invoke_anchor_changed_hooks in 5.05ms
[  9.316s] [EditMode] starting fix_action_bar_nan_size
[  9.316s] [EditMode] finished fix_action_bar_nan_size in 57.08µs
[  9.316s] [EditMode] starting fix_action_bar_scale
[  9.316s] [EditMode] finished fix_action_bar_scale in 6.11µs
[  9.316s] [Workarounds] finished init_edit_mode_layout in 204.23ms
[  9.316s] [Workarounds] starting patch_ui_parent_panel_toggles
[  9.317s] [Workarounds] finished patch_ui_parent_panel_toggles in 286.66µs
[  9.317s] [Workarounds] starting patch_uiparent_onupdate_worklists
[  9.317s] [Workarounds] finished patch_uiparent_onupdate_worklists in 146.30µs
[  9.317s] [Workarounds] starting init_chat_type_colors
[  9.317s] [Workarounds] finished init_chat_type_colors in 155.02µs
[  9.317s] [Workarounds] starting patch_chat_voice_button_surface
[  9.317s] [Workarounds] finished patch_chat_voice_button_surface in 305.07µs
[  9.317s] [Workarounds] starting patch_item_socketing_tooltips
[  9.317s] [Workarounds] finished patch_item_socketing_tooltips in 78.72µs
[  9.317s] [Workarounds] starting patch_character_select_selected_name
[  9.317s] [Workarounds] finished patch_character_select_selected_name in 44.15µs
[  9.317s] [Workarounds] starting patch_character_create_defaults
[  9.318s] [Workarounds] finished patch_character_create_defaults in 304.91µs
[  9.318s] [Workarounds] starting patch_character_frame_title_refresh
[  9.329s] [Workarounds] finished patch_character_frame_title_refresh in 10.91ms
[  9.329s] [Workarounds] starting patch_vignette_pin_template
[  9.329s] [Workarounds] finished patch_vignette_pin_template in 135.21µs
[  9.329s] [Workarounds] starting patch_fog_of_war_pin_mixin
[  9.329s] [Workarounds] finished patch_fog_of_war_pin_mixin in 337.17µs
[  9.329s] [Workarounds] starting patch_map_exploration_pin_mixin
[  9.330s] [Workarounds] finished patch_map_exploration_pin_mixin in 452.85µs
[  9.330s] [Workarounds] starting patch_map_canvas_data_provider_attachment
[  9.330s] [Workarounds] finished patch_map_canvas_data_provider_attachment in 80.50µs
[  9.330s] [Workarounds] starting ensure_adventure_map_frame_surface
[  9.330s] [Workarounds] finished ensure_adventure_map_frame_surface in 204.32µs
[  9.330s] [Workarounds] starting patch_action_bar_button_event_fanout
[  9.330s] [Workarounds] finished patch_action_bar_button_event_fanout in 133.05µs
[  9.330s] [Workarounds] starting patch_paging_controls_page_text
[  9.330s] [Workarounds] finished patch_paging_controls_page_text in 50.46µs
[  9.330s] [Workarounds] starting patch_talent_edge_frame_level_sync
[  9.330s] [Workarounds] finished patch_talent_edge_frame_level_sync in 37.40µs
[  9.330s] [Workarounds] starting patch_catalog_shop_product_card_defaults
[  9.330s] [Workarounds] finished patch_catalog_shop_product_card_defaults in 80.75µs
[  9.330s] [Workarounds] starting patch_game_time_defaults
[  9.330s] [Workarounds] finished patch_game_time_defaults in 8.63µs
[  9.330s] [Workarounds] starting patch_tooltip_nineslice_surface
[  9.330s] [Workarounds] finished patch_tooltip_nineslice_surface in 61.97µs
[  9.330s] [Workarounds] starting patch_container_frame_token_tracker
[  9.330s] [Workarounds] finished patch_container_frame_token_tracker in 54.74µs
[  9.330s] [Workarounds] starting patch_achievement_display_set_achievements
[  9.330s] [Workarounds] finished patch_achievement_display_set_achievements in 11.41µs
[  9.330s] [Workarounds] starting patch_housing_dashboard_preload
[  9.330s] [Workarounds] finished patch_housing_dashboard_preload in 75.03µs
[  9.330s] [Workarounds] starting patch_lfg_lock_list
[  9.330s] [Workarounds] finished patch_lfg_lock_list in 8.08µs
[  9.330s] [Workarounds] starting patch_auction_house_browse_results_event
[  9.330s] [Workarounds] finished patch_auction_house_browse_results_event in 96.51µs
[  9.330s] [Workarounds] starting patch_auction_house_search_context_aliases
[  9.331s] [Workarounds] finished patch_auction_house_search_context_aliases in 43.76µs
[  9.331s] [Workarounds] starting patch_auth_challenge_frame_parent
[  9.331s] [Workarounds] finished patch_auth_challenge_frame_parent in 57.61µs
[  9.331s] [Workarounds] starting patch_settings_surface_defaults
[  9.331s] [Workarounds] finished patch_settings_surface_defaults in 475.22µs
[  9.331s] [Workarounds] starting patch_settings_canvas_layout_visibility
[  9.331s] [Workarounds] finished patch_settings_canvas_layout_visibility in 151.13µs
[  9.331s] [Startup] apply_post_load_workarounds: workarounds complete
[  9.420s] [Startup] apply_post_load_workarounds: globals restored
[  9.420s] [Startup] apply_post_load_workarounds: patch 12.1 compatibility restored
[  9.420s] [Startup] apply_post_load_workarounds: seterrorhandler restored
[  9.446s] [Startup] post-load workarounds complete in 308.89ms
[  9.446s] [Startup] restarting GC after bootstrap
[  9.636s] [Startup] GC restart complete in 189.90ms
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```

#### result.json (complete, 617 bytes)

```text
{
  "source": "cd9f42756472b8f8d706fd3e592f1977b3d89868",
  "profile": "retail",
  "filter": "shared ordinary-alpha fixture valid",
  "expected_cases": 0,
  "launcher_exit": 0,
  "summaries": [],
  "stdout_bytes": 40,
  "stderr_bytes": 14696,
  "receipt_stat_reused": {
    "st_dev": 66306,
    "st_ino": 148916796,
    "st_size": 408310896,
    "st_mtime_ns": 1791943848752615957
  },
  "unit": "<UNIT_04>",
  "scope": "Pre-producer shared complete Lua fixture on frozen retail normal binary: ordinary numeric alpha valid; not newlycompiledRusttest/native/render proof"
}
```

### retail invalid

Directory: `<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-retail-green-retry2-20261014T020756467959Z/runtime-region-alpha-invalid-red-20261014T031737945231Z`

#### admission.json (complete, 2407 bytes)

```text
{
  "utc": "2026-10-14T03:17:37.952469+00:00",
  "raw": {
    "memory.current": "27275988992\n",
    "memory.high": "42949672960\n",
    "memory.max": "45097156608\n",
    "memory.stat": "anon 9657569280\nfile 14744481792\nkernel 2779234304\nkernel_stack 5668864\npagetables 52318208\nsec_pagetables 0\npercpu 20707904\nsock 0\nvmalloc 0\nshmem 1576808448\nzswap 641571105\nzswapped 2652020736\nzswap_incomp 16236544\nfile_mapped 874770432\nfile_dirty 26636288\nfile_writeback 0\nswapcached 726274048\nanon_thp 88080384\nfile_thp 109051904\nshmem_thp 0\ninactive_anon 4066824192\nactive_anon 7252840448\ninactive_file 6372884480\nactive_file 6794817536\nunevictable 0\nslab_reclaimable 2016300680\nslab_unreclaimable 30474232\nslab 2046774912\nhugetlb 0\nworkingset_refault_anon 913073206\nworkingset_refault_file 1171682703\nworkingset_activate_anon 90574271\nworkingset_activate_file 48938070\nworkingset_restore_anon 93459775\nworkingset_restore_file 115602286\nworkingset_nodereclaim 426322757\npgdemote_kswapd 0\npgdemote_direct 0\npgdemote_khugepaged 0\npgdemote_proactive 0\npgsteal_kswapd 12457927611\npgsteal_direct 8638355554\npgsteal_khugepaged 93420\npgsteal_proactive 0\npgscan_kswapd 13612909476\npgscan_direct 12107092579\npgscan_khugepaged 112894\npgscan_proactive 0\npgrefill 10149524892\npgpromote_success 0\npgscan 25720114949\npgsteal 21096376585\npswpin 75131435\npswpout 284192717\npgfault 45513981282\npgmajfault 1125622152\npgactivate 6441815995\npgdeactivate 0\npglazyfree 0\npglazyfreed 1\nswpin_zero 4468636\nswpout_zero 27719445\nzswpin 549687707\nzswpout 1799058401\nzswpwb 284620882\nthp_fault_alloc 4695854\nthp_collapse_alloc 4895\nthp_swpout 0\nthp_swpout_fallback 35555\nnuma_pages_migrated 0\nnuma_pte_updates 0\nnuma_hint_faults 0\n",
    "memory.pressure": "some avg10=0.63 avg60=1.48 avg300=2.38 total=6979492245\nfull avg10=0.36 avg60=0.76 avg300=1.87 total=6042563610\n",
    "cpu.pressure": "some avg10=3.60 avg60=11.28 avg300=9.92 total=27443138045\nfull avg10=0.36 avg60=1.18 avg300=0.77 total=2565556358\n",
    "io.pressure": "some avg10=9.10 avg60=9.49 avg300=11.06 total=17062440988\nfull avg10=7.51 avg60=5.81 avg300=8.10 total=12984495592\n"
  },
  "half_clean_file_and_slab_reclaim": 7141283652,
  "projected": 24429672636,
  "host_available": 32921563136,
  "allowed": true,
  "worker_max": 4294967296,
  "worker_high": 3221225472,
  "profile": "retail"
}
```

#### argv.json (complete, 3186 bytes)

```text
[
  "systemd-run",
  "--user",
  "--wait",
  "--unit=<UNIT_05>",
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
  "StandardOutput=file:<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-retail-green-retry2-20261014T020756467959Z/runtime-region-alpha-invalid-red-20261014T031737945231Z/stdout",
  "-p",
  "StandardError=file:<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-retail-green-retry2-20261014T020756467959Z/runtime-region-alpha-invalid-red-20261014T031737945231Z/stderr",
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
  "<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-retail-green-retry2-20261014T020756467959Z/source",
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
  "<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-retail-green-retry2-20261014T020756467959Z/artifacts/wow-sim",
  "--no-addons",
  "--no-saved-vars",
  "--exec-lua",
  "alphaContractMode = 'invalid'\nlocal parent = CreateFrame(\"Frame\", nil, UIParent)\nparent:SetAlpha(0.5)\nlocal frame = CreateFrame(\"Frame\", nil, parent)\nlocal regions = { frame, frame:CreateTexture(), frame:CreateFontString() }\n\nif alphaContractMode == \"valid\" then\n    for _, region in ipairs(regions) do\n        for _, alpha in ipairs({ 0, 0.5, 1 }) do\n            assert(select(\"#\", region:SetAlpha(alpha)) == 0, \"SetAlpha returns no values\")\n            assert(region:GetAlpha() == alpha, \"valid alpha must roundtrip\")\n        end\n    end\nelseif alphaContractMode == \"invalid\" then\n    local nearBoundary = 2 ^ -30\n    local invalid = { -0.25, 1.25, -nearBoundary, 1 + nearBoundary, -math.huge, math.huge, 0 / 0 }\n    for _, region in ipairs(regions) do\n        region:SetAlpha(0.625)\n        local effectiveBefore = region:GetEffectiveAlpha()\n        for _, alpha in ipairs(invalid) do\n            local accepted = pcall(function() region:SetAlpha(alpha) end)\n            assert(not accepted, \"SetAlpha must reject out-of-range numeric alpha: \" .. tostring(alpha))\n            assert(region:GetAlpha() == 0.625, \"rejection must preserve stored alpha\")\n            assert(region:GetEffectiveAlpha() == effectiveBefore, \"rejection must preserve inherited alpha\")\n        end\n    end\nelse\n    error(\"unknown alpha contract mode\")\nend\nprint(\"REGION_ALPHA_CONTRACT_COMPLETE\", alphaContractMode)\n",
  "lua-errors"
]
```

#### launcher.txt (complete, 309 bytes)

```text
Running as unit: <UNIT_05>.service
          Finished with result: exit-code
Main processes terminated with: code=exited, status=1/FAILURE
               Service runtime: 4.737s
             CPU time consumed: 4.674s
                   Memory peak: 1.3G (swap: 0B)
```

#### stdout (complete, 615 bytes)

```text
[
  {
    "message": "[exec-lua] error: Lua error: SetAlpha must reject out-of-range numeric alpha: -0.25\nstack traceback:\n\t(string):1: in main chunk",
    "count": 1
  },
  {
    "message": "Lua Error: [exec-lua] error: Lua error: SetAlpha must reject out-of-range numeric alpha: -0.25\nstack traceback:\n\t(string):1: in main chunk\n[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:59: in function 'GetErrorData'\n[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:72: in function <[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:71>",
    "count": 1
  }
]
```

#### stderr (complete, 15246 bytes)

```text
[  0.000s] [startup] wow-sim process started pid=<PID_02>
[  0.000s] Resource limits: 2 CPU core(s)
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.004s] [Startup] SimState::default complete in 4.11ms
[  0.005s] [Startup] rilua VM created in 149.44µs
[  0.005s] [Startup] builtin frames initialized in 106.99µs
[  0.005s] [Startup] template registry cleared in 17.83µs
[  0.005s] [Startup] intrinsic templates registered in 7.49µs
[  0.005s] [Startup] initial app_data lua handle installed in 321.00ns
[  0.224s] [Startup] init_lua_state complete in 219.77ms
[  0.224s] [Startup] final app_data lua handle installed in 311.00ns
[  0.225s] [Startup] initial screen globals installed in 137.47µs
[  0.225s] [Startup] WowLuaEnv::new complete
[  0.225s] [Startup] screen size set to 1600x1200 in 69.54µs
[  0.225s] [Startup] WowFontSystem::new begin casc=false
[  0.235s] [Startup] font system fallback loaded in 10.41ms
[  0.235s] [Startup] cosmic font system built in 20.52µs
[  0.235s] [Startup] swash cache created in 6.33µs
[  0.235s] [Startup] WowFontSystem::new complete in 10.47ms
[  0.235s] [Startup] font system created in 10.48ms
[  0.235s] Sound disabled
[  0.235s] [Startup] begin stop GC
[  0.235s] [Startup] end stop GC in 521.00ns
[  0.235s] [Startup] begin configure saved variables
[  0.235s] SavedVariables loading disabled
[  0.235s] [Startup] end configure saved variables in 1.81µs
[  0.235s] [Startup] begin load keybindings
[  0.235s] [Startup] end load keybindings in 391.00ns
[  0.235s] [Startup] begin configure edit mode cache
[  0.235s] EditMode cache source (read-only): <CHARACTER_ID_01> @ <CHARACTER_01>
[  0.235s] [Startup] end configure edit mode cache in 38.23µs
[  0.235s] [Startup] begin read ServerSnapshot edit mode layout
[  0.235s] [Startup] end read ServerSnapshot edit mode layout in 30.00ns
[  0.236s] [Startup] begin load edit mode cache
[  0.238s] Loaded EditMode layout cache from WTF
[  0.238s] [Startup] end load edit mode cache in 2.51ms
[  0.238s] [Startup] begin load ServerSnapshot character state
[  0.238s] [Startup] end load ServerSnapshot character state in 140.00ns
[  0.238s] [Startup] begin load Blizzard addons
[  0.249s] Loading 291 Blizzard addons...
[  2.511s] Blizzard addons loaded in 2.27s (io=6.99ms xml=152.38ms xmlproc=1.41s frames⊂xmlproc=1.41s lua=510.50ms [compile=424.73ms call=85.77ms] gc=deferred, bytecode cache: 1569/1569 hits (100%, lookup_miss=0 replay_fail=0 stored=0 store_fail=0))
  frame breakdown: setup=657.67ms finalize=1.66s (2482 frames)
  setup: code_build=7.69ms exec_lua=650.21ms props=7.21ms
  finalize: layers=44.87ms (1151 tex, 689 fs) anim=13.53ms button=26.10ms lifecycle=631.62ms (1761 fires)
[  2.511s] [Startup] end load Blizzard addons in 2.27s
[  2.511s] [Startup] begin prepare chat frame for third-party addons
[  2.512s] [Startup] end prepare chat frame for third-party addons in 794.10µs
[  2.512s] [Startup] begin apply post-Blizzard load workarounds
[  2.512s] [Startup] applying post-load workarounds
[  2.511s] [Startup] apply_post_load_workarounds: begin
[  2.511s] [Workarounds] starting patch_edit_mode_manager
[  2.512s] [Workarounds] finished patch_edit_mode_manager in 362.43µs
[  2.512s] [Workarounds] starting init_edit_mode_layout
[  2.512s] [EditMode] starting setup_layout_info
[  2.531s] [EditMode] finished setup_layout_info in 19.71ms
[  2.531s] [EditMode] starting apply_system_anchors
[  2.755s] [EditMode] finished apply_system_anchors in 224.17ms
[  2.755s] [EditMode] starting finalize_action_bar_positions
[  2.758s] [EditMode] finished finalize_action_bar_positions in 2.79ms
[  2.758s] [EditMode] starting invoke_anchor_changed_hooks
[  2.759s] [EditMode] finished invoke_anchor_changed_hooks in 587.56µs
[  2.759s] [EditMode] starting fix_action_bar_nan_size
[  2.759s] [EditMode] finished fix_action_bar_nan_size in 57.01µs
[  2.759s] [EditMode] starting fix_action_bar_scale
[  2.759s] [EditMode] finished fix_action_bar_scale in 6.08µs
[  2.759s] [Workarounds] finished init_edit_mode_layout in 247.41ms
[  2.759s] [Workarounds] starting patch_ui_parent_panel_toggles
[  2.759s] [Workarounds] finished patch_ui_parent_panel_toggles in 309.05µs
[  2.759s] [Workarounds] starting patch_uiparent_onupdate_worklists
[  2.759s] [Workarounds] finished patch_uiparent_onupdate_worklists in 134.36µs
[  2.759s] [Workarounds] starting init_chat_type_colors
[  2.760s] [Workarounds] finished init_chat_type_colors in 159.13µs
[  2.760s] [Workarounds] starting patch_chat_voice_button_surface
[  2.760s] [Workarounds] finished patch_chat_voice_button_surface in 158.50µs
[  2.760s] [Workarounds] starting patch_item_socketing_tooltips
[  2.760s] [Workarounds] finished patch_item_socketing_tooltips in 83.92µs
[  2.760s] [Workarounds] starting patch_character_select_selected_name
[  2.760s] [Workarounds] finished patch_character_select_selected_name in 43.73µs
[  2.760s] [Workarounds] starting patch_character_create_defaults
[  2.760s] [Workarounds] finished patch_character_create_defaults in 313.48µs
[  2.760s] [Workarounds] starting patch_character_frame_title_refresh
[  2.770s] [Workarounds] finished patch_character_frame_title_refresh in 9.40ms
[  2.770s] [Workarounds] starting patch_vignette_pin_template
[  2.770s] [Workarounds] finished patch_vignette_pin_template in 157.76µs
[  2.770s] [Workarounds] starting patch_fog_of_war_pin_mixin
[  2.770s] [Workarounds] finished patch_fog_of_war_pin_mixin in 378.22µs
[  2.770s] [Workarounds] starting patch_map_exploration_pin_mixin
[  2.779s] [Workarounds] finished patch_map_exploration_pin_mixin in 8.66ms
[  2.779s] [Workarounds] starting patch_map_canvas_data_provider_attachment
[  2.779s] [Workarounds] finished patch_map_canvas_data_provider_attachment in 118.11µs
[  2.779s] [Workarounds] starting ensure_adventure_map_frame_surface
[  2.780s] [Workarounds] finished ensure_adventure_map_frame_surface in 763.40µs
[  2.780s] [Workarounds] starting patch_action_bar_button_event_fanout
[  2.780s] [Workarounds] finished patch_action_bar_button_event_fanout in 154.35µs
[  2.780s] [Workarounds] starting patch_paging_controls_page_text
[  2.780s] [Workarounds] finished patch_paging_controls_page_text in 53.31µs
[  2.780s] [Workarounds] starting patch_talent_edge_frame_level_sync
[  2.780s] [Workarounds] finished patch_talent_edge_frame_level_sync in 38.84µs
[  2.780s] [Workarounds] starting patch_catalog_shop_product_card_defaults
[  2.780s] [Workarounds] finished patch_catalog_shop_product_card_defaults in 82.14µs
[  2.780s] [Workarounds] starting patch_game_time_defaults
[  2.780s] [Workarounds] finished patch_game_time_defaults in 10.09µs
[  2.780s] [Workarounds] starting patch_tooltip_nineslice_surface
[  2.780s] [Workarounds] finished patch_tooltip_nineslice_surface in 64.99µs
[  2.780s] [Workarounds] starting patch_container_frame_token_tracker
[  2.780s] [Workarounds] finished patch_container_frame_token_tracker in 54.05µs
[  2.780s] [Workarounds] starting patch_achievement_display_set_achievements
[  2.780s] [Workarounds] finished patch_achievement_display_set_achievements in 11.59µs
[  2.780s] [Workarounds] starting patch_housing_dashboard_preload
[  2.780s] [Workarounds] finished patch_housing_dashboard_preload in 90.80µs
[  2.780s] [Workarounds] starting patch_lfg_lock_list
[  2.780s] [Workarounds] finished patch_lfg_lock_list in 55.78µs
[  2.780s] [Workarounds] starting patch_auction_house_browse_results_event
[  2.781s] [Workarounds] finished patch_auction_house_browse_results_event in 94.55µs
[  2.781s] [Workarounds] starting patch_auction_house_search_context_aliases
[  2.781s] [Workarounds] finished patch_auction_house_search_context_aliases in 44.86µs
[  2.781s] [Workarounds] starting patch_auth_challenge_frame_parent
[  2.781s] [Workarounds] finished patch_auth_challenge_frame_parent in 95.32µs
[  2.781s] [Workarounds] starting patch_settings_surface_defaults
[  2.781s] [Workarounds] finished patch_settings_surface_defaults in 495.70µs
[  2.781s] [Workarounds] starting patch_settings_canvas_layout_visibility
[  2.781s] [Workarounds] finished patch_settings_canvas_layout_visibility in 157.00µs
[  2.781s] [Startup] apply_post_load_workarounds: workarounds complete
[  2.872s] [Startup] apply_post_load_workarounds: globals restored
[  2.873s] [Startup] apply_post_load_workarounds: patch 12.1 compatibility restored
[  2.873s] [Startup] apply_post_load_workarounds: seterrorhandler restored
[  2.873s] [Startup] post-load workarounds complete in 361.41ms
[  2.873s] [Startup] end apply post-Blizzard load workarounds in 361.42ms
[  2.873s] [Startup] begin load third-party addons
[  2.873s] Addon loading disabled
[  2.873s] [Startup] end load third-party addons in 1.48µs
[  2.873s] [Startup] begin sync addon names to Lua
[  2.874s] [Startup] end sync addon names to Lua in 486.77µs
[  2.874s] [Startup] applying post-load workarounds
[  2.873s] [Startup] apply_post_load_workarounds: begin
[  2.873s] [Workarounds] starting patch_edit_mode_manager
[  2.874s] [Workarounds] finished patch_edit_mode_manager in 515.59µs
[  2.874s] [Workarounds] starting init_edit_mode_layout
[  2.874s] [EditMode] starting setup_layout_info
[  2.894s] [EditMode] finished setup_layout_info in 20.61ms
[  2.894s] [EditMode] starting apply_system_anchors
[  3.013s] [EditMode] finished apply_system_anchors in 118.27ms
[  3.018s] [EditMode] starting finalize_action_bar_positions
[  3.021s] [EditMode] finished finalize_action_bar_positions in 3.14ms
[  3.021s] [EditMode] starting invoke_anchor_changed_hooks
[  3.026s] [EditMode] finished invoke_anchor_changed_hooks in 5.67ms
[  3.026s] [EditMode] starting fix_action_bar_nan_size
[  3.026s] [EditMode] finished fix_action_bar_nan_size in 69.30µs
[  3.026s] [EditMode] starting fix_action_bar_scale
[  3.026s] [EditMode] finished fix_action_bar_scale in 6.48µs
[  3.026s] [Workarounds] finished init_edit_mode_layout in 152.86ms
[  3.026s] [Workarounds] starting patch_ui_parent_panel_toggles
[  3.027s] [Workarounds] finished patch_ui_parent_panel_toggles in 305.09µs
[  3.027s] [Workarounds] starting patch_uiparent_onupdate_worklists
[  3.027s] [Workarounds] finished patch_uiparent_onupdate_worklists in 151.49µs
[  3.027s] [Workarounds] starting init_chat_type_colors
[  3.027s] [Workarounds] finished init_chat_type_colors in 161.45µs
[  3.027s] [Workarounds] starting patch_chat_voice_button_surface
[  3.027s] [Workarounds] finished patch_chat_voice_button_surface in 343.26µs
[  3.027s] [Workarounds] starting patch_item_socketing_tooltips
[  3.028s] [Workarounds] finished patch_item_socketing_tooltips in 85.95µs
[  3.028s] [Workarounds] starting patch_character_select_selected_name
[  3.028s] [Workarounds] finished patch_character_select_selected_name in 45.22µs
[  3.028s] [Workarounds] starting patch_character_create_defaults
[  3.028s] [Workarounds] finished patch_character_create_defaults in 314.17µs
[  3.028s] [Workarounds] starting patch_character_frame_title_refresh
[  3.039s] [Workarounds] finished patch_character_frame_title_refresh in 11.52ms
[  3.039s] [Workarounds] starting patch_vignette_pin_template
[  3.040s] [Workarounds] finished patch_vignette_pin_template in 173.69µs
[  3.040s] [Workarounds] starting patch_fog_of_war_pin_mixin
[  3.040s] [Workarounds] finished patch_fog_of_war_pin_mixin in 362.33µs
[  3.040s] [Workarounds] starting patch_map_exploration_pin_mixin
[  3.041s] [Workarounds] finished patch_map_exploration_pin_mixin in 497.14µs
[  3.041s] [Workarounds] starting patch_map_canvas_data_provider_attachment
[  3.041s] [Workarounds] finished patch_map_canvas_data_provider_attachment in 89.96µs
[  3.041s] [Workarounds] starting ensure_adventure_map_frame_surface
[  3.041s] [Workarounds] finished ensure_adventure_map_frame_surface in 223.86µs
[  3.041s] [Workarounds] starting patch_action_bar_button_event_fanout
[  3.041s] [Workarounds] finished patch_action_bar_button_event_fanout in 143.68µs
[  3.041s] [Workarounds] starting patch_paging_controls_page_text
[  3.041s] [Workarounds] finished patch_paging_controls_page_text in 52.73µs
[  3.041s] [Workarounds] starting patch_talent_edge_frame_level_sync
[  3.041s] [Workarounds] finished patch_talent_edge_frame_level_sync in 40.42µs
[  3.041s] [Workarounds] starting patch_catalog_shop_product_card_defaults
[  3.041s] [Workarounds] finished patch_catalog_shop_product_card_defaults in 84.73µs
[  3.041s] [Workarounds] starting patch_game_time_defaults
[  3.041s] [Workarounds] finished patch_game_time_defaults in 9.49µs
[  3.041s] [Workarounds] starting patch_tooltip_nineslice_surface
[  3.041s] [Workarounds] finished patch_tooltip_nineslice_surface in 59.05µs
[  3.041s] [Workarounds] starting patch_container_frame_token_tracker
[  3.041s] [Workarounds] finished patch_container_frame_token_tracker in 58.88µs
[  3.041s] [Workarounds] starting patch_achievement_display_set_achievements
[  3.041s] [Workarounds] finished patch_achievement_display_set_achievements in 12.41µs
[  3.041s] [Workarounds] starting patch_housing_dashboard_preload
[  3.041s] [Workarounds] finished patch_housing_dashboard_preload in 87.71µs
[  3.041s] [Workarounds] starting patch_lfg_lock_list
[  3.041s] [Workarounds] finished patch_lfg_lock_list in 8.75µs
[  3.041s] [Workarounds] starting patch_auction_house_browse_results_event
[  3.042s] [Workarounds] finished patch_auction_house_browse_results_event in 100.51µs
[  3.042s] [Workarounds] starting patch_auction_house_search_context_aliases
[  3.042s] [Workarounds] finished patch_auction_house_search_context_aliases in 45.57µs
[  3.042s] [Workarounds] starting patch_auth_challenge_frame_parent
[  3.042s] [Workarounds] finished patch_auth_challenge_frame_parent in 64.60µs
[  3.042s] [Workarounds] starting patch_settings_surface_defaults
[  3.042s] [Workarounds] finished patch_settings_surface_defaults in 515.18µs
[  3.042s] [Workarounds] starting patch_settings_canvas_layout_visibility
[  3.042s] [Workarounds] finished patch_settings_canvas_layout_visibility in 163.16µs
[  3.042s] [Startup] apply_post_load_workarounds: workarounds complete
[  3.137s] [Startup] apply_post_load_workarounds: globals restored
[  3.167s] [Startup] apply_post_load_workarounds: patch 12.1 compatibility restored
[  3.167s] [Startup] apply_post_load_workarounds: seterrorhandler restored
[  3.167s] [Startup] post-load workarounds complete in 293.58ms
[  3.167s] [Startup] restarting GC after bootstrap
[  3.371s] [Startup] GC restart complete in 203.24ms
Lua error: [exec-lua] error: Lua error: SetAlpha must reject out-of-range numeric alpha: -0.25
stack traceback:
	(string):1: in main chunk
Lua error: Lua Error: [exec-lua] error: Lua error: SetAlpha must reject out-of-range numeric alpha: -0.25
stack traceback:
	(string):1: in main chunk
[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:59: in function 'GetErrorData'
[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:72: in function <[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:71>

=== Final observed Lua error summary ===
Status: FAILED
Lua errors: 2 unique, 2 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 2 occurrence(s)
```

#### result.json (complete, 622 bytes)

```text
{
  "source": "cd9f42756472b8f8d706fd3e592f1977b3d89868",
  "profile": "retail",
  "filter": "shared ordinary-alpha fixture invalid",
  "expected_cases": 0,
  "launcher_exit": 1,
  "summaries": [],
  "stdout_bytes": 615,
  "stderr_bytes": 15246,
  "receipt_stat_reused": {
    "st_dev": 66306,
    "st_ino": 148916796,
    "st_size": 408310896,
    "st_mtime_ns": 1791943848752615957
  },
  "unit": "<UNIT_05>",
  "scope": "Pre-producer shared complete Lua fixture on frozen retail normal binary: ordinary numeric alpha invalid; not newlycompiledRusttest/native/render proof"
}
```

### ptr valid

Directory: `<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-ptr-green-20261014T021515390590Z/runtime-region-alpha-valid-red-20261014T031743299542Z`

#### admission.json (complete, 2403 bytes)

```text
{
  "utc": "2026-10-14T03:17:43.329669+00:00",
  "raw": {
    "memory.current": "28907409408\n",
    "memory.high": "42949672960\n",
    "memory.max": "45097156608\n",
    "memory.stat": "anon 11204788224\nfile 14837391360\nkernel 2769080320\nkernel_stack 5947392\npagetables 55934976\nsec_pagetables 0\npercpu 20710624\nsock 0\nvmalloc 0\nshmem 1576853504\nzswap 627755084\nzswapped 2602741760\nzswap_incomp 16076800\nfile_mapped 897056768\nfile_dirty 344064\nfile_writeback 0\nswapcached 724135936\nanon_thp 90177536\nfile_thp 109051904\nshmem_thp 0\ninactive_anon 3918217216\nactive_anon 8947687424\ninactive_file 6465642496\nactive_file 6794817536\nunevictable 0\nslab_reclaimable 2015776192\nslab_unreclaimable 30728336\nslab 2046504528\nhugetlb 0\nworkingset_refault_anon 913078589\nworkingset_refault_file 1171686664\nworkingset_activate_anon 90574273\nworkingset_activate_file 48938070\nworkingset_restore_anon 93459777\nworkingset_restore_file 115602286\nworkingset_nodereclaim 426322757\npgdemote_kswapd 0\npgdemote_direct 0\npgdemote_khugepaged 0\npgdemote_proactive 0\npgsteal_kswapd 12457927611\npgsteal_direct 8638355554\npgsteal_khugepaged 93420\npgsteal_proactive 0\npgscan_kswapd 13612909480\npgscan_direct 12107092579\npgscan_khugepaged 112894\npgscan_proactive 0\npgrefill 10149524946\npgpromote_success 0\npgscan 25720114953\npgsteal 21096376585\npswpin 75131443\npswpout 284192717\npgfault 45514946941\npgmajfault 1125625289\npgactivate 6441815999\npgdeactivate 0\npglazyfree 0\npglazyfreed 1\nswpin_zero 4468648\nswpout_zero 27719445\nzswpin 549693070\nzswpout 1799058401\nzswpwb 284620882\nthp_fault_alloc 4695855\nthp_collapse_alloc 4895\nthp_swpout 0\nthp_swpout_fallback 35555\nnuma_pages_migrated 0\nnuma_pte_updates 0\nnuma_hint_faults 0\n",
    "memory.pressure": "some avg10=0.34 avg60=1.34 avg300=2.33 total=6979492246\nfull avg10=0.19 avg60=0.69 avg300=1.83 total=6042563610\n",
    "cpu.pressure": "some avg10=1.98 avg60=10.21 avg300=9.72 total=27443138830\nfull avg10=0.19 avg60=1.06 avg300=0.75 total=2565556953\n",
    "io.pressure": "some avg10=6.02 avg60=8.81 avg300=10.88 total=17062600257\nfull avg10=5.14 avg60=5.48 avg300=7.98 total=12984650455\n"
  },
  "half_clean_file_and_slab_reclaim": 7189456608,
  "projected": 26012920096,
  "host_available": 31300276224,
  "allowed": true,
  "worker_max": 4294967296,
  "worker_high": 3221225472,
  "profile": "ptr"
}
```

#### argv.json (complete, 3137 bytes)

```text
[
  "systemd-run",
  "--user",
  "--wait",
  "--unit=<UNIT_02>",
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
  "StandardOutput=file:<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-ptr-green-20261014T021515390590Z/runtime-region-alpha-valid-red-20261014T031743299542Z/stdout",
  "-p",
  "StandardError=file:<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-ptr-green-20261014T021515390590Z/runtime-region-alpha-valid-red-20261014T031743299542Z/stderr",
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
  "<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-ptr-green-20261014T021515390590Z/source",
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
  "<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-ptr-green-20261014T021515390590Z/artifacts/wow-sim",
  "--no-addons",
  "--no-saved-vars",
  "--exec-lua",
  "alphaContractMode = 'valid'\nlocal parent = CreateFrame(\"Frame\", nil, UIParent)\nparent:SetAlpha(0.5)\nlocal frame = CreateFrame(\"Frame\", nil, parent)\nlocal regions = { frame, frame:CreateTexture(), frame:CreateFontString() }\n\nif alphaContractMode == \"valid\" then\n    for _, region in ipairs(regions) do\n        for _, alpha in ipairs({ 0, 0.5, 1 }) do\n            assert(select(\"#\", region:SetAlpha(alpha)) == 0, \"SetAlpha returns no values\")\n            assert(region:GetAlpha() == alpha, \"valid alpha must roundtrip\")\n        end\n    end\nelseif alphaContractMode == \"invalid\" then\n    local nearBoundary = 2 ^ -30\n    local invalid = { -0.25, 1.25, -nearBoundary, 1 + nearBoundary, -math.huge, math.huge, 0 / 0 }\n    for _, region in ipairs(regions) do\n        region:SetAlpha(0.625)\n        local effectiveBefore = region:GetEffectiveAlpha()\n        for _, alpha in ipairs(invalid) do\n            local accepted = pcall(function() region:SetAlpha(alpha) end)\n            assert(not accepted, \"SetAlpha must reject out-of-range numeric alpha: \" .. tostring(alpha))\n            assert(region:GetAlpha() == 0.625, \"rejection must preserve stored alpha\")\n            assert(region:GetEffectiveAlpha() == effectiveBefore, \"rejection must preserve inherited alpha\")\n        end\n    end\nelse\n    error(\"unknown alpha contract mode\")\nend\nprint(\"REGION_ALPHA_CONTRACT_COMPLETE\", alphaContractMode)\n",
  "lua-errors"
]
```

#### launcher.txt (complete, 304 bytes)

```text
Running as unit: <UNIT_02>.service
          Finished with result: success
Main processes terminated with: code=exited, status=0/SUCCESS
               Service runtime: 8.964s
             CPU time consumed: 6.142s
                   Memory peak: 1.4G (swap: 0B)
```

#### stdout (complete, 40 bytes)

```text
REGION_ALPHA_CONTRACT_COMPLETE	valid
[]
```

#### stderr (complete, 14690 bytes)

```text
[  0.000s] [startup] wow-sim process started pid=<PID_03>
[  0.000s] Resource limits: 2 CPU core(s)
[  0.035s] [Startup] WowLuaEnv::new begin
[  0.182s] [Startup] SimState::default complete in 142.79ms
[  0.203s] [Startup] rilua VM created in 21.22ms
[  0.214s] [Startup] builtin frames initialized in 10.37ms
[  0.224s] [Startup] template registry cleared in 9.84ms
[  0.224s] [Startup] intrinsic templates registered in 13.58µs
[  0.224s] [Startup] initial app_data lua handle installed in 1.29µs
[  0.709s] [Startup] init_lua_state complete in 485.13ms
[  0.709s] [Startup] final app_data lua handle installed in 351.00ns
[  0.709s] [Startup] initial screen globals installed in 250.52µs
[  0.709s] [Startup] WowLuaEnv::new complete
[  0.710s] [Startup] screen size set to 1600x1200 in 1.25ms
[  0.710s] [Startup] WowFontSystem::new begin casc=false
[  0.729s] [Startup] font system fallback loaded in 18.76ms
[  0.735s] [Startup] cosmic font system built in 5.33ms
[  0.737s] [Startup] swash cache created in 2.09ms
[  0.737s] [Startup] WowFontSystem::new complete in 26.29ms
[  0.738s] [Startup] font system created in 27.14ms
[  0.746s] Sound disabled
[  0.768s] [Startup] begin stop GC
[  0.768s] [Startup] end stop GC in 461.00ns
[  0.768s] [Startup] begin configure saved variables
[  0.768s] SavedVariables loading disabled
[  0.768s] [Startup] end configure saved variables in 4.34µs
[  0.768s] [Startup] begin load keybindings
[  0.768s] [Startup] end load keybindings in 220.00ns
[  0.768s] [Startup] begin configure edit mode cache
[  0.783s] EditMode cache source (read-only): <CHARACTER_ID_02> @ <CHARACTER_02>
[  0.783s] [Startup] end configure edit mode cache in 15.69ms
[  0.783s] [Startup] begin read ServerSnapshot edit mode layout
[  0.783s] [Startup] end read ServerSnapshot edit mode layout in 40.00ns
[  0.783s] [Startup] begin load edit mode cache
[  0.795s] Loaded EditMode layout cache from WTF
[  0.795s] [Startup] end load edit mode cache in 11.20ms
[  0.795s] [Startup] begin load ServerSnapshot character state
[  0.795s] [Startup] end load ServerSnapshot character state in 160.00ns
[  0.795s] [Startup] begin load Blizzard addons
[  1.340s] Loading 283 Blizzard addons...
[  6.131s] Blizzard addons loaded in 5.33s (io=947.91ms xml=641.40ms xmlproc=2.20s frames⊂xmlproc=2.18s lua=612.67ms [compile=458.63ms call=154.04ms] gc=deferred, bytecode cache: 1568/1568 hits (100%, lookup_miss=0 replay_fail=0 stored=0 store_fail=0))
  frame breakdown: setup=1.01s finalize=2.59s (2481 frames)
  setup: code_build=13.23ms exec_lua=995.23ms props=11.37ms
  finalize: layers=68.85ms (1148 tex, 685 fs) anim=24.10ms button=55.42ms lifecycle=973.92ms (1760 fires)
[  6.131s] [Startup] end load Blizzard addons in 5.34s
[  6.131s] [Startup] begin prepare chat frame for third-party addons
[  6.132s] [Startup] end prepare chat frame for third-party addons in 892.36µs
[  6.132s] [Startup] begin apply post-Blizzard load workarounds
[  6.132s] [Startup] applying post-load workarounds
[  6.072s] [Startup] apply_post_load_workarounds: begin
[  6.072s] [Workarounds] starting patch_edit_mode_manager
[  6.072s] [Workarounds] finished patch_edit_mode_manager in 463.57µs
[  6.073s] [Workarounds] starting init_edit_mode_layout
[  6.073s] [EditMode] starting setup_layout_info
[  6.085s] [EditMode] finished setup_layout_info in 12.30ms
[  6.085s] [EditMode] starting apply_system_anchors
[  6.319s] [EditMode] finished apply_system_anchors in 234.36ms
[  6.319s] [EditMode] starting finalize_action_bar_positions
[  6.322s] [EditMode] finished finalize_action_bar_positions in 2.61ms
[  6.322s] [EditMode] starting invoke_anchor_changed_hooks
[  6.322s] [EditMode] finished invoke_anchor_changed_hooks in 554.66µs
[  6.322s] [EditMode] starting fix_action_bar_nan_size
[  6.322s] [EditMode] finished fix_action_bar_nan_size in 56.75µs
[  6.322s] [EditMode] starting fix_action_bar_scale
[  6.322s] [EditMode] finished fix_action_bar_scale in 6.07µs
[  6.322s] [Workarounds] finished init_edit_mode_layout in 249.97ms
[  6.322s] [Workarounds] starting patch_ui_parent_panel_toggles
[  6.323s] [Workarounds] finished patch_ui_parent_panel_toggles in 323.88µs
[  6.323s] [Workarounds] starting patch_uiparent_onupdate_worklists
[  6.323s] [Workarounds] finished patch_uiparent_onupdate_worklists in 145.80µs
[  6.323s] [Workarounds] starting init_chat_type_colors
[  6.323s] [Workarounds] finished init_chat_type_colors in 158.35µs
[  6.323s] [Workarounds] starting patch_chat_voice_button_surface
[  6.323s] [Workarounds] finished patch_chat_voice_button_surface in 155.16µs
[  6.323s] [Workarounds] starting patch_item_socketing_tooltips
[  6.323s] [Workarounds] finished patch_item_socketing_tooltips in 86.27µs
[  6.323s] [Workarounds] starting patch_character_select_selected_name
[  6.323s] [Workarounds] finished patch_character_select_selected_name in 46.61µs
[  6.323s] [Workarounds] starting patch_character_create_defaults
[  6.324s] [Workarounds] finished patch_character_create_defaults in 332.44µs
[  6.324s] [Workarounds] starting patch_character_frame_title_refresh
[  6.345s] [Workarounds] finished patch_character_frame_title_refresh in 21.25ms
[  6.345s] [Workarounds] starting patch_vignette_pin_template
[  6.345s] [Workarounds] finished patch_vignette_pin_template in 146.07µs
[  6.345s] [Workarounds] starting patch_fog_of_war_pin_mixin
[  6.346s] [Workarounds] finished patch_fog_of_war_pin_mixin in 374.27µs
[  6.346s] [Workarounds] starting patch_map_exploration_pin_mixin
[  6.365s] [Workarounds] finished patch_map_exploration_pin_mixin in 19.12ms
[  6.365s] [Workarounds] starting patch_map_canvas_data_provider_attachment
[  6.365s] [Workarounds] finished patch_map_canvas_data_provider_attachment in 108.45µs
[  6.365s] [Workarounds] starting ensure_adventure_map_frame_surface
[  6.365s] [Workarounds] finished ensure_adventure_map_frame_surface in 679.35µs
[  6.365s] [Workarounds] starting patch_action_bar_button_event_fanout
[  6.366s] [Workarounds] finished patch_action_bar_button_event_fanout in 143.57µs
[  6.366s] [Workarounds] starting patch_paging_controls_page_text
[  6.366s] [Workarounds] finished patch_paging_controls_page_text in 54.39µs
[  6.366s] [Workarounds] starting patch_talent_edge_frame_level_sync
[  6.366s] [Workarounds] finished patch_talent_edge_frame_level_sync in 41.48µs
[  6.366s] [Workarounds] starting patch_catalog_shop_product_card_defaults
[  6.366s] [Workarounds] finished patch_catalog_shop_product_card_defaults in 85.69µs
[  6.366s] [Workarounds] starting patch_game_time_defaults
[  6.366s] [Workarounds] finished patch_game_time_defaults in 8.80µs
[  6.366s] [Workarounds] starting patch_tooltip_nineslice_surface
[  6.366s] [Workarounds] finished patch_tooltip_nineslice_surface in 67.84µs
[  6.366s] [Workarounds] starting patch_container_frame_token_tracker
[  6.366s] [Workarounds] finished patch_container_frame_token_tracker in 60.58µs
[  6.366s] [Workarounds] starting patch_achievement_display_set_achievements
[  6.366s] [Workarounds] finished patch_achievement_display_set_achievements in 10.96µs
[  6.366s] [Workarounds] starting patch_housing_dashboard_preload
[  6.366s] [Workarounds] finished patch_housing_dashboard_preload in 84.14µs
[  6.366s] [Workarounds] starting patch_lfg_lock_list
[  6.366s] [Workarounds] finished patch_lfg_lock_list in 52.43µs
[  6.366s] [Workarounds] starting patch_auction_house_browse_results_event
[  6.366s] [Workarounds] finished patch_auction_house_browse_results_event in 95.36µs
[  6.366s] [Workarounds] starting patch_auction_house_search_context_aliases
[  6.366s] [Workarounds] finished patch_auction_house_search_context_aliases in 42.97µs
[  6.366s] [Workarounds] starting patch_auth_challenge_frame_parent
[  6.366s] [Workarounds] finished patch_auth_challenge_frame_parent in 94.38µs
[  6.366s] [Workarounds] starting patch_settings_surface_defaults
[  6.367s] [Workarounds] finished patch_settings_surface_defaults in 500.57µs
[  6.367s] [Workarounds] starting patch_settings_canvas_layout_visibility
[  6.367s] [Workarounds] finished patch_settings_canvas_layout_visibility in 156.66µs
[  6.367s] [Startup] apply_post_load_workarounds: workarounds complete
[  6.466s] [Startup] apply_post_load_workarounds: globals restored
[  6.467s] [Startup] apply_post_load_workarounds: patch 12.1 compatibility restored
[  6.467s] [Startup] apply_post_load_workarounds: seterrorhandler restored
[  6.527s] [Startup] post-load workarounds complete in 394.92ms
[  6.527s] [Startup] end apply post-Blizzard load workarounds in 394.93ms
[  6.527s] [Startup] begin load third-party addons
[  6.527s] Addon loading disabled
[  6.527s] [Startup] end load third-party addons in 1.80µs
[  6.527s] [Startup] begin sync addon names to Lua
[  6.527s] [Startup] end sync addon names to Lua in 410.66µs
[  6.527s] [Startup] applying post-load workarounds
[  6.467s] [Startup] apply_post_load_workarounds: begin
[  6.467s] [Workarounds] starting patch_edit_mode_manager
[  6.468s] [Workarounds] finished patch_edit_mode_manager in 487.10µs
[  6.468s] [Workarounds] starting init_edit_mode_layout
[  6.468s] [EditMode] starting setup_layout_info
[  6.483s] [EditMode] finished setup_layout_info in 14.74ms
[  6.483s] [EditMode] starting apply_system_anchors
[  6.674s] [EditMode] finished apply_system_anchors in 190.86ms
[  6.674s] [EditMode] starting finalize_action_bar_positions
[  6.676s] [EditMode] finished finalize_action_bar_positions in 2.93ms
[  6.676s] [EditMode] starting invoke_anchor_changed_hooks
[  6.682s] [EditMode] finished invoke_anchor_changed_hooks in 5.39ms
[  6.682s] [EditMode] starting fix_action_bar_nan_size
[  6.682s] [EditMode] finished fix_action_bar_nan_size in 69.77µs
[  6.682s] [EditMode] starting fix_action_bar_scale
[  6.682s] [EditMode] finished fix_action_bar_scale in 6.88µs
[  6.682s] [Workarounds] finished init_edit_mode_layout in 214.09ms
[  6.682s] [Workarounds] starting patch_ui_parent_panel_toggles
[  6.682s] [Workarounds] finished patch_ui_parent_panel_toggles in 313.52µs
[  6.682s] [Workarounds] starting patch_uiparent_onupdate_worklists
[  6.682s] [Workarounds] finished patch_uiparent_onupdate_worklists in 151.73µs
[  6.682s] [Workarounds] starting init_chat_type_colors
[  6.683s] [Workarounds] finished init_chat_type_colors in 157.20µs
[  6.683s] [Workarounds] starting patch_chat_voice_button_surface
[  6.683s] [Workarounds] finished patch_chat_voice_button_surface in 334.85µs
[  6.683s] [Workarounds] starting patch_item_socketing_tooltips
[  6.683s] [Workarounds] finished patch_item_socketing_tooltips in 86.76µs
[  6.683s] [Workarounds] starting patch_character_select_selected_name
[  6.683s] [Workarounds] finished patch_character_select_selected_name in 46.53µs
[  6.683s] [Workarounds] starting patch_character_create_defaults
[  6.683s] [Workarounds] finished patch_character_create_defaults in 322.69µs
[  6.683s] [Workarounds] starting patch_character_frame_title_refresh
[  6.695s] [Workarounds] finished patch_character_frame_title_refresh in 11.42ms
[  6.695s] [Workarounds] starting patch_vignette_pin_template
[  6.695s] [Workarounds] finished patch_vignette_pin_template in 150.14µs
[  6.695s] [Workarounds] starting patch_fog_of_war_pin_mixin
[  6.695s] [Workarounds] finished patch_fog_of_war_pin_mixin in 359.91µs
[  6.695s] [Workarounds] starting patch_map_exploration_pin_mixin
[  6.696s] [Workarounds] finished patch_map_exploration_pin_mixin in 486.40µs
[  6.696s] [Workarounds] starting patch_map_canvas_data_provider_attachment
[  6.696s] [Workarounds] finished patch_map_canvas_data_provider_attachment in 87.91µs
[  6.696s] [Workarounds] starting ensure_adventure_map_frame_surface
[  6.696s] [Workarounds] finished ensure_adventure_map_frame_surface in 221.66µs
[  6.696s] [Workarounds] starting patch_action_bar_button_event_fanout
[  6.696s] [Workarounds] finished patch_action_bar_button_event_fanout in 139.89µs
[  6.696s] [Workarounds] starting patch_paging_controls_page_text
[  6.696s] [Workarounds] finished patch_paging_controls_page_text in 53.30µs
[  6.696s] [Workarounds] starting patch_talent_edge_frame_level_sync
[  6.696s] [Workarounds] finished patch_talent_edge_frame_level_sync in 39.18µs
[  6.696s] [Workarounds] starting patch_catalog_shop_product_card_defaults
[  6.697s] [Workarounds] finished patch_catalog_shop_product_card_defaults in 83.45µs
[  6.697s] [Workarounds] starting patch_game_time_defaults
[  6.697s] [Workarounds] finished patch_game_time_defaults in 8.49µs
[  6.697s] [Workarounds] starting patch_tooltip_nineslice_surface
[  6.697s] [Workarounds] finished patch_tooltip_nineslice_surface in 64.31µs
[  6.697s] [Workarounds] starting patch_container_frame_token_tracker
[  6.697s] [Workarounds] finished patch_container_frame_token_tracker in 58.14µs
[  6.697s] [Workarounds] starting patch_achievement_display_set_achievements
[  6.697s] [Workarounds] finished patch_achievement_display_set_achievements in 11.95µs
[  6.697s] [Workarounds] starting patch_housing_dashboard_preload
[  6.697s] [Workarounds] finished patch_housing_dashboard_preload in 80.06µs
[  6.697s] [Workarounds] starting patch_lfg_lock_list
[  6.697s] [Workarounds] finished patch_lfg_lock_list in 8.10µs
[  6.697s] [Workarounds] starting patch_auction_house_browse_results_event
[  6.697s] [Workarounds] finished patch_auction_house_browse_results_event in 100.82µs
[  6.697s] [Workarounds] starting patch_auction_house_search_context_aliases
[  6.697s] [Workarounds] finished patch_auction_house_search_context_aliases in 45.43µs
[  6.697s] [Workarounds] starting patch_auth_challenge_frame_parent
[  6.697s] [Workarounds] finished patch_auth_challenge_frame_parent in 61.99µs
[  6.697s] [Workarounds] starting patch_settings_surface_defaults
[  6.698s] [Workarounds] finished patch_settings_surface_defaults in 494.39µs
[  6.698s] [Workarounds] starting patch_settings_canvas_layout_visibility
[  6.698s] [Workarounds] finished patch_settings_canvas_layout_visibility in 156.62µs
[  6.698s] [Startup] apply_post_load_workarounds: workarounds complete
[  6.795s] [Startup] apply_post_load_workarounds: globals restored
[  6.796s] [Startup] apply_post_load_workarounds: patch 12.1 compatibility restored
[  6.796s] [Startup] apply_post_load_workarounds: seterrorhandler restored
[  6.856s] [Startup] post-load workarounds complete in 328.58ms
[  6.856s] [Startup] restarting GC after bootstrap
[  7.055s] [Startup] GC restart complete in 199.58ms
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```

#### result.json (complete, 608 bytes)

```text
{
  "source": "a5397a2d60ebdd2dc813309671bfb26534e3621c",
  "profile": "ptr",
  "filter": "shared ordinary-alpha fixture valid",
  "expected_cases": 0,
  "launcher_exit": 0,
  "summaries": [],
  "stdout_bytes": 40,
  "stderr_bytes": 14690,
  "receipt_stat_reused": {
    "st_dev": 66306,
    "st_ino": 149051649,
    "st_size": 415341528,
    "st_mtime_ns": 1791944301479256281
  },
  "unit": "<UNIT_02>",
  "scope": "Pre-producer shared complete Lua fixture on frozen ptr normal binary: ordinary numeric alpha valid; not newlycompiledRusttest/native/render proof"
}
```

### ptr invalid

Directory: `<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-ptr-green-20261014T021515390590Z/runtime-region-alpha-invalid-red-20261014T031753482479Z`

#### admission.json (complete, 2401 bytes)

```text
{
  "utc": "2026-10-14T03:17:53.500633+00:00",
  "raw": {
    "memory.current": "28395073536\n",
    "memory.high": "42949672960\n",
    "memory.max": "45097156608\n",
    "memory.stat": "anon 11853791232\nfile 13748576256\nkernel 2699046912\nkernel_stack 6406144\npagetables 57180160\nsec_pagetables 0\npercpu 20644256\nsock 0\nvmalloc 0\nshmem 803631104\nzswap 715986595\nzswapped 2868387840\nzswap_incomp 16367616\nfile_mapped 851619840\nfile_dirty 327680\nfile_writeback 0\nswapcached 767631360\nanon_thp 88080384\nfile_thp 178257920\nshmem_thp 0\ninactive_anon 2948800512\nactive_anon 9793335296\ninactive_file 7960113152\nactive_file 4984819712\nunevictable 0\nslab_reclaimable 1855697840\nslab_unreclaimable 30947888\nslab 1886645728\nhugetlb 0\nworkingset_refault_anon 913093875\nworkingset_refault_file 1171694303\nworkingset_activate_anon 90574354\nworkingset_activate_file 48938074\nworkingset_restore_anon 93459858\nworkingset_restore_file 115602290\nworkingset_nodereclaim 426325189\npgdemote_kswapd 0\npgdemote_direct 0\npgdemote_khugepaged 0\npgdemote_proactive 0\npgsteal_kswapd 12458208659\npgsteal_direct 8638390789\npgsteal_khugepaged 93420\npgsteal_proactive 0\npgscan_kswapd 13613219280\npgscan_direct 12107129240\npgscan_khugepaged 112894\npgscan_proactive 0\npgrefill 10149946634\npgpromote_success 0\npgscan 25720461414\npgsteal 21096692868\npswpin 75133628\npswpout 284192717\npgfault 45516739577\npgmajfault 1125637980\npgactivate 6441851989\npgdeactivate 0\npglazyfree 0\npglazyfreed 1\nswpin_zero 4468723\nswpout_zero 27721819\nzswpin 549706096\nzswpout 1799140960\nzswpwb 284620882\nthp_fault_alloc 4695859\nthp_collapse_alloc 4895\nthp_swpout 0\nthp_swpout_fallback 35555\nnuma_pages_migrated 0\nnuma_pte_updates 0\nnuma_hint_faults 0\n",
    "memory.pressure": "some avg10=0.12 avg60=1.13 avg300=2.25 total=6979502622\nfull avg10=0.07 avg60=0.58 avg300=1.77 total=6042573986\n",
    "cpu.pressure": "some avg10=0.72 avg60=8.64 avg300=9.39 total=27443164749\nfull avg10=0.07 avg60=0.90 avg300=0.72 total=2565568359\n",
    "io.pressure": "some avg10=4.34 avg60=8.05 avg300=10.65 total=17063105905\nfull avg10=3.92 avg60=5.20 avg300=7.83 total=12985127614\n"
  },
  "half_clean_file_and_slab_reclaim": 6974347736,
  "projected": 25715693096,
  "host_available": 31441006592,
  "allowed": true,
  "worker_max": 4294967296,
  "worker_high": 3221225472,
  "profile": "ptr"
}
```

#### argv.json (complete, 3143 bytes)

```text
[
  "systemd-run",
  "--user",
  "--wait",
  "--unit=<UNIT_03>",
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
  "StandardOutput=file:<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-ptr-green-20261014T021515390590Z/runtime-region-alpha-invalid-red-20261014T031753482479Z/stdout",
  "-p",
  "StandardError=file:<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-ptr-green-20261014T021515390590Z/runtime-region-alpha-invalid-red-20261014T031753482479Z/stderr",
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
  "<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-ptr-green-20261014T021515390590Z/source",
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
  "<PRIVATE_V>/widget-discrete-progress-current/crafted-quality-ptr-green-20261014T021515390590Z/artifacts/wow-sim",
  "--no-addons",
  "--no-saved-vars",
  "--exec-lua",
  "alphaContractMode = 'invalid'\nlocal parent = CreateFrame(\"Frame\", nil, UIParent)\nparent:SetAlpha(0.5)\nlocal frame = CreateFrame(\"Frame\", nil, parent)\nlocal regions = { frame, frame:CreateTexture(), frame:CreateFontString() }\n\nif alphaContractMode == \"valid\" then\n    for _, region in ipairs(regions) do\n        for _, alpha in ipairs({ 0, 0.5, 1 }) do\n            assert(select(\"#\", region:SetAlpha(alpha)) == 0, \"SetAlpha returns no values\")\n            assert(region:GetAlpha() == alpha, \"valid alpha must roundtrip\")\n        end\n    end\nelseif alphaContractMode == \"invalid\" then\n    local nearBoundary = 2 ^ -30\n    local invalid = { -0.25, 1.25, -nearBoundary, 1 + nearBoundary, -math.huge, math.huge, 0 / 0 }\n    for _, region in ipairs(regions) do\n        region:SetAlpha(0.625)\n        local effectiveBefore = region:GetEffectiveAlpha()\n        for _, alpha in ipairs(invalid) do\n            local accepted = pcall(function() region:SetAlpha(alpha) end)\n            assert(not accepted, \"SetAlpha must reject out-of-range numeric alpha: \" .. tostring(alpha))\n            assert(region:GetAlpha() == 0.625, \"rejection must preserve stored alpha\")\n            assert(region:GetEffectiveAlpha() == effectiveBefore, \"rejection must preserve inherited alpha\")\n        end\n    end\nelse\n    error(\"unknown alpha contract mode\")\nend\nprint(\"REGION_ALPHA_CONTRACT_COMPLETE\", alphaContractMode)\n",
  "lua-errors"
]
```

#### launcher.txt (complete, 306 bytes)

```text
Running as unit: <UNIT_03>.service
          Finished with result: exit-code
Main processes terminated with: code=exited, status=1/FAILURE
               Service runtime: 7.076s
             CPU time consumed: 5.992s
                   Memory peak: 1.5G (swap: 0B)
```

#### stdout (complete, 615 bytes)

```text
[
  {
    "message": "[exec-lua] error: Lua error: SetAlpha must reject out-of-range numeric alpha: -0.25\nstack traceback:\n\t(string):1: in main chunk",
    "count": 1
  },
  {
    "message": "Lua Error: [exec-lua] error: Lua error: SetAlpha must reject out-of-range numeric alpha: -0.25\nstack traceback:\n\t(string):1: in main chunk\n[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:59: in function 'GetErrorData'\n[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:72: in function <[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:71>",
    "count": 1
  }
]
```

#### stderr (complete, 15233 bytes)

```text
[  0.000s] [startup] wow-sim process started pid=<PID_04>
[  0.000s] Resource limits: 2 CPU core(s)
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.003s] [Startup] SimState::default complete in 2.96ms
[  0.003s] [Startup] rilua VM created in 155.21µs
[  0.004s] [Startup] builtin frames initialized in 100.43µs
[  0.004s] [Startup] template registry cleared in 19.45µs
[  0.004s] [Startup] intrinsic templates registered in 6.76µs
[  0.004s] [Startup] initial app_data lua handle installed in 701.00ns
[  0.189s] [Startup] init_lua_state complete in 185.30ms
[  0.189s] [Startup] final app_data lua handle installed in 681.00ns
[  0.189s] [Startup] initial screen globals installed in 107.82µs
[  0.189s] [Startup] WowLuaEnv::new complete
[  0.189s] [Startup] screen size set to 1600x1200 in 46.68µs
[  0.189s] [Startup] WowFontSystem::new begin casc=false
[  0.199s] [Startup] font system fallback loaded in 9.69ms
[  0.199s] [Startup] cosmic font system built in 21.85µs
[  0.199s] [Startup] swash cache created in 4.99µs
[  0.199s] [Startup] WowFontSystem::new complete in 9.74ms
[  0.199s] [Startup] font system created in 9.76ms
[  0.199s] Sound disabled
[  0.200s] [Startup] begin stop GC
[  0.200s] [Startup] end stop GC in 431.00ns
[  0.200s] [Startup] begin configure saved variables
[  0.200s] SavedVariables loading disabled
[  0.200s] [Startup] end configure saved variables in 1.90µs
[  0.200s] [Startup] begin load keybindings
[  0.200s] [Startup] end load keybindings in 151.00ns
[  0.200s] [Startup] begin configure edit mode cache
[  0.200s] EditMode cache source (read-only): <CHARACTER_ID_02> @ <CHARACTER_02>
[  0.200s] [Startup] end configure edit mode cache in 63.60µs
[  0.200s] [Startup] begin read ServerSnapshot edit mode layout
[  0.200s] [Startup] end read ServerSnapshot edit mode layout in 30.00ns
[  0.200s] [Startup] begin load edit mode cache
[  0.201s] Loaded EditMode layout cache from WTF
[  0.201s] [Startup] end load edit mode cache in 500.70µs
[  0.201s] [Startup] begin load ServerSnapshot character state
[  0.201s] [Startup] end load ServerSnapshot character state in 40.00ns
[  0.201s] [Startup] begin load Blizzard addons
[  0.211s] Loading 283 Blizzard addons...
[  4.816s] Blizzard addons loaded in 4.61s (io=13.39ms xml=233.17ms xmlproc=2.26s frames⊂xmlproc=2.24s lua=1.84s [compile=1.68s call=159.99ms] gc=deferred, bytecode cache: 1568/1568 hits (100%, lookup_miss=0 replay_fail=0 stored=0 store_fail=0))
  frame breakdown: setup=1.08s finalize=2.61s (2481 frames)
  setup: code_build=11.90ms exec_lua=1.07s props=11.43ms
  finalize: layers=69.93ms (1148 tex, 685 fs) anim=19.97ms button=40.23ms lifecycle=984.26ms (1760 fires)
[  4.816s] [Startup] end load Blizzard addons in 4.62s
[  4.816s] [Startup] begin prepare chat frame for third-party addons
[  4.817s] [Startup] end prepare chat frame for third-party addons in 1.02ms
[  4.817s] [Startup] begin apply post-Blizzard load workarounds
[  4.817s] [Startup] applying post-load workarounds
[  4.817s] [Startup] apply_post_load_workarounds: begin
[  4.817s] [Workarounds] starting patch_edit_mode_manager
[  4.817s] [Workarounds] finished patch_edit_mode_manager in 510.61µs
[  4.817s] [Workarounds] starting init_edit_mode_layout
[  4.817s] [EditMode] starting setup_layout_info
[  4.830s] [EditMode] finished setup_layout_info in 12.56ms
[  4.830s] [EditMode] starting apply_system_anchors
[  5.077s] [EditMode] finished apply_system_anchors in 246.99ms
[  5.077s] [EditMode] starting finalize_action_bar_positions
[  5.080s] [EditMode] finished finalize_action_bar_positions in 3.72ms
[  5.080s] [EditMode] starting invoke_anchor_changed_hooks
[  5.081s] [EditMode] finished invoke_anchor_changed_hooks in 693.60µs
[  5.081s] [EditMode] starting fix_action_bar_nan_size
[  5.081s] [EditMode] finished fix_action_bar_nan_size in 71.93µs
[  5.081s] [EditMode] starting fix_action_bar_scale
[  5.081s] [EditMode] finished fix_action_bar_scale in 6.87µs
[  5.081s] [Workarounds] finished init_edit_mode_layout in 264.13ms
[  5.081s] [Workarounds] starting patch_ui_parent_panel_toggles
[  5.082s] [Workarounds] finished patch_ui_parent_panel_toggles in 359.24µs
[  5.082s] [Workarounds] starting patch_uiparent_onupdate_worklists
[  5.082s] [Workarounds] finished patch_uiparent_onupdate_worklists in 154.36µs
[  5.082s] [Workarounds] starting init_chat_type_colors
[  5.082s] [Workarounds] finished init_chat_type_colors in 182.83µs
[  5.082s] [Workarounds] starting patch_chat_voice_button_surface
[  5.082s] [Workarounds] finished patch_chat_voice_button_surface in 192.34µs
[  5.082s] [Workarounds] starting patch_item_socketing_tooltips
[  5.082s] [Workarounds] finished patch_item_socketing_tooltips in 93.73µs
[  5.082s] [Workarounds] starting patch_character_select_selected_name
[  5.082s] [Workarounds] finished patch_character_select_selected_name in 49.40µs
[  5.082s] [Workarounds] starting patch_character_create_defaults
[  5.083s] [Workarounds] finished patch_character_create_defaults in 361.63µs
[  5.083s] [Workarounds] starting patch_character_frame_title_refresh
[  5.093s] [Workarounds] finished patch_character_frame_title_refresh in 10.38ms
[  5.093s] [Workarounds] starting patch_vignette_pin_template
[  5.093s] [Workarounds] finished patch_vignette_pin_template in 212.86µs
[  5.093s] [Workarounds] starting patch_fog_of_war_pin_mixin
[  5.094s] [Workarounds] finished patch_fog_of_war_pin_mixin in 425.48µs
[  5.094s] [Workarounds] starting patch_map_exploration_pin_mixin
[  5.104s] [Workarounds] finished patch_map_exploration_pin_mixin in 10.44ms
[  5.104s] [Workarounds] starting patch_map_canvas_data_provider_attachment
[  5.104s] [Workarounds] finished patch_map_canvas_data_provider_attachment in 142.31µs
[  5.104s] [Workarounds] starting ensure_adventure_map_frame_surface
[  5.105s] [Workarounds] finished ensure_adventure_map_frame_surface in 781.63µs
[  5.105s] [Workarounds] starting patch_action_bar_button_event_fanout
[  5.105s] [Workarounds] finished patch_action_bar_button_event_fanout in 179.54µs
[  5.105s] [Workarounds] starting patch_paging_controls_page_text
[  5.105s] [Workarounds] finished patch_paging_controls_page_text in 58.27µs
[  5.105s] [Workarounds] starting patch_talent_edge_frame_level_sync
[  5.105s] [Workarounds] finished patch_talent_edge_frame_level_sync in 43.46µs
[  5.105s] [Workarounds] starting patch_catalog_shop_product_card_defaults
[  5.106s] [Workarounds] finished patch_catalog_shop_product_card_defaults in 95.16µs
[  5.106s] [Workarounds] starting patch_game_time_defaults
[  5.106s] [Workarounds] finished patch_game_time_defaults in 10.67µs
[  5.106s] [Workarounds] starting patch_tooltip_nineslice_surface
[  5.106s] [Workarounds] finished patch_tooltip_nineslice_surface in 70.45µs
[  5.106s] [Workarounds] starting patch_container_frame_token_tracker
[  5.106s] [Workarounds] finished patch_container_frame_token_tracker in 61.40µs
[  5.106s] [Workarounds] starting patch_achievement_display_set_achievements
[  5.106s] [Workarounds] finished patch_achievement_display_set_achievements in 12.50µs
[  5.106s] [Workarounds] starting patch_housing_dashboard_preload
[  5.106s] [Workarounds] finished patch_housing_dashboard_preload in 93.93µs
[  5.106s] [Workarounds] starting patch_lfg_lock_list
[  5.106s] [Workarounds] finished patch_lfg_lock_list in 54.65µs
[  5.106s] [Workarounds] starting patch_auction_house_browse_results_event
[  5.106s] [Workarounds] finished patch_auction_house_browse_results_event in 108.76µs
[  5.106s] [Workarounds] starting patch_auction_house_search_context_aliases
[  5.106s] [Workarounds] finished patch_auction_house_search_context_aliases in 46.07µs
[  5.106s] [Workarounds] starting patch_auth_challenge_frame_parent
[  5.106s] [Workarounds] finished patch_auth_challenge_frame_parent in 103.62µs
[  5.106s] [Workarounds] starting patch_settings_surface_defaults
[  5.107s] [Workarounds] finished patch_settings_surface_defaults in 536.98µs
[  5.107s] [Workarounds] starting patch_settings_canvas_layout_visibility
[  5.107s] [Workarounds] finished patch_settings_canvas_layout_visibility in 167.63µs
[  5.107s] [Startup] apply_post_load_workarounds: workarounds complete
[  5.219s] [Startup] apply_post_load_workarounds: globals restored
[  5.220s] [Startup] apply_post_load_workarounds: patch 12.1 compatibility restored
[  5.220s] [Startup] apply_post_load_workarounds: seterrorhandler restored
[  5.221s] [Startup] post-load workarounds complete in 403.74ms
[  5.221s] [Startup] end apply post-Blizzard load workarounds in 403.75ms
[  5.221s] [Startup] begin load third-party addons
[  5.221s] Addon loading disabled
[  5.221s] [Startup] end load third-party addons in 1.78µs
[  5.221s] [Startup] begin sync addon names to Lua
[  5.221s] [Startup] end sync addon names to Lua in 480.40µs
[  5.221s] [Startup] applying post-load workarounds
[  5.221s] [Startup] apply_post_load_workarounds: begin
[  5.221s] [Workarounds] starting patch_edit_mode_manager
[  5.221s] [Workarounds] finished patch_edit_mode_manager in 610.27µs
[  5.221s] [Workarounds] starting init_edit_mode_layout
[  5.221s] [EditMode] starting setup_layout_info
[  5.236s] [EditMode] finished setup_layout_info in 14.29ms
[  5.236s] [EditMode] starting apply_system_anchors
[  5.447s] [EditMode] finished apply_system_anchors in 211.41ms
[  5.447s] [EditMode] starting finalize_action_bar_positions
[  5.450s] [EditMode] finished finalize_action_bar_positions in 3.26ms
[  5.451s] [EditMode] starting invoke_anchor_changed_hooks
[  5.457s] [EditMode] finished invoke_anchor_changed_hooks in 6.17ms
[  5.457s] [EditMode] starting fix_action_bar_nan_size
[  5.457s] [EditMode] finished fix_action_bar_nan_size in 80.08µs
[  5.457s] [EditMode] starting fix_action_bar_scale
[  5.457s] [EditMode] finished fix_action_bar_scale in 8.70µs
[  5.457s] [Workarounds] finished init_edit_mode_layout in 235.30ms
[  5.457s] [Workarounds] starting patch_ui_parent_panel_toggles
[  5.457s] [Workarounds] finished patch_ui_parent_panel_toggles in 332.70µs
[  5.457s] [Workarounds] starting patch_uiparent_onupdate_worklists
[  5.457s] [Workarounds] finished patch_uiparent_onupdate_worklists in 162.65µs
[  5.457s] [Workarounds] starting init_chat_type_colors
[  5.457s] [Workarounds] finished init_chat_type_colors in 171.83µs
[  5.457s] [Workarounds] starting patch_chat_voice_button_surface
[  5.458s] [Workarounds] finished patch_chat_voice_button_surface in 374.17µs
[  5.458s] [Workarounds] starting patch_item_socketing_tooltips
[  5.458s] [Workarounds] finished patch_item_socketing_tooltips in 99.28µs
[  5.458s] [Workarounds] starting patch_character_select_selected_name
[  5.458s] [Workarounds] finished patch_character_select_selected_name in 49.55µs
[  5.458s] [Workarounds] starting patch_character_create_defaults
[  5.458s] [Workarounds] finished patch_character_create_defaults in 350.20µs
[  5.458s] [Workarounds] starting patch_character_frame_title_refresh
[  5.470s] [Workarounds] finished patch_character_frame_title_refresh in 11.34ms
[  5.470s] [Workarounds] starting patch_vignette_pin_template
[  5.470s] [Workarounds] finished patch_vignette_pin_template in 189.85µs
[  5.470s] [Workarounds] starting patch_fog_of_war_pin_mixin
[  5.470s] [Workarounds] finished patch_fog_of_war_pin_mixin in 401.33µs
[  5.470s] [Workarounds] starting patch_map_exploration_pin_mixin
[  5.471s] [Workarounds] finished patch_map_exploration_pin_mixin in 525.96µs
[  5.471s] [Workarounds] starting patch_map_canvas_data_provider_attachment
[  5.471s] [Workarounds] finished patch_map_canvas_data_provider_attachment in 95.42µs
[  5.471s] [Workarounds] starting ensure_adventure_map_frame_surface
[  5.471s] [Workarounds] finished ensure_adventure_map_frame_surface in 245.06µs
[  5.471s] [Workarounds] starting patch_action_bar_button_event_fanout
[  5.471s] [Workarounds] finished patch_action_bar_button_event_fanout in 154.10µs
[  5.471s] [Workarounds] starting patch_paging_controls_page_text
[  5.471s] [Workarounds] finished patch_paging_controls_page_text in 54.98µs
[  5.471s] [Workarounds] starting patch_talent_edge_frame_level_sync
[  5.471s] [Workarounds] finished patch_talent_edge_frame_level_sync in 40.73µs
[  5.471s] [Workarounds] starting patch_catalog_shop_product_card_defaults
[  5.472s] [Workarounds] finished patch_catalog_shop_product_card_defaults in 90.48µs
[  5.472s] [Workarounds] starting patch_game_time_defaults
[  5.472s] [Workarounds] finished patch_game_time_defaults in 9.31µs
[  5.472s] [Workarounds] starting patch_tooltip_nineslice_surface
[  5.472s] [Workarounds] finished patch_tooltip_nineslice_surface in 69.16µs
[  5.472s] [Workarounds] starting patch_container_frame_token_tracker
[  5.472s] [Workarounds] finished patch_container_frame_token_tracker in 67.78µs
[  5.472s] [Workarounds] starting patch_achievement_display_set_achievements
[  5.472s] [Workarounds] finished patch_achievement_display_set_achievements in 13.95µs
[  5.472s] [Workarounds] starting patch_housing_dashboard_preload
[  5.472s] [Workarounds] finished patch_housing_dashboard_preload in 103.06µs
[  5.472s] [Workarounds] starting patch_lfg_lock_list
[  5.472s] [Workarounds] finished patch_lfg_lock_list in 8.85µs
[  5.472s] [Workarounds] starting patch_auction_house_browse_results_event
[  5.472s] [Workarounds] finished patch_auction_house_browse_results_event in 113.26µs
[  5.472s] [Workarounds] starting patch_auction_house_search_context_aliases
[  5.472s] [Workarounds] finished patch_auction_house_search_context_aliases in 49.74µs
[  5.472s] [Workarounds] starting patch_auth_challenge_frame_parent
[  5.472s] [Workarounds] finished patch_auth_challenge_frame_parent in 73.28µs
[  5.472s] [Workarounds] starting patch_settings_surface_defaults
[  5.473s] [Workarounds] finished patch_settings_surface_defaults in 908.84µs
[  5.473s] [Workarounds] starting patch_settings_canvas_layout_visibility
[  5.473s] [Workarounds] finished patch_settings_canvas_layout_visibility in 167.75µs
[  5.473s] [Startup] apply_post_load_workarounds: workarounds complete
[  5.577s] [Startup] apply_post_load_workarounds: globals restored
[  5.578s] [Startup] apply_post_load_workarounds: patch 12.1 compatibility restored
[  5.578s] [Startup] apply_post_load_workarounds: seterrorhandler restored
[  5.578s] [Startup] post-load workarounds complete in 357.01ms
[  5.578s] [Startup] restarting GC after bootstrap
[  5.785s] [Startup] GC restart complete in 207.10ms
Lua error: [exec-lua] error: Lua error: SetAlpha must reject out-of-range numeric alpha: -0.25
stack traceback:
	(string):1: in main chunk
Lua error: Lua Error: [exec-lua] error: Lua error: SetAlpha must reject out-of-range numeric alpha: -0.25
stack traceback:
	(string):1: in main chunk
[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:59: in function 'GetErrorData'
[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:72: in function <[Interface/AddOns/Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua]:71>

=== Final observed Lua error summary ===
Status: FAILED
Lua errors: 2 unique, 2 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 2 occurrence(s)
```

#### result.json (complete, 613 bytes)

```text
{
  "source": "a5397a2d60ebdd2dc813309671bfb26534e3621c",
  "profile": "ptr",
  "filter": "shared ordinary-alpha fixture invalid",
  "expected_cases": 0,
  "launcher_exit": 1,
  "summaries": [],
  "stdout_bytes": 615,
  "stderr_bytes": 15233,
  "receipt_stat_reused": {
    "st_dev": 66306,
    "st_ino": 149051649,
    "st_size": 415341528,
    "st_mtime_ns": 1791944301479256281
  },
  "unit": "<UNIT_03>",
  "scope": "Pre-producer shared complete Lua fixture on frozen ptr normal binary: ordinary numeric alpha invalid; not newlycompiledRusttest/native/render proof"
}
```

## Save receipt contract

report.md and proof.json saved with 0600 permissions, fsync on each file and output directory, and full-byte readback comparison. proof.json preserves every artifact and exact coverage classification; no diagnostic excerpts replace original contents.
