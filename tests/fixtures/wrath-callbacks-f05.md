# Wrath callback fixture F05

The native fixture loads the committed Wrath FrameXML TOC via the F04 builder,
then invokes original callbacks. Non-Wrath bodies remain unchanged.

## Native consumer coverage

- `MainMenuBar.lua`: native main bar registration, page-number refresh, vehicle-leave registration (three events, not Retail's five).
- `ActionButton.lua`, `ActionBarFrame.xml`, `MultiActionBars.xml`: named CheckButtons and direct parent chains; four multi-bars, no modern button containers. Native input globals exclude TryUseActionButton and ExtraActionButtonKey.
- `BonusActionBarFrame.lua`, `PetActionBarFrame.lua`, `VehicleMenuBar.lua`: native stance/pet event sets, stance checked state refresh, vehicle power refresh.
- `CastingBarFrame.lua`: CastingBarFrame rather than PlayerCastingBarFrame; original start/delay/failure/interrupt callbacks, native text global.
- `BuffFrame.lua`: UNIT_AURA refresh, BUFF_ACTUAL_DISPLAY and BuffButton1; AuraButton_OnUpdate duration formatting, color/alpha, duration visibility and 500-microsecond steady-state budget. Native duration has no modern font-threshold/anchor rewrite; that part remains Retail-only.

Consumer evidence is the shipped 12340 UI. It proves what the fixture calls, not
native producer contracts. Producer corrections require separate native-contract
evidence and an observed failure through those consumers.

## Report-only gate candidates

These original tests remain registered and unchanged on Wrath. Their modern-only
assertions have no native counterpart; failure is reported, not cfg-gated.

- File-scope mixin declarations; seven surface_mixins tests. Wrath uses free functions, not these mixins.
- ActionBarActionEventsFrame and ActionBarButtonEventsFrame registration; RegisterFrame event-routing surface. Wrath registers directly on buttons.
- SpellFlyout; extra-bar helpers; highlight tracking; artifact/paragon bar helpers. These modern UI systems are absent from 12340.
- Cast bar EditMode lock-to-player setting. Wrath has no EditMode system.

Partial modern-only assertions are not recreated: neutral-faction selection,
MultiBar5/6/7, modern button containers, extra-action input and modern multibar
query helpers. Native coverage above retains the shared input/event/state intent.

## Limits

No vendor functions are replaced. Timed aura measurements use a supplied aura and
call the actual installed OnUpdate handler; they do not model native timing.
Undefined native producer behavior is a gap, not an invented model.
