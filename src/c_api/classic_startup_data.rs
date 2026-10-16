//! Finite Vanilla startup data, grounded in cached Blizzard API documentation.

use crate::lua_api::globals::strings::string_data::game_enums::ITEM_QUALITY_COLORS_DATA;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register(lua: &mut rilua::Lua) -> crate::Result<()> {
    // ItemQualitiesDocumentation.lua:6-21 (0..8, nine values);
    // QuestConstantsDocumentation.lua:154-158 (ClassicPreWrath limit five).
    lua.exec(
        "LE_ITEM_QUALITY_POOR = Enum.ItemQuality.Poor\n\
         NUM_LE_ITEM_QUALITYS = Enum.ItemQualityMeta.NumValues\n\
         Constants.QuestWatchConsts_ClassicPreWrath = { MAX_QUEST_WATCHES = 5 }",
    )?;
    let state = lua.state_mut();
    let namespace = super::helpers::ensure_namespace(state, "C_Item")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetItemQualityColor",
        get_item_quality_color,
    )?;
    Ok(())
}

fn get_item_quality_color(state: &mut LuaState) -> LuaResult<u32> {
    let quality = i32::from_stack(state, 1)?;
    let Some(&(_, r, g, b, _)) = ITEM_QUALITY_COLORS_DATA.iter().find(|row| row.0 == quality)
    else {
        return Ok(0);
    };
    state.push(Val::Num(r));
    state.push(Val::Num(g));
    state.push(Val::Num(b));
    Ok(3)
}
