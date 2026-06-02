//! `C_ChatBubbles` probe surface backed by `SimState.chat_bubbles`.
//!
//! `C_ChatBubbles.GetAllChatBubbles(includeForbidden?)` returns an array of
//! lightweight tables representing world chat bubbles, one per entry in
//! `SimState.chat_bubbles`. Each table carries `message`, `sender`, and
//! `chatType`. Since the simulator does not render 3D speech bubbles there is
//! no real Frame backing them; `frame_id` is included as `frameID` when present.

use crate::c_api::helpers::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_chat_bubbles_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_ChatBubbles")?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAllChatBubbles",
        c_chat_bubbles_get_all,
    )?;
    Ok(())
}

fn c_chat_bubbles_get_all(state: &mut LuaState) -> LuaResult<u32> {
    let bubbles = borrow_state(state)?.chat_bubbles.clone();
    let array = create_table(state);
    for (index, bubble) in bubbles.into_iter().enumerate() {
        let t = create_table(state);
        let message = create_string(state, &bubble.message);
        let sender = create_string(state, &bubble.sender);
        let chat_type = create_string(state, &bubble.chat_type);
        table_set(state, t, "message", message);
        table_set(state, t, "sender", sender);
        table_set(state, t, "chatType", chat_type);
        if let Some(fid) = bubble.frame_id {
            table_set(state, t, "frameID", Val::Num(fid as f64));
        }
        set_table_array(state, array, index as i64 + 1, t);
    }
    state.push(array);
    Ok(1)
}
