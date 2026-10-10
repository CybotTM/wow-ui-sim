# Historical profile compile-message verification

Verified 2026-10-10. PASS for saved compiler-artifact counts, all eight nonfresh target identities in EACH profile, and claimed rounded elapsed durations.

## Scope and evidence

Input directory: `/home/osso/.local/state/wow-ui-sim/verification/historical-map-ordinary-current/20261010T225705Z`. Read each profile’s entire `compile.stdout` JSON-lines stream, `compile.stderr`, `submission.json`, and `compile-result.json`. Read the supplied handoff correction separately. No builds, tests, operations, delegation, source/binary hashing, or runtime/source/binary admission rechecks. Existing admission PASS is outside this verification.

Counts are compiler-artifact MESSAGE RECORDS, not unique output files, compiler units, CPU samples, or test counts. Timestamp subtraction uses submission.started → compile-result.ended; integer microseconds preserve the saved timestamp precision.

## Per-profile counts and elapsed wall time

| Profile | Artifact records | fresh=true | fresh=false | Raw elapsed seconds | Rounded seconds |
|---|---:|---:|---:|---:|---:|
| wrath | 663 | 655 | 8 | 190.922430 | 190.92 |
| mists | 663 | 655 | 8 | 106.577779 | 106.58 |
| era | 663 | 655 | 8 | 128.349182 | 128.35 |
| anniversary | 663 | 655 | 8 | 142.407461 | 142.41 |
| wowforever | 663 | 655 | 8 | 73.417071 | 73.42 |

Each stream also contains 74 build-script-executed records and one successful build-finished record (738 JSON records total). Saved compile results report exit 0 and no stream errors. Every stderr reports six deprecated iced_wgpu manifest lint-key warnings; this is not a warning-free claim.

## Exact targets and actual features

### wrath

- Start: `2026-10-10T22:57:05.778893+00:00`
- Compile end: `2026-10-10T23:00:16.701323+00:00`
- Recorded revision: `0ed37cd1a9fce247b6740fb6cb1bbbb8e18eafa2`
- Requested features: `sound,gui,casc,client-wrath`
- Actual compiler-artifact features (identical on all eight nonfresh records): `casc`, `client-wrath`, `gui`, `rodio`, `sound`.
- Nonfresh target names/kinds, in saved message order: `wow_ui_sim` (lib), `panel-visual-metrics` (bin), `bench_talents` (bin), `bench_steady_state` (bin), `bench_spellbook` (bin), `wow-sim` (bin), `wow-cli` (bin), `integration` (test).

### mists

- Start: `2026-10-10T23:01:20.501367+00:00`
- Compile end: `2026-10-10T23:03:07.079146+00:00`
- Recorded revision: `0ed37cd1a9fce247b6740fb6cb1bbbb8e18eafa2`
- Requested features: `sound,gui,casc,client-mists`
- Actual compiler-artifact features (identical on all eight nonfresh records): `casc`, `client-mists`, `gui`, `prefork-full-ui`, `rodio`, `sound`.
- Nonfresh target names/kinds, in saved message order: `wow_ui_sim` (lib), `panel-visual-metrics` (bin), `bench_spellbook` (bin), `bench_steady_state` (bin), `bench_talents` (bin), `wow-sim` (bin), `wow-cli` (bin), `integration` (test).

### era

- Start: `2026-10-10T23:06:20.620627+00:00`
- Compile end: `2026-10-10T23:08:28.969809+00:00`
- Recorded revision: `0ed37cd1a9fce247b6740fb6cb1bbbb8e18eafa2`
- Requested features: `sound,gui,casc,client-era`
- Actual compiler-artifact features (identical on all eight nonfresh records): `casc`, `client-era`, `gui`, `rodio`, `sound`.
- Nonfresh target names/kinds, in saved message order: `wow_ui_sim` (lib), `bench_spellbook` (bin), `bench_steady_state` (bin), `panel-visual-metrics` (bin), `bench_talents` (bin), `wow-sim` (bin), `wow-cli` (bin), `integration` (test).

### anniversary

- Start: `2026-10-10T23:08:30.305090+00:00`
- Compile end: `2026-10-10T23:10:52.712551+00:00`
- Recorded revision: `d19b4cf098eba8f75ae2885ec347cb3f9501f315`
- Requested features: `sound,gui,casc,client-anniversary`
- Actual compiler-artifact features (identical on all eight nonfresh records): `casc`, `client-anniversary`, `gui`, `rodio`, `sound`.
- Nonfresh target names/kinds, in saved message order: `wow_ui_sim` (lib), `bench_talents` (bin), `bench_spellbook` (bin), `panel-visual-metrics` (bin), `bench_steady_state` (bin), `wow-sim` (bin), `wow-cli` (bin), `integration` (test).

### wowforever

- Start: `2026-10-10T23:10:53.965812+00:00`
- Compile end: `2026-10-10T23:12:07.382883+00:00`
- Recorded revision: `d19b4cf098eba8f75ae2885ec347cb3f9501f315`
- Requested features: `sound,gui,casc,client-wowforever`
- Actual compiler-artifact features (identical on all eight nonfresh records): `aura-containers`, `aura-instance-enumeration`, `aura-xml-widgets`, `base-spell-relationships`, `casc`, `client-wowforever`, `forbidden-animation-aspects`, `forbidden-aspects`, `gui`, `native-duration-formatting`, `numeric-rule-formatters`, `on-update-modes`, `player-cast-durations`, `rodio`, `sound`, `timed-signal-maps`.
- Nonfresh target names/kinds, in saved message order: `wow_ui_sim` (lib), `panel-visual-metrics` (bin), `bench_spellbook` (bin), `bench_steady_state` (bin), `bench_talents` (bin), `wow-sim` (bin), `wow-cli` (bin), `integration` (test).

## Interpretation and source-difference limits

All five profiles independently contain one library + SIX bins + one integration-test target. The previous “five bins” statement is false. Benchmark-named binaries here are compilation targets, not evidence of benchmark execution.

All submissions record `cargo test --offline --locked --no-default-features --features sound,gui,casc,client-<profile> --test integration --no-run --message-format=json`, with CARGO_BUILD_JOBS=12. Actual emitted features include rodio in every profile, prefork-full-ui additionally in Mists, and the explicitly listed additional capabilities in Forever. These are different feature-selected compilation workloads, not merely different client labels.

Recorded revisions differ: Wrath/Mists/Era use 0ed37cd1a9fce247b6740fb6cb1bbbb8e18eafa2; Anniversary/Forever use d19b4cf098eba8f75ae2885ec347cb3f9501f315. This saved metadata does not establish the exact source diff, equivalence of effective source inputs, or which changes affected compiler cost. No source snapshots or live source were reread, compared, or rehashed. The existing within-submission source admission is not an across-profile source-identity proof.

Elapsed values are historical submission-to-compile-end wall time only, including any overhead inside that interval. Not timing-HTML unit durations, CPU time/attribution, or exclusive library/bin/link costs. Message freshness does not quantify per-target cost or establish matched cache state. No controlled comparison, repeatability measurement, optimization benefit, speedup, or whole-profile runtime/test correctness follows from these records.
