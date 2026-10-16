# wowdecomp behavioral audit

Source-only research, 2026-10-15. **46 documented source-comparison entries—not the total gap count and not 46 native-confirmed bugs.** Simulator implementation unchanged; no reconstructed client code was executed, built or imported.

## Start here

- [Finding catalogue](findings.md): all 46 entries, with links to exact evidence, observable fixtures, required behavior changes and regression proposals.
- [Coverage ledger](coverage.md): which methods/aspects were actually compared and which remain unchecked; includes [2,900 per-export rows](coverage.tsv) and [105 per-file totals](coverage-by-file.tsv).
- [Cataclysm 15595 follow-up](cataclysm-15595.md): five additional differences, mixed-origin EditBox timing, cross-version checks and rejected leads.
- [Security assessment](security.md): bounded static inspection, no confirmed implant found—not a declaration that execution is safe.
- [XML Include-depth question](xml-and-dispatch.md#include-depth--unmeasured-boundary-not-a-demonstrated-mismatch): no explicit guard in the traced paths, but no numeric maximum or native depth test established.

## Exchanged implementation checklist

**Cross-host reported evidence, not independently inspected here.** The agent-server session `40fcbe4f-2021-4001-938e-4a570838fa4a` supplied the revision-scoped results below. Checked boxes mean reported simulator implementation/proof for the named scope, not local verification, native parity or current-tip acceptance. This local checkout remains the historical audit baseline; no remote implementation was imported. User approval remains required before implementing candidates from this exchange.

- [x] **C7 button-font precedence — reported completed simulator behavior.** Retail `3a5301718`: corrected button module **12 PASS** (three C7 cases plus nine controls). Wrath `0184775b5`: bare C7 **3 PASS**. Reported coverage includes disabled precedence, highlight while pushed, independent pushed offset and retained text-child identity. Native parity, live shared-font mutation and unspecified nil-slot policy remain unproved.
- [x] **C2/C10 old EditBox gap report — superseded by reported shaped/deferred implementation.** Retail `3a5301718`: cursor/layout/lifecycle **14 PASS**. Retail `03769bb00`: XML **3 PASS**, scroll **40 PASS**, genuine cached scrolling consumer **exit 0**. Wrath `0184775b5`: keyboard **28 PASS**, EditBox **15 PASS**. Counts belong to their reported suites/revisions; do not sum them as distinct capabilities or infer every historical C10 corner case was tested.
- [x] **Missing FontString spacing API lead — rejected, not a defect or pending implementation.** Our independent static shared-registration/index trace established that the methods are retained on FontStrings; see the [bounded non-findings](cataclysm-15595.md#bounded-non-findings-and-remaining-leads). This rejects an absence claim, not all spacing/layout/native discrepancies.
- [ ] **Genuine Wrath scrolling consumer — open.** Peer reports the guard remains; bare Wrath unit-family passes do not close that integration boundary. No reported proof here authorizes guard removal.
- [ ] **C11 message access filter, C12 checked-disabled texture, T7 justification validation and T8 color conversion — unimplemented/unverified in this exchange.** No additional implementation proof supplied. Peer read-only message triage reports optional `GetNumMessages` arguments are ignored and `Message` has `message_id`, not independent access-ID state; target-profile insertion/filter contract remains unestablished. Other candidate current-tip behavior was not established by this exchange.

The historical finding prose and [coverage ledger](coverage.md) retain their original pinned source-comparison classifications. They are not a current implementation checklist; reported fixes do not retroactively turn old static differences into matches. The remaining native/profile limits and open consumer/candidate items above stay open.

## Historical comparison coverage

| Status | Registration rows |
|---|---:|
| Checked matching **aspect** | 27 |
| Accepted static difference | 74 |
| Unresolved/partial comparison | 70 |
| Unchecked behavior | 2,729 |
| **Bounded inventory total** | **2,900** |

One finding can span multiple methods; X1/X2 describe XML loader/template behavior rather than literal exported methods. The [coverage matrix and extraction limits](coverage.md) are authoritative for these counts. Registration entries are not a complete API universe or proof of callable/implemented behavior. Most inventory rows remain unchecked; they are not passes or exclusions.

## Provenance and interpretation

| Item | Pinned evidence |
|---|---|
| External repository | `https://github.com/SatyPardus/wowdecomp`, checkout `~/Repos/wowdecomp` |
| External revision | `0e4e9fe0a4a9fa08027c5eed4b2c5e873739cfd3` |
| Claimed original | Root `README.md:3`: WoW **3.3.5a**, `Wow.exe` build **12340**, June 2010 |
| Initial simulator baseline | `739986c9fa011ca4ab8150ed9df073e14896e3a6` |
| Expanded simulator source baseline | `2da20225ba147dda579dd01164917cc16f0618ed`; subsequent audit commits change documentation only |
| Follow-up reconstruction | `~/Repos/wowdecomp15595`, revision `959b0b85c5f55c565328d7c3ca33027a65e41327`, declares 4.3.4/build 15595; [tag semantics and follow-up baseline](cataclysm-15595.md#provenance-and-evidence-levels) |
| Proof level | Static source/registration/consumer comparison; no executed regression, GPU or native-client acceptance |

**Wrath profile is not original 3.3.5a.** `src/client_profile.rs:79-80,143-146` selects interface `38001`. Historical source is evidence, not authority to change Wrath Classic, retail, PTR, Era, Mists, Anniversary or Forever indiscriminately. Owning specs and available target-profile/native evidence must resolve contract conflicts before implementation; no Forever-client probes are assumed available.

Upstream README warns reconstructed functions can differ from the original. `OFFSET`/`VERIFY` comments and completion percentages are author annotations, not independent original-client validation. This audit paraphrases behavior rather than importing implementation. Licensing/provenance rights for code reuse have not been established.

## Reading the findings

A documented difference needs the reconstructed source and the actual simulator path, a discriminating fixture, proposed behavior changes/tests and explicit applicability/proof limits. Fixtures are source-derived proposals, **not observed executions**. Existing tests referenced here were inspected, not run. Some capability gaps need a missing state producer before their fixture can run; those are identified rather than fabricated as current repros.

The [catalogue](findings.md) separates missing model/render/dispatch capabilities, boundary differences, historical/versioned features and existing simulator-contract conflicts. It is a research backlog, not replacement specifications or authorization to implement every original-client subsystem. Intentional 3D rendering exclusions remain intact.

## Reconstruction counterexample

`Engine/Source/Frame/CSimpleFontString_Scripts.cpp:674-706` defines indented-wrap getter/setter bodies, but `:748-749` registers `GetIndentedWordWrap` to the setter and `SetIndentedWordWrap` to the getter. Simulator `src/lua_api/frame/methods/text_attribute_event/mod.rs:362-375` registers the normally named counterparts.

Do **not** invert simulator APIs to imitate that table. This unresolved reconstruction/registration anomaly is not an established simulator bug; bodies, names and registration/call paths must be checked together. The [15595 follow-up](cataclysm-15595.md#cross-version-checks-on-existing-questions) finds the same inversion there; duplicated reconstruction text is not independent native confirmation.
