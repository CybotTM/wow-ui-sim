# Default-profile pending controls — current reconciliation

Date: 2026-10-12. Read-only reconciliation; no tests, builds, checks, formatters, payload reads, hashing, or execution performed.

## Result

**No exact default-profile behavior selector identified as genuinely unexecuted based on the inspected current receipts.** The earlier pending note in `retail1200-gates-resumed.md` is stale for the specific controls it described; later accepted receipts supersede those pending claims narrowly. This is not global or full Retail 12.0.0 acceptance.

- The historical native-epoch retirement selector is `patch_12_0_0_deprecated::patch_12_0_0_deprecated_native_retirement`. The accepted independent count-green audit records this exact selector executed and passed 1/1 in receipt A at submission revision `095081101f04254270dee6805ac6b8cbb8e9ea3b`. It asserts bare API absence for 21 data rows; it does not test cached loading/successors or native WoW.
- The cached retirement/successor selector is `patch_12_0_0_deprecated::patch_12_0_0_deprecated_retirement_and_successors`. It is not covered by count-green's selected runs. The accepted retirement-successors record separately documents this cached case passing at historical revision `c6bc87c120`, saved full-suite log line 13804. That is a genuine accepted historical execution receipt, but it does not establish execution against the later `84dcf2723` implementation/current submission. Main subsequently located exact named PASS in the already accepted frozen BA log at line13356: `test patch_12_0_0_deprecated::patch_12_0_0_deprecated_retirement_and_successors ... ok`. That is one existing prefork case, already inside accepted totals—not a new execution. Historical-c6-only uncertainty is superseded for the BA snapshot; no later-HEAD/hermetic/native acceptance claim.
- Exact-12.0.0 prose, early-policy, and bare-retirement selectors at `095081101` are explicitly accounted for as 3/3, 1/1, and 1/1 respectively by the independent count-green audit. The all-default count controls are a different scope and do not turn those into default-profile cached-retirement coverage.

## Proof level and boundary

Evidence is static reconciliation of the requested handoff/spec/data and accepted documentation receipts. No sealed artifact payload or hash was inspected or recomputed. No new execution was performed. Main's targeted retained-log lookup establishes this exact case passed in the later frozen BA snapshot. The candidate is not a genuinely missing selector; do not rerun for newer HEAD/documentation. This strengthens only named existing coverage, not whole current-revision acceptance. Broad/default profile acceptance remains open.

Sources: `docs/specs/patch-12-0-0-prose-models.md` (bounded count receipt); `docs/specs/retirement-successors.md` (historical cached case); `docs/specs/patch-12-0-0-extract-enums-deprecated.md` (scope and historical-control status); `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/12.0.0-session-2026-10-05/count-green-20261010/independent-report.md` (accepted receipt audit).
