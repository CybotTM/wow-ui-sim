use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_get_inbox_num_items(lua, Rc::clone(&state))?;
    register_get_inbox_header_info(lua, Rc::clone(&state))?;
    register_get_inbox_item(lua, state)?;
    Ok(())
}

fn register_get_inbox_num_items(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "GetInboxNumItems",
        lua.create_function(move |_, ()| {
            let st = state.borrow();
            let count = st.player.inbox.len() as i32;
            Ok((count, count))
        })?,
    )
}

fn register_get_inbox_header_info(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "GetInboxHeaderInfo",
        lua.create_function(move |lua, index: i32| {
            let st = state.borrow();
            let i = (index - 1) as usize;
            let Some(mail) = st.player.inbox.get(i) else {
                return Ok(mlua::MultiValue::new());
            };
            let has_item = !mail.items.is_empty();
            let package_icon = if has_item {
                Value::String(lua.create_string("Interface\\Icons\\INV_Misc_QuestionMark")?)
            } else {
                Value::Nil
            };
            Ok(mlua::MultiValue::from_vec(vec![
                package_icon,
                Value::Integer(mail.stationery_icon as i64),
                Value::String(lua.create_string(&mail.sender)?),
                Value::String(lua.create_string(&mail.subject)?),
                Value::Integer(mail.money as i64),
                Value::Integer(mail.cod_amount as i64),
                Value::Number(mail.days_left as f64),
                Value::Integer(mail.items.len() as i64),
                Value::Boolean(mail.was_read),
                Value::Boolean(mail.was_returned),
                Value::Boolean(!mail.body.is_empty()),
                Value::Boolean(mail.can_reply),
                Value::Boolean(mail.is_gm),
            ]))
        })?,
    )
}

fn register_get_inbox_item(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "GetInboxItem",
        lua.create_function(move |lua, (index, slot): (i32, i32)| {
            let st = state.borrow();
            let mail_idx = (index - 1) as usize;
            let slot_idx = (slot - 1) as usize;
            let Some(mail) = st.player.inbox.get(mail_idx) else {
                return Ok(mlua::MultiValue::new());
            };
            let Some(attach) = mail.items.get(slot_idx) else {
                return Ok(mlua::MultiValue::new());
            };
            let item = crate::items::get_item(attach.item_id);
            let name = item.map_or("Unknown", |i| i.name);
            let texture = item.map_or(0, |i| i.icon_file_data_id);
            let quality = item.map_or(attach.quality, |i| i.quality as i32);
            // (name, itemID, texture, count, quality, canUse, isCurrency)
            Ok(mlua::MultiValue::from_vec(vec![
                Value::String(lua.create_string(name)?),
                Value::Integer(attach.item_id as i64),
                Value::Integer(texture as i64),
                Value::Integer(attach.count as i64),
                Value::Integer(quality as i64),
                Value::Boolean(true),
                Value::Boolean(false),
            ]))
        })?,
    )
}
