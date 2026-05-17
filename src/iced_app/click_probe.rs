//! Headless probes for the GUI mouse dispatch path.

use std::time::Instant;

use iced::Size;

use super::app::{INIT_DEBUG, INIT_ENV, INIT_EXEC_LUA, INIT_SAVED_VARS};
use super::frame_collect::collect_hittable_frames;
use super::hit_grid::HitGrid;
use super::state::CanvasMessage;
use super::strata_emit::build_hittable_rects;
use super::{App, DebugOptions, Message};
use crate::lua_api::WowLuaEnv;
use crate::render::texture::UI_SCALE;
use crate::saved_variables::SavedVariablesManager;

#[derive(Debug, Clone, Copy)]
pub struct NamedClick<'a> {
    pub frame_name: &'a str,
}

pub fn run_headless_named_click_probe(
    env: WowLuaEnv,
    saved_vars: Option<SavedVariablesManager>,
    screen_size: Size,
    setup_lua: &str,
    clicks: &[NamedClick<'_>],
) -> Result<(), String> {
    install_boot_params(env, saved_vars);
    let (mut app, _) = App::boot();
    app.screen_size.set(screen_size);
    app.env
        .borrow()
        .set_screen_size(screen_size.width, screen_size.height);

    let _ = app.update(Message::ProcessTimers(Instant::now()));
    app.env
        .borrow()
        .exec(setup_lua)
        .map_err(|error| format!("setup Lua failed: {error}"))?;
    let _ = app.update(Message::ProcessTimers(Instant::now()));

    for click in clicks {
        eprintln!("[headless-click-probe] clicking {}", click.frame_name);
        click_named_frame(&mut app, screen_size, click.frame_name)?;
    }

    Ok(())
}

fn install_boot_params(env: WowLuaEnv, saved_vars: Option<SavedVariablesManager>) {
    INIT_ENV.with(|cell| *cell.borrow_mut() = Some(env));
    INIT_DEBUG.with(|cell| *cell.borrow_mut() = Some(DebugOptions::default()));
    INIT_SAVED_VARS.with(|cell| *cell.borrow_mut() = saved_vars);
    INIT_EXEC_LUA.with(|cell| *cell.borrow_mut() = None);
}

fn click_named_frame(app: &mut App, screen_size: Size, frame_name: &str) -> Result<(), String> {
    rebuild_hittable_cache(app, screen_size);
    let point = named_frame_center(app, frame_name)?;
    let _ = app.update(Message::CanvasEvent(CanvasMessage::MouseMove(point)));
    rebuild_hittable_cache(app, screen_size);
    let _ = app.update(Message::CanvasEvent(CanvasMessage::MouseDown(point)));
    rebuild_hittable_cache(app, screen_size);
    let _ = app.update(Message::CanvasEvent(CanvasMessage::MouseUp(point)));
    Ok(())
}

fn named_frame_center(app: &App, frame_name: &str) -> Result<iced::Point, String> {
    let env = app.env.borrow();
    let state = env.state().borrow();
    let frame_id = state
        .widgets
        .iter_ids()
        .filter(|id| {
            state
                .widgets
                .get(*id)
                .is_some_and(|frame| frame.name.as_deref() == Some(frame_name))
        })
        .max_by_key(|id| {
            let frame = state.widgets.get(*id).expect("filtered frame should exist");
            (frame.visible, frame.layout_rect.is_some(), *id)
        })
        .ok_or_else(|| format!("frame not found: {frame_name}"))?;
    let frame = state
        .widgets
        .get(frame_id)
        .expect("selected frame should exist");
    let rect = frame
        .layout_rect
        .ok_or_else(|| format!("frame has no layout rect: {frame_name}"))?;
    Ok(iced::Point::new(
        (rect.x + rect.width / 2.0) * UI_SCALE,
        (rect.y + rect.height / 2.0) * UI_SCALE,
    ))
}

fn rebuild_hittable_cache(app: &App, screen_size: Size) {
    let env = app.env.borrow();
    let mut state = env.state().borrow_mut();
    state.ensure_layout_rects();
    let strata_buckets = state
        .get_strata_buckets()
        .expect("visible strata buckets should exist")
        .clone();
    let collected = collect_hittable_frames(&state.widgets, &strata_buckets);
    let hittable = build_hittable_rects(&collected, &state.widgets);
    let grid = HitGrid::new(hittable, screen_size.width, screen_size.height);
    *app.cached_hittable.borrow_mut() = Some(grid);
}
