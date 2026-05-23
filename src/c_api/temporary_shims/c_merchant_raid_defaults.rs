//! Temporary defaults for unmodeled merchant and raid-lock state.
//!
//! The simulator has no merchant inventory or raid instance lock model yet.
//! These no-state defaults keep startup probes callable until those systems
//! exist.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table_with_fields;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_merchant_and_raid_defaults(state: &mut LuaState) -> LuaResult<()> {
    register_merchant_defaults(state)?;
    register_raid_lock_defaults(state)
}

fn register_merchant_defaults(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_MerchantFrame")?;
    table_set_rust_fn_static(state, namespace, "GetItemInfo", get_merchant_item_info)
}

fn register_raid_lock_defaults(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_RaidLocks")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "IsEncounterComplete",
        is_encounter_complete,
    )?;
    table_set_rust_fn_static(state, namespace, "RequestRaidInfo", request_raid_info)
}

fn get_merchant_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let empty_name = state.gc.intern_string_static(b"");
    let item_info = create_table_with_fields(
        state,
        &[
            ("name", Val::Str(empty_name)),
            ("texture", Val::Nil),
            ("price", Val::Num(0.0)),
            ("stackCount", Val::Num(1.0)),
            ("numAvailable", Val::Num(-1.0)),
            ("isPurchasable", Val::Bool(false)),
            ("isUsable", Val::Bool(false)),
            ("extendedCost", Val::Bool(false)),
            ("currencyID", Val::Nil),
            ("spellID", Val::Nil),
        ],
    );
    state.push(item_info);
    Ok(1)
}

fn is_encounter_complete(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn request_raid_info(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_merchant_and_raid_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if type(C_MerchantFrame.GetItemInfo) ~= "function" then return "merchant_method" end
                local item = C_MerchantFrame.GetItemInfo(1)
                if type(item) ~= "table" then return "merchant_item" end
                if item.price ~= 0 or item.stackCount ~= 1 or item.numAvailable ~= -1 then return "merchant_shape" end
                if item.isPurchasable ~= false or item.isUsable ~= false or item.extendedCost ~= false then return "merchant_flags" end
                if type(C_RaidLocks.IsEncounterComplete) ~= "function" then return "raid_method" end
                if C_RaidLocks.IsEncounterComplete(1) ~= false then return "raid_complete" end
                if C_RaidLocks.RequestRaidInfo() ~= nil then return "raid_request" end
                return "ok"
                "#,
            )
            .expect("merchant/raid default probe should run");

        assert_eq!(result, "ok");
    }
}
