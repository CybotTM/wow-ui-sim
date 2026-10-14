# Resource binding follow-up — widget-two full suite

**Artifact-only follow-up.** No suite launch, polling, service mutation, binary-payload inspection, or delegation. Sources: submit/preflight record `verification/widget-two-fullsuite-submit-current/20261014T165753802196Z.json`; resource sample `.../resource-observation-20261014T1710.json`; terminal report and proof under `verification/widget-two-fullsuite-terminal-current/`; recorded result `<private-result-store>/4fc115b9357e39794423a7da2145960f27273e4e.{json,log}`; runner source `<private-project-root>/wow-ui-sim/tools/full_suite.py`.

## Revision and preflight binding

The submit record names SHA `4fc115b9357e39794423a7da2145960f27273e4e`; the current suite JSON has both `sha` and `ref` equal to that full SHA. The JSON records integration, prefork, and lib each exiting 0. The terminal report's log accounting likewise belongs to that same revision. This binds the saved run result to the requested revision identifier; it does not independently attest which source tree/build artifacts were actually executed.

Preflight at `2026-10-14T16:57:53.808781Z` recorded `gitstatus clean and git clean -ndx -e target empty; no deletion executed`. The admission snapshot recorded host cgroup `memory.current=37,127,712,768`, `memory.high=42,949,672,960`, `memory.max=45,097,156,608` bytes; projected use `37,748,069,904` bytes against host available `41,498,370,048`; budget `17,179,869,184` bytes; `allowed=true`. Hold was expired. These are admission-time facts, not suite resource consumption or service limits.

## Requested unit and observed service state

Submission returned exit 0, unit `<private-service-unit>`, invocation ID `<private-invocation-id>`. Runner source requests a transient user unit in `agents.slice` with `CPUQuota=1200%`, `MemoryHigh=16G`, `MemoryMax=16G`. Those are requested properties from source, not proof they were applied.

The saved observation at `2026-10-14T17:10:36.195748Z` identifies that exact unit. Its recorded cgroup values were `memory.high=17,179,869,184` and `memory.max=17,179,869,184` bytes (16 GiB each). Thus the requested memory limits are evidenced as effective at observation time. `cpu.stat` was recorded (usage 5,265,910,527 µs; user 4,222,718,400 µs; system 1,043,192,126 µs; throttled 2,635 periods / 850,682,467 µs), but the observation does not record `cpu.max` or `CPUQuotaPerSecUSec`; effective CPU quota is not independently established.

At that observation, `memory.current=4,746,985,472` bytes. `memory.peak=17,179,869,184` bytes exactly equals the configured 16-GiB maximum; this establishes the cgroup's recorded peak reached the limit, not a process RSS value or which process caused it. `memory.events`: `high=4`, `max=1186`, `oom=0`, `oom_kill=0`, `oom_group_kill=0` (other counters: `low=0`, `sock_throttled=0`). Therefore the recorded counters show limit/max events but no OOM or OOM-kill events at observation. Do not infer that no transient pressure/throttling occurred: `high` and `max` are nonzero, and sampled `memory.peak` reached the cap.

A separate read-only service metadata query observed `ActiveState=inactive`, `SubState=dead`, `Result=success`, `ExecMainCode=0`, `ExecMainStatus=0`; `ControlGroup` empty and live `MemoryCurrent`, `MemoryPeak`, `CPUUsageNSec` unavailable. This is terminal service status, not a live measurement. The contemporaneous unit resource JSON is the evidence for the cgroup's saved peak and event counters.

## What this does and does not establish

- **Established:** recorded run SHA, clean preflight statement, submit command's successful unit creation, exact unit identity, effective memory high/max and sampled cgroup peak/events, later inactive/dead successful service result with exit status 0, and suite JSON's three step exit-0 outcomes.
- **Not established:** effective CPU quota; process-level peak RSS; resource consumption for each suite step; runtime source/binary identity beyond recorded SHA; causality between reaching the memory cap/events and successful/failed test behavior. The suite finished successfully according to its result JSON, while the cgroup sample proves the resource cgroup reached its memory maximum. Neither fact explains the other causally.
- Runner defaults `CARGO_BUILD_JOBS=12`, `CARGO_TERM_COLOR=never`, `FULL_SUITE_JOBS=16` (nextest thread count), and inherits the environment. Those are source/config values. Result JSON records `WOW_SIM_TIMEOUT_FIXTURE_CAPACITY=null`; it does not capture the complete effective environment.

No payloads inspected. Existing suite artifacts and terminal report were not changed. Report file mode 0600; directory mode 0700.
