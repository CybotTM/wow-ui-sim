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
