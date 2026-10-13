# Addon local-table access

`C_AddOns.GetAddOnLocalTable` returns the actual second Lua-chunk argument of a loaded addon opting in with `AllowAddOnTableAccess: 1`. [Patch 11.1.7 source](../../data/patch-api/sources/11.1.7-api-changes.txt) and cached `AddOnsDocumentation.lua:149–162` establish opt-in and denial of Blizzard addon tables to insecure callers. Native absent-target/error shape is unknown; policies below are explicit simulator choices.

## What it must do

- [ ] Return the same nonnil namespace table used by all files of the opted-in addon; preserve identity across repeated calls and later addon loads. Caller mutation is visible to original target closures.
- [ ] Retain that namespace through garbage collection after chunks return, including when only a weak test witness remains.
- [ ] Resolve canonical loaded addon names, not display titles; never implicitly load a target or return a fabricated/copied table.
- [ ] **Modeled policy:** unknown, registered-but-unloaded, title-only and unopted targets raise errors. Missing or zero opt-in does not expose the table.
- [ ] Enforce documented `AllowedWhenUntainted` arguments using genuine VM secret authentication; preserve wrappers and caller taint.
- [ ] Deny an insecure caller access to a Blizzard-classified target even with a plain name. Secure access to a loaded opted-in Blizzard target remains possible. Error shape is modeled; denial itself is sourced.

## How it works

- [Addon loading](../addon-loading-pipeline.md).
- [Lua API system](../wiki/systems/lua-api.md).

## Implementation inventory

- `src/loader/addon.rs`, `src/loader/lua_file.rs`: actual shared namespace creation/chunk arguments.
- `src/toc/mod.rs`: metadata and Blizzard classification.
- `src/c_api/c_addons.rs`, `c_addons_registration.rs`: namespace access API destination; producer/retention not implemented yet.

## Tests asserting this spec

`tests/addon_local_table.rs`: six unrun behavioral tests with real temporary two-addon loading, original namespace identity/mutation, GC lifetime, opt-in/absence and genuine security boundaries. No RED/GREEN claim until executed.

## Known gaps (current cycle)

- [ ] Obtain genuine behavioral RED, implement retained rooted namespaces and opt-in access, then verify.
- [ ] Native absent-target behavior and error text remain unverified.

## Out of scope

Reload/unload, failed-load/bootstrap-only publication, numeric selectors, aliases/case normalization and native/earlier-profile parity are separate contracts. No vendor edits or copied/empty namespace compatibility paths.
