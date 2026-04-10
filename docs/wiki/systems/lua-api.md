# Lua API

The Lua API layer bridges Lua addon code with the Rust simulation engine. It provides WoW-compatible globals, 300+ frame methods, C_* namespaces, and a timer system — all backed by `WowLuaEnv` and `SimState`.

## WowLuaEnv (`src/lua_api/env.rs`)

```rust
pub struct WowLuaEnv {
    pub(crate) lua: Lua,
    pub(crate) state: Rc<RefCell<SimState>>,
    on_update_errors: RefCell<HashSet<u64>>,
}
```

Initializes with full Lua stdlib, creates UIParent/WorldFrame via `create_builtin_frames`, then calls `register_globals`. Execution entry points: `exec()`, `exec_named()`, `exec_with_varargs()` (addon loading with `addonName, addonTable` varargs), `eval()`.

## FrameHandle Userdata (`src/lua_api/frame/handle.rs`)

```rust
pub struct FrameHandle { pub id: u64, pub state: Rc<RefCell<SimState>> }
```

Links Lua userdata to the Rust `Frame` via `id`. `__newindex` syncs `parent.Child = frame` assignments into `parent_frame.children_keys`. `__index` resolves methods in priority order: mlua method table → children_keys → `__frame_fields[id]` → fallback stubs.

## Method Categories (14+ submodules)

| Module | Key methods |
|--------|-------------|
| `methods_core` | GetName, SetSize, GetRect, Show/Hide/IsVisible, SetAlpha, SetFrameStrata, EnableMouse, GetEffectiveScale |
| `methods_anchor` | SetPoint, ClearAllPoints, SetAllPoints, GetNumAnchors, GetPoint |
| `methods_event` | RegisterEvent, UnregisterEvent, RegisterAllEvents, IsEventRegistered |
| `methods_script` | SetScript, GetScript, HookScript, WrapScript, ClearScripts, HasScript |
| `methods_create` | CreateTexture, CreateFontString, CreateFrame, CreateAnimationGroup |
| `methods_texture` | SetTexture, SetAtlas, SetTexCoord, SetVertexColor, SetBlendMode, SetDrawLayer |
| `methods_text` | SetText, GetText, SetFont, SetTextColor, SetJustifyH/V, SetWordWrap |
| `methods_button` | SetNormalTexture, SetPushedTexture, GetFontString, SetButtonState, IsPressed |
| `methods_hierarchy` | GetParent, SetParent, GetChildren, GetNumChildren, GetRegions |
| `methods_meta` | `__index`, `__newindex`, `__len`, `__eq` |

Widget-specific: EditBox (SetMultiLine, SetAutoFocus), Slider (SetMinMaxValues, SetValue, SetOrientation), StatusBar (SetStatusBarColor), Cooldown (SetCooldown), Tooltip (SetOwner, AddLine, AddDoubleLine), MessageFrame (AddMessage).

## Global Functions

**Core overrides** — `print` appends to `SimState.console_output`; `ipairs` iterates frame children; `getmetatable` returns a fake metatable exposing all frame methods; `string.format` maps `%F` → `%f` for LuaJIT compatibility.

**CreateFrame** — Parses type/name/parent/template, registers frame, links parent-child, inherits strata/level, creates widget type defaults, applies templates, returns FrameHandle.

**Font system** — `CreateFont()`, standard fonts (GameFontNormal, ChatFontNormal, SystemFont_Small, etc.) stored as Lua tables with `__fontPath`, `__fontHeight`, `__fontFlags` keys.

**Object pools** — `CreateFramePool`, `CreateFrameFactory` (multi-template), `CreateObjectPool` (generic acquire/release).

**Utilities** — `wipe()`, `tinsert/tremove()`, `CopyTable()`, `MergeTable()`, `Mixin()`, `CreateFromMixins()`, `getglobal/setglobal()`, `loadstring()`, `strsplit()`.

**Security** — `issecure()`, `securecall()`, `securecallfunction()`, `securecallmethod()`, `forceinsecure()`, `hooksecurefunc()` (from Elune or fallback stubs).

## C_* Namespaces

C_Timer (After, NewTimer, NewTicker), C_Map (stub), C_Item (GetItemInfo stub), C_System (GetLocale → "enUS"), C_EditMode (GetLayouts), C_Quest, C_AchievementInfo, C_ClassTalents, C_Guild, C_LFGList, C_Mail, C_ActionBar — most return nil/false/0 stubs.

## Timer System

`C_Timer.After(seconds, cb)`, `C_Timer.NewTicker(seconds, cb, iterations)` — backed by `WowLuaEnv.schedule_timer()`. `process_timers()` fires due callbacks each tick. `next_timer_delay()` drives the event loop scheduling.

## Animation System

`CreateAnimationGroup()` returns a group supporting `Play()`, `Stop()`, `Pause()`, `SetLooping()`, and `SetScript("OnFinished")`. Animation types: Alpha, Translation, Scale, Rotation, FlipBook, VertexColor, Path. `fire_on_update()` ticks animation groups after OnUpdate handlers.

## Sources

- [lua-api.md](../../lua-api.md) — WowLuaEnv, FrameHandle, method categories, globals, C_* namespaces, timers

## See Also

- [[frame-data-flow]] — method lookup order, __index/__newindex, Mixin() application
- [[event-system]] — fire_event, SetScript, OnUpdate tick mechanism
- [[widget-system]] — Frame struct backing each FrameHandle
