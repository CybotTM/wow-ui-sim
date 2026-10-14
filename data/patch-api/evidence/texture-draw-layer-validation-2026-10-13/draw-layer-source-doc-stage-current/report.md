# Draw-layer bounded source/doc audit proposal

Scope: producer 6e085fce9, test formatting 6bdb02b06, tests/spec ab0. Read-only canonical audit; only this local handoff directory is written. No canonical edits, commits, workloads, builds, checks, hashes, or delegation. Git show inspected the two supplied revisions; revision IDs are labels, not newly computed attestations. Initial git read failed because Pyrun cwd was not the repository; explicit -C corrected the read.

## Readability violations

None identified in the changed validation/call site or bounded test fixture/helper. No metric/check tooling executed. Adjacent existing functions are excluded; no refactor proposed.

## SOURCE and proof boundary

| Boundary | Status | Evidence / limitation |
|---|---|---|
| Ordinary CreateTexture supplied-name validation | SOURCE | hierarchy.rs:457–473; call at 510–512 precedes Frame::new at 517 and registration at 534. DrawLayer::from_str rejects unknown supplied names. Not executed producer proof. |
| Retail/PTR containment | SOURCE | ACTIVE profile guard returns early elsewhere. VectorGraphics passes Some object type and skips new validation. Earlier-profile/vector behavior not freshly executed. |
| Complete shared fixture, preproducer Retail/PTR | RED, supplied status | Owning spec records valid-mode completion; invalid CLI1, accepted INVALID, regions 5 -> 6, first rejection assertion fails. Count assertion and invalid completion marker unreached. No new receipt audit performed for PTR. |
| Initial independent Retail RED | Retained independent artifact audit | Local verification/invalid-draw-layer-red-current/report.md records source711 normal Lua, five valid baseline roundtrips, pcall true, 5 -> 6, exit1; count assertion unreached. Independent audit does not cover PTR or current producer. |
| New Rust cases | Authored / uncompiled | Two profile-gated cases invoke complete shared fixture; formatting-only revision adds no execution proof. |
| Producer GREEN / valid preservation / startup / check-format | Pending | No current execution evidence supplied or generated here. Native, renderer, broader profiles and full-parent acceptance not established. |

## Minimal exact-edit proposal

exact-edits.json contains four unapplied edits: one existing-widget section, index/log links, and explicit source revision labels in owning spec. Contract checkboxes remain unchecked. Existing spec is accurate about the first failure boundary and remains status SSOT. No public acceptance packet or nonexistent evidence link is proposed. Local independent report is referenced here only, not turned into a public wiki acceptance link. All proposed repository links target existing files; new section anchor exists only after applying the proposal.

Inputs read: wiki index opening/current entries (bounded, not whole historical catalog), SCHEMA, rust-readability and spec-format skills, widget inventory opening, wiki log opening, owning spec, current hierarchy creation section, Rust/shared Lua fixture, supplied producer/testformat diffs, initial independent Retail report. No public evidence payload audit.
