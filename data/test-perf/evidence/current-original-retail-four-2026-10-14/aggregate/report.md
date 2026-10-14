# Current original Retail four-case aggregate artifact audit

**PASS — 4 distinct current original cases passed; 0 failed; 4 total.** Existing-artifact audit only. No workloads, builds, hashes, service actions, source edits, or delegation. Only requested report/proof files written.

Epoch: `widget-discrete-progress-current/draw-layer-retail-original-prefork-20261014T041742609972Z`.
Compiled revision recorded: `1b707b28b16eb291e36180b2a12ac2f0411264af`.
Snapshot inspected: `widget-discrete-progress-current/draw-layer-retail-green-20261014T040918198388Z/source`, revision `6bdb02b06`.
`source-scope.json` records a docs-only delta: texture draw-layer spec, widget-system wiki, wiki index/log. Runtime/tests unchanged is accepted metadata, not fresh source-byte authentication.

## Current evidence matrix
| Group | Distinct cases | Evidence | Result |
|---|---|---|---|
| cast-bar | `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix` | Accepted private `current-original-cast-bar-current/report.md`; six raw files NOT reopened | 1/1 |
| chat | `chat_frame::test_chat_editbox_click_type_and_submit`; `chat_frame::test_chat_editbox_text_color_after_activation` | All six files fully read in `runtime-original-chat-20261014T050226097478Z` | 2/2 |
| spellbook | `blizzard_player_spells_loads::mainline_spellbook_keybind_opens_and_closes_without_runtime_errors` | All six files fully read in `runtime-original-spellbook-20261014T050251771531Z` | 1/1 |

New bundles' six files: admission.json, argv.json, launcher.txt, result.json, stdout, stderr. Count evidence comes from custom stdout, not the generic helper. Both result.json files legitimately contain `summaries=[]`.

Chat full stdout:
```
running 2 tests
test chat_frame::test_chat_editbox_click_type_and_submit ... ok
test chat_frame::test_chat_editbox_text_color_after_activation ... ok

test result: ok. 2 passed; 0 failed; 2 total
```
Spellbook full stdout:
```
running 1 tests
test blizzard_player_spells_loads::mainline_spellbook_keybind_opens_and_closes_without_runtime_errors ... ok

test result: ok. 1 passed; 0 failed; 1 total
```
Cast-bar accepted report quotes its custom `1 passed; 0 failed; 1 total` and original named case. No historical result supplies any member of this current aggregate. Historical original four-case checkpoint remains historical, not a rerun claim, not extra cases, and not refreshed by old data.

## Actual original constructors/assertions and exact wiring
All source references below are in the reused 6b snapshot, not a replacement fixture.

- `tests/prefork_full_ui.rs:226-246` contains exact 2/1/1 registries using original `::run` bodies. `:301-320` branches on exact-group selection before the default full-registry path. `:370-388` maps chat to `chat_frame::setup_env`, cast-bar to `spell_casting::env_with_full_blizzard_ui`, spellbook to `blizzard_player_spells_loads::load_runtime_game_ui`. New argv explicitly selects chat/spellbook. `:455-486` uses `prefork::run_with_setup`, normal conformance invocation, parent bytecode-cache bypass, original constructor and cache-memory release. `:495-518` captures conformance output and suppresses it on success: wiring and aggregate success are NOT case-level conformance diagnostic proof.
- `tests/chat_frame.rs:71-115`: original 1024x768 environment, profile Blizzard path, discovery/load, post-load workarounds, startup events. Load errors are logged as `[load name] FAILED`, then continued (not asserted); event results are discarded, including common ADDON_LOADED/PLAYER_ENTERING_WORLD helpers. `:278-395`: hooks C_ChatInfo.SendChatMessage while calling original, verifies initial lack of focus, actual click/focus, character-key input `hello world`, ENTER, exactly one SAY message and empty editbox. `:442-463`: actual click/focus, RGB white and alpha 1 within 0.01. These are input/message/color/alpha assertions, not rendered pixel or real-network delivery proof.
- `tests/spell_casting.rs:138-164,216-232`: default cached Blizzard constructor, discovery/load, workarounds and original startup events; load/event results discarded. `:447-567`: default unlocked value 0, unattached cast bar under BottomManagedFrameContainer; detached player-anchor call succeeds without attaching; setting LockToPlayerFrame=1 and updating setting map makes cast anchoring succeed, attach and parent to PlayerFrame. Retail does not execute Mists-only fade/managed-parent assertions; not casting-duration or pixel proof.
- `tests/blizzard_player_spells_loads.rs:12-15,133-170`: original default cached Game-screen constructor, intrinsic templates, screen-aware startup loading, per-addon ADDON_LOADED, cleanup restoration, globals/name synchronization, workarounds and production startup. Unlike cast-bar/chat, explicit startup load and per-addon event failures here panic; downstream production startup is not a comprehensive asserted error inventory. `:210-249`: clears prior lua_errors, presses S, runs ten extra ticks, asserts spellbook exists/shown; presses S and ten ticks again, asserts hidden and interaction-window lua_errors empty. Clearing errors excludes preexisting startup errors. `src/startup.rs:107-122,162-185` logs timer/OnUpdate failures; a passing case alone cannot replace full diagnostics.

## Full diagnostics and isolation
Chat stdout/stderr: 196/6,958 bytes. Spellbook: 171/9,048 bytes. Both stderr files fully read through their final lines, not reduced to exit codes. Chat records initialization, workarounds/global restoration, then EditMode anchors through reapply_player_frame_anchor. Spellbook records those stages plus login/world/post-login, FIRST_FRAME_RENDERED, and both S dispatches (795.1ms, 104.5ms). No explicit failure/error diagnostic observed in either supplied full stderr or stdout. This is NOT comprehensive startup-error freedom: continuing/discarded results and spellbook error clearing constrain evidence.

Both admission records allowed=true and Retail. Recorded argv: MemoryMax=4GiB, MemoryHigh=3GiB, CPUQuota=200% (two CPU equivalents), TERM timeout 86 seconds with two-second kill grace, control-group kill mode; bwrap root/snapshot/target read-only, separate PID and network namespaces, temporary /tmp and device/proc mounts, CASC=0, sound=0. Network namespace isolation is not native-client parity. Chat launcher: exit 0/SUCCESS, runtime 25.586s, CPU 26.821s, peak 1.4G, swap 0B. Spellbook: exit 0/SUCCESS, runtime 31.976s, CPU 31.154s, peak 1G, swap 9.8M. Cast-bar limits/diagnostics reused only from accepted report. No new services launched by audit.

Accepted executable-copy receipt reused. Fresh stat only matches accepted device/inode/size/mtime_ns: 66306 / 149562126 / 514806728 / 1791951505845443186; both new result.json stat records agree. No binary/source/library/archive bytes reopened for authentication; no fresh hash. Stat agreement is not a new byte-integrity proof.

Cache-before/after pair belongs ONLY to accepted cast-bar audit. That report accepts unchanged observed manifest/cache/marker stat fields and marker text, not full cache bytes or all descendants. No new chat/spellbook cache pair supplied; do not impute per-group equality from cast-bar observations or read-only bindings.

## Dates and boundaries
Chat bundle label: October 14, 2026 05:02:26.097478 UTC = October 14, 2026 00:02:26.097478 CDT. Spellbook: October 14, 2026 05:02:51.771531 UTC = October 14, 2026 00:02:51.771531 CDT. Earlier accepted cast-bar: October 14, 2026 04:26:21.001865 UTC = October 13, 2026 23:26:21.001865 CDT. These are artifact-label conversions, not independent clock validation or a claim of “today.”

Proven: four named current original Retail fixture cases at recorded revision, bounded by their concrete assertions and diagnostics. Unproven/not audited: full registry, full preload, complete conformance case results, native parity, all profiles, comprehensive Lua-error inventory, cache-byte equivalence, general parent correctness and source/archive/library authentication. No excluded workloads performed.
