use super::{LuaCommand, client, handle_connection};
use crate::iced_app::{App, AppInit, Message};
use crate::lua_api::WowLuaEnv;
use crate::render::{GlyphAtlas, WowFontSystem};
use crate::screen::ScreenKind;
use crate::texture::TextureManager;
use std::cell::RefCell;
use std::net::Shutdown;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const IO_TIMEOUT: Duration = Duration::from_secs(5);

struct CloseConnection(UnixStream);

impl Drop for CloseConnection {
    fn drop(&mut self) {
        // Unblock both workers even when the app loop or an assertion panics.
        self.0.shutdown(Shutdown::Both).unwrap();
    }
}

fn build_app(lua_rx: mpsc::Receiver<LuaCommand>) -> App {
    let env = Rc::new(RefCell::new(WowLuaEnv::new().unwrap()));
    env.borrow().set_screen_mode(ScreenKind::Game);
    env.borrow().set_screen_size(800.0, 600.0);
    let (_cmd_tx, cmd_rx) = tokio::sync::mpsc::channel(1);
    let app = App::build_app(AppInit {
        env,
        log_messages: Vec::new(),
        texture_manager: Rc::new(RefCell::new(TextureManager::new())),
        font_system: Rc::new(RefCell::new(WowFontSystem::new())),
        glyph_atlas: Rc::new(RefCell::new(GlyphAtlas::new())),
        cmd_rx,
        lua_rx,
        debug_borders: false,
        debug_anchors: false,
        saved_vars: None,
        config: crate::config::SimConfig::default(),
    });
    app.gui_startup_complete.set(true);
    app
}

fn accept_connection(listener: &UnixListener) -> UnixStream {
    let deadline = Instant::now() + IO_TIMEOUT;
    loop {
        match listener.accept() {
            Ok((stream, _)) => return stream,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                assert!(Instant::now() < deadline, "Exec client did not connect");
                thread::sleep(Duration::from_millis(1));
            }
            Err(error) => panic!("accept Exec client: {error}"),
        }
    }
}

fn exchange(
    app: &mut App,
    listener: &UnixListener,
    socket_path: &Path,
    cmd_tx: &mpsc::Sender<LuaCommand>,
    code: &str,
) -> String {
    thread::scope(|scope| {
        let client = scope.spawn(|| client::exec(socket_path, code));
        let stream = accept_connection(listener);
        stream.set_read_timeout(Some(IO_TIMEOUT)).unwrap();
        stream.set_write_timeout(Some(IO_TIMEOUT)).unwrap();
        let _close = CloseConnection(stream.try_clone().unwrap());
        let server = scope.spawn(move || handle_connection(stream, cmd_tx));
        let deadline = Instant::now() + IO_TIMEOUT;
        while !client.is_finished() {
            assert!(Instant::now() < deadline, "Exec response timed out");
            let _ = app.update(Message::ProcessTimers(Instant::now()));
            thread::sleep(Duration::from_millis(1));
        }
        let output = client.join().unwrap().unwrap();
        server.join().unwrap().unwrap();
        output
    })
}

#[test]
fn exec_reload_ui_replaces_environment_before_the_next_connection() {
    let directory = tempfile::tempdir().unwrap();
    let socket_path = directory.path().join("reload.sock");
    let listener = UnixListener::bind(&socket_path).unwrap();
    listener.set_nonblocking(true).unwrap();
    let (cmd_tx, cmd_rx) = mpsc::channel();
    let mut app = build_app(cmd_rx);
    let old_lua = Rc::downgrade(&app.env.borrow().lua);

    let response = exchange(
        &mut app,
        &listener,
        &socket_path,
        &cmd_tx,
        "reload_sentinel = true; print('RELOAD_REQUEST'); ReloadUI(); print('CALL_RETURNED')",
    );
    assert_eq!(response, "RELOAD_REQUEST\nCALL_RETURNED");
    let response = exchange(
        &mut app,
        &listener,
        &socket_path,
        &cmd_tx,
        "assert(reload_sentinel == nil, 'no fresh Lua environment after ReloadUI'); print('FRESH_ENV')",
    );
    assert_eq!(response, "FRESH_ENV");
    assert!(old_lua.upgrade().is_none(), "IPC must release the old VM");
}

#[test]
fn exec_print_output_is_captured_once_per_call_across_connections() {
    let directory = tempfile::tempdir().unwrap();
    let socket_path = directory.path().join("exec.sock");
    let listener = UnixListener::bind(&socket_path).unwrap();
    listener.set_nonblocking(true).unwrap();
    let (cmd_tx, cmd_rx) = mpsc::channel();
    let mut app = build_app(cmd_rx);

    let first = exchange(
        &mut app,
        &listener,
        &socket_path,
        &cmd_tx,
        "ipc_exec_count = (ipc_exec_count or 0) + 1; print('one')",
    );
    assert_eq!(
        app.env
            .borrow()
            .eval::<i32>("return ipc_exec_count")
            .unwrap(),
        1
    );
    assert_eq!(first, "one");

    let second = exchange(
        &mut app,
        &listener,
        &socket_path,
        &cmd_tx,
        "assert(ipc_exec_count == 1); ipc_second_seen = true",
    );
    assert_eq!(second, "");
    assert!(
        app.env
            .borrow()
            .eval::<bool>("return ipc_second_seen")
            .unwrap()
    );

    let third = exchange(&mut app, &listener, &socket_path, &cmd_tx, "print('two')");
    assert_eq!(third, "two");

    let repeated = exchange(
        &mut app,
        &listener,
        &socket_path,
        &cmd_tx,
        "print('same'); print('same')",
    );
    assert_eq!(repeated, "same\nsame");
}
