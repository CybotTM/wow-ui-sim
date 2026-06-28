# FontVirtualProbe

Captures **real-client ground truth** for whether top-level XML `<Font>`
definitions publish global `Font` objects when `virtual="true"` is present.

## Why

XML virtual frame-like templates are usually template-only and do not publish a
live `_G.Name` object. Fonts are special enough that static docs are ambiguous:
`virtual="true"` can mean "usable as an XML inheritance template", but that does
not by itself say whether the named font is also globally referenceable.

wow-ui-sim currently creates a live `Font` object for every top-level `<Font>`
regardless of `virtual`, so live-client proof is needed before treating that as a
compatibility requirement.

## What it probes

The addon declares:

- `FontVirtualProbeVirtualFont`: `<Font virtual="true">`
- `FontVirtualProbeConcreteFont`: `<Font>` without `virtual`
- one `<FontString>` inheriting each font

On `PLAYER_LOGIN`, it writes `FontVirtualProbeDB` (SavedVariables) with:

- `_G.<fontName>` existence, Lua type, object type, object name, and font path
- `FontString:GetFontObject()` identity vs the expected global font object
- inherited `FontString:GetFont()` path/height/flags

## Run it

1. Install + enable (see `../create-and-install-wow-addon.md`).
2. Log in.
3. `/reload` or log out so SavedVariables flush.
4. Pull `FontVirtualProbeDB` back from `WTF/Account/<ACCOUNT>/SavedVariables/FontVirtualProbe.lua`.

## Captured result (client 12.0.7.68275, interface 120007, 2026-06-28)

```
virtualFont:
  exists      = true
  luaType     = table
  objectType  = Font
  objectName  = FontVirtualProbeVirtualFont
  fontPath    = Fonts\FRIZQT__.TTF

concreteFont:
  exists      = true
  luaType     = table
  objectType  = Font
  objectName  = FontVirtualProbeConcreteFont
  fontPath    = Fonts\FRIZQT__.TTF

virtualString:
  objectType                      = FontString
  objectName                      = FontVirtualProbeVirtualString
  fontObjectName                  = FontVirtualProbeVirtualFont
  fontObjectSameAsExpectedGlobal  = true
  fontPath                        = Fonts\FRIZQT__.TTF
  fontHeight                      = 16.99999809265137
  fontFlags                       = ""

concreteString:
  objectType                      = FontString
  objectName                      = FontVirtualProbeConcreteString
  fontObjectName                  = FontVirtualProbeConcreteFont
  fontObjectSameAsExpectedGlobal  = true
  fontPath                        = Fonts\FRIZQT__.TTF
  fontHeight                      = 19
  fontFlags                       = ""
```

### Conclusion

`<Font name="X" virtual="true">` **does publish a live global Font object** in
retail 12.0.7. `virtual="true"` does not suppress `_G.X` for fonts.

The virtual font also works as an inheritance source: the `<FontString>` that
inherits it reports the same object from `GetFontObject()` as `_G.X`.
