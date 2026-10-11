# Method surface baselines

Method-surface comparisons must select the baseline for the current compiled configuration, rather than compare every profile against Retail expectations. The comparison and refresh helpers live in [`tests/method_diff_coverage.rs`](../../tests/method_diff_coverage.rs). See the [comparison documentation](../wow-client-diff/README.md#compiled-configuration-method-baselines) for baseline locations and proof status.

## What it must do

- [ ] Select exactly one baseline using `ACTIVE.cache_subdir`, the highest actually enabled `retail-*` epoch cfg, and the `forbidden-aspects` and `on-update-modes` flags.
- [ ] Use `no-retail-epoch` when no Retail epoch cfg is enabled; do not infer an epoch from metadata or substitute `1207`.
- [ ] Preserve root `docs/wow-client-diff/diff_methods_{extra,missing}.txt` for `retail` / `retail-12-1-0` with both flags enabled. Use `docs/wow-client-diff/method-baselines/<profile>/<epoch>/forbidden-<N>-on-update-<N>/` otherwise, with each flag encoded as `0` or `1`.
- [ ] Keep the two existing exact assertions and method discovery unchanged; ordinary execution compares only the current compiled configuration, not a loop over all profiles or epochs. The default Retail build therefore uses one Retail baseline.
- [ ] Make the ignored refresh helper write only the selected current configuration. Missing baseline data, including older Retail epochs, must fail explicitly without fallback or guessed fixtures.

## How it works

- [Method comparison investigation](../wiki/investigations/method-dispatch-refactor.md#cross-profile-comparisons--2026-10-11-utc)
- [Baseline selection and proof status](../wow-client-diff/README.md#compiled-configuration-method-baselines)

## Implementation inventory

- `tests/method_diff_coverage.rs` — existing exact method comparisons, discovery, and ignored refresh helper; configuration selection work is pending verification.
- `src/client_profile.rs` — active compiled profile identity.
- `docs/wow-client-diff/diff_methods_extra.txt`, `diff_methods_missing.txt` — retained default-configuration comparison fixtures.
- `docs/wow-client-diff/method-baselines/` — non-default configuration fixture location required by this contract.

## Tests asserting this spec

- `tests/method_diff_coverage.rs` — the existing two exact assertions remain the comparison contract; proof of configuration selection and selected-only refresh is pending main-thread verification.

## Known gaps (current cycle)

- [ ] Verify selection, exact assertions, selected-only refresh, and explicit missing-baseline failure against the final implementation.
- [ ] Six current fixtures derive from copied, verified diagnostics; this provenance is not passing assertion evidence. Older Retail epochs still lack current baseline data.

## Out of scope

- Running all baselines on every default test invocation: baseline selection follows the current build only.
- Changing discovery, native reference data, method implementations, model behavior, or receiver semantics: this change selects comparison expectations only.
- Establishing native parity or closing historical failures: historical suite FAIL and parent OPEN remain until separately proven resolved.
