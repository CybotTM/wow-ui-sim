//! rilua-backed OnUpdate bridge.

use super::state::SimState;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn register<T>(_lua: &T, _state: &Rc<RefCell<SimState>>) -> crate::Result<()> {
    Ok(())
}

pub(crate) fn fire(env: &super::env::WowLuaEnv, elapsed: f64) -> crate::Result<()> {
    let frame_ids = {
        let mut sim = env.state().borrow_mut();
        if sim.visible_on_update_cache.is_none() {
            let mut visible = sim
                .on_update_frames
                .iter()
                .copied()
                .filter(|&id| sim.widgets.is_ancestor_visible(id))
                .collect::<Vec<_>>();
            visible.sort_unstable();
            sim.visible_on_update_cache = Some(visible);
        }
        sim.visible_on_update_cache.clone().unwrap_or_default()
    };

    {
        let mut lua = env.rilua_mut();
        super::rilua_script_helpers::dispatch_on_update(&mut lua, &frame_ids, elapsed)?;
    }

    for frame_id in &frame_ids {
        env.fire_script_handler(*frame_id, "OnPostUpdate", vec![rilua::Val::Num(elapsed)])?;
    }

    env.finalize_frame_metrics(elapsed * 1000.0);
    Ok(())
}
