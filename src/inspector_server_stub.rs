//! Non-Linux stub for `iced_layout_inspector::server`.
//!
//! The inspector's `server` feature pulls in `peercred-ipc`, which uses
//! Linux-only socket options (`SO_PEERCRED`). On Windows and macOS we keep
//! the public types the simulator references but replace the actual server
//! logic with a no-op `init()` that returns a dead receiver. The match arms
//! and field accesses in `iced_app/update_servers.rs`/`update.rs`/`app.rs`
//! continue to compile and behave correctly (no commands ever arrive).

use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum Command {
    Dump {
        respond: oneshot::Sender<String>,
    },
    Input {
        field: String,
        value: String,
        respond: oneshot::Sender<Result<(), String>>,
    },
    Click {
        label: String,
        respond: oneshot::Sender<Result<(), String>>,
    },
    Submit {
        respond: oneshot::Sender<Result<(), String>>,
    },
    Key {
        key: String,
        respond: oneshot::Sender<Result<(), String>>,
    },
    Screenshot {
        respond: oneshot::Sender<Result<ScreenshotData, String>>,
    },
}

#[derive(Debug)]
pub struct ScreenshotData {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

pub fn init() -> (mpsc::Receiver<Command>, ()) {
    let (_tx, rx) = mpsc::channel::<Command>(1);
    (rx, ())
}
