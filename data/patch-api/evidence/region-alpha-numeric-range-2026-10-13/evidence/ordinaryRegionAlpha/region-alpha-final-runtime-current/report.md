# Region alpha — final bounded existing-runtime audit

**PASS: numeric-model runtime acceptance only.** No native WoW, render, full-parent, public-copy, current-Mists or all-Classic acceptance. No workloads, builds, tests, service mutations, Bash, cwd switches, delegation or hashes performed by this audit.

## Evidence scope

Base: `<PRIVATE_V>/widget-discrete-progress-current`.
Retail epoch `region-alpha-retail-green-20261014T032102641185Z`; recorded source `71127a8e164c9ebeeee3439251984d08402483dc`.
PTR epoch `region-alpha-ptr-green-20261014T032843188553Z`; recorded source `51508fde9aa3657b09ea806d5c0411a28dc0e07b`, production delta `71127a8e1`.
Retail711/PTR515 docs-only source delta is caller-established scope; revision metadata records both sources and the shared production delta. This audit does not authenticate source/history, executables, libraries or archives. OLD480 Mists is distinct: source `480b9f934e017ff87e0c79c2ba28b47b33cded19`, epoch `xml-selfkey-mists-green-20261013T222949790458Z`.

Retail's accepted `region-alpha-retail-runtime-current/report.md` supplies five unique Rust cases and its accepted twelve raw texts. The underlying twelve artifacts were not reopened; their contents are not recopied here. Five new bundles were fully read: admission.json, argv.json, launcher.txt, result.json, stdout, stderr each (30 artifacts). Full new diagnostics are embedded in proof.json; stdout/stderr byte sizes match result records. Every new launcher records exit 0 and `code=exited, status=0/SUCCESS`; each admission allowed execution. These are historical records, not fresh execution or binary authentication.

## Concrete coverage matrix

| Exact behavior | Retail | PTR | Proof level |
|---|---|---|---|
| Numeric 0, 0.5, 1 accepted; zero setter returns; direct alpha roundtrip | PASS | PASS | One new Rust case/profile; shared fixture, 9 combinations/profile |
| Seven invalid numbers rejected without stored/effective alpha mutation | PASS | PASS | One new Rust case/profile; 21 combinations/profile, complete marker |
| Same alpha true no-op | PASS | PASS | Existing Rust case/profile; visual/rect dirty state |
| Invalid range preserves unchanged render state | PASS | PASS | Existing Rust case/profile; rejection plus visual/rect dirty state |
| Real alpha change dirties parent and child | PASS | PASS | Existing Rust case/profile; dirty IDs, not rendered pixels |

**10 passing modern outcomes, five unique cases/profile: two new + three preservation.** No failed/ignored/measured cases. PTR parent summaries: 2 passed / 10203 filtered and 3 passed / 10202 filtered. Retail accepted parent summaries: 2 passed / 11026 filtered and 3 passed / 11025 filtered. Nested three one-case child summaries repeat the same preservation cases; they are not additional tests.

Fixture targets Frame, Texture and FontString. Invalid inputs: -0.25, 1.25, -(2^-30), 1+2^-30, -infinity, +infinity and NaN: 3 x 7 = 21 reached combinations/profile, not 21 tests. Each target is initialized to stored alpha 0.625 under a parent set to 0.5. Every failed pcall must preserve direct GetAlpha()==0.625 and GetEffectiveAlpha()==the pre-rejection snapshot. No getter replacement in the fixture, Rust wrapper or normal payload. Selected Retail/PTR fixture text is identical; both normal argv payloads contain the entire shared fixture in valid then invalid modes. Completion markers are after all loops, proving completion rather than only entry.

## Separate normal startup acceptance

Retail NEW `runtime-normal-region-alpha-bounded-20261014T033641027269Z` and PTR NEW `runtime-normal-region-alpha-bounded-fresh-receipt-20261014T034803444604Z` each emit exactly both valid/invalid completion markers then `[]`. Full diagnostics finish `Status: CLEAN`, 0 unique / 0 occurrences, 0 attributed owners, 0 unattributed occurrences. Diagnostics show Blizzard loading, post-load workarounds and GC completion: Retail 291 addons, PTR 283. CASC and sound disabled; third-party addons and SavedVariables disabled. **Two startup passes, not extra Rust tests.** Both repeat the full 9-valid/21-invalid fixture coverage; do not add their iterations to unique test counts.

PTR NEW numeric `runtime-region-alpha-numeric-contract-fresh-receipt-20261014T034802519361Z` has both completion markers in full stderr and two passing cases. NEW preservation `runtime-region-alpha-alpha-preservation-fresh-receipt-20261014T034802857240Z` has all three complete initialization sequences and three unique passing parent cases.

## Resource records

Normal hard budget 2.5 GiB (2,684,354,560 bytes); soft high 2 GiB (2,147,483,648 bytes), corroborated by admission and argv. Integration hard/soft budgets 2 GiB / 1.5 GiB. No parent limit increase is accepted or inferred.

| Recorded run | Runtime | CPU | Memory peak, verbatim |
|---|---|---|---|
| Retail normal | 7.545s | 6.978s | 2G (swap: 0B) |
| PTR numeric | 320ms | 250ms | 132.7M (swap: 0B) |
| PTR preservation | 572ms | 536ms | 86.4M (swap: 0B) |
| PTR normal | 7.599s | 7.025s | 2G (swap: 0B) |
| OLD480 Mists observation | 6.811s | 6.352s | 1.8G (swap: 0B) |

Retail accepted earlier integration records remain numeric 1.100s / 942ms / 131.8M and preservation 1.565s / 1.517s / 84.1M, swap 0B. **Rounded `2G` peak does not prove strictly below the 2 GiB soft high.** No exact peak or absence of soft-limit pressure is inferred.

## Preserved failure history and provenance limits

Retail earlier 4 GiB normal admission remains denied and zero-launch: directory `runtime-normal-region-alpha-20261014T033047042808Z` contains admission only; projected 44,240,631,340 bytes exceeded parent high 42,949,672,960. New bounded startup does not rewrite that history.

PTR first `runtime-region-alpha-numeric-contract-20261014T034151676995Z` is still empty: setup failure/zero-launch, not a failed executed test. Prior4258 claimed authentication/delivery remains UNKNOWN, not recovered. Fresh4259 receipts are delivered per caller-established history; existing receipt metadata reports PASS and matches fresh runtime recorded stat tuples. Only metadata reused; no present executable/stat/hash authentication. Receipt historical hash_reads=1 each concerns those fresh gates only; **global prior hash-read total UNKNOWN**. Audit hash reads: zero. No public payload reads; external libraries, caches/settings and archives unauthenticated.

## Separate legacy observation

OLD480 Mists NEW `runtime-legacy-region-alpha-control-20261014T034110918151Z`: Frame/Texture/FontString each SetAlpha(2)->1, SetAlpha(-0.25)->0, SetAlpha(0.5)->0.5. Nine ordinary clamp/valid combinations completed, `LEGACY_MISTS_ALPHA_CLAMP_COMPLETE` then `[]`, full diagnostics CLEAN/zero errors. Separate observation, not current-Mists build/runtime acceptance, native history or all-Classic coverage. Not added to modern ten-case or two-startup totals.

## Saved proof

`report.md` and `proof.json` under `<PRIVATE_V>/region-alpha-final-runtime-current/`: mode 0600, file fsync, directory fsync, full byte-for-byte readback verified. Proof contains all 30 new artifact texts, accepted Retail report by reuse, fixture/wrapper, bounded preservation assertions, metadata associations, historical denial and zero-launch classification, and proof ledger. Unproven: native/render/full-parent/public-copy acceptance, current-Mists/all-Classic coverage and present cryptographic authentication.
