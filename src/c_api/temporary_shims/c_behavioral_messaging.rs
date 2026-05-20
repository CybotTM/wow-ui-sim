//! C_BehavioralMessaging temporary shim — notification receipts are not modeled.
//!
//! `Blizzard_BehavioralMessaging` only needs this namespace to acknowledge
//! displayed notifications during load/runtime. Until the simulator tracks
//! behavioral-notification server state, the receipt call is a no-op.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_behavioral_messaging(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_BehavioralMessaging")?;
    table_set_rust_fn_static(
        state,
        ns,
        "SendNotificationReceipt",
        send_notification_receipt,
    )?;
    Ok(())
}

fn send_notification_receipt(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
