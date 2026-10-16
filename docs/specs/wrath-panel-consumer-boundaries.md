# Wrath panel consumer boundaries

Wrath cached FrameXML must receive its required constants, XML region spelling,
and nested frame initialization. These are bounded consumer compatibility
requirements, not a claim of whole-panel operation or measured native parity.
Tests read unchanged profile-cache bytes and require their pinned SHA-256 values.

## What it must do

### GameTime

- [ ] Publish `PI == math.pi` before the original `GameTime.lua` captures it.
- [ ] Complete that chunk and format explicit 13:07 and midnight inputs using
  deterministic localized strings and both military-time CVar settings.

### Floating chat XML

- [ ] Parse the entire original `FloatingChatFrame.xml`, including its observed
  `Fontstring` spelling, while preserving canonical `FontString` support.
- [ ] Construct the original `FriendsMicroButton` subtree unchanged: its count
  region starts at `0`, then its own OnLoad displays `5` for two BNet and three
  WoW online friends. The named region remains a child FontString.

### Options frame construction

- [ ] Load original `OptionsFrameTemplates.xml`, then `AudioOptionsFrame.xml`
  through normal XML/addon loading; initialize dynamically created category
  buttons even while inherited static child OnLoads are executing. The hidden
  XML root must not force these public dynamic buttons to be initially hidden.
- [ ] Each original parent/category/button initializer executes once, with named
  ButtonText and toggle regions wired before use. The outer XML lifecycle must
  not suppress unrelated synchronous public CreateFrame calls.
- [ ] The same original AudioOptionsFrame subtree works with a loader-supplied
  explicit UIParent, covering direct construction as well as generated Lua.
- [ ] Original category addition displays `Sound` at left offset 8 and its expanded
  child `Effects` at offset 16 without captured callback or loader errors.

## How it works

- [XML template lifecycle](../xml-template-system.md)
- [Addon loading](../addon-loading-pipeline.md)
- [Frame representation and dispatch](../frame-data-flow.md)

## Implementation inventory

- `tests/wrath_panel_boundaries.rs`: Wrath-only original-consumer regressions,
  deterministic external inputs, callback observations and cache requirements.
- `src/wrath/compat_bootstrap.lua`: guarded Wrath PI alias, installed before vendor chunks.
- `src/xml/types_elements.rs`: narrow LayerElement deserialize alias, with no case folding.
- `src/loader/xml_frame/setup.rs`: XML-owned creation flags scoped/restored on both paths.
- `src/lua_api/globals/create_frame/mod.rs`: consumes those flags before template
  callbacks; passes the root lifecycle decision explicitly to template application.
- Producer patch is staged, not compiled or executed; all acceptance boxes stay open.

## Tests asserting this spec

`tests/wrath_panel_boundaries.rs` contains four original-consumer tests: GameTime,
FloatingChatFrame, normal AudioOptionsFrame XML, explicit-parent AudioOptionsFrame.
All are gated by `client-wrath`; no new value conversion implementation is needed.

## Known gaps (current cycle)

- [ ] Compile and execute tests-only RED, then producer GREEN on the pinned Wrath
  profile. Current handoff is source-only and explicitly prohibits builds.
- [ ] Capture startup errors separately before claiming any downstream reduction.

## Out of scope

- Arbitrary XML case folding: only the exact shipped alias is demonstrated.
- Full chat startup: the test parses the complete file but constructs its original
  FriendsMicroButton subtree with deterministic unrelated count/anchor inputs.
- Skill invalid-index tuples, legacy sound kit-name translation, who-result routing,
  and map-landmark projection: native contracts remain unverified in this handoff.
- Blizzard/vendor Lua edits, reconstructed clients, rendering fixes and fallbacks.
