# wowdecomp behavioral audit

Source-only research begun 2026-10-15. No simulator implementation changes and no execution of the reconstructed client. Read the [security assessment](security.md) before using the external checkout.

## Provenance and interpretation

| Item | Pinned evidence |
|---|---|
| External repository | `https://github.com/SatyPardus/wowdecomp`, cloned into `~/Repos/wowdecomp` |
| External revision | `0e4e9fe0a4a9fa08027c5eed4b2c5e873739cfd3` |
| Claimed original | Root `README.md:3`: WoW **3.3.5a**, `Wow.exe` build **12340**, June 2010 |
| Simulator comparison baseline | `739986c9fa011ca4ab8150ed9df073e14896e3a6` |
| Proof level | Static source comparison, not executed regressions or original-client measurements |

**The simulator's Wrath profile is not the original 3.3.5a client.** `src/client_profile.rs:97-98,164-172` selects interface `38001` for Wrath. Historical behavior below is evidence for investigation, not automatic authority to change Wrath Classic, retail, PTR, Era, Mists, Anniversary or Forever. Each eventual implementation must resolve its target-profile contract against existing specs and available native evidence. No Forever-client probes are assumed available.

The upstream README itself warns that a ported function can differ from the original. `OFFSET` comments identify claimed original addresses; `VERIFY` labels and completion percentages are not independent validation. This audit paraphrases reconstructed behavior rather than importing code. Licensing/provenance rights for code reuse have not been established.

## Evidence standard

An actionable entry needs both a reconstructed source path and the actual current simulator path, an observable difference, the required behavioral change and a proposed regression. Existing tests are coverage context, not evidence they were run here. Registration and call paths matter: a function body alone does not establish exposed API behavior.

- **Static discrepancy:** both implementations demonstrably differ; original-client parity and cross-profile applicability remain unverified.
- **Candidate:** a source inconsistency, incomplete trace or unresolved profile difference prevents promoting it to a rework requirement.
- **Rejected:** simulator already implements the behavior, or the difference is an intentional scope exclusion.

This directory is a research backlog, not a replacement for owning feature specs or authorization to implement changes. No tests, builds or simulator runtime acceptance are claimed.

## Reconstruction counterexample

`Engine/Source/Frame/CSimpleFontString_Scripts.cpp:674-706` defines getter/setter bodies for indented wrapping, but `:748-749` registers `GetIndentedWordWrap` to the setter and `SetIndentedWordWrap` to the getter. Simulator `src/lua_api/frame/methods/text_attribute_event/mod.rs:362-375` registers their normally named counterparts.

Do **not** invert simulator APIs to imitate that table. This is an unresolved reconstruction/registration anomaly, not an established simulator bug. It demonstrates why this source cannot be treated as native truth without qualification.
