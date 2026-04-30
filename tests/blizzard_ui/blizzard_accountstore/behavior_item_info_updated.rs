//! Behavior pin for the `ACCOUNT_STORE_ITEM_INFO_UPDATED` event
//! handling contract in the `Blizzard_AccountStore` lane.
//!
//! Spec/source mismatch finding (PLAN.md task: firing
//! `ACCOUNT_STORE_ITEM_INFO_UPDATED` for an item visible in
//! `ItemRack` calls `RefreshSelectedCard` and re-reads
//! `C_AccountStore.GetItemInfo` for the affected itemID): the plan
//! makes four claims that all diverge from the actual implementation
//! at `Blizzard_AccountStoreCardTemplates.lua:15-118`.
//!
//! 1. **`RefreshSelectedCard` does not exist anywhere.** Verified
//!    via `grep -rn RefreshSelectedCard Interface/BlizzardUI/` — zero
//!    matches in the entire BlizzardUI source tree. The
//!    PLAN-named method is a phantom; no addon defines it. The
//!    actual callback is `AccountStoreBaseCardMixin:SetItemID` at
//!    line 115 (called from inside `:OnEvent` after the itemID
//!    filter passes).
//!
//! 2. **The handler is on `AccountStoreBaseCardMixin`, NOT on
//!    `AccountStoreItemRackMixin`.** PLAN says "ItemRack" handles
//!    the event. Actual: each individual card frame
//!    (`AccountStoreBaseCardMixin` instance) registers
//!    `AccountStoreBaseCardEvents` (lines 15-18 — a two-element
//!    array containing `"UI_MODEL_SCENE_INFO_UPDATED"` and
//!    `"ACCOUNT_STORE_ITEM_INFO_UPDATED"`) via
//!    `FrameUtil.RegisterFrameForEvents(self, AccountStoreBaseCardEvents)`
//!    in its OnShow at line 51. `AccountStoreItemRackMixin`
//!    (`Blizzard_AccountStoreItemRack.lua:18-56`) defines exactly
//!    four methods — `SetCategoryType`, `SetItems`, `Refresh`,
//!    `GetMaxCards` — and registers ZERO events. The rack is a
//!    layout-only frame; it spawns cards via a `cardPool` and the
//!    cards themselves handle the per-item event.
//!
//! 3. **The handler filters by itemID equality.** Lines 112-116
//!    read the first vararg as `itemID`, compare
//!    `itemID == self.itemInfo.id`, and only call `:SetItemID(itemID)`
//!    if equal. PLAN's "for an item visible in ItemRack" framing
//!    glosses over this — cards displaying OTHER items (or no item)
//!    silently ignore the event. A regression that dropped the
//!    `if itemID == self.itemInfo.id then` guard would re-render
//!    every visible card on every event fire, multiplying the
//!    `C_AccountStore.GetItemInfo` calls by the visible card count.
//!
//! 4. **The re-read happens inside `SetItemID`, not at the event
//!    handler.** `SetItemID` at line 128 calls
//!    `local itemInfo = C_AccountStore.GetItemInfo(itemID)` — the
//!    PLAN's "re-reads C_AccountStore.GetItemInfo" claim is correct
//!    in shape but located one method deeper than the PLAN's
//!    framing implies. The re-read is part of the SetItemID
//!    contract and runs on every SetItemID call (whether triggered
//!    by the event handler at line 115, by `:Refresh()` at
//!    `Blizzard_AccountStoreItemRack.lua:37`, or by
//!    `CheckForItemStateUpdate` at line 207 — three call sites total).
//!
//! Five tests pin the contract:
//!
//! - `refresh_selected_card_method_does_not_exist_on_either_mixin`
//!   asserts both `AccountStoreItemRackMixin.RefreshSelectedCard`
//!   and `AccountStoreBaseCardMixin.RefreshSelectedCard` are nil.
//!   Structural-absence tripwire for the PLAN-named method — flips
//!   if a future Blizzard change adds the method (forcing a re-pin
//!   against the new contract).
//!
//! - `account_store_item_rack_mixin_methods_match_actual_source`
//!   asserts the four actual methods (SetCategoryType, SetItems,
//!   Refresh, GetMaxCards) are functions and that PLAN-named or
//!   plausibly-named methods (RefreshSelectedCard, OnEvent) are
//!   nil. Pins the rack's no-event-handler contract.
//!
//! - `on_event_for_matching_item_id_calls_set_item_id_with_matching_id`
//!   replaces `AccountStoreBaseCardMixin.SetItemID` with a tracker;
//!   directly invokes
//!   `AccountStoreBaseCardMixin.OnEvent(stub_self, "ACCOUNT_STORE_ITEM_INFO_UPDATED", item_id)`
//!   with `stub_self.itemInfo.id == item_id`; asserts
//!   (call_count=1, captured_self=stub_self, captured_item_id=item_id).
//!   Pins the actual handler dispatch path.
//!
//! - `on_event_for_non_matching_item_id_does_not_call_set_item_id`
//!   replaces SetItemID with a tracker; invokes OnEvent with an
//!   itemID that does NOT match `stub_self.itemInfo.id`; asserts
//!   the tracker received ZERO calls. Pins the
//!   `if itemID == self.itemInfo.id` filter at line 114.
//!
//! - `set_item_id_re_reads_c_account_store_get_item_info_for_the_affected_item_id`
//!   replaces `C_AccountStore.GetItemInfo` with a tracker that
//!   captures the itemID arg and returns nil (which short-circuits
//!   SetItemID at line 132 — "if not itemInfo then self:Hide()
//!   return end" — keeping the test focused on the C_API call
//!   without exercising the rest of the SetItemID body); invokes
//!   `AccountStoreBaseCardMixin.SetItemID(stub_self, item_id)`;
//!   asserts (call_count=1, captured_item_id=item_id). Pins the
//!   re-read at line 128.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccountStore";

#[test]
fn refresh_selected_card_method_does_not_exist_on_either_mixin() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let rack_method_type: String = env
            .eval("return type(AccountStoreItemRackMixin.RefreshSelectedCard)")
            .expect("AccountStoreItemRackMixin.RefreshSelectedCard probe must run cleanly");

        assert_eq!(
            rack_method_type, "nil",
            "Expected `type(AccountStoreItemRackMixin.RefreshSelectedCard) == \"nil\"` (PLAN.md \
             spec/source mismatch tripwire — the plan names `RefreshSelectedCard` as the method \
             that handles the ACCOUNT_STORE_ITEM_INFO_UPDATED event, but `grep -rn \
             RefreshSelectedCard Interface/BlizzardUI/` yields zero matches in the entire \
             BlizzardUI source tree), got `{rack_method_type}`. A non-nil reading would prove \
             Blizzard added the method, forcing a re-pin against the new mixin contract."
        );

        let card_method_type: String = env
            .eval("return type(AccountStoreBaseCardMixin.RefreshSelectedCard)")
            .expect("AccountStoreBaseCardMixin.RefreshSelectedCard probe must run cleanly");

        assert_eq!(
            card_method_type, "nil",
            "Expected `type(AccountStoreBaseCardMixin.RefreshSelectedCard) == \"nil\"` — same \
             tripwire on the per-card mixin. The actual handler is \
             `AccountStoreBaseCardMixin:SetItemID` (called from inside \
             `AccountStoreBaseCardMixin:OnEvent` at \
             `Blizzard_AccountStoreCardTemplates.lua:115` after the itemID-equality filter \
             passes). Got `{card_method_type}`."
        );
    });
}

const ACTUAL_RACK_METHODS: &[(&str, &str)] = &[
    ("SetCategoryType", "Blizzard_AccountStoreItemRack.lua:20-24"),
    ("SetItems", "Blizzard_AccountStoreItemRack.lua:26-29"),
    ("Refresh", "Blizzard_AccountStoreItemRack.lua:31-52"),
    ("GetMaxCards", "Blizzard_AccountStoreItemRack.lua:54-56"),
];

const ABSENT_RACK_METHODS: &[(&str, &str)] = &[
    (
        "RefreshSelectedCard",
        "PLAN-named method that does not exist anywhere in BlizzardUI",
    ),
    (
        "OnEvent",
        "the rack does NOT register events; per-card frames register \
         AccountStoreBaseCardEvents in their own OnShow at line 51",
    ),
];

#[test]
fn account_store_item_rack_mixin_methods_match_actual_source() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for (method_name, source_site) in ACTUAL_RACK_METHODS {
            let method_type: String = env
                .eval(&format!(
                    "return type(AccountStoreItemRackMixin[{method_name:?}])"
                ))
                .unwrap_or_else(|error| {
                    panic!(
                        "type(AccountStoreItemRackMixin.{method_name}) probe must run cleanly: \
                         {error}"
                    )
                });

            assert_eq!(
                method_type, "function",
                "Expected `type(AccountStoreItemRackMixin.{method_name}) == \"function\"` \
                 ({source_site}), got `{method_type}`. A non-function reading would prove the \
                 actual rack surface lost a method (forcing a re-pin against the new contract)."
            );
        }

        for (absent_method, mismatch_reason) in ABSENT_RACK_METHODS {
            let method_type: String = env
                .eval(&format!(
                    "return type(AccountStoreItemRackMixin[{absent_method:?}])"
                ))
                .unwrap_or_else(|error| {
                    panic!(
                        "type(AccountStoreItemRackMixin.{absent_method}) probe must run cleanly: \
                         {error}"
                    )
                });

            assert_eq!(
                method_type, "nil",
                "Expected `type(AccountStoreItemRackMixin.{absent_method}) == \"nil\"` \
                 ({mismatch_reason}), got `{method_type}`. A non-nil reading would prove either \
                 (a) Blizzard moved the event handler onto the rack (forcing a re-pin against \
                 the new dispatch shape — and breaking the per-card filter that limits work to \
                 cards actually displaying the affected itemID), or (b) the PLAN-named method \
                 finally appeared in the source."
            );
        }
    });
}

#[test]
fn on_event_for_matching_item_id_calls_set_item_id_with_matching_id() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const MATCHING_ITEM_ID: i64 = 444_777_001;

        seed_set_item_id_tracker(env);
        seed_stub_card_with_item_info_id(env, MATCHING_ITEM_ID);

        env.eval::<()>(&format!(
            r#"
            AccountStoreBaseCardMixin.OnEvent(
                _G.__behavior_item_info_updated_stub_card,
                "ACCOUNT_STORE_ITEM_INFO_UPDATED",
                {MATCHING_ITEM_ID}
            )
            return
            "#
        ))
        .expect("Direct OnEvent invocation must run cleanly");

        let (call_count, captured_self_marker, captured_item_id): (i64, String, i64) = env
            .eval(
                "return _G.__behavior_item_info_updated_set_item_id_calls, \
                 _G.__behavior_item_info_updated_captured_self_marker, \
                 _G.__behavior_item_info_updated_captured_item_id",
            )
            .expect("Tracker readout must run cleanly");

        assert_eq!(
            call_count, 1,
            "Expected exactly ONE `SetItemID` call after a direct invocation of \
             AccountStoreBaseCardMixin.OnEvent with event=ACCOUNT_STORE_ITEM_INFO_UPDATED and \
             itemID matching `stub_card.itemInfo.id` \
             (`Blizzard_AccountStoreCardTemplates.lua:113-115`), got {call_count}. A zero \
             reading would prove the matching-id branch was bypassed; a value > 1 would prove \
             the branch fan-outs into multiple SetItemID calls per event."
        );

        assert_eq!(
            captured_self_marker, "behavior_item_info_updated_stub_card",
            "Expected the first arg to `SetItemID` (`self`) to be the same stub_card we \
             constructed in `seed_stub_card_with_item_info_id` (identified by its `__marker` \
             field), got marker=`{captured_self_marker:?}`. A different marker would prove the \
             handler dispatched into a different frame's SetItemID, breaking the per-card scope."
        );

        assert_eq!(
            captured_item_id, MATCHING_ITEM_ID,
            "Expected the second arg to `SetItemID` (`itemID`) to equal {MATCHING_ITEM_ID} (the \
             itemID passed to OnEvent and equal to `stub_card.itemInfo.id`), got \
             {captured_item_id}. The handler at line 115 calls `self:SetItemID(itemID)` — \
             passing the EVENT'S itemID, not `self.itemInfo.id` (they happen to be equal here \
             because the filter at line 114 only lets matching ids through). A divergence \
             would prove the handler started passing a different value (e.g. self.itemInfo.id, \
             which would be a no-op rename but would diverge if the event's itemID started \
             carrying additional payload)."
        );

        teardown_set_item_id_tracker(env);
    });
}

#[test]
fn on_event_for_non_matching_item_id_does_not_call_set_item_id() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const STUB_CARD_ITEM_ID: i64 = 555_111_001;
        const EVENT_ITEM_ID: i64 = 555_111_999;

        seed_set_item_id_tracker(env);
        seed_stub_card_with_item_info_id(env, STUB_CARD_ITEM_ID);

        env.eval::<()>(&format!(
            r#"
            AccountStoreBaseCardMixin.OnEvent(
                _G.__behavior_item_info_updated_stub_card,
                "ACCOUNT_STORE_ITEM_INFO_UPDATED",
                {EVENT_ITEM_ID}
            )
            return
            "#
        ))
        .expect("Non-matching OnEvent invocation must run cleanly");

        let call_count: i64 = env
            .eval("return _G.__behavior_item_info_updated_set_item_id_calls")
            .expect("Tracker readout must run cleanly");

        assert_eq!(
            call_count, 0,
            "Expected ZERO `SetItemID` calls after firing OnEvent with itemID={EVENT_ITEM_ID} \
             on a stub_card whose `itemInfo.id={STUB_CARD_ITEM_ID}` — the handler at \
             `Blizzard_AccountStoreCardTemplates.lua:114` filters with \
             `if itemID == self.itemInfo.id then`, so non-matching ids fall through without \
             invoking SetItemID. Got {call_count} call(s). A non-zero reading would prove the \
             filter was dropped (every visible card would re-render on every event fire, \
             multiplying GetItemInfo calls by the visible card count)."
        );

        teardown_set_item_id_tracker(env);
    });
}

#[test]
fn set_item_id_re_reads_c_account_store_get_item_info_for_the_affected_item_id() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const SENTINEL_ITEM_ID: i64 = 666_999_777;

        seed_get_item_info_tracker_returning_nil(env);

        env.eval::<()>(&format!(
            r#"
            local stub_card = {{}}
            stub_card.Hide = function(_self) end
            AccountStoreBaseCardMixin.SetItemID(stub_card, {SENTINEL_ITEM_ID})
            return
            "#
        ))
        .expect("Direct SetItemID invocation must run cleanly");

        let (call_count, captured_item_id): (i64, i64) = env
            .eval(
                "return _G.__behavior_item_info_updated_get_item_info_calls, \
                 _G.__behavior_item_info_updated_get_item_info_captured_id",
            )
            .expect("GetItemInfo tracker readout must run cleanly");

        assert_eq!(
            call_count, 1,
            "Expected exactly ONE `C_AccountStore.GetItemInfo` call after a direct invocation \
             of AccountStoreBaseCardMixin.SetItemID(stub_card, {SENTINEL_ITEM_ID}) — the body at \
             `Blizzard_AccountStoreCardTemplates.lua:128` reads \
             `local itemInfo = C_AccountStore.GetItemInfo(itemID)` as the first executable \
             statement after assigning self.itemID. Got {call_count}. A zero reading would \
             prove the re-read was dropped (cards would never refresh on item-info-updated \
             events); a value > 1 would prove the body started fan-out reads."
        );

        assert_eq!(
            captured_item_id, SENTINEL_ITEM_ID,
            "Expected the captured itemID arg to `C_AccountStore.GetItemInfo` to equal \
             {SENTINEL_ITEM_ID} (the second arg to SetItemID), got {captured_item_id}. A \
             different value would prove the body started passing a different identifier (e.g. \
             self.itemID — the just-assigned value at line 126 — which would be the same in \
             practice but a regression to a stale or transformed id would diverge)."
        );

        teardown_get_item_info_tracker(env);
    });
}

fn seed_set_item_id_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_item_info_updated_set_item_id_calls = 0
        _G.__behavior_item_info_updated_captured_self_marker = "<unset>"
        _G.__behavior_item_info_updated_captured_item_id = -1
        _G.__behavior_item_info_updated_original_set_item_id = AccountStoreBaseCardMixin.SetItemID
        AccountStoreBaseCardMixin.SetItemID = function(self, item_id)
            _G.__behavior_item_info_updated_set_item_id_calls =
                _G.__behavior_item_info_updated_set_item_id_calls + 1
            _G.__behavior_item_info_updated_captured_self_marker = self.__marker or "<no-marker>"
            _G.__behavior_item_info_updated_captured_item_id = item_id
        end
        return
        "#,
    )
    .expect("seeding SetItemID tracker on AccountStoreBaseCardMixin must run cleanly");
}

fn seed_stub_card_with_item_info_id(env: &WowLuaEnv, item_info_id: i64) {
    env.eval::<()>(&format!(
        r#"
        local stub_card = {{}}
        stub_card.__marker = "behavior_item_info_updated_stub_card"
        local item_info = {{}}
        item_info.id = {item_info_id}
        stub_card.itemInfo = item_info
        setmetatable(stub_card, {{ __index = AccountStoreBaseCardMixin }})
        _G.__behavior_item_info_updated_stub_card = stub_card
        return
        "#
    ))
    .expect("seeding stub_card with itemInfo.id must run cleanly");
}

fn seed_get_item_info_tracker_returning_nil(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_item_info_updated_get_item_info_calls = 0
        _G.__behavior_item_info_updated_get_item_info_captured_id = -1
        _G.__behavior_item_info_updated_original_get_item_info = C_AccountStore.GetItemInfo
        C_AccountStore.GetItemInfo = function(item_id)
            _G.__behavior_item_info_updated_get_item_info_calls =
                _G.__behavior_item_info_updated_get_item_info_calls + 1
            _G.__behavior_item_info_updated_get_item_info_captured_id = item_id
            return nil
        end
        return
        "#,
    )
    .expect("seeding C_AccountStore.GetItemInfo tracker must run cleanly");
}

fn teardown_set_item_id_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        if _G.__behavior_item_info_updated_original_set_item_id ~= nil then
            AccountStoreBaseCardMixin.SetItemID =
                _G.__behavior_item_info_updated_original_set_item_id
            _G.__behavior_item_info_updated_original_set_item_id = nil
        end
        _G.__behavior_item_info_updated_set_item_id_calls = nil
        _G.__behavior_item_info_updated_captured_self_marker = nil
        _G.__behavior_item_info_updated_captured_item_id = nil
        _G.__behavior_item_info_updated_stub_card = nil
        return
        "#,
    )
    .expect("SetItemID tracker tear-down must run cleanly");
}

fn teardown_get_item_info_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        if _G.__behavior_item_info_updated_original_get_item_info ~= nil then
            C_AccountStore.GetItemInfo =
                _G.__behavior_item_info_updated_original_get_item_info
            _G.__behavior_item_info_updated_original_get_item_info = nil
        end
        _G.__behavior_item_info_updated_get_item_info_calls = nil
        _G.__behavior_item_info_updated_get_item_info_captured_id = nil
        return
        "#,
    )
    .expect("GetItemInfo tracker tear-down must run cleanly");
}
