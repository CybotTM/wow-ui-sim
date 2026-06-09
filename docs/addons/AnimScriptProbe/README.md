# AnimScriptProbe

Captures **real-client ground truth** for which script handlers the live WoW
engine accepts on animation objects, so it can be compared against schemas
(`UI.xsd` `AnimScriptsType`), models (wowless `uiobjects`), and wow-ui-sim.

## Why

Neither Blizzard's `UI.xsd` (`AnimScriptsType` / `AnimGroupScriptsType`) nor
wowless's runtime model list `OnEvent` for animations. A claim surfaced that the
*engine* nonetheless accepts `OnEvent` on animation objects. The authoring schema
is not a complete runtime contract, so the only way to settle it is to probe the
live client.

wow-ui-sim cannot answer this: its `SetScript` is ungated (accepts every common
handler on any widget, including animations) and its `HasScript` on animation
containers reports *binding presence*, not *type support*. So it returns
`SetScript=true` / `HasScript=false` for everything — useless as ground truth.

## What it probes

For a `Frame`, an `AnimationGroup`, and every `Animation` subtype
(`Alpha`, `Translation`, `Scale`, `Rotation`, `LineTranslation`, `LineScale`,
`Path`, `FlipBook`, `VertexColor`), it records — per handler in
`{OnLoad, OnUpdate, OnEvent, OnPlay, OnPause, OnStop, OnFinished, OnLoop,
OnShow, OnHide, OnEnter, OnLeave}`:

- `HasScript(name)` — the engine's own "is this handler supported here" answer.
- whether `SetScript(name, fn)` succeeds (errors on unsupported handlers).

Results are written to `AnimScriptProbeDB` (SavedVariables) as a full matrix, and
a summary line is printed at `PLAYER_LOGIN` and on `/animprobe`.

## Run it

1. Install + enable (see `../create-and-install-wow-addon.md`).
2. Log in, or run `/animprobe`.
3. `/reload` or log out so SavedVariables flush.
4. Pull `AnimScriptProbeDB` back and compare (see the "Read SavedVariables Back"
   section of the create-and-install doc).

## Captured result (client 12.0.5.67823, interface 120005, 2026-06-08)

`HasScript` on an unsupported handler **errors**; on a supported one returns
`true`. `SetScript` succeeds only on supported handlers. They agreed exactly, and
all 9 Animation subtypes returned identical results.

| Handler | Animation (all 9 subtypes) | AnimationGroup | Frame (control) |
|---|:--:|:--:|:--:|
| OnLoad | ✓ | ✓ | ✓ |
| OnUpdate | ✓ | ✓ | ✓ |
| OnPlay | ✓ | ✓ | ✗ |
| OnPause | ✓ | ✓ | ✗ |
| OnStop | ✓ | ✓ | ✗ |
| OnFinished | ✓ | ✓ | ✗ |
| OnLoop | ✗ | ✓ | ✗ |
| OnEvent | ✗ | ✗ | ✓ |
| OnShow / OnHide / OnEnter / OnLeave | ✗ | ✗ | ✓ |

### Conclusion

- **The engine does NOT accept `OnEvent` on animations or animation groups.**
  `HasScript(OnEvent)` errors and `SetScript(OnEvent)` fails on both. Any claim
  that the engine accepts `OnEvent` on animations is false for this build.
- Animation shared set = {OnLoad, OnUpdate, OnPlay, OnPause, OnStop, OnFinished}
  (6 handlers). AnimationGroup = that set **+ OnLoop**.
- Blizzard's `UI.xsd` (`AnimScriptsType` / `AnimGroupScriptsType`) matches this
  exactly. wowless's `uiobjects` model is incomplete — it omits OnLoad and
  OnPause, which the engine does support.
