# Independent artifact verification — 2026-10-11

**PASS — bounded cache-isolation evidence only. No parent-goal closure.**

Read/followed `/home/osso/AgentConfig/skills/verify/SKILL.md`, artifact mode adapted to existing trial artifacts. No reruns, builds, operations, delegation, Bash, rg or fd. Complete stdout parsed (every JSON record), complete stderr and all submission/admission/treatment/result/outcome files inspected. JSON companion preserves exact stderr, target records, generated-output snapshots, cgroup counters and source artifact SHA256 manifest.

## Exact counts

| Pair / phase | stdout records | artifact records | fresh | nonfresh | exit | root lib + six bins |
|---|---:|---:|---:|---:|---:|---|
| RED warm | 738 | 663 | 663 | 0 | 0 | 7/7 fresh |
| RED timestamp treatment | 738 | 663 | 655 | 8 | 0 | 0/7 fresh |
| Post warm | 741 | 665 | 665 | 0 | 0 | 7/7 fresh |
| Post timestamp treatment | 741 | 665 | 663 | 2 | 0 | 7/7 fresh |
| Post preparation after Mists — excluded | 741 | 665 | 665 | 0 | 0 | 7/7 fresh |

All five streams have exactly one successful build-finished record, zero compiler-message records. RED streams each have 74 build-script-executed records; post streams each have 75. These messages include cached build-script outputs and do not individually establish execution. Each stderr has 6 manifest deprecation warnings plus one summary line; not warning-clean.

Post two nonfresh compiler-artifact records: `wow_ui_sim_test_harness` library and `integration` test. Helper build-script compiled artifact remains fresh; its **RunCustomBuild** fingerprint is dirty. RED eight nonfresh: `wow_ui_sim`, `integration`, `bench_spellbook`, `bench_steady_state`, `bench_talents`, `panel-visual-metrics`, `wow-cli`, `wow-sim`. Target features match within each pair. These counts are recomputed from complete stdout, not accepted from outcome booleans.

## Submission / fingerprints / treatment

Identical argv and environment overrides for both submissions:

```text
/usr/bin/cargo test --offline --locked --test integration --no-run --message-format=json
CARGO_BUILD_JOBS=12
CARGO_LOG=cargo::compiler::fingerprint=info,cargo::core::compiler::fingerprint=info
CARGO_NET_OFFLINE=true
```

Default retail feature set; no no-default-features/profile override. RED revision `9cc19496885f027be02a0f2f3db36e5065ffdda4`; post revision `f0e5f64acb81746ac16627d3bb2b589212f1156c`. Dependency/cache artifact counts differ: 663 vs 665. No identical-revision speedup claim.

Both treatments touch only the timestamp of `tests/achievements_api.rs`, with unchanged content recorded. Original mtime: `1791693772266407141`; RED treatment: `1791694473604792400`; post treatment: `1791696330448011461`.

RED stderr contains 9 dirty fingerprint records: root RunCustomBuild, root lib, six bins, integration. Trigger: watched `/tests` newer than root build-script output. Post stderr contains 3: helper RunCustomBuild, helper lib, integration; watched `/tests` newer than helper output. Post root lib/six-bin dirty fingerprints absent, and their 7/7 fresh flags corroborate isolation. Warm/preparation stderr has zero dirty records.

## Outputs / restoration

Post generated outputs: **8/8 native files** (7 objects + archive) retain recorded SHA256, byte size and mtime; **2/2 helper-generated Rust files** retain SHA256 and byte size but change mtime. `integration_tests.rs`: 240920 bytes; `prefork_full_ui_cases.rs`: 667061 bytes. Root output directory is `wow-ui-sim-e80f468d388abb3a`; generated Rust output is in `wow-ui-sim-test-harness-a0dc779095698bb1`. No whole-native-system or profile conclusion follows.

RED generated outputs: **10/10 change mtime**, including 8 native files and 2 Rust files. **8/8 native byte sizes match**, but native hashes were not recorded; byte equality cannot be inferred from size. **2/2 Rust hashes match**; their byte sizes were not recorded. Root output directory is `wow-ui-sim-d538b71804b3645e`.

Restoration: post **11/11** logged before/after source hashes equal; RED **6/6** equal. Outcome warm/treatment copies exactly match standalone result JSONs. Both outcomes record mtime restored. Independent current read of touched file confirms original mtime `1791693772266407141` and SHA256 `df27ff8751b1cbe6143c47f10bde8f96db22f932e890f63b4e95d665fc74a4c7`, matching both before hashes. Historical restoration instant is not separately timestamped; not whole-repository byte verification.

## Cgroup / CPU arithmetic

Separate logged worker cgroups: `wow-test-watch-reasons-20261011t045433z.service` and `wow-dev-cache-20261011t052529z.service`, both under user `agents.slice`; submission cpu.stat paths match those cgroups. All **5/5** admissions succeeded at attempt 1 with no relevant Cargo process listed. RED load1 2.85; post 3.62. Logged counters are evidence, not live membership/quota audit.

| Phase | usage_usec delta | CPU seconds | wall seconds | CPU/wall | nr_throttled delta | throttled_usec delta |
|---|---:|---:|---:|---:|---:|---:|
| RED warm | 281825 | 0.281825 | 0.39689681500021834 | 0.7100712057864333 | 0 | 0 |
| RED treatment | 82580194 | 82.580194 | 33.4831080820004 | 2.4663240281565395 | 27 | 6656321 |
| Post warm | 227247 | 0.227247 | 0.21637955200276338 | 1.0502240063658965 | 0 | 0 |
| Post treatment | 14173933 | 14.173933 | 14.475307194006746 | 0.9791801175638245 | 0 | 0 |

Recomputed `(after.usage_usec - before.usage_usec)/1e6` and CPU seconds / recorded wall agree with supplied figures. Whole worker-cgroup CPU includes launcher/Cargo/rustc/linker overhead; not instantaneous busy-core saturation or per-bin attribution. RED throttling and distinct revisions/cache graphs prohibit causal benchmark extrapolation. Preparation inspected, never included in this timing comparison.

## Boundary

Evidence proves this one default integration no-run timestamp pair keeps post root lib + six bins fresh while rebuilding helper lib + integration test. No test execution, runtime behavior, whole-profile/native compatibility, full-suite, deployment, saturation or parent-goal completion claimed. Toolchain versions absent from supplied artifacts.

Excluded preparation CPU arithmetic also matches: delta 261806 usec; 0.261806 CPU seconds / 0.5343670389993349 wall seconds = 0.4899366556931777.
