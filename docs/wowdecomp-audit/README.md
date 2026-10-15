# wowdecomp behavioral audit

Source-only research dated 2026-10-15. No simulator implementation changes and no execution of the reconstructed client. Read the [security assessment](security.md) before using the external checkout.

## Expanded audit

The initial nine findings below were a bounded first pass, **not the total gap count**. The expanded pass inventories literal script registrations and tracks checked, mismatched, unresolved and unchecked behavior rather than treating a small shortlist as coverage.

[Additional rendering and minimap findings](additional-rendering-and-map.md): T4 glyph alpha-gradient rendering, T5 transactional font validation, M1 2D minimap overlays, M2 position-derived ping state/events, M3 zoom/view/CVar coupling. All remain static historical-source comparisons, not native-confirmed modern defects.

## Initial rework list

**Nine initial static discrepancies; zero runtime/native reproductions performed in this audit.** IDs refer to detailed entries with exact source locations, required changes and proposed behavioral regressions.

| ID | Behavior needing rework | Detail / proof |
|---|---|---|
| T1 | Texture coordinates: wrong arity silently ignored/accepted; bounds unchecked | [Texture validation](textures-and-text.md#t1--reject-invalid-texture-coordinate-arity-and-range-before-mutation); static setter comparison |
| T2 | Blend names accepted without validation; distinct modes collapse to alpha | [Blend modes](textures-and-text.md#t2--blend-mode-validation-and-rendering-must-agree); static API/enum comparison, GPU equations unverified |
| T3 | Indented wrapping stored but not consumed by layout/rendering | [Text wrapping](textures-and-text.md#t3--indented-wrapping-must-reach-layoutrendering-not-private-getter-storage); static Font→flag→renderer trace |
| C1 | Programmatic Slider values ignore step; step changes do not re-snap | [Slider quantization](controls-and-layout.md#c1--slider-value-assignment-ignores-its-step); static assignment comparison |
| C2 | EditBox cursor geometry callback has registration but no producer | [Cursor notification](controls-and-layout.md#c2--editbox-accepts-oncursorchanged-but-does-not-produce-it); static producer search and downstream Wrath guard |
| C3 | Negative/reversed selection endpoints select different text | [Highlight ranges](controls-and-layout.md#c3--reversednegative-highlighttext-endpoints-select-the-wrong-range); static range/output derivation |
| L1 | Protected-recipient region-anchor eligibility absent | [Anchor targets](controls-and-layout.md#l1--protected-frames-can-use-region-targets-that-the-reconstruction-rejects); static guard/mutation comparison |
| X1 | Non-frame virtual templates ignore trust-sensitive replacement precedence | [Template collisions](xml-and-dispatch.md#x1--non-frame-virtual-templates-overwrite-definitions-without-trust-precedence); static registry comparison |
| X2 | Include failures inconsistently abort siblings or disappear from returned diagnostics | [Include failures](xml-and-dispatch.md#x2--failed-includes-have-inconsistent-continuation-and-diagnostic-behavior); static file/status/error-path trace |

These are rework leads grounded in differing implementations. Adoption of historical behavior requires resolving the supported profile's contract; the matrix does **not** claim nine native-confirmed bugs.

## Coverage and gaps

| Compared domain | Outcome | Missing or excluded proof |
|---|---|---|
| Layout/parenting/visibility/scale/alpha/strata | L1 retained; missing reparent propagation rejected because implemented | No exhaustive layout/native/profile parity |
| Buttons/CheckButtons/Sliders/StatusBars/ScrollFrames/input | C1 retained; disabled checked-texture issue rejected; other examined controls yielded no isolated finding | No runtime interaction/GPU tests |
| Texture UVs/blending/tiling/gradients/rotation and font wrapping | T1–T3 retained; rotation/gradient/coercion leads remain unresolved | No pixel comparisons, full blend-equation audit or font-metric/native proof |
| EditBox/animation lifecycle | C2–C3 retained; UTF-8/focus/pause/explicit-stop omissions rejected; animation edge cases remain leads | No callback timing/native/large-tick loop proof |
| XML/addon loading/script identity/taint | X1–X2 retained; owner-taint and object-representation claims not established | No full taint/loader/registration-surface audit |

Not covered: complete gameplay/C_* API comparison, modern-only APIs, every XML/widget method, platform-specific behavior, full event/input engine, dependency correctness or full repository security. Full 3D/client networking/update functionality is outside simulator rework scope. Existing tests were inspected selectively, not executed; no build or full suite was needed for this documentation-only audit.

## Provenance and interpretation

| Item | Pinned evidence |
|---|---|
| External repository | `https://github.com/SatyPardus/wowdecomp`, cloned into `~/Repos/wowdecomp` |
| External revision | `0e4e9fe0a4a9fa08027c5eed4b2c5e873739cfd3` |
| Claimed original | Root `README.md:3`: WoW **3.3.5a**, `Wow.exe` build **12340**, June 2010 |
| Simulator comparison baseline | `739986c9fa011ca4ab8150ed9df073e14896e3a6` |
| Proof level | Static source comparison, not executed regressions or original-client measurements |

**The simulator's Wrath profile is not the original 3.3.5a client.** `src/client_profile.rs:79-80,143-146` selects interface `38001` for Wrath. Historical behavior below is evidence for investigation, not automatic authority to change Wrath Classic, retail, PTR, Era, Mists, Anniversary or Forever. Each eventual implementation must resolve its target-profile contract against existing specs and available native evidence. No Forever-client probes are assumed available.

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
