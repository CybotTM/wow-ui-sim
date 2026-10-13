# Owning publication reconciliation — artifact verification

**OVERALL: PASS — bounded publication reconciliation at `ccd526c6c9f783cf7d3fe15eda05c42c6cabab59`.**

## Four artifact checks

[EXIST] PASS — all five changed files exist; both epochs contain full compile streams, submissions, sealed receipts/paths, separate owning commands/streams and row JSON. Reconciled failed dual-filter command/streams remain present. Full byte/line inventory and exact commands are in `ledger.json`.

[SUBSTANTIVE] PASS — parsed every compile stdout record (741 per epoch: 665 compiler-artifact, 75 build-script-executed, one build-finished). Both terminate with `{"reason":"build-finished","success":true}`; zero compiler-message records. Read complete stderr/stdout runtime streams and all 59 + 48 rows in each epoch. Six manifest deprecation warnings per compile remain; not warning-clean. Original compile finished in 1m 37s; reconciled compile in 39.47s. Reconciled fixtures contain exactly five and seven IDs, not placeholders.

[WIRED] PASS — `tests/patch_10_2_5_publication_sweep.rs:13` and `tests/patch_11_1_7_publication_sweep.rs:13` include the owning fixtures; cases call shared `run_publication_sweep`. `tests/common/publication_sweep.rs:306–318` persists complete rows before asserting exact equality. Registry generation in `crates/test-harness/build.rs:90–102,198–224` reads marked cases; `tests/prefork_full_ui.rs:222–224` consumes that registry. Saved single-filter commands name the cases and their exact row outputs; terminal named-case PASS proves reachability. `tests/common/publication_probe.rs:134–136` constructs VertexColor via texture → animation group → animation, then inspects method lookup rather than calling endpoints.

[ANTI-PATTERN] PASS — commit changes documentation and fixture data only. Eight added lines contain zero TODO/FIXME/HACK/XXX/except: pass markers. Empty catches/commented-out implementation are not applicable. No production or Rust test source changes.

## Exact coverage and outcomes

| Owning patch | Original rows | Original OK / gaps | Original selected result | Reconciled selected result | Fixture |
| --- | ---: | ---: | --- | --- | --- |
| 10.2.5 | 59 | 54 / 5 | 0 PASS / 1 FAIL, stale 14-ID fixture | 1 PASS / 0 FAIL | exact 5 gaps |
| 11.1.7 | 48 | 41 / 7 | 0 PASS / 1 FAIL, stale 8-ID fixture | 1 PASS / 0 FAIL | exact 7 gaps |

Original stdout and stderr explicitly report `new gaps: []`. Fixture subtraction removes **exactly nine + one resolved IDs**, adds none. Both reconciled row maps equal their original maps completely, including expected/observed fields and classifications. Register source IDs, symbols, sections and directions match all rows. Current fixture contents equal committed fixtures. Three later `running 0 tests` groups in each successful stream add no case credit.

Resolved IDs:

- `wt-global-api-C_Item.GetItemStatDelta-87`
- `wt-global-api-C_Item.GetItemStats-88`
- `wt-global-api-C_Item.GetSetBonusesForSpecializationByItemID-89`
- `wt-global-api-C_Ping.GetCooldownInfo-94`
- `wt-global-api-UnitSetRoleEnum-109`
- `wt-widgets-VertexColor:GetEndColor-141`
- `wt-widgets-VertexColor:GetStartColor-142`
- `wt-widgets-VertexColor:SetEndColor-143`
- `wt-widgets-VertexColor:SetStartColor-144`
- `wt-global-api-C_AddOns.GetAddOnLocalTable-30`

The three item members, ping cooldown member, role global and addon-local-table member report `raw=function; lookup=function`; the four VertexColor endpoints report `VertexColor; lookup=function`. These are publication observations, not function-call or behavior proofs.

Remaining 10.2.5 gaps: ping texture kit and transcription members have `raw=nil; lookup=function`; UnitAura/UnitBuff/UnitDebuff require absence but retain raw functions. Remaining 11.1.7 seven gaps are the exact committed fixture members, each `raw=nil; lookup=function`. Full IDs and observations are retained in ledger. No remaining-gap implementation was attempted.

## Terminal status and failed attempt

10.2.5 saved stdout: `test patch_10_2_5_publication_sweep::patch_10_2_5_publication_sweep ... ok`; `test result: ok. 1 passed; 0 failed; 1 total`. Caller supplied terminal exit 0 for both reconciled single-filter runs; verifier did not query the 10.2.5 service. These exit codes are caller observations, distinct from the independently read terminal streams.

11.1.7 saved stdout now contains its named `... ok` and `test result: ok. 1 passed; 0 failed; 1 total`. Exactly one read-only service query returned exit 0:

```text
systemctl --user show wow-pass2-owning-reconciled-1117.service --property=ActiveState,SubState,Result,ExecMainCode,ExecMainStatus
ActiveState=inactive
SubState=dead
Result=success
ExecMainCode=0
ExecMainStatus=0
```

No active/pending case remains in these saved streams. LoadState was not requested: default-looking status after transient-unit unload is not independent historical exit proof. Selected terminal stdout independently establishes the case result; no undocumented process exit code is fabricated.

The earlier `runtime-command.json` passes two positional filters; `runtime.stdout` is empty (zero selected cases). Full stderr contains four identical `prefork_full_ui: only one positional filter is supported` lines. **FAIL / unsupported / zero credit**, retained unchanged. Supplied files contain no durable exit-code receipt for this failed invocation. Separate single-filter PASS does not erase or relabel it.

## Source and historical boundaries

Read-only Git comparison `f3a7dcb84` → `ccd526c6c` lists only three docs and two owning fixture JSONs. Main runtime code, test Rust source, registers and historical evidence paths are unchanged by reconciliation. Historical spec tables and RED receipts retain their original counts and source attribution. Commit docs still say reconciled execution pending: that was the commit-time state; this private later verification records terminal PASS without editing repository docs.

Original submission declares `f3a7dcb84f9b4acb68eb6ce9b0d756d438285a88`; reconciled submission declares `ccd526c6c9f783cf7d3fe15eda05c42c6cabab59`. Caller reports current HEAD `e1b3b0e3d` adds only availability input scaffold/tests/spec, not its getter. This report binds owning proof to `ccd526c6c`; no transfer to current-source behavior or availability-getter credit. Binary receipts and existing digests were read only, not recomputed/authenticated. Saved Cargo target/command attribution is evidence, not independent immutable-source or clean-working-tree attestation. Compiler versions are absent from supplied artifacts.

No builds, tests, hashes, repository edits, delegation, polling or reruns. Only authored files: this `report.md` and sibling `ledger.json`. Original and failed-attempt evidence preserved. **No behavior/signature/output/security/native parity, new producer, other-profile, full-suite or whole-goal acceptance credit.**
