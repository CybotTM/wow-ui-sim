# Patch 8.2.0 publication sweep

Audit Warcraft Wiki page 533235 revision 5142162 against current retail. Source: `data/patch-api/sources/8.2.0-*`. [Audit](../wiki/investigations/patch-8-2-0-api-audit.md).

## Requirements

- [x] Pin fetched current revision and retain every caption-table inventory and non-inventory occurrence with opt-in recipes.
- [x] Discover publication gaps through the prefork full-UI harness; apply later master registers chronologically, including the merged 8.2.5 register.
- [x] Model `C_VoiceChat.SetMasterVolumeScale` through existing voice-chat state for normalized settings-slider inputs; getter reflects writes, unrelated output volume remains unchanged. Reject invalid input without mutation to preserve the simulator state invariant; native out-of-range/security parity is not claimed.
- [x] Retire unused `C_UIWidgetManager.GetTextureWithStateVisualizationInfo` on retail only after whole-word cached/source/test consumer scans. Preserve Mists' existing lookup.
- [x] Account for every unresolved inventory/prose gap with exact reasons, without placeholder publication credit.
- [x] Prove all publication sweeps, targeted affected behavior, own source reproduction, all register reproductions, Python fixtures, format and warning-clean non-vendor Mists check. Preserve and report inherited saved-extract failures rather than rewriting prior sources.

## Remaining contracts

- [ ] Fifty-three exact inventory gaps and ten substantive prose contracts remain recorded-problematic; exact producer, lifecycle or permission-policy reasons live in the audit's per-ID review.
- [x] Apply the merged 8.2.5 register; its six removals supersede six 8.2.0 publication gaps with bounded absence coverage.
- [ ] Inherited 12.0.5, 12.0.7 and 12.1.0 saved-extract reproduction failures remain unchanged, as in the completed 8.3.0 audit.

## Widget producer source checkpoint — 2026-10-14

`8645aaf0a` implements complete DoubleStatusBar26 / TextureAndText19 projections; `a62c81768` registers both and removes only `wt-global-api-C_UIWidgetManager.GetTextureAndTextVisualizationInfo-135` from known gaps. Requested ledger reconciliation is **54→53**; literal committed JSON arrays instead contain **53→52** entries across `a62c81768^`→`a62c81768`. This discrepancy is retained, not silently normalized or represented as a fresh sweep PASS. Historical counts below retain their original proof scope.

Row134 (`GetTextureAndTextRowVisualizationInfo`) is unchanged. DoubleStatusBar has zero evidenced publication rows; none invented. [TextureAndText owner](ui-widget-texture-and-text.md) and [DoubleStatusBar owner](ui-widget-double-status-bar.md) retain observed Retail RED4 PASS /8 FAIL, initial absence-only4 PASS, synthetic-fallback distinction and pending independent classification. Source is implemented but unverified: no behavioral GREEN, native/earlier availability UNKNOWN, no completion claim.

## Tests

`tests/patch_8_2_0_publication_sweep.rs`, `patch_8_2_0_publication_fixes.rs`, `patch_8_2_0_cached_surfaces.rs`, `patch_8_2_0_classic_surfaces.rs`; extractor/generator behavioral fixtures. Evidence and dynamic validator: `data/patch-api/evidence/8.2.0-session-2026-10-08/`.

## Exclusions

Publication alone is not signature, security, populated-output, historical or payload parity. No vendor/cache/Wowless edits, shims, Blizzard monkey-patches, full integration suite, push, merge or agents. Inherited saved-extract non-reproducibility remains recorded rather than silently rewritten.
