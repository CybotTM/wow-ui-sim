use iced::widget::shader;
use iced::{Point, Rectangle, mouse};

use super::super::Message;
use super::super::state::CanvasMessage;

/// Map a mouse event inside `bounds` to a canvas message action.
pub(super) fn handle_mouse_event(
    mouse_event: &mouse::Event,
    bounds: Rectangle,
    cursor: mouse::Cursor,
) -> Option<shader::Action<Message>> {
    match mouse_event {
        mouse::Event::CursorMoved { position } => handle_cursor_moved(*position, bounds),
        mouse::Event::CursorLeft => publish_canvas_event(CanvasMessage::MouseLeave),
        mouse::Event::ButtonPressed(mouse::Button::Left) => {
            publish_cursor_event(cursor, bounds, CanvasMessage::MouseDown)
        }
        mouse::Event::ButtonReleased(mouse::Button::Left) => {
            publish_cursor_event(cursor, bounds, CanvasMessage::MouseUp)
        }
        mouse::Event::ButtonPressed(mouse::Button::Right) => {
            publish_cursor_event(cursor, bounds, CanvasMessage::RightMouseDown)
        }
        mouse::Event::ButtonReleased(mouse::Button::Right) => {
            publish_cursor_event(cursor, bounds, CanvasMessage::RightMouseUp)
        }
        mouse::Event::ButtonPressed(mouse::Button::Middle) => {
            publish_cursor_event(cursor, bounds, CanvasMessage::MiddleClick)
        }
        mouse::Event::WheelScrolled { delta } => publish_scroll_event(delta),
        _ => None,
    }
}

fn handle_cursor_moved(position: Point, bounds: Rectangle) -> Option<shader::Action<Message>> {
    if bounds.contains(position) {
        let local = Point::new(position.x - bounds.x, position.y - bounds.y);
        publish_canvas_event(CanvasMessage::MouseMove(local))
    } else {
        publish_canvas_event(CanvasMessage::MouseLeave)
    }
}

fn publish_cursor_event(
    cursor: mouse::Cursor,
    bounds: Rectangle,
    to_message: impl Fn(Point) -> CanvasMessage,
) -> Option<shader::Action<Message>> {
    cursor
        .position_in(bounds)
        .and_then(|position| publish_canvas_event(to_message(position)))
}

fn publish_scroll_event(delta: &mouse::ScrollDelta) -> Option<shader::Action<Message>> {
    let dy = match delta {
        mouse::ScrollDelta::Lines { y, .. } => *y,
        mouse::ScrollDelta::Pixels { y, .. } => *y / 30.0,
    };
    Some(shader::Action::publish(Message::Scroll(0.0, dy)))
}

fn publish_canvas_event(message: CanvasMessage) -> Option<shader::Action<Message>> {
    Some(shader::Action::publish(Message::CanvasEvent(message)))
}
