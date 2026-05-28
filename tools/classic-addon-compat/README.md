# Per-addon compat shims

Each subdirectory here holds optional compatibility shims for one
manifest addon. `scripts/test-classic-addons.sh` symlinks every immediate
subdirectory of `<addon>/` into `Interface/AddOns/` when running that
addon, so the loader treats them as ordinary in-tree companion addons.

```
tools/classic-addon-compat/
└── <ManifestAddonName>/
    └── <ShimAddonName>/
        ├── <ShimAddonName>.toc      # ## LoadFirst: 1
        └── <ShimAddonName>.lua      # rawget-guarded global stubs
```

Layout rules:

- The outer directory **must match the manifest's `name` column** in
  `tools/classic-addon-manifest.tsv`. The harness uses this name to find
  shims for the addon it's about to run.
- Inner subdirectories are arbitrary slug names that become the symlinked
  addon directory name (`Interface/AddOns/<inner>`).
- The TOC inside each shim addon must declare `## LoadFirst: 1` so the
  loader runs it before the third-party addon — the rawget guards then
  win against later real definitions only if the third-party addon
  defines them, which is the desired ordering.

## When to add a shim

The harness reports an `addon-induced errors` count per addon. When that
count is nonzero, inspect
`target/addon-harness/<addon>-lua-errors.json`, identify the missing
globals or namespaces, and add stub entries to a shim under the addon's
directory.

Use `rawget(_G, "X") == nil then ... end` guards so a real definition
from any later-loaded addon (or a future change to the simulator's
runtime surface) takes precedence.

## When NOT to add a shim

If the same gap shows up across **multiple** Mists addons, promote it to
`src/mists/compat_bootstrap.lua` or to a Rust backing model instead.

Per-addon shims should stay narrow — addon-specific quirks, not
broadly-missing APIs.
