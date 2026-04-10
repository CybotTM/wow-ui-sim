//! PlayerModel widget methods.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::get_sim_state;

pub fn add_player_model_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_player_model_stubs(methods);
}

fn add_player_model_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "ApplySpellVisualKit",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("CanSetUnit", |_, _this, ()| Ok(false));
    methods.add_method(
        "FreezeAnimation",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("GetDisplayInfo", |lua, this, ()| {
        Ok(read_player_model_frame(lua, this.0, |frame| {
            frame.model_appearance.display_info.unwrap_or(0) as i64
        })
        .unwrap_or(0))
    });
    methods.add_method("GetDoBlend", |_, _this, ()| Ok(false));
    methods.add_method("GetKeepModelOnHide", |_, _this, ()| Ok(false));
    methods.add_method("HasAnimation", |lua, this, ()| {
        Ok(read_player_model_frame(lua, this.0, |frame| {
            frame.model_appearance.animation_id.is_some()
        })
        .unwrap_or(false))
    });
    methods.add_method("PlayAnimKit", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method(
        "SetBarberShopAlternateForm",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("SetDoBlend", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetItem", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetItemAppearance", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
    methods.add_method("SetKeepModelOnHide", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
    methods.add_method("StopAnimKit", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("ZeroCachedCenterXY", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
}

fn read_player_model_frame<T>(
    lua: &mlua::Lua,
    id: u64,
    read: impl FnOnce(&crate::widget::Frame) -> T,
) -> Option<T> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state.widgets.get(id).map(read)
}
