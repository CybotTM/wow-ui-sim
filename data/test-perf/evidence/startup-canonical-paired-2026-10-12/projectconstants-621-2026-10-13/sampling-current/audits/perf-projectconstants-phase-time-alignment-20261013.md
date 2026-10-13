# ProjectConstants perf phase-time alignment

**Assessment: NO DEFENSIBLE ALIGNMENT.** Existing metadata does not allow assigning `ProjectConstants.lua`'s reported 696.2 ms window to a corresponding bounded range of perf samples. Preserve `UNKNOWN`; do not relabel any current sample subset as ProjectConstants work. A future run needs an explicit marker recorded in the perf clock domain or equivalent synchronized phase-boundary evidence.

## Existing evidence inspected (read-only)

- Current analysis: `<PRIVATE_VERIFICATION_ROOT>/perf-cache-compile-sampling-621-current/` (`perf-script.stdout`, existing reports, `proof.json`, argv, source stderr/result/metadata).
- Original recording: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-removal-green-20261013T193523870575Z/runtime-perf-cache-compile-sampling/` (`perf.data`, argv, stderr, result/admission metadata).
- Run identity links: perf samples name PID `1921871`, matching the source stderr's first line `[  0.000s] [startup] wow-sim process started pid=1921871`. The recorded command runs that wow-sim binary under `perf record`; sampling clock values span approximately `338289.562301`–`338298.039711`.
- Source stderr orders `[0.631s] Loading 291 Blizzard addons...`, then `loading file ProjectConstants.lua`, then `Blizzard_ProjectConstants loaded ... (696.2ms: ... compile=696.0ms ...)`, but the per-addon lines have **no elapsed timestamps**. The next timestamped aggregate is `[5.200s] Blizzard addons loaded in 4.65s ...`.
- Canonical `src/logging.rs` shows elapsed prefixes use process-global `OnceLock<Instant>` (`global_elapsed_prefix`); `src/bin/wow_sim/main.rs` initializes it from `env.state().borrow().start_time`. This is process/environment startup elapsed time, not a per-`WowLuaEnv` reset at each addon. It establishes the source log's general time origin, not a phase-boundary timestamp for ProjectConstants.
- `perf.data` metadata/proof records sampling frequency/event and run configuration, but no phase marker. Existing perf-script samples contain compiler/parser frames, yet no file/addon identity or reliable phase-entry/exit marker. Temporal proximity inferred from the beginning of the process or from a guessed mapping of perf clock to source elapsed time would not establish that the samples fall inside the 696.2 ms interval.
- The service's recorded `Service runtime: 8.696s` and `CPU time consumed: 7.870s` are aggregate process/unit measures, not phase markers. They cannot align this addon interval to sample timestamps.

## Conclusion and next evidence

The run/PID identity is defensible; phase-to-sample temporal alignment is not. Neither perf headers/argv nor PID/service timestamps identify the addon phase boundaries in the perf clock domain. Do not call the earliest samples, or any arbitrarily selected 696 ms slice, ProjectConstants samples. For a future authorized capture, emit explicit begin/end phase markers with timestamps from a clock correlated to perf's sample clock (or record marker events in the same perf stream); then bound samples by those observed markers. No new runtime, sampling, build, hash, delegation, or tracked changes were performed for this assessment.
