//! EditBox presentation and coalesced text notifications at dirty-state flushes.

use std::cell::RefCell;
use std::rc::Rc;

use rilua::{LuaApiMut, Val};

use super::env::{WowLuaAppData, WowLuaEnv};
use crate::render::editbox_layout::{EditBoxLayoutRequest, shape_editbox_cursor};
use crate::render::font::WowFontSystem;
use crate::widget::editbox::EditBoxLayoutInput;

type CursorGeometry = (f32, f32, f32, f32);
type FontSystem = Rc<RefCell<WowFontSystem>>;

enum LayoutRefresh {
    Unavailable,
    Unchanged,
    Changed(EditBoxLayoutInput),
}

struct EditBoxUpdate {
    cursor: Option<CursorGeometry>,
    text_changed: Option<bool>,
}

pub(crate) fn flush(env: &WowLuaEnv) -> crate::Result<()> {
    let fonts = font_system(env);
    for id in editbox_ids(env) {
        let Some(update) = refresh_editbox(env, id, fonts.as_ref()) else {
            continue;
        };
        if let Some((x, y, width, height)) = update.cursor {
            env.fire_script_handler(
                id,
                "OnCursorChanged",
                vec![
                    Val::Num(x.into()),
                    Val::Num(y.into()),
                    Val::Num(width.into()),
                    Val::Num(height.into()),
                ],
            )?;
        }
        if let Some(user_input) = update.text_changed {
            env.fire_script_handler(id, "OnTextChanged", vec![Val::Bool(user_input)])?;
        }
    }
    Ok(())
}

fn font_system(env: &WowLuaEnv) -> Option<FontSystem> {
    let mut lua = env.rilua_mut();
    lua.state_mut()
        .app_data::<WowLuaAppData>()
        .and_then(|data| data.font_system.clone())
}

fn editbox_ids(env: &WowLuaEnv) -> Vec<u64> {
    let mut sim = env.state().borrow_mut();
    sim.ensure_layout_rects();
    let mut ids = sim.widgets.iter_editbox_ids().collect::<Vec<_>>();
    ids.sort_unstable();
    ids
}

fn read_layout_refresh(env: &WowLuaEnv, id: u64) -> LayoutRefresh {
    let sim = env.state().borrow();
    let Some(frame) = sim.widgets.get(id) else {
        return LayoutRefresh::Unavailable;
    };
    let Some(rect) = frame.layout_rect else {
        return LayoutRefresh::Unavailable;
    };
    let size = (rect.width, rect.height);
    let unchanged = frame
        .editbox_presentation
        .as_ref()
        .and_then(|presentation| presentation.input.as_ref())
        .is_some_and(|input| input.matches_frame(frame, size));
    if unchanged {
        LayoutRefresh::Unchanged
    } else {
        LayoutRefresh::Changed(EditBoxLayoutInput::from_frame(frame, size))
    }
}

fn refresh_editbox(env: &WowLuaEnv, id: u64, fonts: Option<&FontSystem>) -> Option<EditBoxUpdate> {
    match read_layout_refresh(env, id) {
        LayoutRefresh::Unavailable => None,
        LayoutRefresh::Unchanged => take_text_change(env, id),
        LayoutRefresh::Changed(input) => {
            let cursor = shape_cursor(fonts, &input);
            commit_presentation(env, id, input, cursor)
        }
    }
}

fn shape_cursor(fonts: Option<&FontSystem>, input: &EditBoxLayoutInput) -> Option<CursorGeometry> {
    let settings = input.settings;
    if !settings.scale.is_finite() || settings.scale <= 0.0 {
        return None;
    }
    let (left, top, width, height) = settings.content_bounds();
    let request = EditBoxLayoutRequest {
        text: input.text.as_deref().unwrap_or_default(),
        font_path: input.font.as_deref(),
        font_size: settings.font_size,
        content_width: width,
        content_height: height,
        word_wrap: settings.word_wrap,
        line_spacing: settings.spacing,
        smooth_scaling: settings.smooth_scaling,
        justify_h: settings.justify_h,
        justify_v: settings.justify_v,
        cursor_scalar: input.cursor,
    };
    let (x, y, height) = shape_editbox_cursor(&mut fonts?.borrow_mut(), &request)?;
    Some((
        (left + x) / settings.scale,
        -(top + y) / settings.scale,
        1.5,
        height / settings.scale,
    ))
}

fn commit_presentation(
    env: &WowLuaEnv,
    id: u64,
    input: EditBoxLayoutInput,
    cursor: Option<CursorGeometry>,
) -> Option<EditBoxUpdate> {
    let mut sim = env.state().borrow_mut();
    let frame = sim.widgets.get_mut(id)?;
    let presentation = frame.editbox_presentation_mut();
    presentation.input = Some(input);
    presentation.cursor_geometry = cursor;
    let text_changed = presentation.take_text_change();
    sim.widgets.mark_visual_dirty(id);
    Some(EditBoxUpdate {
        cursor,
        text_changed,
    })
}

fn take_text_change(env: &WowLuaEnv, id: u64) -> Option<EditBoxUpdate> {
    let mut sim = env.state().borrow_mut();
    let frame = sim.widgets.get_mut(id)?;
    let text_changed = frame.editbox_presentation.as_mut()?.take_text_change();
    Some(EditBoxUpdate {
        cursor: None,
        text_changed,
    })
}
