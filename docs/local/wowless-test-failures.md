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
- **Slider SetThumbTexture fileID**: Unknown fileIDs stored as raw string. GetTexture returns integer for numeric textures. Tests: `tests/widget_slider.rs`
- **Font object**: CreateFont registry (same object on repeat), numeric name coercion, no-name error, SetFontObject cycle detection, IsObjectType. Tests: `tests/font_object.rs`
- **StatusBar GetStatusBarTexture**: Returns nil on empty StatusBar (no auto-create). SetStatusBarTexture returns true.
- **WorldFrame**: GetObjectType returns "Frame" (WoW quirk), IsObjectType("Frame") returns false, CreateFrame("WorldFrame") errors. Tests: `tests/global_frames.rs`
- **Animation target**: SetTarget validates arguments — errors on nil/missing. Tests: wowless uiobjects.lua
- **$parent ignoresanontop**: `find_named_ancestor` skips UIParent, falls back to "Top". Tests: `tests/parent_sub.rs`
- **Texture SetColorTexture/SetTexture round-trip**: SetTexture stores original numeric file data ID in `texture_file_data_id`; GetTexture returns it as integer.
- **CreateFrame unknown types**: CreateFrame errors on unrecognized type names (e.g. "WorldFrame"). Added "EventFrame" as alias for Frame.
- **Region rect**: GetLeft/Right/Top/Bottom/Center/GetRect return nothing when no anchors. IsRectValid checks dirty flag without resolving. GetWidth/Height/Size(true) return explicit dimensions. SetSize no-op when unchanged. Separated rect_dirty cache from rect_dirty_ids. Tests: `Interface/AddOns/Wowless/tests/test_region_rect.lua`
- **Font vfs**: Fresh CreateFont returns nil from GetFont (removed default `__fontPath`). Tests: `tests/font_api.rs`

## Remaining Failures

### Frame OnShow/OnHide mutual recursion (1 failure)

Recursion pattern differs — WoW limits Show/Hide recursion depth differently than our simulator.

### Frame RegisterEventCallback (1 failure)

`RegisterEventCallback` with funtainer arg — missing `funtainer` support.

### Button states/textures (8 failures)

- `button states`: State transition error handling differs
- `button text`: Reset state child count wrong
- `button textures parent`: Multiple issues with reparenting, child count, texture reuse

### StatusBar (1 failure)

`SetStatusBarColor`: After SetStatusBarTexture + SetStatusBarColor, vertex color not applied (want 0.8, got 1).

### Event registration/dispatch (2 async failures)

- `event dispatch order`: Async event ordering is non-deterministic or wrong
- `individual event reg before all`: Registration order differs

### Event registration (1 sync failure)

`none state`: Event count differs (want 2, got 1).

### ScrollingMessageFrame (4 failures)

- `wrapsarg`: Missing function
- `mixin empty/metatable`: Mixin method resolution order differs

### SimpleCheckout (1 failure)

Missing `SimpleCheckout` frame type support.

### Visibility OnShow ordering (2 failures)

Children should see updated visibility before parent's OnShow fires. Current order: parent first, then children.

### C_Timer.NewTimer (1 async failure)

`C_Timer.NewTimer` callback receives unexpected true argument instead of nil.
