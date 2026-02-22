# Wowless Self-Test Failures

Results from `wow-sim --no-saved-vars self-test` against the Wowless addon test suite.

## Previously Fixed

- **SetAllPoints implicit parent**: `SetAllPoints()` resolves to parent_id. Tests: `tests/methods_anchor.rs`
- **Anchor cycle detection**: Raises Lua errors on cycles. Tests: `tests/methods_anchor.rs`
- **GetNumPoints default**: Returns 0 on fresh frame. Test: `test_get_num_points_default_zero`
- **$parent substitution**: Start-of-string, case-insensitive, ancestor chain walk, "Top" fallback. Tests: `tests/parent_sub.rs`
- **CreateFrame with frame in name position**: Non-coercible name → name=nil, parent=nil. Test: `test_create_frame_with_frame_in_name_position`
- **Taint error message**: `GetAttribute` validates arguments and reports addon name.
- **Frame level**: `SetParent` only recalculates level when parent changes. Tests: `tests/frame_level.rs`
- **$parent "Top" fallback**: `substitute_parent_name` uses `explicit_parent` (before UIParent fallback); `find_named_ancestor` skips UIParent → "Top" fallback. Tests: `tests/parent_sub.rs`
- **SetAllPoints implicitscreen**: Added `default_parent` flag to Frame. SetAllPoints no-arg on default-parented frames stores None; explicit-parent stores parent_id. Tests: `tests/methods_anchor.rs`
- **Anchor cycle error messages**: BFS cycle detection with Relative/Dependent/Dependent ancestors details, no "runtime error:" prefix. Tests: `tests/methods_anchor.rs`
- **SetColorTexture GetTexture nil**: Headless behavior — SetColorTexture clears frame.texture so GetTexture returns nil. Tests: `tests/methods_texture.rs`
- **Slider SetThumbTexture fileID**: Unknown fileIDs stored as raw string. GetTexture returns integer for numeric textures. Tests: `tests/methods_texture.rs`
- **Font object**: CreateFont registry (same object on repeat), numeric name coercion, no-name error, SetFontObject cycle detection, IsObjectType. Tests: `tests/font_object.rs`
- **StatusBar GetStatusBarTexture**: Returns nil on empty StatusBar (no auto-create). SetStatusBarTexture returns true.
- **WorldFrame**: GetObjectType returns "Frame" (WoW quirk), IsObjectType("Frame") returns false, CreateFrame("WorldFrame") errors. Tests: `tests/global_frames.rs`
- **Animation target**: SetTarget validates arguments — errors on nil/missing. Tests: wowless uiobjects.lua
- **$parent ignoresanontop**: `find_named_ancestor` skips UIParent, falls back to "Top". Tests: `tests/parent_sub.rs`
- **Texture SetColorTexture/SetTexture round-trip**: SetTexture stores original numeric file data ID in `texture_file_data_id`; GetTexture returns it as integer.
- **CreateFrame unknown types**: CreateFrame errors on unrecognized type names (e.g. "WorldFrame"). Added "EventFrame" as alias for Frame.
- **Region rect**: GetLeft/Right/Top/Bottom/Center/GetRect return nothing when no anchors. IsRectValid checks dirty flag without resolving. GetWidth/Height/Size(true) return explicit dimensions. SetSize no-op when unchanged.
- **Font vfs**: Fresh CreateFont returns nil from GetFont (removed default `__fontPath`). Tests: `tests/font_api.rs`
- **OnShow/OnHide mutual recursion**: Iterative handler loop with 12-invocation limit.
- **RegisterEventCallback**: Stub returns `true` (was returning nothing).
- **OnShow/OnHide ordering**: Children-first depth-first ordering (children fire before parents).
- **Button states**: SetButtonState validates input (errors on invalid), GetButtonState returns "DISABLED" when disabled, Disable resets button_state.
- **Button default children**: Removed 5 default children. Textures/text created lazily via Set* methods.
- **Frame parent keys**: Added `parent_key` field to Frame for deterministic GetParentKey.

## Remaining Failures (12)

### Button text (1 failure)

`SetFontStringFtoG transition from ftext to gtext poststate`: want 0 regions, got 1. SetFontString needs to properly transfer/remove the FontString child when reassigning between buttons.

### StatusBar (1 failure)

`SetStatusBarColor`: After SetStatusBarTexture + SetStatusBarColor, vertex color not applied to texture (want 0.8, got 1).

### Event registration (1 sync failure)

`none state`: Event count differs (want 2, got 1). IsEventRegistered may need to check both unit and non-unit registrations.

### Event dispatch order (2 async failures)

- `event dispatch order`: Async event ordering is non-deterministic
- `individual event reg before all`: Registration order differs

### ScrollingMessageFrame (5 failures)

- `fn/wrapsarg`: SetOnTextCopiedCallback wrapper missing
- `mixin/empty`: Mixin has extra entries
- `mixin/metatable/*`: Method resolution order and metatable type wrong

### SimpleCheckout (1 failure)

Missing `SimpleCheckout`/`Checkout` frame type and forbidden frame support.

### C_Timer.NewTimer (1 async failure)

Callback receives unexpected `true` argument instead of nil.
