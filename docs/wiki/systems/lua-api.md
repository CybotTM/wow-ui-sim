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

Initializes with full Lua stdlib, creates UIParent/WorldFrame via `create_builtin_frames`, then calls `register_globals`. Execution entry points: `exec()`, `exec_public()`, `exec_named()`, `exec_with_varargs()` (addon loading with `addonName, addonTable` varargs), `eval()`.

### Loader Environment Boundaries

`LoaderEnv::exec()` runs dynamic loader code in the currently loading addon's environment, including secure-environment and loading-scoped fenv state. `LoaderEnv::exec_public()` deliberately skips that loading fenv state while preserving the addon's secure file execution. Use it only for an addon-specific bridge that must publish a narrow compatibility surface to `_G`; it is not a generic secure-to-public mirroring mechanism. The AuthChallenge workaround uses this boundary to restore five exported callbacks publicly while the `Blizzard_AuthChallengeUI` addon remains secure (`src/lua_api/loader_env.rs`, `src/lua_api/workarounds/temporary/auth_challenge_frame_parent.rs`).

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

### Texture identity

For known WoW texture paths resolved by the bundled texture manifest, `Texture:GetTexture()` and `Texture:GetTextureFileID()` return the numeric fileDataID, while `Texture:GetTextureFilePath()` preserves the path. Current proofs include `Interface\\TargetingFrame\\UI-Classes-Circles` → `237669` and `Interface\\ICONS\\INV_Misc_QuestionMark` → `134400`.

## Global Functions

**Core overrides** — `print` appends to `SimState.console_output`; `ipairs` iterates frame children; `getmetatable` returns a fake metatable exposing all frame methods; `string.format` maps `%F` → `%f` for LuaJIT compatibility.

**CreateFrame** — Parses type/name/parent/template, registers frame, links parent-child, inherits strata/level, creates widget type defaults, applies templates, returns FrameHandle.

**Font system** — `CreateFont()`, standard fonts (GameFontNormal, ChatFontNormal, SystemFont_Small, etc.) stored as Lua tables with `__fontPath`, `__fontHeight`, `__fontFlags` keys.

**Object pools** — `CreateFramePool`, `CreateFrameFactory` (multi-template), `CreateObjectPool` (generic acquire/release).

**Utilities** — `wipe()`, `tinsert/tremove()`, `CopyTable()`, `MergeTable()`, `Mixin()`, `CreateFromMixins()`, `getglobal/setglobal()`, `loadstring()`, `strsplit()`.

**Security** — `issecure()`, `securecall()`, `securecallfunction()`, `securecallmethod()`, `forceinsecure()`, `hooksecurefunc()` (from Elune or fallback stubs).

## C_* Namespaces

C_Timer (After, NewTimer, NewTicker), C_Map (stub), C_Item (GetItemInfo stub), C_System (GetLocale → "enUS"), C_EditMode (GetLayouts), C_CatalogShop (`GetVCProductInfos()` returns a fresh empty table), C_ChromieTime (retail/PTR empty-state queries and no-op actions), C_Quest, C_AchievementInfo, C_ClassTalents, C_Guild, C_LFGList, C_Mail, C_ActionBar — most return nil/false/0 stubs.

### `C_PlayerChoice` (PTR 12.1)

`C_PlayerChoice` is a state-backed deterministic compatibility model. `GetCurrentPlayerChoiceInfo()` returns nil with the default empty state or a documented `PlayerChoiceInfo` table with nested options, buttons, and currency/item/reputation rewards when `SimState.player_choice.current` is seeded. `GetNumRerolls()`, `GetRemainingTime()`, and `IsWaitingForPlayerChoiceResponse()` expose local query state. `SendPlayerChoiceResponse()`, `RequestRerollPlayerChoice()`, and `OnUIClosed()` record local mutator intent. This does not model retail timing, server validation, reroll economics, or live service values.

**Spell descriptions** — `C_Spell.GetSpellDescription()` and `C_TooltipInfo.GetSpellByID()` both route through `src/spell_description_resolver.rs` before text reaches Lua or tooltip lines. The resolver expands Blizzard DB2-style tokens such as `$s1`, `$<damage>`, `$<dmg>`, `$<shield>`, `${...}` arithmetic, `$STR`, `$INT`, `$AP`, `$MHP`, and simple conditional control tokens against `SimState` player stats and the simulator's spell-effect model. AP-scaled Paladin/Demon Hunter formulas are grounded in the local SimulationCraft dump (`~/Repos/simc/SpellDataDump/allspells.txt`): Avenger's Shield uses `1.55 * AP`, Crusader Strike `1.4 * AP`, Shield of the Righteous `0.95 * AP`, Eye Beam `$<dmg>` uses `10 * 0.4026 * AP`, and Shield of Vengeance `$<shield>` uses `30% max health * (1 + versatility damage)`. This keeps spellbook/tooltips from showing raw `$...` placeholders and prevents tooltip-only one-off replacements from drifting away from the C API surface.

## Timer System

`C_Timer.After(seconds, cb)`, `C_Timer.NewTicker(seconds, cb, iterations)` — backed by `WowLuaEnv.schedule_timer()`. `process_timers()` fires due callbacks each tick. `next_timer_delay()` drives the event loop scheduling.

## Animation System

`CreateAnimationGroup()` returns a group supporting `Play()`, `Stop()`, `Pause()`, `SetLooping()`, and `SetScript("OnFinished")`. Animation types: Alpha, Translation, Scale, Rotation, FlipBook, VertexColor, Path. `fire_on_update()` ticks animation groups after OnUpdate handlers.

## Sources

- [lua-api.md](../../lua-api.md) — WowLuaEnv, FrameHandle, method categories, globals, C_* namespaces, timers
- [spell_description_resolver.rs](../../../src/spell_description_resolver.rs) — shared spell-description token resolver
- [container_portrait_texture.rs](../../../src/lua_api/workarounds/temporary/container_portrait_texture.rs) — retail texture fileDataID proof
- [item_button_helper_defaults.rs](../../../src/lua_api/workarounds/temporary/item_button_helper_defaults.rs) — item-button texture fileDataID proof
- [c_chromie_time.rs](../../../src/c_api/c_chromie_time.rs) — retail/PTR empty-state C_ChromieTime surface
- [loader_env.rs](../../../src/lua_api/loader_env.rs) — secure versus public dynamic loader execution
- [auth_challenge_frame_parent.rs](../../../src/lua_api/workarounds/temporary/auth_challenge_frame_parent.rs) — addon-specific AuthChallenge public export bridge
- `/home/osso/Repos/simc/SpellDataDump/allspells.txt` — coefficient and variable formulas used for AP/health-scaled spell text

## See Also

- [[frame-data-flow]] — method lookup order, __index/__newindex, Mixin() application
- [[event-system]] — fire_event, SetScript, OnUpdate tick mechanism
- [[widget-system]] — Frame struct backing each FrameHandle
- [[texture-atlas]] — texture path resolution, atlas identity, and rendering consumers
