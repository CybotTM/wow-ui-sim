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
    let Ok(player) = lua.create_string("player") else { return };
    let args = &[
        mlua::Value::String(player.clone()),
        mlua::Value::Integer(cast_id as i64),
        mlua::Value::Integer(spell_id as i64),
    ];
    let _ = env.fire_event_with_args("UNIT_SPELLCAST_STOP", args);
    let _ = env.fire_event_with_args("UNIT_SPELLCAST_SUCCEEDED", args);
    let _ = crate::lua_api::globals::action_bar_api::push_action_button_state_update(
        &env.state(), env.lua(),
    );
}

/// Apply healing from a completed cast spell to the target or self.
pub(super) fn apply_heal_effect(
    state: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    env: &crate::lua_api::WowLuaEnv,
    spell_id: u32,
) {
    const HEAL_AMOUNT: i32 = 20_000;
    if !matches!(spell_id, 19750 | 82326 | 85673) {
        return;
    }
    let unit_event = compute_heal_target(state, HEAL_AMOUNT);
    if let Some(unit) = unit_event {
        let lua = env.lua();
        if let Ok(unit_str) = lua.create_string(&unit) {
            let _ = env.fire_event_with_args(
                "UNIT_HEALTH",
                &[mlua::Value::String(unit_str)],
            );
        }
    }
}

fn compute_heal_target(
    state: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    amount: i32,
) -> Option<String> {
    let mut s = state.borrow_mut();
    if let Some(ref mut t) = s.current_target {
        if !t.is_enemy {
            if t.health <= 0 { return None; }
            t.health = (t.health + amount).min(t.health_max);
            let healed = t.health;
            let unit_id = t.unit_id.clone();
            if let Some(idx) = crate::lua_api::globals::unit_api::parse_party_index(&unit_id) {
                if let Some(m) = s.party_members.get_mut(idx) {
                    m.health = healed;
                }
            }
            Some(unit_id)
        } else {
            heal_player(&mut s, amount)
        }
    } else {
        heal_player(&mut s, amount)
    }
}

fn heal_player(s: &mut crate::lua_api::SimState, amount: i32) -> Option<String> {
    if s.player_health <= 0 { return None; }
    s.player_health = (s.player_health + amount).min(s.player_health_max);
    Some("player".to_string())
}

/// If a spec change was pending, apply it and fire PLAYER_SPECIALIZATION_CHANGED.
pub(super) fn apply_spec_change(
    state: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    env: &crate::lua_api::WowLuaEnv,
) {
    let changed = {
        let mut s = state.borrow_mut();
        s.pending_spec_change.take().map(|idx| { s.active_spec_index = idx; })
    };
    if changed.is_some() {
        let lua = env.lua();
        let Ok(unit) = lua.create_string("player") else { return };
        let _ = env.fire_event_with_args("PLAYER_SPECIALIZATION_CHANGED", &[mlua::Value::String(unit)]);
    }
}
