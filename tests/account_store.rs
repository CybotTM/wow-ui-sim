use wow_ui_sim::lua_api::WowLuaEnv;

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
