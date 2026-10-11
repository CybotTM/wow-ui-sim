# Independent saved-receipt verification — unit2 authentic RED

**PASS — expected RED authenticated, not a passing implementation.** Saved revision `c4f90ce2ecf6bfbede6b61ef21638003956a6f5b`; parent `446ca8cd545b194f65fe2e6b19cde6afedcca562`. Evidence epoch: `20261010T234631Z/` beneath this report's directory. Unit: `wow-unit2-red-20261010t234631z`.

Verification read `/home/osso/AgentConfig/skills/verify/SKILL.md`, saved worker/request, admission/submission/compile/artifact/execution/outcome receipts, complete test stdout/stderr, complete compile stderr, all compile JSON records, new test source and relevant producer/registration source. No build, test, replay, operation, tracked edit, delegation, Bash, rg or fd was performed. Receipts were already complete when located; no waiting or resubmission was necessary. Missing receipts were not treated as passing.

## Exact behavioral results

| Saved scope | Observed result | Exit |
|---|---|---|
| `unit_event_second_unit::` | 2 executed: 1 passed, 1 failed; 0 ignored/measured; 10715 filtered out | 101 |
| `reregistering_and_unregistering_clear_the_second_unit_filter` | PASS within preceding execution | 101 for containing execution |
| `second_unit_filter_receives_modeled_player_and_target_health_events` | Expected FAIL: `second unit must receive target damage` | 101 for containing execution |
| Exact `frame_creation::visibility_scripts::test_set_shown_boolean_transitions_update_frame_state` | 1 passed, 0 failed/ignored/measured; 10716 filtered out | 0 |

Combined: **3 executed, 2 passed, 1 expected failure**. Both executions used the sealed integration binary, `timeout 90`, `--nocapture`, `--test-threads=1`; the boolean fixture also used `--exact`. No timeout exit. First execution: October 10, 2026, 23:47:43.645911–23:47:44.133698 UTC; second: 23:47:44.133835–23:47:44.416419 UTC.

### Failure boundary and controls

`0.stderr` identifies panic at `tests/unit_event_second_unit.rs:40:6` with Lua assertion text **`second unit must receive target damage`**. Source assertion is line 35. Both instant casts are present in stderr: spell 85673, then 35395.

The test executes sequentially. Reaching line 35 proves preceding assertions completed: player heal changes 50000 to 70000 (line 26), both-unit listener receives player (27), player-only listener receives player (28), target-only listener suppresses player (29); target damage changes 50000 to 35000 (32), target-only listener receives target (33), player-only listener suppresses target damage (34). This is missing second-unit delivery **after successful modeled producers and single-unit controls**, not a failure caused by absent health production or an earlier player-heal failure. The `IsEventRegistered` query at lines 36–37 follows the failed assertion and was **not reached**.

Lifecycle test passes replacement, unregister and unregister-all controls (lines 43–76). It does not independently prove second-unit delivery; that is the expected failing test's responsibility.

Boolean fixture is a direct Frame API test (`tests/frame_creation/visibility_scripts.rs:4–18`): Hide → `SetShown(true)` → `SetShown(false)`, with `IsShown` assertions. No downstream Blizzard UI fixture or injected replacement method is involved.

## Source and producer provenance

Commit changes only two test files: added `tests/unit_event_second_unit.rs` (76 lines), modified `tests/frame_creation/visibility_scripts.rs` (+17 lines); 93 insertions, no production changes. Parent/RED `git ls-tree` output is identical across `src`, `build.rs`, `Cargo.toml`, `Cargo.lock` (1656 entries); production diff is empty.

All **3846** paths in `source-before.json`, `source-after.json` and `source-after-execution.json` are present, dictionaries identical. Independently recomputed current-file SHA256 matches every saved entry; independently computed Git blob identities match the c4f90ce2e commit tree for every path. Commit tree covers exactly those 3846 snapshot paths: zero missing paths, zero mismatches. Each snapshot file SHA256: `a62bbd38dd0245dd7ff3d4ac7028641aaaeb49051e51d9e5c082d7fcb0f49d63`.

| Source | SHA256, identical before/after compile/execution and independently recomputed |
|---|---|
| `tests/unit_event_second_unit.rs` | `4363f0e04672a7698d32a4a777886b02bad379f04518b221fa73da3763e8ca4e` |
| `tests/frame_creation/visibility_scripts.rs` | `02e8df60ce63834c9cd232f1ad096b4e203b409eafa11aad63345977b55d3c3a` |
| `src/lua_api/globals/admin.rs` | `4c2db2e233824d702bb20d3bbe9209e897637c91e489d1878d129d9f51e29d07` |
| `src/lua_api/globals/combat_verbs.rs` | `ee327eb5201d2c3c29034b54d1de00c9aa13ad6b4cd9775396d5b0a3e727f4d0` |
| `src/lua_api/game_data.rs` | `c4a553c29b3516dc61416a1b4cf6ae451a6491c8a36ce2155f9bbc883f42fb0a` |
| `src/lua_api/frame/methods/text_attribute_event/events.rs` | `75c7c9c8118bb8c889182fc2171b47234553905c2a85bd39b590785e18e88a19` |
| `build.rs` | `df6b286152daa948557c17c4abb8776af2aed573c45f1fcb4901fd2b4a8ccde2` |

**Admin health setters do not fire events.** `admin.rs:570–577` and `609–618` mutate health/max and return; no event dispatch. They seed this fixture before registrations. Documentation is not producer evidence.

Actual producer: registration at `combat_verbs.rs:630` exposes `CastSpellByID`; `cast_spell_by_id` (367–374) calls `execute_spell_by_id` (202–228), which calls `apply_spell_to_target` (263–282). It calls `game_data::apply_spell_to_state` and dispatches `UNIT_HEALTH` for `UnitHealthChanged(unit_id)`. `game_data.rs:219–240`, `298–343` performs damage to enemy target or heals player when target is enemy. No health-producer change occurred in this commit or across saved snapshots.

Preproduction registration source `events.rs:41–67` reads only first-unit argument at stack slot 3 and registers that unit; no second-unit argument is consumed. This agrees with observed first-unit success and second-unit failure.

Wiring: `tests/integration.rs:1` includes generated `integration_tests.rs`; `build.rs:54–90`, `464–472` discovers top-level Rust tests and emits modules. Saved runtime output names both newly discovered tests and the exact nested visibility fixture, proving they executed in the integration harness.

## Compile, artifact, controller and admission

Saved compile argv: `/usr/bin/cargo test --offline --locked --test integration --no-run --message-format=json`, cwd `/home/osso/Projects/wow/wow-ui-sim`, override `CARGO_BUILD_JOBS=12`. Started 23:46:31.680951 UTC; ended 23:47:42.422679 UTC. `compile-result.json`: **exit 0**, no stream errors, source equal. Complete JSON stdout: **738 records** = 663 compiler-artifact (655 fresh, 8 nonfresh), 74 build-script-executed, 1 build-finished with success true; zero compiler-message records. Complete stderr ends with `Finished test profile [optimized + debuginfo] ... in 1m 10s`. Six dependency manifest lint-key deprecations are reported; stderr additionally summarizes six warnings. This is compile success, not warning-free proof.

Exactly one integration compiler-artifact: target kind test, source `tests/integration.rs`, fresh false, executable `target/debug/deps/integration-6fa7ee1f035abc08`. Test profile opt_level 1, line-tables-only debuginfo, assertions/overflow checks enabled. Features in saved artifact receipt match that compiler record; client-retail/profile-retail with retail capability features through retail-12-1-0 and default/gui/casc/sound included.

Sealed copy `20261010T234631Z/integration-test`: **467314720 bytes**; independently recomputed SHA256 **`c255f0dfbfb9730a7830a240f2ea9d7317f6fe28fab51825ca776a2c4de758f0`**, matching artifact receipt. Worker checks original/copy equality before execution; outcome records artifact unchanged afterwards. Both execution argv entries point to this sealed copy. Artifact is admitted for the saved test scopes, not a general whole-profile acceptance claim.

Submission receipt exit 0, invocation ID `8d875604a2384ebf89d97490d1abf11e`. Saved cgroup explicitly names `agents.slice/wow-unit2-red-20261010t234631z.service`. Controller stdout contains one completed receipt; controller stderr empty. Queue argv requests CPUQuota=1200%, MemoryMax=16G, Type=oneshot, RemainAfterExit=yes. These are saved submission settings, not an independent measurement of live cgroup enforcement.

Admission: attempt **1**, admitted true, MemAvailable **38.990535736083984 GiB**, load1 **18.61**, relevant Cargo processes **[]**; matches worker thresholds at least 6 GiB, load below 24, no relevant Cargo. Complete result receipts exist; no missing receipt was converted into a pass.

## Proof limits

Receipt/source/artifact verification only; no replay. Hash snapshots cover tracked `src`, `tests`, `Cargo.toml`, `Cargo.lock`, `build.rs`, `.cargo/config.toml`, not all vendored/external/build inputs or a hermetic dependency closure. No independent toolchain-version receipt exists in this epoch; no version command was run. No claim of green repair, whole-suite success, native WoW parity, or production deployment.

**Conclusion: PASS for authentic tests-only preproduction RED admission at c4f90ce2e. Exact failure boundary and passing lifecycle/boolean controls verified.**
