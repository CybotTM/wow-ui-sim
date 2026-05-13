# CASC Known-Good Root Debug Probes

FDID `1579624` is a known-good Blizzard UI file probe. Font FDID `615960` was present in one older local CASC resolution cache, but newer retail caches can miss the standard font FDIDs and fall back to path-based reads. If another root parser cannot find these FDIDs, check MFST/root parsing, locale/content filtering, build/cache selection, and path casing before treating the listfile entry as authoritative.

## Content

### FDID 1579624: BaseAPIMixin.lua

Local resolution cache:

```text
cache: ~/.cache/asset-resolver/casc/wow/7b498dd7e196bf4161d631064f617189/resolution.sqlite
fdid: 1579624
path: interface/addons/blizzard_apidocumentation/baseapimixin.lua
Blizzard path: Blizzard_APIDocumentation/BaseAPIMixin.lua
content key: 51812203898E1E22B9927DDB95AF0897
encoding key: 888BDED7A0E91FAF8A1AE2368DA1A35E
size: 2641 bytes
sha256: 8A7DEEC6860C5C1DF52F938ADAA7C97312DE8666FA3DAB43B643857815F72103
line endings: CRLF
```

The local listfiles agree:

```text
data/wow-ui-sim-listfile.csv:
1579624;interface/addons/blizzard_apidocumentation/baseapimixin.lua

data/full-community-listfile.csv:
1579624;interface/addons/blizzard_apidocumentation/baseapimixin.lua

data/blizzard-ui-files.txt:
Blizzard_APIDocumentation/BaseAPIMixin.lua
```

### Extraction Proof

`asset-resolver` can extract the file locally:

```text
/home/osso/Projects/world-of-osso/asset-resolver/target/debug/casc-local 1579624 -o /tmp/fdid-1579624-check

CASC: extracted FDID 1579624 -> /tmp/fdid-1579624-check/1579624.lua
1 extracted, 0 failed
```

The extracted file is ASCII Lua with CRLF line endings. The first lines are:

```lua
BaseAPIMixin = {};

function BaseAPIMixin:GetType()
	return assert(false);
end
```

### FDID 615960: Friz Quadrata Font

The standard WoW font `FRIZQT__.TTF` resolved and extracted in the older cache below. This is no longer a stable known-good probe: on the May 13, 2026 local retail cache, FDIDs `615960`, `615958`, and `615971` were absent from `resolution.sqlite`, and path fallback failed when the cached/listfile path was lowercase (`fonts/frizqt__.ttf`, `fonts/arialn.ttf`, `fonts/frizqt___cyr.ttf`).

```text
fdid: 615960
path: fonts/frizqt__.ttf
content key: 87CD491CD119E8C7AA48B562D7482ED8
encoding key: DB472FF5CA74465BAA066021CD837645
size: 38 KiB
md5/content key: 87cd491cd119e8c7aa48b562d7482ed8
sha256: 73de74d5d63690f29c7f97a9225edc8bd6f89e5103806af3714e4d7bfb9474e9
file type: TrueType Font data
```

The local listfiles agree:

```text
data/wow-ui-sim-listfile.csv:
615960;Fonts/FRIZQT__.TTF

data/full-community-listfile.csv:
615960;fonts/frizqt__.ttf
```

Extraction proof:

```text
/home/osso/Projects/world-of-osso/asset-resolver/target/debug/casc-local 615960 -o /tmp/fdid-font-check

CASC: extracted FDID 615960 -> /tmp/fdid-font-check/615960.ttf
1 extracted, 0 failed
```

Spelling note: the extension is `.ttf`, not `.tff`.

### May 2026 Font Path-Casing Failure

Symptom:

```text
asset-cache byte resolve failed: fdid 615960: CASC read FDID 615960 via listfile path fonts/frizqt__.ttf: read CASC path fonts/frizqt__.ttf: Content not found: Path not found in root file: fonts/frizqt__.ttf
```

Root cause:

- `data/wow-ui-sim-listfile.csv` stored bundled font rows as lowercase paths.
- `WowFontSystem` also lowercased the requested CASC font filename.
- When the asset-resolver resolution cache did not contain those font FDIDs, `resolve_bytes(fdid)` fell back to the listfile path and asked CASC for the lowercase path, which this root file rejected.

Fix:

- Preserve canonical WoW font path casing in the bundled listfile (`Fonts/FRIZQT__.TTF`, `Fonts/ARIALN.TTF`, `Fonts/FRIZQT___CYR.TTF`).
- Key bundled listfile lookups by normalized path while preserving the original path on `ListfileEntry`.
- Do not stop at skipping the noisy `resolve_bytes(fdid)` path. That suppresses the error but leaves `WowFontSystem` using a system fallback face, which is visibly wrong in the objective tracker.
- Font loading now first tries path-to-encoding resolution from cached `root.bin` / `encoding.bin`, then falls back to the known standard font encoding keys and reads the real font bytes from the local CASC archives:
  - `FRIZQT__.TTF`: `DB472FF5CA74465BAA066021CD837645`
  - `ARIALN.TTF`: `B118D76FD2E2BDA9AAB0118B508D0FB1`
  - `FRIZQT___CYR.TTF`: `78AEBA943ABCFF292438DA989CC1E728`

### Gethe 12.0.5 Check

The Gethe `wow-ui-source` `12.0.5` file at:

```text
Interface/AddOns/Blizzard_APIDocumentation/BaseAPIMixin.lua
```

matches the local CASC extract after normalizing GitHub LF text back to Blizzard CRLF bytes:

```text
md5/content key: 51812203898e1e22b9927ddb95af0897
sha256: 8a7deec6860c5c1df52f938adaa7c97312de8666fa3dab43b643857815f72103
cmp: identical
```

This is the same CRLF-vs-LF pitfall seen with other Blizzard UI source files: GitHub serves normalized LF text, while CASC stores original CRLF bytes.

### Build Caveat

At the time of this check, `/syncthing/World of Warcraft/.build.info` still reported the active retail product as:

```text
Version: 12.0.1.66066
Build Key: 7b498dd7e196bf4161d631064f617189
Product: wow
```

The FDID/key result above is from that local CASC cache. It still byte-matches Gethe `12.0.5` for this specific file, but a full 12.0.5 root-debug pass should rebuild or select the cache for the 12.0.5 build key once the synced Windows install has fully landed.

### Debug Implications

If a Windows-side parser cannot find FDID `1579624` or `615960` in root:

- Confirm it is reading the `wow` product root for the intended build, not `wow_classic`, `_beta_`, or a stale build key.
- Confirm the root parser handles modern MFST root files and block headers correctly.
- Do not rely on path lookup alone; root maps `FileDataID -> content key`, then encoding maps `content key -> encoding key`.
- Check locale/content filtering. The local resolution cache selected content key `51812203898E1E22B9927DDB95AF0897` for FDID `1579624` and `87CD491CD119E8C7AA48B562D7482ED8` for FDID `615960`.
- Verify byte hashes with CRLF line endings when comparing against Gethe/GitHub source.

## Sources

- [data/wow-ui-sim-listfile.csv](../../../data/wow-ui-sim-listfile.csv) — local FDID-to-path listfile entry.
- [data/full-community-listfile.csv](../../../data/full-community-listfile.csv) — community listfile entry.
- [data/blizzard-ui-files.txt](../../../data/blizzard-ui-files.txt) — Blizzard UI manifest path.
- `~/.cache/asset-resolver/casc/wow/7b498dd7e196bf4161d631064f617189/resolution.sqlite` — local FDID to content/encoding key cache.
- `/syncthing/World of Warcraft/.build.info` — active local WoW product/build metadata.
- `https://raw.githubusercontent.com/Gethe/wow-ui-source/12.0.5/Interface/AddOns/Blizzard_APIDocumentation/BaseAPIMixin.lua` — Gethe source mirror used for CRLF hash comparison.

## See Also

- [[addon-loading]] — Blizzard UI addon loading and source cache behavior.
- [[cli-commands]] — `wow-cli casc` and related CLI commands.
