# Saved-proof verification — source621

**Runtime result: FAIL. Artifact interpretation: verified.** No builds, runtime execution, hashes, delegation, tracked edits, or threshold changes performed. This review follows verify/SKILL.md as the assigned verifier, using saved artifacts only.

## Evidence scope

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-removal-green-20261013T193523870575Z`.

Read complete `runtime-perf-owner-localization/{stdout,stderr,admission.json,argv.json,launcher.txt,result.json}` and `revision.json`. Both streams fit within a single read: stdout 644 bytes and stderr 1975 bytes according to the saved result. Source association is the saved source621 metadata and read-only source mount; no fresh binary/source identity or hash verification was performed.

## Cases and execution

One selected test: `addon_loading_perf::heaviest_blizzard_addons_stay_within_per_addon_load_budgets`. **0 passed, 1 failed, 0 ignored, 0 measured, 10993 filtered out.** Two summaries are child/parent reports of the same case, not two cases. Child finished in 7.59s; parent in 7.77s. Saved `systemd-run --wait` launcher exit is 101; service main status is 101, runtime 7.883s, CPU time 6.829s, peak memory 1.9G, swap 0B. This is a completed assertion failure, not a timeout or admission rejection.

Body execution is directly evidenced by startup logs, eight concrete addon totals, and the budget assertion panic at `tests/addon_loading_perf.rs:167`. The child panic is followed by the parent wrapper panic at `tests/common/timeout_reexec.rs:184`: `failed (exit code 101)`.

Read source: `tests/addon_loading_perf.rs`, `tests/perf/addon_loading.rs`, relevant `tests/common/mod.rs` definitions, complete `tests/common/timeout_reexec.rs`, and `src/loader/mod.rs:314–402`. The wrapper reexecutes the exact named test, records a handshake before calling its closure, drains both streams to EOF, validates the handshake, and replays failed child output. The parent failure contains no handshake-error suffix; this corroborates successful handshake validation, though the ephemeral handshake itself is not retained here.

## Admission and limits

Saved admission at 2026-10-13T19:55:55.967373+00:00: `allowed=true`, retail, worker max 4294967296 bytes, high 3221225472 bytes. Parent memory current 41641750528, high 42949672960, max 45097156608; recorded reclaim estimate 7610636112, projected 38326081712, host available 19691286528 bytes. This proves the recorded admission decision, not independently the admission algorithm or resource isolation from all host activity.

Launcher specifies MemoryMax=4G, MemoryHigh=3G, CPUQuota=200%, KillMode=control-group; outer timeout 86s, TERM then kill after 2s. Harness child timeout is 120s; outer 86s is the tighter run bound. Single test thread; Linux performance wrapper requests Exclusive workload gating. Bubblewrap uses network isolation, read-only root/source/target mounts, private /tmp, CASC=0 and sound disabled. Saved argv proves requested limits; no live property inspection was performed.

## Owner totals and preserved failure

| Owner | Recorded total | Applicable limit |
|---|---:|---:|
| Blizzard_ProjectConstants | 1.42s | unknown-heavy threshold >1s: FAIL |
| Blizzard_UIPanels_Game | 485.19ms | 3240ms |
| Blizzard_ActionBar | 480.05ms | 9240ms |
| Blizzard_GroupFinder | 360.31ms | 1100ms |
| Blizzard_CompactRaidFrames | 335.43ms | 1200ms |
| Blizzard_UnitFrame | 239.34ms | 1600ms |
| Blizzard_SharedXML | 179.57ms | unknown-heavy threshold >1s |
| Blizzard_Communities | 166.22ms | 1600ms |

Known budgets use budget + max(20% budget, 400ms). Blizzard_FrameXML's applicable limit is 1300ms; its actual total is not printed in the top eight. The complete regression list reports only ProjectConstants, providing indirect evidence that other known samples were present and within their limits. Exact unprinted totals remain unavailable.

`ProjectConstants became a new heavy-addon outlier at 1.42s` is the preserved failure. Its diagnostic suggestion to add a budget is not an approved remediation. No threshold or source changes made.

## Interpretation limits

Per-addon samples are `result.timing.total()` assigned to the discovery-loop owner name. `LoadTiming::total()` sums IO, XML parse, XML process, Lua execution, and SavedVariables timings; it does not sum overlapping detailed frame subsets. These are aggregate owner-labelled elapsed phase totals, not exclusive owner CPU measurements. Service CPU is whole-service accounting, not ProjectConstants CPU. Host pressure, scheduling, nested work, cache behavior, or GC could contribute; this evidence does not isolate a causal mechanism.

This single source621 retail Game-loading probe uses SavedVariables and does not prove repeatability, improvement/regression versus another revision, CharacterCreation/CharacterSelect performance, or repair/attribution of historical frozenCCD. Adjacent phase-budget tests were not selected. Failure remains open.
