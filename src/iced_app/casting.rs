//! Cast completion, healing, and spec change logic.

/// Check if a cast has completed and extract its info, clearing state.
pub(super) fn extract_completed_cast(
    state: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Option<(u32, u32)> {
    let mut s = state.borrow_mut();
    let c = s.casting.as_ref()?;
    let now = s.start_time.elapsed().as_secs_f64();
    if now < c.end_time {
        return None;
    }
    let cast_id = c.cast_id;
    let spell_id = c.spell_id;
    s.casting = None;
    Some((cast_id, spell_id))
}

/// Fire UNIT_SPELLCAST_STOP and UNIT_SPELLCAST_SUCCEEDED events.
pub(super) fn fire_cast_complete_events(
    env: &crate::lua_api::WowLuaEnv,
    cast_id: u32,
    spell_id: u32,
) {
    let lua = env.lua();
    let Ok(player) = lua.create_string("player") else {
        return;
    };
    let args = &[
        mlua::Value::String(player.clone()),
        mlua::Value::Integer(cast_id as i64),
        mlua::Value::Integer(spell_id as i64),
    ];
    let _ = env.fire_event_with_args("UNIT_SPELLCAST_STOP", args);
    let _ = env.fire_event_with_args("UNIT_SPELLCAST_SUCCEEDED", args);
    let _ = crate::lua_api::globals::action_bar_api::push_action_button_state_update(
        &env.state(),
        env.lua(),
    );
}

/// Apply spell effects (damage or healing) based on spell target type.
pub(super) fn apply_spell_effect(
    state: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    env: &crate::lua_api::WowLuaEnv,
    spell_id: u32,
) {
    if let Some(unit_id) = crate::lua_api::game_data::apply_spell_to_state(state, spell_id) {
        let lua = env.lua();
        if let Ok(unit_str) = lua.create_string(&unit_id) {
            let _ = env.fire_event_with_args("UNIT_HEALTH", &[mlua::Value::String(unit_str)]);
        }
    }
}

/// If a spec change was pending, apply it and fire PLAYER_SPECIALIZATION_CHANGED.
pub(super) fn apply_spec_change(
    state: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    env: &crate::lua_api::WowLuaEnv,
) {
    let changed = {
        let mut s = state.borrow_mut();
        s.player.pending_spec_change.take().map(|idx| {
            s.player.active_spec_index = idx;
        })
    };
    if changed.is_some() {
        let lua = env.lua();
        let Ok(unit) = lua.create_string("player") else {
            return;
        };
        let _ = env.fire_event_with_args(
            "PLAYER_SPECIALIZATION_CHANGED",
            &[mlua::Value::String(unit)],
        );
    }
}
