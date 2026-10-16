# Message-frame access history (C11)

Status: producer `269ed6f17` replaces the passive scaffold; historical behavioral RED accepted. Bounded producer GREEN accepted from the sealed component receipts below. No native, full-profile, startup, format or type-check acceptance is implied.

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

`tests/c11_message_access.rs` (Wrath-only execution; historical RED and post-GREEN receipts below):

- `c11_message_frame_counts_only_selected_access_id` and `c11_scrolling_message_frame_counts_only_selected_access_id`: concrete grouped counts plus empty/total/order/clear preservation (no redundant standalone control).
- `c11_filtered_info_indexes_selected_history_and_returns_transfer_metadata`: filtered order and distinct metadata.
- `c11_remove_selected_access_id_preserves_other_and_untagged_messages`: selective removal.
- `c11_original_wrath_temporary_window_moves_only_selected_access_history`: whole unchanged cached public files, bounded chrome fixture, real history methods. Only `FCF_OpenTemporaryWindow` is captured; expected success/destination/counts are asserted with the captured consumer error in the failure diagnostic. File, digest, fixture and setup-error checks remain outside that capture.

## Bounded RED and post-GREEN receipts — 2026-10-16

Historical tests/passive scaffold: `680a35a1e`; producer: `269ed6f17`. The independent RED report authenticates production-input hashes despite the libraries' older recorded HEAD and documents the cfg-excluded Retail helper conversion difference; do not attribute producer behavior to the RED libraries.

- Retail RED component: `/home/osso/.local/state/wow-ui-sim/verification/green-batch/label-retail-red-20261016T194438112412Z`.
- Wrath RED component: `/home/osso/.local/state/wow-ui-sim/verification/green-batch/label-wrath-red-20261016T195708968486Z`.
- Independent RED acceptance: `/home/osso/.local/state/wow-ui-sim/verification/green-batch/label-red-report.md` (saved artifact/source reconciliation, not a fresh run).

**C11 observed RED:** Retail cfg-excluded (no C11 execution); Wrath 5 FAIL / 0 PASS. Counts, filtered index and selective removal fail value assertions. Whole-file fixture loads/digests/setup succeed; the captured public consumer outcome is `(false,0,5)` rather than `(true,2,3)`, retaining the nil-info diagnostic. This is an asserted unsuccessful transfer, not a successful consumer execution; detailed metadata/color checks await GREEN.

**Accepted bounded GREEN receipt:** Retail `/home/osso/.local/state/wow-ui-sim/handoff/label-producers-green-private/components/label-prod-retail-labels-20261016T210501464288Z`; Wrath `/home/osso/.local/state/wow-ui-sim/handoff/label-producers-green-private/components/label-prod-wrath-labels-20261016T211404318442Z`; independent reports `/home/osso/.local/state/wow-ui-sim/verification/label-producers-green/retail-report.md` and `/home/osso/.local/state/wow-ui-sim/verification/label-producers-green/wrath-report.md`. Those reports own executed identities/counts, assertion reachability, source/artifact identity and preservation scope. Retail cannot earn Wrath-only C11/legacy quest credit. Other profiles, ordinary startup, format/check and native parity remain separate gates.

## Known gaps (current cycle)

- [x] Saved tests/passive-scaffold behavioral RED accepted, including whole-file bounded Wrath fixture setup and explicit failed transfer outcome.
- [ ] Producer GREEN and preservation are bounded by the resolved receipts above; no Retail C11 execution or full chat lifecycle acceptance follows.
- [ ] Requested `~/Repos/wowless/data/products/wow/uiobjects.yaml` is absent on this host; signatures were not verified from Wowless.

## Out of scope

- Native reconstruction or native-parity claims; labels are pointers only.
- Retail/PTR/Mists/Forever chat argument/return migrations: populated public consumers iterate total history and filter in Lua instead, with different tuple layouts.
- Era/Anniversary consumer conclusions: caches absent. Cataclysm is unsupported and excluded.
- Full startup, docking, private-message routing, rendering, timers, invalid arguments and out-of-range return semantics: not established by this bounded consumer.
