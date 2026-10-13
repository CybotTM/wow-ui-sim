# Mists Store anchor regression investigation

Date: 2026-10-13
Scope: read-only source/runtime-artifact investigation only. No tests or project executables were run by this investigation; no vendor files, fixtures, builds, auth, replay, or delegation.

## Evidence examined

- Saved runtime packet: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/xml-selfkey-mists-green-20261013T222949790458Z/runtime-store-sibling-anchor-preservation/`
- Runtime was produced from source revision `480b9f934e017ff87e0c79c2ba28b47b33cded19`, profile Mists, filter `store_anchor_regression::`, 2 expected tests. Result: 0 passed, 2 failed, exit 101; complete stdout/stderr are saved in that packet.
- Test source: `<REPO>/tests/store_anchor_regression.rs`.
- Active cached addon: `<PROFILE_CACHE>/blizzard-ui/mists/AddOns/Blizzard_StoreUI/`.
- Source paths inspected: `src/loader/xml_texture.rs`, `src/loader/helpers.rs`, `src/loader/xml_frame_codegen.rs`, `src/loader/xml_layer_batch.rs`, `src/loader/xml_frame/finalize.rs`.

## Mists fixture contract

The active cached `Blizzard_StoreUI.toc` lines 12–13 select `TBC\Blizzard_StoreUISecure.lua` and `TBC\Blizzard_StoreUI.xml` for Mists. It does not select the Vanilla paths.

In cached `TBC/Blizzard_StoreUI.xml`:

- Lines 1754–1758 declare `RightInset` as a **Frame**, with anchors `TOPRIGHT (-6,-40)` and `BOTTOMLEFT relativeKey="$parent.LeftInset" relativePoint="BOTTOMRIGHT" x="24" y="0"`.
- Lines 2092–2095 declare product cards with `parentArray="ProductCards"`; the initial card anchor is `relativeKey="$parent.RightInset"` at TOPLEFT `(6,-6)`.

The alternate cached `Vanilla/Blizzard_StoreUI.xml` has the analogous RightInset anchor at line 1730 with `x="2"`; that is not the file selected for Mists. Thus the test assertion at `tests/store_anchor_regression.rs:60` requiring `x == 2` is a Retail/Vanilla fixture assumption when run against Mists. The test’s intended LeftInset sibling relationship exists in Mists, but its Mists authored offset is 24.

## Failure 1: RightInset anchor assertion

The test queries `StoreFrame.RightInset:GetPoint(2)` and asserts BOTTOMLEFT relative to LeftInset with offset `(2,0)`. The runtime failure is the final Rust `assert!(resolved)` at test line 66; the error is not a Lua query exception. Saved stderr does not include the returned tuple, so it does not establish whether the mismatch is only the expected x offset or whether the relative target/point/geometry also differ.

The inspected XML producer `xml_texture.rs:329–370` applies `normalize_published_texture_self_key` only to `<Texture>` creation. It only rewrites a texture’s own direct `$parent.<parentKey>` anchor to `$parent`; it does not implement generic frame child or sibling-key behavior. `RightInset` is a `<Frame>`, and its disputed anchor is a sibling reference (`$parent.LeftInset`), not the frame’s own self-key. Therefore this test is outside that producer’s direct behavior and cannot establish whether the failure is downstream of XML480. The supplied producer mechanism neither proves nor disproves the frame sibling-anchor result.

**Assessment:** fixture-contract mismatch is established for the asserted offset. The actual `GetPoint(2)` target resolution remains unresolved from saved output. Do not label it a pre-existing product defect or XML480 regression on this evidence.

## Failure 2: product card enumeration

The test at `tests/store_anchor_regression.rs:84` indexes `StoreFrame.productCardPoolCollection:EnumerateActive()`. The saved Lua error is `attempt to index local '(for generator)' (a nil value)` at the loop, before any card bounds or width assertions execute.

The active cached Mists `TBC` XML publishes cards under `StoreFrame.ProductCards` (`parentArray="ProductCards"`, XML lines 2092–2095), not `productCardPoolCollection`. The active cached `TBC/Blizzard_StoreUISecure.lua` uses that array: lines 899–912 create/populate `self.ProductCards[i]` and line 1049 invokes `StoreFrame_CreateCards`. Its category/layout code iterates `ipairs(self.ProductCards)` (line 795) and indexes the array (e.g. lines 576, 659). Searches of the active cached TBC Store XML and Lua found no `productCardPoolCollection`, `EnumerateActive`, or pool-creation reference.

Source producer distinction: frame XML `parentArray` handling is separate from the texture self-key normalization. `xml_frame_codegen.rs:140–158` emits direct frame parentArray insertion; `xml_frame/finalize.rs` seeds child parent arrays; `xml_layer_batch.rs:256–300, 373–404` handles parent-key/array attachment for layer children. These paths show why “source XML producer only Texture direct-ownkey” cannot explain or validate this pool field assumption. The absent pool property is inconsistent with the inspected Mists Store fixture contract; this failure does not test product-card bounds or prove an XML480 regression.

## Conclusion and limits

Both failures are real executions, but neither is valid evidence that XML480 broke the behavior it claims to test:

1. RightInset: test hard-codes Vanilla/Retail x=2 while the loaded Mists TBC declaration specifies x=24. Sibling target/geometry must be observed separately before drawing conclusions about frame-anchor resolution.
2. Cards: test queries a pool API absent from the loaded Mists Store implementation, which uses `ProductCards`. It fails before measuring bounds.

No changes were made. This report does not propose fixture weakening or vendor edits. It also does not decide the correct cross-profile regression design; that is outside this bounded read-only investigation.
