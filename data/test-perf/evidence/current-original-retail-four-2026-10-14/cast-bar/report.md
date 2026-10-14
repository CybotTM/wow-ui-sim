# Current original cast-bar artifact audit

Verdict: PASS for one current retail original case only. Existing-artifact audit; no workload, build, hash, service action, source edit, or delegation performed.

## Evidence and scope
Epoch: widget-discrete-progress-current/draw-layer-retail-original-prefork-20261014T041742609972Z.
Runtime bundle: runtime-original-cast-bar-20261014T042621001865Z.
All six artifacts read completely: admission.json, argv.json, launcher.txt, result.json, stdout, stderr. Also read cache-before.json, cache-after.json, source-scope.json and accepted receipt-prefork_full_ui.json metadata. No accepted source/archive/library/binary copy authentication was repeated. Snapshot source was inspected only for requested fixture wiring, constructor, startup sequence and assertion body.

Actual custom stdout:
```
running 1 tests
test spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix ... ok

test result: ok. 1 passed; 0 failed; 1 total
```
Launcher exit 0; service main process status 0/SUCCESS; service runtime 11.837s; CPU time 13.461s; memory peak 885.7M, swap 0B. result.json summaries=[] does not mean missing execution: generic helper regex shape does not capture this custom summary; direct stdout supplies exact case/count evidence.

## Original source wiring and behavior
Inspected reused snapshot 6bdb02b06 under draw-layer-retail-green-20261014T040918198388Z/source. source-scope metadata records compiled revision 1b707b28b16eb291e36180b2a12ac2f0411264af and docs-only delta (texture-draw-layer spec and widget wiki/index/log); runtime code/tests unchanged per that metadata, not independently byte-authenticated.

- tests/prefork_full_ui.rs:238-241: CAST_BAR_FIXTURE_CASES contains exactly the named original spell_casting case and its ::run function.
- :301-320: exact-group environment selects run_exact_fixture_group before default full-registry path.
- :370-381: cast-bar selects CAST_BAR_FIXTURE_CASES and spell_casting::env_with_full_blizzard_ui, not a replacement constructor.
- :455-486: run_exact_fixture uses prefork::run_with_setup; normal branch invokes conformance subprocess, parent bytecode-cache bypass, original create_env, and parent cache-memory release. This wiring is not independent proof of every conformance case or general parent correctness.
- tests/spell_casting.rs:138-164: original constructor resolves default Blizzard cache, creates WowLuaEnv, sets 1024x768, supplies that addon base path, discovers/loads Blizzard addons, applies post-load workarounds, then fires original startup events. load_addon results are discarded: constructor completion does not prove every addon loaded successfully.
- :216-232: startup sequence includes ADDON_LOADED for WoWUISim, VARIABLES_LOADED, PLAYER_LOGIN, initial PLAYER_ENTERING_WORLD, EDIT_MODE_LAYOUTS_UPDATED, UPDATE_BINDINGS, DISPLAY_SIZE_CHANGED, UI_SCALE_CHANGED. Several event results are discarded.
- :447-567: original assertion body checks default lock value 0, unattached startup cast bar and retail BottomManagedFrameContainer parent; detached PlayerFrame:ApplySystemAnchor succeeds without attaching; setting LockToPlayerFrame=1 and UpdateSettingMap(true), then cast ApplySystemAnchor succeeds, attaches and yields PlayerFrame parent. Mists-only fade/managed-parent assertions are not covered by retail. This is anchor/lock behavior, not casting-duration or pixel-rendering coverage.

## Isolation, diagnostics, cache
Recorded argv uses root read-only bwrap bind, read-only snapshot/target binds, --unshare-pid and --unshare-net, temporary /tmp, device/proc mounts; 4GiB MemoryMax, 3GiB MemoryHigh, CPUQuota=200% (two CPU equivalents), 86-second TERM timeout with two-second kill grace. WOW_SIM_CASC=0 and WOW_SIM_NO_SOUND=1. Admission allowed=true and matching worker limits. No WTF/vendor modifications evidenced; read-only runtime bindings prevent ordinary persistent writes there. Audit itself modified no WTF/vendor files. Not a filesystem forensic proof.

Full 6,952-byte stderr was read, not inferred from CLI exit 0. It records WowLuaEnv initialization, post-load workaround stages, restored globals/12.1 compatibility/error handler, and subsequent EditMode layout/anchor work through reapply_player_frame_anchor. No explicit error diagnostic appears in this supplied stderr. Neither quiet stderr nor exit 0 establishes complete startup-error freedom: original loader/event paths discard results, and no separate comprehensive Lua-error inventory was supplied.

Cache-before/after show unchanged observed manifest/cache stat fields (device, inode, size, mtime_ns), both marker stat fields and complete marker TEXT; cache-after additionally records matches_before_observed_fields=true. Provenance text records retail wow version 12.1.0.69933, source=casc-local-or-cdn, fallback=none; completion text is ok. Embedded manifest digest is marker text, not a new hash check. These observations do not prove cache byte identity, every descendant unchanged, full cache equivalence, or source authenticity.

Accepted receipt metadata reports PASS, one prior hash read, seal digest/stat agreement for prefork executable copy; runtime result reuses the same four stat fields. Receipt scope explicitly accepts executable-copy integrity only, not source/archive/library/cache authentication, load/runtime/fixture conformance. No binary/library/archive reread or hash performed in this audit.

## Coverage boundary
Current original case: 1/1 passed, 0 failed. Historical original four-case evidence is not refreshed, combined, or promoted by this bundle. Full preload/full registry, complete conformance, full cache byte equivalence, native-client parity, all-profile behavior, and general parent correctness remain unproven here. Existing conformance call wiring and launcher success are not substitutes for their case-level diagnostic evidence.

Artifact timestamps are copied literally (October 14, 2026 UTC); they are evidence labels, not a claim about the audit's local date or independent clock validation.
