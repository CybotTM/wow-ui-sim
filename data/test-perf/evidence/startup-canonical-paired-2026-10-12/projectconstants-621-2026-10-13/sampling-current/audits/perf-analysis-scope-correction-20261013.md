# Perf analysis scope correction — 2026-10-13

Original offline report remains unchanged at `<PRIVATE_VERIFICATION_ROOT>/perf-cache-compile-sampling-621-current/report.md`.

- `load_pack_bytes`0.52% self is a visible leaf share, not an upper bound on inclusive first-cache initialization. Source puts lazy pack read/validation/indexing and cache replay inside the per-file compile timer. Do not adopt the original report's VM-over-cache focus as an established hottest-cost diagnosis.
- Complete script accounting:6,715 blocks, weighted period6,721,721,715;5,032 blocks have no recovered frames, median depth0;6,208 unresolved leaves (92.4497%). Named category counts may overlap and missing symbols do not prove non-execution. Full counts are in `<PRIVATE_VERIFICATION_ROOT>/perf-cache-compile-chain-counts-current/report.json`.
- ProjectConstants logged696.2ms total/696.0ms compile-cache/54.5µs call in the sampled run. Perf timestamps and untimestamped addon lines do not establish an exact addon-phase window; no per-addon-exclusive CPU attribution.
- Actual kernel is7.2.9-arch1-1 (`uname -r`, also recorded perf metadata).7.2.10-1 is the installed perf version, not an independently supplied kernel version. Original report's contrary host-note wording is a version-confusion erratum.
- Runtime sampling succeeded; zero reported lost records. Addr2line errors and unresolved callchains remain an analysis blocker, not proof of zero cache cost. No warning suppression, new sampling, reauthentication, source/runtime fix or kernel changes performed by this correction.

Userspace CPU, kernel/off-CPU, elapsed loader buckets, source/native parity and frozen CCD repair remain separate. Original exact heaviest-addon1.42s assertion FAIL is preserved; no budget inflation.
