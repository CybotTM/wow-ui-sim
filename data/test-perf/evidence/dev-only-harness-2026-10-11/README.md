# Dev-only automatic test harness — bounded proof

Implementation: helper `ab95adb95`, wiring `c14ccbc4c`, lock `256e54ef1`. Source-only follow-ups `f0e5f64ac`/`a3d6de1ac` change documentation, not the nine sealed compile inputs. Automatic discovery moves into unpublished workspace crate `crates/test-harness`, used as a root dev-dependency. Production build script retains locked-Rilua identity, native ICU and Windows resources. Explicit workspace builds may select the helper; Cargo.lock still has a root watch.

## Coverage

| Evidence | Handled | Missing / boundary |
|---|---|---|
| Generated source | Original integration/prefork inner bodies identical apart from exported macro envelopes, banner and framing newline | Not whole-file identity; discovery of a newly added input not executed |
| Retail listings | Integration 10,718→10,718; prefork 2,325→2,325; exact identity sets | Listings are not execution of all bodies |
| Mists listings | Integration 8,199→8,200 only for prior reward test `069f1cfbd`; prefork 3→3 exact | Original Mists cast RED remains; listed bodies not executed here |
| Selected cases | Retail 8/8, Mists 5/5 | Selected conformance/reward/metatable checks only |
| Original Retail bodies | Nested generated startup and manual WorldMap case 2/2 through ordinary prefork runner | Not all markers/groups/profiles; prerequisite-conformance output is discarded |
| Checks | Format exit 0; default `cargo check --offline --locked --tests` exit 0 | Six manifest deprecations and seven test-helper dead-code diagnostics; no separate Mists check |
| Seals | Nine build/control inputs unchanged; cache trial 11 inputs unchanged and test mtime restored | Not whole-source/dependency/native provenance |

Old local prefork binaries were first copied/sealed for listing, with Cargo fingerprint features. This is not reconstructed old whole-source provenance. Original-run metadata uses a wrong relative `artifact_receipt` pointer; the actual receipt is `compile-controls/retail/artifacts.json`, matched by absolute executable argv. Original private metadata is preserved, not repaired. Binary metadata/seals are retained as text; executables are not tracked.

## Default test-timestamp cache trial

Same argv/environment within both pairs: `cargo test --offline --locked --test integration --no-run --message-format=json`, jobs12, offline, Cargo fingerprint logging. Preparation after Mists/check work is excluded from comparison.

| Pair | Warm fresh | Treatment fresh / nonfresh | Root library + six binaries in treatment |
|---|---:|---:|---:|
| Before extraction, `9cc194968` | 663/663 | 655 / 8 | 0/7 fresh |
| After extraction, `f0e5f64ac` | 665/665 | 663 / 2 | 7/7 fresh |

Post two nonfresh artifacts are helper library and integration test. Helper RunCustomBuild is dirty while its compiled build-script artifact remains fresh. Native output snapshots retain hashes/sizes/mtimes 8/8; helper-generated Rust retains hashes/sizes 2/2 with changed mtimes. Touched test content and original timestamp are restored.

Observed treatment windows: before 33.483108 s wall / 82.580194 CPU-s (~2.4663 coarse CPU equivalents); after 14.475307 s / 14.173933 CPU-s (~0.9792). Different revisions/artifact graphs and before-trial throttling prohibit normalized speedup or causal benchmark extrapolation. Whole worker-cgroup CPU includes launcher/Cargo/rustc/linker, not instantaneous busy cores or per-binary attribution. Jobs12/quota12 are ceilings, not measured saturation.

## Retention and acceptance

Text receipts will be retained in `compile-controls/`, `cache-before/`, `cache-after/`, `original-cases/`, and `reports/`, with an exact copy/hash manifest. Private original receipts remain unchanged. Pending retention verification does not invalidate already accepted original proof; it does prevent claiming the tracked packet complete.

Source, selected simulator execution and local cache isolation have separate bounded proof. Desktop/native acceptance, original Mists hidden-startup behavior, Garrison authentic catalog, source-blocked Wrath/Era/Anniversary, historical full-suite failures and broader handoff remain OPEN. No provider/model change, vendor/cache acquisition, binary deployment or push.

Related: [contract](../../../../docs/specs/prefork-test-harness.md), [build brief](../../../../docs/rust-integration-test-target-size-brief.md), [native workflow](../../../../docs/remote-builds.md).
