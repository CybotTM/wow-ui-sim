use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{AccountStoreCategoryInfo, AccountStoreCurrencyInfo};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn begin_purchase_records_item_and_returns_success_by_default() {
    // AccountStoreBaseCardMixin:SelectCard wires the confirmation popup's
    // OnAccept to call C_AccountStore.BeginPurchase(self.itemID). The default
    // state should report success so the popup branch resolves cleanly.
    let env = env();
    let succeeded: bool = env
        .eval(r#"return C_AccountStore.BeginPurchase(420555)"#)
        .unwrap();
    assert!(succeeded, "default state should report queued purchase");
    assert_eq!(
        env.state().borrow().last_account_store_purchase_request,
        Some(420555),
        "begin_purchase should record the requested item id on SimState"
    );
}

#[test]
fn begin_purchase_returns_false_when_state_flag_disabled() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_begin_purchase_succeeds = false;
    }
    let succeeded: bool = env
        .eval(r#"return C_AccountStore.BeginPurchase(987654)"#)
        .unwrap();
    assert!(
        !succeeded,
        "purchase should report failure when SimState flag is disabled"
    );
    assert_eq!(
        env.state().borrow().last_account_store_purchase_request,
        Some(987654),
        "the requested item id should still be recorded on a failing call"
    );
}

#[test]
fn begin_purchase_overwrites_previous_request() {
    let env = env();
    env.eval::<()>(
        r#"
        C_AccountStore.BeginPurchase(111)
        C_AccountStore.BeginPurchase(222)
        "#,
    )
    .unwrap();
    assert_eq!(
        env.state().borrow().last_account_store_purchase_request,
        Some(222),
        "the most recent call should be the one recorded"
    );
}

#[test]
fn refund_item_records_item_and_returns_success_by_default() {
    // AccountStoreBaseCardMixin:SelectCard wires the refund popup's OnAccept
    // to call C_AccountStore.RefundItem(self.itemID) for refundable items.
    let env = env();
    let succeeded: bool = env
        .eval(r#"return C_AccountStore.RefundItem(420555)"#)
        .unwrap();
    assert!(succeeded, "default state should report queued refund");
    assert_eq!(
        env.state().borrow().last_account_store_refund_request,
        Some(420555),
        "refund_item should record the requested item id on SimState"
    );
}

#[test]
fn refund_item_returns_false_when_state_flag_disabled() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_refund_succeeds = false;
    }
    let succeeded: bool = env
        .eval(r#"return C_AccountStore.RefundItem(987654)"#)
        .unwrap();
    assert!(
        !succeeded,
        "refund should report failure when SimState flag is disabled"
    );
    assert_eq!(
        env.state().borrow().last_account_store_refund_request,
        Some(987654),
        "the requested item id should still be recorded on a failing call"
    );
}

#[test]
fn get_category_items_returns_empty_array_for_unknown_category() {
    // AccountStoreItemDisplayMixin:OnCategorySelected and ItemRack:Refresh both
    // expect to iterate the result with ipairs, so the empty case must return
    // a table — not nil — to avoid breaking the iteration.
    let env = env();
    let count: i64 = env
        .eval(
            r#"
            local items = C_AccountStore.GetCategoryItems(9999)
            return #items
            "#,
        )
        .unwrap();
    assert_eq!(
        count, 0,
        "unknown category should yield an empty array, never nil"
    );
}

#[test]
fn get_category_items_returns_seeded_item_ids_in_order() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state
            .account_store_category_items
            .insert(7, vec![100, 200, 300]);
    }
    let (n, first, second, third): (i64, i64, i64, i64) = env
        .eval(
            r#"
            local items = C_AccountStore.GetCategoryItems(7)
            return #items, items[1], items[2], items[3]
            "#,
        )
        .unwrap();
    assert_eq!(n, 3);
    assert_eq!(first, 100);
    assert_eq!(second, 200);
    assert_eq!(third, 300);
}

#[test]
fn get_category_items_isolates_categories() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_category_items.insert(1, vec![11, 12]);
        state.account_store_category_items.insert(2, vec![21]);
    }
    let (cat1_count, cat2_count, cat3_count): (i64, i64, i64) = env
        .eval(
            r#"
            return #C_AccountStore.GetCategoryItems(1),
                   #C_AccountStore.GetCategoryItems(2),
                   #C_AccountStore.GetCategoryItems(3)
            "#,
        )
        .unwrap();
    assert_eq!(cat1_count, 2);
    assert_eq!(cat2_count, 1);
    assert_eq!(cat3_count, 0, "unseeded category should still be empty");
}

#[test]
fn get_currency_id_for_store_returns_nil_when_unseeded() {
    // Footer.CurrencyAvailable:OnEnter and AccountStoreFrame:OnStoreFrontSet
    // both branch on `currency ~= nil` to decide whether to show the tooltip.
    let env = env();
    let value: Option<i64> = env
        .eval(r#"return C_AccountStore.GetCurrencyIDForStore(42)"#)
        .unwrap();
    assert_eq!(
        value, None,
        "an unseeded storefront must report nil so the tooltip stays hidden"
    );
}

#[test]
fn get_currency_id_for_store_returns_seeded_currency() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_currency_for_store.insert(7, 1234);
    }
    let value: i64 = env
        .eval(r#"return C_AccountStore.GetCurrencyIDForStore(7)"#)
        .unwrap();
    assert_eq!(value, 1234);
}

#[test]
fn get_currency_id_for_store_isolates_storefronts() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_currency_for_store.insert(1, 100);
        state.account_store_currency_for_store.insert(2, 200);
    }
    let (a, b, missing): (Option<i64>, Option<i64>, Option<i64>) = env
        .eval(
            r#"
            return C_AccountStore.GetCurrencyIDForStore(1),
                   C_AccountStore.GetCurrencyIDForStore(2),
                   C_AccountStore.GetCurrencyIDForStore(3)
            "#,
        )
        .unwrap();
    assert_eq!(a, Some(100));
    assert_eq!(b, Some(200));
    assert_eq!(missing, None, "unseeded id stays nil even with siblings");
}

#[test]
fn get_currency_info_returns_nil_when_unseeded() {
    // AccountStoreUtil.IsCurrencyAtWarningThreshold and AddCurrencyTotalTooltip
    // both branch on `currencyInfo` being non-nil before reading any field.
    let env = env();
    let value: Option<i64> = env
        .eval(r#"return C_AccountStore.GetCurrencyInfo(99) and 1 or nil"#)
        .unwrap();
    assert_eq!(value, None, "unknown currency id should report nil");
}

#[test]
fn get_currency_info_returns_seeded_struct_with_all_fields() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_currency_info.insert(
            55,
            AccountStoreCurrencyInfo {
                id: 55,
                amount: 1200,
                max_quantity: Some(5000),
                name: "Plunder".to_string(),
                icon: 4242,
            },
        );
    }
    let (id, amount, max, name, icon): (i64, i64, i64, String, i64) = env
        .eval(
            r#"
            local info = C_AccountStore.GetCurrencyInfo(55)
            return info.id, info.amount, info.maxQuantity, info.name, info.icon
            "#,
        )
        .unwrap();
    assert_eq!(id, 55);
    assert_eq!(amount, 1200);
    assert_eq!(max, 5000);
    assert_eq!(name, "Plunder");
    assert_eq!(icon, 4242);
}

#[test]
fn get_currency_info_omits_max_quantity_when_none() {
    // AccountStoreUtil.IsCurrencyAtWarningThreshold uses
    // `if currencyInfo and currencyInfo.maxQuantity then` to gate the warning
    // branch — uncapped currencies must surface as nil, not 0.
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_currency_info.insert(
            12,
            AccountStoreCurrencyInfo {
                id: 12,
                amount: 50,
                max_quantity: None,
                name: "Soft Currency".to_string(),
                icon: 1,
            },
        );
    }
    let (max_is_nil, threshold_branch_skipped): (bool, bool) = env
        .eval(
            r#"
            local info = C_AccountStore.GetCurrencyInfo(12)
            local skipped = true
            if info and info.maxQuantity then
                skipped = false
            end
            return info.maxQuantity == nil, skipped
            "#,
        )
        .unwrap();
    assert!(max_is_nil, "uncapped currency must report nil maxQuantity");
    assert!(
        threshold_branch_skipped,
        "the maxQuantity-gated warning branch must be skipped when nil"
    );
}

#[test]
fn get_storefront_state_defaults_to_available() {
    // CharacterSelectNavBar and Blizzard_EndOfMatchUI gate the storefront
    // entry button on `state == Enum.AccountStoreState.Available` (0). An
    // unseeded test must not accidentally gate the button closed.
    let env = env();
    let (default_state, available, equal): (i64, i64, bool) = env
        .eval(
            r#"
            local s = C_AccountStore.GetStoreFrontState(123)
            return s, Enum.AccountStoreState.Available, s == Enum.AccountStoreState.Available
            "#,
        )
        .unwrap();
    assert_eq!(default_state, 0, "default state must be Available (0)");
    assert_eq!(available, 0);
    assert!(
        equal,
        "default value must compare equal to the enum constant"
    );
}

#[test]
fn get_storefront_state_returns_seeded_state() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_storefront_state.insert(7, 2);
    }
    let value: i64 = env
        .eval(r#"return C_AccountStore.GetStoreFrontState(7)"#)
        .unwrap();
    assert_eq!(value, 2, "seeded Unavailable state should be returned");
}

#[test]
fn get_storefront_state_isolates_storefronts() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_storefront_state.insert(1, 1);
        state.account_store_storefront_state.insert(2, 2);
    }
    let (a, b, c): (i64, i64, i64) = env
        .eval(
            r#"
            return C_AccountStore.GetStoreFrontState(1),
                   C_AccountStore.GetStoreFrontState(2),
                   C_AccountStore.GetStoreFrontState(3)
            "#,
        )
        .unwrap();
    assert_eq!(a, 1, "storefront 1 reports Unknown");
    assert_eq!(b, 2, "storefront 2 reports Unavailable");
    assert_eq!(c, 0, "unseeded storefront 3 falls back to Available");
}

#[test]
fn request_storefront_info_update_records_id_and_returns_no_values() {
    // AccountStoreFrame:OnStoreFrontSet and Blizzard_EndOfMatchUI fire this
    // call as the entry point of the storefront load flow; the simulator
    // records it so a test can match a later ACCOUNT_STORE_FRONT_UPDATED
    // event to the same id.
    let env = env();
    let return_count: i64 = env
        .eval(
            r##"
            local before = select("#", C_AccountStore.RequestStoreFrontInfoUpdate(77))
            return before
            "##,
        )
        .unwrap();
    assert_eq!(
        return_count, 0,
        "RequestStoreFrontInfoUpdate should return no values"
    );
    assert_eq!(
        env.state()
            .borrow()
            .last_account_store_storefront_info_request,
        Some(77),
        "the requested storefront id should be recorded on SimState"
    );
}

#[test]
fn request_storefront_info_update_overwrites_previous_request() {
    let env = env();
    env.eval::<()>(
        r#"
        C_AccountStore.RequestStoreFrontInfoUpdate(11)
        C_AccountStore.RequestStoreFrontInfoUpdate(22)
        "#,
    )
    .unwrap();
    assert_eq!(
        env.state()
            .borrow()
            .last_account_store_storefront_info_request,
        Some(22),
        "the latest request id wins"
    );
}

#[test]
fn get_category_info_returns_nil_when_unseeded() {
    // AccountStoreCategoryMixin:SetCategory and OnCategorySelected dereference
    // the result without a nil-guard once a real id is supplied. Tests that
    // never seed a category should still see a clean nil — never a sentinel
    // table — so we don't accidentally mask missing-data bugs.
    let env = env();
    let value: Option<i64> = env
        .eval(r#"return C_AccountStore.GetCategoryInfo(404) and 1 or nil"#)
        .unwrap();
    assert_eq!(value, None, "unknown category id should report nil");
}

#[test]
fn get_category_info_returns_seeded_struct_with_all_fields() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_categories.insert(
            42,
            AccountStoreCategoryInfo {
                id: 42,
                name: "Mounts".to_string(),
                category_type: 3,
                icon: 1357,
            },
        );
    }
    let (id, name, category_type, icon): (i64, String, i64, i64) = env
        .eval(
            r#"
            local info = C_AccountStore.GetCategoryInfo(42)
            return info.id, info.name, info.type, info.icon
            "#,
        )
        .unwrap();
    assert_eq!(id, 42);
    assert_eq!(name, "Mounts");
    assert_eq!(category_type, 3);
    assert_eq!(icon, 1357);
}

#[test]
fn get_category_info_isolates_categories() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_categories.insert(
            1,
            AccountStoreCategoryInfo {
                id: 1,
                name: "Boost".to_string(),
                category_type: 1,
                icon: 11,
            },
        );
        state.account_store_categories.insert(
            2,
            AccountStoreCategoryInfo {
                id: 2,
                name: "Transmog".to_string(),
                category_type: 2,
                icon: 22,
            },
        );
    }
    let (n1, n2, missing): (String, String, Option<String>) = env
        .eval(
            r#"
            local a = C_AccountStore.GetCategoryInfo(1)
            local b = C_AccountStore.GetCategoryInfo(2)
            local c = C_AccountStore.GetCategoryInfo(3)
            return a.name, b.name, c and c.name or nil
            "#,
        )
        .unwrap();
    assert_eq!(n1, "Boost");
    assert_eq!(n2, "Transmog");
    assert_eq!(missing, None, "unseeded category 3 stays nil");
}

#[test]
fn refund_item_is_independent_of_begin_purchase_state() {
    // The two SimState flags are decoupled: a refund failure must not bleed
    // into purchase reporting and vice versa.
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_store_refund_succeeds = false;
    }
    let purchase_ok: bool = env
        .eval(r#"return C_AccountStore.BeginPurchase(101)"#)
        .unwrap();
    let refund_ok: bool = env
        .eval(r#"return C_AccountStore.RefundItem(202)"#)
        .unwrap();
    assert!(
        purchase_ok,
        "purchase flag stays true even when refund flag is flipped off"
    );
    assert!(
        !refund_ok,
        "refund flag controls only the refund return value"
    );
    let state = env.state().borrow();
    assert_eq!(state.last_account_store_purchase_request, Some(101));
    assert_eq!(state.last_account_store_refund_request, Some(202));
}
