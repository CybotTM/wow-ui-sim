//! Admin API: mail and premade listing management.

use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value, Variadic};
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn register_mail_admin_api(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    register_add_mail(lua, t, Rc::clone(&state))?;
    register_clear_inbox(lua, t, Rc::clone(&state))?;
    register_set_inbox_count(lua, t, state)?;
    Ok(())
}

fn register_add_mail(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "AddMail", {
        move |lua, args: Variadic<Value>| {
            let sender = str_arg(&args, 0, "Unknown");
            let subject = str_arg(&args, 1, "No Subject");
            let body = str_arg(&args, 2, "");
            let money = uint_arg(&args, 3);
            let items = parse_mail_items(lua, args.get(4))?;

            let mut st = state.borrow_mut();
            let id = st.player.next_mail_id;
            st.player.next_mail_id += 1;
            st.player.inbox.push(build_mail(id, sender, subject, body, money, items));
            Ok(())
        }
    })
}

fn register_clear_inbox(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "ClearInbox", move |_, ()| {
        state.borrow_mut().player.inbox.clear();
        Ok(())
    })
}

fn register_set_inbox_count(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    super::admin_api::set_fn(lua, t, "SetInboxCount", move |_, count: i32| {
        let mut st = state.borrow_mut();
        st.player.inbox.clear();
        for i in 0..count {
            let id = st.player.next_mail_id;
            st.player.next_mail_id += 1;
            let sender = format!("Player{}", i + 1);
            let subject = format!("Test Mail #{}", i + 1);
            let body = format!("This is test mail message {}.", i + 1);
            st.player.inbox.push(build_mail(id, sender, subject, body, 0, Vec::new()));
        }
        Ok(())
    })
}

fn build_mail(
    id: u64,
    sender: String,
    subject: String,
    body: String,
    money: u64,
    items: Vec<crate::lua_api::state_types::MailAttachment>,
) -> crate::lua_api::state_types::MailMessage {
    crate::lua_api::state_types::MailMessage {
        id,
        sender,
        subject,
        body,
        money,
        cod_amount: 0,
        items,
        days_left: 30.0,
        was_read: false,
        was_returned: false,
        can_reply: true,
        is_gm: false,
        stationery_icon: 0,
    }
}

fn str_arg(args: &[Value], index: usize, default: &str) -> String {
    match args.get(index) {
        Some(Value::String(s)) => s.to_string_lossy(),
        _ => default.to_string(),
    }
}

fn uint_arg(args: &[Value], index: usize) -> u64 {
    match args.get(index) {
        Some(Value::Integer(n)) => *n as u64,
        Some(Value::Number(n)) => *n as u64,
        _ => 0,
    }
}

/// Parse the optional items table argument for AddMail.
fn parse_mail_items(
    _lua: &Lua,
    value: Option<&Value>,
) -> Result<Vec<crate::lua_api::state_types::MailAttachment>> {
    use crate::lua_api::state_types::MailAttachment;

    let Some(Value::Table(tbl)) = value else {
        return Ok(Vec::new());
    };
    let mut items = Vec::new();
    for pair in tbl.sequence_values::<mlua::Table>() {
        let entry = pair?;
        let item_id = entry
            .get::<u32>("item_id")
            .or_else(|_| entry.get::<u32>(1))
            .unwrap_or(0);
        let count = entry
            .get::<i32>("count")
            .or_else(|_| entry.get::<i32>(2))
            .unwrap_or(1);
        if item_id > 0 {
            items.push(MailAttachment {
                item_id,
                count,
                quality: 1,
            });
        }
    }
    Ok(items)
}

pub(super) fn register_premade_admin_api(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    register_add_premade_listing(lua, t, Rc::clone(&state))?;
    register_clear_premade_listings(lua, t, Rc::clone(&state))?;
    register_update_premade_listing(lua, t, state)?;
    Ok(())
}

fn register_add_premade_listing(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    use crate::lua_api::state_types::PremadeListing;
    super::admin_api::set_fn(
        lua,
        t,
        "AddPremadeListing",
        move |_, (name, comment, activity_id, num, max): (String, String, u32, i32, i32)| {
            let mut st = state.borrow_mut();
            let id = st.world.premade_listings.len() as u32 + 1;
            st.world.premade_listings.push(PremadeListing {
                search_result_id: id,
                name,
                comment,
                leader_name: "Player".to_string(),
                activity_id,
                num_members: num,
                max_members: max,
                voice_chat: false,
                auto_accept: false,
                is_delisted: false,
            });
            Ok(id)
        },
    )
}

fn register_clear_premade_listings(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    super::admin_api::set_fn(lua, t, "ClearPremadeListings", move |_, ()| {
        state.borrow_mut().world.premade_listings.clear();
        Ok(())
    })
}

fn register_update_premade_listing(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    super::admin_api::set_fn(
        lua,
        t,
        "UpdatePremadeListing",
        move |_, (result_id, field, value): (u32, String, Value)| {
            let mut st = state.borrow_mut();
            let Some(listing) = st
                .world
                .premade_listings
                .iter_mut()
                .find(|l| l.search_result_id == result_id)
            else {
                return Ok(());
            };
            match field.as_str() {
                "numMembers" => {
                    if let Value::Integer(n) = value {
                        listing.num_members = n as i32;
                    }
                }
                "isDelisted" => {
                    if let Value::Boolean(b) = value {
                        listing.is_delisted = b;
                    }
                }
                _ => {}
            }
            Ok(())
        },
    )
}
