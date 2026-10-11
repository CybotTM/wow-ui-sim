# Roleset body/model audit — current source

Read-only audit of the literal Retail 12.1 API-change statement and current model. No implementation approval or new gate implied.

## Contract evidence

`data/patch-api/sources/12.1.0-api-changes.txt` says: “A new system has been added called the Roleset System, which allows you to tag a frame as being part of a "roleset". You can then use the new C_Roleset.ApplyRolesetFilters to specify which rolesets are currently active. Frames in an inactive roleset will never be shown, regardless of their shown state.” It names `C_Roleset.ApplyRolesetFilters`, but gives no argument shape, native ID/default behavior, coercion, or mapping of inactive status to allow/block semantics.

`docs/specs/forever-rolesets.md` governs the Forever membership change: expose `AddRoleset`, `RemoveRoleset`, `SetRolesets`, `GetRolesetNames`; per-frame membership; Set replaces and empty clears; preserve name-to-boolean table and lookup. It expressly says native roleset/security semantics are not claimed. Thus this spec does not constrain the Retail 12.1 filter behavior beyond membership representation.

## Current literal model

`src/c_api/c_roleset.rs`: registers `ApplyRolesetFilters` and parses its first two arguments as tables named `blockedRolesets` and `allowedRolesets`; arrays are read to first nil and values accepted through `val_to_string`. It replaces `SimState.active_blocked_rolesets` and `.active_allowed_rolesets`, then refreshes tagged frames. `is_filtered` marks a tagged frame filtered if a tag appears in blocked, or if a nonempty allowlist has no matching tag; the literal special tag `alwaysBlocked` is always filtered. Untagged frames are explicitly exempt in code as an inference.

`src/widget/frame.rs`: `Frame.rolesets: Vec<String>` stores tags; `Frame.roleset_filtered: bool` stores the derived filter bit separately from `visible` (the shown state). `src/lua_api/frame/methods/text_attribute_event/rolesets.rs` exposes Add/Remove/Set/Get, and retail-gates `IsRolesetFiltered`; `SetRolesets` parses comma-separated strings, nil clears. XML roleset tagging also exists per the test fixture.

Visibility consumes the bit: `SimState` updates displayed state when filter changes; renderer/query visibility is via the existing displayed/ancestor visibility paths. The behavioral test below confirms filtered own frame is not visible while still shown, and descendants become not visible through ancestry without acquiring their own filtered bit.

## Coverage / falsification

`tests/patch_12_1_0_rolesets.rs::inactive_roleset_hides_frame_and_children_while_shown` (source inspected; **not run**) asserts active block filter hides tagged frame while `IsShown()` remains true; checks child visibility, untagged frame visibility, block/allow hit/miss, tag mutation, clear, and strata-bucket removal/restoration. `always_blocked_roleset_never_shows_without_filters` tests reserved behavior. `xml_roleset_attribute_tags_frame` tests XML tags. `tests/patch_12_1_0_surface_closures.rs::roleset_membership_is_not_filtered_without_active_filters` and `tests/patch_12_1_0_added_globals.rs::roleset_filters_round_trip_through_active_lists` provide adjacent coverage. Forever spec proof points to `tests/wowforever_rolesets.rs` for membership only.

Result: the provided-state visibility invariant is represented and has direct behavioral test source coverage. There is no evidence here establishing native argument ordering/shape, ID/default/coercion semantics, or native equivalence of allow/block interpretation. Existing implementation is not grounds to infer those omitted native details. The user-requested exact named c75 PASS was not found in inspected evidence; no test was rerun. Source audit only; tests/builds/operations/network/vendor acquisition/delegation not performed.
