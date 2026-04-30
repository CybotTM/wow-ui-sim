//! Behavior pin for category-selection event flow in the
//! Blizzard_AccountStore lane.
//!
//! Spec/source mismatch finding (PLAN.md task naming
//! `AccountStoreMixin:CategorySelected(categoryID)`): the plan describes
//! a single `AccountStoreMixin:CategorySelected` entry point that
//! triggers an `EventRegistry` callback that drives
//! `AccountStoreItemDisplayMixin:OnCategorySelected`, which calls
//! `C_AccountStore.GetCategoryItems(categoryID)` and "resets the page
//! index to 1". Every clause of that description is wrong against the
//! actual source:
//!
//! 1. **Entry point name mismatch.** There is NO `CategorySelected`
//!    method on `AccountStoreMixin`. The mixin only defines `OnLoad`,
//!    `OnShow`, `OnHide`, `SetStoreFrontID`, and `SetFullscreenMode`
//!    (`Blizzard_AccountStore.lua:18-64`). The actual trigger sites
//!    for the `"AccountStore.CategorySelected"` event live on
//!    different mixins:
//!    - `AccountStoreCategoryMixin:OnClick` at
//!      `Blizzard_AccountStoreCategoryList.lua:4-7` — the per-row
//!      button click that fires when a user selects a category.
//!    - `AccountStoreCategoryListMixin:SetCategories` at
//!      `Blizzard_AccountStoreCategoryList.lua:67-73` — the auto-select
//!      branch that fires `categories[1]` after the data provider is
//!      built.
//!
//! 2. **"Drives one handler" mismatch.** The event has TWO subscribers,
//!    not one. Both are registered via `AddStaticEventMethod`:
//!    - `AccountStoreItemDisplayMixin.OnCategorySelected` (registered
//!      at `Blizzard_AccountStoreItemDisplay.lua:59`).
//!    - `AccountStoreCategoryListMixin.OnCategorySelected` (registered
//!      at `Blizzard_AccountStoreCategoryList.lua:26`).
//!    The PLAN names only the ItemDisplay subscriber and treats the
//!    CategoryList subscriber as if it didn't exist. The CategoryList
//!    handler at `Blizzard_AccountStoreCategoryList.lua:57-65` updates
//!    the row-selected highlight in the scroll box — a UI side effect
//!    that consumers depend on for the "selected category looks
//!    selected" contract.
//!
//! 3. **C_API call gating mismatch.** The PLAN reads as if
//!    `OnCategorySelected` always calls `C_AccountStore.GetCategoryItems`.
//!    The actual body at `Blizzard_AccountStoreItemDisplay.lua:114-135`
//!    gates ALL of the GetCategoryItems / GetCategoryInfo / itemRack
//!    work behind `if categoryID ~= self.categoryID or forceUpdate`
//!    (line 116). Re-firing the event with the same `categoryID` and
//!    no `forceUpdate` means the C_API path is SKIPPED entirely —
//!    only the trailing `SetPage(pageToShow, pageForceUpdate)` runs,
//!    and `pageForceUpdate` stays at the caller-supplied `forceUpdate`
//!    (nil/false), so even SetPage may early-return at its own
//!    `page == self.currentPage and not forceUpdate` gate.
//!
//! 4. **"Resets the page index to 1" mismatch.** The body at
//!    `Blizzard_AccountStoreItemDisplay.lua:132` reads
//!    `local pageToShow = self.categoryLastPage[categoryID] or 1` —
//!    a per-category MEMORIZED last-viewed page, not a constant-1
//!    reset. The `or 1` fallback fires only when a category is
//!    selected for the first time (no entry in `categoryLastPage`).
//!    For categories the user has already navigated, the page index
//!    is restored to where they left off. This is the OPPOSITE of
//!    "reset to 1": the design intentionally preserves per-category
//!    pagination state across category switches.
//!
//! Four tests pin the contract:
//!
//! - `category_selected_method_is_absent_from_account_store_mixin`
//!   asserts `AccountStoreMixin.CategorySelected` is nil. Pins the
//!   PLAN-named entry point as ABSENT — the structural tripwire
//!   that flips if Blizzard ever adds a method by that name to the
//!   parent mixin.
//! - `account_store_category_mixin_on_click_triggers_account_store_category_selected_event_with_category_id`
//!   replaces `EventRegistry.TriggerEvent` with a Lua tracker, builds
//!   a stub-self with a sentinel `categoryID`, directly invokes
//!   `AccountStoreCategoryMixin.OnClick(stub_self)`, and asserts the
//!   tracker recorded exactly `("AccountStore.CategorySelected",
//!   <sentinel>)` — pins the actual trigger site (NOT
//!   `AccountStoreMixin`) and the actual event name.
//! - `account_store_category_selected_event_has_two_subscribers_in_lane_not_one`
//!   asserts BOTH `AccountStoreItemDisplayMixin.OnCategorySelected`
//!   and `AccountStoreCategoryListMixin.OnCategorySelected` are
//!   defined as functions — pins the dual-subscriber model that
//!   the PLAN's "drives one handler" framing omits.
//! - `item_display_on_category_selected_uses_memorized_per_category_page_not_resetting_to_one`
//!   builds a plain-table stub self with `categoryLastPage = {[42] = 7}`,
//!   replaces `self.SetPage` with a tracker, replaces `C_AccountStore`
//!   methods with stubs, and invokes the mixin method directly. Asserts
//!   SetPage was called with `page = 7` for the memorized id, then
//!   re-invokes for an unseen id (99) and asserts `page = 1` for the
//!   fallback — pins the memoized per-category page logic that
//!   refutes the "resets to 1" PLAN claim.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AccountStore";

#[test]
fn category_selected_method_is_absent_from_account_store_mixin() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let category_selected_type: String = env
            .eval("return type(AccountStoreMixin.CategorySelected)")
            .expect("`AccountStoreMixin.CategorySelected` type probe must run cleanly");

        assert_eq!(
            category_selected_type, "nil",
            "Expected `AccountStoreMixin.CategorySelected` to be nil — the PLAN.md task names \
             `AccountStoreMixin:CategorySelected(categoryID)` as the entry point that triggers \
             the category-selection event, but the AccountStoreMixin only defines OnLoad, \
             OnShow, OnHide, SetStoreFrontID, and SetFullscreenMode (no `CategorySelected` \
             method anywhere in `Blizzard_AccountStore.lua`). The actual trigger sites for \
             `\"AccountStore.CategorySelected\"` live on (a) `AccountStoreCategoryMixin:OnClick` \
             at `Blizzard_AccountStoreCategoryList.lua:4-7` (per-row button click) and (b) \
             `AccountStoreCategoryListMixin:SetCategories` at `Blizzard_AccountStoreCategoryList.lua:67-73` \
             (auto-select first category branch). A non-nil reading here means either (a) \
             Blizzard added a `CategorySelected` method to AccountStoreMixin (forcing a re-pin \
             against the new entry point — and likely a refactor of the trigger-site \
             distribution), (b) some addon in the smoke shape monkey-patched the mixin (worth \
             investigating because it would shadow a future Blizzard addition), or (c) a \
             re-export from another mixin via the simulator's class-mixin merging accidentally \
             surfaces a `CategorySelected` field (a regression in the mixin-merge logic)."
        );
    });
}

#[test]
fn account_store_category_mixin_on_click_triggers_account_store_category_selected_event_with_category_id()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const SENTINEL_CATEGORY_ID: i64 = 654_321;

        env.eval::<()>(&format!(
            r#"
            _G.__behavior_category_selected_trigger_event_name = nil
            _G.__behavior_category_selected_trigger_payload = nil
            _G.__behavior_category_selected_trigger_call_count = 0
            _G.__behavior_category_selected_original_trigger = EventRegistry.TriggerEvent
            EventRegistry.TriggerEvent = function(_self, event_name, payload)
                _G.__behavior_category_selected_trigger_event_name = event_name
                _G.__behavior_category_selected_trigger_payload = payload
                _G.__behavior_category_selected_trigger_call_count =
                    _G.__behavior_category_selected_trigger_call_count + 1
            end
            _G.__behavior_category_selected_original_play_sound = PlaySound
            PlaySound = function() end
            _G.__behavior_category_selected_stub_self = {{ categoryID = {SENTINEL_CATEGORY_ID} }}
            return
            "#
        ))
        .expect(
            "replacing EventRegistry.TriggerEvent + PlaySound with Lua trackers and seeding \
             stub_self must run cleanly",
        );

        env.eval::<()>(
            r#"
            AccountStoreCategoryMixin.OnClick(_G.__behavior_category_selected_stub_self)
            return
            "#,
        )
        .expect(
            "Direct invocation of `AccountStoreCategoryMixin.OnClick(stub_self)` must run \
             cleanly — the body at `Blizzard_AccountStoreCategoryList.lua:4-7` is two lines: \
             PlaySound(SOUNDKIT.ACCOUNT_STORE_CATEGORY_SELECT) and EventRegistry:TriggerEvent. \
             Both globals are replaced by the test's trackers, so the body runs without \
             touching real audio or the real registry",
        );

        let (event_name, payload, call_count): (String, i64, i64) = env
            .eval(
                r#"
                return _G.__behavior_category_selected_trigger_event_name or "<nil>",
                       _G.__behavior_category_selected_trigger_payload or -1,
                       _G.__behavior_category_selected_trigger_call_count
                "#,
            )
            .expect("post-OnClick trigger tracker probe must run cleanly");

        assert_eq!(
            call_count, 1,
            "Expected EventRegistry.TriggerEvent to have been invoked exactly once after \
             `AccountStoreCategoryMixin.OnClick(stub_self)`. The body at \
             `Blizzard_AccountStoreCategoryList.lua:6` calls `EventRegistry:TriggerEvent(...)` \
             unconditionally — one OnClick MUST produce exactly one TriggerEvent call. A zero \
             count means OnClick errored before reaching the trigger line (the only line above \
             is `PlaySound(SOUNDKIT.ACCOUNT_STORE_CATEGORY_SELECT)` which the test stubs to a \
             no-op). A count > 1 means the body now fires multiple events per click (worth \
             investigating because consumers implicitly assume one-event-per-click)."
        );

        assert_eq!(
            event_name, "AccountStore.CategorySelected",
            "Expected the recorded event name to equal \"AccountStore.CategorySelected\" — the \
             literal event-name string passed at `Blizzard_AccountStoreCategoryList.lua:6`. A \
             different reading means the body started using a different event identifier \
             (forcing a re-pin against the new event name — and likely re-keying the two \
             AddStaticEventMethod registrations at lines 26 and 59 that depend on this exact \
             name)."
        );

        assert_eq!(
            payload, SENTINEL_CATEGORY_ID,
            "Expected the recorded TriggerEvent payload to equal the sentinel \
             ({SENTINEL_CATEGORY_ID}) — the value seeded into `stub_self.categoryID`. The body \
             reads `self.categoryID` BY NAME at the call site (`Blizzard_AccountStoreCategoryList.lua:6` \
             — `EventRegistry:TriggerEvent(\"AccountStore.CategorySelected\", self.categoryID)`). \
             A different recorded value means either (a) the body started passing a different \
             field (forcing a re-pin against the new payload contract), or (b) a wrapper layer \
             intercepts and rewrites the payload before TriggerEvent sees it (a regression \
             worth investigating because both subscribers receive `categoryID` directly)."
        );

        env.eval::<()>(
            r#"
            EventRegistry.TriggerEvent = _G.__behavior_category_selected_original_trigger
            PlaySound = _G.__behavior_category_selected_original_play_sound
            _G.__behavior_category_selected_original_trigger = nil
            _G.__behavior_category_selected_original_play_sound = nil
            _G.__behavior_category_selected_trigger_event_name = nil
            _G.__behavior_category_selected_trigger_payload = nil
            _G.__behavior_category_selected_trigger_call_count = nil
            _G.__behavior_category_selected_stub_self = nil
            return
            "#,
        )
        .expect("trigger/playsound tracker tear-down must run cleanly");
    });
}

#[test]
fn account_store_category_selected_event_has_two_subscribers_in_lane_not_one() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (item_display_type, category_list_type): (String, String) = env
            .eval(
                r#"
                return type(AccountStoreItemDisplayMixin.OnCategorySelected),
                       type(AccountStoreCategoryListMixin.OnCategorySelected)
                "#,
            )
            .expect("dual-subscriber type probe must run cleanly");

        assert_eq!(
            item_display_type, "function",
            "Expected `AccountStoreItemDisplayMixin.OnCategorySelected` to be a function — \
             defined at `Blizzard_AccountStoreItemDisplay.lua:114-135` and registered as an \
             EventRegistry static-method callback at line 59. A nil/non-function reading means \
             the mixin definition was removed or renamed (forcing a re-pin against the new \
             handler shape — and likely re-routing the C_API GetCategoryItems / GetCategoryInfo \
             call paths that this handler hosts)."
        );

        assert_eq!(
            category_list_type, "function",
            "Expected `AccountStoreCategoryListMixin.OnCategorySelected` to be a function — \
             defined at `Blizzard_AccountStoreCategoryList.lua:57-65` and registered as an \
             EventRegistry static-method callback at line 26. The PLAN.md task description \
             treats `AccountStoreItemDisplayMixin:OnCategorySelected` as the SINGLE handler \
             driven by the event, but the CategoryList subscriber ALSO runs on every \
             AccountStore.CategorySelected fire — its body finds the matching scroll-box row \
             and updates the row-selected highlight (`Blizzard_AccountStoreCategoryList.lua:58-64`). \
             A nil reading here means Blizzard removed the dual-subscriber model (forcing a \
             re-pin against the new single-handler shape — and likely re-implementing the \
             row-selected-highlight update via a different path like a per-row OnClick callback \
             rather than the central event)."
        );
    });
}

#[test]
fn item_display_on_category_selected_uses_memorized_per_category_page_not_resetting_to_one() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const MEMORIZED_CATEGORY_ID: i64 = 42;
        const MEMORIZED_PAGE: i64 = 7;
        const UNSEEN_CATEGORY_ID: i64 = 99;
        const FALLBACK_PAGE: i64 = 1;
        const STUB_CATEGORY_TYPE: &str = "STUB_CATEGORY_TYPE";

        env.eval::<()>(&format!(
            r#"
            _G.__behavior_category_selected_set_page_calls = {{}}
            _G.__behavior_category_selected_get_category_items_calls = 0
            _G.__behavior_category_selected_original_get_category_items =
                C_AccountStore.GetCategoryItems
            _G.__behavior_category_selected_original_get_category_info =
                C_AccountStore.GetCategoryInfo
            C_AccountStore.GetCategoryItems = function()
                _G.__behavior_category_selected_get_category_items_calls =
                    _G.__behavior_category_selected_get_category_items_calls + 1
                return {{}}
            end
            C_AccountStore.GetCategoryInfo = function()
                return {{ type = {STUB_CATEGORY_TYPE:?} }}
            end
            local stub_item_rack = {{
                Hide = function() end,
                Show = function() end,
                GetMaxCards = function() return 4 end,
                SetItems = function() end,
                SetCategoryType = function() end,
                SetPoint = function() end,
            }}
            _G.__behavior_category_selected_stub_self = {{
                categoryID = nil,
                categoryItems = nil,
                currentPage = 0,
                categoryLastPage = {{ [{MEMORIZED_CATEGORY_ID}] = {MEMORIZED_PAGE} }},
                areItemsAvailable = false,
                categoryTypeToItemRack = {{ [{STUB_CATEGORY_TYPE:?}] = stub_item_rack }},
                currentItemRack = nil,
                CreateItemRack = function() return stub_item_rack end,
                SetPage = function(_self, page, force_update)
                    table.insert(
                        _G.__behavior_category_selected_set_page_calls,
                        {{ page = page, force_update = force_update }}
                    )
                end,
            }}
            return
            "#
        ))
        .expect(
            "stub-self construction + C_AccountStore tracker installation must run cleanly — \
             the test rebuilds a minimal AccountStoreItemDisplayMixin instance shape with the \
             specific fields the OnCategorySelected body reads/writes, plus a SetPage tracker \
             that records each invocation",
        );

        env.eval::<()>(&format!(
            r#"
            AccountStoreItemDisplayMixin.OnCategorySelected(
                _G.__behavior_category_selected_stub_self,
                {MEMORIZED_CATEGORY_ID},
                true
            )
            return
            "#
        ))
        .expect(
            "Direct invocation of `AccountStoreItemDisplayMixin.OnCategorySelected(stub, 42, \
             true)` must run cleanly — `forceUpdate=true` enters the gate at line 116, \
             GetCategoryItems and GetCategoryInfo run (both stubbed), GetOrCreateTableEntryByCallback \
             finds the pre-seeded itemRack so CreateItemRack is not invoked, currentItemRack \
             is nil (no prior selection), pageToShow resolves to categoryLastPage[42]=7, and \
             SetPage(self, 7, true) runs through the stub tracker",
        );

        let memorized_page_calls: Vec<i64> = env
            .eval(
                r#"
                local results = {}
                for _, call in ipairs(_G.__behavior_category_selected_set_page_calls) do
                    table.insert(results, call.page)
                end
                return results
                "#,
            )
            .expect("post-memorized-call SetPage call list probe must run cleanly");

        assert_eq!(
            memorized_page_calls.len(),
            1,
            "Expected exactly one SetPage call after the memorized-id invocation. The body at \
             `Blizzard_AccountStoreItemDisplay.lua:114-135` calls `self:SetPage(pageToShow, \
             pageForceUpdate)` once at line 133 — the only SetPage call site inside \
             OnCategorySelected. A different count means either (a) Blizzard added another \
             SetPage call (forcing a re-pin), or (b) the body now routes through a different \
             pagination method, or (c) the gate at line 116 took an unexpected branch."
        );

        assert_eq!(
            memorized_page_calls[0], MEMORIZED_PAGE,
            "Expected SetPage to have been called with page={MEMORIZED_PAGE} (the value pre-seeded \
             into `stub_self.categoryLastPage[{MEMORIZED_CATEGORY_ID}]`) — pins the memoized \
             per-category page logic at `Blizzard_AccountStoreItemDisplay.lua:132`: \
             `local pageToShow = self.categoryLastPage[categoryID] or 1`. The PLAN.md task \
             description says OnCategorySelected \"resets the page index to 1\" — a constant-1 \
             reset would have produced page=1 here, which is the OPPOSITE of the actual \
             behavior. The design intentionally preserves per-category pagination state across \
             category switches: switching from cat A page 3 to cat B page 5 and back to cat A \
             returns to A's page 3, not page 1. A reading of {FALLBACK_PAGE} here means the \
             body adopted the PLAN-described constant-1 reset pattern (forcing a re-pin against \
             the new behavior — and likely retiring the `categoryLastPage` table at line 13 \
             since it would no longer be consulted)."
        );

        env.eval::<()>(&format!(
            r#"
            _G.__behavior_category_selected_set_page_calls = {{}}
            AccountStoreItemDisplayMixin.OnCategorySelected(
                _G.__behavior_category_selected_stub_self,
                {UNSEEN_CATEGORY_ID},
                true
            )
            return
            "#
        ))
        .expect(
            "Direct invocation of `AccountStoreItemDisplayMixin.OnCategorySelected(stub, 99, \
             true)` must run cleanly — same flow but for the unseen-category branch",
        );

        let unseen_page_calls: Vec<i64> = env
            .eval(
                r#"
                local results = {}
                for _, call in ipairs(_G.__behavior_category_selected_set_page_calls) do
                    table.insert(results, call.page)
                end
                return results
                "#,
            )
            .expect("post-unseen-call SetPage call list probe must run cleanly");

        assert_eq!(
            unseen_page_calls.len(),
            1,
            "Expected exactly one SetPage call after the unseen-id invocation, mirroring the \
             memorized-id case"
        );

        assert_eq!(
            unseen_page_calls[0], FALLBACK_PAGE,
            "Expected SetPage to have been called with page={FALLBACK_PAGE} (the `or 1` fallback \
             at `Blizzard_AccountStoreItemDisplay.lua:132`) — pins the path where \
             `categoryLastPage[{UNSEEN_CATEGORY_ID}]` is nil and the expression evaluates to \
             the literal 1. This is the ONLY situation in which the page index actually \
             resolves to 1 — when a category is selected for the first time. The companion \
             memorized-id assertion above proves that previously-seen categories return to \
             their last-viewed page, NOT to 1."
        );

        let get_category_items_calls: i64 = env
            .eval("return _G.__behavior_category_selected_get_category_items_calls")
            .expect("GetCategoryItems call counter probe must run cleanly");

        assert_eq!(
            get_category_items_calls, 2,
            "Expected `C_AccountStore.GetCategoryItems` to have been called exactly twice — \
             once per OnCategorySelected invocation. The gate at \
             `Blizzard_AccountStoreItemDisplay.lua:116` (`if categoryID ~= self.categoryID or \
             forceUpdate`) is taken on both calls because (a) the first call switches \
             categoryID from nil to {MEMORIZED_CATEGORY_ID}, and (b) the second call switches \
             from {MEMORIZED_CATEGORY_ID} to {UNSEEN_CATEGORY_ID}. Inside the gate, line 118 \
             unconditionally calls `C_AccountStore.GetCategoryItems(categoryID)`. A count of 0 \
             or 1 means either the gate logic changed (forcing a re-pin) or the GetCategoryItems \
             call was moved out of the gate (the PLAN-shaped \"always calls\" pattern, which \
             would invert the behavior under same-id re-fires)."
        );

        env.eval::<()>(
            r#"
            C_AccountStore.GetCategoryItems =
                _G.__behavior_category_selected_original_get_category_items
            C_AccountStore.GetCategoryInfo =
                _G.__behavior_category_selected_original_get_category_info
            _G.__behavior_category_selected_original_get_category_items = nil
            _G.__behavior_category_selected_original_get_category_info = nil
            _G.__behavior_category_selected_get_category_items_calls = nil
            _G.__behavior_category_selected_set_page_calls = nil
            _G.__behavior_category_selected_stub_self = nil
            return
            "#,
        )
        .expect("C_AccountStore restore + tracker tear-down must run cleanly");
    });
}
