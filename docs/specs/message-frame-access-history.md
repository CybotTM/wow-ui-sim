# Message-frame access history (C11)

Historical producer `269ed6f17` replaces the passive scaffold; sealed RED/GREEN receipts remain below. The 2026-10-20 lifecycle repair and corrected C11 tests have separate bounded runtime, format and default check proof below. No complete native, full-profile or startup acceptance is implied.

Consumer-derived simulator contract for `ScrollingMessageFrame` history methods and inferred plain `MessageFrame` row lifecycle in `src/lua_api/frame/methods/widgets/message_frame/`. Public Wrath UI requires filtered scrolling-history transfer; plain-frame history queries are not native 12340 APIs. This spec makes no complete native-parity claim. Architecture context: [Lua API](../lua-api.md).

## What it must do

- [ ] On Wrath ScrollingMessageFrame, `GetNumMessages(id)` counts the selected access-ID group without mutation; nil/omitted/zero selects total history. Unknown nonzero IDs count zero.
- [ ] Wrath ScrollingMessageFrame `AddMessage(text, r, g, b, typeID, backFill, accessID, extraData)` stores independent type/access/extraData tokens regardless of false, true or omitted backFill. Missing tokens represent zero. The existing Rust `message_id` field stores typeID (called lineID by the public consumer); no new identifier type is implied.
- [ ] On Wrath ScrollingMessageFrame, both `GetMessageInfo(i)` and `GetMessageInfo(i, id)` return exactly `text, accessID, typeID, extraData`, in one-based selected history order. Nil/zero ID means unfiltered. Invalid/missing indices raise Lua errors, not an empty row or zero return values.
- [ ] On Wrath, `RemoveMessagesByAccessID(id)` requires a numeric ID and returns nothing. It removes exact matches; zero removes implicit/explicit zero-ID rows, not all history. Other groups retain order.
- [ ] Wrath plain MessageFrame exposes none of the scrolling history methods. Its own methods are the authenticated 28 listed below, plus inherited Frame/Region/ScriptObject methods. Retail plain MessageFrame and scrolling color-tuple behavior remain unchanged.
- [ ] The unchanged public Wrath `FCF_OpenTemporaryWindow` transfer clears stale destination history, copies only the requested conversation in order, retains line/extraData tokens, recovers colors through `ChatHistory_GetChatType(extraData)`, and removes only that conversation from source history.

## Authoritative 12340 scope — 2026-10-20

Laptop reconstructed-source answers `message-frame-access-12340.md` and `message-frame-queue-12340-20261020.md` in the private `handoff/laptop-labels-answers-20261018/` bind these requirements to build 12340; they are not native-execution results. The complete plain table (`CSimpleMessageFrame_Scripts.cpp:337–366`) has Set/Get pairs for FontObject, Font, TextColor, ShadowColor, ShadowOffset, Spacing, JustifyH, JustifyV, InsertMode, IndentedWordWrap, Fading, TimeVisible and FadeDuration, plus AddMessage and Clear. Inheritance is Frame → Region → ScriptObject. In particular HasMessageByID, ResetMessageFadeByID, SetFadePower, GetFadePower and GetFontStringByID are absent on Wrath plain MessageFrame; ScrollingMessageFrame retains them.

The [frozen Wrath diff](../wow-client-diff/method-baselines/wrath/no-retail-epoch/forbidden-0-on-update-0/diff_methods_extra.txt) marks plain GetMessageInfo (line5983), GetNumMessages (line6000), GetMaxLines (line5981), SetMaxLines (line6345), and other scrolling-only methods as EXTRA. It does not mark the five methods above EXTRA. This is a documented conflict, not evidence of 12340 presence: the [baseline policy](../wow-client-diff/method-baselines/README.md) says its native build/epoch/product are unknown. Explicitly approved resolution: authenticated 12340 source outranks that inventory for Wrath only. Frozen discovery/baseline artifacts remain untouched.

Queue/Clear/GetRect/layer scheduling is not changed by this access-ID repair. The new answer authenticates enqueue and Clear-before-drain, but exact simulated update/layer timing remains a separate compatibility boundary. BackFill ordering is explicitly excluded; this lane only retains its metadata arguments.

## How it works

- [Lua API architecture](../lua-api.md)
- [Widget architecture](../widget-system.md)

## Implementation inventory

- `src/lua_api/message_frame.rs`: stored message text, colors, identifiers and history state.
- `src/lua_api/frame/methods/widgets/message_frame/add.rs`: message ingestion; bounded Wrath history argument shape, existing short simulator calls preserved.
- `src/lua_api/frame/methods/widgets/message_frame/getters.rs`: total/filtered count and indexed retrieval.
- `src/lua_api/frame/methods/widgets/message_frame/transform.rs`: access-ID removal and metadata preservation across existing transforms.
- `src/lua_api/frame/methods/widgets/message_frame/mod.rs`: method registration and distinction between shared message methods and scrolling-only entries.
- `src/lua_api/methods/frame_metatable.rs`: distinct Wrath plain/scrolling cached indices; XML intrinsic promotion reattaches the scrolling index.

## Tests asserting this spec

`tests/smf_access_id.rs` covers the authenticated plain surface, both cache-creation orders, Lua/XML/runtime-template aliases, zero-ID count/removal, four-value metadata, bounds errors and native argument positions with true/omitted backFill. Retail cases preserve the plain method surface and XML history.

`tests/c11_message_access.rs` (Wrath transfer cases plus a Retail color-tuple preservation case; historical RED and post-GREEN receipts below):

- `c11_scrolling_message_frame_counts_only_selected_access_id`: concrete grouped counts plus empty/total/order/clear preservation.
- `c11_plain_message_frame_queues_and_clear_cancels_before_drain_inferred`: AddMessage returns nothing; concrete active-row state stays empty before update, Clear cancels queued insertion, and a subsequent row appears on update. Plain-frame scheduling remains INFERRED; no history query is used.
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

## Wrath scrolling lifecycle repair — 2026-10-20

`392651e97` selected the queued-row lifecycle by `WidgetType::MessageFrame`, which also represents ScrollingMessageFrame. Restore immediate scrolling history across explicit Lua creation, direct XML and runtime-template child creation. Both insertion and update use the same predicate: only Wrath MessageFrame widgets without the ScrollingMessageFrame object-type name acquire queued rows/timers. Plain MessageFrame/UIErrorsFrame remains queued.

`template::set_intrinsic` stamps the existing Rust `object_type_name` for the ScrollingMessageFrame base on Wrath only: that is the only profile with this queued lifecycle. Retail identity and history output are unchanged. No new widget enum, flag, history method or access-ID API is introduced. `tests/widgets_b6_c6.rs` asserts synchronous metadata/removal and timer-free history across an update for all three creation paths.

Laptop clean-room evidence `handoff/laptop-labels-answers-20261018/message-frame-access-12340.md` contains two answers agreeing on synchronous native scrolling history and absence of GetNumMessages/GetMessageInfo/RemoveMessagesByAccessID on native plain MessageFrame. Plain-frame queuing remains INFERRED because timing evidence conflicts between answers. Historical C11 plain-frame immediate-history assertions were non-native expectations, not missing producer behavior. Explicitly authorized test-only commit `8fb92fc43` removes those assertions and retains supported AddMessage/Clear coverage with inferred scheduling. Production method registration remains unchanged; no native method-surface parity is claimed.

### Bounded repair proof

Fixture `1e468a530`: C11 0/5, C6 5/7. Both C6 failures report `scrolling history must be immediate`; the original temporary-window transfer observes `(true,0,0)` instead of `(true,2,3)`. Producer `c6e974619` restores the scrolling cases: unchanged C6 7/7, original empty combatlog refilter 1/1, and existing `message_frame::test_add_message_and_num_messages` 1/1 (it creates ScrollingMessageFrame; independently supplied base `391446092` receipt failed count0 vs2). Intermediate unchanged C11 is 2/5 because three tests still contain non-native plain-frame expectations.

After test-only `8fb92fc43`, explicit ADD-admitted Wrath rebuild followed by **C11-only rerun passes 5/5**, including the unchanged public transfer and new inferred plain-frame Clear-before-drain case. C6/combatlog/existing insert were not redundantly rerun: production and their test files are unchanged. Default Retail history remains 30/30 before/after the producer; C11 is cfg-excluded (zero tests), not five Retail passes. Post-correction format and default full-feature cargo check exit0. No whole-profile, rendering-pixel or native-run acceptance follows.

Exact commands, stdout/stderr, artifact hashes, revision scope and source/proof applicability are kept in the private `wrath-msg-access` handoff (`report.md`, `proof-ledger.json`). Runtime receipts record unchanged original CVars stat/hash; each child uses a byte-exact private copy. Plain-frame timing remains INFERRED, not upgraded by simulator test success.

## Known gaps (current cycle)

- [x] Saved tests/passive-scaffold behavioral RED accepted, including whole-file bounded Wrath fixture setup and explicit failed transfer outcome.
- [ ] Producer GREEN and preservation are bounded by the resolved receipts above; no Retail C11 execution or full chat lifecycle acceptance follows.
- [ ] Requested `~/Repos/wowless/data/products/wow/uiobjects.yaml` is absent on this host; signatures were not verified from Wowless.

## Out of scope

- Native reconstruction or native-parity claims; labels are pointers only.
- Retail/PTR/Mists/Forever chat argument/return migrations: their current tuple layouts remain unchanged.
- Era/Anniversary consumer conclusions: caches absent. Cataclysm is unsupported and excluded.
- Full docking, private-message routing, rendering, timers, backFill ordering and complete input-coercion parity. The new explicit invalid-history-index and missing-removal-ID errors are in scope; they do not establish complete argument-validation parity.
