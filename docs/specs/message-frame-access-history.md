# Message-frame access history (C11)

Status: corrected tests/passive scaffold staged; RED/GREEN unexecuted. Both widget types have inert removal publication; transfer exceptions are observed values checked by an explicit outcome assertion, not masked setup failures.

Consumer-derived simulator contract for `MessageFrame` and `ScrollingMessageFrame` history methods in `src/lua_api/frame/methods/widgets/message_frame/`. Public Wrath UI requires filtered transfer; this spec makes no native-parity claim. Architecture context: [Lua API](../lua-api.md).

## What it must do

- [ ] With distinct numeric access IDs and untagged messages, `GetNumMessages(id)` counts exactly the selected group, including zero for an absent ID, without mutating history. Both widget types share this contract.
- [ ] `GetNumMessages()` retains its existing total-history behavior before and after filtered reads; unfiltered text order and clear behavior remain unchanged.
- [ ] On Wrath, retain the public consumer's `AddMessage(text, r, g, b, lineID, false, accessID, extraData)` metadata. Access ID, line ID and extraData are independent values, not alpha or a single interchangeable identifier.
- [ ] `GetMessageInfo(i, id)` indexes the selected subsequence using one-based history order and returns `text, accessID, lineID, extraData` for the Wrath transfer consumer. Existing unfiltered simulator returns remain unchanged; this is a bounded simulator contract, not an assertion of complete Wrath API compatibility.
- [ ] `RemoveMessagesByAccessID(id)` removes only selected entries, preserving other groups and untagged entries in order.
- [ ] The unchanged public Wrath `FCF_OpenTemporaryWindow` transfer clears stale destination history, copies only the requested conversation in order, retains line/extraData tokens, recovers colors through `ChatHistory_GetChatType(extraData)`, and removes only that conversation from source history.

## How it works

- [Lua API architecture](../lua-api.md)
- [Widget architecture](../widget-system.md)

## Implementation inventory

- `src/lua_api/message_frame.rs`: stored message text, colors, identifiers and history state.
- `src/lua_api/frame/methods/widgets/message_frame/add.rs`: message ingestion; bounded Wrath history argument shape, existing short simulator calls preserved.
- `src/lua_api/frame/methods/widgets/message_frame/getters.rs`: total/filtered count and indexed retrieval.
- `src/lua_api/frame/methods/widgets/message_frame/transform.rs`: access-ID removal and metadata preservation across existing transforms.
- `src/lua_api/frame/methods/widgets/message_frame/mod.rs`: method registration.

## Tests asserting this spec

`tests/c11_message_access.rs` (staged, not run):

- `c11_message_frame_counts_only_selected_access_id` and `c11_scrolling_message_frame_counts_only_selected_access_id`: concrete grouped counts plus empty/total/order/clear preservation (no redundant standalone control).
- `c11_filtered_info_indexes_selected_history_and_returns_transfer_metadata`: filtered order and distinct metadata.
- `c11_remove_selected_access_id_preserves_other_and_untagged_messages`: selective removal.
- `c11_original_wrath_temporary_window_moves_only_selected_access_history`: whole unchanged cached public files, bounded chrome fixture, real history methods. Only `FCF_OpenTemporaryWindow` is captured; expected success/destination/counts are asserted with the captured consumer error in the failure diagnostic. File, digest, fixture and setup-error checks remain outside that capture.

## Known gaps (current cycle)

- [ ] Run tests-only against the recorded HEAD and retain actual behavioral RED evidence before applying the separately staged producer.
- [ ] Confirm the whole-file bounded Wrath fixture loads, then run producer GREEN and relevant preservation checks; no execution is authorized in this staging task.
- [ ] Requested `~/Repos/wowless/data/products/wow/uiobjects.yaml` is absent on this host; signatures were not verified from Wowless.

## Out of scope

- Native reconstruction or native-parity claims; labels are pointers only.
- Retail/PTR/Mists/Forever chat argument/return migrations: populated public consumers iterate total history and filter in Lua instead, with different tuple layouts.
- Era/Anniversary consumer conclusions: caches absent. Cataclysm is unsupported and excluded.
- Full startup, docking, private-message routing, rendering, timers, invalid arguments and out-of-range return semantics: not established by this bounded consumer.
