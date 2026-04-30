//! Behavior pin for the category-list rebuild path triggered by
//! `AccountStore.StoreFrontSet`.
//!
//! Spec/source mismatch finding (PLAN.md task: `AccountStoreCategoryListMixin:Refresh`
//! rebuilds the category buttons from `C_AccountStore.GetCategories()` and
//! `GetCategoryInfo`, preserving selection across refresh). FOUR claims
//! diverge from the actual source at
//! `Blizzard_AccountStoreCategoryList.lua:1-78`.
//!
//! 1. **There is no `Refresh` method on `AccountStoreCategoryListMixin`.**
//!    The mixin defines OnLoad, InitScrollBox, OnStoreFrontSet,
//!    OnCategorySelected, SetCategories, SetRowSelectedState (lines 20, 29,
//!    53, 57, 67, 75). The actual rebuild entry point is
//!    `OnStoreFrontSet` (line 53), invoked via the EventRegistry
//!    "AccountStore.StoreFrontSet" subscription registered at line 25.
//!
//! 2. **`GetCategories` is called WITH the storeFrontID parameter, not
//!    zero-arg.** Line 54 reads
//!    `self:SetCategories(C_AccountStore.GetCategories(storeFrontID))`.
//!    The PLAN-named `GetCategories()` zero-arg form does not exist on this
//!    path.
//!
//! 3. **`GetCategoryInfo` is NOT called by the list mixin at all.** It is
//!    called by the per-button mixin
//!    `AccountStoreCategoryMixin:SetCategory(categoryID)` at line 12, which
//!    runs from the scroll box element initializer set up in
//!    `InitScrollBox` (line 31-33). The list mixin's `SetCategories` only
//!    calls `CreateDataProviderWithAssignedKey` + `ScrollBox:SetDataProvider`
//!    + `EventRegistry:TriggerEvent`. The per-category lookup is deferred
//!    to button realization, not done up-front by the list mixin.
//!
//! 4. **Selection is RESET to the first category, NOT preserved across
//!    refresh.** Line 72 reads
//!    `EventRegistry:TriggerEvent("AccountStore.CategorySelected", categories[1])`
//!    unconditionally. The line 71 comment even says
//!    "Start with the first category selected." `SetDataProvider` on line
//!    69 passes `ScrollBoxConstants.RetainScrollPosition` — that retains
//!    the SCROLL position, not the selection. The PLAN's
//!    "preserving selection across refresh" claim is the OPPOSITE of the
//!    actual behavior.
//!
//! Six tests pin the contract:
//!
//! - `account_store_category_list_mixin_does_not_define_refresh` —
//!   surface tripwire that `type(AccountStoreCategoryListMixin.Refresh) ==
//!   "nil"`. A non-nil reading would prove the PLAN-named method was added
//!   (forcing a re-pin against its actual body).
//!
//! - `account_store_category_list_mixin_on_store_front_set_and_set_categories_are_functions`
//!   — surface positive that the actual entry points still exist.
//!
//! - `account_store_category_mixin_set_category_is_the_per_button_get_category_info_caller`
//!   — pins that `AccountStoreCategoryMixin.SetCategory` (the PER-BUTTON
//!   mixin, distinct from the list mixin) is a function and is the actual
//!   `GetCategoryInfo` call site (line 12). A nil reading would prove the
//!   per-button mixin or its method was renamed.
//!
//! - `on_store_front_set_calls_get_categories_with_store_front_id_param` —
//!   stubs `C_AccountStore.GetCategories` with a tracker, invokes
//!   `OnStoreFrontSet(stub, 42)`, asserts the tracker fired once with `42`
//!   AND the stubbed `SetCategories` received the tracker's return. Pins
//!   axes 1 and 2: a zero-call reading would prove the dispatch was
//!   rerouted; an arg mismatch would prove the parameter was dropped (the
//!   PLAN-named zero-arg form).
//!
//! - `set_categories_does_not_directly_call_get_category_info` — stubs
//!   `C_AccountStore.GetCategoryInfo` with a tracker, neuters
//!   `EventRegistry.TriggerEvent` so subscribed listeners (e.g.
//!   `AccountStoreItemDisplayMixin:OnCategorySelected` which DOES call
//!   GetCategoryInfo at item_display.lua:120) cannot fire indirectly,
//!   invokes `SetCategories(stub, {10, 20, 30})`, asserts the
//!   `GetCategoryInfo` tracker received ZERO calls. Pins axis 3: a
//!   non-zero reading would prove the PLAN's claim came true (the list
//!   mixin started doing per-category lookups directly instead of
//!   deferring to the scroll box element initializer).
//!
//! - `set_categories_resets_selection_to_first_category_not_preserves_selection`
//!   — stubs `EventRegistry.TriggerEvent` with a capture, invokes
//!   `SetCategories(stub, {10, 20, 30})`, asserts the tracker captured
//!   exactly one `("AccountStore.CategorySelected", 10)` call. Then
//!   invokes `SetCategories(stub, {99, 88})` and asserts a second
//!   capture of `("AccountStore.CategorySelected", 99)` — proving the
//!   trigger fires with the NEW first element regardless of any prior
//!   selection. Pins axis 4: the PLAN's "preserving selection across
//!   refresh" would predict the second call to fire with `10` (the
//!   previous selection); the actual behavior fires with `99` (the new
//!   first).

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccountStore";
const STORE_FRONT_ID: i64 = 42;
const FIRST_CATEGORY_ID: i64 = 10;
const SECOND_CATEGORY_ID: i64 = 20;
const THIRD_CATEGORY_ID: i64 = 30;
const REPLACED_FIRST_CATEGORY_ID: i64 = 99;
const REPLACED_SECOND_CATEGORY_ID: i64 = 88;

#[test]
fn account_store_category_list_mixin_does_not_define_refresh() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let refresh_type: String = env
            .eval("return type(AccountStoreCategoryListMixin.Refresh)")
            .expect("AccountStoreCategoryListMixin.Refresh probe must run cleanly");

        assert_eq!(
            refresh_type, "nil",
            "Expected `type(AccountStoreCategoryListMixin.Refresh) == \"nil\"` per \
             `Blizzard_AccountStoreCategoryList.lua:18-78` — the mixin defines OnLoad, \
             InitScrollBox, OnStoreFrontSet, OnCategorySelected, SetCategories, and \
             SetRowSelectedState; there is no Refresh method. The actual rebuild entry point \
             is `OnStoreFrontSet` (line 53), invoked via the EventRegistry \
             \"AccountStore.StoreFrontSet\" subscription registered at line 25. Got \
             `{refresh_type}`. A non-nil reading would prove the PLAN-named method was added \
             upstream — forcing a re-pin against its actual body."
        );
    });
}

#[test]
fn account_store_category_list_mixin_on_store_front_set_and_set_categories_are_functions() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (on_store_front_set_type, set_categories_type): (String, String) = env
            .eval(
                r#"
                return type(AccountStoreCategoryListMixin.OnStoreFrontSet),
                       type(AccountStoreCategoryListMixin.SetCategories)
                "#,
            )
            .expect("entry-point probe must run cleanly");

        assert_eq!(
            on_store_front_set_type, "function",
            "Expected `type(AccountStoreCategoryListMixin.OnStoreFrontSet) == \"function\"` \
             per `Blizzard_AccountStoreCategoryList.lua:53-55`. Got \
             `{on_store_front_set_type}`. A non-function reading would prove the actual \
             rebuild entry point was renamed or moved — likely to `Refresh`, validating the \
             PLAN's claim by relocation."
        );

        assert_eq!(
            set_categories_type, "function",
            "Expected `type(AccountStoreCategoryListMixin.SetCategories) == \"function\"` per \
             `Blizzard_AccountStoreCategoryList.lua:67-73` — the data-provider dispatch site \
             that fires the `AccountStore.CategorySelected` event with `categories[1]`. Got \
             `{set_categories_type}`. A non-function reading would prove the dispatch path \
             was inlined into OnStoreFrontSet or moved onto a per-button mixin."
        );
    });
}

#[test]
fn account_store_category_mixin_set_category_is_the_per_button_get_category_info_caller() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let set_category_type: String = env
            .eval("return type(AccountStoreCategoryMixin.SetCategory)")
            .expect("AccountStoreCategoryMixin.SetCategory probe must run cleanly");

        assert_eq!(
            set_category_type, "function",
            "Expected `type(AccountStoreCategoryMixin.SetCategory) == \"function\"` per \
             `Blizzard_AccountStoreCategoryList.lua:9-15` — this PER-BUTTON mixin (distinct \
             from the list mixin) is the actual `C_AccountStore.GetCategoryInfo` call site \
             (line 12), invoked from the scroll box element initializer set up in \
             `InitScrollBox` (line 31-33). The list mixin's SetCategories does NOT call \
             GetCategoryInfo — the lookup is deferred to button realization. Got \
             `{set_category_type}`. A nil reading would prove the per-button mixin moved or \
             the method was renamed, breaking the deferred-lookup contract."
        );
    });
}

#[test]
fn on_store_front_set_calls_get_categories_with_store_front_id_param() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_get_categories_tracker(env);
        seed_set_categories_capture(env);
        seed_stub_category_list(env);

        env.eval::<()>(&format!(
            r#"
            AccountStoreCategoryListMixin.OnStoreFrontSet(
                _G.__behavior_category_list_refresh_stub_list, {STORE_FRONT_ID}
            )
            return
            "#
        ))
        .expect("OnStoreFrontSet invocation must run cleanly");

        let (calls, arg, set_categories_received_first): (i64, i64, i64) = env
            .eval(
                r#"
                local capture = _G.__behavior_category_list_refresh_set_categories_capture or {}
                local first = nil
                if capture[1] and capture[1].categories then
                    first = capture[1].categories[1]
                end
                return _G.__behavior_category_list_refresh_get_categories_calls or 0,
                       _G.__behavior_category_list_refresh_get_categories_arg or -1,
                       first or -1
                "#,
            )
            .expect("dispatch readout must run cleanly");

        assert_eq!(
            calls, 1,
            "Expected exactly ONE `C_AccountStore.GetCategories` call after \
             OnStoreFrontSet({STORE_FRONT_ID}) — line 54 issues a single dispatch. Got \
             {calls}. A zero reading would prove the dispatch was rerouted; a value > 1 \
             would prove a redundant call was added."
        );

        assert_eq!(
            arg, STORE_FRONT_ID,
            "Expected GetCategories to receive {STORE_FRONT_ID} (the storeFrontID parameter) \
             — line 54 reads `C_AccountStore.GetCategories(storeFrontID)`. Got {arg}. A \
             reading of -1 (the seed default) would prove the call was made with no \
             argument (the PLAN-named `GetCategories()` zero-arg form); any other mismatch \
             would prove the parameter was substituted."
        );

        assert_eq!(
            set_categories_received_first, FIRST_CATEGORY_ID,
            "Expected the SetCategories capture to record categories[1] == \
             {FIRST_CATEGORY_ID} (the first id in the GetCategories tracker's return value \
             {{{FIRST_CATEGORY_ID}, {SECOND_CATEGORY_ID}, {THIRD_CATEGORY_ID}}}). Got \
             {set_categories_received_first}. A mismatch would prove the GetCategories \
             return value was filtered, transformed, or replaced before reaching \
             SetCategories."
        );

        teardown_stub_category_list(env);
        teardown_set_categories_capture(env);
        teardown_get_categories_tracker(env);
    });
}

#[test]
fn set_categories_does_not_directly_call_get_category_info() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_get_category_info_tracker(env);
        seed_event_registry_trigger_capture(env);
        seed_stub_category_list(env);

        env.eval::<()>(&format!(
            r#"
            AccountStoreCategoryListMixin.SetCategories(
                _G.__behavior_category_list_refresh_stub_list,
                {{ {FIRST_CATEGORY_ID}, {SECOND_CATEGORY_ID}, {THIRD_CATEGORY_ID} }}
            )
            return
            "#
        ))
        .expect("SetCategories invocation must run cleanly");

        let get_category_info_calls: i64 = env
            .eval("return _G.__behavior_category_list_refresh_get_category_info_calls or 0")
            .expect("GetCategoryInfo call-count readout must run cleanly");

        assert_eq!(
            get_category_info_calls, 0,
            "Expected ZERO direct `C_AccountStore.GetCategoryInfo` calls during SetCategories \
             — the list mixin's SetCategories (lines 67-73) only calls \
             CreateDataProviderWithAssignedKey + ScrollBox:SetDataProvider + \
             EventRegistry:TriggerEvent; it does NOT look up per-category info. The \
             GetCategoryInfo call site is `AccountStoreCategoryMixin:SetCategory` (line 12), \
             which runs from the scroll box element initializer when buttons are realized — \
             not synchronously inside SetCategories. The test neuters \
             EventRegistry.TriggerEvent so subscribed listeners (e.g. \
             AccountStoreItemDisplayMixin:OnCategorySelected at item_display.lua:120) cannot \
             fire indirect lookups. Got {get_category_info_calls}. A non-zero reading would \
             prove the list mixin started doing per-category lookups directly (matching \
             PLAN's claim), breaking the deferred-realization design."
        );

        teardown_stub_category_list(env);
        teardown_event_registry_trigger_capture(env);
        teardown_get_category_info_tracker(env);
    });
}

#[test]
fn set_categories_resets_selection_to_first_category_not_preserves_selection() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_event_registry_trigger_capture(env);
        seed_stub_category_list(env);

        env.eval::<()>(&format!(
            r#"
            AccountStoreCategoryListMixin.SetCategories(
                _G.__behavior_category_list_refresh_stub_list,
                {{ {FIRST_CATEGORY_ID}, {SECOND_CATEGORY_ID}, {THIRD_CATEGORY_ID} }}
            )
            return
            "#
        ))
        .expect("first SetCategories invocation must run cleanly");

        let (first_event_name, first_event_arg, first_call_count): (String, i64, i64) = env
            .eval(
                r#"
            local captured = _G.__behavior_category_list_refresh_event_registry_calls or {}
            local last = captured[#captured] or {}
            return last.event_name or "<no-event-captured>", last.arg or -1, #captured
            "#,
            )
            .expect("first-event-trigger readout must run cleanly");

        assert_eq!(
            first_call_count, 1,
            "Expected exactly ONE EventRegistry trigger after the first SetCategories — \
             line 72 fires `EventRegistry:TriggerEvent(\"AccountStore.CategorySelected\", \
             categories[1])`. Got {first_call_count}. A zero reading would prove the \
             selection-trigger was removed (which would leave the UI without a selected \
             category after refresh); a value > 1 would prove a redundant trigger was added."
        );

        assert_eq!(
            first_event_name, "AccountStore.CategorySelected",
            "Expected the first event name to be \"AccountStore.CategorySelected\" per line \
             72. Got `{first_event_name}`. A mismatch would prove the event topic was \
             renamed, breaking subscribers like \
             AccountStoreItemDisplayMixin:OnCategorySelected (item_display.lua:59) and \
             AccountStoreCategoryListMixin:OnCategorySelected (this file, line 26)."
        );

        assert_eq!(
            first_event_arg, FIRST_CATEGORY_ID,
            "Expected the first event arg to be {FIRST_CATEGORY_ID} (= categories[1] from \
             the input array). Got {first_event_arg}. A mismatch would prove the trigger \
             arg path was changed (e.g. wrapped in a table, replaced with the data-provider \
             element, or pulled from a different source)."
        );

        env.eval::<()>(&format!(
            r#"
            AccountStoreCategoryListMixin.SetCategories(
                _G.__behavior_category_list_refresh_stub_list,
                {{ {REPLACED_FIRST_CATEGORY_ID}, {REPLACED_SECOND_CATEGORY_ID} }}
            )
            return
            "#
        ))
        .expect("second SetCategories invocation must run cleanly");

        let (second_event_arg, total_call_count): (i64, i64) = env
            .eval(
                r#"
                local captured = _G.__behavior_category_list_refresh_event_registry_calls or {}
                local last = captured[#captured] or {}
                return last.arg or -1, #captured
                "#,
            )
            .expect("second-event-trigger readout must run cleanly");

        assert_eq!(
            total_call_count, 2,
            "Expected exactly TWO total EventRegistry triggers after two SetCategories \
             calls — each call fires once unconditionally. Got {total_call_count}. A \
             reading of 1 would prove the second trigger was suppressed; a value > 2 \
             would prove a redundant trigger leaked through."
        );

        assert_eq!(
            second_event_arg, REPLACED_FIRST_CATEGORY_ID,
            "Expected the second event arg to be {REPLACED_FIRST_CATEGORY_ID} (the FIRST \
             element of the new categories array), NOT {FIRST_CATEGORY_ID} (the \
             previously-passed first element). The PLAN's claim of \"preserving selection \
             across refresh\" would predict {FIRST_CATEGORY_ID} here — that selection state \
             survives the refresh. The actual behavior at line 72 fires \
             `EventRegistry:TriggerEvent(\"AccountStore.CategorySelected\", categories[1])` \
             with the NEW first element, RESETTING selection to the first of the new array. \
             Comment at line 71 even says \"Start with the first category selected.\" Got \
             {second_event_arg}. A reading of {FIRST_CATEGORY_ID} would prove the PLAN's \
             selection-preserving claim came true (a real upstream change)."
        );

        teardown_stub_category_list(env);
        teardown_event_registry_trigger_capture(env);
    });
}

fn seed_get_categories_tracker(env: &WowLuaEnv) {
    env.eval::<()>(&format!(
        r#"
        _G.__behavior_category_list_refresh_get_categories_calls = 0
        _G.__behavior_category_list_refresh_get_categories_arg = -1
        _G.__behavior_category_list_refresh_original_get_categories =
            C_AccountStore.GetCategories
        C_AccountStore.GetCategories = function(store_front_id)
            _G.__behavior_category_list_refresh_get_categories_calls =
                _G.__behavior_category_list_refresh_get_categories_calls + 1
            _G.__behavior_category_list_refresh_get_categories_arg = store_front_id or -1
            return {{ {FIRST_CATEGORY_ID}, {SECOND_CATEGORY_ID}, {THIRD_CATEGORY_ID} }}
        end
        return
        "#
    ))
    .expect("seeding GetCategories tracker must run cleanly");
}

fn teardown_get_categories_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        C_AccountStore.GetCategories =
            _G.__behavior_category_list_refresh_original_get_categories
        _G.__behavior_category_list_refresh_original_get_categories = nil
        _G.__behavior_category_list_refresh_get_categories_calls = nil
        _G.__behavior_category_list_refresh_get_categories_arg = nil
        return
        "#,
    )
    .expect("GetCategories tracker tear-down must run cleanly");
}

fn seed_get_category_info_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_category_list_refresh_get_category_info_calls = 0
        _G.__behavior_category_list_refresh_original_get_category_info =
            C_AccountStore.GetCategoryInfo
        C_AccountStore.GetCategoryInfo = function(_category_id)
            _G.__behavior_category_list_refresh_get_category_info_calls =
                _G.__behavior_category_list_refresh_get_category_info_calls + 1
            return { name = "stub", icon = "stub" }
        end
        return
        "#,
    )
    .expect("seeding GetCategoryInfo tracker must run cleanly");
}

fn teardown_get_category_info_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        C_AccountStore.GetCategoryInfo =
            _G.__behavior_category_list_refresh_original_get_category_info
        _G.__behavior_category_list_refresh_original_get_category_info = nil
        _G.__behavior_category_list_refresh_get_category_info_calls = nil
        return
        "#,
    )
    .expect("GetCategoryInfo tracker tear-down must run cleanly");
}

fn seed_event_registry_trigger_capture(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_category_list_refresh_event_registry_calls = {}
        _G.__behavior_category_list_refresh_original_trigger_event =
            EventRegistry.TriggerEvent
        EventRegistry.TriggerEvent = function(_self, event_name, arg)
            local captured = _G.__behavior_category_list_refresh_event_registry_calls
            captured[#captured + 1] = { event_name = event_name, arg = arg }
        end
        return
        "#,
    )
    .expect("seeding EventRegistry.TriggerEvent capture must run cleanly");
}

fn teardown_event_registry_trigger_capture(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        EventRegistry.TriggerEvent =
            _G.__behavior_category_list_refresh_original_trigger_event
        _G.__behavior_category_list_refresh_original_trigger_event = nil
        _G.__behavior_category_list_refresh_event_registry_calls = nil
        return
        "#,
    )
    .expect("EventRegistry.TriggerEvent capture tear-down must run cleanly");
}

fn seed_set_categories_capture(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_category_list_refresh_set_categories_capture = {}
        _G.__behavior_category_list_refresh_original_set_categories =
            AccountStoreCategoryListMixin.SetCategories
        AccountStoreCategoryListMixin.SetCategories = function(_self, categories)
            local capture = _G.__behavior_category_list_refresh_set_categories_capture
            capture[#capture + 1] = { categories = categories }
        end
        return
        "#,
    )
    .expect("seeding SetCategories capture must run cleanly");
}

fn teardown_set_categories_capture(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        AccountStoreCategoryListMixin.SetCategories =
            _G.__behavior_category_list_refresh_original_set_categories
        _G.__behavior_category_list_refresh_original_set_categories = nil
        _G.__behavior_category_list_refresh_set_categories_capture = nil
        return
        "#,
    )
    .expect("SetCategories capture tear-down must run cleanly");
}

fn seed_stub_category_list(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        local scroll_box = {}
        scroll_box.__data_providers = {}
        scroll_box.SetDataProvider = function(self, provider, retain_flag)
            local captured = self.__data_providers
            captured[#captured + 1] = { provider = provider, retain_flag = retain_flag }
        end

        local stub = {}
        stub.ScrollBox = scroll_box
        stub.SetCategories = AccountStoreCategoryListMixin.SetCategories

        _G.__behavior_category_list_refresh_stub_list = stub
        return
        "#,
    )
    .expect("seeding stub category list must run cleanly");
}

fn teardown_stub_category_list(env: &WowLuaEnv) {
    env.eval::<()>("_G.__behavior_category_list_refresh_stub_list = nil; return")
        .expect("stub category list tear-down must run cleanly");
}
