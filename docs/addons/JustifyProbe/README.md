# JustifyProbe

JustifyProbe captures live WoW behavior for unanchored justified FontStrings.

It specifically tests:

- direct unanchored `FontString` children in a frame layer;
- the implicit `Button.ButtonText` `FontString`;
- named FontString elements inside EditBox definitions;
- EditBox, MessageFrame, and ScrollingMessageFrame owner/FontString-region probes;
- no-inset and explicit `TextInsets` owner variants;
- direct and button text variants with no `<Size>`, width-only `<Size>`, height-only `<Size>`, and width+height `<Size>` where relevant.
- explicit vertical-only anchors (`TOP`, `BOTTOM`) to distinguish "no points" from "not horizontally anchored";
- explicit horizontal anchors (`LEFT`, `RIGHT`, `TOPLEFT`) as controls.

## Install

Copy the folder to:

```text
World of Warcraft/_retail_/Interface/AddOns/JustifyProbe/
```

## Run

Log in and the addon captures once automatically after `PLAYER_LOGIN`.
It captures again on `PLAYER_LOGOUT`, so `/reload` or logging out persists the latest probe to SavedVariables.

Manual commands:

```text
/justifyprobe
/jprobe
```

Results are saved to:

```text
WTF/Account/<ACCOUNT>/SavedVariables/JustifyProbe.lua
```

The latest run is in:

```lua
JustifyProbeDB.latest
```

Historical runs are kept in:

```lua
JustifyProbeDB.runs
```

Each probe records object name, `GetNumPoints()`, each `GetPoint(i)`, width, height, `GetJustifyH()`, `GetJustifyV()`, and text.

## Expected Use

Run this in retail, then compare:

- whether unanchored frame-layer FontStrings get a default single anchor from horizontal justification;
- whether `Button.ButtonText` gets the same default anchor behavior;
- whether EditBox FontString children do not get that default behavior;
- whether explicit `<Size>` width/height changes anchor generation or only dimensions.
- whether vertical-only anchors still trigger default horizontal anchoring.
- whether EditBox/MessageFrame/ScrollingMessageFrame FontString regions skip the default anchor behavior;
- whether MessageFrame/ScrollingMessageFrame expose text through regions at all;
- whether owner `TextInsets` affect those FontString region anchors.
