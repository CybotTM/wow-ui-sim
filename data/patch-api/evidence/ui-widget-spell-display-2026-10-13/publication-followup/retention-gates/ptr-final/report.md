# SpellDisplay PTR final retained-TEXT and documentation verification

Revision: `03e4071432634a588fd6285e0541ce41c307c857`. Independent verifier followed `<HOME>/AgentConfig/skills/verify/SKILL.md`. Bounded artifact mode; no fixes, delegation, tracked edits, runtime/build/binary/library/archive/source-authentication work.

## Verdict

**FAIL private0600 upstream originals. PASS all five retained TEXT projections, recursive public privacy, changed owner links/prose, and bounded retained-report requirement justification.**

- Exact whole-text originals → projection: **5/5 PASS**. Originals **50,858 bytes**, public **50,518 bytes**; all ten sizes/SHA256 receipts agree. Five snapshots byte-equal originals; ordered exact substitution plus newline normalization yields complete public bytes. No clipping, summary replacement or JSON reserialization.
- Retained private stage: **16/16 files mode0600**, including both mappings and five frozen originals. Upstream originals: **5/5 mode0644**, not0600; paths below. No changes made.
- Recursive privacy: **14 scoped texts**, 2268 strings/keys, 95 decoded JSON objects, 2282 variants; **0 leaks**. Full UTF-8, recursively decoded JSON keys/values/embedded strings, escaped Unicode/hex/slash/backslash/case/percent/HTML variants; no clipping or depth/record cap. Scope only new five, additive manifest and changed prose.
- Owning documentation: **8/8 inspected**, **27/27 changed relative links resolve**, **9/9 anchor occurrences resolve to meaningful checkpoint/owner headings**. Old README29/33 now correctly reach `retail-post-removal-runtime/report.md` and `defaultcheck/report.md`; commit363e corrects exact old URLs. New README31 reaches complete PTR report.
- Requirements: **10/10 Spell checked**, **first4/4 removal checked**, justified by both retained independent reached-assertion reports (28/28 each, eight integration scopes plus separate normal CLEAN). Last preservation requirement remains **unchecked**: Classic unproved. Evidence is retained reports, not fresh assertion tracing/source or runtime authentication.

## Permission finding — no fix

- `<PRIVATE_VERIFICATION_ROOT>/spell-display-removal-ptr-runtime-current/report.md`: observed0644, expected0600 under requested private0600 check. Retained stage snapshot is0600.
- `<PRIVATE_VERIFICATION_ROOT>/spell-display-removal-ptr-runtime-current/runtime-proof.json`: observed0644, expected0600 under requested private0600 check. Retained stage snapshot is0600.
- `<PRIVATE_VERIFICATION_ROOT>/spell-display-removal-ptr-runtime-current/test-results.json`: observed0644, expected0600 under requested private0600 check. Retained stage snapshot is0600.
- `<PRIVATE_VERIFICATION_ROOT>/spell-display-removal-ptr-runtime-current/bounded-source-comparison.json`: observed0644, expected0600 under requested private0600 check. Retained stage snapshot is0600.
- `<PRIVATE_VERIFICATION_ROOT>/spell-display-removal-ptr-runtime-current/readback-proof.json`: observed0644, expected0600 under requested private0600 check. Retained stage snapshot is0600.

## Historical versus current boundaries

Historical3916 report remains **FAIL links only** at `6de1bbbb0`: historical244 whole-text copyprivacy PASS is inherited, not rerun. All **246 existing committed publication entries** have identical blob/path/mode metadata at current revision; only **six additive files** (five PTR texts + manifest). Historical report read before/after byte-equal; untouched. Historical PTR producer **6/9 PASS,3/9 FAIL,exit101** remains reported; separate historical startup CLEAN does not erase it.

Current combined Retail/PTR retained report acceptance covers supplied Spell snapshots/query/feed/secret policy, cached lifecycle/context/polling, selected state-based removal and limited preservation controls. Retail report owns source621; PTR owns documentation-overlay source844. Checked DTO/schema clauses use the retained reports' bounded source reasoning; this audit does not independently inspect implementation/source or establish source-to-binary identity. Native/widget catalogs/world-loot/render/full-parent and Classic-policy preservation remain unproved. Retention/publication integrity is separate from runtime proof and source authentication. Tracked owner PENDING prose not rewritten.

## Proof/readback

`proof.json`: 59704 bytes, SHA256 `2d3a77181946afb0d9053429535ac8bc9b14c601bfee133eb6d562d25dbac0a9`; complete byte-equal and JSON-equal readback. Per-text sizes/digests, private modes, links/anchor headings and requirement-to-case mapping recorded. `report.md` saved mode0600 and full byte-equal readback. Writes only requested output directory.
