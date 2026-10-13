# Genuine RED — saved-evidence validation

**PASS: genuine RED established.** Saved runtime is **FAILED: 0 passed / 5 unique failed**, not a feature-completion result.

## Scope

- Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T115734909991Z`.
- Historical source revision: `42f206f665ebdef86cdabe28e43fbad57221ff93` (DTO/map/test-only pre-producer stage).
- Reviewed saved compilation/runtime evidence and relevant archived source, not current production source. Current main's later implementation is outside this verdict.
- No builds, test executions, service operations, delegation, binary/library content reads, new hashes, or tracked edits. Only saved text/source reads, parsing, sealed-executable stat, and requested report writes.

## Compilation and selection

Full `compile.stdout` decoded: **741 valid JSON records**, comprising 665 compiler artifacts, 75 build-script records, one build-finished record; **zero compiler-message records**. Final line 741 is `{"reason":"build-finished","success":true}`. Unique integration test artifact at line 740 has `target.kind=["test"]`, `profile.test=true`, `fresh=false`, executable `target/debug/deps/integration-9f07e4a55f2bbbf7`. Its complete record matches `seal-integration.json`; sealed path matches `compile-result.json` and runtime argv.

Full `compile.stderr`: compilation finished in **5m 19s**, with six deprecated iced-wgpu-patched manifest lint-key warnings (and one aggregate warning line), not a warning-free build. No compiler error reported. Terminal evidence: `SubState=exited`, `Result=success`, `ExecMainCode=1` (systemd exited code), `ExecMainStatus=0`; terminal timestamp **2026-10-13 07:02:54 CDT / 12:02:54 UTC**. Earlier observation files say running and are not terminal proof. `ActiveState=active` is consistent with saved `RemainAfterExit=yes` submission.

Recorded local toolchain: Cargo/rustc **1.99.0**; saved build argv uses `/usr/bin/cargo test --test integration --no-run --offline --locked -j4 --message-format=json-render-diagnostics`, `RUSTC=/usr/bin/rustc`, systemd 8 GiB maximum / 6 GiB high / 800% CPU. Compile admission says allowed; peak 6,444,810,240 bytes below saved worker maximum.

## Authentication — receipts plus stat only

Saved receipt reports `authentication_result=PASS`, digest match, and before/after stat match. Saved seal and receipt digests agree. Current **stat only** matches both: device 66306, inode 140286946, size 470644760, mtime_ns 1791892974138753954. No executable bytes read or digest independently recomputed in this validation. Receipt authentication is not behavioral proof.

## Runtime evidence and failure boundary

Full runtime stdout (1669 bytes), stderr (5292 bytes), argv, wait-launcher, result and admission inspected. Saved argv selects the sealed integration executable with `ui_widget_manager_discrete:: --nocapture --test-threads=1`, under `systemd-run --wait`, timeout 86 seconds / kill-after 2, read-only bwrap source overlay, network isolation, tmpfs `/tmp`, `WOW_SIM_CASC=0`, `WOW_SIM_NO_SOUND=1`. Runtime admission says allowed with 4 GiB maximum / 3 GiB high; wait-launcher reports **exit 101**, service runtime 841 ms, memory peak 153.1M. This is a test failure, not timeout or OOM evidence.

Final parent summary, verbatim:

> test result: FAILED. 0 passed; 5 failed; 0 ignored; 0 measured; 10967 filtered out; finished in 0.77s

All five distinct test identities fail at archived `tests/widget_discrete/fixtures.rs:42:33`, with the same exact diagnostic:

> discrete [first genuine getter/record RED]: Lua error: missing supplied DiscreteProgressSteps record

| Test suffix under `ui_widget_manager_discrete::` | Observed boundary |
|---|---|
| `consumer::discrete_actual_cached_manager_updates_hides_restores_removes_unregisters` | Missing supplied record table |
| `queries::discrete_numeric_ids_preserve_fractional_negative_and_signed_zero_keys` | Missing supplied record table |
| `queries::discrete_same_ids_are_environment_owned` | Missing supplied record table |
| `queries::discrete_snapshot_is_complete_fresh_and_observes_supplied_replace_remove` | Missing supplied record table |
| `security::discrete_secrets_allow_untainted_deny_tainted_and_recover_public_snapshots` | Missing supplied record table |

The earlier **0 passed / 1 failed** summary is the consumer timeout-reexec child, not a sixth test. Archived `timeout_reexec.rs:177-204` replays child output and reports its exit 101 at line 184. The duplicate parent consumer panic is excluded; neither timeout expiration nor a second independent failure is counted.

## Source ordering and coverage limits

Archived fixture constructs all 19 supplied fields and inserts directly into the environment-owned map (`fixtures.rs:7-38`). Every test initializes its environment, seeds ID 760301, then calls `assert_initial_snapshot`. Within that helper (`fixtures.rs:50-83`), the getter function-type assertion succeeds before the supplied-table assertion fails at Lua source line 59. Thus failure is **not missing getter publication**, and the log does not independently identify the non-table return value as nil.

Consumer ordering (`consumer.rs:263-276`) places initial snapshot before membership seeding, `load_cached_consumer`, atlas prerequisites, XML consumer construction, geometry and lifecycle assertions. No `[discrete-cache]` marker occurs. **Zero observed environment/seed/setup failures**: all five reach the intended supplied-record boundary. This does not prove later cached-consumer prerequisites work. Startup intrinsic-template initialization is visible in stderr; “before templates” here means the explicit cached consumer/template path, not absence of all template initialization.

| Capability | Proof level in this epoch |
|---|---|
| Supplied record DTO/map and test compilation | Compiles; archived definitions inspected |
| Getter is callable but does not return supplied table | Five unique behavioral RED failures |
| 19-field projection, freshness, replace/remove, numeric-ID behavior, environment isolation | **Unreached** |
| Secret wrappers, tainted rejection, public recovery | **Unreached**, including wrapper installation |
| Cached manager loading, atlas application, XML template, event/pool/layout/lifecycle behavior | **Unreached** |

## Warnings

Saved `revision.json` and `runtime-inputs.json` associate build/snapshot with the historical commit, but explicitly provide no source-tree/archive hash. Runtime binds the live target directory and external profile cache read-only, not a separately authenticated dependency/cache snapshot. Those provenance limits are retained; no fresh library/cache authentication was attempted. Cache/atlas consumers were not reached, so their correctness is not claimed. Admissions are saved observations, not fresh capacity measurements. This verdict validates only this historical RED epoch, not current implementation or GREEN readiness.
