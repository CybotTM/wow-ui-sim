# Animation complete saved-artifact proof

**PASS — logical 6/6 PASS; wait/launcher exit 0.**

Revision: `a8a148e98d8dc32cf81e789fc8926830c9ed0161` (sealed a8).
Runtime epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-green-retry1-20261013T143805954256Z/runtime-animation-complete-20261013T155447771415Z`.
Explicit working directory: `<REPO>` (saved systemd working-directory and bwrap chdir).

## Exact saved command (not rerun)

```text
systemd-run --user --wait --unit=wow-animation-complete-20261013t155447771415z --slice=agents.slice -p MemoryMax=4G -p MemoryHigh=3G -p CPUQuota=200% -p KillMode=control-group -p StandardOutput=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-green-retry1-20261013T143805954256Z/runtime-animation-complete-20261013T155447771415Z/stdout -p StandardError=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-green-retry1-20261013T143805954256Z/runtime-animation-complete-20261013T155447771415Z/stderr --working-directory=<REPO> /usr/bin/timeout --signal=TERM --kill-after=2 86 /usr/bin/bwrap --die-with-parent --unshare-net --ro-bind / / --ro-bind <PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-green-retry1-20261013T143805954256Z/source <REPO> --ro-bind <REPO>/target <REPO>/target --tmpfs /tmp --dev /dev --proc /proc --chdir <REPO> --setenv HOME <HOME> --setenv WOW_SIM_CASC 0 --setenv WOW_SIM_NO_SOUND 1 <PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-green-retry1-20261013T143805954256Z/artifacts/deps/integration-9f07e4a55f2bbbf7 animation_query_lifecycle:: --nocapture --test-threads=1
```

## Artifact inspection

- Fully read argv (1702 bytes), admission (2389 bytes), launcher (299 bytes), stdout (661 bytes / 11 lines), stderr (3831 bytes / 66 lines), result (435 bytes).
- Admission allowed at 2026-10-13T15:54:47.778641+00:00. Limits: MemoryMax=4G, MemoryHigh=3G, CPUQuota=200%, KillMode=control-group; timeout TERM 86s, kill-after 2s; read-only saved source/root, network unshared, tmpfs /tmp; CASC=0, sound disabled.
- Launcher: success, exited 0; service runtime 659ms, CPU 585ms, memory peak 131.7M, swap 0B. Test summary: 6 passed, 0 failed, 0 ignored, 0 measured, 10970 filtered out, 0.60s.
- All stderr inspected: six VM startup sequences, 66 `[Startup]` diagnostic lines; no observed runtime failure. This is **not full Blizzard CLEAN**.
- Existing `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-green-retry1-20261013T143805954256Z/receipt-integration.json` says authentication PASS. Stat-only reuse matches saved before/after/current device, inode, size and mtime_ns. No binary read, new hash or authentication.
- Existing `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-green-retry1-20261013T143805954256Z/revision.json` identifies exact a8 revision; saved runtime result agrees. Existing `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-green-retry1-20261013T143805954256Z/compile-result.json`: build_finished=true, 741 JSON records, compiler_messages=[], ExecMainStatus=0. Existing text only, no build rerun.

## Six source-backed behavioral cases

Source fully inspected using bounded reads 1–200 and 201–333: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-green-retry1-20261013T143805954256Z/source/tests/animation_query_lifecycle.rs`. Each selector below has its own stdout `... ok` record.

- **PASS** `animation_query_lifecycle::owner_queries_follow_pause_resume_restart_and_stop` (lines 39–93): Playing/paused/done/pending and idle states; elapsed/progress freeze on pause, advance on resume, reset on restart/stop; stopped state persists.
- **PASS** `animation_query_lifecycle::pending_finish_queries_are_settled_before_callbacks` (lines 95–133): Finish from pause sets pending state; zero-delta update settles flags and progress before OnFinished; callback fires once and again after restart/natural completion.
- **PASS** `animation_query_lifecycle::reverse_and_repeat_queries_follow_playback_direction` (lines 135–178): Reverse elapsed/progress moves from 0.75 to 0.5; REPEAT wraps to 0.25 forward and 0.75 reverse with loop/direction flags.
- **PASS** `animation_query_lifecycle::bounce_loop_callback_observes_direction_change` (lines 180–204): OnLoop sees BOUNCE, active state, self identity and alternating reverse flag; two crossings produce loops=1/reverse and loops=2/forward.
- **PASS** `animation_query_lifecycle::delayed_animation_queries_use_local_active_elapsed` (lines 206–262): Start-delay flag and zero local progress; active local elapsed/progress 0.25 and 0.75; end-delay clamps local progress to 1 while group continues; completion settles done.
- **PASS** `animation_query_lifecycle::delaying_uses_order_start_and_longest_parallel_duration` (lines 264–332): Order-2 longest duration plus end delay sets order-9 start; forward/reverse timeline exact tuples; pause removes delaying; omitted resume direction preserves reverse, explicit false switches forward without resetting elapsed.

## Failures and limits

Observed failures: none in the scoped saved artifacts.

Six-case coexistence/query proof only. No broad CCD/PTR/native/global-coverage, full-Blizzard-clean or performance-cause inference. Saved metadata and stat reuse are not fresh independent binary/library content authentication. No rerun, build, operations, delegation, tracked edit or commit.
