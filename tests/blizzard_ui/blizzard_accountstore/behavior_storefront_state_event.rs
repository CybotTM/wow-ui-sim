//! Behavior pin for the `STORE_FRONT_STATE_UPDATED` event handling
//! contract in the `Blizzard_AccountStore` lane.
//!
//! Spec/source mismatch finding (PLAN.md task: firing
//! `STORE_FRONT_STATE_UPDATED` drives `AccountStoreMixin:OnEvent` to
//! refresh visibility based on `C_AccountStore.GetStoreFrontState`):
//! the plan claims an `AccountStoreMixin:OnEvent` method exists and
//! that it routes `STORE_FRONT_STATE_UPDATED` into a visibility
//! refresh on `AccountStoreFrame`. Five separate findings refute the
//! claim:
//!
//! 1. **`AccountStoreMixin:OnEvent` does not exist.**
//!    `Blizzard_AccountStore.lua:16-64` defines exactly five methods
//!    on `AccountStoreMixin`: `OnLoad` (lines 18-24), `OnShow`
//!    (lines 26-29), `OnHide` (lines 31-40), `SetStoreFrontID`
//!    (lines 42-48), and `SetFullscreenMode` (lines 50-64). No
//!    `OnEvent` method. Verified via
//!    `grep -n "function AccountStoreMixin:" Blizzard_AccountStore.lua`
//!    — the entire mixin has zero `OnEvent` declaration.
//!
//! 2. **`AccountStoreFrame` does not register
//!    `STORE_FRONT_STATE_UPDATED` (or any other event) at OnLoad or
//!    later.** `AccountStoreMixin:OnLoad` only calls
//!    `SetPortraitToAsset` and writes `UIPanelWindows[...]`. No
//!    `RegisterEvent` call exists in the entire mixin. Already pinned
//!    by the companion `surface_events.rs` test
//!    (`account_store_frame_does_not_register_plan_named_events_after_onload`).
//!    This file pins the structural-absence in the `behavior` lane
//!    too, so a regression that ADDS a stub OnEvent is caught even
//!    if it skips RegisterEvent.
//!
//! 3. **`STORE_FRONT_STATE_UPDATED` is registered by FOUR sibling
//!    addons, none of them `Blizzard_AccountStore`:**
//!    - `Blizzard_CharacterSelectNavBar/CharacterSelectNavBar.lua:207`
//!      registers it; the OnEvent at lines 130-132 sets
//!      `self.PlunderstoreButton:SetEnabled(...)`.
//!    - `Blizzard_PVPUI/Mainline/Blizzard_PVPUI.lua:2439` lists it in
//!      a frame-events array; the OnEvent at line 2489 dispatches to
//!      a sibling-update path.
//!    - `Blizzard_PlunderstormBasics/Blizzard_PlunderstormBasics.lua:42`
//!      registers it; the OnEvent at lines 95-97 toggles a button.
//!    - `Blizzard_PlunderstormPrematchUI/Blizzard_PlunderstormPrematchUI.lua:115`
//!      reads `C_AccountStore.GetStoreFrontState` directly (no event
//!      registration — polls on demand).
//!
//! 4. **The behavior is NOT visibility-refresh on `AccountStoreFrame`.**
//!    All real handlers control BUTTON ENABLED STATE via
//!    `:SetEnabled(GetStoreFrontState(id) == Enum.AccountStoreState.Available)`,
//!    NOT `AccountStoreFrame:Show()` / `Hide()`. The button-enabled
//!    semantic gates the *entry-point* button (the one that opens the
//!    store) — the store frame itself is never shown/hidden in
//!    response to the event. The PLAN's "refresh visibility on
//!    AccountStoreFrame" framing is therefore incompatible with all
//!    four handler sites.
//!
//! 5. **The comparison constant is `Enum.AccountStoreState.Available`
//!    (= 0).** All four real handlers compare the C_API return value
//!    against `Enum.AccountStoreState.Available`. The simulator seeds
//!    the enum at `enum_data/missing_enums.lua:317-323` with
//!    `Available = 0`, `Unavailable = 2`, `Unknown = 1`. The C_API
//!    `C_AccountStore.GetStoreFrontState` at
//!    `globals/missing_surface/account_store.rs:154-165` returns the
//!    seeded `account_store_storefront_state` SimState entry, falling
//!    back to `ACCOUNT_STORE_STATE_AVAILABLE` (= 0) for unseeded
//!    storefront ids. So calling the C_API on any storefront id with
//!    no SimState seed yields `Available`, matching the
//!    button-enabled check directly.
//!
//! Five tests pin the contract:
//!
//! - `account_store_mixin_on_event_method_does_not_exist`
//!   asserts `AccountStoreMixin.OnEvent` is nil and the five actual
//!   methods (`OnLoad`, `OnShow`, `OnHide`, `SetStoreFrontID`,
//!   `SetFullscreenMode`) are functions. Structural-absence tripwire
//!   for the PLAN-named entry point.
//!
//! - `account_store_frame_does_not_register_storefront_state_event`
//!   asserts `AccountStoreFrame:IsEventRegistered("STORE_FRONT_STATE_UPDATED")`
//!   is false. Lane-absence tripwire — flips if a future change
//!   wires the event to `AccountStoreFrame` (which would force a
//!   re-pin against the new contract).
//!
//! - `account_store_state_available_constant_is_zero`
//!   asserts `Enum.AccountStoreState.Available == 0` and the other
//!   two values match the seeded shape (Unknown=1, Unavailable=2).
//!   Constant-pin for the comparison literal used by real handlers.
//!
//! - `c_account_store_get_storefront_state_is_callable_and_returns_numeric`
//!   asserts `C_AccountStore.GetStoreFrontState` is a function and
//!   that calling it with an arbitrary storefront id returns a
//!   number. Pins the C_API surface (the depends-on for this PLAN
//!   item) — the surface DOES exist; the PLAN's claim about
//!   `AccountStoreMixin:OnEvent` is what doesn't.
//!
//! - `firing_storefront_state_updated_does_not_invoke_a_visibility_refresh_on_account_store_frame`
//!   replaces `AccountStoreFrame:Show` and `AccountStoreFrame:Hide`
//!   with logging trackers, fires `STORE_FRONT_STATE_UPDATED` via
//!   the global `EventRegistry:TriggerEvent`-equivalent path
//!   (synchronously dispatching the frame event through the
//!   simulator's event queue), and asserts the trackers received
//!   ZERO calls. This is the negative-behavior tripwire — even if a
//!   future regression added a stub `OnEvent` that called
//!   `Show`/`Hide`, this test would catch it.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccountStore";
const FRAME_NAME: &str = "AccountStoreFrame";

const ACTUAL_ACCOUNT_STORE_MIXIN_METHODS: &[(&str, &str)] = &[
    ("OnLoad", "Blizzard_AccountStore.lua:18-24"),
    ("OnShow", "Blizzard_AccountStore.lua:26-29"),
    ("OnHide", "Blizzard_AccountStore.lua:31-40"),
    ("SetStoreFrontID", "Blizzard_AccountStore.lua:42-48"),
    ("SetFullscreenMode", "Blizzard_AccountStore.lua:50-64"),
];

#[test]
fn account_store_mixin_on_event_method_does_not_exist() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let on_event_type: String = env
            .eval("return type(AccountStoreMixin.OnEvent)")
            .expect("type(AccountStoreMixin.OnEvent) probe must run cleanly");

        assert_eq!(
            on_event_type, "nil",
            "Expected `type(AccountStoreMixin.OnEvent) == \"nil\"` (PLAN.md spec/source mismatch \
             tripwire — the plan names `AccountStoreMixin:OnEvent` as the entry point that handles \
             `STORE_FRONT_STATE_UPDATED` and refreshes `AccountStoreFrame` visibility, but the \
             entire `AccountStoreMixin` table at `Blizzard_AccountStore.lua:16-64` declares zero \
             OnEvent method — only OnLoad/OnShow/OnHide/SetStoreFrontID/SetFullscreenMode), got \
             `{on_event_type}`. A non-nil reading would prove either (a) Blizzard added an \
             `:OnEvent` method (forcing a re-pin against the new mixin contract), or (b) some \
             other addon monkey-patched `AccountStoreMixin.OnEvent` (a behavior leak across \
             lanes that this tripwire is designed to catch)."
        );

        for (method_name, source_site) in ACTUAL_ACCOUNT_STORE_MIXIN_METHODS {
            let method_type: String = env
                .eval(&format!("return type(AccountStoreMixin[{method_name:?}])"))
                .unwrap_or_else(|error| {
                    panic!("type(AccountStoreMixin.{method_name}) probe must run cleanly: {error}")
                });

            assert_eq!(
                method_type, "function",
                "Expected `type(AccountStoreMixin.{method_name}) == \"function\"` ({source_site}), \
                 got `{method_type}`. A non-function reading would mean the actual mixin shape \
                 changed (one of the five real methods was renamed or removed), forcing a re-pin \
                 against the new contract."
            );
        }
    });
}

#[test]
fn account_store_frame_does_not_register_storefront_state_event() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let registered: bool = env
            .eval(&format!(
                "return _G[{FRAME_NAME:?}]:IsEventRegistered(\"STORE_FRONT_STATE_UPDATED\")"
            ))
            .expect("AccountStoreFrame:IsEventRegistered probe must run cleanly");

        assert!(
            !registered,
            "Expected `AccountStoreFrame:IsEventRegistered(\"STORE_FRONT_STATE_UPDATED\")` to \
             return false (PLAN.md spec/source mismatch tripwire — the event is NOT registered \
             anywhere in the `Blizzard_AccountStore` lane; verified via \
             `grep -rn STORE_FRONT_STATE_UPDATED Interface/BlizzardUI/Blizzard_AccountStore/` \
             which yields zero matches in the addon's own files. The event lives in four sibling \
             addons: `Blizzard_CharacterSelectNavBar/CharacterSelectNavBar.lua:207`, \
             `Blizzard_PVPUI/Mainline/Blizzard_PVPUI.lua:2439`, \
             `Blizzard_PlunderstormBasics/Blizzard_PlunderstormBasics.lua:42`, and \
             `Blizzard_PlunderstormPrematchUI/Blizzard_PlunderstormPrematchUI.lua:115` — none of \
             them this lane), got true. A true reading would prove either (a) Blizzard moved the \
             event registration onto `AccountStoreFrame` (forcing a re-pin against the new OnLoad \
             contract), or (b) the simulator's event-routing leaked a sibling addon's \
             RegisterEvent onto `AccountStoreFrame` (a regression in `register_event` / \
             `registered_events` storage scoping)."
        );
    });
}

const ACTUAL_ACCOUNT_STORE_STATE_VALUES: &[(&str, i64)] =
    &[("Available", 0), ("Unknown", 1), ("Unavailable", 2)];

#[test]
fn account_store_state_available_constant_is_zero() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for (variant_name, expected_value) in ACTUAL_ACCOUNT_STORE_STATE_VALUES {
            let actual_value: i64 = env
                .eval(&format!("return Enum.AccountStoreState[{variant_name:?}]"))
                .unwrap_or_else(|error| {
                    panic!("Enum.AccountStoreState.{variant_name} probe must run cleanly: {error}")
                });

            assert_eq!(
                actual_value, *expected_value,
                "Expected `Enum.AccountStoreState.{variant_name} == {expected_value}` (seeded by \
                 `enum_data/missing_enums.lua:317-323`), got {actual_value}. The `Available = 0` \
                 value is the comparison constant used by all four real \
                 `STORE_FRONT_STATE_UPDATED` handlers in sibling addons (e.g. \
                 `CharacterSelectNavBar.lua:132`: \
                 `:SetEnabled(C_AccountStore.GetStoreFrontState(id) == Enum.AccountStoreState.Available)`). \
                 A drift in this value would silently break the button-enabled gate across all \
                 four sibling lanes — the C_API would return a non-zero state on an Available \
                 storefront and the comparison would yield false, disabling the entry-point \
                 button on a storefront that is actually open."
            );
        }
    });
}

#[test]
fn c_account_store_get_storefront_state_is_callable_and_returns_numeric() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ARBITRARY_STOREFRONT_ID: i64 = 9_876_543;

        let function_type: String = env
            .eval("return type(C_AccountStore.GetStoreFrontState)")
            .expect("type(C_AccountStore.GetStoreFrontState) probe must run cleanly");

        assert_eq!(
            function_type, "function",
            "Expected `type(C_AccountStore.GetStoreFrontState) == \"function\"` (C_API surface \
             registered at `globals/missing_surface/account_store.rs:34`: \
             `table_set_rust_fn_static(state, table_ref, \"GetStoreFrontState\", get_storefront_state)`), \
             got `{function_type}`. A non-function reading would prove the C_API surface was \
             dropped — this is the depends-on gap named in the PLAN.md task and the surface \
             actually DOES exist; the PLAN's claim about `AccountStoreMixin:OnEvent` is what \
             doesn't."
        );

        let return_type: String = env
            .eval(&format!(
                "return type(C_AccountStore.GetStoreFrontState({ARBITRARY_STOREFRONT_ID}))"
            ))
            .expect("C_AccountStore.GetStoreFrontState call must run cleanly");

        assert_eq!(
            return_type, "number",
            "Expected `type(C_AccountStore.GetStoreFrontState({ARBITRARY_STOREFRONT_ID})) == \
             \"number\"` (the implementation at \
             `globals/missing_surface/account_store.rs:154-165` always pushes a `Val::Num`), got \
             `{return_type}`. The default value for an unseeded storefront id is \
             `ACCOUNT_STORE_STATE_AVAILABLE` (= 0), matching `Enum.AccountStoreState.Available` \
             — so the button-enabled check on any unseeded storefront yields true (the entry-point \
             button is enabled by default). A non-number reading would prove the C_API now \
             returns a different shape (table, string), breaking real handler comparisons."
        );
    });
}

#[test]
fn firing_storefront_state_updated_does_not_invoke_a_visibility_refresh_on_account_store_frame() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const ANY_STOREFRONT_ID: i64 = 7;

        seed_show_hide_trackers_on_account_store_frame(env);

        env.eval::<()>(&format!(
            r#"
            if AccountStoreFrame.RegisterEvent then
                AccountStoreFrame:RegisterEvent("STORE_FRONT_STATE_UPDATED")
            end
            if FireEvent then
                FireEvent("STORE_FRONT_STATE_UPDATED", {ANY_STOREFRONT_ID})
            elseif EventRegistry and EventRegistry.TriggerEvent then
                EventRegistry:TriggerEvent("STORE_FRONT_STATE_UPDATED", {ANY_STOREFRONT_ID})
            end
            return
            "#
        ))
        .expect("STORE_FRONT_STATE_UPDATED firing path must run cleanly");

        let (show_calls, hide_calls): (i64, i64) = env
            .eval(
                "return _G.__behavior_storefront_state_show_calls, \
                 _G.__behavior_storefront_state_hide_calls",
            )
            .expect("Show/Hide tracker readouts must run cleanly");

        assert_eq!(
            show_calls, 0,
            "Expected `AccountStoreFrame:Show` to receive ZERO calls after firing \
             STORE_FRONT_STATE_UPDATED (PLAN.md spec/source mismatch tripwire — the plan claims \
             the event drives a visibility refresh on `AccountStoreFrame`, but no handler in the \
             `Blizzard_AccountStore` lane registers for the event AND no `AccountStoreMixin:OnEvent` \
             method exists). Got {show_calls} call(s). A non-zero reading would prove either (a) \
             a future regression added a stub `AccountStoreMixin:OnEvent` that calls \
             `:Show()` (the PLAN-shaped behavior reappearing as a regression), or (b) some \
             cross-addon handler leaked into the lane and ended up gated against `AccountStoreFrame` \
             (a separation-of-concerns regression where a sibling addon's button-enabled \
             handler started toggling the wrong frame)."
        );

        assert_eq!(
            hide_calls, 0,
            "Expected `AccountStoreFrame:Hide` to receive ZERO calls after firing \
             STORE_FRONT_STATE_UPDATED. Got {hide_calls} call(s). Same tripwire shape as the \
             Show assertion — a non-zero reading would mean a regression introduced visibility \
             toggling on `AccountStoreFrame` in response to the event, which is incompatible \
             with all four real handler sites (all of which control button-enabled state on a \
             DIFFERENT frame, not the AccountStoreFrame itself)."
        );

        teardown_show_hide_trackers_on_account_store_frame(env);
    });
}

fn seed_show_hide_trackers_on_account_store_frame(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_storefront_state_show_calls = 0
        _G.__behavior_storefront_state_hide_calls = 0
        _G.__behavior_storefront_state_original_show = AccountStoreFrame.Show
        _G.__behavior_storefront_state_original_hide = AccountStoreFrame.Hide
        AccountStoreFrame.Show = function(_self)
            _G.__behavior_storefront_state_show_calls = _G.__behavior_storefront_state_show_calls + 1
        end
        AccountStoreFrame.Hide = function(_self)
            _G.__behavior_storefront_state_hide_calls = _G.__behavior_storefront_state_hide_calls + 1
        end
        return
        "#,
    )
    .expect("seeding Show/Hide trackers on AccountStoreFrame must run cleanly");
}

fn teardown_show_hide_trackers_on_account_store_frame(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        if _G.__behavior_storefront_state_original_show ~= nil then
            AccountStoreFrame.Show = _G.__behavior_storefront_state_original_show
            _G.__behavior_storefront_state_original_show = nil
        end
        if _G.__behavior_storefront_state_original_hide ~= nil then
            AccountStoreFrame.Hide = _G.__behavior_storefront_state_original_hide
            _G.__behavior_storefront_state_original_hide = nil
        end
        _G.__behavior_storefront_state_show_calls = nil
        _G.__behavior_storefront_state_hide_calls = nil
        return
        "#,
    )
    .expect("Show/Hide tracker tear-down must run cleanly");
}
