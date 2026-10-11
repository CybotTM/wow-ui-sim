# Unit2 compile numeric independent verification

**PASS** — parsed all seven existing raw Cargo JSON streams and cgroup snapshot pairs. Numeric-only artifact verification; no functional gate repeated.

Source: `/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/20261010T235452Z`.
Compared: `handoff/unit2-compile-cost-main-correction.json` and `handoff/unit2-cgroup-all-phases-current.md` under `/home/osso/.local/state/wow-ui-sim`.

## Results

| Profile | Artifacts / fresh / nonfresh | Submission-to-end wall s | Cgroup CPU delta s | Coarse avg cores | Throttle count delta | Throttle delta s | End current GiB | Cumulative peak GiB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| retail | 663 / 655 / 8 | 69.265395 | 407.066217 | 5.88 | 77 | 20.136978 | 6.49 | 10.59 |
| ptr | 663 / 655 / 8 | 74.729750 | 480.055965 | 6.42 | 90 | 16.057025 | 12.59 | 16.00 |
| wrath | 663 / 655 / 8 | 68.457247 | 437.143125 | 6.39 | 61 | 11.547135 | 8.58 | 16.00 |
| mists | 663 / 655 / 8 | 67.172273 | 417.372480 | 6.21 | 54 | 12.363961 | 7.17 | 16.00 |
| era | 663 / 655 / 8 | 76.515767 | 493.874389 | 6.45 | 74 | 9.482109 | 3.56 | 16.00 |
| anniversary | 663 / 655 / 8 | 91.396864 | 520.703790 | 5.70 | 71 | 14.972853 | 4.20 | 16.00 |
| wowforever | 663 / 655 / 8 | 104.899245 | 668.318357 | 6.37 | 50 | 9.076555 | 5.15 | 16.00 |

All seven: exactly one library (`wow_ui_sim`), six bins (`bench_spellbook`, `bench_steady_state`, `bench_talents`, `panel-visual-metrics`, `wow-cli`, `wow-sim`), and one integration test target (`integration`) nonfresh. `panel-visual-metrics` is nonfresh in **7/7**, not retail-only; the retail-only claim is false.

Correction JSON counts, exact target names, feature lists, revisions, and six-decimal wall values match raw evidence. Markdown timestamps, revisions, result metadata, CPU deltas, coarse averages, throttle deltas, current memory, and cumulative peaks match at displayed precision. Recorded compile result metadata: 7/7 exit 0, source_equal=true, stream_errors=[]. These are existing recorded results, not new build or functional verification.

Before/after membership and limits match within every phase. All snapshots: cpu.max `1200000 100000` = 12 CPU quota; memory.max `17179869184` = 16 GiB.

## Raw arithmetic evidence

- **retail** revision `cc4f4c782f66c955e635d857bfc85a79a021ca24`; start `2026-10-10T23:55:04.267595+00:00`, end `2026-10-10T23:56:13.532990+00:00`; usage_usec `10748082` → `417814299` (delta `407066217`); memory.current end `6972710912` bytes; memory.peak `1389862912` → `11376263168` bytes.
- **ptr** revision `cc4f4c782f66c955e635d857bfc85a79a021ca24`; start `2026-10-10T23:56:57.412003+00:00`, end `2026-10-10T23:58:12.141753+00:00`; usage_usec `507060290` → `987116255` (delta `480055965`); memory.current end `13517881344` bytes; memory.peak `17179869184` → `17179987968` bytes.
- **wrath** revision `24c727075a69a8a4bbf813156248e0547f3150f9`; start `2026-10-10T23:58:53.366061+00:00`, end `2026-10-11T00:00:01.823308+00:00`; usage_usec `1057098781` → `1494241906` (delta `437143125`); memory.current end `9212018688` bytes; memory.peak `17179987968` → `17179987968` bytes.
- **mists** revision `24c727075a69a8a4bbf813156248e0547f3150f9`; start `2026-10-11T00:00:04.730519+00:00`, end `2026-10-11T00:01:11.902792+00:00`; usage_usec `1497124963` → `1914497443` (delta `417372480`); memory.current end `7695933440` bytes; memory.peak `17179987968` → `17179987968` bytes.
- **era** revision `24c727075a69a8a4bbf813156248e0547f3150f9`; start `2026-10-11T00:01:14.999085+00:00`, end `2026-10-11T00:02:31.514852+00:00`; usage_usec `1917453126` → `2411327515` (delta `493874389`); memory.current end `3824140288` bytes; memory.peak `17179987968` → `17179987968` bytes.
- **anniversary** revision `24c727075a69a8a4bbf813156248e0547f3150f9`; start `2026-10-11T00:02:35.133994+00:00`, end `2026-10-11T00:04:06.530858+00:00`; usage_usec `2414833341` → `2935537131` (delta `520703790`); memory.current end `4510162944` bytes; memory.peak `17179987968` → `17179987968` bytes.
- **wowforever** revision `24c727075a69a8a4bbf813156248e0547f3150f9`; start `2026-10-11T00:04:09.708496+00:00`, end `2026-10-11T00:05:54.607741+00:00`; usage_usec `2938560124` → `3606878481` (delta `668318357`); memory.current end `5527805952` bytes; memory.peak `17179987968` → `17179987968` bytes.

Wall = compile-result.ended − submission.started (including submission overhead). CPU seconds = (after.usage_usec − before.usage_usec)/1,000,000. Coarse average cores = CPU seconds / wall seconds. Snapshot intervals and submission intervals are not asserted to have identical boundaries.

Memory peak is a cumulative shared-cgroup high-water mark, **not an independent per-phase peak**; memory.current is point-in-time. Aggregate cgroup usage is not exclusive compiler CPU. No frontend attribution, speedup, saturation, or saturation-cause conclusion is established. Revisions differ between retail/PTR and the other five profiles; no same-revision comparative benchmark claimed.

No binary hashing, builds, tests, replay, operational mutations, tracked edits, delegation, Bash, rg, or fd performed. Only this private report written.
