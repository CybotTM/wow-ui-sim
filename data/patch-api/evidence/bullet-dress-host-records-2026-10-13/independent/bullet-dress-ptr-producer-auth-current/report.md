# PASS — bounded PTR compile/copy verification

Epoch: `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z`
Source metadata: `69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c` (not later HEAD).

- Full compile stdout (574600 bytes), stderr (1275 bytes), terminal and all epoch JSON metadata read. 741 JSON records: 665 compiler artifacts, 75 build-script records, 1 final build-finished success=true; zero compiler diagnostics.
- Six iced-wgpu-patched manifest deprecations; not a warning-free build. Cargo reports 6m24s. Saved terminal: exited, success, exit 0; October 13, 2026 19:45:51–19:52:17 CDT; peak 6945943552 bytes. Cargo/rustc 1.99.0 recorded.
- Both receipts absent before hashing. Each new copy read sequentially once for SHA-256; pre/open/post stats match its seal. Receipts written mode 0600, fsynced (file/directory), read back.
- Integration: `be41045bd62326b163fe34292f8cb2c32984b4df16a3b04dec09bd519752b365`.
- wow-sim: `0d70e1017459651a82e5135d966bae6b6a5855062d495e91946e072a8f13b975`.

Receipts: `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/receipt-integration.json`, `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/receipt-wow-sim.json`.
Cargo PTR feature metadata agrees with seals; saved wow-sim artifact association matches the stdout record. This is metadata association, not independent binary-to-source provenance.

Runtime **UNPROVED**. No Cargo reruns, tests, checks, fmt, workloads, service actions, delegation, Bash or cwd changes. No authentication of old binaries, libraries, archives or source trees. External libraries/cache/settings and later fixture edits excluded.
