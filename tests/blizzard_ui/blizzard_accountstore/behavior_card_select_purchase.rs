//! Behavior pin for the purchase branch of `AccountStoreBaseCardMixin:SelectCard`.
//!
//! Spec/source mismatch finding (PLAN.md task for
//! `AccountStoreBaseCardMixin:SelectCard` purchase): the plan
//! describes the method as taking an `itemID` parameter and shows a
//! confirmation popup whose `OnAccept` calls
//! `C_AccountStore.BeginPurchase(itemID)`. The actual 25-line body at
//! `Blizzard_AccountStoreCardTemplates.lua:172-196` diverges in five
//! ways:
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
//! 1. **Parameter framing mismatch.** `SelectCard` takes NO caller-
//!    supplied arguments — only the implicit `self`. The PLAN sentence
//!    `SelectCard(itemID)` implies the caller passes the item id, but
//!    the body reads `self.itemInfo.id` (line 189) — the cached id
//!    from the table that `SetItemID` populated at lines 125-130 via
//!    `C_AccountStore.GetItemInfo(itemID)`. Real callers
//!    (`Blizzard_AccountStoreCardTemplates.lua:32` —
//!    `OnLoad`-installed mouse-up callback fires `self:SelectCard()`)
//!    invoke it without parameters.
//!
//! 2. **Single method handles BOTH purchase AND refund.** PLAN.md
//!    splits this into two sibling tasks (`behavior_card_select_purchase.rs`
//!    here and `behavior_card_select_refund.rs` next) as if they were
//!    independent code paths, but the source has ONE `SelectCard`
//!    method that branches internally on
//!    `itemInfo.status == Enum.AccountStoreItemStatus.Refundable` to
//!    pick the purchase or refund route. Both routes share the same
//!    popup-API decision (`if StaticPopup_Hide then`), the same
//!    confirmation-format selection, and the same closure structure;
//!    only the inner two lines (sound + C_API call) differ. This file
//!    pins the purchase branch; the refund branch sibling file pins
//!    the same method's other arm.
//!
//! 3. **Item id source mismatch.** `BeginPurchase` is called with
//!    `itemInfo.id` (line 189), not a caller-supplied `itemID`. The
//!    closure captures `itemInfo` (the local from line 175 = `self.itemInfo`)
//!    by upvalue; mutating `self.itemInfo` between SelectCard and the
//!    user clicking "Yes" would NOT change the id passed to
//!    BeginPurchase — the closure already captured the table reference.
//!    A fresh `SetItemID` on the same card AFTER SelectCard would
//!    swap `self.itemInfo` to a new table, but the popup's pending
//!    closure still points at the old table.
//!
//! 4. **Dual popup-API dispatch.** The body has TWO popup branches
//!    gated by `if StaticPopup_Hide then` (line 180). The "modern"
//!    branch (lines 181-191) calls
//!    `StaticPopup_ShowGenericConfirmation(confirmation, callback)` —
//!    with the PLAN-shaped OnAccept-as-callback semantics. The
//!    "fallback" branch (lines 192-195) calls
//!    `StaticPopup_Show("ACCOUNT_STORE_BEGIN_PURCHASE_OR_REFUND",
//!    confirmation, nil, itemInfo)` — a popup-by-name dispatch where
//!    the OnAccept lives in
//!    `Blizzard_StaticPopup_Glue/Mainline/GlueDialogDefs.lua:163-177`
//!    (the glue context, NOT the main UI). The PLAN's "OnAccept calls
//!    BeginPurchase" framing only describes the modern branch's
//!    closure shape; the fallback branch's OnAccept lives in a
//!    different file and uses `data` (the fourth arg) instead of an
//!    upvalue capture. Under the smoke harness the modern branch is
//!    always taken (StaticPopup_Hide is part of the core static-popup
//!    surface).
//!
//! 5. **PLAN-omitted PlaySound dual-call.** The body calls PlaySound
//!    TWICE per accept: once with `SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT`
//!    at entry (line 173, before any popup work) and once with
//!    `SOUNDKIT.ACCOUNT_STORE_ITEM_PURCHASE` inside the closure
//!    (line 188, before BeginPurchase). PLAN omits both. A regression
//!    that drops either constant would NOT be caught by a test that
//!    only pins the BeginPurchase call.
//!
//! Four tests pin the contract:
//!
//! - `select_card_is_a_function_that_reads_item_id_from_self_item_info_id_not_a_caller_argument`
//!   asserts SelectCard is a function, then calls
//!   `AccountStoreBaseCardMixin.SelectCard(stub_self, ITEM_ID_NEVER_PASSED)`
//!   with an extra positional arg, fires the captured callback, and
//!   asserts BeginPurchase received the value from
//!   `stub_self.itemInfo.id` (NOT the extra arg). Pins that SelectCard
//!   ignores caller-supplied positional args past `self` because the
//!   declaration has an empty parameter list.
//! - `select_card_purchase_path_invokes_static_popup_show_generic_confirmation_with_callback_function`
//!   replaces `StaticPopup_ShowGenericConfirmation` with a tracker
//!   that captures `(text, callback)`; directly invokes
//!   `AccountStoreBaseCardMixin.SelectCard(stub_self)` with a stub
//!   itemInfo carrying status=Unowned; asserts the tracker received
//!   exactly one call and the captured callback is a function. Pins
//!   the modern-branch dispatch shape and confirms the StaticPopup_Hide
//!   gate routes through the closure-callback branch under the smoke
//!   harness.
//! - `select_card_purchase_callback_invokes_c_account_store_begin_purchase_with_self_item_info_id_not_caller_arg`
//!   captures the callback (as above), invokes the captured callback
//!   directly, asserts `C_AccountStore.BeginPurchase` was called
//!   exactly once with the sentinel value pre-seeded into
//!   `stub_self.itemInfo.id`, and asserts `C_AccountStore.RefundItem`
//!   was NOT called. Pins the purchase-branch C_API call site and
//!   the upvalue-capture id source.
//! - `select_card_purchase_path_plays_item_select_at_entry_and_item_purchase_inside_callback`
//!   replaces PlaySound with a tracker that records the full
//!   call-order list; invokes SelectCard then the captured callback;
//!   asserts the recorded sequence is exactly
//!   `[ACCOUNT_STORE_ITEM_SELECT, ACCOUNT_STORE_ITEM_PURCHASE]` —
//!   pins both PlaySound calls and their order, refuting the PLAN's
//!   silent omission of the audio side effects.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccountStore";

#[test]
fn select_card_is_a_function_that_reads_item_id_from_self_item_info_id_not_a_caller_argument() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let fn_type: String = env
            .eval("return type(AccountStoreBaseCardMixin.SelectCard)")
            .expect("`AccountStoreBaseCardMixin.SelectCard` type probe must run cleanly");

        assert_eq!(
            fn_type, "function",
            "Precondition: `AccountStoreBaseCardMixin.SelectCard` must be a function — defined \
             at `Blizzard_AccountStoreCardTemplates.lua:172-196`. A non-function reading means \
             the mixin definition was removed or renamed (forcing a re-pin against the new \
             entry-point shape)."
        );

        const ITEM_ID_FROM_SELF: i64 = 8_888_001;
        const ITEM_ID_NEVER_PASSED: i64 = 9_999_999;

        seed_select_card_trackers(env, ITEM_ID_FROM_SELF);

        env.eval::<()>(&format!(
            r#"
            AccountStoreBaseCardMixin.SelectCard(
                _G.__behavior_card_purchase_stub_self,
                {ITEM_ID_NEVER_PASSED}
            )
            _G.__behavior_card_purchase_show_generic_callback()
            return
            "#
        ))
        .expect(
            "SelectCard + captured callback must run cleanly with an extra positional arg — \
             Lua silently drops args past the declared parameter list, so passing \
             ITEM_ID_NEVER_PASSED as a second arg should NOT propagate to BeginPurchase",
        );

        let begin_purchase_arg: i64 = env
            .eval("return _G.__behavior_card_purchase_begin_purchase_arg or -1")
            .expect("post-callback BeginPurchase arg probe must run cleanly");

        assert_eq!(
            begin_purchase_arg, ITEM_ID_FROM_SELF,
            "Expected `C_AccountStore.BeginPurchase` to have been called with \
             {ITEM_ID_FROM_SELF} (the value pre-seeded into `stub_self.itemInfo.id`), NOT \
             {ITEM_ID_NEVER_PASSED} (the extra arg passed to SelectCard). The body at \
             `Blizzard_AccountStoreCardTemplates.lua:172` declares \
             `function AccountStoreBaseCardMixin:SelectCard()` with an EMPTY parameter list — \
             the colon-method gives an implicit `self` and no other parameter. Lua semantics \
             silently discard extra positional args at call time. The closure at line 189 \
             reads `itemInfo.id` from the local at line 175 (`local itemInfo = self.itemInfo`), \
             which is captured by upvalue from the enclosing scope. A reading of \
             {ITEM_ID_NEVER_PASSED} here means SelectCard now takes a second parameter that \
             reaches into the closure (the PLAN-shaped \"SelectCard(itemID)\" signature — \
             forcing a re-pin against the new contract and likely retiring the \
             `self.itemInfo` cache pattern at line 175)."
        );

        teardown_select_card_trackers(env);
    });
}

#[test]
fn select_card_purchase_path_invokes_static_popup_show_generic_confirmation_with_callback_function()
{
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ITEM_ID_SENTINEL: i64 = 7_777_111;

        seed_select_card_trackers(env, ITEM_ID_SENTINEL);

        env.eval::<()>(
            r#"
            AccountStoreBaseCardMixin.SelectCard(_G.__behavior_card_purchase_stub_self)
            return
            "#,
        )
        .expect(
            "Direct invocation of `AccountStoreBaseCardMixin.SelectCard(stub_self)` must run \
             cleanly — the body reads stub_self.itemInfo (provided by the seed), formats a \
             confirmation string, and routes through the modern popup branch (StaticPopup_Hide \
             is defined under the smoke harness)",
        );

        let (show_call_count, captured_callback_type, captured_text_starts_with_format_prefix): (
            i64,
            String,
            bool,
        ) = env
            .eval(
                r#"
                return _G.__behavior_card_purchase_show_generic_call_count,
                       type(_G.__behavior_card_purchase_show_generic_callback),
                       string.find(
                           _G.__behavior_card_purchase_show_generic_text or "",
                           "Are you sure",
                           1,
                           true
                       ) ~= nil
                "#,
            )
            .expect("post-SelectCard StaticPopup_ShowGenericConfirmation tracker probe must run cleanly");

        assert_eq!(
            show_call_count, 1,
            "Expected `StaticPopup_ShowGenericConfirmation` to have been invoked exactly once \
             after `AccountStoreBaseCardMixin.SelectCard(stub_self)`. The body at \
             `Blizzard_AccountStoreCardTemplates.lua:183` calls \
             `StaticPopup_ShowGenericConfirmation(confirmation, callback)` unconditionally \
             inside the modern-popup branch (lines 180-191, gated by `if StaticPopup_Hide then`). \
             The smoke harness loads `Blizzard_StaticPopup` so `StaticPopup_Hide` is defined and \
             the modern branch is always taken. A zero count means either (a) the gate fell \
             through to the fallback branch (a regression in StaticPopup_Hide registration), or \
             (b) the body errored before reaching the call (most likely an itemInfo-shape \
             mismatch — the stub provides name, price, currencyID, status, id which are the \
             only fields the body reads)."
        );

        assert_eq!(
            captured_callback_type, "function",
            "Expected the captured callback (second arg to StaticPopup_ShowGenericConfirmation) \
             to be a function — the body at `Blizzard_AccountStoreCardTemplates.lua:183-191` \
             passes an inline `function () ... end` closure. A non-function reading means the \
             body started passing some other value (e.g. a table descriptor) — forcing a re-pin \
             against the new dispatch shape and likely a refactor of the popup-acceptance \
             pipeline."
        );

        assert!(
            captured_text_starts_with_format_prefix,
            "Expected the captured confirmation text to contain \"Are you sure\" — the prefix of \
             `PLUNDERSTORE_PURCHASE_CONFIRMATION_FORMAT` (\"Are you sure you want to purchase \
             %s for %s?\" per `data/global_strings.rs:22780`). The body at line 178 formats the \
             selected confirmationFormat with itemInfo.name and the formatted price; for a \
             non-refundable status the format selected at line 177 is \
             PLUNDERSTORE_PURCHASE_CONFIRMATION_FORMAT (the `or` branch). A different prefix \
             reading would indicate the format-selection logic changed (e.g. the refund format \
             leaked into the purchase path) or the global string itself was renamed."
        );

        teardown_select_card_trackers(env);
    });
}

#[test]
fn select_card_purchase_callback_invokes_c_account_store_begin_purchase_with_self_item_info_id_not_caller_arg()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ITEM_ID_SENTINEL: i64 = 7_777_222;

        seed_select_card_trackers(env, ITEM_ID_SENTINEL);

        env.eval::<()>(
            r#"
            AccountStoreBaseCardMixin.SelectCard(_G.__behavior_card_purchase_stub_self)
            _G.__behavior_card_purchase_show_generic_callback()
            return
            "#,
        )
        .expect(
            "SelectCard + captured-callback invocation must run cleanly — the captured callback \
             is the closure at `Blizzard_AccountStoreCardTemplates.lua:183-191` whose body \
             branches on isRefundable (false here, status=Unowned), runs PlaySound (stubbed), \
             and calls C_AccountStore.BeginPurchase(itemInfo.id) — both PlaySound and \
             BeginPurchase are tracked by the seed",
        );

        let (begin_purchase_count, begin_purchase_arg, refund_item_count): (i64, i64, i64) = env
            .eval(
                r#"
                return _G.__behavior_card_purchase_begin_purchase_call_count,
                       _G.__behavior_card_purchase_begin_purchase_arg or -1,
                       _G.__behavior_card_purchase_refund_item_call_count
                "#,
            )
            .expect("post-callback C_AccountStore tracker probe must run cleanly");

        assert_eq!(
            begin_purchase_count, 1,
            "Expected `C_AccountStore.BeginPurchase` to have been invoked exactly once after \
             firing the captured popup callback. The closure at \
             `Blizzard_AccountStoreCardTemplates.lua:189` calls `C_AccountStore.BeginPurchase(\
             itemInfo.id)` unconditionally inside the `else` branch (non-refundable status). A \
             zero count means either (a) the closure took the refundable branch (line 184-186) \
             — but the stub seeded status=Unowned (1) which is NOT \
             Enum.AccountStoreItemStatus.Refundable (2), so this would mean the enum value or \
             the comparison shape changed, or (b) the closure errored before reaching the \
             BeginPurchase call (the only line above is PlaySound, which the test stubs to a \
             no-op, so an error there would have to be in the SOUNDKIT lookup itself)."
        );

        assert_eq!(
            begin_purchase_arg, ITEM_ID_SENTINEL,
            "Expected `C_AccountStore.BeginPurchase` to have been called with the sentinel \
             ({ITEM_ID_SENTINEL}) — the value pre-seeded into `stub_self.itemInfo.id`. The \
             closure at line 189 reads `itemInfo.id` BY NAME — `itemInfo` is the local at \
             line 175 (`local itemInfo = self.itemInfo`) captured as an upvalue, NOT a \
             caller-supplied argument. A different recorded value means either (a) the body \
             switched to a different field (e.g. `self.itemID` from line 126, which would be a \
             behavior change because itemID and itemInfo.id can diverge if SetItemID is called \
             with an id that GetItemInfo doesn't recognize — at lines 131-134 the body returns \
             early without setting itemInfo, so a stale itemInfo could persist), or (b) the \
             closure now accepts a parameter and the caller passes a different id (the \
             PLAN-shaped `BeginPurchase(itemID)` flow with `itemID` as the SelectCard parameter \
             — forcing a re-pin against the new signature)."
        );

        assert_eq!(
            refund_item_count, 0,
            "Expected `C_AccountStore.RefundItem` to NOT have been called for a non-refundable \
             item — the closure's `if isRefundable then` branch at line 184 routes refundable \
             items to RefundItem. The stub's status=Unowned (1) takes the `else` branch \
             (BeginPurchase), so RefundItem must not fire. A non-zero count means either (a) \
             both branches are now executed (a regression in branch logic), or (b) the \
             isRefundable comparison was inverted/broken (the body uses `==` against \
             `Enum.AccountStoreItemStatus.Refundable` per line 176 — a regression in the enum \
             lookup or the `==` operator semantics could surface here)."
        );

        teardown_select_card_trackers(env);
    });
}

#[test]
fn select_card_purchase_path_plays_item_select_at_entry_and_item_purchase_inside_callback() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ITEM_ID_SENTINEL: i64 = 7_777_333;
        const ITEM_SELECT_SOUND_ID: i64 = 555_111;
        const ITEM_PURCHASE_SOUND_ID: i64 = 555_222;

        env.eval::<()>(&format!(
            r#"
            SOUNDKIT = SOUNDKIT or {{}}
            SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT = {ITEM_SELECT_SOUND_ID}
            SOUNDKIT.ACCOUNT_STORE_ITEM_PURCHASE = {ITEM_PURCHASE_SOUND_ID}
            return
            "#
        ))
        .expect("seeding SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT/_PURCHASE sentinels must run cleanly");

        seed_select_card_trackers(env, ITEM_ID_SENTINEL);

        env.eval::<()>(
            r#"
            _G.__behavior_card_purchase_play_sound_log = {}
            PlaySound = function(sound_kit_id)
                table.insert(_G.__behavior_card_purchase_play_sound_log, sound_kit_id)
            end
            return
            "#,
        )
        .expect("replacing PlaySound with a logging tracker must run cleanly");

        env.eval::<()>(
            r#"
            AccountStoreBaseCardMixin.SelectCard(_G.__behavior_card_purchase_stub_self)
            local pre_callback_log_length = #_G.__behavior_card_purchase_play_sound_log
            _G.__behavior_card_purchase_pre_callback_log_length = pre_callback_log_length
            _G.__behavior_card_purchase_show_generic_callback()
            return
            "#,
        )
        .expect("SelectCard + captured-callback invocation must run cleanly");

        let (pre_callback_count, total_count, first_id, second_id): (i64, i64, i64, i64) = env
            .eval(
                r#"
                local log = _G.__behavior_card_purchase_play_sound_log
                return _G.__behavior_card_purchase_pre_callback_log_length,
                       #log,
                       log[1] or -1,
                       log[2] or -1
                "#,
            )
            .expect("post-callback PlaySound log probe must run cleanly");

        assert_eq!(
            pre_callback_count, 1,
            "Expected exactly one PlaySound call BEFORE the captured callback fires. The body \
             at `Blizzard_AccountStoreCardTemplates.lua:173` calls \
             `PlaySound(SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT)` as the first statement of \
             SelectCard, BEFORE any popup work. The closure body (line 188) plays a SECOND \
             sound only after the user accepts. A pre-callback count of 0 means the entry \
             PlaySound was removed (a regression that would silently drop the click feedback), \
             and a count > 1 means the popup-show path is now playing additional sounds before \
             the user accepts (worth investigating because StaticPopup_ShowGenericConfirmation \
             itself does not play sounds in the simulator)."
        );

        assert_eq!(
            first_id, ITEM_SELECT_SOUND_ID,
            "Expected the first PlaySound call to use the sentinel for \
             SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT ({ITEM_SELECT_SOUND_ID}) — the body at line 173 \
             reads `SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT` BY NAME, so a sentinel-seed proves the \
             constant name is literally `SOUNDKIT.ACCOUNT_STORE_ITEM_SELECT` (not e.g. \
             `SOUNDKIT.ACCOUNT_STORE_CARD_SELECT` or a runtime-config'd alias). A different \
             reading means the constant was renamed (forcing a re-pin against the new name)."
        );

        assert_eq!(
            total_count, 2,
            "Expected exactly two PlaySound calls total (entry + callback). The body fires one \
             at line 173 (entry) and one at line 188 (inside the closure for non-refundable). A \
             total of 1 means the closure-branch sound was dropped (a regression in audio \
             feedback for the purchase path); a total > 2 means a third PlaySound site was \
             added (forcing a re-pin against the new sequence)."
        );

        assert_eq!(
            second_id, ITEM_PURCHASE_SOUND_ID,
            "Expected the second PlaySound call to use the sentinel for \
             SOUNDKIT.ACCOUNT_STORE_ITEM_PURCHASE ({ITEM_PURCHASE_SOUND_ID}) — the closure at \
             line 188 reads `SOUNDKIT.ACCOUNT_STORE_ITEM_PURCHASE` BY NAME inside the `else` \
             (non-refundable) branch. The companion `behavior_card_select_refund.rs` test \
             pins the symmetric refund-branch case (`SOUNDKIT.ACCOUNT_STORE_ITEM_REFUND`). A \
             different reading here means either (a) the constant was renamed, or (b) the \
             closure took the refundable branch instead — but the stub status=Unowned (1) \
             guards against that, so a misread would indicate a regression in the \
             Enum.AccountStoreItemStatus.Refundable comparison."
        );

        teardown_select_card_trackers(env);
        env.eval::<()>(
            r#"
            _G.__behavior_card_purchase_play_sound_log = nil
            _G.__behavior_card_purchase_pre_callback_log_length = nil
            return
            "#,
        )
        .expect("PlaySound log tear-down must run cleanly");
    });
}

fn seed_select_card_trackers(env: &WowLuaEnv, item_id: i64) {
    seed_stub_self_with_item_info(env, item_id);
    seed_purchase_popup_trackers(env);
    seed_purchase_api_trackers(env);
    seed_purchase_side_effect_stubs(env);
}

fn seed_stub_self_with_item_info(env: &WowLuaEnv, item_id: i64) {
    env.eval::<()>(&format!(
        r#"
        local item_info = {{}}
        item_info.id = {item_id}
        item_info.name = "BehaviorCardPurchaseSentinelItemName"
        item_info.price = 100
        item_info.currencyID = 1
        item_info.status = Enum.AccountStoreItemStatus.Unowned
        local stub_self = {{}}
        stub_self.itemInfo = item_info
        _G.__behavior_card_purchase_stub_self = stub_self
        return
        "#
    ))
    .expect("seeding stub_self with sentinel itemInfo must run cleanly");
}

fn seed_purchase_popup_trackers(env: &WowLuaEnv) {
    seed_show_generic_confirmation_tracker(env);
    seed_static_popup_hide_stub(env);
}

fn seed_show_generic_confirmation_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_purchase_show_generic_call_count = 0
        _G.__behavior_card_purchase_show_generic_text = nil
        _G.__behavior_card_purchase_show_generic_callback = nil
        _G.__behavior_card_purchase_original_show_generic = StaticPopup_ShowGenericConfirmation
        StaticPopup_ShowGenericConfirmation = function(text, callback)
            _G.__behavior_card_purchase_show_generic_call_count =
                _G.__behavior_card_purchase_show_generic_call_count + 1
            _G.__behavior_card_purchase_show_generic_text = text
            _G.__behavior_card_purchase_show_generic_callback = callback
        end
        return
        "#,
    )
    .expect("seeding StaticPopup_ShowGenericConfirmation tracker must run cleanly");
}

fn seed_static_popup_hide_stub(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_purchase_original_static_popup_hide = StaticPopup_Hide
        StaticPopup_Hide = function() end
        return
        "#,
    )
    .expect("seeding StaticPopup_Hide silent stub must run cleanly");
}

fn seed_purchase_api_trackers(env: &WowLuaEnv) {
    seed_c_account_store_trackers(env);
}

fn seed_c_account_store_trackers(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_purchase_begin_purchase_call_count = 0
        _G.__behavior_card_purchase_begin_purchase_arg = nil
        _G.__behavior_card_purchase_original_begin_purchase = C_AccountStore.BeginPurchase
        C_AccountStore.BeginPurchase = function(item_id)
            _G.__behavior_card_purchase_begin_purchase_call_count =
                _G.__behavior_card_purchase_begin_purchase_call_count + 1
            _G.__behavior_card_purchase_begin_purchase_arg = item_id
        end
        _G.__behavior_card_purchase_refund_item_call_count = 0
        _G.__behavior_card_purchase_original_refund_item = C_AccountStore.RefundItem
        C_AccountStore.RefundItem = function()
            _G.__behavior_card_purchase_refund_item_call_count =
                _G.__behavior_card_purchase_refund_item_call_count + 1
        end
        return
        "#,
    )
    .expect("seeding C_AccountStore.BeginPurchase + RefundItem trackers must run cleanly");
}

fn seed_purchase_side_effect_stubs(env: &WowLuaEnv) {
    seed_play_sound_silent_stub(env);
    seed_format_currency_display_stub(env);
}

fn seed_play_sound_silent_stub(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_purchase_original_play_sound = PlaySound
        PlaySound = function() end
        return
        "#,
    )
    .expect("seeding PlaySound silent stub must run cleanly");
}

fn seed_format_currency_display_stub(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_purchase_original_format_currency = AccountStoreUtil.FormatCurrencyDisplay
        AccountStoreUtil.FormatCurrencyDisplay = function(price, _currency_id)
            return tostring(price) .. "g"
        end
        return
        "#,
    )
    .expect("seeding AccountStoreUtil.FormatCurrencyDisplay stub must run cleanly");
}

fn teardown_select_card_trackers(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        StaticPopup_ShowGenericConfirmation = _G.__behavior_card_purchase_original_show_generic
        StaticPopup_Hide = _G.__behavior_card_purchase_original_static_popup_hide
        C_AccountStore.BeginPurchase = _G.__behavior_card_purchase_original_begin_purchase
        C_AccountStore.RefundItem = _G.__behavior_card_purchase_original_refund_item
        PlaySound = _G.__behavior_card_purchase_original_play_sound
        AccountStoreUtil.FormatCurrencyDisplay = _G.__behavior_card_purchase_original_format_currency
        _G.__behavior_card_purchase_original_show_generic = nil
        _G.__behavior_card_purchase_original_static_popup_hide = nil
        _G.__behavior_card_purchase_original_begin_purchase = nil
        _G.__behavior_card_purchase_original_refund_item = nil
        _G.__behavior_card_purchase_original_play_sound = nil
        _G.__behavior_card_purchase_original_format_currency = nil
        _G.__behavior_card_purchase_show_generic_call_count = nil
        _G.__behavior_card_purchase_show_generic_text = nil
        _G.__behavior_card_purchase_show_generic_callback = nil
        _G.__behavior_card_purchase_begin_purchase_call_count = nil
        _G.__behavior_card_purchase_begin_purchase_arg = nil
        _G.__behavior_card_purchase_refund_item_call_count = nil
        _G.__behavior_card_purchase_stub_self = nil
        return
        "#,
    )
    .expect("SelectCard tracker tear-down must run cleanly");
}
