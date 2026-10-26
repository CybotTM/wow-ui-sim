# Mists LFR availability

Mists `C_LFGInfo.IsLFREnabled` reads `SimState.lfr_availability: Option<bool>`. Each Lua environment owns its own value. The [contract](../../specs/mists-lfr-availability.md) requires supplied availability, not a level, catalog or expansion inference.

## Publication and unknown input

`c_lfg_info` publishes the separate `lfr_availability` C API module only when `client_profile::ACTIVE` is Mists. Supplied true/false returns one boolean. None raises a named `native-unknown` missing-input error, following the Forever target-spell diagnostic convention. This explicit simulator policy was approved because native defaults are unknown and the declaration excludes nil; it does not claim native error behavior.

## Original consumer boundary

Pinned Mists 5.5.4.70268 `Blizzard_APIDocumentationGenerated/LFGInfoDocumentation.lua:179` declares the query. The selected `Blizzard_GroupFinder_Classic.toc` permits Mists and loads original `Classic/PVEFrame.lua` and `.xml`. The XML installs `GroupFinderFrame_OnEvent`, which calls `GroupFinderFrame_EvaluateButtonVisibility`. Its line229 reads availability; true shows the original LFR button and false hides it while closing the gap between neighboring buttons.

The regression verifies the selected TOC, original XML button identity, parent and script binding, and Lua source provenance before observing availability. Runtime RED at the original line229 reports the missing function. Initial GREEN reaches the separate missing `CanPlayerUseScenarioFinder` at line270 (declared in the same pinned documentation at line51). The parent approved bounding this slice there: a line observer checks the LFR visibility and hidden-gap anchor before line270 executes; the test then accepts only that exact named downstream error. Missing LFR at line229 is rethrown, never accepted. The revised byte-identical pair passes GREEN and fails RED at line229. Ten supplied-state observations span both booleans, levels1/90, two environments and unset recovery. Each callback proves visibility and the false-gap anchor before line270, then requires the exact named successor error. No Blizzard source is edited or replaced.

## Stock residual

Paired Mists stock at integrated master and the producer is 6 keys/12 occurrences on fresh copies of the current persisted CVar bytes. Exactly the missing-function OnLoad key and its reporter copy become named unsupplied-availability keys, count1 each; all surviving keys/counts are unchanged. This is explicit missing-input reporting, not clean startup or native-default acceptance.

## Verification scope

Mists, Retail and Era full-feature all-target checks pass with zero Cargo JSON or plain compiler warnings. Publication is runtime-gated, not hidden behind a new cfg. Formatting and changed-line readability pass. Other-profile runtime behavior and a full-suite run are not claimed.

## Sources

- [Contract and test inventory](../../specs/mists-lfr-availability.md)
- `src/c_api/lfr_availability.rs` — query and missing-input boundary.
- `tests/mists_lfr_availability.rs` — original callback, transitions and isolation.

## See Also

- [Addon loading](../../addon-loading-pipeline.md) — selected TOC and XML loading.
- [Party minimum](../../specs/party-min-level-scenario-availability.md) — separate Retail/PTR scenario model, not an LFR input.
