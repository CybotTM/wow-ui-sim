# Mount Journal Clicks Did Not Switch Selection

Clicking a mount row in the Mount Journal with the real mouse did nothing — the
selection (and the displayed mount) never changed. Programmatic `row:Click()`
worked fine, which hid the bug from every Lua-level test. Root cause was the
startup-XML fast-path parser fusing the two string arguments of an inline
`RegisterForClicks("LeftButtonUp", "RightButtonUp")` call into one garbage
string, leaving the rows registered for a click edge that can never match.

## Symptoms

- GUI: clicking a different mount in `MountJournal.ScrollBox` left
  `MountJournal.selectedMountID` (and the mount display) unchanged.
- Headless: `row:Click("LeftButton")` via `--exec-lua` switched the selection
  correctly, with or without addons/saved vars. All state-layer checks passed:
  rows mouse-enabled, click-enabled, enabled, hit-test candidates clean.
- Probe trace showed `OnMouseDown` and `OnMouseUp` firing on the row but never
  `OnClick`.

## Root Cause

`parse_single_string_literal` in
`src/lua_api/globals/create_frame/template_chain/parser.rs` stripped only the
first and last `"`:

```rust
arg.strip_prefix('"')?.strip_suffix('"')
```

`MountListButtonTemplate`'s inline `<OnLoad>` is
`self:RegisterForClicks("LeftButtonUp", "RightButtonUp");`. The generic
`MethodWithStringArg` parser (`parser_method_family.rs`) runs **before** the
dedicated multi-arg `RegisterForClicks` parser in the
`parse_inline_single_fast_handler` chain, and the naive quote strip happily
accepted the whole arg list as ONE string: `LeftButtonUp", "RightButtonUp`.

`registered_click_buttons` then contained that single fused entry, so
`frame_click_registration_matches` (`src/iced_app/mouse.rs`) never matched
`LeftButtonUp`/`AnyUp` and `fire_left_click_sequence` was skipped. An *empty*
registration set would have fallen back to the Button default (click on left
up); the fused entry was strictly worse than no parse at all.

`Button:Click()` dispatches `OnClick` directly without consulting click
registration — which is why every headless/Lua-level repro passed.

## Fix

`parse_single_string_literal` now rejects values containing an interior quote,
so multi-arg calls fall through to `parse_registration_family`'s
`parse_inline_register_for_clicks` (correct comma-split parsing) or the
compiled-Lua slow path. `parse_string_literal_args` delegates per-part to the
same helper. Strings legitimately containing commas (`SetText("Hello, world")`)
still parse as a single arg.

Regression tests: `does_not_fuse_two_string_args_into_one` and
`accepts_string_arg_containing_comma` in `parser_method_family.rs`.

## Verification Tooling

The hidden `headless-click-probe` subcommand gained a `mounts` panel that
drives the **real** GUI mouse pipeline (HitGrid + MouseMove/Down/Up dispatch)
against the mount list and asserts `selectedMountID` actually switches:

```bash
wow-sim --no-addons --no-saved-vars headless-click-probe mounts
```

Supporting extensions (all in `src/iced_app/click_probe.rs` /
`src/bin/wow_sim/gui_commands.rs`):

- `NamedClick.frame_name` accepts dotted paths for anonymous frames:
  `MountJournal.ScrollBox.ScrollTarget.#2` (parentKey segments + 1-based `#N`
  child index).
- Probe plans take an optional `verify_lua` executed after the clicks.
- `WOW_SIM_DEBUG_CLICK_DISPATCH=1` traces left-click release dispatch
  (hit frame, `mouse_down_frame`, `clicks_on_up`, registered click buttons) —
  this is what exposed the fused registration string.

## Debugging Lessons

- A click that fires `OnMouseDown`/`OnMouseUp` but not `OnClick` is a click
  *registration* failure, not a hit-testing failure.
- `Button:Click()` bypasses `RegisterForClicks` — Lua-level click tests cannot
  catch registration bugs; only the GUI dispatch path
  (`headless-click-probe` / `CanvasMessage`) can.
- The XML fast path mimics common inline handler statements with string
  pattern-matching; when a widget behaves differently from an identical
  Lua-created widget, suspect the fast-path parse of its inline scripts.
  The mount rows were likely not the only victims — any template with inline
  multi-arg `RegisterForClicks` was affected until this fix.

## Related

- `docs/startup-xml-fast-path.md` — the fast-path system this bug lived in.
- `docs/hit-testing.md` — mouse event flow (down/up/click edges).
- `investigations/` pages on talent/achievement click probes use the same
  `headless-click-probe` harness.
