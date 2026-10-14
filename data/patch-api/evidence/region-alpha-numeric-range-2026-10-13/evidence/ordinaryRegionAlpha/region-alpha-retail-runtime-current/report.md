# Region alpha Retail runtime audit — partial GREEN

**Classification: partialGREEN, not completion.** Existing artifacts only; no test/build/startup reruns.

## Epoch and provenance

Epoch: `<PRIVATE_V>/widget-discrete-progress-current/region-alpha-retail-green-20261014T032102641185Z`.
Recorded source: `71127a8e164c9ebeeee3439251984d08402483dc`; Retail profile. Existing integration and wow-sim receipts accepted. Current device/inode/size/mtime metadata matches both recorded receipts and seals. No new binary, library, or source-archive hash reads. Recorded hashes are prior evidence, not fresh cryptographic authentication. External libraries, caches, settings and source archive remain unauthenticated.

Artifact UTC `2026-10-14 03:21–03:30` corresponds to local `2026-10-13 22:21–22:30 CDT`; no date conflict.

## Concrete coverage

| Claim | Evidence | Classification |
|---|---|---|
| Valid numeric alpha | Exact new Rust case PASS; `REGION_ALPHA_CONTRACT_COMPLETE\tvalid` once | PASS, targeted Retail |
| Invalid numeric alpha, no mutation | Exact new Rust case PASS; `REGION_ALPHA_CONTRACT_COMPLETE\tinvalid` once | PASS, targeted Retail |
| Same alpha true no-op | Exact existing alpha case PASS | PASS, dirty-state assertions |
| Rejected range preserves unchanged render state | Exact existing alpha case PASS | PASS, dirty-state assertions, not rendering |
| Real change marks parent and child visual dirty | Exact existing alpha case PASS | PASS, dirty-state assertions, not rendering |

**5 unique outcomes:** 2 new numeric cases + 3 alpha preservation/no-op/dirty cases, zero failed/ignored/measured. Three nested one-case wrapper child summaries repeat the same three alpha cases, not three additional cases. Parent summaries: `2 passed; 11026 filtered out` and `3 passed; 11025 filtered out`.

Shared fixture reached its complete markers after every loop: valid `0, 0.5, 1` across Frame/Texture/FontString (9 iterations); invalid `-0.25, 1.25, -(2^-30), 1+2^-30, -infinity, +infinity, NaN` across those targets (21 iterations). Iterations and assertions are not additional tests. Controls assert zero setter returns, direct GetAlpha roundtrip, pcall rejection, stored alpha 0.625 preserved and inherited effective alpha unchanged. Inspected fixture and Rust wrappers have no post-get overrides.

Both runtime directories contain all six artifacts: admission.json, argv.json, launcher.txt, result.json, stdout, stderr. Complete stdout/stderr read; sizes match results. Launcher exit 0 and recorded process `code=exited, status=0/SUCCESS` for both. Numeric stderr: full initialization diagnostics plus both exact completion markers; alpha stderr: three complete initialization sequences. No reported runtime errors/warnings in these stderr files. Full diagnostics retained in proof.json, not inferred from exit alone.

## Producer and resources

Recorded no-run compilation succeeded (741 records, no source compiler messages). Full compile.stderr has six deprecated iced-wgpu manifest lint keys and aggregate warning; not warning-free. Existing producer unit read-only observation: loaded, active/exited (RemainAfterExit), Result=success, ExecMainCode=1 (exited), ExecMainStatus=0; memory peak 7,847,985,152 bytes; CPU 779,334,166,000 ns. No service changes.

Runtime units are now not-found: their default systemctl success/zero fields do not prove execution. Existing launchers record numeric runtime 1.100s, CPU 942ms, peak 131.8M; alpha runtime 1.565s, CPU 1.517s, peak 84.1M; swap 0B each.

## Still unproven / blocked

Normal startup after producer **NOT EXECUTED**: normal runtime directory contains admission.json only, allowed=false, **zero launches**. Recorded projected memory 44,240,631,340 bytes exceeded memory.high 42,949,672,960 bytes; this is admission denial, not a runtime test failure. Normal executable association/seal does not establish startup behavior.

PTR asynchronous compilation is outside this gate and not audited. Native WoW parity, rendered pixels, full parent acceptance, normal startup and PTR acceptance remain unproven. Dirty-state assertions are not native/render/full-parent proof.

## Saved evidence

`<PRIVATE_V>/region-alpha-retail-runtime-current/report.md` and `proof.json`: mode 0600, file fsync, directory fsync, complete readback byte equality. Proof embeds all twelve runtime artifact texts, full compile stderr, controls, receipt metadata and proof ledger. No Bash, delegation, public-payload reads, authentication, new hashes, builds, reruns or service mutations.
