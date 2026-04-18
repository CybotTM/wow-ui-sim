# Startup XML Fast Path

This note explains the XML loader fast path that was added to cut startup cost during Blizzard UI load. It complements the deeper investigation in [`docs/wiki/investigations/startup-createframe-profile.md`](docs/wiki/investigations/startup-createframe-profile.md).

## Goal

The original XML startup path treated most inline `<Scripts>` bodies as generic Lua:

1. generate Lua source for the body
2. compile a closure
3. install it on the frame
4. execute it later through normal Lua dispatch

That is flexible, but expensive at startup. The fast path keeps semantics for a narrow, verified subset of XML script bodies while avoiding the generic compile path.

## How It Works

The fast path has three stages:

1. Parse the inline body into a known-safe shape.
2. Lower that shape into a `FastHandlerRef` enum variant.
3. Install a small cached wrapper closure instead of compiling arbitrary generated Lua.

The parser lives under:

- `src/lua_api/globals/create_frame/template_chain/parser.rs`
- `src/lua_api/globals/create_frame/template_chain/parser_method_family.rs`
- `src/lua_api/globals/create_frame/template_chain/parser_global_family.rs`
- `src/lua_api/globals/create_frame/template_chain/parser_function_family.rs`

The lowered representation is:

- `src/lua_api/globals/create_frame/template_chain.rs`

The wrapper builders live under:

- `src/lua_api/globals/create_frame/template_chain/builders.rs`
- `src/lua_api/globals/create_frame/template_chain/builder_method_family.rs`
- `src/lua_api/globals/create_frame/template_chain/builder_global_family.rs`
- `src/lua_api/globals/create_frame/template_chain/builder_function_family.rs`

If a script body does not match a supported shape, the loader falls back to the old generic path.

## What Kinds Of Bodies Match

The fast path currently covers a large set of exact and semi-structured shapes, including:

- direct method handlers such as `self:Method()`
- global function calls such as `Foo(self)` or `Foo("text")`
- parent and grandparent method calls
- safe inline assignments
- safe fixed-length sequences
- selected conditional forms
- selected tooltip patterns
- selected nested function-result argument forms

Examples of shapes that are now fast-installed:

```lua
self:OnClick()
self:GetParent():GetParent():SetDisabledStateOnCommunityFinderOptions(not self:GetChecked())
GameTooltip:SetOwner(self, "ANCHOR_RIGHT")
GameTooltip:SetText(MicroButtonTooltipText(CHARACTER_INFO, "TOGGLECHARACTER0"), 1.0, 1.0, 1.0)
if ( PetitionFrame.petitionType == "guild" ) then
    StaticPopup_Show("RENAME_GUILD")
end
PetBattleAbilityTooltip_SetAura(Enum.BattlePetOwner.Weather, PET_BATTLE_PAD_INDEX, 1)
PetBattleAbilityTooltip_Show("TOP", self, "BOTTOM", 0, 0)
```

Recent structural additions also cover:

- `stmt; if ... end stmt`
- `if GlobalFn(OtherFn()) then ... end`
- `local text = self:GetText(); if text and #text > 0 then parent:Method(self:GetText()); self:SetText(""); end`

## Measurement

Two signals matter:

### 1. Wall-clock startup

The user-supplied earlier debug sample on `--no-saved-vars --no-addons` was about:

- `8.23s` addon load time

Best clean runs after the XML loader fast-path campaign reached roughly:

- `4.18s` addon load time

That is about a `4.0s` reduction, or roughly `49%` faster on that path.

Wall-clock remains noisy in shared-worktree runs, so it should be read together with the fast-path counters.

### 2. XML fast-path counters

The loader can print:

- `hits`
- `slow`
- `total`
- `scripts` misses

Early in the campaign, a representative state was:

- `hits=942`
- `slow=1276`
- `scripts=964`

Current profiled state after the latest verified changes is:

- `hits=1961`
- `slow=257`
- `total=2218`
- `scripts=140`

That means the campaign eliminated about:

- `824` script-shape misses
- and added about `1019` fast-path installs

## Current Profile Snapshot

Latest profiled startup on the shared debug tree:

- addon load `6.20s`
- `xmlproc=4.06s`
- `setup=2.05s`
- `exec_lua=2.01s`
- `finalize=4.47s`
- `lifecycle=1.57s`
- `xml fast path: hits=1961 slow=257 total=2218 misses: scripts=140, no_explicit_parent=114, xml_attributes=8, root_frame_reuse=2`

This run was not fully clean. It also hit unrelated CVar/API gaps:

- `RegisterCVar`
- `GetCVarBitfield`

Those errors do not invalidate the fact that the fast-path counter moved, but they do make the wall-clock less trustworthy than a fully clean startup run.

## What The Fast Path Saved

The savings came from reducing:

- Lua source generation during XML load
- Lua chunk compilation during XML load
- temporary allocation churn around generated script installation
- repeated template-chain work that could be lowered to cached wrappers

In practice, the biggest visible reduction was in:

- `xmlproc`
- `setup`
- `exec_lua`

The fast path does not help every startup bucket. `finalize` and `lifecycle` are separate costs.

## Current Remaining Miss Families

The remaining `scripts=140` misses are no longer dominated by trivial shapes. The visible top families are now mostly tooltip-heavy conditionals and branchy handlers, for example:

- GuildControl checkbox confirmation branch
- truncation-based tooltip bodies
- tooltip bodies with extra `Show()` or `AddLine(...)`
- richer conditional tooltip sequences

The next best wins are expected to come from widening tooltip families rather than adding more one-off exact handlers.

## How To Reproduce

Build first:

```bash
cargo build --bin wow-sim
```

Then profile the XML fast path:

```bash
WOW_SIM_PROFILE_XML_FAST_PATH=1 cargo run --bin wow-sim -- --no-saved-vars --no-addons lua-errors
```

The loader prints the counter line and the top script misses to stderr. JSON Lua errors, if any, are emitted to stdout.

## Safety Rule

The fast path is intentionally conservative:

- only verified shapes should be accepted
- any uncertain body must fall back to the generic path
- proof requires both focused tests and a live startup rerun

That tradeoff is what kept the campaign from turning into a broad semantic regression across Blizzard XML.
