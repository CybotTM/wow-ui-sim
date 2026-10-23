# Pinned PTR Blizzard UI source

PTR selects `wowt` **12.1.7.70424** using an immutable Blizzard CDN content index, independent of the locally installed retail/PTR product. The exact build/config/Gethe identities and generation evidence live in the [content-index contract](../ptr-cdn-content-index.md#selected-identity-and-regeneration-evidence). Gethe supplies source path discovery and revision identity, never runtime replacement bytes. Retail and Classic source acquisition remain unchanged.

## What it must do

- [ ] Require exact schema/product/version/build/CDN/install/Gethe identity for `wowt` 70424 and reject previous `wowxptr` 69594 cache provenance, even when required files exist; Rust GREEN pending.
- [x] Verify encoded BLTE identity and decoded content size/hash before accepting source bytes.
- [x] Require exact HTTP partial-content ranges; reject full-archive or wrong-range responses.
- [x] Retry transient HTTP failures with bounded backoff and respect `Retry-After`.
- [ ] Synchronize all 4,570 selected source entries, write completion only after all entries succeed, and reuse only hash-matching files; isolated 70424 runtime acquisition pending.
- [ ] Load PTR startup and representative panels against the selected cache. The existing baseline belongs to 69594, not 70424; capture waits for the matching epoch lane. A completed cache is not compatibility proof.

## How it works

- [Content-index regeneration](../ptr-cdn-content-index.md)
- [Client profiles](client-profiles.md)
- [Patch update procedure](../updating-blizzard-ui-to-a-new-patch.md)

## Implementation inventory

- `data/blizzard-ui-builds/ptr.json`: Blizzard build/config identity, Gethe revision, and source encoding/content keys with archive locations.
- `data/blizzard-ui-files/ptr.txt`: matching complete source path inventory.
- `src/blizzard_ui_sync/pinned.rs`: provenance, cache validation and bounded parallel synchronization.
- `src/blizzard_ui_sync/pinned_download.rs`: range retrieval and BLTE/content validation.
- `Cargo.toml`: direct `cascette-formats` dependency reuses the already-pinned BLTE decoder; `httpdate` parses HTTP-date `Retry-After` values.

## Tests asserting this spec

- `src/blizzard_ui_sync/pinned.rs`: exact versus legacy provenance.
- `src/blizzard_ui_sync/pinned_download.rs`: decoded bytes, corrupt encoding/content, local HTTP range and transient-response behavior.

## Historical 69594 proof (not 70424 acceptance)

- [ ] Resolve native pixel-rounding behavior from the selected [live PTR probe](../addons/PixelRoundingProbe/README.md), then prove startup and representative panels. Three ClassNameplate initializers account for the remaining six records, down from 109 after the math/table/timer/string/Curio fixes.

Independent source verification matched all 4,025 cache files to pinned size/content keys and reproduced the content-index JSON semantically. This establishes source integrity, not complete PTR compatibility.

## Out of scope

- General-purpose TVFS support or repairing upstream CASC parsers: this runtime consumes a generated index for one pinned build.
- Gethe content fallback, relabeling installed `wowt` files, or silently following a newer product head.
