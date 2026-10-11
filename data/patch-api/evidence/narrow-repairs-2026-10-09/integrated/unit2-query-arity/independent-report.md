# Independent saved CLI query-arity verification

**PASS — five explicit current-model query states; exact two-value returns.** Saved epoch `20261011T001323Z`, main unit `wow-unit2-query-20261011t001323z`. Execution receipt: October 11, 2026, 00:13:24.284377–00:13:30.248097 UTC. This is saved-artifact verification, not a new execution or native-client claim.

Read `/home/osso/AgentConfig/skills/verify/SKILL.md`; inspected root and copied epoch probe, worker, queue submission, controller, admission, submission, outcome, complete stdout/stderr, original unit2 GREEN report and artifact receipt. Independently parsed receipts and compared small probe bytes/SHA256. No build, test replay, operations, source edits, delegation, Bash, rg, fd, or new aliases. Only this requested private report was written.

## Executed behavioral coverage

The copied `probe.lua:1–7` uses `select('#', frame:IsEventRegistered('UNIT_HEALTH')) == 2`, then checks both returned values. Every state calls that helper before the final success marker:

| State | Expected two-value result | Evidence |
|---|---|---|
| Fresh frame, before registration | `false, nil` | lines 10–11 |
| Valid player/target pair | `true, 'player'` | lines 12–13 |
| Replacement by single target | `true, 'target'` | lines 14–15 |
| Unregister | `false, nil` | lines 16–17 |
| Fresh frame, invalid-first current-model policy | `true, nil` | lines 19–22 |

**5/5 states reached and passed.** Nil-valued returns count toward arity; successful value comparisons alone would not prove this. Invalid-first behavior is explicitly simulator-model policy, not native WoW evidence.

## Exact output and execution identity

Complete stdout is exactly:

```text
UNIT_QUERY_ARITY_PASS
[]
```

Independent parsing confirms marker plus trailing JSON `[]`; outcome records `exit: 0`, `marker_present: true`, `errors: []`. Marker is required: exit 0/empty JSON without it would not establish PASS. Complete stderr ends with `Status: CLEAN`, `Lua errors: 0 unique, 0 occurrence(s)`, `Attributed owners: 0`, and `Unattributed Lua errors: 0 occurrence(s)`. ALSA/no-audio-device diagnostics are present; stderr is not empty or generally error-free.

Submission invokes `/usr/bin/timeout 90` on the sealed Retail normal `wow-sim-normal`, with `--no-addons --no-saved-vars --exec-lua @<epoch>/probe.lua lua-errors`, repository cwd, and `WOW_SIM_CASC=0`. Queue submission exits 0 and names the requested main unit; saved cgroup matches it. Admission succeeds on attempt 1. Controller contains a completed evaluation, names this epoch, and its executed source equals `worker.py`; controller stderr is 0 bytes.

Root and epoch probe bytes independently match; copied SHA256 matches submission `cb9a3ccd0a64a42f1c861fb363434a4eb80cbb7e9fff0c2ecfa845829215f1f4`. Worker hashes the sealed binary before execution against the original receipt, then hashes it again after execution; completed outcome records `artifact_unchanged: true` and `probe_unchanged: true`. Post-check of probe covers root bytes; independent comparison additionally confirms the copied executed probe. These are saved worker pre/post seal checks, not hermetic provenance or a new independent large-binary rehash.

## Composition with original GREEN

Original report: [unit2 GREEN](../unit2-green-current/independent-report.md), epoch `20261010T235452Z`, repair `cc4f4c782f66c955e635d857bfc85a79a021ca24`. Its bounded seven-profile gate records 49/49 commands exit 0 and 119/119 cases pass; Retail/PTR/Forever separate normal startups each exit 0 with JSON `[]`. Those results are retained prior evidence, not rerun or newly expanded by this probe.

Submission artifact metadata exactly equals the original Retail `normal-artifact.json`: non-test normal binary, `client-retail`/default features, sealed path `../unit2-green-current/20261010T235452Z/retail/wow-sim-normal`, SHA256 `66402b0b9bd93cbad7dc082706962ff28a41ca79601e61b804e9542bd10c4451`. Current file size is 407315976 bytes, matching original GREEN evidence. Same known artifact plus worker pre/post matching seals supplies bounded identity composition; no receipt contradiction required another huge rehash.

**Conclusion:** saved normal Retail CLI execution behaviorally proves exactly two query return values across these five states, including pair registration and single replacement. It adds the explicit arity proof absent from the original GREEN execution scope. No native query/second-unit return parity claim, no other-profile arity claim, no new unit aliases, and no GUI/CASC-enabled/full-addon/audio claim. No current-checkout completion claim beyond this sealed artifact.
