# ChatFrame Scrollbar Anchor Reapply

`ChatFrame1`'s misplaced scrollbar and oversized edit box came from the simulator's inline-anchor reapply path for inherited child frames. When a child frame had both `inherits="..."` and inline anchors like `relativeTo="$parentBackground"`, the reapply code substituted `$parent` with the child's own resolved name instead of the actual parent frame name. That turned valid targets such as `ChatFrame1Background` into nonexistent globals such as `ChatFrame1ResizeButtonBackground`, leaving a nil-target anchor that the layout engine treated as screen-relative.

## Content

## Symptoms

- `ChatFrame1.ScrollBar` ended up far to the right of `ChatFrame1`, even though its top anchor still pointed to `ChatFrame1:TOPRIGHT`.
- `ChatFrame1.ResizeButton` lost its intended `relativeTo="$parentBackground"` anchor and kept only an implicit-parent `BOTTOMRIGHT`.
- `ChatFrame1.ScrollToBottomButton` anchored to the misplaced resize button, and `ChatFrame1.ScrollBar` anchored to that button.
- `ChatFrame1EditBox` then stretched to an absurd width because its right anchor follows `ChatFrame1.ScrollBar`.

## Root Cause

The bug was in `reapply_inline_anchors()` in the template child creation path:

- inherited child frames are created in [`src/lua_api/globals/template/children.rs`](../../../src/lua_api/globals/template/children.rs)
- after template application, inline child anchors intentionally replace inherited anchors
- that reapply step called `direct::set_single_anchor(..., child_name)` for the child's own anchors

For child-frame anchor strings, `$parent` should resolve against the actual parent frame name, not the child's resolved global name. In the chat-frame case:

- XML wanted `relativeTo="$parentBackground"` on `ChatFrame1ResizeButton`
- buggy reapply resolved that as `ChatFrame1ResizeButtonBackground`
- name lookup failed, so the anchor stored `relative_to_id = None`
- the layout engine treats `None` on normal `SetPoint` anchors as screen-relative, which pushed the resize button to the screen edge

This was a simulator bug, not a Blizzard `FCF_UpdateScrollbarAnchors` bug and not a `ScrollBar -> Track` object mix-up.

## Fix

- Pass the actual `parent_name` into `reapply_inline_anchors()`.
- Reapply child-frame inline anchors with `direct::set_single_anchor(..., parent_name)` instead of `child_name`.

That restores correct `$parent...` substitution for inherited child frames with inline anchors.

## Regression Coverage

Two regressions cover the fix:

- [`tests/scroll_widgets_minimal.rs`](../../../tests/scroll_widgets_minimal.rs) adds a focused template repro proving that a child button with `inherits="..."` and `relativeTo="$parentBackground"` resolves to `TestInlineReapplyBackground`.
- [`tests/chat_frame.rs`](../../../tests/chat_frame.rs) now checks the live `ChatFrame1` layout and fails if the scrollbar drifts away from the frame edge or if the edit box width explodes again.

## Sources

- [children.rs](../../../src/lua_api/globals/template/children.rs) — inherited child creation and inline-anchor reapply path
- [FloatingChatFrame.xml](../../../Interface/BlizzardUI/Blizzard_ChatFrameBase/Mainline/FloatingChatFrame.xml) — `ResizeButton`, `ScrollToBottomButton`, and `EditBox` anchor chain
- [chat_frame.rs](../../../tests/chat_frame.rs) — full Blizzard repro and regression
- [scroll_widgets_minimal.rs](../../../tests/scroll_widgets_minimal.rs) — focused template repro for `$parentBackground`

## See Also

- [[xml-template-system]] — template inheritance and child creation flow
- [[layout-system]] — how anchor targets affect final frame rects
