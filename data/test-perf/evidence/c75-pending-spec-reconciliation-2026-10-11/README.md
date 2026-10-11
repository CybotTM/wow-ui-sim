# Saved c75 passes — stale spec reconciliation

Thirteen named PASS lines are extracted from the existing c75 log; they are already included in its accepted totals, not new executions. Three read-only source/pass audits are retained byte-identically. No tests or builds rerun; current requested source/test bodies and Cargo pin match c75.

| Scope | Existing proof | Limit |
|---|---|---|
| Managed aura dirty lifecycle | Line7176: hidden dirty/mode2 persists; visible dispatch clears dirty/mode0; further dispatch stays clean/disabled; captured Lua errors empty | Registration retention and populated/native aura behavior are not directly asserted |
| Scenario criteria | Four named cases PASS; line8247 isolation case includes three opaque restricted outputs from tainted plain-token query, caller-taint preservation, denied unwrap/secret-token input and readable public row | Per-row `identity_restricted` is supplied classification, not a global/native identity resolver |
| AuraContainer intrinsic/default aspect | Line5837: EventRegistrations bit, retained Blizzard registration, rejected addon registration/query/clear, secure registration and plain-frame control | Native per-operation/default-aspect parity unverified; no ManagedAuraContainer rule inferred from its name |
| Generic event guards | Lines5792–5793: eight guarded APIs with caller-sensitive behavior; listener/callback controls in audit | Current secure callers pass; old spec secure-rejection/mask-zero claims are historical, not current |
| Roleset visibility | Five named PASS lines7650/7696–7698/7719; inactive tagged frame remains shown but not visible, child/untagged controls, tag/XML/filter changes and strata membership | Provided filter interpretation is modeled policy; native argument shape/IDs/defaults/coercion remain unverified |

Evidence: [named raw lines](named-pass-lines.json), [dirty/scenario audit](pending-spec-pass-reuse-c75.md), [aspect audit](aura-existing-pass-reuse-c75.md), [roleset source audit](roleset-body-model-current.md), [copy manifest](retention-manifest.json). The roleset source audit did not find the execution lines; the separate raw-line extraction supplies those existing PASS records. The aspect audit names a nonexistent `forbidden_aspects/events.rs` path: actual event callers are in `text_attribute_event/events.rs`, with `ensure_forbidden_aspect_absent` defined in `forbidden_aspects.rs`. The original copied audit is preserved; this path correction does not change test proof.

The enclosing [c75 suite](../../../patch-api/evidence/narrow-repairs-2026-10-09/integrated/fullsuite-c75/independent-report.md) remains FAIL: 15,024PASS/1GarrisonFAIL/19skips/15,025executions. Earlier synthetic dirty-phase and intrinsic mask-zero failures remain history. No native/full-profile/whole-handoff completion or new enforcement change follows.
