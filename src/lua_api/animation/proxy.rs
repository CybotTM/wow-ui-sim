//! Table-backed proxy helpers for fallback animation handles.
//!
//! Animation groups and animations that do not have backing frame IDs still need
//! stable Lua objects for script callbacks and custom fields. These proxies mirror
//! the frame proxy pattern: a public Lua table with hidden userdata stored in
//! `__lud`, plus a per-handle user-value table for dynamic fields.

use super::{AnimGroupHandle, AnimHandle};
use crate::lua_api::proxy_helpers::{lookup_registered_method, proxy_userdata, wrap_fn_with_userdata};
use crate::lua_api::SimState;
use mlua::{Lua, Value};
use std::cell::RefCell;
use std::rc::Rc;

const ANIM_GROUP_CACHE_KEY: &str = "__anim_group_refs";
const ANIM_CACHE_KEY: &str = "__anim_refs";
const ANIM_PROXY_MT_KEY: &str = "__anim_proxy_mt";
const ANIM_BIND_METHOD_KEY: &str = "__anim_bind_method_helper";

pub(crate) fn group_handle_ref(
    lua: &Lua,
    group_id: u64,
    state: &Rc<RefCell<SimState>>,
) -> mlua::Result<Value> {
    ensure_proxy_support(lua)?;
    let cache = get_or_create_cache(lua, ANIM_GROUP_CACHE_KEY)?;
    let cached: Value = cache.raw_get(group_id as i64)?;
    if !cached.is_nil() {
        return Ok(cached);
    }

    let userdata = lua.create_userdata(AnimGroupHandle {
        group_id,
        state: Rc::clone(state),
    })?;
    let proxy = create_proxy(lua, userdata)?;
    cache.raw_set(group_id as i64, proxy.clone())?;
    Ok(proxy)
}

pub(crate) fn anim_handle_ref(
    lua: &Lua,
    group_id: u64,
    anim_index: usize,
    state: &Rc<RefCell<SimState>>,
) -> mlua::Result<Value> {
    ensure_proxy_support(lua)?;
    let cache = get_or_create_nested_cache(lua, ANIM_CACHE_KEY, group_id)?;
    let cache_index = (anim_index + 1) as i64;
    let cached: Value = cache.raw_get(cache_index)?;
    if !cached.is_nil() {
        return Ok(cached);
    }

    let userdata = lua.create_userdata(AnimHandle {
        group_id,
        anim_index,
        state: Rc::clone(state),
    })?;
    let proxy = create_proxy(lua, userdata)?;
    cache.raw_set(cache_index, proxy.clone())?;
    Ok(proxy)
}

pub(crate) fn clear_anim_handle_cache(lua: &Lua, group_id: u64) -> mlua::Result<()> {
    let cache = get_or_create_cache(lua, ANIM_CACHE_KEY)?;
    cache.raw_set(group_id as i64, Value::Nil)
}

fn ensure_proxy_support(lua: &Lua) -> mlua::Result<()> {
    register_bind_method_helper(lua)?;
    install_proxy_metatable(lua)
}

fn register_bind_method_helper(lua: &Lua) -> mlua::Result<()> {
    if lua
        .named_registry_value::<mlua::Function>(ANIM_BIND_METHOD_KEY)
        .is_ok()
    {
        return Ok(());
    }
    lua.set_named_registry_value(
        ANIM_BIND_METHOD_KEY,
        crate::lua_api::cfunc_wrap::create_bind_factory(lua)?,
    )
}

fn install_proxy_metatable(lua: &Lua) -> mlua::Result<()> {
    if lua
        .named_registry_value::<mlua::Table>(ANIM_PROXY_MT_KEY)
        .is_ok()
    {
        return Ok(());
    }
    let mt = create_proxy_metatable(lua)?;
    lua.set_named_registry_value(ANIM_PROXY_MT_KEY, mt)
}

fn create_proxy(lua: &Lua, userdata: mlua::AnyUserData) -> mlua::Result<Value> {
    userdata.set_user_value(lua.create_table()?)?;
    let proxy = lua.create_table()?;
    proxy.raw_set("__lud", userdata)?;
    let mt: mlua::Table = lua.named_registry_value(ANIM_PROXY_MT_KEY)?;
    proxy.set_metatable(Some(mt));
    Ok(Value::Table(proxy))
}

fn get_or_create_cache(lua: &Lua, key: &str) -> mlua::Result<mlua::Table> {
    lua.named_registry_value(key).or_else(|_| {
        let cache = lua.create_table()?;
        lua.set_named_registry_value(key, cache.clone())?;
        Ok(cache)
    })
}

fn get_or_create_nested_cache(lua: &Lua, key: &str, group_id: u64) -> mlua::Result<mlua::Table> {
    let cache = get_or_create_cache(lua, key)?;
    let group_key = group_id as i64;
    let nested: Value = cache.raw_get(group_key)?;
    if let Value::Table(table) = nested {
        return Ok(table);
    }
    let table = lua.create_table()?;
    cache.raw_set(group_key, table.clone())?;
    Ok(table)
}

fn create_proxy_metatable(lua: &Lua) -> mlua::Result<mlua::Table> {
    let mt = lua.create_table()?;
    mt.raw_set("__index", create_proxy_index(lua)?)?;
    mt.raw_set("__newindex", create_proxy_newindex(lua)?)?;
    mt.raw_set("__eq", create_proxy_eq(lua)?)?;
    mt.raw_set("__tostring", create_proxy_tostring(lua)?)?;
    Ok(mt)
}

fn create_proxy_index(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, (this, key): (mlua::Table, Value)| {
        let proxy_value = Value::Table(this);
        let Some(userdata) = proxy_userdata(&proxy_value) else {
            return Ok(Value::Nil);
        };

        if let Some(fields) = proxy_fields(&proxy_value)? {
            let field_value: Value = fields.raw_get(key.clone())?;
            if !field_value.is_nil() {
                return Ok(field_value);
            }
        }

        let registered = lookup_registered_method(&userdata, &key)?;
        if let Value::Function(function) = registered {
            return Ok(Value::Function(wrap_fn_with_userdata(
                lua, function, userdata, ANIM_BIND_METHOD_KEY,
            )?));
        }
        Ok(registered)
    })
}

fn create_proxy_newindex(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|_, (this, key, value): (mlua::Table, Value, Value)| {
        let proxy_value = Value::Table(this);
        let Some(fields) = proxy_fields(&proxy_value)? else {
            return Ok(());
        };
        fields.raw_set(key, value)?;
        Ok(())
    })
}

fn create_proxy_eq(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|_, (a, b): (Value, Value)| {
        let Some(a_ud) = proxy_userdata(&a) else {
            return Ok(false);
        };
        let Some(b_ud) = proxy_userdata(&b) else {
            return Ok(false);
        };
        Ok(same_group_handle(&a_ud, &b_ud) || same_anim_handle(&a_ud, &b_ud))
    })
}

fn create_proxy_tostring(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|_, this: mlua::Table| Ok(proxy_display_name(&Value::Table(this))))
}

fn proxy_fields(value: &Value) -> mlua::Result<Option<mlua::Table>> {
    let Some(userdata) = proxy_userdata(value) else {
        return Ok(None);
    };
    userdata.user_value::<mlua::Table>().map(Some)
}

fn same_group_handle(a_ud: &mlua::AnyUserData, b_ud: &mlua::AnyUserData) -> bool {
    a_ud.borrow::<AnimGroupHandle>()
        .ok()
        .zip(b_ud.borrow::<AnimGroupHandle>().ok())
        .is_some_and(|(a, b)| a.group_id == b.group_id)
}

fn same_anim_handle(a_ud: &mlua::AnyUserData, b_ud: &mlua::AnyUserData) -> bool {
    a_ud.borrow::<AnimHandle>()
        .ok()
        .zip(b_ud.borrow::<AnimHandle>().ok())
        .is_some_and(|(a, b)| a.group_id == b.group_id && a.anim_index == b.anim_index)
}

fn proxy_display_name(value: &Value) -> String {
    if let Some(userdata) = proxy_userdata(value) {
        if let Ok(group) = userdata.borrow::<AnimGroupHandle>() {
            return format!("AnimationGroup: 0x{:08X}", group.group_id);
        }
        if let Ok(anim) = userdata.borrow::<AnimHandle>() {
            return format!("Animation: 0x{:08X}:{}", anim.group_id, anim.anim_index);
        }
    }
    "Animation: 0x00000000".to_string()
}

#[cfg(test)]
mod tests {
    use super::{anim_handle_ref, group_handle_ref};
    use crate::lua_api::SimState;
    use crate::lua_api::animation::{AnimGroupState, AnimState, AnimationType};
    use mlua::{Lua, LuaOptions, StdLib};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn make_lua() -> Lua {
        unsafe { Lua::unsafe_new_with(StdLib::ALL, LuaOptions::default()) }
    }

    fn make_state_with_group() -> (Rc<RefCell<SimState>>, u64) {
        let state = Rc::new(RefCell::new(SimState::default()));
        let group_id = {
            let mut state_ref = state.borrow_mut();
            let group_id = state_ref.next_anim_group_id;
            state_ref.next_anim_group_id += 1;
            state_ref
                .animation_groups
                .insert(group_id, AnimGroupState::new(1));
            group_id
        };
        (state, group_id)
    }

    #[test]
    fn group_handle_proxy_is_cached_table_with_persistent_fields() {
        let lua = make_lua();
        let (state, group_id) = make_state_with_group();

        let group = group_handle_ref(&lua, group_id, &state).expect("group proxy should exist");
        let group_again =
            group_handle_ref(&lua, group_id, &state).expect("group proxy should be cached");
        lua.globals().set("group", group).unwrap();
        lua.globals().set("group_again", group_again).unwrap();

        let (group_type, same_group, sync_key): (String, bool, String) = lua
            .load(
                r#"
                group.syncKey = "shared"
                return type(group), group == group_again, group_again.syncKey
            "#,
            )
            .eval()
            .unwrap();

        assert_eq!(group_type, "table");
        assert!(same_group, "cached proxies should compare equal");
        assert_eq!(sync_key, "shared");
    }

    #[test]
    fn group_and_anim_handle_proxies_roundtrip_through_methods() {
        let lua = make_lua();
        let (state, group_id) = make_state_with_group();
        {
            let mut state_ref = state.borrow_mut();
            let group = state_ref.animation_groups.get_mut(&group_id).unwrap();
            let mut anim = AnimState::new(AnimationType::Alpha);
            anim.name = Some("FadeIn".to_string());
            group.animations.push(anim);
        }

        let group = group_handle_ref(&lua, group_id, &state).unwrap();
        let anim = anim_handle_ref(&lua, group_id, 0, &state).unwrap();
        lua.globals().set("group", group).unwrap();
        lua.globals().set("anim", anim).unwrap();

        let (anim_type, same_parent): (String, bool) = lua
            .load("return type(anim), anim:GetParent() == group")
            .eval()
            .unwrap();
        let created_type: String = lua
            .load(
                r#"
                local created = group:CreateAnimation("Alpha", "Later")
                return type(created)
            "#,
            )
            .eval()
            .unwrap();

        assert_eq!(anim_type, "table");
        assert!(
            same_parent,
            "animation parent should be the cached group proxy"
        );
        assert_eq!(created_type, "table");
    }
}
