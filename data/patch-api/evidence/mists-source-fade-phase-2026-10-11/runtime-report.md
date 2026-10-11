# Mists saved-evidence verification

Evidence: `/home/osso/.local/state/wow-ui-sim/verification/mists-original-phase-current/20261011T155435Z`
Revision: `4e3848a77747bfdfe048f68fca9cc1e231c5750f`

**OVERALL: FAIL — original cast-bar case remains failing.**

| Proof | Result | Evidence |
|---|---|---|
| Compile | PASS, exit 0 | compile-result.json; compile.stdout build-finished success=true |
| Artifact identity | PASS, 2/2 | Cargo records/features/profile match sealed records; independent SHA-256 and size match |
| Before/after inputs | PASS, 13/13 | compile-submission.json vs outcome.json; saved inventories only |
| Original prefork case | **0 PASS, 1 FAIL; exit 1** | original.stdout:1–4; original.stderr panic at tests/spell_casting.rs:495:13 |
| Fade control | **1 PASS, 0 FAIL; exit 0** | fade-control.stdout:7,11; nested summaries represent one case |

## Original failure boundary

Case: `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix`.
Hidden condition passed before `parent==UIParent` failed: `hidden Mists cast bar should remain parented to UIParent`.
Raw settled trace (`original.stderr:118`) confirms `IsShown=false`, `parent.name=UIParentBottomManagedFrameContainer`, `parent==bottom=true`, and both managed membership entries `nil`.
Before EDIT_MODE_LAYOUTS_UPDATED: parent UIParent; after event/startup: parent bottom container, shown and membership present; settled: still parent bottom container, hidden and membership nil.
Original trace has no numeric 0.55s label. The request supplies that original settling duration; fade-control.stderr explicitly logs `after 0.55s: (false, false, true)`. No full original case pass or later assertions claimed.

## Authenticated artifacts

Features for both: `casc, client-mists, gui, prefork-full-ui, rodio, sound`. Both Cargo records have `fresh=false` and test profile opt_level=1, line-tables-only debuginfo, debug assertions and overflow checks enabled.

- `integration`: compile.stdout:741; 438425936 bytes; SHA-256 `d44335ada015b215e7bf93a106304083c008e84c090ef035dec99d6780771870`; hashed once.

- `prefork_full_ui`: compile.stdout:740; 342171960 bytes; SHA-256 `f0c67902a017f9a9bfb8b3a37a0e5fd8f50f6460c4cd386bc2d2e94abaf483f0`; hashed once.

## Thirteen saved before/after inputs

| Input | Before = after SHA-256 |
|---|---|
| `Cargo.lock` | `5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1` |
| `Cargo.toml` | `91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9` |
| `build.rs` | `cdebff6b6dec0e19e1124e734b2603432e2c47a7b956f9a4568ae412de182741` |
| `src/c_api/mod.rs` | `2b7241f282613f370e1b726bce309099638dc2d2688b342e7579e8052a8d9798` |
| `src/c_api/registration.rs` | `a8d9d5e107889ee8fdfa643eae08b1eee824e4e61d604853b1d9520236064ba4` |
| `src/lua_api/globals/missing_surface/achievement_info.rs` | `b97e5655c45e6594a63f526b0c95c72ac393821cbdb918cfcb38e10e83a64a9a` |
| `src/lua_api/state.rs` | `0a9603dcb351ab356f41bb6f1fd8d6e6a7e3a114d790cea1236fe0157253e351` |
| `src/lua_api/state/sim_state.rs` | `1454e312e6996cd9b7bb64ed4a37bdd85ff881e3a67bd5ef10d19306e11e9cd4` |
| `tests/c_achievement_info_probes.rs` | `480fc7f79350873eff06c4ecaa53329e14dfb994e5675253f02d3f21574089e8` |
| `tests/c_achievement_info_probes/superceding.rs` | `665a09046d8a5e1d4508a0d9a61711378c2732df4f8de3bd569afb40423ca641` |
| `tests/garrison_damage_classes.rs` | `3cf05b173c70ba57b599b0a87556dda27e8f414196585af507ebf83b7077e58e` |
| `tests/mists_cast_fade.rs` | `4fea29a7241f6eb9953f38b59e9699bf4f38ee16a0bf9144dc3fb735d6657257` |
| `tests/spell_casting.rs` | `d903e049824281d6e24189fa98b4fa6830966aa986439cfa08ce021ca5a4c3af` |

## Failures and limits preserved

Startup Lua frame-creation self-anchor errors appear in both executions. Early diagnostic reads of nil systemInfo produce READ_ERROR. Compile emitted six deprecated Clippy manifest-key warnings. Original panic remains a failure. Full stdout/stderr retained verbatim in report.json; source evidence remains untouched.
Native parity not assessed and not a gate. Saved revision attribution is authenticated against submission metadata, not a fresh whole-tree or native capture.
No tests/builds/ops/source edits/cwd changes/delegation performed.

## Exact case output

```text
running 1 tests
test spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix ... FAILED (panic: hidden Mists cast bar should remain parented to UIParent)

test result: FAILED. 0 passed; 1 failed; 1 total
```

```text

running 1 test
test mists_cast_fade::mists_idle_cast_bar_hides_after_source_fade_finishes ... 
running 1 test
test mists_cast_fade::mists_idle_cast_bar_hides_after_source_fade_finishes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8210 filtered out; finished in 6.13s

ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8210 filtered out; finished in 6.20s
```
