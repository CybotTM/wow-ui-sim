//! Behavior pin for `AccountStoreUtil.IsCurrencyAtWarningThreshold`.
//!
//! Spec/source mismatch finding (PLAN.md task for
//! `AccountStoreUtil.IsCurrencyAtWarningThreshold(currencyID)`): the
//! plan describes the function as returning true when
//! `displayedAmount <= threshold` from `GetCurrencyInfo` — i.e. a
//! "low-balance" warning. The actual six-line function at
//! `Blizzard_AccountStoreUtil.lua:65-72` differs in five ways and
//! implements the OPPOSITE semantic — a "currency approaching the
//! account-wide cap" warning:
//!
//! ```lua
//! local AccountStoreWarningThresholdPercentage = 0.66;  -- file-local at line 2
//!
//! function AccountStoreUtil.IsCurrencyAtWarningThreshold(accountStoreCurrencyID)
//!     local currencyInfo = C_AccountStore.GetCurrencyInfo(accountStoreCurrencyID);
//!     if currencyInfo and currencyInfo.maxQuantity then
//!         return currencyInfo.amount >= (currencyInfo.maxQuantity * AccountStoreWarningThresholdPercentage);
//!     end
//!
//!     return false;
//! end
//! ```
//!
//! 1. **Comparison direction reversed.** PLAN says
//!    `displayedAmount <= threshold` (warn when amount is LOW); actual
//!    is `amount >= maxQuantity * percentage` (warn when amount is
//!    HIGH, approaching the cap). The companion sites in the same file
//!    confirm the cap-warning semantic: `FormatCurrencyDisplayWithWarning`
//!    at lines 87-92 uses `>= maxQuantity` and
//!    `>= maxQuantity * percentage` to pick RED_FONT_COLOR vs
//!    NORMAL_FONT_COLOR; `AddCurrencyTotalTooltip` at lines 117-122
//!    uses the SAME thresholds to format
//!    `ACCOUNT_STORE_CURRENCY_MAX_TOOLTIP_FORMAT` ("at cap") vs
//!    `ACCOUNT_STORE_CURRENCY_APPROACHING_MAX_TOOLTIP_FORMAT`
//!    ("approaching max"). The PLAN's "low-balance" framing is
//!    incompatible with all three sites.
//!
//! 2. **Field name mismatch.** The body reads `currencyInfo.amount`,
//!    NOT `currencyInfo.displayedAmount`. The PLAN-named
//!    `displayedAmount` field does not exist on the table populated by
//!    `populate_currency_info_table` at
//!    `globals/missing_surface/account_store.rs:240-249`, which sets
//!    `id`, `amount`, `maxQuantity`, `name`, `icon` only.
//!
//! 3. **Threshold is computed, not stored.** PLAN implies a separate
//!    `threshold` field on the currency record; actual implementation
//!    multiplies `maxQuantity * AccountStoreWarningThresholdPercentage`,
//!    where the percentage is the file-local `0.66` declared at line 2.
//!    This is a closed-over upvalue, NOT exposed to addons or settings.
//!    The threshold value for a currency with `maxQuantity = 100` is
//!    therefore `66`, and the warning fires when `amount >= 66`.
//!
//! 4. **PLAN-omitted nil-currencyInfo fallback.** When
//!    `C_AccountStore.GetCurrencyInfo(currencyID)` returns nil (unknown
//!    currency id under the simulator's
//!    `account_store.rs:122-136` lookup, or any future addon-side
//!    invalid id), the body short-circuits at line 67 (`if currencyInfo
//!    and currencyInfo.maxQuantity then`) and returns false.
//!
//! 5. **PLAN-omitted nil-maxQuantity fallback.** When the currency
//!    record exists but has no `maxQuantity` field (an uncapped
//!    currency — e.g. a currency that never triggers the
//!    "approaching max" warning), the body again short-circuits and
//!    returns false. The simulator's `populate_currency_info_table`
//!    only sets `maxQuantity` when `info.max_quantity.is_some()`
//!    (lines 243-245), so absent-cap currencies behave as expected.
//!
//! Four tests pin the contract:
//!
//! - `is_currency_at_warning_threshold_returns_true_when_amount_meets_or_exceeds_two_thirds_of_max_quantity`
//!   stubs `C_AccountStore.GetCurrencyInfo` to return successively
//!   `{amount=66, maxQuantity=100}` (boundary, should fire the warning
//!   at exactly 66% via `>=`) and `{amount=65, maxQuantity=100}`
//!   (one below boundary, should NOT fire). Asserts true then false.
//!   Pins both the `>=` direction and the 0.66 percentage.
//! - `is_currency_at_warning_threshold_uses_amount_field_not_plan_named_displayed_amount_field`
//!   stubs GetCurrencyInfo to return
//!   `{amount=70, displayedAmount=10, maxQuantity=100}`. The actual
//!   body reads `.amount` (70 >= 66 → true); a regression that started
//!   reading `.displayedAmount` would compute (10 >= 66 → false). Pins
//!   the field name and rules out the PLAN-named field.
//! - `is_currency_at_warning_threshold_returns_false_when_currency_info_is_nil`
//!   stubs GetCurrencyInfo to return nil; asserts false. Pins the
//!   `if currencyInfo` short-circuit at line 67.
//! - `is_currency_at_warning_threshold_returns_false_when_max_quantity_is_nil_for_uncapped_currency`
//!   stubs GetCurrencyInfo to return `{amount=999}` (no maxQuantity);
//!   asserts false. Pins the `and currencyInfo.maxQuantity` second
//!   guard — uncapped currencies never trigger a cap-warning, even at
//!   astronomical amounts.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccountStore";

#[test]
fn is_currency_at_warning_threshold_returns_true_when_amount_meets_or_exceeds_two_thirds_of_max_quantity()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ANY_CURRENCY_ID: i64 = 11;

        seed_get_currency_info_amount_max_stub(env, 66, 100);

        let at_boundary: bool = env
            .eval(&format!(
                "return AccountStoreUtil.IsCurrencyAtWarningThreshold({ANY_CURRENCY_ID})"
            ))
            .expect("IsCurrencyAtWarningThreshold call must run cleanly at boundary");

        assert!(
            at_boundary,
            "Expected IsCurrencyAtWarningThreshold to return TRUE for amount=66 with \
             maxQuantity=100 — the threshold is `maxQuantity * 0.66` (file-local \
             AccountStoreWarningThresholdPercentage at `Blizzard_AccountStoreUtil.lua:2`), \
             so 100 * 0.66 = 66, and the body checks `amount >= threshold` (line 68). The \
             boundary (amount == threshold) fires because of the `>=` operator. A false reading \
             would prove either (a) the comparison flipped to `>` (strict instead of \
             non-strict), or (b) the percentage changed from 0.66, or (c) the direction \
             reversed to `<=` (matching the PLAN's low-balance shape, which would mean the \
             function now warns when amount is LOW)."
        );

        seed_get_currency_info_amount_max_stub(env, 65, 100);

        let below_boundary: bool = env
            .eval(&format!(
                "return AccountStoreUtil.IsCurrencyAtWarningThreshold({ANY_CURRENCY_ID})"
            ))
            .expect("IsCurrencyAtWarningThreshold call must run cleanly below boundary");

        assert!(
            !below_boundary,
            "Expected IsCurrencyAtWarningThreshold to return FALSE for amount=65 with \
             maxQuantity=100 — `65 >= 66` is false, so the cap-warning does not fire one unit \
             below the threshold. A true reading would prove either (a) the percentage dropped \
             below 0.65 (more aggressive warning), or (b) the comparison flipped to `<=` \
             (the PLAN-shaped low-balance reading where amount=65 with threshold=66 would fire \
             a low-balance warning)."
        );

        teardown_get_currency_info_stub(env);
    });
}

#[test]
fn is_currency_at_warning_threshold_uses_amount_field_not_plan_named_displayed_amount_field() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ANY_CURRENCY_ID: i64 = 12;

        env.eval::<()>(
            r#"
            _G.__behavior_currency_warning_original_get_currency_info = C_AccountStore.GetCurrencyInfo
            C_AccountStore.GetCurrencyInfo = function(_currency_id)
                return { amount = 70, displayedAmount = 10, maxQuantity = 100 }
            end
            return
            "#,
        )
        .expect(
            "seeding GetCurrencyInfo stub returning both amount and displayedAmount must run cleanly",
        );

        let result: bool = env
            .eval(&format!(
                "return AccountStoreUtil.IsCurrencyAtWarningThreshold({ANY_CURRENCY_ID})"
            ))
            .expect("IsCurrencyAtWarningThreshold call must run cleanly");

        assert!(
            result,
            "Expected TRUE for {{amount=70, displayedAmount=10, maxQuantity=100}} — the body at \
             line 68 reads `currencyInfo.amount` (70 >= 66 = true), NOT \
             `currencyInfo.displayedAmount` (10 >= 66 = false). A false reading would prove the \
             body now reads the PLAN-named `displayedAmount` field (forcing a re-pin against \
             the new field contract and likely breaking real callers because the \
             populate_currency_info_table at \
             `globals/missing_surface/account_store.rs:240-249` does NOT populate a \
             `displayedAmount` field — it only sets id/amount/maxQuantity/name/icon)."
        );

        teardown_get_currency_info_stub(env);
    });
}

#[test]
fn is_currency_at_warning_threshold_returns_false_when_currency_info_is_nil() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ANY_CURRENCY_ID: i64 = 13;

        env.eval::<()>(
            r#"
            _G.__behavior_currency_warning_original_get_currency_info = C_AccountStore.GetCurrencyInfo
            C_AccountStore.GetCurrencyInfo = function(_currency_id) return nil end
            return
            "#,
        )
        .expect("seeding nil-returning GetCurrencyInfo stub must run cleanly");

        let result: bool = env
            .eval(&format!(
                "return AccountStoreUtil.IsCurrencyAtWarningThreshold({ANY_CURRENCY_ID})"
            ))
            .expect("IsCurrencyAtWarningThreshold call must run cleanly with nil currencyInfo");

        assert!(
            !result,
            "Expected FALSE when `C_AccountStore.GetCurrencyInfo` returns nil — the body at \
             line 67 short-circuits with `if currencyInfo and currencyInfo.maxQuantity then` and \
             falls through to `return false` at line 71. A true reading would prove the \
             short-circuit was removed (forcing the body to attempt `nil.maxQuantity` which \
             would error, OR to use a different fallback like `true`)."
        );

        teardown_get_currency_info_stub(env);
    });
}

#[test]
fn is_currency_at_warning_threshold_returns_false_when_max_quantity_is_nil_for_uncapped_currency() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ANY_CURRENCY_ID: i64 = 14;

        env.eval::<()>(
            r#"
            _G.__behavior_currency_warning_original_get_currency_info = C_AccountStore.GetCurrencyInfo
            C_AccountStore.GetCurrencyInfo = function(_currency_id)
                return { amount = 999999 }
            end
            return
            "#,
        )
        .expect("seeding GetCurrencyInfo stub returning currency with no maxQuantity must run cleanly");

        let result: bool = env
            .eval(&format!(
                "return AccountStoreUtil.IsCurrencyAtWarningThreshold({ANY_CURRENCY_ID})"
            ))
            .expect("IsCurrencyAtWarningThreshold call must run cleanly with nil maxQuantity");

        assert!(
            !result,
            "Expected FALSE for an uncapped currency (maxQuantity=nil) regardless of how high \
             the amount goes — the body's `and currencyInfo.maxQuantity` second guard at line 67 \
             rules out cap-warnings for currencies with no defined cap. A true reading would \
             prove the body now treats nil-maxQuantity as an implicit cap (e.g. by defaulting \
             to some constant), which would surface false positives for uncapped currencies. \
             populate_currency_info_table at `globals/missing_surface/account_store.rs:243-245` \
             only sets `maxQuantity` when `info.max_quantity.is_some()`, so absent-cap currencies \
             intentionally produce a record without the field."
        );

        teardown_get_currency_info_stub(env);
    });
}

fn seed_get_currency_info_amount_max_stub(env: &WowLuaEnv, amount: i64, max_quantity: i64) {
    env.eval::<()>(&format!(
        r#"
        if _G.__behavior_currency_warning_original_get_currency_info == nil then
            _G.__behavior_currency_warning_original_get_currency_info = C_AccountStore.GetCurrencyInfo
        end
        C_AccountStore.GetCurrencyInfo = function(_currency_id)
            return {{ amount = {amount}, maxQuantity = {max_quantity} }}
        end
        return
        "#
    ))
    .expect("seeding GetCurrencyInfo stub with explicit amount/maxQuantity must run cleanly");
}

fn teardown_get_currency_info_stub(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        if _G.__behavior_currency_warning_original_get_currency_info ~= nil then
            C_AccountStore.GetCurrencyInfo = _G.__behavior_currency_warning_original_get_currency_info
            _G.__behavior_currency_warning_original_get_currency_info = nil
        end
        return
        "#,
    )
    .expect("GetCurrencyInfo stub tear-down must run cleanly");
}
