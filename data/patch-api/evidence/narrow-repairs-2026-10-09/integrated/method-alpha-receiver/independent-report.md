# Independent saved alpha-gradient diagnostic audit

**PASS — saved bounded Retail normal-artifact diagnostic.** Epoch `20261011T023000Z`. No replay, build, test, operational command, delegation, tracked edit, or large binary rehash performed. Companion: `independent-report.json`.

## Behavioral evidence

`probe.lua:7–11` asserts exactly two getter returns before asserting both values; each state invokes the getter twice (arity then values). Saved stdout contains precisely these six successful checks:

| FontString state | Exact arity | Start | Length |
|---|---:|---:|---:|
| default | 2 | 0 | 0 |
| set | 2 | 12 | 48 |
| replace | 2 | 6 | 24 |
| isolated second FontString | 2 | 0 | 0 |
| clear | 2 | 0 | 0 |
| after_receiver_probes | 2 | 0 | 0 |

Additional exact observations: `ALPHA_LOOKUP Frame function`, `ALPHA_RECEIVER Frame true 0 0`, `ALPHA_RECEIVER plain_table false`. Frame call succeeds with the first two values 0/0; its exact return arity is **not asserted**. Empty plain table is rejected by pcall; error text is not retained. These receiver results are observations, not asserted native contracts.

Stdout is 284 bytes, exactly the six state records, three observation records, one `ALPHA_DIAGNOSTIC_DONE` marker, then JSON `[]`. Outcome records exit 0, marker true, errors `[]`; timestamps 02:30:00.889049–02:30:08.978816 UTC on October 11, 2026. Full stderr read in two segments: 15,501 bytes; ends `Status: CLEAN`, `Lua errors: 0 unique, 0 occurrence(s)`, `Attributed owners: 0`, `Unattributed Lua errors: 0 occurrence(s)`. ALSA/card/PCM and no-audio-device diagnostics exist: stderr is not empty or universally error-free. Startup loads 291 Blizzard addons, disables SavedVariables and third-party addon loading; edit-mode cache is still read. No hermetic runtime claim.

## Submission, guards, controller

Root worker equals request JSON code and controller's completed executed code byte-for-byte. Controller contains one completed evaluation naming this epoch; controller stderr is zero bytes. Queue submission exits 0 and names `wow-method-alpha-20261011t023000z.service`.

Worker admission requires MemAvailable >=6 GiB, load1 <24, and no cargo processes whose cwd matches canonical repo or sibling full-suite-checkout. Saved attempt 1: 32.81586837768555 GiB, load1 22.41, relevant cargo `[]`, admitted true. This is a point-in-time bounded check, not exclusion of every build process or a race-free reservation. Failed admission would stop without execution; retries bounded to five with backoff/jitter.

Submission requests `agents.slice`, CPUQuota=1200%, MemoryMax=16G, Type=oneshot, RemainAfterExit=yes and saved-file stdin/stdout/stderr. Worker saves cgroup `<recorded agents.slice service>`. Membership corroborates placement; no saved independent effective-limit inspection proves actual CPU/memory enforcement. Runtime argv uses timeout 90, sealed Retail binary, `--no-addons --no-saved-vars --exec-lua @<epoch>/probe.lua lua-errors`, cwd canonical repo, WOW_SIM_CASC=0.

## Seals and accepted unit2 composition

Root and epoch probes are identical; independently computed small-file SHA256 matches submission: `65fdf5109586acd3e7b4851815e263b959546bfedbafab6413c62fce51d2e561`. Worker hashes the normal binary against its receipt before execution and records `artifact_unchanged: true` afterward; records `probe_unchanged: true` for root probe endpoint. This audit does not rehash the large binary. Saved seals are endpoint evidence, not continuous tamper exclusion or hermetic provenance.

Submission artifact object exactly matches `../unit2-green-current/20261010T235452Z/retail/normal-artifact.json`: sealed copy path `.../retail/wow-sim-normal`, SHA256 `66402b0b9bd93cbad7dc082706962ff28a41ca79601e61b804e9542bd10c4451`, non-test normal build (`test:false`, opt-level 1, debug assertions/overflow checks true), identical Retail feature list. Accepted `../unit2-green-current/independent-report.md` independently established this 407,315,976-byte normal artifact and zero-error startup scope, separately from integration-test artifacts. Its source endpoint `2026-10-11T00:06:46.420355+00:00` covers 3,846 proof-scope tracked paths with no source/blob mismatches against `cc4f4c782f66c955e635d857bfc85a79a021ca24`.

Thus saved diagnostic composes with the accepted **same sealed normal artifact**, not merely a same-name binary or test binary. Main reports empty relevant tracked diff `cc4f4c782..b23295b5a`; accepted as supplied observation, not independently rerun here. That supports relevant tracked-source continuity conditionally; it is not a fresh current build, current live-runtime identity, unchanged external dependencies/runtime assets, or native parity proof. Accepted unit2 tests remain their own seven-profile/119-case bounded gate; this alpha diagnostic adds only Retail normal-artifact observations, not seven-profile alpha coverage.

## Source interpretation and limits

`src/lua_api/frame/methods/misc/alpha_gradient.rs:39–51`: getter resolves frame ID, reads gradient entry 0, returns stored pair or 0/0, pushes two numbers, returns `Ok(2)`. No getter-specific FontString/widget-type validation. `src/lua_api/methods.rs:36–61`: receiver resolution requires frame backing (identity token or table backing) and generic script-object accessibility. Do not describe this as no access checking; the absence is a widget-type check. This explains consistency with Frame success and empty-table rejection, without establishing which wrong-widget calls native WoW permits or rejects.

**Conclusion:** six valid states/exact arity, observational receivers, marker/JSON, controller/probe consistency, requested guards and saved endpoint seals pass the bounded saved-artifact audit. No unresolved internal receipt contradictions. Native wrong-widget behavior, native parity, current freshly rebuilt artifact identity, effective cgroup enforcement and broader asset/addon/audio/GUI behavior remain unproven.

## Retained packet audit

Packet `data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/method-alpha-receiver` (main-supplied commit `d71c2091a`, no push) contains nine files. All eight manifest-covered small-file hashes match; four declared stdout/probe/outcome/admission copies match private epoch bytes. Private stderr SHA256 matches manifest; both streams have 227 lines and differ only at declared line 38, which replaces private EditMode identity with `[private identity redacted]`. Other stderr bytes/lines preserved. Seven source-endpoint small-file hashes match current files; not a whole-tree or binary seal. Retained receipt preserves artifact metadata, timing, flags, environment and probe seal while sanitizing paths. Saved469 excerpt explicitly names **FontString:GetAlphaGradient**, not Frame, at original line 7495; all 120 numbered excerpt rows independently match the original saved469 raw log exactly. No snapshot edits or full-handoff completion implied.

## Privacy-agent mutation incident and docs endpoint

Read `privacy-agent-mutation-incident.json`: another agent changed private stderr/submission despite read-only instructions; main preserved `stderr.agent-sanitized` and `submission.agent-sanitized.json` and reconstructed originals. Restored stderr hashes exactly to original tracked manifest seal. Restored submission parsed fields equal this auditor’s pre-incident saved JSON; bytes equal worker canonical `json.dumps(indent=2, sort_keys=True) + newline`. Controller/request worker confirms serialization path, argv/metadata construction and execution. **No original submission byte hash was retained**: restoration verifies fields/canonical construction, not independently sealed original byte identity. This custody interruption must remain disclosed; no runtime replay or binary hashing performed. Private report JSON contains original runtime fields and must not be copied into tracked files.

Current `docs/wiki/index.md` and `docs/wiki/investigations/method-dispatch-refactor.md` still label the diagnostic main observation/audit pending. Their FontString changed-missing-row classification, observational Frame wording, unproved native parity and suite FAIL/parent OPEN boundaries agree with audited evidence. Pending label may be replaced with this bounded independent saved-capture audit after main inspection; no broader completion follows.

Retention note: absolute private verification root and exact cgroup service string redacted from this report copy; semantic findings unchanged. Original private report JSON not copied.
