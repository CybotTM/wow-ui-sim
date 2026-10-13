# Supplied currency War Mode flags — 2026-10-12

## PTR privacy correction — 2026-10-13

[Correction ledger](privacy-correction.json) supersedes byte/privacy claims for the changed PTR text only. Actual-profile identity substitutions replace the incomplete Retail-only mapping; runtime evidence and original private sources remain unchanged. Registered payload digests/bytes and totals are updated; byte-copy equality becomes transformed equality for changed copies. Historical reports remain revision-scoped; current privacy gate pending.

[Contract](../../../../docs/specs/currency-war-mode-bonus.md) · [Current patch handoff](../../../../docs/wiki/investigations/patch-8-1-0-api-audit.md#currency-warmode-query--testspec-stage-2026-10-12)

| Scope | Exact evidence | Status |
|---|---|---|
| Initial public query controls, `4a6f5ad45` | Authenticated compile0; five tests0PASS/5FAIL, runtime101 | Authentic RED |
| Real empty state and positive controls, `a1de57b4e` | Authenticated compile0; nine tests0PASS/9FAIL, runtime101 after preserved initial resource block | Authentic expanded RED |
| Getter `ce2c37ad5`, Retail | Independent9PASS/0FAIL; normal291-addon raw query/arity/empty-state/type assertions, exit0/`[]`; changed-Rust readability no violations | Bounded PASS |
| PTR | NinePASS/0FAIL; normal283-addon raw query assertions exit0/`[]`; first resource block and deferred admission preserved | Independent bounded PASS |
| Current publication fixture `dcbc347ad3` | Owning prefork1PASS;145 rows91OK/54gaps, exact query raw=function/lookup=function | Independent bounded PASS |
| Final formatting/default check | Both exit0, relevant source hashes unchanged | Independent bounded PASS |

Flags come from an explicitly supplied empty-by-default map. Independently nullable boolean values, strict identity, live updates/removal and secret-first validation are modeled query behavior, not native currency catalogs. Positive QuestUtils tooltip behavior is not established. Its first-instance status is computed locally from duplicate IDs, not a missing Rust field; separate PvP desired/quest eligibility/faction-grant models remain unproved.

`initial-retention-manifest.json`:54 byte-equal copies/756283 bytes. `through-check-retention-manifest.json` adds51 copies:105 total/895538 bytes, including independent first-auth receipts and full retained runtime streams; independent2610 verifies105/105. `final-retention-manifest.json` adds four main byte-compared final report/receipt copies:109 total/1044831bytes. Final bounded gate PASS; parent acceptance remains OPEN. No binary, key or credential payload. Pending snapshots and later execution follow-ups remain separate. Six inherited iced manifest deprecations retained. No old artifacts reauthenticated or completed sibling cases replayed. Frozen BA3FAIL and parent full handoff remain OPEN.
