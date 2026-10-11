# Independent saved-artifact audit

Date: 2026-10-11

**PASS — bounded evidence only:** epoch `20261011T052700Z` records two original default-Retail cases passing through the ordinary full-UI prefork runner. This is not native-client, full-suite, all-marker, all-fixture-group, or cross-profile acceptance. Broader active goal remains open; any log fix is separate. Mists/vendor-blocked lanes were not changed.

## Exact results and complete output

| Case | Recorded exit | Named result | Counts |
|---|---:|---|---|
| `prefork_full_ui_nested::fixture::preloaded_parent_has_normal_game_startup` | 0 | `test prefork_full_ui_nested::fixture::preloaded_parent_has_normal_game_startup ... ok` | 1 passed; 0 failed; 1 total |
| `test_keybindings_panels_detail::world_map_floor_dropdown_hidden_without_subzone_or_map_group` | 0 | `test test_keybindings_panels_detail::world_map_floor_dropdown_hidden_without_subzone_or_map_group ... ok` | 1 passed; 0 failed; 1 total |

Each stdout additionally contains three `running 0 tests` / `0 passed; 0 failed; 0 total` groups, not three more executed cases. Total named controls: **2 passed, 0 failed**. `results.json` and `outcome.json` agree with stdout and submission argv.

Read all saved output: `0.stdout` 334 bytes/13 lines; `0.stderr` 8,768 bytes/138 lines; `1.stdout` 353 bytes/13 lines; `1.stderr` 8,875 bytes/140 lines. No truncation in these reads. Both stderr streams show real Game startup through login, PLAYER_ENTERING_WORLD, post-login events and FIRST_FRAME_RENDERED. Startup ends at 3.190s and 2.960s respectively; manual case then logs `M → TOGGLEWORLDMAP → ToggleWorldMap()` and `M executed in 69.7ms`.

**Actual visible startup diagnostics:** no error or warning records in either complete saved stdout/stderr. Existing `[Workarounds]` and `[EditMode]` messages, `Skipping EDIT_MODE_LAYOUTS_UPDATED`, and `patch 12.1 compatibility restored` are present; they are not zero-output startup. No compiler logs exist in this epoch, so compiler warning cleanliness is not claimed. Successful prerequisite-conformance output is captured and discarded by runner source; its diagnostic contents/counts cannot be audited from these files.

## Ordinary real-prefork wiring

- `worker.py` equals the code in `request.jsonl`. It rejects pre-existing `PREFORK_CONFORMANCE_SUITE`, `PREFORK_CONFORMANCE_DRIVER`, `PREFORK_CONFORMANCE_TREE_CHILD`, and `PREFORK_EXACT_FIXTURE_GROUP`; invokes the sealed `new-prefork` with exact original selectors, `--nocapture`, `--test-threads=1`, timeout 90; sets `WOW_SIM_CASC=0`. It does not build, rehash, change fixtures, or select the conformance shortcut. This is source evidence, not a separately saved complete environment snapshot.
- `tests/prefork_full_ui.rs:301-354` routes ordinary main through workload lock to full-UI setup; combines manual and generated cases at lines 245-252. Selected setup first runs prerequisite conformance, then `preload_full_game_ui`. Thus **no conformance shortcut** does not mean **no prerequisite conformance subprocess**. Three unmatched chat/cast-bar/spellbook groups explain zero-case summaries.
- `tests/common/prefork_full_ui_preload.rs:56-88` creates real `WowLuaEnv`, 1024x768 Game screen, discovers/loads synced Blizzard startup addons including bootstrap-only nodes, applies existing workarounds, restarts GC, fires startup events, releases parent bytecode memory. Lines 90-183 check for newly recorded Lua errors at load/event/workaround boundaries. Passing setup means those checked boundaries reported no new recorded Lua errors, not proof of every possible error channel.
- `tests/common/prefork.rs:374-417` uses `libc::fork` after setup and single-thread check. Lines 447-477 run child setup and the selected original function on inherited state. Lines 625-661 classify pass only when child exited 0 with CHILD_PASS status. Source supports real prefork; saved output does not independently expose child PIDs/syscall traces.
- Original nested marker: `tests/prefork_full_ui_nested.rs:1-2` path-declares fixture; `tests/prefork_full_ui_nested/fixture.rs:3-17` retains UIParent/WorldFrame existence and exact 1024x768 assertions. `tests/common/mod.rs:115-123` preserves body as `run`; `crates/test-harness/build.rs:243-257` emits named Case pointing to that run function. Manual registration `tests/prefork_full_ui.rs:188-190` points directly to original function in `tests/test_keybindings_panels_detail.rs:19-82`, retaining subzone clearing, M dispatch, shown world-map requirement, dropdown lookup/hidden requirement, and empty captured-error assertion.

## Source and binary provenance

Read-only `git show 256e54ef1:<path>` content comparison returned exit 0 and exact equality for all ten inspected source files: both nested marker files; manual world-map file; preload constructor; runner; prefork worker; process helper; common marker wrapper; test-harness library; registry generator. No constructors/assertions/source files were edited.

Accepted prior seal; **no binary rehash or execution** in this audit. Prior `dev-harness-green-current/20261011T051913Z/retail/artifacts.json` identifies the exact submitted sealed `new-prefork`, 509,485,432 bytes, stored SHA-256 `435706882388fe25a24afbf2d28d464989f528efe8caa72a10fa3b131c1febea`, default Retail/prefork features, opt-level 1, debug assertions and overflow checks enabled. That receipt has no source-revision field; association with `256e54ef1` is the supplied prior-seal premise plus submissions, not newly reconstructed build provenance.

Receipt metadata caveat: per-case `artifact_receipt` is `../dev-harness-green-current/...`. Resolved relative to the epoch submission file, it does not point to the actual sibling verification root (would need `../../...`). The absolute executable argv and explicitly supplied prior receipt agree. No metadata repair performed.

Controller submission records exit 0 for enqueue, not test completion; controller stdout records a completed worker and the matching epoch path. Controller stderr is empty (0 bytes). Admission records first-attempt acceptance, 34.83588790893555 GiB available, load1 2.16, no relevant Cargo processes.

## Audit boundaries

No rerun, build, binary hashing, operational mutation, delegation, Bash, rg, native/full-profile acceptance, or Mists/vendor repair. Only report files written. Read-only Git source retrieval used. Tooling deviation: one file-find tool attempt failed immediately with `fd is not available on PATH`; no fd search ran. Subsequent location discovery used Python filesystem reads.
