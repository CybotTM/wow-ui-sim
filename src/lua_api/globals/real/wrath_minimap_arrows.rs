//! 12340 miniature world-map arrow ownership; model rendering remains unsupported.
use crate::lua_api::globals::create_frame::create_frame_instance;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, extract_frame_id};
use crate::lua_bridge::stack_val;
use crate::widget::WidgetType;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

const WRONG_TYPE: &str = "CreateMiniWorldMapArrowFrame(): Wrong object type, expected frame";

pub fn register(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "CreateMiniWorldMapArrowFrame", create_mini_arrows)
}

fn create_mini_arrows(state: &mut LuaState) -> LuaResult<u32> {
    let parent = resolve_parent(state)?;
    if borrow_state(state)?.wrath_legacy.mini_map_arrows.is_some() {
        return Ok(0);
    }
    let arrow = create_arrow(state, parent, "PlayerMiniArrowFrame")?;
    let effect = create_arrow(state, parent, "PlayerMiniArrowEffectFrame")?;
    borrow_state_mut(state)?.wrath_legacy.mini_map_arrows = Some((arrow, effect));
    Ok(0)
}

fn resolve_parent(state: &LuaState) -> LuaResult<u64> {
    let value = stack_val(state, 1);
    let Val::Table(reference) = value else {
        return Err(runtime_error("Usage: CreateMiniWorldMapArrowFrame(parent)"));
    };
    let identity = state.gc.tables.get(reference).map(|table| table.get_int(0));
    if matches!(identity, None | Some(Val::Nil)) {
        return Err(runtime_error(
            "CreateMiniWorldMapArrowFrame(): Couldn't find 'this' in parent object",
        ));
    }
    let id = extract_frame_id(state, value).ok_or_else(|| runtime_error(WRONG_TYPE))?;
    let sim = borrow_state(state)?;
    let frame = sim
        .widgets
        .get(id)
        .ok_or_else(|| runtime_error(WRONG_TYPE))?;
    if matches!(
        frame.widget_type,
        WidgetType::Texture | WidgetType::FontString | WidgetType::Line
    ) {
        return Err(runtime_error(WRONG_TYPE));
    }
    Ok(id)
}

fn create_arrow(state: &mut LuaState, parent: u64, name: &str) -> LuaResult<u64> {
    let id = create_frame_instance(
        state,
        WidgetType::Model,
        "Model",
        Some(name.to_owned()),
        Some(parent),
        true,
        None,
    )?;
    borrow_state_mut(state)?
        .widgets
        .get_mut(id)
        .expect("new model frame")
        .model_state_mut()
        .model_path = Some("MinimapArrow.mdx".to_owned());
    Ok(id)
}
