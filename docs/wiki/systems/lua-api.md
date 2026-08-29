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

Enum initialization seeds known `Enum.*` values into existing child tables rather than replacing those tables. This matters after Blizzard Lua extends an enum during addon loading: later runtime-surface restoration reseeds required values without deleting Blizzard-added members such as CooldownViewer categories.

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

Widget-specific: EditBox (SetMultiLine, SetAutoFocus), Slider (SetMinMaxValues, SetValue, SetOrientation), StatusBar (SetStatusBarColor), Cooldown (SetCooldown), Tooltip (SetOwner, AddLine, AddDoubleLine), MessageFrame (AddMessage), Browser (`NavigateTo`, `NavigateHome`). Browser navigation methods are callable no-result compatibility methods; the simulator does not open external content.

Model-family widgets (`Model`, `ModelScene`, `PlayerModel`, and related model frames) expose the Lua surface needed by Blizzard code, but 3D rendering is intentionally out of scope. Visual-only calls such as `ClearFog` are callable no-ops; modeled object state and actor methods remain separately documented where supported.

### Texture identity

For known WoW texture paths resolved by the bundled texture manifest, `Texture:GetTexture()` and `Texture:GetTextureFileID()` return the numeric fileDataID, while `Texture:GetTextureFilePath()` preserves the source path. Use `GetTextureFilePath()` when an assertion needs the authored path rather than the numeric texture identity. Current proofs include `Interface\\TargetingFrame\\UI-Classes-Circles` → `237669` and `Interface\\ICONS\\INV_Misc_QuestionMark` → `134400`.

## Global Functions

**Core overrides** — `print` appends to `SimState.console_output`; `ipairs` iterates frame children; `getmetatable` returns a fake metatable exposing all frame methods; `string.format` maps `%F` → `%f` for LuaJIT compatibility.

**CreateFrame** — Parses type/name/parent/template, registers frame, links parent-child, inherits strata/level, creates widget type defaults, applies templates, returns FrameHandle.

**CreateWindow** — Returns a frame-backed `SimpleWindow` for Blizzard external-tool panels. The second argument initializes topmost state; `SetWindowSize`/`SetMinSize` enforce dimensions, `IsTopmost`/`SetTopmost` expose the modeled flag, and `Close` hides the frame. `SetTitle` and `SetFocus` are callable no-ops; popup-style, position, and focus persistence are not modeled. Owner frames use `SetWindow`/`GetWindow` and ordinary anchoring.

**Font system** — `CreateFont()`, standard fonts (GameFontNormal, ChatFontNormal, SystemFont_Small, etc.) stored as Lua tables with canonical `__fontPath`, `__fontHeight`, and `__fontFlags` keys. FontString snapshots prefer those canonical fields and retain legacy aliases (`__font`, `__height`, `__outline`) only as fallback, so XML `inherits="GameFontNormalLarge"` preserves the inherited size and flags.

**Versioned UI strings** — `register_all_ui_strings()` combines generated global strings with static compatibility data. The live enUS retail 12.1 slice registers exactly 32 probe-proven scalar strings only under `profile-retail` + `retail-12-1-0`; 12 probe-proven nil globals remain absent rather than receiving placeholders or aliases.

**Object pools** — `CreateFramePool`, `CreateFrameFactory` (multi-template), `CreateObjectPool` (generic acquire/release).

**Utilities** — `wipe()`, `tinsert/tremove()`, `CopyTable()`, `MergeTable()`, `Mixin()`, `CreateFromMixins()`, `getglobal/setglobal()`, `loadstring()`, `strsplit()`. Both `string.split(...)` and string `:split(...)` accept Blizzard's delimiter-receiver form, including empty and equal-length punctuation inputs, while retaining ordinary input-first behavior.

**Security** — `issecure()`, `securecall()`, `securecallfunction()`, `securecallmethod()`, `forceinsecure()`, `hooksecurefunc()` (from Elune or fallback stubs). The read-only `debug.isglobalindex()` query reports whether the active `_G.__index` invocation came from a syntactic global load; its VM-scoped provenance excludes explicit `_G` table reads and restores across nesting, errors, and coroutine swaps. With no modeled click-binding profile, `C_ClickBindings.GetBindingType()` returns `Enum.ClickBindingType.None` and `ExecuteBinding()` is inert, allowing secure-button `type` attributes to dispatch normally.

**Modeled legacy globals** — `ClearTarget()` clears the current target and returns `true` iff a target existed; it returns `false` when no target was set and preserves the `PLAYER_TARGET_CHANGED` event. `IsTimerunningEnabled()` and `GetRemainingTimerunningSeasonSeconds()` read the simulator's Timerunning season state; the countdown is zero when no season is active. `GetGuildTabardFiles()` is registered on retail as well as classic profiles and returns the modeled guild-tabard file tuple used by Blizzard's guild-bank UI. The temporary `Kiosk` namespace defaults `IsEnabled()` and `IsCompetitiveModeEnabled()` to `false` and `GetKioskLoginInfo()` to three `nil` values while preserving existing members.

**Focused compatibility boundaries** — `EJ_SetLootFilter()` accepts integer Lua values and numeric strings locally; nil, invalid, and non-integral inputs become zero without changing global argument coercion. Chat startup assigns `DEFAULT_CHAT_FRAME` whenever `ChatFrame1` exists, while `ChatFrame1.editBox` remains conditional on `ChatFrame1EditBox`. Temporary chat-window state also backs `SetChatWindowName()` and `SetChatWindowDocked()` round-trips through `GetChatWindowInfo()`; these fields remain in the compatibility table until saved chat-layout state is modeled.

## C_* Namespaces

C_Timer (After, NewTimer, NewTicker), C_Map (stub), C_Item (`IsConsumableItem`, `IsEquippableItem`, and `IsItemInRange` are state-backed; other listed item methods remain mixed implemented/stubbed), C_System (GetLocale → "enUS"), C_EditMode (GetLayouts), C_CatalogShop (`GetVCProductInfos()` returns a fresh empty table), C_ChromieTime (retail/PTR empty-state queries and no-op actions), C_StringUtil (`EscapeDecimalNonPrintables` preserves valid UTF-8 and replaces ASCII control bytes except tab/newline/carriage return, plus invalid UTF-8 bytes, with decimal escapes), C_Quest, C_AchievementInfo, C_ClassTalents, C_Guild, C_LFGList, C_Mail, C_ActionBar — most return nil/false/0 stubs.

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
- rilua commits `1a7c9de` and `3630419` — VM-scoped syntactic-global lookup provenance and `debug.isglobalindex()`
- [auth_challenge_frame_parent.rs](../../../src/lua_api/workarounds/temporary/auth_challenge_frame_parent.rs) — addon-specific AuthChallenge public export bridge
- [enums.rs](../../../src/lua_api/env_init/enums.rs) — enum-table reseeding that preserves existing Blizzard extensions
- [strings/mod.rs](../../../src/lua_api/globals/strings/mod.rs) — generated and versioned UI-string registration
- [more_strings.rs](../../../src/lua_api/globals/strings/string_data/more_strings.rs) — live retail 12.1 string data table
- [register.rs](../../../src/lua_api/globals/register.rs) — exact live GlobalStrings contract test
- [timerunning.rs](../../../src/lua_api/globals/real/timerunning.rs) — state-backed legacy Timerunning globals
- [bank_storage_verbs.rs](../../../src/lua_api/globals/bank_storage_verbs.rs) — retail guild-tabard lookup registration
- [c_string_util_decimal.rs](../../../src/c_api/c_string_util_decimal.rs) — decimal escaping for control and invalid UTF-8 bytes
- [font_strings.rs](../../../src/lua_api/frame/methods/button_anchor_hierarchy/font_strings.rs) — canonical Font object field precedence and FontString snapshots
- [chat_window_defaults.rs](../../../src/lua_api/workarounds/temporary/chat_window_defaults.rs) — temporary chat-window name/docking state and public round-trip defaults
- [compat_overrides.rs](../../../src/lua_api/globals/compat_overrides.rs) — table-form `string.split` compatibility
- [formatting_utility_defaults.rs](../../../src/lua_api/workarounds/temporary/formatting_utility_defaults.rs) — string-metatable `:split` compatibility
- [click_bindings_defaults.rs](../../../src/lua_api/workarounds/temporary/click_bindings_defaults.rs) — no-profile click-binding behavior
- [loot.rs](../../../src/lua_api/globals/missing_surface/encounter_journal/loot.rs) — local Encounter Journal filter coercion
- [chat_init.rs](../../../src/lua_api/chat_init.rs) — default chat-frame initialization
- [browser.rs](../../../src/lua_api/frame/methods/widgets/browser.rs) — no-result Browser navigation methods
- [model.rs](../../../src/lua_api/frame/methods/widgets/model.rs) — model-family Lua surface and intentional 3D visual no-ops
- [widget_methods_model.rs](../../../tests/widget_methods_model.rs) — `ClearFog` publication and no-op behavior proof
- [simple_window.rs](../../../src/lua_api/globals/create_frame/simple_window.rs) — frame-backed `CreateWindow` compatibility contract
- [render_layers.rs](../../../src/lua_api/frame/methods/misc/render_layers.rs) — `SetWindow`/`GetWindow` owner attachment
- [kiosk_namespace_defaults.rs](../../../src/lua_api/workarounds/temporary/kiosk_namespace_defaults.rs) — inert Kiosk defaults
- [targeting_verbs.rs](../../../src/lua_api/globals/targeting_verbs.rs) — state-backed targeting globals, including `ClearTarget()`'s boolean result
- [targeting_verbs.rs](../../../tests/targeting_verbs.rs) — targeting global behavior proofs
- `/home/osso/Repos/simc/SpellDataDump/allspells.txt` — coefficient and variable formulas used for AP/health-scaled spell text

## See Also

- [[frame-data-flow]] — method lookup order, __index/__newindex, Mixin() application
- [[addon-loading]] — addon execution, idempotent loaded-addon handling, and environment boundaries
- [[taint-system]] — secure/public environments and secure-button publication boundaries
- [[post-load-workaround-audit]] — explicit post-cleanup restoration hooks
- [[event-system]] — fire_event, SetScript, OnUpdate tick mechanism
- [[widget-system]] — Frame struct backing each FrameHandle
- [[texture-atlas]] — texture path resolution, atlas identity, and rendering consumers
