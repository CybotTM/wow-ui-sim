# returnedLoadResult — independent compile/source gate

**Verdict: PARTIAL_COMPILE_SOURCE. Behavior, repair, full acceptance: PENDING.**

## Authentication
New probe authenticated exactly once before long evidence output. Computed size **349156608 bytes**, SHA256 **5c0d33ddbe6fee24d061efd742b6de2e3bd96faa0443cf866dca4f23e4c91a66**; both expected comparisons match. First receipt: `/home/osso/.local/state/wow-ui-sim/verification/startup-addon-inner-timing-current/compile-independent-auth.json`. Future behavior gate MUST reuse this receipt; never rehash. No other binary/library payload read or hash.

## Compile/source evidence
- Actual saved compile guard: 2026-10-12 05:01:21.007662 UTC, load1 15.23, available RAM 22.126293182373047 GiB, relevant Cargo []: admitted. No fresh guard run.
- Saved rustc result: exit 0, source_unchanged=true. Full compiler stdout and stderr read: both 0 bytes. Controller receipt completed; controller stderr empty. No build rerun.
- Toolchain receipt: /usr/bin/rustc 1.99.0 (b940084d7, 2026-09-28), Arch 1:1.99.0-1, x86_64-unknown-linux-gnu, LLVM 23.1.1; saved toolchain exit 0. Edition 2024, opt-level 1, clang/mold, native CPU, 4 linker threads. Exact argv retained in proof JSON.
- Frozen source equals private, tracked working source, and committed source at cfcede4a0ec641c311599a3e03205a3851bc1b23 byte-for-byte (6288 bytes). This establishes source identity, not an independent source-preflight PASS.
- Both selected library projections match their unique compiler-artifact Cargo records in the recorded producer log. Current stat matches saved bytes/mtime: wow_ui_sim 430762454 / 1791773709051369162; serde_json 2871460 / 1791648084346200629. wow_ui_sim uses client-retail; producer revision recorded as 262987b1a65f049d5c3e158b18a8eb73cc22e4ba. Stat/record evidence is not payload authentication, hermetic provenance, or frozen acceptance-library proof.

## New source contract coverage
| Boundary | Observed source/compile evidence | Proof level |
|---|---|---|
| Setup | Read-only bytecode mode precedes WowLuaEnv::new; Game screen, 1024x768; exactly active-profile Blizzard addon path; no extra path or scan registration | Source + typed compile |
| Load sequence | discover_blizzard_addons_for_screen(Game), sequential load_addon, env.loader_env; no startup events/ticks | Source + typed compile; behavior pending |
| Returned fields | Identity and file counts; 8 base and 10 XML/frame nanosecond keys; 4 creation/6 cache counters; 3 diagnostic lengths | Source + typed compile |
| Bucket meaning | lua_exec aliases compile+call; detail buckets overlap; total uses public LoadTiming::total, not an all-field exclusive sum | Source; src/loader/mod.rs:277-380 |
| Errors/count/exit | First hard error stops; successful prefix preserved; summary count is rows.len; errors classify variant without Lua/XML text; environment/output/hard errors yield failure | Source + typed compile; runtime pending |
| Completion limits | Empty discovery and zero-file results retained; complete means no returned hard error, not all addons loaded or no diagnostics | Source only |

## Runtime and retained gaps
Exact runtime request `runtime-request-20261012T050400298081Z.jsonl` retains compile request definitions but strips the final compile_probe() call; adds submit_runtime and dispatches only print(submit_runtime(new epoch)). Definition of compile_probe is inert: no compile dispatch from this request. Read as data only, not executed.

Actual saved runtime guard: 2026-10-12 05:04:00.319151 UTC, load1 33.79, RAM 21.60045623779297 GiB, relevant Cargo []: denied (load threshold <24). RESOURCE_BLOCKED, submitted=false, executed_observations=0; runtime-submission.json absent. No runtime behavior established.

Main format.json exit 0 and manual readability receipt reused without formatter or rereview. Agent 2554 preflight report was not persisted: wrong /root destination produced EACCES. preflight-output-gap.json retained; **no source-preflight PASS invented**. Old probes 2542/2549 not read, hashed, run, or rebuilt. Old startup row/file scope excluded. No tests, checks, runtime, guards, network, operational changes, or delegation performed.

## Owned outputs
- `/home/osso/.local/state/wow-ui-sim/verification/startup-addon-inner-timing-current/compile-independent-report.md`
- `/home/osso/.local/state/wow-ui-sim/verification/startup-addon-inner-timing-current/compile-independent-proof.json`
