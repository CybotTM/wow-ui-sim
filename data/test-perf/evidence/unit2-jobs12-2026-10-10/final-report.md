# Already-required unit2 builds — jobs12 cost/utilization

[Numeric verification](independent-report.md) admits seven saved compile phases: each663artifact records/655fresh/8nonfresh, library+sixbins (including panel-visual-metrics)+integration. Benchmark-named binaries were compiled, not benchmarked. [Exact summary](summary.json), [hashes](hash-manifest.json).

| Profile | Submission-to-end wall seconds | Coarse cgroup CPU equivalents |
|---|---:|---:|
| Retail | 69.265395 | 5.88 |
| PTR | 74.729750 | 6.42 |
| Wrath | 68.457247 | 6.39 |
| Mists | 67.172273 | 6.21 |
| Era | 76.515767 | 6.45 |
| Anniversary | 91.396864 | 5.70 |
| Forever | 104.899245 | 6.37 |

Compile intervals sum552.436541s (9m12.44s), excluding separate checks, runtime, artifact hashing and between-phase gaps. Different features/caches; Retail/PTR submissioncc and five others docs-only24, same captured source blobs, not one whole-tree revision or controlled comparison.

Actual compile-window cgroup CPU usage divided by wall time yields5.70–6.45 logical CPU equivalents under recorded12-CPU quota; includes compiler and worker stream handling, not exclusive rustc/frontend/codegen attribution. Throttling counters observed, no saturation-cause or speedup inference. Memory limit16GiB; unit's cumulative peak reaches about16GiB after PTR, not separate per-profile peaks or evidence of an OOM. No extra benchmark/rebuild or resource-limit changes.
