//! Behavior pin for `AccountStoreBaseCardMixin:UpdateRefundTime` and the
//! per-card OnUpdate throttle.
//!
//! Spec/source mismatch finding (PLAN.md task: when `GetItemInfo` reports
//! `refundSecondsRemaining > 0`, the card's refund-time tooltip text formats
//! the duration via `SecondsToTime` and updates on OnUpdate as the timer
//! counts down): the plan makes four claims that diverge from the actual
//! source at `Blizzard_AccountStoreCardTemplates.lua:2, 96-103, 120-123,
//! 198-223` and `Blizzard_AccountStoreCardTemplates.xml:48`.
//!
//! 1. **The formatter is `RefundTimeFormatter:Format`, NOT `SecondsToTime`.**
//!    Lines 120-123 read:
//!
//!    ```lua
//!    local RefundTimeFormatter = CreateFromMixins(SecondsFormatterMixin);
//!    RefundTimeFormatter:Init(0, SecondsFormatter.Abbreviation.OneLetter, SecondsFormatter.Interval.Minutes, true, true);
//!    RefundTimeFormatter:SetStripIntervalWhitespace(true);
//!    RefundTimeFormatter:SetMinInterval(SecondsFormatter.Interval.Minutes);
//!    ```
//!
//!    `RefundTimeFormatter` is a file-local `SecondsFormatterMixin` instance
//!    configured for one-letter abbreviations ("1m", "2h") at minutes
//!    interval. The PLAN-named `SecondsToTime` is a different global
//!    (defined in FrameXML for HH:MM:SS-style readouts) and is NOT invoked
//!    on the refund-text path.
//!
//! 2. **The text is written to `self.RefundText` (a FontString on the card),
//!    NOT a tooltip.** XML at line 48 declares
//!    `<FontString parentKey="RefundText" inherits="GameFontNormal" ...>` as
//!    a layout-anchored ARTWORK FontString on the card — anchored TOP to
//!    BuyButton's BOTTOM, x=10/-10 left/right padding. The PLAN's "tooltip
//!    text" framing implies a hover-only GameTooltip readout; the actual
//!    surface is always-visible card chrome (gated by `:SetShown(refundable)`
//!    at line 217).
//!
//! 3. **The gate is `(status == Refundable) AND refundSecondsRemaining`,
//!    NOT `refundSecondsRemaining > 0`.** Line 216 reads:
//!
//!    ```lua
//!    local refundable = (itemInfo.status == Enum.AccountStoreItemStatus.Refundable) and itemInfo.refundSecondsRemaining;
//!    ```
//!
//!    The PLAN names only the seconds-remaining condition. The actual gate
//!    requires BOTH the status to be Refundable AND the seconds field to be
//!    truthy. A non-Refundable status with a non-zero seconds field hides
//!    the refund text entirely. The seconds check is also a TRUTHINESS
//!    check, not a numeric `> 0`: `0` is truthy in Lua and would still
//!    trigger the refund-text branch (with `RefundTimeFormatter:Format(0)`
//!    rendering as "0m" or similar).
//!
//! 4. **OnUpdate is THROTTLED to 1.0s cadence and indirects through
//!    `CheckForItemStateUpdate`.** Line 2 declares
//!    `local AccountStoreCardUpdateCadenceSeconds = 1.0`. The OnUpdate body
//!    at lines 96-103 accumulates dt into `self.timeSinceUpdate` and only
//!    fires `CheckForItemStateUpdate` when the accumulator exceeds
//!    `AccountStoreCardUpdateCadenceSeconds`. CheckForItemStateUpdate
//!    (lines 198-212) re-reads `C_AccountStore.GetItemInfo(self.itemID)`
//!    and only calls `UpdateRefundTime` when `refundSecondsRemaining`
//!    differs from the cached value. The PLAN's "updates on OnUpdate as
//!    the timer counts down" framing implies per-tick refresh; the actual
//!    behavior is at-most-once-per-second AND only when GetItemInfo
//!    reports a different value.
//!
//! Five tests pin the contract:
//!
//! - `update_refund_time_method_exists_on_account_store_base_card_mixin` —
//!   surface check that `AccountStoreBaseCardMixin.UpdateRefundTime` is a
//!   function. A non-function reading would prove the method moved off the
//!   mixin (e.g. onto a per-card-type override).
//!
//! - `seconds_to_time_is_a_callable_global_but_update_refund_time_does_not_invoke_it`
//!   — replaces `SecondsToTime` with a tracker that returns a sentinel
//!   string; invokes UpdateRefundTime on a stub card with status=Refundable
//!   and refundSecondsRemaining=300; asserts the SecondsToTime tracker
//!   received ZERO calls AND the captured RefundText text does NOT contain
//!   the sentinel. PLAN tripwire: a non-zero call count would prove the
//!   refund text path was wired through SecondsToTime (matching PLAN's
//!   claim).
//!
//! - `update_refund_time_does_not_set_refund_text_when_status_is_not_refundable`
//!   — seeds a stub card with status=Owned and refundSecondsRemaining=300
//!   (a positive seconds value that the PLAN-named `> 0` gate alone would
//!   trip); invokes UpdateRefundTime; asserts the captured shown_state is
//!   falsy AND no SetText call landed AND the OnUpdate handler was cleared
//!   (set to nil). Pins the AND-of-status-and-seconds gate.
//!
//! - `update_refund_time_sets_refund_text_with_localized_format_when_refundable`
//!   — seeds a stub card with status=Refundable and
//!   refundSecondsRemaining=300; invokes UpdateRefundTime; asserts the
//!   captured shown_state is truthy, the captured text contains the
//!   `ACCOUNT_STORE_REFUND_TEXT_FORMAT` prefix ("Time left to refund:"),
//!   and the captured OnUpdate handler is the mixin's OnUpdate function
//!   (not nil). Pins the positive-path SetText shape.
//!
//! - `account_store_base_card_on_update_throttles_check_for_item_state_update_to_one_second_cadence`
//!   — replaces `CheckForItemStateUpdate` on the stub card with a tracker;
//!   invokes OnUpdate(stub, 0.5), OnUpdate(stub, 0.4), OnUpdate(stub, 0.2)
//!   (cumulative dt = 1.1, crossing the 1.0s cadence on the third call);
//!   asserts the tracker fired exactly ONCE. Then invokes OnUpdate(stub,
//!   0.5) again (post-reset accumulator at 0.5, below cadence); asserts
//!   the tracker is still at 1. Pins the throttle and the
//!   reset-after-fire semantics. A 0-call reading on the third call
//!   would prove the cadence was raised; a 2+ reading on the fourth call
//!   would prove the accumulator wasn't reset to 0 after firing.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccountStore";

#[test]
fn update_refund_time_method_exists_on_account_store_base_card_mixin() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let update_refund_time_type: String = env
            .eval("return type(AccountStoreBaseCardMixin.UpdateRefundTime)")
            .expect("AccountStoreBaseCardMixin.UpdateRefundTime probe must run cleanly");

        assert_eq!(
            update_refund_time_type, "function",
            "Expected `type(AccountStoreBaseCardMixin.UpdateRefundTime) == \"function\"` \
             (`Blizzard_AccountStoreCardTemplates.lua:214-223`), got \
             `{update_refund_time_type}`. A non-function reading would prove the method moved \
             off the base card mixin (e.g. onto a per-card-type override like \
             AccountStoreCreatureCardMixin) — forcing a re-pin against the new dispatch path \
             and likely breaking the SetItemID->UpdateRefundTime call chain at line 138."
        );
    });
}

#[test]
fn seconds_to_time_is_a_callable_global_but_update_refund_time_does_not_invoke_it() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let seconds_to_time_type: String = env
            .eval("return type(SecondsToTime)")
            .expect("SecondsToTime global probe must run cleanly");

        assert_eq!(
            seconds_to_time_type, "function",
            "Expected `type(SecondsToTime) == \"function\"` — confirms the PLAN-named function \
             is a real callable global (defined in FrameXML for HH:MM:SS-style readouts), so \
             the spec/source mismatch is in the dispatch path, not in the function's \
             existence. Got `{seconds_to_time_type}`."
        );

        seed_seconds_to_time_tracker(env);
        seed_refundable_stub_card(env, /*seconds_remaining=*/ 300);

        env.eval::<()>(
            r#"
            AccountStoreBaseCardMixin.UpdateRefundTime(_G.__behavior_refund_timer_stub_card)
            return
            "#,
        )
        .expect("UpdateRefundTime invocation must run cleanly");

        let (call_count, text_contains_sentinel): (i64, bool) = env
            .eval(
                r#"
                local count = _G.__behavior_refund_timer_seconds_to_time_calls or 0
                local text = _G.__behavior_refund_timer_stub_card.RefundText.__text or ""
                return count, string.find(text, "BEHAVIOR_REFUND_TIMER_SECONDS_TO_TIME_SENTINEL", 1, true) ~= nil
                "#,
            )
            .expect("SecondsToTime tracker readout must run cleanly");

        assert_eq!(
            call_count, 0,
            "Expected ZERO `SecondsToTime` calls during UpdateRefundTime — the actual \
             formatter at `Blizzard_AccountStoreCardTemplates.lua:120-123` is a file-local \
             `SecondsFormatterMixin` instance (`RefundTimeFormatter`) configured for \
             one-letter abbreviations at minutes interval; the dispatch path goes through \
             `RefundTimeFormatter:Format(refundSecondsRemaining)` at line 220, NOT through \
             `SecondsToTime`. Got {call_count}. A non-zero reading would prove the PLAN's \
             claim came true (a real upstream change) — and would also break the \
             one-letter-abbreviation contract (SecondsToTime returns HH:MM:SS strings, not \
             abbreviations)."
        );

        assert!(
            !text_contains_sentinel,
            "Expected the captured RefundText text NOT to contain the SecondsToTime sentinel — \
             this confirms the formatter output (the actual `RefundTimeFormatter:Format` \
             return value) was used for the SetText call, not the SecondsToTime tracker's \
             return. A true reading would prove SecondsToTime's return leaked into the \
             refund text."
        );

        teardown_refundable_stub_card(env);
        teardown_seconds_to_time_tracker(env);
    });
}

#[test]
fn update_refund_time_does_not_set_refund_text_when_status_is_not_refundable() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_owned_stub_card_with_seconds(env, /*seconds_remaining=*/ 300);

        env.eval::<()>(
            r#"
            AccountStoreBaseCardMixin.UpdateRefundTime(_G.__behavior_refund_timer_stub_card)
            return
            "#,
        )
        .expect("UpdateRefundTime invocation must run cleanly");

        let (shown_truthy, text_was_set, on_update_is_nil): (bool, bool, bool) = env
            .eval(
                r#"
                local card = _G.__behavior_refund_timer_stub_card
                local shown = card.RefundText.__shown
                local text = card.RefundText.__text
                local handler = card.__on_update_handler
                return shown and true or false, text ~= nil, handler == nil
                "#,
            )
            .expect("stub-card readout must run cleanly");

        assert!(
            !shown_truthy,
            "Expected `RefundText:SetShown(refundable)` to be called with a falsy `refundable` \
             when status is Owned (not Refundable). The gate at \
             `Blizzard_AccountStoreCardTemplates.lua:216` reads \
             `(status == Enum.AccountStoreItemStatus.Refundable) and refundSecondsRemaining`. \
             With status=Owned, the AND short-circuits to `false` regardless of \
             refundSecondsRemaining. A truthy reading would prove the status gate was dropped \
             (PLAN's `refundSecondsRemaining > 0` standalone gate would behave this way — and \
             the test seeds refundSecondsRemaining=300 specifically to exercise that path)."
        );

        assert!(
            !text_was_set,
            "Expected `RefundText:SetText(...)` NOT to be called when refundable is falsy — \
             the SetText call at line 221 is INSIDE the `if refundable then` guard at line \
             219. A true reading would prove the SetText call leaked outside the guard."
        );

        assert!(
            on_update_is_nil,
            "Expected `SetScript(\"OnUpdate\", nil)` when refundable is falsy — line 218 \
             reads `self:SetScript(\"OnUpdate\", refundable and self.OnUpdate or nil)`. \
             With refundable=false, the expression short-circuits to nil, clearing the \
             OnUpdate handler. A non-nil reading would prove the OnUpdate handler stayed \
             registered (a regression that would burn cycles on every frame for non-refundable \
             cards)."
        );

        teardown_refundable_stub_card(env);
    });
}

#[test]
fn update_refund_time_sets_refund_text_with_localized_format_when_refundable() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_refundable_stub_card(env, /*seconds_remaining=*/ 300);

        env.eval::<()>(
            r#"
            AccountStoreBaseCardMixin.UpdateRefundTime(_G.__behavior_refund_timer_stub_card)
            return
            "#,
        )
        .expect("UpdateRefundTime invocation must run cleanly");

        let (shown_truthy, text_contains_localized_prefix, handler_is_mixin_on_update): (
            bool,
            bool,
            bool,
        ) = env
            .eval(
                r#"
                local card = _G.__behavior_refund_timer_stub_card
                local shown = card.RefundText.__shown
                local text = card.RefundText.__text or ""
                local has_prefix = string.find(text, "Time left to refund:", 1, true) ~= nil
                local handler_match = card.__on_update_handler == AccountStoreBaseCardMixin.OnUpdate
                return shown and true or false, has_prefix, handler_match
                "#,
            )
            .expect("stub-card readout must run cleanly");

        assert!(
            shown_truthy,
            "Expected `RefundText:SetShown(refundable)` to be called with a truthy value when \
             status=Refundable AND refundSecondsRemaining=300. The gate at \
             `Blizzard_AccountStoreCardTemplates.lua:216` evaluates to the seconds value \
             itself (300), which is truthy in Lua. A falsy reading would prove the status \
             check was inverted or the seconds-truthiness check was tightened to `> 0`."
        );

        assert!(
            text_contains_localized_prefix,
            "Expected the captured RefundText text to contain the localized prefix \
             \"Time left to refund:\" — `ACCOUNT_STORE_REFUND_TEXT_FORMAT` resolves to \
             \"Time left to refund: |cffffffff%s|r\" per `data/global_strings.rs:22796`. \
             Line 221 reads `self.RefundText:SetText(ACCOUNT_STORE_REFUND_TEXT_FORMAT:format(timeString))`. \
             A false reading would prove either the localization key changed (forcing a \
             re-pin against the new prefix) or the format-call shape changed (e.g. the \
             format-vs-set-text order was inverted, or the prefix was wrapped in additional \
             color codes)."
        );

        assert!(
            handler_is_mixin_on_update,
            "Expected `SetScript(\"OnUpdate\", self.OnUpdate)` to register the mixin's \
             OnUpdate function when refundable is truthy — line 218's `refundable and \
             self.OnUpdate or nil` resolves to `self.OnUpdate` for the refundable case. The \
             stub card seeds `card.OnUpdate = AccountStoreBaseCardMixin.OnUpdate`, so the \
             captured handler should be byte-equal to the mixin's OnUpdate. A mismatch \
             reading would prove the OnUpdate dispatcher was rerouted (e.g. wrapped in a \
             closure or replaced with a different per-card handler)."
        );

        teardown_refundable_stub_card(env);
    });
}

#[test]
fn account_store_base_card_on_update_throttles_check_for_item_state_update_to_one_second_cadence() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_throttle_stub_card(env);

        env.eval::<()>(
            r#"
            local card = _G.__behavior_refund_timer_throttle_stub_card
            AccountStoreBaseCardMixin.OnUpdate(card, 0.5)
            AccountStoreBaseCardMixin.OnUpdate(card, 0.4)
            AccountStoreBaseCardMixin.OnUpdate(card, 0.2)
            return
            "#,
        )
        .expect("OnUpdate sequence (0.5+0.4+0.2 = 1.1) must run cleanly");

        let calls_after_threshold_crossed: i64 = env
            .eval("return _G.__behavior_refund_timer_check_calls or 0")
            .expect("check-calls readout must run cleanly");

        assert_eq!(
            calls_after_threshold_crossed, 1,
            "Expected exactly ONE `CheckForItemStateUpdate` call after OnUpdate(0.5) + \
             OnUpdate(0.4) + OnUpdate(0.2) (cumulative dt = 1.1, crossing the 1.0s cadence on \
             the third call) — `AccountStoreCardUpdateCadenceSeconds` at \
             `Blizzard_AccountStoreCardTemplates.lua:2` is 1.0, and the OnUpdate body at \
             lines 96-103 only fires CheckForItemStateUpdate when `timeSinceUpdate > \
             AccountStoreCardUpdateCadenceSeconds`. Got {calls_after_threshold_crossed}. A \
             zero reading would prove the cadence was raised above 1.1s (or the comparison \
             was tightened to `>=`); a value > 1 would prove the throttle was removed."
        );

        env.eval::<()>(
            r#"
            local card = _G.__behavior_refund_timer_throttle_stub_card
            AccountStoreBaseCardMixin.OnUpdate(card, 0.5)
            return
            "#,
        )
        .expect("post-reset OnUpdate(0.5) must run cleanly");

        let calls_after_post_reset: i64 = env
            .eval("return _G.__behavior_refund_timer_check_calls or 0")
            .expect("post-reset check-calls readout must run cleanly");

        assert_eq!(
            calls_after_post_reset, 1,
            "Expected the call count to STAY at 1 after a fourth OnUpdate(0.5) call (the \
             accumulator was reset to 0 on the third call's fire path at line 100, so 0.5 is \
             well below the 1.0s cadence). Got {calls_after_post_reset}. A value > 1 would \
             prove the accumulator wasn't reset on fire (line 100: `self.timeSinceUpdate = \
             0;` is what makes the throttle bound the call rate to once-per-cadence rather \
             than once-then-every-tick)."
        );

        teardown_throttle_stub_card(env);
    });
}

fn seed_seconds_to_time_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_refund_timer_seconds_to_time_calls = 0
        _G.__behavior_refund_timer_original_seconds_to_time = SecondsToTime
        SecondsToTime = function(seconds, _noseconds, _notabs, _maxcount, _roundup)
            _G.__behavior_refund_timer_seconds_to_time_calls =
                _G.__behavior_refund_timer_seconds_to_time_calls + 1
            return "BEHAVIOR_REFUND_TIMER_SECONDS_TO_TIME_SENTINEL"
        end
        return
        "#,
    )
    .expect("seeding SecondsToTime tracker must run cleanly");
}

fn teardown_seconds_to_time_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        SecondsToTime = _G.__behavior_refund_timer_original_seconds_to_time
        _G.__behavior_refund_timer_original_seconds_to_time = nil
        _G.__behavior_refund_timer_seconds_to_time_calls = nil
        return
        "#,
    )
    .expect("SecondsToTime tracker tear-down must run cleanly");
}

fn seed_refundable_stub_card(env: &WowLuaEnv, seconds_remaining: i64) {
    seed_stub_card_with_status_and_seconds(
        env,
        "Enum.AccountStoreItemStatus.Refundable",
        seconds_remaining,
    );
}

fn seed_owned_stub_card_with_seconds(env: &WowLuaEnv, seconds_remaining: i64) {
    seed_stub_card_with_status_and_seconds(
        env,
        "Enum.AccountStoreItemStatus.Owned",
        seconds_remaining,
    );
}

fn seed_stub_card_with_status_and_seconds(
    env: &WowLuaEnv,
    status_expr: &str,
    seconds_remaining: i64,
) {
    env.eval::<()>(&format!(
        r#"
        local refund_text = {{}}
        refund_text.__shown = nil
        refund_text.__text = nil
        refund_text.SetShown = function(self, shown) self.__shown = shown end
        refund_text.SetText = function(self, text) self.__text = text end

        local card = {{}}
        card.RefundText = refund_text
        card.itemInfo = {{
            status = {status_expr},
            refundSecondsRemaining = {seconds_remaining},
        }}
        card.OnUpdate = AccountStoreBaseCardMixin.OnUpdate
        card.__on_update_handler = "BEHAVIOR_REFUND_TIMER_HANDLER_NEVER_SET_SENTINEL"
        card.SetScript = function(self, _name, handler) self.__on_update_handler = handler end

        _G.__behavior_refund_timer_stub_card = card
        return
        "#
    ))
    .expect("seeding stub card must run cleanly");
}

fn teardown_refundable_stub_card(env: &WowLuaEnv) {
    env.eval::<()>("_G.__behavior_refund_timer_stub_card = nil; return")
        .expect("stub card tear-down must run cleanly");
}

fn seed_throttle_stub_card(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_refund_timer_check_calls = 0
        local card = {}
        card.timeSinceUpdate = 0
        card.CheckForItemStateUpdate = function(_self)
            _G.__behavior_refund_timer_check_calls =
                _G.__behavior_refund_timer_check_calls + 1
        end
        _G.__behavior_refund_timer_throttle_stub_card = card
        return
        "#,
    )
    .expect("seeding throttle stub card must run cleanly");
}

fn teardown_throttle_stub_card(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_refund_timer_throttle_stub_card = nil
        _G.__behavior_refund_timer_check_calls = nil
        return
        "#,
    )
    .expect("throttle stub card tear-down must run cleanly");
}
