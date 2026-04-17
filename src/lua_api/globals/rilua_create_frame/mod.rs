//! rilua RustFn equivalents for global functions from create_frame.rs,
//! global_frames.rs, and dropdown_api.rs.
//!
//! Each public function is a `rilua::RustFn` compatible signature:
//!   `fn foo(state: &mut LuaState) -> LuaResult<u32>`
//! Args start at index 1 (no self).
//!
//! `register_all` registers all globals on a rilua Lua state.

mod dropdown_api;
mod helpers;
mod template_chain;

use crate::lua_api::rilua_methods::{borrow_state, extract_frame_id, frame_ref};
use crate::lua_bridge::FromStack;
use crate::widget::WidgetType;
use helpers::set_global_raw;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

// ---------------------------------------------------------------------------
// CreateFrame
// ---------------------------------------------------------------------------

pub fn create_frame(state: &mut LuaState) -> LuaResult<u32> {
    let args = parse_create_frame_args(state)?;
    let (parent_id, parent_explicit) = resolve_parent_id(state, args.parent_val)?;
    let frame_id = crate::lua_api::globals::create_frame::create_frame_instance(
        state,
        args.widget_type,
        &args.frame_type,
        args.name,
        (parent_id != 0).then_some(parent_id),
        parent_explicit,
        args.id,
    )?;
    let fire_on_load = borrow_state(state)?.suppress_runtime_on_load_depth == 0;
    template_chain::apply_runtime_template_chain(
        state,
        frame_id,
        args.inherits.as_deref(),
        fire_on_load,
    )?;
    let frame_val = frame_ref(state, frame_id)?;
    state.push(frame_val);
    Ok(1)
}
struct CreateFrameArgs {
    frame_type: String,
    widget_type: WidgetType,
    name: Option<String>,
    parent_val: Val,
    inherits: Option<String>,
    id: Option<i32>,
}

fn parse_create_frame_args(state: &mut LuaState) -> LuaResult<CreateFrameArgs> {
    let frame_type: String = FromStack::from_stack(state, 1)?;
    let name: Option<String> = FromStack::from_stack(state, 2)?;
    let parent_val: Val = FromStack::from_stack(state, 3)?;
    let inherits: Option<String> = FromStack::from_stack(state, 4)?;
    let id: Option<f64> = FromStack::from_stack(state, 5)?;
    let widget_type = WidgetType::from_str(&frame_type)
        .ok_or_else(|| rilua::runtime_error(format!("unknown frame type '{frame_type}'")))?;
    Ok(CreateFrameArgs {
        frame_type,
        widget_type,
        name,
        parent_val,
        inherits,
        id: id.map(|n| n as i32),
    })
}

fn resolve_parent_id(state: &mut LuaState, parent_val: Val) -> LuaResult<(u64, bool)> {
    let parent_explicit = !matches!(parent_val, Val::Nil);
    let parent_id = if parent_explicit {
        extract_frame_id(state, parent_val)
            .ok_or_else(|| rilua::runtime_error("CreateFrame parent must be a frame or nil"))?
    } else {
        let sim = borrow_state(state)?;
        sim.widgets.get_id_by_name("UIParent").unwrap_or_default()
    };
    Ok((parent_id, parent_explicit))
}

// ---------------------------------------------------------------------------
// Global frames registration
// ---------------------------------------------------------------------------

pub fn register_global_frames(lua: &mut rilua::Lua) -> LuaResult<()> {
    let state = lua.state_mut();
    let named_frames = {
        let sim = borrow_state(state)?;
        sim.widgets
            .named_frames()
            .map(|(id, name)| (id, name.clone()))
            .collect::<Vec<_>>()
    };
    for (id, name) in named_frames {
        let frame_val = frame_ref(state, id)?;
        set_global_raw(state, &name, frame_val);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// register_all
// ---------------------------------------------------------------------------

/// Register all globals from create_frame.rs, global_frames.rs, and dropdown_api.rs
/// onto the rilua Lua state.
pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "CreateFrame", create_frame)?;
    register_global_frames(lua)?;
    dropdown_api::register_dropdown_constants(lua)?;
    dropdown_api::register_dropdown_mutators(lua)?;
    dropdown_api::register_dropdown_selections(lua)?;
    dropdown_api::register_dropdown_queries(lua)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn create_frame_registers_named_global_and_parent() {
        let env = WowLuaEnv::new().expect("env");

        env.exec(
            r#"
            local child = CreateFrame("Frame", "RiluaCreateFrameChild", UIParent)
            assert(child ~= nil, "CreateFrame should return a frame")
            assert(type(child) == "table", "CreateFrame should expose frames as tables")
            assert(RiluaCreateFrameChild == child, "named frame should be global")
            assert(child:GetParent() == UIParent, "parent should be assigned")
        "#,
        )
        .expect("CreateFrame should create a named child frame");

        let parent_name: Option<String> = env
            .eval("local p = RiluaCreateFrameChild:GetParent(); return p and p:GetName()")
            .expect("eval parent name");
        assert_eq!(parent_name.as_deref(), Some("UIParent"));
    }
}
