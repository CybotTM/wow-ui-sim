# Texture source resolution cache

`SimState` owns an `Rc<RefCell<TextureManager>>`. SetTexture existence checks and environment-associated GUI/headless rendering use that same manager; standalone diagnostic utilities create local managers. No global source cache.

## Lookup and identity

The resolver still probes the case-preserving normalized request exactly, in original root and extension order, before its case-insensitive phase. There is no persistent request-to-result map: normalized request spelling remains an input on every call, so differently cased requests cannot alias an exact-case decision. The shared directory index is keyed by actual directory path within the manager's ordered-root context and compile-time client profile. Root-vector changes clear it; Lua and GUI ReloadUI clear it even when root spelling is unchanged.

Directory listings retain the first enumerated entry per Unicode-lowercased name, including directories; callers still enforce file/directory types. Unix freshness checks include device, inode, mtime and ctime nanoseconds; other hosts use modification time and length. A missing name forces a fresh enumeration once per synchronous search regardless of matching timestamps. Extension probes share that fresh listing only within the search. Failed/partial listings are not reusable negative evidence.

The old sticky CPU RGBA/BC miss gates are removed; the missing-path set only limits logging. Existing decoded pixel/dimension/crop and GPU-ready/failed handles are not hot-reload invalidated. Concurrent filesystem mutation is not an atomic snapshot guarantee.

## Proof boundaries

The Linux behavioral fixture calibrates an inotify directory observer against three real read_dir opens, then exercises 512 setters across 64 existing and 64 missing mixed-case requests in a 525-file leaf. The assertion bounds actual directory-open events, not helper calls; elapsed time is supporting evidence, not a universal deadline. This does not reproduce the 95-second real addon callback or prove first-draw responsiveness.

## Sources

- [Contract and test inventory](../../specs/texture-source-resolution-cache.md).
- [Texture selection and atlas architecture](../../texture-atlas-system.md).

## See Also

- [[ellesmereui-forever]] — real addon acceptance remains separate.
