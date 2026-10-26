# Mists supplied LFR availability

Mists `C_LFGInfo.IsLFREnabled` consumes an explicit environment-local availability input. The pinned 5.5.4.70268 declaration is `Blizzard_APIDocumentationGenerated/LFGInfoDocumentation.lua:179`; the original consumer is `Blizzard_GroupFinder/Classic/PVEFrame.lua:229`. See the [model and consumer boundary](../wiki/systems/mists-lfr-availability.md).

## What it must do

- [ ] Publish the query only on Mists and return exactly one boolean for supplied true or false.
- [ ] Preserve unknown availability as an unset value. An unset query raises `native-unknown: C_LFGInfo.IsLFREnabled has no supplied availability or documented default`; it does not silently disable LFR. This is approved simulator missing-input policy, not native error behavior.
- [ ] The selected original XML-bound PVEFrame callback shows the LFR button for supplied true and hides it for supplied false, with repeated changes and the original hidden-LFR gap-closing anchor.
- [ ] Independent environments retain independent input and button state. Player level does not determine availability.
- [ ] Clearing a supplied value restores the explicit missing-input error; supplying again restores the original consumer's behavior.

## How it works

- [Supplied model and original consumer](../wiki/systems/mists-lfr-availability.md)
- [Addon loading](../addon-loading-pipeline.md)

## Implementation inventory

- `src/c_api/lfr_availability.rs` — boolean query and explicit unset error.
- `src/c_api/c_lfg_info.rs` — Mists-only publication through ACTIVE.
- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs` — environment-local optional input and unset initialization.

## Tests asserting this spec

- `tests/mists_lfr_availability.rs::original_pve_frame_tracks_supplied_lfr_availability_and_isolation` — original TOC/XML/Lua identity, transitions, isolation and unset recovery.

## Known gaps (current cycle)

- [ ] Original-consumer runtime RED/GREEN and stock comparison await an admitted build slot.

## Out of scope

Native service/default policy, automatic availability derivation from level/catalog/expansion, usability policy, scenario catalogs, other-profile publication and Blizzard edits are excluded. The consumer treats nil and false as hidden, but the declaration requires a non-nil bool; the explicit unset error avoids turning unknown into an invented disabled default.
