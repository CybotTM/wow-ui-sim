# Texture source resolution cache

`SimState` owns an `Rc<RefCell<TextureManager>>`. SetTexture existence checks and environment-associated GUI/headless rendering use that same manager; standalone diagnostic utilities create local managers. No global source cache.

## Lookup and identity

The resolver still probes the case-preserving normalized request exactly, in original root and extension order, before its case-insensitive phase. There is no persistent request-to-result map: normalized request spelling remains an input on every call, so differently cased requests cannot alias an exact-case decision. The shared directory index is keyed by actual directory path within the manager's ordered-root context and compile-time client profile. Root-vector changes clear it; Lua and GUI ReloadUI clear it even when root spelling is unchanged.

Directory listings retain the first enumerated entry per Unicode-lowercased name, including directories; callers still enforce file/directory types. Unix freshness checks include device, inode, mtime and ctime nanoseconds; other hosts use modification time and length. A missing name forces a fresh enumeration once per synchronous search regardless of matching timestamps. Extension probes share that fresh listing only within the search. Failed/partial listings are not reusable negative evidence.

CPU RGBA/BC negative entries are keyed by normalized request within the ordered-root/profile context. They retain the consulted directory/file metadata (including absent paths), not permanent misses. Repeated unchanged loads validate those stamps without reopening directories; creation/deletion/replacement invalidates the entry. Root changes and reload clear both negative maps. Failed/partial enumeration cannot create reusable negatives. CASC extraction misses retain the existing CPU failure-cache behavior for an unchanged local generation; native CASC availability is not polled per draw. Local extraction paths are observed after the resolver's own directory creation, preventing self-invalidating misses. The missing-path set only limits logging. Existing decoded pixel/dimension/crop and GPU-ready/failed handles are not hot-reload invalidated. Concurrent filesystem mutation is not an atomic snapshot guarantee.

## Proof boundaries

A child-process preservation control uses a private incomplete CASC install and a local catalog row for FDID 134400. It counts actual emitted extraction-failure diagnostics across first, unchanged warm, root-change and ReloadUI phases; no synthetic extraction counter or native-CASC hot reload model is added.

A separate 64-frame BC+RGBA missing-source fixture bounds directory opens to two before creation, then verifies newly created PNG pixels. The pre-existing non-BLP telemetry test still requires cheap unchanged negatives; permanence is replaced by metadata-validated freshness.

The Linux behavioral fixture calibrates an inotify directory observer against three real read_dir opens, then exercises 512 setters across 64 existing and 64 missing mixed-case requests in a 525-file leaf. The assertion bounds actual directory-open events, not helper calls; elapsed time is supporting evidence, not a universal deadline. This does not reproduce the 95-second real addon callback or prove first-draw responsiveness.

## Sources

- [Contract and test inventory](../../specs/texture-source-resolution-cache.md).
- [Texture selection and atlas architecture](../../texture-atlas-system.md).

## See Also

- [[ellesmereui-forever]] — real addon acceptance remains separate.
