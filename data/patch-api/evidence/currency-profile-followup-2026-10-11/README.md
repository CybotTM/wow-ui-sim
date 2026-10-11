# Currency profile follow-up — partial pure-model matrix

Historical captures below retain two Era resource blocks and initially unattempted Anniversary/Forever. [Later seven-profile proof](../currency-seven-profile-2026-10-11/README.md) supersedes current targeted-matrix status without rewriting these receipts or unblocking vendor/native lanes.

[Independent PTR/Wrath gate](gate/report.md): 16/16 named cases each, 32 actual PASS, compile/execution exits0. Both selected binary seals and eight scoped before/after/current inputs match. Code scope matches `1dcc59c07`; later changes through `ae6c25bae` are docs/data only. Six unique external manifest deprecations remain. Fixtures include supplied Retail metadata; passing a build profile does not establish that profile's native catalog.

| Lane | Execution | Limit |
|---|---|---|
| PTR | 16 PASS | Pure `WowLuaEnv` state/API probes, no vendor-loaded startup |
| Wrath | 16 PASS | Same shared simulator fixture; vendor-input lane remains blocked |
| Era | Zero cases in two guarded blocks | Original and remaining-only retry stop after five admission attempts; not a source failure or PASS |
| Anniversary / Forever | Unattempted | No compile/execution evidence |

[Original outcome](original-outcome/outcome.json) includes Era RESOURCE_BLOCKED while `results.json` includes only completed lanes. The original gate did not authenticate that control flow; separately retained worker source contains the break before writing `results.json`. [Retry outcome](retry-outcome/outcome.json) contains only Era RESOURCE_BLOCKED. Retry targets only Era/Anniversary/Forever, not completed PTR/Wrath. Retry scope prose inherits the original five-profile label; actual loop and submission receipt define the remaining-only scope. Admission records preserve load/RAM/process observations; guards were not bypassed.

[Manifest](retention-manifest.json): 33 exact text copies, 1,307,164 bytes. Binaries, native/vendor inputs and private session/provider payloads excluded. Initial copy evidence is direct caller preservation; packet endpoint gate is separate from the source/runtime gate above.

Combined with [default/Mists evidence](../currency-supplied-state-2026-10-11/README.md), four configurations have 68 GREEN executions of 20 distinct named tests. No new distinct behavior count, source-input unblocking, native/full-profile acceptance or parent closure follows. [Contract](../../../../docs/specs/currency-list-field-projection.md). [Completed full suite](../currency-fullsuite-2026-10-11/README.md) remains FAIL: 15,028 PASS/1 Garrison FAIL/19 skips/15,029 executions; four new currency regressions PASS inside totals. c75 Garrison FAIL remains historical. [Retained endpoint audit](../currency-fullsuite-2026-10-11/matrix-gate/report.md) covers this packet at `2b1ab710d`; its async wording describes that earlier endpoint.
