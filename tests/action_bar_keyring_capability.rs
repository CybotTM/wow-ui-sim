//! Host capability projection, not native keyring eligibility rules.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn ptr_keyring_capability_follows_supplied_state() {
    let env = WowLuaEnv::new().unwrap();
    if ACTIVE == ClientProfile::Retail {
        env.exec("assert(rawget(C_ActionBar, 'ShouldShowKeyring') == nil)")
            .expect("PTR-only addition must not expand Retail publication");
        return;
    }
    assert_eq!(ACTIVE, ClientProfile::Ptr);
    for enabled in [false, true, false] {
        env.state().borrow_mut().action_bar_state.keyring_ui_enabled = enabled;
        env.exec(&format!(
            "assert(select('#', C_ActionBar.ShouldShowKeyring()) == 1); \
             assert(type(C_ActionBar.ShouldShowKeyring()) == 'boolean'); \
             assert(C_ActionBar.ShouldShowKeyring() == {enabled})"
        ))
        .expect("one non-nil boolean from the explicit host capability");
    }
}
