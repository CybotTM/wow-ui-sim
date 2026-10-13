# Independent saved Mists startup and anchor classification

PASS bounded saved executions at source association `480b9f934e017ff87e0c79c2ba28b47b33cded19`. Historical normal self-anchor FAIL remains FAIL. Followed verify/SKILL.md as assigned verifier; saved-evidence inspection only. No builds, runtime reruns, operational actions, rehash, source/vendor/result edits, commits or delegation.

## Coverage matrix

| Saved boundary | Observed outcome | Proof level / exclusions |
|---|---|---|
| Normal Mists CLI cached startup | PASS exit0; stdout [] | Complete 220-line stderr audited, 142 Blizzard addons loaded; zero observed Lua errors. Not native/render/full-parent/installed-CASC acceptance. |
| methods_anchor:: | PASS 33/33; 0 failed/ignored/measured; 8215 filtered | Exact source-module tests match all 33 executed names. Shared runtime API guards, not missing-key XML acceptance. |
| template_child_anchor_override:: | PASS 3/3; 0 failed/ignored/measured; 8245 filtered | Generated Frame template inheritance/inline anchors and scale/layout; not Texture missing-key/native proof. |
| Historical normal startup, 2026-10-10 | FAIL exit1; 2 unique/2 occurrences | Original primary self-anchor plus ScriptErrors reporting wrapper retained. Current pass does not rewrite historical failure. |

## Full startup diagnostic inventory

Read complete stdout/stderr/argv/admission/launcher/result for all three runtimes; verbatim text and numbered inventories retained in proof.json. Normal stdout 3 bytes/1 line; stderr 14,711 bytes/220 lines. Diagnostics comprise 1 [startup], 56 [Startup], 124 [Workarounds], 24 [EditMode], 15 unbracketed lines. No emitted warnings, Lua errors, panic, timeout, failed-load or resource-kill diagnostics. Words in `seterrorhandler restored` and the zero-error summary are not failure emissions.

Normal unbracketed inventory: resource CPU limit; Sound disabled; SavedVariables disabled; read-only EditMode cache source; 142 addons announced and load completion; three frame-breakdown/setup/finalize lines; third-party addon loading disabled; final summary heading plus CLEAN status, 0 unique/0 occurrences, 0 attributed owners, 0 unattributed occurrences. Cached load: 1094/1094 bytecode hits, 0 lookup misses/replay failures/stores/store failures; 2490 frames. Both post-load workaround passes and Mists post_load complete; globals and error handler restored; GC restarted. These diagnostics describe existing simulator workarounds and font fallback, not pristine vendor/native execution. Zero runtime warnings does not imply zero compile warnings: saved report-integration.json records six manifest deprecations and one unit_roles dead-code warning; compile rerun/audit outside this task.

Startup argv explicitly disables CASC and sound via WOW_SIM_CASC=0 / WOW_SIM_NO_SOUND=1; --no-addons means third-party addons, NOT the 142 cached Blizzard addons; --no-saved-vars disables SavedVariables, NOT all WTF reads. Stderr confirms `EditMode cache source (read-only): <ACCOUNT> @ <MISTS_REALM>/<PLAYER>`. Root/source/target read-only binds, temporary /tmp, network unshared. Admission allowed=true, MemoryMax4G/MemoryHigh3G/CPUQuota200%, timeout86s plus 2s kill-after. Launcher success, exited0, service7.038s, CPU6.443s, peak2.1G/swap0. No clean-room cache independence claim.

methods stderr: 21,202 bytes/363 lines, exclusively [Startup], exactly 33 WowLuaEnv begin/complete sequences; zero warning/error/panic/timeout emissions. Launcher exited0, service8.221s, CPU7.703s, peak115.9M/swap0; harness8.14s. Template-child stderr: 1,920 bytes/33 lines, exclusively [Startup], 3 begin/complete sequences; zero warning/error/panic/timeout emissions. Launcher exited0, service1.089s, CPU890ms, peak130.3M/swap0; harness0.99s. Both admissions allowed; same bounded sandbox/limits; integration tests use basic environment setup, not a 142-addon normal startup.

## Additive count corrections

Original methods result expected_cases=15 is an incorrect predicted source-count premise. Saved module has 33 #[test] functions plus one non-test env helper; all 33 names exactly match saved executed passing cases. Correction is 15 predicted -> 33 observed, NOT 18 newly added tests, NOT a runtime failure. Template-child result expected_cases=2 likewise predicted incorrectly: 3 #[test] functions plus setup helper; all three executed. Correction 2 predicted -> 3 observed includes scale/layout guard. Both historical result.json files remain unchanged. No rerun required or performed.

## Exact methods_anchor behavioral guards

All rows PASS at the saved Mists boundary; source is epoch source/tests/methods_anchor.rs.

| Test / source line | Concrete assertion |
|---|---|
| `test_set_point_basic`:15 | one point after CENTER/UIParent (10,20) |
| `test_set_point_reports_plumber_topelft_typo_compatibility`:30 | TOPELFT canonicalizes point and relative point to TOPLEFT; console compatibility notice |
| `test_live_addon_script_handlers_are_accepted`:53 | EditBox OnCursorChanged, tooltip default-anchor/frame-stack/add-money, Model OnModelLoaded registration succeeds; HasScript checked for cursor/default-anchor/model |
| `test_get_point_returns_values`:77 | TOPLEFT/BOTTOMRIGHT and offsets (5,-10); relative identity is not asserted |
| `test_set_point_default_relative_point`:94 | CENTER defaults relative point CENTER |
| `test_set_point_accepts_nil_relative_to`:106 | nil relative target becomes UIParent; TOPLEFT and zero offsets |
| `test_set_point_multiple`:123 | two distinct points retained |
| `test_set_point_replaces_same_point`:139 | replacement CENTER x=99 |
| `test_clear_all_points`:163 | two points cleared to zero |
| `test_set_all_points`:184 | two anchors created |
| `test_set_all_points_offsets_zero`:199 | first anchor offsets both zero |
| `test_adjust_points_offset`:220 | (10,20)+(5,-3)=(15,17) |
| `test_adjust_points_offset_multiple_anchors`:238 | second anchor receives (10,10) |
| `test_get_point_by_name`:261 | BOTTOMRIGHT lookup point/relative point and (-5,-10) |
| `test_get_point_by_name_not_found`:281 | absent TOPLEFT returns nil |
| `test_set_point_self_reference_raises_error`:303 | explicit full-form self-target pcall fails |
| `test_set_all_points_explicit_parent_returns_parent`:323 | no-arg SetAllPoints returns explicit parent in both point targets |
| `test_set_all_points_explicit_nil_returns_nil`:350 | explicit nil SetAllPoints returns nil for both targets |
| `test_set_all_points_implicit_parent_returns_parent`:380 | no-arg SetAllPoints TOPLEFT/UIParent/TOPLEFT/(0,0) |
| `test_get_num_points_default_zero`:409 | new frame has zero points |
| `test_set_point_self_cycle_raises_error`:427 | short-form self-target pcall fails |
| `test_set_point_indirect_cycle_raises_error`:442 | two-frame dependency cycle pcall fails |
| `test_set_all_points_self_cycle_raises_error`:462 | SetAllPoints self-target pcall fails |
| `test_cycle_error_dag0_self_anchor_message`:489 | exact SetPoint self-anchor error message |
| `test_cycle_error_dag1_direct_cycle_message`:515 | exact two-frame dependent error; no ancestor section |
| `test_cycle_error_dag2_one_ancestor_message`:545 | exact dependent error with one ancestor |
| `test_cycle_error_dag3_two_ancestors_message`:575 | exact dependent error with ordered two ancestors |
| `test_cycle_error_all0_set_all_points_self`:608 | exact self-anchor error naming SetAllPoints |
| `test_cycle_error_all3_set_all_points_chain`:627 | exact SetAllPoints dependent-chain error with ordered ancestors |
| `test_set_point_explicit_nil_relative_to_resolves_to_parent`:661 | five-arg nil/nil form resolves NilRelParent, not screen |
| `test_set_point_relative_frame_with_offsets`:697 | short frame+numeric-offset form targets SPRelAnchor, not parent |
| `test_set_point_relative_frame_with_offsets_values`:719 | short form preserves offsets (6,-4) |
| `test_set_point_full_form_still_works`:740 | full form targets SPFullAnchor, relative BOTTOMLEFT (offsets not asserted) |

These pcall rejection/message tests preserve runtime self-anchor/cycle prohibitions; they do not prove transactional retention of pre-existing points after rejection, XML self-key creation, all unknown anchor forms or native error parity. Direct Lua preservation has separate focused evidence; it is not inferred here.

## Template-child behavioral guards

- World-map-style synthetic Frame child: inherited TOPLEFT/BOTTOMRIGHT then inline TOPLEFT/BOTTOMLEFT/RIGHT; observable left0, top delta0, width800, height600, at least3 points. Does not assert an exact point count or native WorldMap startup.
- Hero-nodes-style synthetic Frame child: inherited TOP survives inline LEFT/BOTTOMRIGHT; all three point names asserted.
- Inherited XML scale0.85 and effective scale0.85; local height equals 362/0.85-90-60 within0.01. No image/render comparison.

## Historical normal FAIL comparison

Read complete original results.json, startup.stdout (859 bytes/10 lines), startup.stderr (15,552 bytes/230 lines) from mists-cast-color-green/20261010T082052Z/execution. Historical normal command also --no-addons --no-saved-vars lua-errors; sound disabled, casc=false, same read-only EditMode cache identity, 142 addons and 1094/1094 bytecode hits. Original exit1 and FAILED summary remain authoritative for that execution: 2 unique/2 occurrences, owner Blizzard_UIPanels_Game2, unattributed0. Primary is failed frame __Blizzard_UIPanels_Game_20279, layer children SetPoint Cannot anchor to itself; second is ScriptErrors wrapper, not independently proven second cause. Current saved stderr has neither emission and reports CLEAN. Historical2483 versus current2490 frames is observed, not proof of complete parent/subtree coverage. Different revisions/epochs/external cache states prevent attributing the entire startup difference solely from this comparison.

## Source480 / receipt boundary

Source revision, source mount, seals, results, normal Cargo feature metadata and existing receipts associate these copied artifacts with Mists/source480; features casc/client-mists/gui/prefork-full-ui/rodio/sound are build metadata, not enabled runtime CASC/sound proof. Existing receipts record PASS and exactly one prior hash read each. This audit performed ZERO executable/library/source/archive hashes. Current stat(dev,inode,size,mtime_ns) equals each receipt/seal and each runtime receipt_stat_reused; integration (66306,147863968,439378344,1791930746926396844), normal (66306,147890032,399454712,1791930710846449484). Metadata/stat continuity is not independently authenticated source provenance, external-cache equality or independent process attestation.

Only bounded cached normal Mists startup and listed shared API/generated Frame controls pass. Native parity, rendering, installed CASC/sound, third-party/SavedVariables loading, other profiles, missing-key Texture behavior and full-parent acceptance remain unproven/outside scope.
