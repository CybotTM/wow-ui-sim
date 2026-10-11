# Shared method-fixture comparison: profile evidence

Main capture `20261011T031711Z`: **12 actual cases, 1 PASS / 11 FAIL**, two original bare comparator assertions per PTR/Wrath/Mists/Era/Anniversary/Forever build. All profiles admitted; no rebuild or Blizzard UI loading/source acquisition. Canonical Retail snapshot files and frozen discovery unchanged. Independent audit pending.

| Simulator build | Complete extra rows | Complete missing rows | Extra assertion | Missing assertion |
|---|---:|---:|---|---|
| Retail, prior separate proof | 12,380 | 33 | PASS, fresh030409 | PASS, reused030002 |
| PTR | 12,412 | 33 | FAIL101 | PASS0 |
| Wrath, Mists, Era, Anniversary — each | 11,633 | 38 | FAIL101 | FAIL101 |
| Forever | 11,825 | 38 | FAIL101 | FAIL101 |

Retail prior proof is **not** part of these 12 new executions. Complete profile comparison files are reconstructed from entire emitted mismatch sets (or exact fixture bytes for the passing assertion); original stdout/stderr and full deltas retained. They are diagnostics, not newly selected canonical fixtures or native API catalogs.

These failures establish disagreement with one shared default-Retail expected result. They do not establish 11 native/profile implementation defects. Native discovery epoch/build/product and receiver semantics remain unknown. PTR adds two published rounding method names across 16 compared types; invocation/rendering/native parity is separate. Three parked source-input lanes remain KEEP BLOCKED. Historical full suite FAIL and parent OPEN unchanged.
