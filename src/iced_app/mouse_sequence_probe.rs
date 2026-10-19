//! Staged input adapter for external integration tests; not a Lua shim.
//! App boot/cache construction follows the existing named-click probe.
use iced::{Point, Size};

use super::app::{INIT_DEBUG, INIT_ENV, INIT_EXEC_LUA, INIT_SAVED_VARS};
use super::frame_collect::collect_hittable_frames;
use super::hit_grid::HitGrid;
use super::strata_emit::build_hittable_rects;
use super::{App, DebugOptions};
use crate::lua_api::WowLuaEnv;

pub enum MouseSequenceStep<'a> {
    Move(Point),
    Down(Point),
    Up(Point),
    Check(fn(&WowLuaEnv)),
    Lua(&'a str),
    Tick(f64),
}

pub fn run_headless_mouse_sequence(env: WowLuaEnv, setup: &str, steps: &[MouseSequenceStep<'_>]) {
    INIT_ENV.with(|cell| *cell.borrow_mut() = Some(env));
    INIT_DEBUG.with(|cell| *cell.borrow_mut() = Some(DebugOptions::default()));
    INIT_SAVED_VARS.with(|cell| *cell.borrow_mut() = None);
    INIT_EXEC_LUA.with(|cell| *cell.borrow_mut() = None);
    let (mut app, _) = App::boot();
    app.screen_size.set(Size::new(1024.0, 768.0));
    app.env.borrow().set_screen_size(1024.0, 768.0);
    app.env
        .borrow()
        .exec(setup)
        .expect("SETUP: original consumer fixture");
    for step in steps {
        rebuild_mouse_cache(&app);
        dispatch_step(&mut app, step);
    }
}

fn dispatch_step(app: &mut App, step: &MouseSequenceStep<'_>) {
    match step {
        MouseSequenceStep::Move(point) => app.handle_mouse_move(*point),
        MouseSequenceStep::Down(point) => app.handle_mouse_down(*point),
        MouseSequenceStep::Up(point) => app.handle_mouse_up(*point),
        MouseSequenceStep::Check(check) => check(&app.env.borrow()),
        MouseSequenceStep::Lua(source) => {
            app.env.borrow().exec(source).expect("F2 assertion");
        }
        MouseSequenceStep::Tick(elapsed) => {
            app.env
                .borrow()
                .fire_on_update(*elapsed)
                .expect("original OnUpdate");
        }
    }
}

fn rebuild_mouse_cache(app: &App) {
    let env = app.env.borrow();
    let mut state = env.state().borrow_mut();
    state.ensure_layout_rects();
    let buckets = state.get_strata_buckets().expect("visible buckets").clone();
    let collected = collect_hittable_frames(&state.widgets, &buckets);
    let hittable = build_hittable_rects(&collected, &state.widgets);
    *app.cached_hittable.borrow_mut() = Some(HitGrid::new(hittable, 1024.0, 768.0));
}
