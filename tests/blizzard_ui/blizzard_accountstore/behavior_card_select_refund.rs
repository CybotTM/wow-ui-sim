//! Behavior pin for the refund branch of `AccountStoreBaseCardMixin:SelectCard`.
//!
//! Spec/source mismatch finding (PLAN.md task for
//! `AccountStoreBaseCardMixin:SelectCard` refund): the plan describes
//! the method as taking an `itemID` parameter and shows a refund popup
//! whose `OnAccept` calls `C_AccountStore.RefundItem(itemID)`. The
//! actual 25-line body at `Blizzard_AccountStoreCardTemplates.lua:172-196`
//! (already pinned in `behavior_card_select_purchase.rs` for the
//! purchase arm) diverges from the PLAN spec in five symmetric ways
//! when the status is Refundable:
//!
//! ```lua
//! function AccountStoreBaseCardMixin:SelectCard()
//!     PlaySound(SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT);
//!
//!     local itemInfo = self.itemInfo;
//!     local isRefundable = itemInfo.status == Enum.AccountStoreItemStatus.Refundable;
//!     local confirmationFormat = isRefundable and ACCOUNT_STORE_REFUND_CONFIRMATION_FORMAT or PLUNDERSTORE_PURCHASE_CONFIRMATION_FORMAT;
//!     local confirmation = confirmationFormat:format(itemInfo.name, AccountStoreUtil.FormatCurrencyDisplay(itemInfo.price, itemInfo.currencyID));
//!
//!     if StaticPopup_Hide then
//!         StaticPopup_Hide("GENERIC_CONFIRMATION");
//!
//!         StaticPopup_ShowGenericConfirmation(confirmation, function ()
//!             if isRefundable then
//!                 PlaySound(SOUNDKIT.ACCOUNT_STORE_ITEM_REFUND);
//!                 C_AccountStore.RefundItem(itemInfo.id);
//!             else
//!                 PlaySound(SOUNDKIT.ACCOUNT_STORE_ITEM_PURCHASE);
//!                 C_AccountStore.BeginPurchase(itemInfo.id);
//!             end
//!         end);
//!     else
//!         local text2 = nil;
//!         StaticPopup_Show("ACCOUNT_STORE_BEGIN_PURCHASE_OR_REFUND", confirmation, text2, itemInfo);
//!     end
//! end
//! ```
//!
//! 1. **Parameter framing mismatch.** Same as the purchase branch:
//!    `SelectCard` takes NO caller-supplied arguments — only the
//!    implicit `self`. The PLAN-shaped `SelectCard(itemID)` signature
//!    does not exist; the body reads `self.itemInfo.id` from the cached
//!    table populated by `SetItemID` at lines 125-130.
//!
//! 2. **Refund branch shares the same method as the purchase branch.**
//!    The PLAN splits this into two sibling tasks
//!    (`behavior_card_select_purchase.rs` and `behavior_card_select_refund.rs`)
//!    as if independent code paths, but the source has ONE `SelectCard`
//!    method that branches internally on
//!    `itemInfo.status == Enum.AccountStoreItemStatus.Refundable` (line
//!    176). The dispatch shape, popup-API decision, and closure
//!    structure are identical between branches; only the inner two
//!    lines differ — for refund, line 185 plays
//!    `SOUNDKIT.ACCOUNT_STORE_ITEM_REFUND` and line 186 calls
//!    `C_AccountStore.RefundItem(itemInfo.id)`.
//!
//! 3. **Item id source mismatch.** Same as the purchase branch:
//!    `RefundItem` is called with `itemInfo.id` (line 186), not a
//!    caller-supplied `itemID`. The closure captures `itemInfo` (the
//!    local from line 175 = `self.itemInfo`) by upvalue. The
//!    `isRefundable` flag is ALSO captured by upvalue from line 176
//!    — once the popup is shown, swapping `self.itemInfo` to a
//!    non-refundable table would NOT route the closure into the purchase
//!    branch when the user accepts; the closure already locked in
//!    `isRefundable=true`.
//!
//! 4. **Confirmation format mismatch (refund-specific).** The format
//!    selected at line 177 is `ACCOUNT_STORE_REFUND_CONFIRMATION_FORMAT`
//!    when isRefundable is true (the `and` branch of the ternary). The
//!    actual format string at `data/global_strings.rs:22797` is
//!    `"Are you sure you want to refund %s for %s?"`. The substring
//!    "want to refund" is unique to the refund format (the purchase
//!    format at line 22780 is "Are you sure you want to purchase %s for
//!    %s?"). The PLAN's "refund popup" framing implies a separate popup
//!    template, but the actual implementation reuses the same
//!    `StaticPopup_ShowGenericConfirmation` call shape and only varies
//!    the format string passed in.
//!
//! 5. **PLAN-omitted PlaySound dual-call.** Same shape as the purchase
//!    branch but with the refund-specific second sound: line 173 plays
//!    `SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT` at entry, line 185 plays
//!    `SOUNDKIT.ACCOUNT_STORE_ITEM_REFUND` inside the closure when the
//!    user accepts. PLAN omits both. A regression dropping either
//!    constant would NOT be caught by a test that only pins the
//!    RefundItem call.
//!
//! Four tests pin the contract:
//!
//! - `select_card_refund_path_invokes_static_popup_show_generic_confirmation_with_refund_format_text`
//!   replaces `StaticPopup_ShowGenericConfirmation` with a tracker that
//!   captures `(text, callback)`; directly invokes
//!   `AccountStoreBaseCardMixin.SelectCard(stub_self)` with a stub
//!   itemInfo carrying status=Refundable; asserts the tracker received
//!   exactly one call, the captured callback is a function, and the
//!   captured text contains "want to refund" (the refund-specific
//!   substring of `ACCOUNT_STORE_REFUND_CONFIRMATION_FORMAT`). Pins the
//!   format-selection branch at line 177 and confirms the refund arm
//!   takes the modern popup branch under the smoke harness.
//! - `select_card_refund_callback_invokes_c_account_store_refund_item_with_self_item_info_id_not_caller_arg`
//!   captures the callback (as above), invokes the captured callback
//!   directly, asserts `C_AccountStore.RefundItem` was called exactly
//!   once with the sentinel value pre-seeded into `stub_self.itemInfo.id`,
//!   and asserts `C_AccountStore.BeginPurchase` was NOT called. Pins
//!   the refund-branch C_API call site and the upvalue-capture id source.
//! - `select_card_refund_path_plays_item_select_at_entry_and_item_refund_inside_callback`
//!   replaces PlaySound with a tracker that records the full
//!   call-order list; invokes SelectCard then the captured callback;
//!   asserts the recorded sequence is exactly
//!   `[ACCOUNT_STORE_ITEM_SELECT, ACCOUNT_STORE_ITEM_REFUND]` —
//!   pins both PlaySound calls and their order, refuting the PLAN's
//!   silent omission of the audio side effects.
//! - `select_card_refund_branch_does_not_call_purchase_path_play_sound_or_begin_purchase`
//!   asserts the refund branch is fully exclusive of the purchase
//!   branch: after firing the captured callback for a Refundable item,
//!   `BeginPurchase` was not called and the recorded PlaySound log
//!   does not contain `SOUNDKIT.ACCOUNT_STORE_ITEM_PURCHASE`. Pins the
//!   isRefundable comparison at line 184 and the absence of fall-through
//!   between the if/else arms.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccountStore";

#[test]
fn select_card_refund_path_invokes_static_popup_show_generic_confirmation_with_refund_format_text()
{
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ITEM_ID_SENTINEL: i64 = 6_666_111;

        seed_select_card_refund_trackers(env, ITEM_ID_SENTINEL);

        env.eval::<()>(
            r#"
            AccountStoreBaseCardMixin.SelectCard(_G.__behavior_card_refund_stub_self)
            return
            "#,
        )
        .expect(
            "Direct invocation of `AccountStoreBaseCardMixin.SelectCard(stub_self)` for a \
             Refundable item must run cleanly — the body reads stub_self.itemInfo (status=Refundable), \
             selects ACCOUNT_STORE_REFUND_CONFIRMATION_FORMAT at line 177, formats the confirmation \
             string, and routes through the modern popup branch (StaticPopup_Hide is defined under \
             the smoke harness)",
        );

        let (show_call_count, captured_callback_type, captured_text_contains_refund_keyword): (
            i64,
            String,
            bool,
        ) = env
            .eval(
                r#"
                return _G.__behavior_card_refund_show_generic_call_count,
                       type(_G.__behavior_card_refund_show_generic_callback),
                       string.find(
                           _G.__behavior_card_refund_show_generic_text or "",
                           "want to refund",
                           1,
                           true
                       ) ~= nil
                "#,
            )
            .expect("post-SelectCard StaticPopup_ShowGenericConfirmation tracker probe must run cleanly");

        assert_eq!(
            show_call_count, 1,
            "Expected `StaticPopup_ShowGenericConfirmation` to have been invoked exactly once \
             after `AccountStoreBaseCardMixin.SelectCard(stub_self)` for a Refundable item. The \
             body at `Blizzard_AccountStoreCardTemplates.lua:183` calls the modern popup API \
             unconditionally inside the `if StaticPopup_Hide then` branch — same dispatch site as \
             the purchase arm, only the format string differs. A zero count means either (a) the \
             gate fell through to the fallback branch (a regression in StaticPopup_Hide \
             registration), or (b) the body errored before reaching the call (most likely an \
             itemInfo-shape mismatch — the stub provides name, price, currencyID, status, id which \
             are the only fields the body reads)."
        );

        assert_eq!(
            captured_callback_type, "function",
            "Expected the captured callback (second arg to StaticPopup_ShowGenericConfirmation) \
             to be a function — same closure shape as the purchase branch, only the inner body \
             differs. A non-function reading means the body started passing some other value \
             (e.g. a table descriptor) — forcing a re-pin against the new dispatch shape."
        );

        assert!(
            captured_text_contains_refund_keyword,
            "Expected the captured confirmation text to contain \"want to refund\" — the unique \
             substring of `ACCOUNT_STORE_REFUND_CONFIRMATION_FORMAT` (\"Are you sure you want to \
             refund %s for %s?\" per `data/global_strings.rs:22797`). The body at line 177 selects \
             the refund format only when `isRefundable` is true (line 176: \
             `itemInfo.status == Enum.AccountStoreItemStatus.Refundable`). A reading without \
             \"want to refund\" means either (a) the format-selection logic took the purchase arm \
             instead (the stub seeded status=Refundable=2 per `missing_enums.lua:288`, so a \
             misroute would indicate the enum value or the `==` semantics changed), or (b) the \
             refund format string was renamed (forcing a re-pin against the new global)."
        );

        teardown_select_card_refund_trackers(env);
    });
}

#[test]
fn select_card_refund_callback_invokes_c_account_store_refund_item_with_self_item_info_id_not_caller_arg()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ITEM_ID_SENTINEL: i64 = 6_666_222;

        seed_select_card_refund_trackers(env, ITEM_ID_SENTINEL);

        env.eval::<()>(
            r#"
            AccountStoreBaseCardMixin.SelectCard(_G.__behavior_card_refund_stub_self)
            _G.__behavior_card_refund_show_generic_callback()
            return
            "#,
        )
        .expect(
            "SelectCard + captured-callback invocation must run cleanly — the captured callback \
             is the closure at `Blizzard_AccountStoreCardTemplates.lua:183-191` whose body \
             branches on isRefundable (true here, status=Refundable), runs PlaySound (stubbed), \
             and calls C_AccountStore.RefundItem(itemInfo.id) — both PlaySound and RefundItem are \
             tracked by the seed",
        );

        let (refund_item_count, refund_item_arg, begin_purchase_count): (i64, i64, i64) = env
            .eval(
                r#"
                return _G.__behavior_card_refund_refund_item_call_count,
                       _G.__behavior_card_refund_refund_item_arg or -1,
                       _G.__behavior_card_refund_begin_purchase_call_count
                "#,
            )
            .expect("post-callback C_AccountStore tracker probe must run cleanly");

        assert_eq!(
            refund_item_count, 1,
            "Expected `C_AccountStore.RefundItem` to have been invoked exactly once after firing \
             the captured popup callback for a Refundable item. The closure at \
             `Blizzard_AccountStoreCardTemplates.lua:186` calls `C_AccountStore.RefundItem(\
             itemInfo.id)` unconditionally inside the `if isRefundable then` branch (refundable \
             status). A zero count means either (a) the closure took the non-refundable branch \
             (line 187-189) — but the stub seeded status=Refundable (2) which IS \
             Enum.AccountStoreItemStatus.Refundable, so this would mean the enum value or the \
             comparison shape changed, or (b) the closure errored before reaching the RefundItem \
             call (the only line above is PlaySound, which the test stubs to a no-op)."
        );

        assert_eq!(
            refund_item_arg, ITEM_ID_SENTINEL,
            "Expected `C_AccountStore.RefundItem` to have been called with the sentinel \
             ({ITEM_ID_SENTINEL}) — the value pre-seeded into `stub_self.itemInfo.id`. The \
             closure at line 186 reads `itemInfo.id` BY NAME — `itemInfo` is the local at line 175 \
             (`local itemInfo = self.itemInfo`) captured as an upvalue, NOT a caller-supplied \
             argument. A different recorded value means either (a) the body switched to a \
             different field (e.g. `self.itemID` from line 126), or (b) the closure now accepts a \
             parameter and the caller passes a different id (the PLAN-shaped \
             `RefundItem(itemID)` flow with `itemID` as the SelectCard parameter — forcing a \
             re-pin against the new signature)."
        );

        assert_eq!(
            begin_purchase_count, 0,
            "Expected `C_AccountStore.BeginPurchase` to NOT have been called for a refundable \
             item — the closure's `if isRefundable then` branch at line 184 routes refundable \
             items to RefundItem and skips the purchase arm entirely. The stub's status=Refundable \
             (2) takes the if-branch (RefundItem), so BeginPurchase must not fire. A non-zero \
             count means either (a) both branches are now executed (a regression in branch logic \
             where the if/else became unconditional), or (b) the isRefundable comparison was \
             inverted/broken — the body uses `==` against `Enum.AccountStoreItemStatus.Refundable` \
             per line 176; a regression in the enum lookup or the `==` operator semantics could \
             surface here."
        );

        teardown_select_card_refund_trackers(env);
    });
}

#[test]
fn select_card_refund_path_plays_item_select_at_entry_and_item_refund_inside_callback() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ITEM_ID_SENTINEL: i64 = 6_666_333;
        const ITEM_SELECT_SOUND_ID: i64 = 444_111;
        const ITEM_REFUND_SOUND_ID: i64 = 444_222;

        env.eval::<()>(&format!(
            r#"
            SOUNDKIT = SOUNDKIT or {{}}
            SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT = {ITEM_SELECT_SOUND_ID}
            SOUNDKIT.ACCOUNT_STORE_ITEM_REFUND = {ITEM_REFUND_SOUND_ID}
            return
            "#
        ))
        .expect("seeding SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT/_REFUND sentinels must run cleanly");

        seed_select_card_refund_trackers(env, ITEM_ID_SENTINEL);

        env.eval::<()>(
            r#"
            _G.__behavior_card_refund_play_sound_log = {}
            PlaySound = function(sound_kit_id)
                table.insert(_G.__behavior_card_refund_play_sound_log, sound_kit_id)
            end
            return
            "#,
        )
        .expect("replacing PlaySound with a logging tracker must run cleanly");

        env.eval::<()>(
            r#"
            AccountStoreBaseCardMixin.SelectCard(_G.__behavior_card_refund_stub_self)
            local pre_callback_log_length = #_G.__behavior_card_refund_play_sound_log
            _G.__behavior_card_refund_pre_callback_log_length = pre_callback_log_length
            _G.__behavior_card_refund_show_generic_callback()
            return
            "#,
        )
        .expect("SelectCard + captured-callback invocation must run cleanly");

        let (pre_callback_count, total_count, first_id, second_id): (i64, i64, i64, i64) = env
            .eval(
                r#"
                local log = _G.__behavior_card_refund_play_sound_log
                return _G.__behavior_card_refund_pre_callback_log_length,
                       #log,
                       log[1] or -1,
                       log[2] or -1
                "#,
            )
            .expect("post-callback PlaySound log probe must run cleanly");

        assert_eq!(
            pre_callback_count, 1,
            "Expected exactly one PlaySound call BEFORE the captured callback fires. The body at \
             `Blizzard_AccountStoreCardTemplates.lua:173` calls \
             `PlaySound(SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT)` as the first statement of SelectCard, \
             BEFORE any popup work — same entry-sound site shared by both arms. The closure body \
             (line 185 for refund) plays a SECOND sound only after the user accepts. A \
             pre-callback count of 0 means the entry PlaySound was removed; a count > 1 means the \
             popup-show path is now playing additional sounds before user acceptance."
        );

        assert_eq!(
            first_id, ITEM_SELECT_SOUND_ID,
            "Expected the first PlaySound call to use the sentinel for \
             SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT ({ITEM_SELECT_SOUND_ID}) — the body at line 173 \
             reads `SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT` BY NAME, so a sentinel-seed proves the \
             constant name is literally `SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT`. A different reading \
             means the constant was renamed (forcing a re-pin against the new name)."
        );

        assert_eq!(
            total_count, 2,
            "Expected exactly two PlaySound calls total (entry + callback). The body fires one at \
             line 173 (entry) and one at line 185 (inside the closure for the refundable branch). \
             A total of 1 means the closure-branch sound was dropped (a regression in audio \
             feedback for the refund path); a total > 2 means a third PlaySound site was added \
             (forcing a re-pin against the new sequence)."
        );

        assert_eq!(
            second_id, ITEM_REFUND_SOUND_ID,
            "Expected the second PlaySound call to use the sentinel for \
             SOUNDKIT.ACCOUNT_STORE_ITEM_REFUND ({ITEM_REFUND_SOUND_ID}) — the closure at line 185 \
             reads `SOUNDKIT.ACCOUNT_STORE_ITEM_REFUND` BY NAME inside the `if isRefundable then` \
             branch. The companion `behavior_card_select_purchase.rs` test pins the symmetric \
             purchase-branch case (`SOUNDKIT.ACCOUNT_STORE_ITEM_PURCHASE`). A different reading \
             here means either (a) the constant was renamed, or (b) the closure took the \
             non-refundable branch instead — but the stub status=Refundable (2) guards against \
             that, so a misread would indicate a regression in the \
             Enum.AccountStoreItemStatus.Refundable comparison."
        );

        teardown_select_card_refund_trackers(env);
        env.eval::<()>(
            r#"
            _G.__behavior_card_refund_play_sound_log = nil
            _G.__behavior_card_refund_pre_callback_log_length = nil
            return
            "#,
        )
        .expect("PlaySound log tear-down must run cleanly");
    });
}

#[test]
fn select_card_refund_branch_does_not_call_purchase_path_play_sound_or_begin_purchase() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ITEM_ID_SENTINEL: i64 = 6_666_444;
        const ITEM_PURCHASE_SOUND_ID: i64 = 333_111;
        const ITEM_REFUND_SOUND_ID: i64 = 333_222;

        env.eval::<()>(&format!(
            r#"
            SOUNDKIT = SOUNDKIT or {{}}
            SOUNDKIT.ACCOUNT_STORE_ITEM_PURCHASE = {ITEM_PURCHASE_SOUND_ID}
            SOUNDKIT.ACCOUNT_STORE_ITEM_REFUND = {ITEM_REFUND_SOUND_ID}
            return
            "#
        ))
        .expect("seeding SOUNDKIT.ACCOUNT_STORE_ITEM_PURCHASE/_REFUND sentinels must run cleanly");

        seed_select_card_refund_trackers(env, ITEM_ID_SENTINEL);

        env.eval::<()>(
            r#"
            _G.__behavior_card_refund_play_sound_log = {}
            PlaySound = function(sound_kit_id)
                table.insert(_G.__behavior_card_refund_play_sound_log, sound_kit_id)
            end
            AccountStoreBaseCardMixin.SelectCard(_G.__behavior_card_refund_stub_self)
            _G.__behavior_card_refund_show_generic_callback()
            return
            "#,
        )
        .expect("SelectCard + captured-callback invocation must run cleanly");

        let (begin_purchase_count, purchase_sound_seen): (i64, bool) = env
            .eval(&format!(
                r#"
                local log = _G.__behavior_card_refund_play_sound_log
                local saw_purchase_sound = false
                for i = 1, #log do
                    if log[i] == {ITEM_PURCHASE_SOUND_ID} then
                        saw_purchase_sound = true
                    end
                end
                return _G.__behavior_card_refund_begin_purchase_call_count, saw_purchase_sound
                "#
            ))
            .expect("post-callback purchase-arm exclusivity probe must run cleanly");

        assert_eq!(
            begin_purchase_count, 0,
            "Expected `C_AccountStore.BeginPurchase` to NOT have been called for a refundable \
             item — the refund branch at line 184-186 calls RefundItem and the purchase branch at \
             line 187-189 is fenced behind the `else`. A non-zero count proves the branches are \
             no longer mutually exclusive (a regression where both arms execute, or where the \
             else became unconditional)."
        );

        assert!(
            !purchase_sound_seen,
            "Expected `SOUNDKIT.ACCOUNT_STORE_ITEM_PURCHASE` ({ITEM_PURCHASE_SOUND_ID}) to NOT \
             appear in the PlaySound log — the closure's `else` arm at line 188 plays this \
             constant for non-refundable items. The stub seeded status=Refundable (2), so the \
             purchase-sound site at line 188 is dead under this test. A `true` reading would \
             prove the if/else exclusivity broke and the closure now plays both sounds."
        );

        teardown_select_card_refund_trackers(env);
        env.eval::<()>(
            r#"
            _G.__behavior_card_refund_play_sound_log = nil
            return
            "#,
        )
        .expect("PlaySound log tear-down must run cleanly");
    });
}

fn seed_select_card_refund_trackers(env: &WowLuaEnv, item_id: i64) {
    seed_refund_stub_self_with_item_info(env, item_id);
    seed_refund_show_generic_confirmation_tracker(env);
    seed_refund_static_popup_hide_stub(env);
    seed_refund_c_account_store_trackers(env);
    seed_refund_play_sound_silent_stub(env);
    seed_refund_format_currency_display_stub(env);
}

fn seed_refund_stub_self_with_item_info(env: &WowLuaEnv, item_id: i64) {
    env.eval::<()>(&format!(
        r#"
        local item_info = {{}}
        item_info.id = {item_id}
        item_info.name = "BehaviorCardRefundSentinelItemName"
        item_info.price = 100
        item_info.currencyID = 1
        item_info.status = Enum.AccountStoreItemStatus.Refundable
        local stub_self = {{}}
        stub_self.itemInfo = item_info
        _G.__behavior_card_refund_stub_self = stub_self
        return
        "#
    ))
    .expect("seeding stub_self with sentinel itemInfo (status=Refundable) must run cleanly");
}

fn seed_refund_show_generic_confirmation_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_refund_show_generic_call_count = 0
        _G.__behavior_card_refund_show_generic_text = nil
        _G.__behavior_card_refund_show_generic_callback = nil
        _G.__behavior_card_refund_original_show_generic = StaticPopup_ShowGenericConfirmation
        StaticPopup_ShowGenericConfirmation = function(text, callback)
            _G.__behavior_card_refund_show_generic_call_count =
                _G.__behavior_card_refund_show_generic_call_count + 1
            _G.__behavior_card_refund_show_generic_text = text
            _G.__behavior_card_refund_show_generic_callback = callback
        end
        return
        "#,
    )
    .expect("seeding StaticPopup_ShowGenericConfirmation tracker must run cleanly");
}

fn seed_refund_static_popup_hide_stub(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_refund_original_static_popup_hide = StaticPopup_Hide
        StaticPopup_Hide = function() end
        return
        "#,
    )
    .expect("seeding StaticPopup_Hide silent stub must run cleanly");
}

fn seed_refund_c_account_store_trackers(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_refund_refund_item_call_count = 0
        _G.__behavior_card_refund_refund_item_arg = nil
        _G.__behavior_card_refund_original_refund_item = C_AccountStore.RefundItem
        C_AccountStore.RefundItem = function(item_id)
            _G.__behavior_card_refund_refund_item_call_count =
                _G.__behavior_card_refund_refund_item_call_count + 1
            _G.__behavior_card_refund_refund_item_arg = item_id
        end
        _G.__behavior_card_refund_begin_purchase_call_count = 0
        _G.__behavior_card_refund_original_begin_purchase = C_AccountStore.BeginPurchase
        C_AccountStore.BeginPurchase = function()
            _G.__behavior_card_refund_begin_purchase_call_count =
                _G.__behavior_card_refund_begin_purchase_call_count + 1
        end
        return
        "#,
    )
    .expect("seeding C_AccountStore.RefundItem + BeginPurchase trackers must run cleanly");
}

fn seed_refund_play_sound_silent_stub(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_refund_original_play_sound = PlaySound
        PlaySound = function() end
        return
        "#,
    )
    .expect("seeding PlaySound silent stub must run cleanly");
}

fn seed_refund_format_currency_display_stub(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_refund_original_format_currency = AccountStoreUtil.FormatCurrencyDisplay
        AccountStoreUtil.FormatCurrencyDisplay = function(price, _currency_id)
            return tostring(price) .. "g"
        end
        return
        "#,
    )
    .expect("seeding AccountStoreUtil.FormatCurrencyDisplay stub must run cleanly");
}

fn teardown_select_card_refund_trackers(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        StaticPopup_ShowGenericConfirmation = _G.__behavior_card_refund_original_show_generic
        StaticPopup_Hide = _G.__behavior_card_refund_original_static_popup_hide
        C_AccountStore.RefundItem = _G.__behavior_card_refund_original_refund_item
        C_AccountStore.BeginPurchase = _G.__behavior_card_refund_original_begin_purchase
        PlaySound = _G.__behavior_card_refund_original_play_sound
        AccountStoreUtil.FormatCurrencyDisplay = _G.__behavior_card_refund_original_format_currency
        _G.__behavior_card_refund_original_show_generic = nil
        _G.__behavior_card_refund_original_static_popup_hide = nil
        _G.__behavior_card_refund_original_refund_item = nil
        _G.__behavior_card_refund_original_begin_purchase = nil
        _G.__behavior_card_refund_original_play_sound = nil
        _G.__behavior_card_refund_original_format_currency = nil
        _G.__behavior_card_refund_show_generic_call_count = nil
        _G.__behavior_card_refund_show_generic_text = nil
        _G.__behavior_card_refund_show_generic_callback = nil
        _G.__behavior_card_refund_refund_item_call_count = nil
        _G.__behavior_card_refund_refund_item_arg = nil
        _G.__behavior_card_refund_begin_purchase_call_count = nil
        _G.__behavior_card_refund_stub_self = nil
        return
        "#,
    )
    .expect("SelectCard refund tracker tear-down must run cleanly");
}
