# PTR widget real-consumer saved-evidence verification

**PASS — bounded saved evidence only.** Verified 2026-10-13. No runtime reruns, builds, executable/library hashing, delegation, service operations, or tracked-file edits.

Epoch: `<PRIVATE_VERIFICATION_ROOT>/ui-widget-bars-hud-green-current/20261013T110748176968Z`.

[Full proof](proof.json) retains every runtime-green file verbatim, launcher script, stat command/output, source diffs, admission recomputation and exact case accounting. Its saved argv/script are evidence, not commands executed by this verification.

## Coverage and authoritative exits

| Unique parent case | Saved runtime proof | Result |
|---|---|---|
| `ui_widget_manager_bars_consumer::actual_cached_status_and_capture_bar_lifecycle` | Final parent summary: 1 passed / 0 failed / 0 ignored / 10143 filtered; actual `systemd-run --wait` return 0; launcher reports `code=exited, status=0/SUCCESS` | PASS |
| `ui_widget_manager_hud_containers::cached_standard_hud_default_zero_leaves_no_active_widget_set` | HUD final parent summary: 2 passed / 0 failed / 0 ignored / 10142 filtered; actual wait return 0 and `status=0/SUCCESS` | PASS |
| `ui_widget_manager_hud_containers::cached_standard_hud_onload_consumes_preseeded_selection_and_records` | Same HUD parent execution, distinct registered test | PASS |

**Total: bar1 + HUD2 = 3 unique parent cases, all PASS.** Exclude one bar child summary and two HUD child summaries. Do not sum the raw `counts` arrays: that would falsely report six passes. `tests/common/timeout_reexec.rs` explains exact-test child execution and successful output replay; final outer summaries and distinct names establish parent accounting.

Both `.terminal.txt` files contain inactive/dead defaults, empty `ExecMainExitTimestamp`, and unset peak/CPU. These post-GC defaults are **not exit proof**. Authoritative evidence is saved wait return (`launcher_exit: 0`) plus actual launcher termination output. Bar service runtime 4.667s, peak 1.7G; HUD 8.387s, peak 1.2G. No current systemd query was performed.

Full stdout/stderr, argv, launcher, individual result and aggregate result were read. Byte lengths agree: bar stdout416/stderr723; HUD stdout791/stderr1459. Saved aggregate results match individual summaries. Stderr contains startup timing and exactly one bar/two HUD diagnostics:

```
[ui-widget-cache] profile=ptr path=<HOME>/.cache/wow-ui-sim/blizzard-ui/ptr/AddOns
```

## Receipt stat — stat only

`stat` exit0 on the sealed integration artifact observed:

| Field | Current stat / receipt / seal |
|---|---|
| st_dev | 66306 |
| st_ino | 140810657 |
| st_size | 470113608 |
| st_mtime_ns | 1791889729583216919 |

Timestamp output: `2026-10-13 06:08:49.583216919 -0500`. All four fields match `receipt-integration.json.stat_after` and `seal-integration.json.stat`. Receipt records authentication PASS and digest match; **digest authentication was not independently repeated**. Stat identity is not content-hash proof. Executable contents were never read or hashed.

## Admissions and launcher boundaries

All five runtime admission records recompute correctly from saved raw memory/pressure inputs, using launcher-script policy: half clean file+reclaimable slab, projected memory below both parent limits, host available above 4GiB, `some avg10 < 5`.

| Record | UTC | Allowed | Projected bytes | some avg10 |
|---|---|---|---|---|
| bar denied1 | 11:26:50.265452 | false | 20175354420 | 7.30 |
| bar denied2 | 11:34:02.222936 | false | 41859263164 | 16.52 |
| preflight3 | 11:36:26.093880 | false | 45690990884 | 8.29 |
| bar admitted | 11:42:06.535203 | true | 38847278572 | 0.11 |
| HUD admitted | 11:42:11.239652 | true | 39721628648 | 0.06 |

Denied records are admission failures, not executed test failures or extra cases. Successful argv uses agents.slice, MemoryMax4G/MemoryHigh3G, CPUQuota200%, KillMode control-group, outer timeout86s, bwrap read-only mounts, isolated network, temporary /tmp, and disabled CASC/audio. These runs consume the existing PTR UI cache; they do not test CASC extraction/network/sound.

## Source and policy boundaries

- Sealed source `1804d1c49d5a54ca95a8099a75611450e79ca480`, production delta `5bd208dba`. Source1804 changes only two consumer tests and three docs: admits PTR cfg, prints active cache, formats an assertion; no assertion weakening or production change. `git diff 5bd208dba 1804d1c49 -- src` is empty.
- Saved launcher mounts older source snapshot `39ce8969a` as the runtime working tree, while executing the sealed source1804 test binary. Do not conflate snapshot identity with compiled-test identity. Runtime-input-reuse records this distinction; current stat matches the saved artifact receipt. No fresh content attestation was permitted.
- Consumer source resolves active-profile cache, uses actual cached addon dependency closures with empty overrides, and executes bar update/zero-neutral/hidden-restore/removal/unregister assertions. HUD cases prove supplied selections/record consumption and cross-set isolation, plus default-zero no registration/pool/publication; they do not assert HUD visibility or native supplied-data parity.
- Production5bd changes simulator `Texture:SetAtlas` to return exactly one modeled application-result boolean: true after resolved application, false for accepted non-applying nil/inactive/missing inputs. Existing validation and mutation paths remain. This supports unchanged cached TextureKit `SetShown(success)` consumers; **simulator compatibility policy, not established native return parity**. Generated docs/Wowless omit returns; this packet does not resolve that conflict. No vendor behavior edits follow from source1804 or production5bd.

## Unproved / excluded

Prior PTR26 and normal-startup results are marked reused by epoch metadata, not newly executed or independently verified here. No full-autoload/full-suite/native/other-profile/GUI acceptance, cache content attestation, or exhaustive atlas-input policy verification is claimed. Historical RED and denied admissions remain separate. Parent acceptance remains OPEN.

Tracked status was empty before report creation and remained empty at proof creation; report destination has no tracked files. Pre-existing untracked `.code-index.db` and `verification/` were preserved. No commits or staging.
