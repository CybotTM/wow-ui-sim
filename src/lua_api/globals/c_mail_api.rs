use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_get_inbox_num_items(lua, Rc::clone(&state))?;
    register_get_inbox_header_info(lua, Rc::clone(&state))?;
    register_get_inbox_item_link(lua, Rc::clone(&state))?;
    register_get_inbox_item(lua, Rc::clone(&state))?;
    register_get_inbox_text(lua, Rc::clone(&state))?;
    register_get_inbox_invoice_info(lua, Rc::clone(&state))?;
    register_has_inbox_item(lua, Rc::clone(&state))?;
    register_inbox_item_can_delete(lua, Rc::clone(&state))?;
    register_check_inbox(lua, Rc::clone(&state))?;
    register_c_mail(lua, Rc::clone(&state))?;
    register_inbox_actions(lua, Rc::clone(&state))?;
    register_send_mail_api(lua, state)?;
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

fn register_get_inbox_item_link(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "GetInboxItemLink",
        lua.create_function(move |lua, (index, slot): (i32, i32)| {
            let st = state.borrow();
            let mail_idx = (index - 1) as usize;
            let slot_idx = (slot - 1) as usize;
            let attach = st
                .player
                .inbox
                .get(mail_idx)
                .and_then(|m| m.items.get(slot_idx));
            let Some(attach) = attach else {
                return Ok(Value::Nil);
            };
            let item = crate::items::get_item(attach.item_id);
            let name = item.map_or("Unknown", |i| i.name);
            let quality = item.map_or(1, |i| i.quality);
            let color = super::c_item_api::quality_color(quality);
            let id = attach.item_id;
            let link = format!("|c{color}|Hitem:{id}::::::::80:::::::::|h[{name}]|h|r");
            Ok(Value::String(lua.create_string(&link)?))
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

fn register_get_inbox_text(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "GetInboxText",
        lua.create_function(move |lua, index: i32| {
            let st = state.borrow();
            let i = (index - 1) as usize;
            let Some(mail) = st.player.inbox.get(i) else {
                return Ok(mlua::MultiValue::new());
            };
            let has_items = !mail.items.is_empty();
            // (bodyText, stationeryID1, stationeryID2, isTakeable, isInvoice, isConsortium)
            Ok(mlua::MultiValue::from_vec(vec![
                Value::String(lua.create_string(&mail.body)?),
                Value::Integer(0),
                Value::Integer(0),
                Value::Boolean(has_items || mail.money > 0),
                Value::Boolean(false),
                Value::Boolean(false),
            ]))
        })?,
    )
}

/// Stub — no auction house integration. Returns nil (no invoice).
fn register_get_inbox_invoice_info(lua: &Lua, _state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "GetInboxInvoiceInfo",
        lua.create_function(|_, _index: i32| Ok(mlua::MultiValue::new()))?,
    )
}

fn register_has_inbox_item(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "HasInboxItem",
        lua.create_function(move |_, (index, slot): (i32, i32)| {
            let st = state.borrow();
            let has = st
                .player
                .inbox
                .get((index - 1) as usize)
                .map_or(false, |m| m.items.get((slot - 1) as usize).is_some());
            Ok(has)
        })?,
    )
}

fn register_inbox_item_can_delete(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "InboxItemCanDelete",
        lua.create_function(move |_, index: i32| {
            let st = state.borrow();
            let can = st
                .player
                .inbox
                .get((index - 1) as usize)
                .map_or(false, |m| m.items.is_empty() && m.money == 0);
            Ok(can)
        })?,
    )
}

fn register_check_inbox(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "CheckInbox",
        lua.create_function(move |lua, ()| {
            let count = state.borrow().player.inbox.len();
            if count > 0 {
                let fire: mlua::Function = lua.globals().get("FireEvent")?;
                fire.call::<()>(("MAIL_INBOX_UPDATE",))?;
            }
            Ok(())
        })?,
    )
}

fn register_c_mail(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let g = lua.globals();
    let t: mlua::Table = match g.get::<Value>("C_Mail")? {
        Value::Table(t) => t,
        _ => {
            let t = lua.create_table()?;
            g.set("C_Mail", t.clone())?;
            t
        }
    };
    t.set(
        "CanCheckInbox",
        lua.create_function(|_, ()| Ok((true, 0)))?,
    )?;
    t.set(
        "HasInboxMoney",
        lua.create_function({
            let s = Rc::clone(&state);
            move |_, index: i32| {
                let st = s.borrow();
                let has = st
                    .player
                    .inbox
                    .get((index - 1) as usize)
                    .map_or(false, |m| m.money > 0);
                Ok(has)
            }
        })?,
    )?;
    Ok(())
}

fn register_inbox_actions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_take_inbox_item(lua, Rc::clone(&state))?;
    register_auto_loot_mail_item(lua, Rc::clone(&state))?;
    register_delete_inbox_item(lua, Rc::clone(&state))?;
    register_return_inbox_item(lua, state)?;
    Ok(())
}

fn fire_mail_event(lua: &Lua, event: &str) -> Result<()> {
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    fire.call::<()>((event,))
}

fn register_take_inbox_item(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "TakeInboxItem",
        lua.create_function(move |lua, (index, slot): (i32, i32)| {
            let mut st = state.borrow_mut();
            let i = (index - 1) as usize;
            let s = (slot - 1) as usize;
            if let Some(mail) = st.player.inbox.get_mut(i) {
                if s < mail.items.len() {
                    mail.items.remove(s);
                }
            }
            drop(st);
            fire_mail_event(lua, "MAIL_SUCCESS")
        })?,
    )
}

fn register_auto_loot_mail_item(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "AutoLootMailItem",
        lua.create_function(move |lua, index: i32| {
            let mut st = state.borrow_mut();
            let i = (index - 1) as usize;
            if let Some(mail) = st.player.inbox.get_mut(i) {
                mail.items.clear();
                mail.money = 0;
            }
            drop(st);
            fire_mail_event(lua, "MAIL_SUCCESS")
        })?,
    )
}

fn register_delete_inbox_item(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "DeleteInboxItem",
        lua.create_function(move |lua, index: i32| {
            let mut st = state.borrow_mut();
            let i = (index - 1) as usize;
            if i < st.player.inbox.len() {
                st.player.inbox.remove(i);
            }
            drop(st);
            fire_mail_event(lua, "MAIL_INBOX_UPDATE")
        })?,
    )
}

fn register_return_inbox_item(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "ReturnInboxItem",
        lua.create_function(move |lua, index: i32| {
            let mut st = state.borrow_mut();
            let i = (index - 1) as usize;
            if i < st.player.inbox.len() {
                st.player.inbox.remove(i);
            }
            drop(st);
            fire_mail_event(lua, "MAIL_INBOX_UPDATE")
        })?,
    )
}

fn register_send_mail_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let g = lua.globals();

    register_get_send_mail_item(lua, &g, Rc::clone(&state))?;
    register_has_send_mail_item(lua, &g, Rc::clone(&state))?;
    register_send_mail_money(lua, &g, Rc::clone(&state))?;
    register_send_mail_cod(lua, &g, Rc::clone(&state))?;
    register_send_mail_actions(lua, &g, state)?;
    Ok(())
}

fn register_get_send_mail_item(
    lua: &Lua, g: &mlua::Table, state: Rc<RefCell<SimState>>,
) -> Result<()> {
    g.set("GetSendMailItem", lua.create_function(move |lua, slot: i32| {
        let st = state.borrow();
        let i = (slot - 1) as usize;
        let Some(Some(attach)) = st.player.send_mail_items.get(i) else {
            return Ok(mlua::MultiValue::new());
        };
        let item = crate::items::get_item(attach.item_id);
        let name = item.map_or("Unknown", |i| i.name);
        let texture = item.map_or(0, |i| i.icon_file_data_id);
        let quality = item.map_or(attach.quality, |i| i.quality as i32);
        Ok(mlua::MultiValue::from_vec(vec![
            Value::String(lua.create_string(name)?),
            Value::Integer(attach.item_id as i64),
            Value::Integer(texture as i64),
            Value::Integer(attach.count as i64),
            Value::Integer(quality as i64),
        ]))
    })?)
}

fn register_has_send_mail_item(
    lua: &Lua, g: &mlua::Table, state: Rc<RefCell<SimState>>,
) -> Result<()> {
    g.set("HasSendMailItem", lua.create_function(move |_, slot: i32| {
        let st = state.borrow();
        let i = (slot - 1) as usize;
        Ok(st.player.send_mail_items.get(i).is_some_and(|s| s.is_some()))
    })?)
}

fn register_send_mail_money(
    lua: &Lua, g: &mlua::Table, state: Rc<RefCell<SimState>>,
) -> Result<()> {
    g.set("SetSendMailMoney", lua.create_function({
        let s = Rc::clone(&state);
        move |_, amount: i64| { s.borrow_mut().player.send_mail_money = amount as u64; Ok(()) }
    })?)?;
    g.set("GetSendMailMoney", lua.create_function(move |_, ()| {
        Ok(state.borrow().player.send_mail_money as i64)
    })?)
}

fn register_send_mail_cod(
    lua: &Lua, g: &mlua::Table, state: Rc<RefCell<SimState>>,
) -> Result<()> {
    g.set("SetSendMailCOD", lua.create_function({
        let s = Rc::clone(&state);
        move |_, amount: i64| { s.borrow_mut().player.send_mail_cod = amount as u64; Ok(()) }
    })?)?;
    g.set("GetSendMailCOD", lua.create_function(move |_, ()| {
        Ok(state.borrow().player.send_mail_cod as i64)
    })?)
}

fn register_send_mail_actions(
    lua: &Lua, g: &mlua::Table, state: Rc<RefCell<SimState>>,
) -> Result<()> {
    g.set("GetSendMailPrice", lua.create_function(|_, ()| Ok(30))?)?;
    g.set("SendMail", lua.create_function({
        let s = Rc::clone(&state);
        move |lua, (_recipient, _subject, _body): (String, String, String)| {
            clear_send_state(&s);
            fire_mail_event(lua, "MAIL_SEND_SUCCESS")
        }
    })?)?;
    g.set("ClearSendMail", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| { clear_send_state(&s); Ok(()) }
    })?)?;
    g.set("SetSendMailShowing", lua.create_function(|_, _: bool| Ok(()))?)?;
    g.set("CloseMail", lua.create_function(move |lua, ()| {
        clear_send_state(&state);
        fire_mail_event(lua, "MAIL_CLOSED")
    })?)?;
    Ok(())
}

fn clear_send_state(state: &Rc<RefCell<SimState>>) {
    let mut st = state.borrow_mut();
    st.player.send_mail_items = Default::default();
    st.player.send_mail_money = 0;
    st.player.send_mail_cod = 0;
}
