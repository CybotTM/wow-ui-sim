# Protected-recipient anchor targets

Shared clean-room simulator policy for `SetPoint`, including Wrath and Retail. Derived from the supplied functional requirements, without consulting reconstructed implementations; not native-client parity. [Existing caller restrictions](../protected-frame-enforcement.md) and [offset writes](protected-anchor-offsets.md) remain separate contracts.

## What it must do

- [ ] A protected recipient rejects a resolved Texture or FontString target, including an ordinary Frame whose active anchor dependencies reach either region kind. Target protection is not the criterion.
- [ ] Decline before mutation: preserve every existing `GetPoint` tuple, point count and resolved geometry. No new error, return-value or event contract is required.
- [ ] Allow unprotected recipients to use region targets, and protected recipients to use eligible ordinary Frames regardless of those targets' own protection flags.
- [ ] Evaluate current active anchor chains on each proposed write; clearing/replacing/restoring a target's anchors changes its eligibility. Preserve existing caller-security, cycle and forbidden-aspect checks.

## Tests asserting this spec

`tests/protected_anchor_targets.rs`: five cases compiled in the source `3fa90ab7b` Retail integration artifact; all remain **UNEXECUTED**, with baseline RED pending actual runtime receipts. Trusted, out-of-combat calls create recipients through public `CreateFrame` consuming a real parsed, simulator-owned protected XML template. No host protection toggle or vendor template patch. Rejected calls compare complete points/geometry; equivalent unprotected witnesses prove each input would move a permitted recipient. Direct Texture/FontString, one/two-hop chains, successful controls and active-chain removal/restoration are covered by authored assertions only.

## Implementation status

Current source has separate caller/combat authorization and pre-mutation target resolution, but no modeled target-kind eligibility gate. Behavioral RED and implementation remain pending.

## Out of scope

`SetAllPoints`, unresolved/deferred target-name policy, retroactive mutation of existing anchors, new combat/security rules, protection propagation, arbitrary native graph/profile parity and rendered pixels.
