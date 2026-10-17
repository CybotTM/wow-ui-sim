//! Temporary Classic product rewards, loot history, tracking and engraving defaults.
//! No native-default or populated claims.
//! Retire each endpoint when its corresponding supplied backing system exists.
use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

const ENGRAVING_AVAILABILITY_LUA: &str = r#"
-- Missing system: engraving feature availability. Retire when c_api owns that
-- availability state. EngravingInfoDocumentation.lua:179-185 declares
-- IsEngravingEnabled() -> value:bool (non-nil). False is our temporary policy.
if rawget(_G, "C_Engraving") == nil then C_Engraving = {} end
if rawget(C_Engraving, "IsEngravingEnabled") == nil then
  function C_Engraving.IsEngravingEnabled() return false end
end
"#;

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let product = ensure_namespace(state, "C_ProductChoice")?;
    table_set_rust_fn_static(state, product, "GetChoices", empty_choices)?;
    let loot = ensure_namespace(state, "C_LootHistory")?;
    table_set_rust_fn_static(state, loot, "GetNumItems", empty_loot_count)?;
    LuaApiMut::register_function(lua, "GetTrackingTexture", no_tracking_texture)?;
    // register_progression_state_probes admits only Era/Anniversary here.
    lua.exec(ENGRAVING_AVAILABILITY_LUA)?;
    Ok(())
}

fn no_tracking_texture(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn empty_loot_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn empty_choices(state: &mut LuaState) -> LuaResult<u32> {
    let choices = create_table(state);
    state.push(choices);
    Ok(1)
}
