# Tooltip Text Alignment

Status: visually verified 2026-04-10.

Tooltip text was hugging the top-left border instead of sitting inside the NineSlice interior.

## Root Cause

The renderer placed tooltip text using the outer frame bounds instead of the inner box defined by the `Tooltip` NineSlice corner pieces.

## Fix (commit `5fa8a438`)

Changed tooltip text placement to use the real `Tooltip` NineSlice inner box. The effective inset is **15px per side**:
- 12px tooltip text padding
- Plus 3px of inner-border inset from 7px corner pieces overlapping the center by 4px

## Verification

Visual inspection of a single `GameTooltip` with one header line confirms:
- Text no longer hugs the top-left border
- Text sits inside the content box implied by the border
- No compensating drift on right or bottom edges

Non-visual proof in:
- `src/iced_app/tooltip.rs` — `tooltip_text_insets_account_for_tooltip_nine_slice_overlap`
- `tests/tooltip_text.rs` — updated tooltip sizing expectations

## Sources

- [tooltip-text-border-alignment.md](../../tooltip-text-border-alignment.md) — verification scene and result
