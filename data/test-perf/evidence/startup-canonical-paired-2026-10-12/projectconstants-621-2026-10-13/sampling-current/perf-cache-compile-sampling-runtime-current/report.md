# Saved sampling invocation/runtime verification

**PASS — invocation/runtime evidence only.** No replay, new sampling, build, hashing, delegation, perf.data analysis, kernel/settings changes, or tracked edits.

## Evidence and counts
- Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-removal-green-20261013T193523870575Z`; saved source revision `621649e56d8418d21c4539b26d787ad40c73431f`, Retail.
- Saved `systemd-run --wait`: exit **0**; service **success**, main exit **0**. Runtime **8.696s**, CPU **7.870s**, memory peak **2.1G**, swap **0B**.
- Entire stdout: **3 bytes**, `[]` plus newline. Entire stderr read/classified: **173668 bytes / 2665 lines**: **2153** file-load lines, **291** addon summaries, **211** timed logs, **8** other lines, **2** perf summaries. All line text and classifications preserved in `proof.json`.
- Newly derived addon-summary totals: **1573 Lua**, **677 XML**, **0 warnings**, **185 nil observations**, **88 missing requirements**. These are observation counts, not unique symbols or newly introduced regressions.
- Final stderr: `Status: CLEAN`; **0 unique / 0 occurrences** Lua errors; **0 attributed owners / 0 unattributed occurrences**. This does not erase nil/missing-requirement observations.
- Perf stderr: `Woken up 218 times to write data`; `Captured and wrote 54.311 MB`. **0 non-summary warning lines; 0 lost-warning lines**. Actual sample/lost-record counts and stack quality remain unproven without offline analysis.
- `perf.data`: stat only, **56986476 bytes**; no contents read. Offline analysis belongs to other agent.

## Exact invocation and isolation
Full ordered argv retained in `proof.json` and original `argv.json`.

`perf record --no-buildid -e cpu-clock:u --freq 999 --call-graph dwarf,8192 -m 512K -o <epoch>/runtime-perf-cache-compile-sampling/perf.data -- <epoch>/artifacts/wow-sim --no-addons --no-saved-vars lua-errors`

Outer invocation: `systemd-run --user --wait`, MemoryMax **4G**, MemoryHigh **3G**, CPUQuota **200%**, KillMode **control-group**; `timeout --signal=TERM --kill-after=2 86`; bwrap network unshared, root/source overlay/target read-only. Sampling output directory sole persistent writable bind; ephemeral `/tmp`, `/dev`, `/proc` also mounted, so not a claim that only one filesystem was writable.

Environment: `WOW_SIM_CASC=0`, `WOW_SIM_NO_SOUND=1`, `WOW_SIM_VERBOSE=1`. Logs confirm `casc=false`, `Sound disabled`, `SavedVariables loading disabled`, `Addon loading disabled`. Blizzard UI still loaded. Read-only EditMode cache was loaded from WTF despite SavedVariables suppression.

## Admission and original receipt reuse
Saved sampling admission timestamp **2026-10-13T20:27:45.091702+00:00**, `allowed=true`, projected memory **33533003648**, host available **18925723648** bytes. Original normal copied executable receipt reused; saved sampling stat and current stat-only check match original receipt: device **66306**, inode **142479845**, size **408320640**, mtime_ns **1791920284944505512**. No new byte authentication; no old-copy reauthentication. Source/executable/library authentication not established anew.

## Actual cache/runtime path
Blizzard load **4.65s**: io **150.10ms**, XML **333.91ms**, xmlproc **2.66s**, Lua **1.08s**, reported compile **914.10ms**, call **164.51ms**. Bytecode cache **1569/1569 hits (100%)**, lookup_miss/replay_fail/stored/store_fail all **0**. Normal cache-or-compile invocation, not proof of cold compilation; compile-labelled timing does not override cache-hit evidence. GC restart completed in **267.51ms**. Logged workaround application observed, not introduced or modified here.

## Installation and scope
Saved installation evidence: `<PRIVATE_VERIFICATION_ROOT>/perf-tool-install-20261013`. Actual exit **0**; **6 packages** installed: pcre 8.45-5, slang 2.3.3-4, numactl 2.0.19-1, libtraceevent 1:1.9.0-1, libpfm 4.14.1-1, perf **7.2.10-1**. Saved `perf_event_paranoid` **2 before / 2 after**. No kernel/settings changes performed in this verification; historical conclusion limited to saved evidence.

Userspace on-CPU sampling only: no kernel/off-CPU coverage, addon-exclusive cost attribution, native-WoW parity, frozen CCD repair, sample analysis, or causal performance claim.
