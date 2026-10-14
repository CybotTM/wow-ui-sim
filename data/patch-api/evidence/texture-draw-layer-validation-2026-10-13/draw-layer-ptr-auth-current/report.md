# NEW PTR sealed-copy integrity gate — PASS

Epoch: `<PRIVATE_V>/widget-discrete-progress-current/draw-layer-ptr-green-20261014T041112962744Z`

- NEW integration: PASS; one sequential SHA-256 read; digest matches seal; pre/post device, inode, size, mtime match seal.
- NEW wow-sim: PASS; one sequential SHA-256 read; digest matches seal; pre/post device, inode, size, mtime match seal.
- Normal artifact association: metadata only. Compiler full features match recorded fingerprint and seals; exactly one recorded deps candidate newly written after admission. Schema: compiler_artifact / profile_unique_deps_path / fingerprint. Root compiler alias overwritten later by Retail prefork (caller-provided context); no current alias stat inspected or matched.
- Complete existing compile JSON: 741 records, terminal build-finished success, zero compiler-message records; six manifest deprecation warnings. Recorded terminal Result=success, ExecMainStatus=0. No build or workload executed by gate.

Source label: `1b707b28b16eb291e36180b2a12ac2f0411264af`; profile: `ptr`. Metadata labels are not source/binary authentication.

Receipts exclusively created in exact epoch: `receipt-integration.json`, `receipt-wow-sim.json`. All four delivered files mode 0600, fsynced and read back; parent directories fsynced.

No Rust/runtime GREEN, native, external library, archive, source-tree, or source/binary provenance authentication claim. No old executable/library/archive/source-tree hashes; no workloads, builds, service operations, canonical edits, or delegation.
