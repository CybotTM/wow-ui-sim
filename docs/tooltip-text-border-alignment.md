# Tooltip Text / Border Alignment

Status: visually verified on 2026-04-10

This note closes the local `PLAN.md` item:

- `Verify fix visually — single bordered tooltip with aligned text`

The renderer-side fix already landed in commit `5fa8a438`, which changed tooltip text placement to use the real `Tooltip` NineSlice inner box instead of the outer frame bounds. The remaining task was to confirm the result on an actual rendered tooltip instead of only relying on the inset math and sizing tests.

## Verification Scene

Use a single `GameTooltip` with one header line and render only the tooltip subtree:

```bash
timeout 90 cargo run --bin wow-sim -- --no-addons --no-saved-vars --exec-lua 'local owner = CreateFrame("Frame", "OffsetOwner", UIParent); owner:SetPoint("CENTER", UIParent, "CENTER", 0, 0); GameTooltip:SetOwner(owner, "ANCHOR_NONE"); GameTooltip:SetPoint("TOPLEFT", owner, "BOTTOMLEFT", 0, -10); GameTooltip:AddLine("Header"); GameTooltip:Show()' screenshot -o /tmp/tooltip-filtered.webp --width 512 --height 384 --filter GameTooltip

timeout 90 cargo run --bin wow-sim -- --no-addons --no-saved-vars --exec-lua 'local owner = CreateFrame("Frame", "OffsetOwner", UIParent); owner:SetPoint("CENTER", UIParent, "CENTER", 0, 0)' screenshot -o /tmp/tooltip-filtered-baseline.webp --width 512 --height 384 --filter GameTooltip
```

## Result

Visual inspection of `/tmp/tooltip-filtered.webp` shows the tooltip header inset cleanly inside the bordered NineSlice interior:

- the text no longer hugs the top-left border
- the text sits inside the same content box implied by the tooltip border
- no compensating drift appeared on the right or bottom edges

The rendered result matches the intended effective inset of `15px` per side:

- `12px` tooltip text padding
- plus `3px` of inner-border inset from the `7px` tooltip corner pieces overlapping the center by `4px`

## Notes

- The screenshot path is still lossy WebP, so this step is recorded as human visual proof rather than a strict pixel-regression test.
- The non-visual proof remains in:
  - `src/iced_app/tooltip.rs` via `tooltip_text_insets_account_for_tooltip_nine_slice_overlap`
  - `tests/tooltip_text.rs` via the updated tooltip sizing expectations
