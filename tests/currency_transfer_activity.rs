//! Supplied transfer activity, separate from transfer request/completion behavior.
#![cfg(feature = "retail-12-0-0")]

use std::fs;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::paths::default_blizzard_ui_addons_path;

fn query_activity(env: &WowLuaEnv) -> bool {
    env.eval("return C_CurrencyInfo.IsCurrencyTransferInProgress()")
        .expect("query actual currency transfer activity")
}

#[test]
fn currency_transfer_activity_defaults_to_exactly_one_false_boolean() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local value = C_CurrencyInfo.IsCurrencyTransferInProgress()
        assert(type(value) == 'boolean', 'transfer activity must be a boolean')
        assert(value == false, 'unsupplied transfer activity is false')
        assert(select('#', C_CurrencyInfo.IsCurrencyTransferInProgress()) == 1)
        "#,
    )
    .expect("actual query returns one nonnil false boolean");
}

#[test]
fn currency_transfer_activity_is_live_nonmutating_and_environment_local() {
    let first = WowLuaEnv::new().unwrap();
    let second = WowLuaEnv::new().unwrap();
    first
        .exec("savedTransferQuery = C_CurrencyInfo.IsCurrencyTransferInProgress")
        .unwrap();
    for supplied in [false, true, false] {
        first.state().borrow_mut().currency_transfer_in_progress = supplied;
        assert_eq!(query_activity(&first), supplied);
        let retained: bool = first.eval("return savedTransferQuery()").unwrap();
        assert_eq!(retained, supplied);
        assert_eq!(
            first.state().borrow().currency_transfer_in_progress,
            supplied
        );
        assert!(!query_activity(&second));
    }
    assert!(!query_activity(&WowLuaEnv::new().unwrap()));
}

fn load_immediate_spinner_consumer(env: &WowLuaEnv) {
    let cache = default_blizzard_ui_addons_path().expect("resolve active cache");
    let path = cache.join("Blizzard_TokenUI/Blizzard_CurrencyTransfer.lua");
    let source = fs::read_to_string(&path).expect("read actual cached transfer consumer");
    let start = source
        .find("function CurrencyTransferMenuMixin:CheckPlayTransferringSpinner()")
        .expect("immediate spinner consumer missing");
    let tail = &source[start..];
    let end = tail
        .find("\nfunction CurrencyTransferMenuMixin:OnCurrencyTransferRequested")
        .expect("immediate spinner consumer boundary missing");
    env.exec("CurrencyTransferMenuMixin = {}")
        .expect("create mixin declaration, not API replacement");
    env.exec_named(
        &tail[..end],
        "@cached/Blizzard_CurrencyTransfer.lua:immediate-spinner",
    )
    .expect("load two unchanged immediate spinner methods");
}

#[test]
fn currency_transfer_activity_drives_actual_bounded_immediate_spinner() {
    let env = WowLuaEnv::new().unwrap();
    load_immediate_spinner_consumer(&env);
    env.exec(
        r#"
        menu = CreateFrame('Frame')
        Mixin(menu, CurrencyTransferMenuMixin)
        menu.TransferringSpinner = CreateFrame('Frame', nil, menu)
        menu.Content = CreateFrame('Frame', nil, menu)
        "#,
    )
    .expect("create real frames without transfer lifecycle scripts");
    for (supplied, shown) in [(false, true), (true, true), (true, false), (false, true)] {
        env.state().borrow_mut().currency_transfer_in_progress = supplied;
        let spinner_shown = supplied && shown;
        let content_shown = !spinner_shown;
        env.exec(&format!(
            r#"
            menu:SetShown({shown})
            menu:CheckPlayTransferringSpinner()
            assert(menu.TransferringSpinner:IsShown() == {spinner_shown}, 'spinner state')
            assert(menu.Content:IsShown() == {content_shown}, 'content state')
            "#,
        ))
        .expect("actual cached immediate spinner reads actual query");
    }
}
