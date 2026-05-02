# Adventure Guide SimpleHTML Markup

## Symptom

Encounter Journal boss overview text rendered raw WoW escape markup such as
`|cFF2959D3|Hspell:1225135|h[Suppression Zones]|h|r` and `|n` instead of
showing player-visible link text with line breaks.

## Root Cause

The overview and bullet templates use `SimpleHTML` children inherited from
`InlineHyperlinkFrameTemplate`. Normal FontStrings already passed through WoW
markup stripping, but `SimpleHTML` only removed HTML tags before storing
`text_stripped`.

There were also two `strip_wow_markup` implementations: the production
feature-independent helper in `render/mod.rs`, and the GUI text helper in
`render/text.rs`. The GUI helper was not the path used by `SimpleHTML`.

## Fix

`SimpleHTML:SetText()` now strips HTML tags first, then passes the result
through the shared WoW markup stripper. The markup stripper also converts
`|n` into a real newline so Encounter Journal descriptions do not show the raw
escape.

Coverage:

- `test_settext_strips_wow_markup_after_html_tags`
- `render::tests::strips_spell_link_before_wow_newline_escape`
- `render::text::tests::converts_wow_newline_escape`
- `render::text::tests::strips_spell_link_before_wow_newline_escape`
