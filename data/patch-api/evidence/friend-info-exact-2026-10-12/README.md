# Exact Friend name query — bounded Retail/PTR proof

Producer `be9c734a4`; corrected security fixture `6ec0c1cb8`. [Contract](../../../../docs/specs/friend-info-name-query.md) and [final independent audit](final-independent-report.md) establish the supplied-state exact query and unchanged Mainline friend-note popup, not native/full-handoff acceptance.

| Scope | Accepted evidence |
|---|---|
| Retail exact query | Four unchanged earlier passes [reused](retail-green-20261012T122455266641Z/partial-independent-report.md); corrected secret case plus three compatibility cases [4 new passes](retail-green-20261012T122910701179Z/retry1-partial-independent-report.md). |
| Retail normal startup | [291 addons, exit0, `[]`, CLEAN0](retail-normal-retry-20261012T123242035822Z/normal-retry-independent-report.md). |
| PTR runtime | [Eight new passes](ptr-runtime-retry-20261012T124813520393Z/runtime-results.json); [283-addon normal startup](ptr-runtime-retry-20261012T124813520393Z/normal.stdout), exit0/`[]`/CLEAN0. |
| Format/default check | [Both exit0; nine source hashes unchanged](check-20261012T124825577210Z/outcome.json). Six inherited dependency-manifest warnings remain. |
| Earlier/indexed preservation | Earlier providers/C_Social retain source-equivalent behavior; current indexed/state assertions pass. No seven-profile runtime or native parity claim. |

**12 newer passing executions +4 reused passes =16 credited scope executions, eight distinct names.** Not16 new runs or a single Retail8 invocation. Actual popup assertions cover whole-addon loading, Mainline provenance, public `StaticPopup_Show`, real edit-box prefill/hide/live reopen and no recorded Lua errors. Secret assertions exercise the getter directly, not the unrelated `canaccessvalue` helper. Its parity gap remains separate. Plain numeric/userdata and malformed UTF-8 inputs were not separately runtime-tested.

## Preserved failure boundaries

- [Initial RED](retail-red-20261012T120059906734Z/independent-red-report.md): four failures; popup stops at an overbroad optional-debug diagnostic prerequisite.
- [Corrected RED](retail-red-20261012T121320784488Z/independent-red-report.md): four failures; real Mainline `OnShow` reaches nil-info/first-prefill failure.
- [First producer partial](retail-green-20261012T122455266641Z/partial-independent-report.md):4PASS/1FAIL; separate diagnostic reaches successful secret getter before fixture helper fails. Original result retained.
- [Retry1 parser failure](retail-green-20261012T122910701179Z/retry1-partial-independent-report.md): normal probe not reached; `@file` transport retry adds evidence without rewriting it.
- [PTR compile/resource history](ptr-green-20261012T123249639935Z/ptr-compile-independent-report.md): first seals authenticated once; six original load-gate refusals/zero cases and later unsafe-memory refusal retained. Final runtime-only admission records changed conditions and proves behavior, not performance.

## Retention

[Manifest](retention-manifest.json):258 byte-exact nonbinary copies,3,816,250 bytes. Raw commands/results/stdout/stderr, source/toolchain metadata and one-time seal authentication receipts retained separately. No executable copied or rehashed; existing receipts reused. Retention/link/privacy audit pending; its private report will not be recursively copied into this packet.

Line-reference erratum: the final audit's `tests/friend_info_exact.rs:128–238` range extends past the file; actual helper/test block ends at218. Typographical reference only; source hashes, assertions and runtime outcomes unchanged.

Native catalogs/name normalization/complete DTO parity, earlier-profile native contracts, full-suite/performance/external acceptance and full handoff remain OPEN.
