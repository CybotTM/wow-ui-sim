//! PlayerModel widget methods.

use super::super::handle::FrameRef;

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
    methods.add_method("GetDisplayInfo", |_, _this, ()| Ok(0i64));
    methods.add_method("GetDoBlend", |_, _this, ()| Ok(false));
    methods.add_method("GetKeepModelOnHide", |_, _this, ()| Ok(false));
    methods.add_method("HasAnimation", |_, _this, ()| Ok(false));
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
