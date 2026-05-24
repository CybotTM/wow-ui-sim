//! Temporary totem state defaults.
//!
//! Active totem slots are not modeled yet. Keep the empty `GetTotemInfo`
//! tuple explicit here until totem slot state exists.

use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    if LuaApiMut::get_global_val(lua, "GetTotemInfo") == Val::Nil {
        LuaApiMut::register_function(lua, "GetTotemInfo", get_totem_info)?;
    }
    Ok(())
}

fn get_totem_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    state.push(Val::Nil);
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Nil);
    Ok(5)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_totem_info_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local haveTotem, name, startTime, duration, icon = GetTotemInfo(1)
                if haveTotem ~= false then return "active" end
                if name ~= nil then return "name" end
                if startTime ~= 0 then return "start" end
                if duration ~= 0 then return "duration" end
                if icon ~= nil then return "icon" end
                return "ok"
                "#,
            )
            .expect("totem defaults probe should run");

        assert_eq!(result, "ok");
    }
}
