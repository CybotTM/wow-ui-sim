# CASC FDID 1579624 Root Debug

FDID `1579624` is present in the local CASC resolution cache and maps to Blizzard API documentation source `Blizzard_APIDocumentation/BaseAPIMixin.lua`. If another root parser cannot find this FDID, the likely failure is in MFST/root parsing, locale/content filtering, or build/cache selection rather than in the listfile path.

## Content

### Verified Mapping

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

If a Windows-side parser cannot find FDID `1579624` in root:

- Confirm it is reading the `wow` product root for the intended build, not `wow_classic`, `_beta_`, or a stale build key.
- Confirm the root parser handles modern MFST root files and block headers correctly.
- Do not rely on path lookup alone; root maps `FileDataID -> content key`, then encoding maps `content key -> encoding key`.
- Check locale/content filtering. The local resolution cache selected content key `51812203898E1E22B9927DDB95AF0897` for this FDID.
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
