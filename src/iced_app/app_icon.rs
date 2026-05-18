//! Window icon for the iced desktop app. Defers pixel rendering to the
//! always-compiled `app_icon_render` module so the same artwork backs both the
//! runtime window icon and the build-time `installer/wow-sim.ico` generator.

use iced::{Size, window};

use crate::app_icon_render::{SIZE, render_icon};

pub(super) fn settings() -> window::Settings {
    window::Settings {
        size: initial_window_size(),
        icon: window::icon::from_rgba(render_icon(), SIZE, SIZE).ok(),
        ..window::Settings::default()
    }
}

pub(super) fn initial_window_size() -> Size {
    Size::new(1024.0, 768.0)
}
