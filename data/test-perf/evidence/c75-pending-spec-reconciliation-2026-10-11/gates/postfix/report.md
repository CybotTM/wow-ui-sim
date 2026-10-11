# c75 spec-reuse path fix — PASS

**POSTFIX ARTIFACT/DOC GATE: PASS**

Revision `758faa27e549d450174e76c71c05217aa7cfd257`, 2026-10-11. Followed verify skill as verifier; no delegation.

- [EXIST] PASS — events spec:17 names existing `src/lua_api/frame/methods/text_attribute_event/events.rs`; old `forbidden_aspects/events.rs` absent.
- [SUBSTANTIVE] PASS — caller-sensitive guard label accurate. Packet README:13 contextualizes original copied audit's wrong path, names actual callers and `ensure_forbidden_aspect_absent` in `forbidden_aspects.rs`, explicitly preserves audit and unchanged proof credit.
- [WIRED] PASS — `events.rs:6–8` imports helper from `super::super::forbidden_aspects`; eight `EventRegistrations` guard calls at26/43/80/141/151/161/181/208. Helper defined at `forbidden_aspects.rs:26–46`; secure callers allowed at36–38. Events spec7–8 and README10 agree. **25/25** relative Markdown links across five scoped docs resolve; inventory in JSON.
- [ANTI-PATTERN] PASS — corrected spec/README: zero TODO/FIXME/HACK/XXX. No new executable code; historical error descriptions explicitly caveated.

## Reuse/preservation

Prior accepted **3/3 copies, 13,969 bytes; 13/13 existing PASS lines** reused from `../c75-spec-reuse-gate/report.json`; not newly verified executions/copies. Prior reports remain **FAIL**, original copied audits untouched. Only this new report pair written; byte comparisons before/after these writes confirm preservation.

Initial read-only Git output: HEAD `758faa27e549d450174e76c71c05217aa7cfd257`, exit0. `git diff --name-only c75ccfe02c52f76a0456ae393581a3b9ca601ce7 -- src tests Cargo.toml Cargo.lock`: exit0, empty stdout. No repeat; selected tracked source/tests/Cargo unchanged. No binary/cache/native provenance credit.

## Claim boundaries

- Managed-aura registration retention not asserted (onupdate spec9/24/28, README6). Distinct bounded AuraContainer Blizzard registration retention not generalized to dirty-phase fixture.
- Scenario supplied row classification not global/native identity-resolver proof (scenario spec14/35/41, README7).
- Roleset provided policy modeled; native argument shape/IDs/defaults/coercion unverified (README11).
- Current secure EventRegistrations callers allowed (events spec7–8, README10). Historical secure-rejection/mask-zero descriptions remain historical.

Enclosing suite remains **15,024 PASS / 1 Garrison FAIL / 19 skips / 15,025 executions**. **0 additional executions**. Native/full-profile/whole-handoff acceptance OPEN. Postfix PASS closes only bounded doc/artifact gate.

Only reads, read-only Git and report writes. No tests/build/check/fullsuite/ops/delegation/Bash/rg/fd/binary hashing. Two report preparation evaluations failed before writing: missing Path import, then absent persisted ctx; corrected using one self-contained evaluation, no Git rerun. No requirement/redesign suggestions.
