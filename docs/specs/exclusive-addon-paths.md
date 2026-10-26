# Exclusive addon paths

`wow-sim` can limit third-party addon discovery to explicit roots. Selection lives in `src/paths.rs` and the CLI loader. See the [CLI guide](../../README.md#exclusive-addon-test-runs) for usage.

## What it must do

- [ ] Repeated `--addons-path DIR` arguments replace all default third-party candidates, in argument order.
- [ ] With no CLI roots, `WOW_SIM_ADDONS_PATH_EXCLUSIVE=1` (or `true`) restricts discovery to `WOW_SIM_ADDONS_PATH`. An unset or empty path selects no third-party roots, not default candidates.
- [ ] With neither exclusive option, discovery retains its existing ordered union of candidates. False/zero does not enable exclusive mode.
- [ ] Existing roots are deduplicated by canonical path; nonexistent roots do not trigger default discovery in exclusive mode.
- [ ] Exclusive selection also limits explicit `C_AddOns.LoadAddOn` and test-file lookup. Ambient `Interface/TestAddOns` is not searched in exclusive mode.
- [ ] Blizzard UI still uses its separate profile cache, and `run-tests` still resolves bundled TestFramework without exposing other bundled addons.

## How it works

- [Addon loading pipeline](../addon-loading-pipeline.md)
- [CLI guide](../../README.md#exclusive-addon-test-runs)

## Implementation inventory

- `src/paths.rs` — path selection and bundled framework location.
- `src/bin/wow_sim/main.rs` — repeatable CLI argument and runtime root registration.
- `src/bin/wow_sim/startup.rs` — selected roots passed to third-party discovery.
- `src/bin/wow_sim/addon_loading.rs` — scan only selected third-party roots.
- `src/addon_tests.rs` — framework and test-file resolution.

## Tests asserting this spec

- `src/bin/wow_sim/exclusive_path_tests.rs` — path selection, CLI parsing and loader isolation.
- `src/paths.rs` — path-resolution units.

## Known gaps (current cycle)

- [ ] RED/GREEN and seven-profile checks pending.

## Out of scope

- Changing default discovery or `--no-addons` policy.
- Sandboxing addon Lua, SavedVariables, or other filesystem access. Exclusive discovery is not a security boundary.
- API producer changes or changes to external addon test scripts.
