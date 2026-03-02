//! A_Admin casting & cooldown simulation.

use crate::lua_api::game_data::{CastingState, SpellCooldownState};
use crate::lua_api::state::SimState;
use super::admin_api::set_fn;
use mlua::{Lua, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_casting_api(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    set_fn(lua, t, "SetCasting", {
        let s = Rc::clone(&state);
        move |_, (spell_id, spell_name, icon_path, duration): (u32, String, String, f64)| {
            let mut st = s.borrow_mut();
            let now = st.start_time.elapsed().as_secs_f64();
            let cast_id = st.next_cast_id;
            st.next_cast_id += 1;
            st.casting = Some(CastingState {
                spell_id, spell_name, icon_path,
                start_time: now, end_time: now + duration, cast_id,
            });
            Ok(())
        }
    })?;
    set_fn(lua, t, "StopCasting", {
        let s = Rc::clone(&state);
        move |_, ()| { s.borrow_mut().casting = None; Ok(()) }
    })?;
    register_cooldown_api(lua, t, state)
}

fn register_cooldown_api(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    set_fn(lua, t, "SetGCD", {
        let s = Rc::clone(&state);
        move |_, duration: f64| {
            let now = s.borrow().start_time.elapsed().as_secs_f64();
            s.borrow_mut().gcd = Some((now, duration));
            Ok(())
        }
    })?;
    set_fn(lua, t, "SetSpellCooldown", {
        let s = Rc::clone(&state);
        move |_, (spell_id, duration): (u32, f64)| {
            let now = s.borrow().start_time.elapsed().as_secs_f64();
            s.borrow_mut().spell_cooldowns.insert(
                spell_id,
                SpellCooldownState { start: now, duration },
            );
            Ok(())
        }
    })?;
    Ok(())
}
