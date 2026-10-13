//! Independent supplied Speak For Me activity on directly documented profiles.
//! False initialization is simulator policy, not native activity or transport.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let publish = match ACTIVE {
        ClientProfile::Retail | ClientProfile::Ptr => cfg!(feature = "retail-12-0-5"),
        ClientProfile::Mists | ClientProfile::WowForever => true,
        ClientProfile::Wrath | ClientProfile::Era | ClientProfile::Anniversary => false,
    };
    if !publish {
        return Ok(());
    }
    let namespace = super::ensure_namespace(state, "C_VoiceChat")?;
    table_set_rust_fn_static(state, namespace, "IsSpeakForMeActive", query_activity)
}

fn query_activity(state: &mut LuaState) -> LuaResult<u32> {
    let active = borrow_state(state)?.voice_chat.is_speak_for_me_active;
    state.push(Val::Bool(active));
    Ok(1)
}
