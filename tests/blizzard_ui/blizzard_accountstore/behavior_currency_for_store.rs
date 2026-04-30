//! Behavior pin for `OnStoreFrontSet` and the currency-for-store lookup that
//! populates the footer's `CurrencyAvailable` FontString.
//!
//! Spec/source mismatch finding (PLAN.md task: `AccountStoreFrame:OnStoreFrontSet`
//! queries `C_AccountStore.GetCurrencyIDForStore(storeFrontID)` to populate the
//! footer currency display, falling back to hidden when nil). Three claims
//! diverge from the actual source at `Blizzard_AccountStore.lua:42-48`,
//! `Blizzard_AccountStoreItemDisplay.lua:10-30, 58, 104-112, 177-184`.
//!
//! 1. **The method lives on `AccountStoreItemDisplayMixin`, NOT
//!    `AccountStoreFrame` / `AccountStoreMixin`.** `AccountStoreFrame` uses
//!    `AccountStoreMixin` (XML at `Blizzard_AccountStore.xml:78`) and exposes
//!    `OnLoad`, `OnShow`, `OnHide`, `SetStoreFrontID`, `SetFullscreenMode` —
//!    no `OnStoreFrontSet`. The frame-side method is
//!    `AccountStoreMixin:SetStoreFrontID(storeFrontID)` at lines 42-48, which
//!    caches `self.storeFrontID`, sets the title, and triggers
//!    `EventRegistry:TriggerEvent("AccountStore.StoreFrontSet", storeFrontID)`.
//!    `AccountStoreItemDisplayMixin:OnStoreFrontSet` is a SUBSCRIBER to that
//!    event registered at `Blizzard_AccountStoreItemDisplay.lua:58`:
//!    `self:AddStaticEventMethod(EventRegistry, "AccountStore.StoreFrontSet",
//!    self.OnStoreFrontSet)`.
//!
//! 2. **`GetCurrencyIDForStore` is called with `self.storeFrontID`, NOT the
//!    raw parameter.** Lines 104-112 read:
//!
//!    ```lua
//!    function AccountStoreItemDisplayMixin:OnStoreFrontSet(storeFrontID)
//!        self:InitializeStore(storeFrontID);
//!        if self.storeFrontID then
//!            C_AccountStore.RequestStoreFrontInfoUpdate(self.storeFrontID);
//!            self.currencyID = C_AccountStore.GetCurrencyIDForStore(self.storeFrontID);
//!            self:UpdateCurrencyAvailable();
//!        end
//!    end
//!    ```
//!
//!    `InitializeStore` at lines 10-30 sets `self.storeFrontID = storeFrontID`
//!    on first-store-or-change; afterwards, the method dispatches both
//!    `C_AccountStore` calls via `self.storeFrontID`. After InitializeStore
//!    runs, the parameter and the cached field are equal — but the access
//!    path is the cached field.
//!
//! 3. **There is NO fall-back-to-hidden when the currency lookup returns
//!    nil.** The PLAN says "falling back to hidden when nil"; the actual
//!    source has no `:SetShown(false)` call on `Footer.CurrencyAvailable` on
//!    any path. When `GetCurrencyIDForStore` returns nil:
//!    - `self.currencyID = nil` (line 109)
//!    - `self:UpdateCurrencyAvailable()` is still called (line 110)
//!    - `UpdateCurrencyAvailable` at lines 177-184 reads `self.currencyID`
//!      (now nil) and calls
//!      `self.Footer.CurrencyAvailable:SetText(AccountStoreUtil.FormatCurrencyDisplayWithWarning(currencyID))`
//!      — the formatter receives nil and returns whatever it formats nil as
//!      (an empty string, a localized "no currency" placeholder, etc.).
//!    - The footer FontString is NEVER hidden by this path. The PLAN-named
//!      "fall back to hidden when nil" behavior does not exist; the actual
//!      fall-back is "format the nil value through
//!      `FormatCurrencyDisplayWithWarning` and SetText the result".
//!
//! Five tests pin the contract:
//!
//! - `account_store_mixin_does_not_define_on_store_front_set` — surface
//!   tripwire that `AccountStoreMixin.OnStoreFrontSet` is `nil`. PLAN
//!   tripwire: a non-nil reading would prove the method moved onto the
//!   frame-side mixin (matching the PLAN's claim), forcing a re-pin against
//!   the new dispatch path.
//!
//! - `account_store_item_display_mixin_on_store_front_set_is_a_function` —
//!   surface tripwire that the actual mixin still exposes the method as a
//!   function. A non-function reading would prove the method moved off the
//!   item-display mixin (e.g. inlined into `SetStoreFrontID`).
//!
//! - `on_store_front_set_caches_store_front_id_via_initialize_store` —
//!   invokes `OnStoreFrontSet(stub, 42)` and asserts `stub.storeFrontID ==
//!   42`. Pins the InitializeStore caching contract: a mismatch would prove
//!   the method bypassed InitializeStore (the PLAN's framing implies the
//!   storeFrontID is used directly, not cached first).
//!
//! - `on_store_front_set_calls_request_and_get_currency_with_self_store_front_id_and_caches_currency`
//!   — replaces `C_AccountStore.RequestStoreFrontInfoUpdate` and
//!   `C_AccountStore.GetCurrencyIDForStore` with trackers that capture their
//!   first argument; invokes `OnStoreFrontSet(stub, 42)` (the
//!   GetCurrencyIDForStore tracker returns 1234 — a sentinel that
//!   distinguishes the cached currency ID from the storeFrontID). Asserts
//!   both trackers fired exactly once with the storeFrontID, AND
//!   `stub.currencyID == 1234`. Pins the dispatch path AND the cache step.
//!   A miscount would prove the method bypassed one of the C_AccountStore
//!   calls; an off-by-arg reading would prove the parameter was used
//!   directly instead of `self.storeFrontID`; a missing
//!   `stub.currencyID == 1234` would prove the cache step was dropped.
//!
//! - `on_store_front_set_does_not_hide_footer_currency_available_when_lookup_returns_nil`
//!   — replaces `C_AccountStore.GetCurrencyIDForStore` with a tracker that
//!   returns `nil`; replaces `AccountStoreUtil.FormatCurrencyDisplayWithWarning`
//!   with a sentinel-returning tracker; invokes `OnStoreFrontSet(stub, 42)`.
//!   Asserts `stub.currencyID` is nil, `stub.Footer.CurrencyAvailable.__shown`
//!   was NEVER assigned (no `:SetShown` call), `stub.Footer.CurrencyAvailable.__text`
//!   IS assigned (SetText was called with the formatter sentinel), AND the
//!   formatter tracker fired exactly once with a nil argument. Pins the
//!   "no fall-back-to-hidden" contract directly: a non-nil `__shown`
//!   reading would prove the PLAN's hidden-fallback claim came true; a
//!   missing `__text` reading would prove SetText was conditionally skipped.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccountStore";
const STORE_FRONT_ID: i64 = 42;
const CURRENCY_ID_SENTINEL: i64 = 1234;
const FORMATTER_SENTINEL: &str = "BEHAVIOR_CURRENCY_FOR_STORE_FORMATTER_SENTINEL";

#[test]
fn account_store_mixin_does_not_define_on_store_front_set() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let on_store_front_set_type: String = env
            .eval("return type(AccountStoreMixin.OnStoreFrontSet)")
            .expect("AccountStoreMixin.OnStoreFrontSet probe must run cleanly");

        assert_eq!(
            on_store_front_set_type, "nil",
            "Expected `type(AccountStoreMixin.OnStoreFrontSet) == \"nil\"` — \
             `AccountStoreMixin` (the mixin actually attached to AccountStoreFrame, per \
             `Blizzard_AccountStore.xml:78`) defines OnLoad/OnShow/OnHide/SetStoreFrontID/SetFullscreenMode \
             at `Blizzard_AccountStore.lua:18,26,31,42,50` and NOT OnStoreFrontSet. The frame-side \
             dispatch is `AccountStoreMixin:SetStoreFrontID` (line 42), which fires the \
             `EventRegistry` event \"AccountStore.StoreFrontSet\" (line 47) — that event is then \
             received by the SUBSCRIBER `AccountStoreItemDisplayMixin:OnStoreFrontSet` registered \
             at `Blizzard_AccountStoreItemDisplay.lua:58`. Got `{on_store_front_set_type}`. A \
             non-nil reading would prove the method was added to the frame-side mixin \
             (matching the PLAN-named dispatch path)."
        );
    });
}

#[test]
fn account_store_item_display_mixin_on_store_front_set_is_a_function() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let on_store_front_set_type: String = env
            .eval("return type(AccountStoreItemDisplayMixin.OnStoreFrontSet)")
            .expect("AccountStoreItemDisplayMixin.OnStoreFrontSet probe must run cleanly");

        assert_eq!(
            on_store_front_set_type, "function",
            "Expected `type(AccountStoreItemDisplayMixin.OnStoreFrontSet) == \"function\"` per \
             `Blizzard_AccountStoreItemDisplay.lua:104-112`. Got `{on_store_front_set_type}`. A \
             non-function reading would prove the method was inlined into the EventRegistry \
             handler closure or moved onto a different mixin — forcing a re-pin against the \
             new dispatch path."
        );
    });
}

#[test]
fn on_store_front_set_caches_store_front_id_via_initialize_store() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_currency_lookup_tracker(env, /*currency_id_or_nil_lua=*/ "nil");
        seed_format_currency_display_tracker(env);
        seed_request_store_front_info_update_tracker(env);
        seed_stub_display(env);

        env.eval::<()>(&format!(
            r#"
            AccountStoreItemDisplayMixin.OnStoreFrontSet(
                _G.__behavior_currency_for_store_stub_display, {STORE_FRONT_ID}
            )
            return
            "#
        ))
        .expect("OnStoreFrontSet invocation must run cleanly");

        let cached_store_front_id: i64 = env
            .eval("return _G.__behavior_currency_for_store_stub_display.storeFrontID")
            .expect("storeFrontID readout must run cleanly");

        assert_eq!(
            cached_store_front_id, STORE_FRONT_ID,
            "Expected `self.storeFrontID == {STORE_FRONT_ID}` after \
             `OnStoreFrontSet({STORE_FRONT_ID})` — line 105 calls `self:InitializeStore(storeFrontID)`, \
             and InitializeStore at lines 10-30 caches `self.storeFrontID = storeFrontID` on the \
             first-store-or-change branch (line 15). Got {cached_store_front_id}. A mismatch \
             would prove the method bypassed InitializeStore — which would also break the \
             subsequent `if self.storeFrontID` guard at line 107 and skip the C_AccountStore \
             dispatch entirely."
        );

        teardown_stub_display(env);
        teardown_request_store_front_info_update_tracker(env);
        teardown_format_currency_display_tracker(env);
        teardown_currency_lookup_tracker(env);
    });
}

#[test]
fn on_store_front_set_calls_request_and_get_currency_with_self_store_front_id_and_caches_currency()
{
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_currency_lookup_tracker(env, &CURRENCY_ID_SENTINEL.to_string());
        seed_format_currency_display_tracker(env);
        seed_request_store_front_info_update_tracker(env);
        seed_stub_display(env);

        env.eval::<()>(&format!(
            r#"
            AccountStoreItemDisplayMixin.OnStoreFrontSet(
                _G.__behavior_currency_for_store_stub_display, {STORE_FRONT_ID}
            )
            return
            "#
        ))
        .expect("OnStoreFrontSet invocation must run cleanly");

        let (request_calls, request_arg, currency_calls, currency_arg, cached_currency_id): (
            i64,
            i64,
            i64,
            i64,
            i64,
        ) = env
            .eval(
                r#"
                local stub = _G.__behavior_currency_for_store_stub_display
                return _G.__behavior_currency_for_store_request_calls or 0,
                       _G.__behavior_currency_for_store_request_arg or -1,
                       _G.__behavior_currency_for_store_currency_calls or 0,
                       _G.__behavior_currency_for_store_currency_arg or -1,
                       stub.currencyID or -1
                "#,
            )
            .expect("dispatch readout must run cleanly");

        assert_eq!(
            request_calls, 1,
            "Expected exactly ONE `C_AccountStore.RequestStoreFrontInfoUpdate` call after \
             OnStoreFrontSet — line 108 issues a single dispatch when `self.storeFrontID` is \
             truthy. Got {request_calls}. A zero reading would prove the dispatch was \
             rerouted; a value > 1 would prove a redundant call was added."
        );

        assert_eq!(
            request_arg, STORE_FRONT_ID,
            "Expected the RequestStoreFrontInfoUpdate call to receive `self.storeFrontID` (= \
             {STORE_FRONT_ID} after InitializeStore caches the parameter). Line 108 reads \
             `C_AccountStore.RequestStoreFrontInfoUpdate(self.storeFrontID)` — accessing the \
             cached field, not the raw parameter. Got {request_arg}. A mismatch would prove \
             either the access was switched to the parameter (which is observationally equal \
             but semantically different), or InitializeStore stopped caching the param."
        );

        assert_eq!(
            currency_calls, 1,
            "Expected exactly ONE `C_AccountStore.GetCurrencyIDForStore` call after \
             OnStoreFrontSet — line 109 issues a single lookup. Got {currency_calls}. A zero \
             reading would prove the cache step was bypassed; a value > 1 would prove the \
             cache was looked up multiple times (a regression that would re-dispatch for every \
             call site instead of using the cached `self.currencyID`)."
        );

        assert_eq!(
            currency_arg, STORE_FRONT_ID,
            "Expected GetCurrencyIDForStore to receive `self.storeFrontID` (= \
             {STORE_FRONT_ID}) per line 109. Got {currency_arg}. A mismatch would prove the \
             access path changed — likely to the parameter directly (PLAN's framing) or to a \
             constant (e.g. `Constants.AccountStoreConsts.PlunderstormStoreFrontID`, which is \
             the OTHER call site at line 49 inside the Footer.CurrencyAvailable OnEnter)."
        );

        assert_eq!(
            cached_currency_id, CURRENCY_ID_SENTINEL,
            "Expected `self.currencyID == {CURRENCY_ID_SENTINEL}` (the GetCurrencyIDForStore \
             tracker's sentinel return value) after the dispatch — line 109 reads \
             `self.currencyID = C_AccountStore.GetCurrencyIDForStore(self.storeFrontID)`. The \
             sentinel is intentionally distinct from {STORE_FRONT_ID} so a mismatch \
             distinguishes a cache-the-param bug from a cache-the-currency-id bug. Got \
             {cached_currency_id}. A reading of {STORE_FRONT_ID} would prove the cache stored \
             the parameter instead of the lookup result."
        );

        teardown_stub_display(env);
        teardown_request_store_front_info_update_tracker(env);
        teardown_format_currency_display_tracker(env);
        teardown_currency_lookup_tracker(env);
    });
}

#[test]
fn on_store_front_set_does_not_hide_footer_currency_available_when_lookup_returns_nil() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_currency_lookup_tracker(env, /*currency_id_or_nil_lua=*/ "nil");
        seed_format_currency_display_tracker(env);
        seed_request_store_front_info_update_tracker(env);
        seed_stub_display(env);

        env.eval::<()>(&format!(
            r#"
            AccountStoreItemDisplayMixin.OnStoreFrontSet(
                _G.__behavior_currency_for_store_stub_display, {STORE_FRONT_ID}
            )
            return
            "#
        ))
        .expect("OnStoreFrontSet invocation must run cleanly");

        let (
            currency_id_is_nil,
            footer_shown_was_assigned,
            footer_text_equals_sentinel,
            formatter_calls,
            formatter_arg_was_nil,
        ): (bool, bool, bool, i64, bool) = env
            .eval(
                r#"
                local stub = _G.__behavior_currency_for_store_stub_display
                local footer = stub.Footer.CurrencyAvailable
                return stub.currencyID == nil,
                       footer.__shown ~= "BEHAVIOR_CURRENCY_FOR_STORE_SHOWN_NEVER_SET",
                       footer.__text == "BEHAVIOR_CURRENCY_FOR_STORE_FORMATTER_SENTINEL",
                       _G.__behavior_currency_for_store_formatter_calls or 0,
                       _G.__behavior_currency_for_store_formatter_arg_was_nil == true
                "#,
            )
            .expect("nil-currency readout must run cleanly");

        assert!(
            currency_id_is_nil,
            "Expected `self.currencyID == nil` after GetCurrencyIDForStore returned nil — line \
             109 unconditionally assigns the lookup result to `self.currencyID`. A non-nil \
             reading would prove a fallback default was added (which would mask the real \
             nil-currency path from `UpdateCurrencyAvailable`)."
        );

        assert!(
            !footer_shown_was_assigned,
            "Expected `Footer.CurrencyAvailable:SetShown(...)` to NEVER be called on the \
             nil-currency path — `OnStoreFrontSet` -> `UpdateCurrencyAvailable` (lines 110, \
             177-184) only calls `SetText` on the footer; there is no `SetShown(false)` \
             anywhere in this dispatch chain. The PLAN-named \"fall back to hidden when nil\" \
             behavior does NOT exist. The stub seeds `Footer.CurrencyAvailable.__shown = \
             \"BEHAVIOR_CURRENCY_FOR_STORE_SHOWN_NEVER_SET\"` as a sentinel; if SetShown were \
             called the value would be overwritten. A `true` reading here would prove the \
             PLAN's hidden-fallback claim came true (a real upstream change)."
        );

        assert!(
            footer_text_equals_sentinel,
            "Expected `Footer.CurrencyAvailable:SetText(formatter_result)` to be called even \
             when currency_id is nil — line 179 calls SetText unconditionally with \
             `AccountStoreUtil.FormatCurrencyDisplayWithWarning(currencyID)`. The formatter \
             tracker returns the sentinel string `{FORMATTER_SENTINEL}`. A false reading \
             would prove SetText was conditionally skipped on nil currency (likely via a \
             newly-added nil guard in UpdateCurrencyAvailable)."
        );

        assert_eq!(
            formatter_calls, 1,
            "Expected exactly ONE `AccountStoreUtil.FormatCurrencyDisplayWithWarning` call \
             via UpdateCurrencyAvailable. Got {formatter_calls}. A zero reading would prove \
             UpdateCurrencyAvailable bailed early on nil currency (the PLAN's \
             hidden-fallback claim manifesting as an early return); a value > 1 would prove \
             SetText was called multiple times."
        );

        assert!(
            formatter_arg_was_nil,
            "Expected the formatter to receive `nil` as its currencyID argument — \
             UpdateCurrencyAvailable reads `local currencyID = self.currencyID` (line 178) \
             and passes it directly into FormatCurrencyDisplayWithWarning. A false reading \
             would prove the nil was coerced or replaced with a default before reaching the \
             formatter."
        );

        teardown_stub_display(env);
        teardown_request_store_front_info_update_tracker(env);
        teardown_format_currency_display_tracker(env);
        teardown_currency_lookup_tracker(env);
    });
}

fn seed_currency_lookup_tracker(env: &WowLuaEnv, currency_id_or_nil_lua: &str) {
    env.eval::<()>(&format!(
        r#"
        _G.__behavior_currency_for_store_currency_calls = 0
        _G.__behavior_currency_for_store_currency_arg = -1
        _G.__behavior_currency_for_store_original_get_currency =
            C_AccountStore.GetCurrencyIDForStore
        C_AccountStore.GetCurrencyIDForStore = function(store_front_id)
            _G.__behavior_currency_for_store_currency_calls =
                _G.__behavior_currency_for_store_currency_calls + 1
            _G.__behavior_currency_for_store_currency_arg = store_front_id
            return {currency_id_or_nil_lua}
        end
        return
        "#
    ))
    .expect("seeding GetCurrencyIDForStore tracker must run cleanly");
}

fn teardown_currency_lookup_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        C_AccountStore.GetCurrencyIDForStore =
            _G.__behavior_currency_for_store_original_get_currency
        _G.__behavior_currency_for_store_original_get_currency = nil
        _G.__behavior_currency_for_store_currency_calls = nil
        _G.__behavior_currency_for_store_currency_arg = nil
        return
        "#,
    )
    .expect("GetCurrencyIDForStore tracker tear-down must run cleanly");
}

fn seed_request_store_front_info_update_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_currency_for_store_request_calls = 0
        _G.__behavior_currency_for_store_request_arg = -1
        _G.__behavior_currency_for_store_original_request =
            C_AccountStore.RequestStoreFrontInfoUpdate
        C_AccountStore.RequestStoreFrontInfoUpdate = function(store_front_id)
            _G.__behavior_currency_for_store_request_calls =
                _G.__behavior_currency_for_store_request_calls + 1
            _G.__behavior_currency_for_store_request_arg = store_front_id
        end
        return
        "#,
    )
    .expect("seeding RequestStoreFrontInfoUpdate tracker must run cleanly");
}

fn teardown_request_store_front_info_update_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        C_AccountStore.RequestStoreFrontInfoUpdate =
            _G.__behavior_currency_for_store_original_request
        _G.__behavior_currency_for_store_original_request = nil
        _G.__behavior_currency_for_store_request_calls = nil
        _G.__behavior_currency_for_store_request_arg = nil
        return
        "#,
    )
    .expect("RequestStoreFrontInfoUpdate tracker tear-down must run cleanly");
}

fn seed_format_currency_display_tracker(env: &WowLuaEnv) {
    env.eval::<()>(&format!(
        r#"
        _G.__behavior_currency_for_store_formatter_calls = 0
        _G.__behavior_currency_for_store_formatter_arg_was_nil = false
        _G.__behavior_currency_for_store_original_formatter =
            AccountStoreUtil.FormatCurrencyDisplayWithWarning
        AccountStoreUtil.FormatCurrencyDisplayWithWarning = function(currency_id)
            _G.__behavior_currency_for_store_formatter_calls =
                _G.__behavior_currency_for_store_formatter_calls + 1
            _G.__behavior_currency_for_store_formatter_arg_was_nil = (currency_id == nil)
            return "{FORMATTER_SENTINEL}"
        end
        return
        "#
    ))
    .expect("seeding FormatCurrencyDisplayWithWarning tracker must run cleanly");
}

fn teardown_format_currency_display_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        AccountStoreUtil.FormatCurrencyDisplayWithWarning =
            _G.__behavior_currency_for_store_original_formatter
        _G.__behavior_currency_for_store_original_formatter = nil
        _G.__behavior_currency_for_store_formatter_calls = nil
        _G.__behavior_currency_for_store_formatter_arg_was_nil = nil
        return
        "#,
    )
    .expect("FormatCurrencyDisplayWithWarning tracker tear-down must run cleanly");
}

fn seed_stub_display(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        local currency_available = {}
        currency_available.__shown = "BEHAVIOR_CURRENCY_FOR_STORE_SHOWN_NEVER_SET"
        currency_available.__text = nil
        currency_available.SetShown = function(self, shown) self.__shown = shown end
        currency_available.SetText = function(self, text) self.__text = text end

        local footer = {}
        footer.CurrencyAvailable = currency_available

        local stub = {}
        stub.Footer = footer
        stub.InitializeStore = AccountStoreItemDisplayMixin.InitializeStore
        stub.UpdateCurrencyAvailable = AccountStoreItemDisplayMixin.UpdateCurrencyAvailable

        _G.__behavior_currency_for_store_stub_display = stub
        return
        "#,
    )
    .expect("seeding stub display must run cleanly");
}

fn teardown_stub_display(env: &WowLuaEnv) {
    env.eval::<()>("_G.__behavior_currency_for_store_stub_display = nil; return")
        .expect("stub display tear-down must run cleanly");
}
