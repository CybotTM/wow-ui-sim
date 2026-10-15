# Protected-recipient anchor targets

Shared clean-room simulator policy for `SetPoint`, including Wrath and Retail. Derived from the supplied functional requirements, without consulting reconstructed implementations; not native-client parity. [Existing caller restrictions](../protected-frame-enforcement.md) and [offset writes](protected-anchor-offsets.md) remain separate contracts.

## What it must do

- [ ] A protected recipient rejects a resolved Texture or FontString target, including an ordinary Frame whose active anchor dependencies reach either region kind. Target protection is not the criterion.
- [ ] Decline before mutation: preserve every existing `GetPoint` tuple, point count and resolved geometry. No new error, return-value or event contract is required.
- [ ] Allow unprotected recipients to use region targets, and protected recipients to use eligible ordinary Frames regardless of those targets' own protection flags.
- [ ] Evaluate current active anchor chains on each proposed write; clearing/replacing/restoring a target's anchors changes its eligibility. Preserve existing caller-security, cycle and forbidden-aspect checks.

## Tests asserting this spec

`tests/protected_anchor_targets.rs`: five unique cases executed in the source `3fa90ab7b6f8cf836e09c19ed34115eba9f245c1` Retail integration artifact: **1 allowed-control PASS, 4 target-rejection FAIL, exit 101**. Saved full streams and result remain private under epoch `cleanroom-controls-retail-baseline-20261015T223734529083Z/protected-targets-first-red`; no published receipt is claimed. Each failure reports `GetPoint: field 2 changed` before later geometry assertions; this does not establish that every authored assertion, chain depth, region variant or removal/restoration phase was reached. Trusted, out-of-combat calls create recipients through public `CreateFrame` consuming a real parsed, simulator-owned protected XML template. No host protection toggle or vendor template patch. Rejected calls compare complete points/geometry; equivalent unprotected witnesses prove each input would move a permitted recipient. Direct Texture/FontString, one/two-hop chains, successful controls and active-chain removal/restoration have authored assertions; the bounded baseline above establishes only the reached failure boundaries and passing control.

## Implementation status

L1 producer authored: `SetPoint` checks the protected recipient's resolved target before the unchanged-point fast path or any anchor/secret-origin mutation. A read-only visited traversal follows current `anchors[].relative_to_id`, rejecting direct Texture/FontString targets or active chains reaching them. Target protection is ignored; reverse dirty dependencies are not consulted. Existing caller-security, cycle and forbidden-aspect checks remain in place; unresolved-target behavior is unchanged.

Producer compilation and GREEN execution are **pending**, owned by main. Existing five tests are unchanged; no new tests or runtime proof are claimed. Requirement checkboxes remain open until acceptance. Wrath, other profiles, native parity and full-parent completion are not established.

## Out of scope

`SetAllPoints`, unresolved/deferred target-name policy, retroactive mutation of existing anchors, new combat/security rules, protection propagation, arbitrary native graph/profile parity and rendered pixels.
