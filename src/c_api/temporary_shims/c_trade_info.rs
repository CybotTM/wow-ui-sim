//! C_TradeInfo temporary warning-policy shim.
//!
//! Trade offer risk state is not modeled yet, so expose the inert startup
//! default from the C API temporary-shim boundary instead of Lua bootstrap.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_trade_info_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_TradeInfo")?;
    table_set_rust_fn_static(
        state,
        ns,
        "ShouldShowTradeOfferWarning",
        should_show_trade_offer_warning,
    )?;
    table_set_rust_fn_static(state, ns, "PickupTradeMoney", pickup_trade_money)
}

fn should_show_trade_offer_warning(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn pickup_trade_money(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn trade_offer_warning_defaults_false() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let (should_warn, pickup_returns): (bool, i32) = env
            .eval(
                r##"
                return C_TradeInfo.ShouldShowTradeOfferWarning(),
                    select("#", C_TradeInfo.PickupTradeMoney(1000))
                "##,
            )
            .expect("trade warning shim should be callable");

        assert!(!should_warn);
        assert_eq!(pickup_returns, 0);
    }
}
