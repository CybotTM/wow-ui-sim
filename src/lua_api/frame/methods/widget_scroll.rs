//! ScrollFrame and ScrollBox widget methods.

use super::super::handle::FrameRef;
use super::methods_hierarchy::reparent_widget;
use crate::lua_api::frame::handle::{extract_frame_id, frame_ref, get_sim_state};
use mlua::Value;

pub fn add_scrollframe_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_scrollframe_child_methods(methods);
    add_scrollframe_offset_methods(methods);
    add_scrollframe_range_methods(methods);
}

pub fn add_scrollbox_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("RegisterCallback", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("ForEachFrame", |_, _this, _cb: mlua::Function| Ok(()));
    methods.add_method("UnregisterCallback", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("CanInterpolateScroll", |_, _this, ()| Ok(false));
    methods.add_method("SetInterpolateScroll", |_, _this, _enabled: bool| Ok(()));
}

fn add_scrollframe_child_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetScrollChild", |lua, this, child: Value| {
        let id = this.0;
        let child_id = match extract_frame_id(&child) {
            Some(cid) => cid,
            None => return Err(mlua::Error::runtime("Usage: ScrollFrame:SetScrollChild(child)")),
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.scroll_child_id = Some(child_id);
        }
        reparent_widget(&mut state.widgets, child_id, Some(id));
        state.visible_on_update_cache = None;
        state.invalidate_layout(child_id);
        Ok(())
    });

    methods.add_method("GetScrollChild", |lua, this, ()| {
        let child_id = {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            state.widgets.get(this.0).and_then(|f| f.scroll_child_id)
        };
        match child_id {
            Some(cid) => frame_ref(lua, cid),
            None => Ok(Value::Nil),
        }
    });

    methods.add_method("UpdateScrollChildRect", |_, _this, ()| Ok(()));
}

fn add_scrollframe_offset_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetHorizontalScroll", |lua, this, offset: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.scroll_horizontal = offset;
        }
        Ok(())
    });

    methods.add_method("GetHorizontalScroll", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.scroll_horizontal).unwrap_or(0.0))
    });

    methods.add_method("SetVerticalScroll", |lua, this, offset: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.scroll_vertical = offset;
        }
        Ok(())
    });

    methods.add_method("GetVerticalScroll", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.scroll_vertical).unwrap_or(0.0))
    });
}

fn add_scrollframe_range_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetHorizontalScrollRange", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let frame = match state.widgets.get(this.0) {
            Some(f) => f,
            None => return Ok(0.0_f64),
        };
        let child_width = frame.scroll_child_id
            .and_then(|cid| state.widgets.get(cid))
            .map(|c| c.width as f64)
            .unwrap_or(0.0);
        Ok((child_width - frame.width as f64).max(0.0))
    });

    methods.add_method("GetVerticalScrollRange", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let frame = match state.widgets.get(this.0) {
            Some(f) => f,
            None => return Ok(0.0_f64),
        };
        let child_height = frame.scroll_child_id
            .and_then(|cid| state.widgets.get(cid))
            .map(|c| c.height as f64)
            .unwrap_or(0.0);
        Ok((child_height - frame.height as f64).max(0.0))
    });
}
