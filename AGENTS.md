# WoW UI Simulator

## Important Rules

- **NEVER modify files in `Interface/AddOns/Wowless/`** — this is an external test suite, not our code.
- **NEVER modify files in `Interface/AddOns/WowlessData/`** — regenerate with `python3 tools/gen_wowless_data.py` (reads from `~/Repos/wowless/data/`). Update the source repo first: `cd ~/Repos/wowless && git pull`.
- **NEVER modify files in `Interface/BlizzardUI/`** — this is a symlink to `vendor/wow-ui-source/Interface/AddOns` (sparse checkout of [Gethe/wow-ui-source](https://github.com/Gethe/wow-ui-source)). To update, run `./scripts/setup-blizzard-ui.sh <TAG>`.

## Architecture Docs

- `docs/layout-system.md` - Anchor/layout system: AnchorPoint, single vs multi-anchor resolution, cycle detection, coordinate systems
- `docs/texture-atlas-system.md` - Texture loading, atlas lookup (~50K entries), nine-slice kits, tiling, UV remapping
- `docs/addon-loading-pipeline.md` - TOC parsing, XML/Lua loading, template registry, SavedVariables, Blizzard addon load order
- `docs/rendering-pipeline.md` - QuadBatch, WGSL shaders, tiered GPU atlas, glyph atlas, strata sorting, hit testing, alpha propagation
- `docs/widget-system.md` - Frame struct (~140 fields), WidgetType enum (18 types), WidgetRegistry, default children, visibility
- `docs/lua-api.md` - WowLuaEnv, FrameHandle, 200+ globals, 300+ frame methods, C_* namespaces, animations, implementation status
- `docs/event-system.md` - Event queue, script handler types (36 handlers), __scripts table, dispatch flow, OnUpdate tick, startup sequence
- `docs/xml-template-system.md` - XML parsing, template registry, inheritance chains, XML-to-widget conversion, inline scripts
- `docs/frame-data-flow.md` - Mixin, events, metamethods, script dispatch
- `docs/button-text-rendering.md` - Button text draw order, three-slice text-behind-background bug
- `docs/glow-plan.md` - Glow/shine effect plan
- `docs/anchor-resolution.md` - Anchor resolution walkthrough: data structures, core functions, single/multi-anchor, SetPoint API, coordinate inversion

## Docker

CI image for running `run-tests` in addon CI pipelines. Published to `ghcr.io/osso/wow-ui-sim`.

### GitHub Action

```yaml
- uses: osso/wow-ui-sim@v1
  with:
    addon: MyAddon
```

### Docker

```bash
# Run addon tests
docker run --rm -v ./MyAddon:/app/Interface/AddOns/MyAddon ghcr.io/osso/wow-ui-sim run-tests MyAddon

# Run with all Blizzard addons loaded
docker run --rm -v ./MyAddon:/app/Interface/AddOns/MyAddon ghcr.io/osso/wow-ui-sim --no-saved-vars run-tests MyAddon
```

### Building Locally

```bash
docker build -t wow-ui-sim .
docker run --rm wow-ui-sim --no-addons --no-saved-vars run-tests Wowless
```

The image is optimized for headless test commands (`run-tests`, `self-test`, `lua-errors`). It excludes audio (built with `--no-default-features`), textures, and GPU drivers to keep the image small (~220MB). The `screenshot` command is not supported in the Docker image.

## Addon Directories

- `./Interface/AddOns/` - Third-party addons bundled with the simulator (e.g., AccountPlayed). This is the simulator's own addon directory, NOT the WoW game extract.

## WoW Game Files

- `./Interface/BlizzardUI/` - Symlink → `vendor/wow-ui-source/Interface/AddOns` (Gethe/wow-ui-source sparse checkout, pinned to tag). Run `./scripts/setup-blizzard-ui.sh` to set up.
- `~/Projects/wow/Interface` - art extract from WoW game files (BLP textures, fallback for texture loading)
- `~/Projects/wow/WTF` - SavedVariables from real WoW installation

## Reference Implementations

- `~/Repos/wowless` - Headless WoW client Lua/XML interpreter. Useful as a reference for WoW API behavior but less complete than wow-ui-sim (no rendering, limited widget support, no layout engine). Use for cross-referencing specific API semantics, not as an implementation target.
  - `wowless/render.lua` - Frame-to-rect conversion with strata/level ordering
  - `wowless/modules/points.lua` - Anchor point system (SetPoint, ClearAllPoints)
  - `wowless/modules/loader.lua` - XML element handlers (anchors, texcoords, colors, gradients)
  - `data/products/wow/uiobjects.yaml` - Full Frame/Texture/Button API definitions
- `~/Repos/wow-ui-schema` - Official UI.xsd schema (62KB) for XML validation

## Rendering Order

**Frame Strata** (low to high): `BACKGROUND < LOW < MEDIUM < HIGH < DIALOG < FULLSCREEN < FULLSCREEN_DIALOG < TOOLTIP`

**Draw Layers** within frames: `BACKGROUND < BORDER < ARTWORK < OVERLAY < HIGHLIGHT`
- Textures render first, then FontStrings (text always above textures in same layer)
- Overlapping textures in same layer have undefined order

**Texture Coordinates**: 8 values - `tlx, tly, blx, bly, trx, try, brx, bry` (top-left, bottom-left, top-right, bottom-right)

### Method/Property Lookup on UserData

All frame methods (`Hide`, `Show`, `IsVisible`, etc.) are registered once on `FrameRef` via `add_method` — mlua resolves them for ALL widget types regardless of `WidgetType`. The per-type method registry (`is_method_allowed`) only gates `getmetatable()` results, not actual method calls. Verified by `test_message_frame_has_global_methods` in `src/loader/tests/mod.rs`. If runtime errors report global methods as nil, the problem is the Lua value not being a `FrameRef` (e.g., overwritten by a table), not a missing method registration.

### UserData vs Table: rawset/rawget

`FrameRef` is a UserData that `type()` reports as `"table"` (custom metamethod), but it is NOT a Lua table. `rawset(frame, key, val)` and `rawget(frame, key)` will **fail** with "table expected, got userdata". To access or clear per-frame fields (children, mixin overrides, custom properties), use `debug.getfenv(frame)[1]`:

```lua
-- Clear a field from a frame's per-instance table
local env = debug.getfenv(frame)
if env and env[1] then rawset(env[1], "SetPoint", nil) end
```

This is the table checked by `__index` (in `PATCH_INDEX_LUA` in `metatable.rs`). EditMode overrides like `SetPointOverride`, `SetScaleOverride`, `ClearAllPointsOverride` are stored here by `OnSystemLoad` and shadow the Rust methods.

## Lua + Rust Architecture

WoW frames exist in **two parallel systems** that must stay in sync:

### Rust Side (rendering)
- `widget::Frame` struct with `WidgetRegistry` HashMap
- Used for layout computation and rendering
- Parent-child via `children: Vec<u64>` and `children_keys: HashMap<String, u64>`

### Lua Side (WoW API)
- `FrameHandle` userdata with metatables
- Used for running actual addon Lua code
- Parent-child via Lua table properties: `parent.TitleContainer = frame`

### How They Connect

Each `FrameHandle` stores an `id: u64` pointing to the Rust `Frame`. Method calls like `:SetText()` use this ID to update Rust state:

```rust
methods.add_method("SetText", |_, this, text: String| {
    let mut state = this.state.borrow_mut();
    state.widgets.get_mut(this.id).text = Some(text);  // Updates Rust via ID
});
```

### Automatic Sync via `__newindex`

When Lua assigns a frame to a property (`parent.Child = frame`), the `__newindex` metamethod automatically syncs to Rust `children_keys`:

```rust
// In FrameHandle's __newindex metamethod (globals.rs)
if let Value::UserData(child_ud) = &value {
    if let Ok(child_handle) = child_ud.borrow::<FrameHandle>() {
        parent_frame.children_keys.insert(key, child_handle.id);
    }
}
```

This allows Rust methods like `SetTitle()` to find child frames via fast HashMap lookup instead of querying Lua. Test: `test_lua_property_syncs_to_rust_children_keys`.

### XML Script Inheritance (`inherit="prepend"` / `inherit="append"`)

The `inherit` attribute on `<Scripts>` elements describes the **inherited** (template) handler's position, not the new instance handler:

- `inherit="prepend"` → inherited (old) runs **first**, instance (new) runs second
- `inherit="append"` → instance (new) runs **first**, inherited (old) runs second

This is counterintuitive — "append" means the inherited handler is appended (runs after), so the new code runs first. Confirmed via wowless (`wowless/modules/loader.lua:201-210`). Implementation: `src/loader/helpers.rs` `emit_chained_handler`.

## Taint System (Elune)

The simulator uses **Elune** — Blizzard's custom Lua 5.1 fork with taint tracking built into the VM. Elune is loaded as a C library (`luaopen_security`, `luaopen_securecalls` in `env.rs`).

### What Elune provides (C runtime, NOT our code)

`issecure`, `issecurevariable`, `securecall`, `securecallfunction`, `forceinsecure`, `hooksecurefunc`, `secureexecuterange`, `debug.setobjecttaint`, `debug.getstacktaint`, `debug.setstacktaint`, `debug.settaintmode`

`issecure()` checks `debug.getstacktaint()` — returns true when stack taint is nil (Blizzard code), false when tainted (addon code). **Do NOT override issecure()** — Elune's implementation is correct and Wowless tests verify it.

### What the simulator provides (`security_api.rs`)

- `securecallmethod` — wrapper around Elune's `securecall`
- `issecretvalue`, `canaccessvalue`, `canaccessallvalues`, `canaccesstable` — permissive stubs (always true/false)
- SecureHandler stubs (do nothing)
- State/attribute driver stubs (do nothing)

### Per-addon taint stamping

- **Loading** (`loader/lua_file.rs`): `debug.setobjecttaint(compiled_func, addon_name)` on each addon's compiled closure. Blizzard UI code is NOT tainted.
- **Script dispatch** (`env.rs`): `debug.setobjecttaint(handler, addon_name)` before calling frame script handlers, using the frame's owner addon.
- **Effect**: When tainted code runs, Elune sets stack taint to the addon name. Variables set by tainted code are marked. `issecurevariable(table, key)` returns `false, "AddonName"`.

### What is NOT enforced

- **Protected frame restrictions**: `SetAttribute` does NOT check `issecure()`. In real WoW, protected frames block attribute changes from insecure code. We don't enforce this — known gap.
- **`SetForbidden`/`IsForbidden`**: Implemented as flags but not enforced on any method.

## Performance

Uses **Lua 5.1** via mlua (WoW's Lua version).

### Running the Simulator

**CRITICAL: Always compile separately from running.** Never use `cargo run` directly with a timeout — compilation alone can take 30s+ and the timeout kills the build mid-compile, wasting all progress. Always build first, then run:
```bash
# Step 1: Build (no timeout — let it finish)
cargo build --bin wow-sim

# Step 2: Run (with timeout — prevents hung processes)
WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 90 cargo run --bin wow-sim
```
**Always use `timeout 90` or less** on the run step to prevent hung processes. Never put a timeout on the build step.

### CLI Arguments

- `--exec-lua "code"` - Execute Lua code after startup events. Works in all modes: GUI (after first frame render), screenshot, and dump-tree. Prefix with `@` to load from file (e.g., `--exec-lua @/tmp/debug.lua`)
- `--no-addons` / `--no-saved-vars` - Same as environment variables below
- `--delay <ms>` - Delay in milliseconds after firing startup events (for dump-tree/screenshot)

### Environment Variables

- `WOW_SIM_NO_SAVED_VARS=1` - Skip loading WTF SavedVariables for faster startup (~18% of load time)
- `WOW_SIM_NO_ADDONS=1` - Skip loading third-party addons (for faster texture testing)
- `WOW_SIM_DEBUG_ELEMENTS=1` - Show debug visualization: red borders around elements and green dots at anchor points
- `WOW_SIM_DEBUG_BORDERS=1` - Show only red borders around elements
- `WOW_SIM_DEBUG_ANCHORS=1` - Show only green dots at anchor points

### Timing Breakdown

Each addon shows timing: `(total: io=X xml=X lua=X sv=X)`
- `io` - File I/O (~3%)
- `xml` - XML parsing (~1%)
- `lua` - Lua execution (~78%)
- `sv` - SavedVariables loading (~18%)

### Known Issues

- `BetterWardrobe/ColorFilter.lua` has very large constant tables (works in WoW's patched LuaJIT)

### Bilinear Atlas Bleed

GPU texture atlas uses `ClampToEdge` + `Linear` (bilinear) filtering. UVs are remapped to atlas slot space in `resolve_and_scale_quads` (`primitive.rs:210-214`) with **no half-pixel inset**, so the sampler bleeds into adjacent content at slot boundaries. Symptoms: thin bright lines at tile seams, colored fringe around icons/buttons.

**Partial fix applied**: `@crop:` paths isolate sub-region textures (nine-slice pieces, tiled textures) into their own atlas slots, preventing bleed from adjacent source texture content. See `crop_path_for_subregion()` in `tiling.rs` and `crop_piece()` in `nine_slice.rs`.

**Remaining**: Half-pixel UV inset needed in `resolve_and_scale_quads` to prevent bleed at atlas slot boundaries globally. Button textures (`emit_button_texture`, `emit_button_highlight` in `quad_builders.rs`) also pass raw atlas UVs without `@crop:`. See `PLAN.md` for details.

### Frame Re-creation and Orphaned Children

Several frames are pre-created in Rust before XML addons load (UIParent, WorldFrame, GameTooltip, etc.). When a Blizzard XML addon later defines a frame with the same name, `CreateFrame` creates a NEW frame with a new ID, orphaning the old one. Any children that were parented to the old frame become invisible because `collect_ancestor_visible_ids` can't reach them — the old frame is hidden and disconnected from the tree.

This is load-order dependent: if addon A creates a child of UIParent, and addon B (loaded later) re-creates UIParent via XML, the child is stranded on the old UIParent. Example: Blizzard_GameTooltip loads before Blizzard_UIParent, so GameTooltip ends up parented to the old UIParent.

**Fix**: `register_new_frame` in `create_frame.rs` calls `migrate_children_to_new_frame()` which reparents all children (updating both `parent_id` and the new parent's `children` Vec / `children_keys` HashMap). When debugging invisible frames, check for duplicate IDs via name lookup — if a frame's parent points to an orphaned/hidden frame, this migration didn't cover it.

### Button Texture Rendering

WoW buttons are **transparent by default** — `build_button_quads` renders nothing when `normal_texture` is None. Visuals come from:
- `SetNormalTexture`/`SetPushedTexture` etc. → stored in `normal_texture`/`pushed_texture` fields, state-dependent
- Child Texture widgets with custom parentKeys → render independently via `build_texture_quads`, NOT state-dependent

`SetAtlas` on a child texture propagates to the parent button's fields ONLY for standard parentKeys: `NormalTexture`, `PushedTexture`, `HighlightTexture`, `DisabledTexture`. Custom parentKeys do NOT propagate — the child renders independently.

**Button texture patterns in Blizzard UI:**

| Pattern | Example | How textures work |
|---------|---------|-------------------|
| Standard slots | UIPanelCloseButton | `<NormalTexture atlas="RedButton-Exit"/>` → propagates to button's `normal_texture` field |
| Single child texture | MinimalScrollBar Back/Forward | `<Texture parentKey="Texture"/>` → atlas set via `ButtonStateBehaviorMixin.OnLoad` → renders as child |
| Three-slice | SharedButtonSmallTemplate (Enable All) | `<Texture parentKey="Left/Right/Center"/>` → atlas set via `ThreeSliceButtonMixin.InitButton()` → children render independently |

### ThreeSliceButtonTemplate

Three-part horizontally-stretched button used for most Blizzard UI action buttons.

**Template chain:** `SharedButtonSmallTemplate` → `BigRedThreeSliceButtonTemplate` → `ThreeSliceButtonTemplate`

**Structure:** 3 child Texture widgets (parentKey "Left", "Right", "Center") + FontString for text. Center has `horizTile=true`.

**Mixin:** `ThreeSliceButtonMixin` sets atlas on children via `InitButton()` (OnLoad) and `UpdateButton()` (state changes). Atlas naming convention: `"atlasName-Left"`, `"atlasName-Right"`, `"_atlasName-Center"` (center has underscore prefix). State suffixes: `""` (normal), `"-Pressed"`, `"-Disabled"`.

**Scale logic:** `UpdateScale()` calculates scale from `buttonHeight / leftAtlasInfo.height`, applies to Left/Right, and uses `SetTexCoord()` to crop edges when button is too narrow for both.

**Key insight:** These buttons have NO `NormalTexture` set — all visuals come from child textures. The button itself must be transparent for the children to show through.

### Dump Limitations

- `--filter` matches frame **display names** (global name or `.parentKey`) per-frame, printing only matching frames
- `--filter-key` matches display names and prints the **full subtree** of matching frames — use this to explore a frame's children
- Debug script hook: `src/main.rs` loads `/tmp/debug-scrollbox-update.lua` before GUI starts (not available in dump command)

## CLI Tools

Both `wow-sim` and `wow-cli` provide CLI subcommands. `wow-sim` subcommands use the same loading as the GUI; `wow-cli` subcommands either load standalone or connect to a running server.

### Lua Errors

Check for Lua errors during startup. Outputs unique errors as JSON to stdout (all other output to stderr). **Always run this after adding/modifying stubs or API registrations.**

```bash
wow-sim --no-addons --no-saved-vars lua-errors 2>/dev/null   # Check for errors (JSON output)
wow-sim lua-errors 2>/dev/null                                # Full load with addons
```

Empty output (no JSON array) means zero errors. Non-empty output lists unique error messages with counts.

### Self-Test (Wowless Suite)

Run the Wowless test suite headlessly. Polls `WowlessTestsDone`/`WowlessTestFailures` globals via OnUpdate ticks. Outputs failures as JSON to stdout.

```bash
wow-sim --no-saved-vars self-test                    # Run Wowless tests (exit 0=pass, 1=fail, 2=timeout)
wow-sim --no-saved-vars self-test --max-ticks 20000  # Increase timeout
```

### Run Tests (Addon Test Runner)

Run Lua test files from `Interface/AddOns/<name>/tests/`. Loads the `TestFramework` addon automatically (even with `--no-addons`).

```bash
wow-sim --no-addons --no-saved-vars run-tests Wowless    # Run Wowless tests (fast, no third-party addons)
wow-sim --no-saved-vars run-tests Wowless                # Run with all addons loaded
```

**Test file syntax** (`Interface/AddOns/<name>/tests/something.lua`):

```lua
test("frame name matches", function()
    local f = CreateFrame("Frame", "MyFrame")
    assertEquals("MyFrame", f:GetName())
end)

async_test("timer fires callback", function(done)
    C_Timer.After(0, function()
        done(function()
            assertTrue(true)
        end)
    end)
end)
```

**Available assertions** (provided by `Interface/AddOns/TestFramework/`):

| Function | Description |
|---|---|
| `assertEquals(expected, actual)` | Strict equality (`~=`) |
| `assertNotEquals(expected, actual)` | Not equal |
| `assertTrue(value)` / `assertFalse(value)` | Truthy/falsy |
| `assertNil(value)` / `assertNotNil(value)` | Nil checks |
| `assertError(fn)` | Function throws an error |
| `assertType(expected, value)` | `type(value) == expected` |
| `assertAlmostEquals(expected, actual, tolerance?)` | Float comparison (default 0.001) |
| `assertContains(haystack, needle)` | String substring or table value |
| `assertStartsWith(str, prefix)` | String prefix |
| `assertEndsWith(str, suffix)` | String suffix |
| `assertMatches(str, pattern)` | Lua pattern match |
| `assertCount(expected, table)` | Table element count |
| `assertTableEquals(expected, actual)` | Deep table equality |
| `assertTableContains(table, subset)` | Table contains subset (deep) |

**Async tests**: Use `async_test("name", function(done) ... end)`. The runner ticks OnUpdate/timers until `done()` is called or 500 ticks timeout. Pass an optional assertion function to `done()` for final checks.

### Screenshot

Render the UI to an image file without starting the GUI (headless GPU, same shader pipeline as the live renderer). Text is not rendered — this is for debugging frame layout and textures.

```bash
wow-sim --no-addons --no-saved-vars screenshot                       # Render to screenshot.webp (1024x768, lossy q15)
wow-sim screenshot -o frame.webp --filter AddonList                  # Render only AddonList subtree
wow-sim screenshot --width 1920 --height 1080                        # Custom resolution
wow-sim --no-addons --no-saved-vars screenshot -o fast.webp           # Fast: skip extras
```

Always saves as lossy WebP at quality 15. Extension is forced to `.webp` regardless of what's passed to `-o`.

Also available via `wow-cli screenshot` (standalone loading, same options).

### Dump Frame Tree

Load UI and dump the frame tree without starting the GUI (for debugging):

```bash
wow-sim --no-addons --no-saved-vars dump-tree                # Fast: skip addons and saved vars
wow-sim dump-tree --filter ScrollBar                         # Filter by name substring
wow-sim dump-tree --filter-key SpellBookFrame                # Filter by name, print full subtree
wow-sim dump-tree --visible-only                             # Show only visible frames
wow-sim dump-tree                                            # Full load with all addons
```

Output shows frame hierarchy with dimensions:
```
AddonList [Frame] (600x550) hidden
  AddonListBg [Texture] (0x0) visible
  AddonListCloseButton [Button] (24x24) visible
```

Also available via `wow-cli dump` (standalone loading, same options).

### Lua REPL

```bash
wow-cli lua                    # Interactive Lua REPL
wow-cli lua -e "print('hi')"   # Execute code and exit
wow-cli lua -l                 # List running servers
```

### Dump Frame Tree (Connected)

Dump the rendered frame tree from a running simulator:

```bash
wow-cli dump-tree                      # Dump all frames
wow-cli dump-tree --filter Button      # Filter by name substring
wow-cli dump-tree --visible-only       # Show only visible frames
```

Output shows frame hierarchy with absolute screen coordinates and dimensions:
```
AddonList [Button] (x=50, y=400, w=80, h=22) visible
  CancelButton [Button] (x=430, y=508, w=80, h=22) visible
    Text [FontString] (x=430, y=508, w=80, h=22) visible text="Cancel"
```

### Convert Texture (BLP to WebP)

Convert a single BLP texture to WebP format:

```bash
wow-cli convert-texture ~/Projects/wow/Interface/BUTTONS/redbuttons.BLP -o ./textures/buttons/redbuttons.webp
```

### Extract Textures (Batch)

Extract all textures referenced by addons to WebP format:

```bash
wow-cli extract-textures                    # Use default paths
wow-cli extract-textures --output ./tex     # Custom output directory
```

This scans addon XML/Lua files for texture references and converts them from BLP to WebP.

## Textures

### Texture Sources (in order of priority)

1. `./textures` - Local WebP textures (fastest, smallest, ~45MB for ~1740 files)
2. `~/Projects/wow/Interface` - Extracted WoW game files (BLP format, fallback)
3. Addon directories - For addon-specific textures

### Adding Missing Textures

**Always convert from BLP game files**, never from `~/Repos/wow-ui-textures`. The `wow-ui-textures` PNG repo has textures at different resolutions than the BLP game files (e.g. `UIFrameMetal2x` is 1024x1024 in the PNG repo but 512x512 in the BLP). Using the wrong resolution causes nine-slice pieces to sample incorrect regions, rendering the wrong part of the texture.

```bash
# Find the BLP file (case-insensitive)
find ~/Projects/wow/Interface -iname "UIFrameMetal2x.BLP"

# Convert to webp (lowercase filename, matching existing convention)
wow-cli convert-texture ~/Projects/wow/Interface/FrameGeneral/UIFrameMetal2x.BLP \
  -o ./textures/framegeneral/uiframemetal2x.webp
```

Naming convention: lowercase directory and filename, no hyphens where the original BLP name has none (e.g. `UIFrameMetal2x.BLP` → `uiframemetal2x.webp`).

### Stale BLP Files

The `~/Projects/wow/Interface` extract can go stale between WoW patches. Staleness has two forms:

**Dimension mismatch**: The BLP dimensions don't match what `UiTextureAtlas.csv` expects. Easy to detect.

**Content mismatch (same dimensions)**: The BLP has the right dimensions but atlas sub-regions contain wrong/empty pixels. This happens when Blizzard rearranges atlas content between patches without changing the texture size. Symptoms: textures render as black squares, transparent where opaque expected, or show wrong content. The `talents.blp` atlas hit this — local BLP was 2048x1024 (correct) but the shadow/border regions were empty because the atlas layout had changed.

To detect content mismatches, extract the atlas sub-region and check if it has meaningful pixel data:
```bash
# Extract atlas sub-region (coords from atlas.rs tex_coords * texture dimensions)
magick textures/foo/bar.webp -crop WxH+X+Y +repage /tmp/claude/region.png
identify -verbose /tmp/claude/region.png  # Check Alpha min/max — all zeros = empty
```

**Fix procedure** (works for both types):

1. Find the fileDataID: `grep -i 'filename.blp' ~/Projects/wow/data/listfile.csv`
2. Optionally verify atlas dimensions: look up fileDataID in `UiTextureAtlas.csv` (columns: `ID,FileDataID,AtlasWidth,AtlasHeight`)
3. Download fresh BLP: `curl -sL "https://wago.tools/api/casc/{fileDataID}" -o /tmp/claude/fresh.blp`
4. Verify dimensions: `python3 -c "import struct; f=open('/tmp/claude/fresh.blp','rb'); f.read(12); w,h=struct.unpack('<II',f.read(8)); print(f'{w}x{h}')"`
5. Convert and replace: `wow-cli convert-texture /tmp/claude/fresh.blp -o ./textures/subdir/name.webp`

**Known stale textures fixed**: `talents.blp` (FileDataID 4556093), `talentsanimations4.blp` (FileDataID 4741460)

### Texture Path Resolution

WoW paths like `Interface\\Buttons\\UI-Panel-Button-Up` are resolved by:
1. Normalizing backslashes to forward slashes
2. Stripping `Interface/` prefix
3. Trying extensions: webp, WEBP, PNG, png, TGA, tga, BLP, blp, jpg, JPG
4. Case-insensitive directory matching as fallback

### Button Textures

Standard WoW button textures in `Interface/Buttons/`:
- `UI-Panel-Button-Up` - Normal state (128x32, dark red gradient with gold border)
- `UI-Panel-Button-Down` - Pressed state
- `UI-Panel-Button-Highlight` - Hover state
- `UI-Panel-Button-Disabled` - Disabled state


please be nice, we are working in parallel, no git stash or git revert
