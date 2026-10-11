# Independent bounded verification — 2026-10-11

**Verdict: PASS for saved snapshot preservation, Cargo manifest-resolution controls, and final Python style/syntax evidence. Historical source/revision custody is PARTIAL. Actual Docker-image and native-host builds: NOT EXECUTED by this verification. No runtime/profile/native acceptance claim.**

Requested follow-up: `fc49b4490`, import/style fixes `4053713c8` / `fda902ac2`. These identifiers are caller-supplied; lint and pycompile receipts explicitly record them. No Git command or historical commit/tree inspection was performed.

## Method and scope

Read `/home/osso/AgentConfig/skills/verify/SKILL.md`; acted as the independent artifact verifier, without delegation. Read saved receipt JSON and complete small stdout/stderr files; loaded and parsed both complete Cargo metadata stdout JSON documents (38,613 / 37,272 bytes). Inspected current Dockerfile, Python wrapper, behavioral fixture, workflow documentation, manifests, and relevant helper implementation. Compared retained inputs using read-only file enumeration and byte/hash inspection. No builds, tests, lint, pycompile, Cargo, Git, Docker, remote/native execution, deployment, or operational reruns. Only the requested report files were written. One file-search tool attempt returned `fd is not available on PATH`; no file-search result or verification credit was obtained.

## Saved proof ledger

All paths below are relative to this report's directory. Each named receipt has accompanying `.stdout` / `.stderr`, preserved unchanged.

| Evidence | Exact saved result | Admission |
| --- | --- | --- |
| `snapshot-red.json` | Exit 1; `Ran 1 test in 0.008s`; `FAILED (failures=3)` | Missing `crates/test-harness/Cargo.toml`, `build.rs`, `src/lib.rs`; three subtest failures, not three independent tests. |
| `snapshot-green.json` | Exit 0; `Ran 1 test in 0.006s`; `OK` | One real-Git temporary-repository snapshot test, 1/1 PASS. |
| `native-red.json` | Cargo metadata exit 101; stdout empty | Missing helper manifest; complete causal stderr retained. |
| `native-green.json` | Cargo metadata exit 0; stderr empty | Five workspace packages, including helper; no compile/host execution. |
| `docker-red.json` | Cargo metadata exit 101; stdout empty | Same missing helper-manifest boundary in staged prefetch inputs. |
| `docker-green.json` | Cargo metadata exit 0; stderr empty; tests directory absent | Corrected prefetch input control, not container/dependency-fetch success. |
| `format-unavailable.json` | `ExecutableMissing before launch; no formatting proof` | Initial unavailable formatter is not a pass. |
| `format.json` | Offline uvx Ruff format exit 0; `2 files left unchanged` | Wrapper and fixture formatting. |
| `format-import-fix.json`, `format-import-spacing.json` | Each exit 0; `1 file left unchanged` | Subsequent fixture formatting. |
| `lint.json` (`fc49b4490`) | Exit 1; `Found 1 error.` | I001 import order at `tools/test_build_host.py:3:1`. |
| `lint-green.json` (`4053713c8`) | **Exit 1**; `Found 1 error.` | Despite filename, still FAIL: I001 import-block spacing at the same location. |
| `lint-final.json` (`fda902ac2`) | Exit 0; `All checks passed!`; stderr empty | Final wrapper + fixture lint PASS. |
| `pycompile.json` (`fc49b4490`) | Exit 0; both streams empty | Wrapper + fixture syntax proof. |
| `pycompile-import-fix.json` (`4053713c8`) | Exit 0; both streams empty | Fixture-only syntax proof. |
| `pycompile-final.json` (`fda902ac2`) | Exit 0; both streams empty | Final fixture syntax proof; prior wrapper syntax credit retained, not rerun. |

Both metadata controls used exactly `/usr/bin/cargo metadata --offline --locked --no-deps --format-version 1`. The full green documents have `version=1`, `resolve=null`, and five workspace members matching their receipt arrays. Root metadata contains 60 dependency declarations and 33 features; the helper declaration is `kind=dev`, not a normal/build dependency. Root target records differ (native 43, Docker 39); this is not test execution or identical target-discovery proof. `--no-deps` metadata does not establish full dependency resolution, fetching, compilation, or build-script execution.

The red Cargo stderr in both lanes ends with:

```
failed to load manifest for dependency `wow-ui-sim-test-harness`
failed to read `<stage>/crates/test-harness/Cargo.toml`
No such file or directory (os error 2)
```

This excerpt abbreviates stage paths; complete verbatim stderr remains in the original files and JSON report.

## Artifact/wiring checks

- **[EXIST] PASS:** `Dockerfile` (98 lines), `scripts/build-host.py` (365), `tools/test_build_host.py` (61), `docs/remote-builds.md` (125), and all three helper inputs exist. Current content hashes are recorded in the JSON report.
- **[SUBSTANTIVE] PASS:** Wrapper `snapshot(root, context)` selects Git-visible sources, validates them, and copies actual bytes; no stub. Fixture initializes a real temporary Git repository, writes seven concrete input files, calls the actual snapshot function, and verifies every copied file's presence and exact contents. It does not assert Python/Docker source substrings or internal method calls as behavior. Snapshot files are the external contract. Fixture inputs are untracked Git-visible files; this single test is not exhaustive tracked/ignored/symlink coverage.
- **[WIRED] PASS:** `scripts/build-host.py:22-24` includes `crates` in `SOURCE_DIRS`; lines 190-216 consume that selection and copy inputs. `build()` calls `snapshot()` at line 251 before `native.execute()` at 259. Dockerfile:35 copies `crates/test-harness/` before `cargo fetch --locked` at 38; binary build remains at 50. `Cargo.toml:12` lists the helper workspace member; its dev-dependency declaration supplies the observed metadata contract. `docs/remote-builds.md:35` explains why binary-only Cargo manifest loading still needs the internal helper. This is wiring inspection supported by behavioral output, not a new source-shape test.
- **[ANTI-PATTERN] PASS for inspected follow-up files:** Each of the four files above has zero TODO/FIXME/HACK/XXX/`except: pass` markers; no empty catches, placeholder bodies, or commented-out implementation blocks observed. Existing unrelated behavior was not refactored.

Native retained snapshots contain 4,047 RED files and 4,050 GREEN files. All 4,047 common files are byte-identical; no RED-only files. The only additions are the helper's manifest, build script, and library source. This supports unchanged common Rust/runtime inputs at this saved boundary; adding copied helper Rust files is not a source implementation change.

Current root manifest, lockfile, and Cargo configuration match both native snapshots and the Docker GREEN stage byte-for-byte. The three helper inputs in native GREEN and Docker GREEN also match current source. Additional copied-directory comparisons yield zero byte mismatches: native `iced-dynamic` 2 files, patched iced 41, xtask 2, helper 3; Docker respective counts 2/43/2/3. Native retains `tests/` and `tests/perf/`; Docker GREEN retains neither. Docker GREEN top-level inputs are exactly `.cargo`, `Cargo.toml`, `Cargo.lock`, `crates`, `iced-dynamic`, `iced-wgpu-patched`, `xtask`, matching the inspected prefetch COPY contract. `.dockerignore` does not exclude the helper.

## Custody and remaining limits

1. Snapshot and Cargo receipts do not record source revision, invocation timestamp, wrapper/Dockerfile before-and-after hashes, or capture-time seals. Byte comparisons and hashes in this report are **current inspection seals**, not retroactive provenance. The actual-wrapper origin is described by the native receipts; an independently sealed wrapper invocation/source-at-capture record is absent. Snapshot GREEN predates the import/style follow-up by the supplied task narrative, but its receipt does not bind that chronology/revision independently.
2. Native RED/GREEN directories remain available and differ only by the three helper files. Docker RED/GREEN share one mutable staging path: current directory represents GREEN; original RED staged contents cannot be independently reconstructed from a preserved separate tree. RED receipt records a `tests` input; GREEN receipt and current tree record no tests directory. No before/after image or stage seals were supplied.
3. Formatting/lint/syntax receipts admit their exact saved results, not a fresh current-tree command pass. Recorded revisions exist only in selected receipts; current source hashes are not Git tree seals. No behavior-test rerun after import-only fixes was performed or implied.
4. Actual Docker image build, `cargo fetch`, native-host build, runtime, transport, profile closure, and deployment remain unproved by this bounded packet. No new requirement or acceptance gate is introduced. Metadata success without tests does not prove helper generation or binary compilation without tests.
5. Caller identifies active full suite `c75`; it is not wrapper/Docker follow-up coverage and was not inspected, awaited, or rerun. Common retained Rust/runtime inputs are unchanged; no repeated Rust suite/check was needed for this artifact task. Historical/full-suite/native/profile gaps remain open.

Machine-readable receipts, hashes, exact small streams, and evidence limitations: `independent-report.json`.
