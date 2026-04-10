use super::{FrameRef, Value, extract_frame_id, frame_ref, get_sim_state};

pub(crate) fn add_tooltip_info_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsOwned", |lua, this, frame: Value| {
        let check_id = extract_frame_id(&frame);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let owned = state
            .tooltips
            .get(&this.0)
            .is_some_and(|td| td.owner_id.is_some() && td.owner_id == check_id);
        Ok(owned)
    });

    methods.add_method("GetOwner", |lua, this, ()| {
        let owner_id = {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            state.tooltips.get(&this.0).and_then(|td| td.owner_id)
        };
        match owner_id {
            Some(oid) => frame_ref(lua, oid),
            None => Ok(Value::Nil),
        }
    });

    methods.add_method("GetAnchorType", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let anchor = state
            .tooltips
            .get(&this.0)
            .map(|td| td.anchor_type.clone())
            .unwrap_or_else(|| "ANCHOR_NONE".to_string());
        Ok(anchor)
    });
}

pub(crate) fn add_tooltip_state_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("FadeOut", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state.set_frame_visible(this.0, false);
        if let Some(td) = state.tooltips.get_mut(&this.0) {
            td.owner_id = None;
        }
        Ok(())
    });
}
