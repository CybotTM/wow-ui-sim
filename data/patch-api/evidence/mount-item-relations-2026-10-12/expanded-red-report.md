# Expanded mount-item RED verification — PENDING

Unit: `wow-mount-item-expanded-red-20261012t143207719476z.service`
Source: `3cc6d36af`
Epoch: `expanded-red-20261012T143207789737Z`

## Available evidence

- Unit last observed `ActiveState=activating`, `SubState=start`. No saved compile result, artifact, runtime result, format result or outcome yet. `Result=success`/`ExecMainStatus=0` during activation is NOT terminal success.
- Saved source-applicability: revision `3cc6d36af`, empty relevant diff, exit 0. Saved six-file source-before hashes present. After-compile/runtime hashes unavailable.
- Toolchain: cargo 1.99.0 (5f94df478 2026-08-27), rustc 1.99.0 (b940084d7 2026-09-28), x86_64-unknown-linux-gnu, LLVM 23.1.1.
- Independent `0808b3cd0..3cc6d36af` relevant diff: only three files, 119 insertions; state field (2 lines), empty initialization (1 line), expanded tests (116 lines). Original complete 4,628-byte three-case file is an exact prefix of expanded 8,158-byte file. No C API producer added.
- `src/lua_api/state/sim_state.rs:580`: `mount_item_relations: HashMap<i32, i32>`; `src/lua_api/state.rs:496`: `HashMap::new()`. Search of src found only these two references and no `GetMountFromItem`. `WowLuaEnv::new` creates its own `Rc<RefCell<SimState::default()>>` at env.rs:77-79. No ordinary-session relation population found. This is source evidence, not native catalog proof.
- Changed Rust lines manually audited using rust-readability guidance: no violations identified. No metrics/lint commands executed; no warning suppressions added.

## Cases and proof boundary

| Case | Authored behavior | Actual runtime proof |
|---|---|---|
| `mount_item_empty_relation_returns_exactly_one_nil` | Exact one-nil misses for 6948/901001 | Pending; original text unchanged |
| `mount_item_requires_one_numeric_identity` | Missing/nil/nonnumeric required-number errors | Pending; original text unchanged |
| `mount_item_authenticates_secret_input_before_type_or_lookup` | Authentic VM secret/taint permission ordering and preservation | Pending; original text unchanged |
| `supplied_mount_item_mapping_needs_no_catalog_target_or_ownership` | Fresh relation empty, ordinary mounts explicitly cleared, 901001→401 without catalog target | Pending; query and post-query emptiness unproven |
| `supplied_mount_item_mapping_replacement_and_removal_are_live` | 901001→401, replacement→402, removal→nil | Pending; replacement/removal not credited |
| `supplied_mount_item_mapping_does_not_leak_into_independent_fresh_environment` | First mapped, second fresh miss, first remains mapped | Pending; second environment not credited |
| `supplied_mount_item_numeric_identities_are_distinct_without_truncation_aliases` | Six supplied integer targets, fourteen fractional/nonfinite/out-of-range/neighbor misses | Pending; numeric matching/nonalias behavior not credited |

Expected boundary is seven failures at earliest missing-API guards; expectation is NOT observed RED. Even compiled setup and a reached first guard would not prove mapped results, replacement/removal, second environment, or later numeric controls. Initial genuine three-case RED remains separately retained; no old seal read or rehashed.

## Remaining terminal inspection

Authenticate only this epoch's new `integration-sealed` once and immediately persist `independent-auth-integration.json`. Read saved full compile/runtime/fmt outputs and exit records, warning diagnostics, source snapshots, and outcome. Replace pending classifications with actual counts/reached/unreached assertions; do not replay commands. No cargo check now: old default check is not current library-scope evidence after SimState changed.

No source edits, build/test/runtime execution, delegation, or unit mutation performed by verifier.
