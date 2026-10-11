# Dev-harness build-input staging — bounded repair

Implementation `fc49b4490`; fixture import/style fixes `4053713c8` and `fda902ac2`. Direct follow-up to the [dev-only extraction](../dev-only-harness-2026-10-11/README.md): Docker and native snapshots omitted the newly declared workspace crate. No Rust runtime, host/profile/toolchain selection, dependency versions or vendor source changed.

- Docker copies `crates/test-harness/` before dependency fetching; the old test-tree COPY is removed from that layer.
- Native source snapshots include `crates/`; helper manifest, build script and library are copied even for binary-only workspace resolution.
- Cargo metadata classifies the helper as a dev-dependency, not a normal/build dependency. Manifest availability is distinct from compiling that crate in production.

## Proof

| Scope | Before | After | Limit |
|---|---|---|---|
| Real-Git snapshot fixture | 1 test, 3 missing-helper-file subtest failures, exit 1 | 1/1 PASS, exit 0 | Seven concrete file outputs; not exhaustive tracked/ignored/symlink coverage |
| Docker prefetch staging | Cargo metadata exit 101: helper manifest missing | Exit 0, helper present, test directory absent | No container build, dependency fetch or production compilation |
| Actual native snapshot | Same missing-manifest exit 101 | Exit 0, helper present; 4,047 common files byte-identical, only three helper additions | Local snapshot/metadata, not remote transport/native-host build |
| Format/lint/syntax | Ruff executable missing initially; then two I001 lint failures | Cached offline formatter exit 0, final lint exit 0, Python syntax checks exit 0 | Failure receipts preserved; no warning suppression |
| Final focused fixture | Import-only fixes followed prior GREEN | At `fda902ac2`, 1/1 PASS, nine current input seals unchanged | Current endpoint proof, not retroactive capture provenance |

Metadata command: `cargo metadata --offline --locked --no-deps --format-version 1`. `resolve=null`; success establishes workspace manifest loading, not full dependency resolution/build-script execution. Native/Docker target inventories differ and are not test-execution counts.

[Independent artifact audit](independent-report.md) accepts bounded saved results and current wiring. Historical custody is partial: original metadata/fixture receipts lack capture-time source revisions/hashes, and Docker RED/GREEN share one mutable staging directory. Native RED/GREEN directories remain separate. Current source comparisons are not historical seals. The later `snapshot-final.json` provides an explicit current revision/time/nine-input seal; it does not rewrite earlier receipts.

[Retention manifest](retention-manifest.json): 51 exact text copies, 231,950 bytes; mutable staging directories excluded, private originals unchanged. Independent audit predates the final focused fixture and tracked copy gate; no broader acceptance follows.

The one asynchronous Rust full suite at `c75ccfe02` covers unchanged Rust scope, not this Python/Docker staging follow-up. No repeat Rust suite/check, Docker publication, deployment or native-host operation. Original Mists RED, authentic Garrison catalog, blocked source lanes and native/full-handoff acceptance remain OPEN.

Related: [native workflow](../../../../docs/remote-builds.md), [harness contract](../../../../docs/specs/prefork-test-harness.md).
