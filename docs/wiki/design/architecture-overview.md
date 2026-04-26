# Architecture Overview

The wow-ui-sim project runs WoW addon Lua outside the game for testing and visual preview. It embeds Lua 5.1 (WoW's exact version) via rilua — a pure-Rust Lua VM with WoW-style taint tracking built in — inside a Rust host, which also handles rendering through the iced GUI framework.

## Goals and Non-Goals

**Primary goal**: Run WoW addons headlessly for CI testing and development. **Secondary goal**: Visual preview of addon UI.

**Non-goals**: full game emulation (combat, spells, inventory), network connectivity, audio, or taint enforcement (security system is stubbed as always-secure).

## Lua + Rust Dual System

WoW frames live in two parallel systems that must stay in sync:

- **Rust side** (`widget::Frame` + `WidgetRegistry`) — owns layout computation, rendering, and the frame tree. Each frame is a `u64`-keyed entry in a `HashMap`.
- **Lua side** (`FrameHandle` userdata + metatables) — exposes the WoW API to addon Lua code. Each `FrameHandle` stores the same `u64` ID pointing to the Rust frame.

Method calls like `:SetText()` use the ID to update Rust state directly. Child assignments (`parent.Child = frame`) are intercepted by `__newindex` to sync `children_keys` on the Rust side, enabling fast HashMap lookups without querying Lua.

## Module Layout

```
Rust Host
├── lua_api/    — WoW globals, C_* namespaces, frame methods
├── widget/     — Frame struct, WidgetRegistry, anchor system
└── render/     — iced canvas, texture loading, text rendering
```

## Phases

| Phase | Status | Description |
|-------|--------|-------------|
| 1 | Complete | Core Lua environment: CreateFrame, events, global aliases, Mixin system |
| 2 | Complete | Widget API: alpha, strata, anchors, parent-child, Texture/FontString |
| 3 | Complete | Addon loading: TOC parser, XML templates, Blizzard_SharedXML |
| 4 | Complete | Rendering: iced canvas, z-ordering, text, BLP/PNG textures |
| 5 | In Progress | Real addon testing: 127+ addons load, missing API stubs being filled |

## Design Decisions

- **Lua 5.1 via rilua**: pure-Rust Lua 5.1 VM that matches WoW's Lua version and provides Elune-style taint tracking natively (no C runtime).
- **Taint tracking**: rilua exposes `debug.setobjecttaint` / `debug.getstacktaint` and propagates stack taint through `CallInfo`. `issecure()` is provided by the simulator (Lua bootstrap fallback when not pre-registered); `issecurevariable` and `securecallmethod` are Rust implementations in `src/lua_api/globals/security.rs`.
- **Auto-generated stubs**: `generated_stubs.rs` (~19K lines) catches all C_* functions not yet hand-written, returning nil/false/0. Hand-written Rust wins via `is_nil()` guard.
- **XML templates**: loaded before user addons; Blizzard_FrameXML is deferred to Phase 5 due to API gaps.

## Sources

- [DESIGN.md](../../../DESIGN.md) — goals, phase list, module diagram
- [AGENTS.md](../../../AGENTS.md) — Lua+Rust dual system, taint system, performance notes

## See Also

- [[scaling-coordinates]] — coordinate system and canvas sizing
- [[addon-compatibility]] — addon loading pipeline and tested addons
- [[api-coverage]] — C_* stub coverage and missing namespaces
- [[development-phases]] — current active work
