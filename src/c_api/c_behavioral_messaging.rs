use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_behavioral_messaging_surface(state: &mut LuaState) -> LuaResult<()> {
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
