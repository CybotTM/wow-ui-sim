//! Behavior pin for `AccountStoreItemDisplayMixin:SetPage` paging contract.
//!
//! Spec/source mismatch finding (PLAN.md task:
//! `AccountStoreItemDisplayMixin:SetPage(n)` clamps to valid range and calls
//! `AccountStoreItemRackMixin:Refresh` with the matching slice of category
//! items): the plan makes three claims that diverge from the actual source at
//! `Blizzard_AccountStoreItemDisplay.lua:150-175` and
//! `Blizzard_AccountStoreItemRack.lua:18-56`.
//!
//! 1. **`SetPage` does NOT call `AccountStoreItemRackMixin:Refresh` directly.**
//!    Line 170 reads `self.currentItemRack:SetItems(items);`. The PLAN names
//!    the wrong dispatch point. `AccountStoreItemRackMixin:SetItems` at lines
//!    26-29 of ItemRack.lua is the actual entry SetPage hits — and SetItems
//!    in turn calls `self:Refresh()` internally. So the rack DOES end up
//!    re-rendered, but the call chain is `SetPage -> SetItems -> Refresh`,
//!    not `SetPage -> Refresh`. A regression that bypassed SetItems and
//!    called Refresh directly would skip the `self.items = items;` assignment
//!    at line 27 — Refresh reads `local items = self.items;` at line 34, so
//!    the new page's items never reach the cardPool factory.
//!
//! 2. **The slice passed to SetItems is CUMULATIVE/buggy for pages above 1.**
//!    Lines 162-166 read:
//!
//!    ```lua
//!    local maxCardsPerPage = self.currentItemRack:GetMaxCards();
//!    for i = 1, page * maxCardsPerPage do
//!        local itemIndex = (page - 1) * maxCardsPerPage + i;
//!        table.insert(items, self.categoryItems[itemIndex]);
//!    end
//!    ```
//!
//!    For page=1 with maxCardsPerPage=4 the loop iterates 4 times producing
//!    items[1..4] — the "matching slice" the PLAN describes. For page=2 with
//!    maxCardsPerPage=4 the loop iterates `2*4=8` times producing
//!    `categoryItems[5..12]`, an 8-element slice for a 4-card rack. The
//!    excess items are silently dropped at the rack render gate (line 47 of
//!    ItemRack.lua: `math.min(#items, self:GetMaxCards())`), but the items
//!    table SetItems receives is not page-2-shaped. The PLAN's "matching
//!    slice" framing only describes the page=1 case correctly. Pinning this
//!    actual behavior gives downstream consumers a tripwire if Blizzard
//!    fixes the upstream loop bound to `for i = 1, maxCardsPerPage` (the
//!    likely intended form).
//!
//! 3. **The "C_AccountStore.GetCategoryItems gap" depends-on is misframed.**
//!    SetPage does not call `C_AccountStore.GetCategoryItems`. The category
//!    items are fetched at `OnCategorySelected` line 118 and stored on
//!    `self.categoryItems`. SetPage only reads from the cached array via
//!    `self.categoryItems[itemIndex]`. So the C_API surface is irrelevant
//!    to the SetPage contract — the gap blocks OnCategorySelected, not
//!    SetPage.
//!
//! Five tests pin the contract:
//!
//! - `set_page_method_exists_on_item_display_mixin` — surface check that
//!   `AccountStoreItemDisplayMixin.SetPage` is a function (and that
//!   `Refresh` does NOT exist on the display mixin, only on the rack mixin).
//!
//! - `set_page_calls_set_items_on_rack_not_refresh_directly` — replaces both
//!   `SetItems` and `Refresh` with trackers; invokes
//!   `AccountStoreItemDisplayMixin.SetPage(stub_display, 1)`; asserts
//!   SetItems was called exactly once and Refresh was called ZERO times by
//!   the SetPage body itself (Refresh runs inside the stubbed SetItems, but
//!   the stub doesn't forward — proving SetPage's direct dispatch target is
//!   SetItems).
//!
//! - `set_page_clamps_below_one_to_one` — invokes SetPage with page=0
//!   (sub-clamp) and page=-5; asserts `stub_display.currentPage == 1` after
//!   each call. Pins `Clamp(page, 1, maxPage)` at line 152.
//!
//! - `set_page_clamps_above_max_to_max_page` — invokes SetPage with
//!   page=999 (way above maxPage); asserts `stub_display.currentPage ==
//!   maxPage`. Pins the upper-bound clamp.
//!
//! - `set_page_passes_cumulative_buggy_slice_for_page_above_one` — seeds
//!   `categoryItems` with sentinels {101, 102, ..., 120}, maxCardsPerPage=4;
//!   invokes SetPage(page=2, forceUpdate=true); asserts the slice captured
//!   by the SetItems tracker has length 8 (NOT 4) and starts at sentinel
//!   105 (= categoryItems[5]). Pins the actual buggy loop bound. A correct
//!   "matching slice" implementation would hand SetItems exactly 4 sentinels
//!   {105, 106, 107, 108}; the actual code hands it 8 sentinels {105..112}.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccountStore";

#[test]
fn set_page_method_exists_on_item_display_mixin() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let set_page_type: String = env
            .eval("return type(AccountStoreItemDisplayMixin.SetPage)")
            .expect("AccountStoreItemDisplayMixin.SetPage probe must run cleanly");

        assert_eq!(
            set_page_type, "function",
            "Expected `type(AccountStoreItemDisplayMixin.SetPage) == \"function\"` \
             (`Blizzard_AccountStoreItemDisplay.lua:150-175`), got `{set_page_type}`. A non-function \
             reading would prove SetPage moved off the display mixin (e.g. onto the rack mixin or \
             a new pager mixin), forcing a re-pin against the new dispatch shape."
        );

        let display_refresh_type: String = env
            .eval("return type(AccountStoreItemDisplayMixin.Refresh)")
            .expect("AccountStoreItemDisplayMixin.Refresh probe must run cleanly");

        assert_eq!(
            display_refresh_type, "nil",
            "Expected `type(AccountStoreItemDisplayMixin.Refresh) == \"nil\"` (PLAN.md spec/source \
             mismatch tripwire — the plan names `AccountStoreItemRackMixin:Refresh` as SetPage's \
             direct dispatch target, but Refresh is a method on the RACK mixin, not the display \
             mixin; SetPage actually calls `self.currentItemRack:SetItems(items)` at line 170, \
             which then calls Refresh internally), got `{display_refresh_type}`. A non-nil reading \
             would prove a Refresh shim landed on the display mixin and the SetPage->SetItems \
             chain was bypassed."
        );

        let rack_set_items_type: String = env
            .eval("return type(AccountStoreItemRackMixin.SetItems)")
            .expect("AccountStoreItemRackMixin.SetItems probe must run cleanly");

        assert_eq!(
            rack_set_items_type, "function",
            "Expected `type(AccountStoreItemRackMixin.SetItems) == \"function\"` \
             (`Blizzard_AccountStoreItemRack.lua:26-29`), got `{rack_set_items_type}`. A non-function \
             reading would prove the rack lost its actual SetPage dispatch entry — SetPage's \
             `self.currentItemRack:SetItems(items)` would error at runtime, which would manifest \
             as broken paging in the live UI."
        );
    });
}

#[test]
fn set_page_calls_set_items_on_rack_not_refresh_directly() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_rack_call_trackers(env);
        seed_stub_display(
            env,
            StubDisplaySeed {
                max_cards_per_page: 4,
                item_count: 8,
            },
        );

        env.eval::<()>(
            r#"
            AccountStoreItemDisplayMixin.SetPage(_G.__behavior_item_rack_paging_stub_display, 1, true)
            return
            "#,
        )
        .expect("Direct SetPage invocation must run cleanly");

        let (set_items_calls, refresh_calls): (i64, i64) = env
            .eval(
                "return _G.__behavior_item_rack_paging_set_items_calls, \
                 _G.__behavior_item_rack_paging_refresh_calls",
            )
            .expect("Tracker readout must run cleanly");

        assert_eq!(
            set_items_calls, 1,
            "Expected exactly ONE `SetItems` call on the rack after SetPage(1, forceUpdate=true) \
             (`Blizzard_AccountStoreItemDisplay.lua:170` reads \
             `self.currentItemRack:SetItems(items)`), got {set_items_calls}. A zero reading would \
             prove SetPage stopped dispatching to SetItems (cards never refresh on page change); a \
             value > 1 would prove fan-out dispatch."
        );

        assert_eq!(
            refresh_calls, 0,
            "Expected ZERO direct `Refresh` calls from SetPage's body — the SetPage source at \
             `Blizzard_AccountStoreItemDisplay.lua:150-175` does not contain `Refresh` anywhere; \
             refresh runs INSIDE SetItems (`Blizzard_AccountStoreItemRack.lua:28`) and the stub \
             SetItems here intentionally does not forward. Got {refresh_calls}. A non-zero reading \
             would prove the PLAN's `SetPage -> Refresh` direct dispatch claim came true (a real \
             upstream change), forcing a re-pin against the new chain — and a regression risk \
             because Refresh without the SetItems-time `self.items = items` assignment renders \
             the previous page's items."
        );

        teardown_rack_call_trackers(env);
        teardown_stub_display(env);
    });
}

#[test]
fn set_page_clamps_below_one_to_one() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_rack_call_trackers(env);
        seed_stub_display(
            env,
            StubDisplaySeed {
                max_cards_per_page: 4,
                item_count: 12,
            },
        );

        for sub_one_page in [0_i64, -5, -100] {
            env.eval::<()>(&format!(
                r#"
                AccountStoreItemDisplayMixin.SetPage(
                    _G.__behavior_item_rack_paging_stub_display,
                    {sub_one_page},
                    true
                )
                return
                "#
            ))
            .unwrap_or_else(|error| {
                panic!("SetPage({sub_one_page}, true) must run cleanly: {error}")
            });

            let current_page: i64 = env
                .eval("return _G.__behavior_item_rack_paging_stub_display.currentPage")
                .expect("currentPage readout must run cleanly");

            assert_eq!(
                current_page, 1,
                "Expected `stub_display.currentPage == 1` after SetPage({sub_one_page}, true) — \
                 the body at `Blizzard_AccountStoreItemDisplay.lua:152` reads \
                 `page = Clamp(page, 1, maxPage)`, so any sub-1 input clamps to 1. Got \
                 {current_page}. A different reading would prove the lower bound of the Clamp call \
                 was dropped or shifted."
            );
        }

        teardown_rack_call_trackers(env);
        teardown_stub_display(env);
    });
}

#[test]
fn set_page_clamps_above_max_to_max_page() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const MAX_CARDS_PER_PAGE: i64 = 4;
        const ITEM_COUNT: i64 = 12;
        // maxPage = ceil(12/4) = 3
        const EXPECTED_MAX_PAGE: i64 = 3;

        seed_rack_call_trackers(env);
        seed_stub_display(
            env,
            StubDisplaySeed {
                max_cards_per_page: MAX_CARDS_PER_PAGE,
                item_count: ITEM_COUNT,
            },
        );

        env.eval::<()>(
            r#"
            AccountStoreItemDisplayMixin.SetPage(
                _G.__behavior_item_rack_paging_stub_display,
                999,
                true
            )
            return
            "#,
        )
        .expect("SetPage(999, true) must run cleanly");

        let current_page: i64 = env
            .eval("return _G.__behavior_item_rack_paging_stub_display.currentPage")
            .expect("currentPage readout must run cleanly");

        assert_eq!(
            current_page, EXPECTED_MAX_PAGE,
            "Expected `stub_display.currentPage == {EXPECTED_MAX_PAGE}` (= ceil(item_count / \
             max_cards_per_page) = ceil({ITEM_COUNT}/{MAX_CARDS_PER_PAGE})) after SetPage(999, \
             true) — the body at line 151 computes `local maxPage = self:GetMaxPage()` (= line 147 \
             `math.ceil(#self.categoryItems / self.currentItemRack:GetMaxCards())`) and line 152 \
             clamps `page = Clamp(page, 1, maxPage)`. Got {current_page}. A different reading \
             would prove either GetMaxPage's denominator changed (e.g. dropped the GetMaxCards \
             call) or the upper-bound clamp was dropped."
        );

        teardown_rack_call_trackers(env);
        teardown_stub_display(env);
    });
}

#[test]
fn set_page_passes_cumulative_buggy_slice_for_page_above_one() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        const MAX_CARDS_PER_PAGE: i64 = 4;
        const ITEM_COUNT: i64 = 20;
        // For page=2 with maxCardsPerPage=4, the buggy loop produces
        // 2*4 = 8 items at indices (2-1)*4 + i for i in 1..8, i.e.
        // categoryItems[5..12]. With sentinels at value = 100 + index,
        // the captured slice should be {105, 106, 107, 108, 109, 110, 111, 112}.
        const EXPECTED_SLICE_LEN: i64 = 8;
        const EXPECTED_FIRST_SLICE_VALUE: i64 = 105;
        const EXPECTED_LAST_SLICE_VALUE: i64 = 112;

        seed_rack_call_trackers(env);
        seed_stub_display(
            env,
            StubDisplaySeed {
                max_cards_per_page: MAX_CARDS_PER_PAGE,
                item_count: ITEM_COUNT,
            },
        );

        env.eval::<()>(
            r#"
            AccountStoreItemDisplayMixin.SetPage(
                _G.__behavior_item_rack_paging_stub_display,
                2,
                true
            )
            return
            "#,
        )
        .expect("SetPage(2, true) must run cleanly");

        let (slice_len, first_value, last_value): (i64, i64, i64) = env
            .eval(
                r#"
                local slice = _G.__behavior_item_rack_paging_captured_items
                local n = #slice
                return n, slice[1] or -1, slice[n] or -1
                "#,
            )
            .expect("Captured items readout must run cleanly");

        assert_eq!(
            slice_len, EXPECTED_SLICE_LEN,
            "Expected SetItems to receive a slice of length {EXPECTED_SLICE_LEN} after \
             SetPage(2, true) with max_cards_per_page={MAX_CARDS_PER_PAGE} and \
             item_count={ITEM_COUNT} — the loop at \
             `Blizzard_AccountStoreItemDisplay.lua:163` iterates `page * maxCardsPerPage` times \
             (= 2*4 = 8), NOT `maxCardsPerPage` times. Got {slice_len}. A reading of 4 would \
             prove Blizzard fixed the loop bound to `for i = 1, maxCardsPerPage` (the likely \
             intended form), which would be a behavior change worth flagging."
        );

        assert_eq!(
            first_value, EXPECTED_FIRST_SLICE_VALUE,
            "Expected the first slice element to equal sentinel {EXPECTED_FIRST_SLICE_VALUE} \
             (= categoryItems[5] = 100 + 5) — for page=2 the inner index is \
             `(page-1)*maxCardsPerPage + i = 1*4 + 1 = 5`. Got {first_value}. A different \
             reading would prove the slice base index changed (e.g. became 0-indexed or \
             dropped the `(page-1)*maxCardsPerPage` offset)."
        );

        assert_eq!(
            last_value, EXPECTED_LAST_SLICE_VALUE,
            "Expected the last slice element to equal sentinel {EXPECTED_LAST_SLICE_VALUE} \
             (= categoryItems[12] = 100 + 12) — for page=2 with `i=8` the inner index is \
             `1*4 + 8 = 12`. Got {last_value}. A different reading would prove the upper bound \
             of the loop changed (e.g. an off-by-one fix or a clamp to `#categoryItems`)."
        );

        teardown_rack_call_trackers(env);
        teardown_stub_display(env);
    });
}

fn seed_rack_call_trackers(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_item_rack_paging_set_items_calls = 0
        _G.__behavior_item_rack_paging_refresh_calls = 0
        _G.__behavior_item_rack_paging_captured_items = {}
        return
        "#,
    )
    .expect("seeding rack call trackers must run cleanly");
}

fn teardown_rack_call_trackers(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_item_rack_paging_set_items_calls = nil
        _G.__behavior_item_rack_paging_refresh_calls = nil
        _G.__behavior_item_rack_paging_captured_items = nil
        return
        "#,
    )
    .expect("rack call tracker tear-down must run cleanly");
}

struct StubDisplaySeed {
    max_cards_per_page: i64,
    item_count: i64,
}

fn seed_stub_display(env: &WowLuaEnv, seed: StubDisplaySeed) {
    seed_stub_display_skeleton(env, seed.item_count);
    seed_stub_rack_on_display(env, seed.max_cards_per_page);
    seed_stub_footer_on_display(env);
    finalize_stub_display_metatable(env);
}

fn seed_stub_display_skeleton(env: &WowLuaEnv, item_count: i64) {
    env.eval::<()>(&format!(
        r#"
        local stub_display = {{}}
        stub_display.__marker = "behavior_item_rack_paging_stub_display"
        stub_display.currentPage = 0
        stub_display.areItemsAvailable = true
        stub_display.categoryID = 7
        stub_display.categoryLastPage = {{}}

        local category_items = {{}}
        for i = 1, {item_count} do
            category_items[i] = 100 + i
        end
        stub_display.categoryItems = category_items

        _G.__behavior_item_rack_paging_stub_display = stub_display
        return
        "#
    ))
    .expect("seeding stub_display skeleton must run cleanly");
}

fn seed_stub_rack_on_display(env: &WowLuaEnv, max_cards_per_page: i64) {
    env.eval::<()>(&format!(
        r#"
        local stub_rack = {{}}
        stub_rack.SetItems = function(_self, items)
            _G.__behavior_item_rack_paging_set_items_calls =
                _G.__behavior_item_rack_paging_set_items_calls + 1
            local copy = {{}}
            for i = 1, #items do copy[i] = items[i] end
            _G.__behavior_item_rack_paging_captured_items = copy
        end
        stub_rack.Refresh = function(_self)
            _G.__behavior_item_rack_paging_refresh_calls =
                _G.__behavior_item_rack_paging_refresh_calls + 1
        end
        stub_rack.GetMaxCards = function(_self) return {max_cards_per_page} end
        _G.__behavior_item_rack_paging_stub_display.currentItemRack = stub_rack
        return
        "#
    ))
    .expect("seeding stub_rack on stub_display must run cleanly");
}

fn seed_stub_footer_on_display(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        local function noop_setenabled(_self, _enabled) end
        local function noop_settext(_self, _text) end
        _G.__behavior_item_rack_paging_stub_display.Footer = {
            PrevPageButton = { SetEnabled = noop_setenabled },
            NextPageButton = { SetEnabled = noop_setenabled },
            PageText = { SetText = noop_settext },
        }
        return
        "#,
    )
    .expect("seeding stub Footer on stub_display must run cleanly");
}

fn finalize_stub_display_metatable(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        setmetatable(
            _G.__behavior_item_rack_paging_stub_display,
            { __index = AccountStoreItemDisplayMixin }
        )
        return
        "#,
    )
    .expect("setting __index metatable on stub_display must run cleanly");
}

fn teardown_stub_display(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_item_rack_paging_stub_display = nil
        return
        "#,
    )
    .expect("stub_display tear-down must run cleanly");
}
