//! Who query routing shared by the legacy and C_FriendList adapters.
//! Explicit response input, not a server, query parser, or native timing model.
#[derive(Debug, Default)]
pub struct WhoRouting {
    pub to_ui: bool,
    pub last_query: Option<String>,
    pub pending: Vec<String>,
    pub ui_results: Vec<String>,
}

pub(crate) fn send_who(state: &mut rilua::vm::state::LuaState) -> rilua::LuaResult<u32> {
    use crate::lua_api::methods::borrow_state_mut;
    use crate::lua_bridge::FromStack;
    let query = String::from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    sim.who_routing.last_query = Some(query);
    let response = sim.who_routing.pending.clone();
    if sim.who_routing.to_ui {
        sim.who_routing.ui_results = response;
        sim.events.push_simple("WHO_LIST_UPDATE");
    } else {
        sim.system_chat_log.extend(response);
    }
    Ok(0)
}
