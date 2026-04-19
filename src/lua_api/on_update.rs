//! rilua-backed OnUpdate bridge.

use super::state::SimState;
use std::cell::RefCell;
use std::rc::Rc;

/// Per-tick GC work budget (rilua work units; `GCSTEPSIZE` is 1024).
/// Starting value — tune once we have real measurements. Smaller values
/// spread collection across more ticks but may fail to keep up with
/// allocation churn; larger values batch work at the cost of per-tick
/// pauses.
const ON_UPDATE_GC_BUDGET: i64 = 1024;

pub(crate) fn register<T>(_lua: &T, _state: &Rc<RefCell<SimState>>) -> crate::Result<()> {
    Ok(())
}

pub(crate) fn fire(env: &super::env::WowLuaEnv, elapsed: f64) -> crate::Result<()> {
    // Pause auto-GC while the tick runs its handlers — batches all
    // collection work into a single gc_step at the end of the tick
    // instead of interleaving mid-dispatch.
    env.gc_stop();

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
        super::script_helpers::dispatch_on_update(&mut lua, &frame_ids, elapsed)?;
    }

    crate::lua_api::frame::methods::button_anchor_hierarchy::advance_animation_groups(
        env, elapsed,
    )?;

    for frame_id in &frame_ids {
        env.fire_script_handler(*frame_id, "OnPostUpdate", vec![rilua::Val::Num(elapsed)])?;
    }

    env.finalize_frame_metrics(elapsed * 1000.0);

    // Advance the collector with a bounded budget. gc_step updates the
    // threshold internally so the next tick starts with a reasonable
    // auto-GC ceiling before our gc_stop pauses it again.
    env.gc_step_with_budget(ON_UPDATE_GC_BUDGET)?;

    Ok(())
}
