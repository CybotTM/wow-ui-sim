//! Lua execution server for wow-sim.
//!
//! Provides a Unix socket server that accepts Lua code and returns results.

use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::{OnceLock, mpsc};
use std::thread;

/// Global socket path for signal handler cleanup.
static SOCKET_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Request sent to the Lua server.
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// Execute Lua code
    Exec { code: String },
    /// Ping to check if server is alive
    Ping,
    /// Dump the frame tree
    DumpTree {
        /// Filter by name (substring match)
        filter: Option<String>,
        /// Only show visible frames
        visible_only: bool,
        /// Show verbose texture detail lines
        verbose: bool,
    },
    /// Render a screenshot to a file
    Screenshot {
        /// Output file path
        output: String,
        /// Image width in pixels
        width: u32,
        /// Image height in pixels
        height: u32,
        /// Render only this frame subtree (name substring match)
        filter: Option<String>,
        /// Crop the output image to WxH+X+Y (e.g., 700x150+400+650)
        crop: Option<String>,
    },
}

/// Response from the Lua server.
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    /// Execution result (captured print output)
    Output(String),
    /// Error message
    Error(String),
    /// Pong response
    Pong,
    /// Frame tree dump
    Tree(String),
}

/// Command sent to the app from the Lua server.
pub enum LuaCommand {
    Exec {
        code: String,
        respond: mpsc::Sender<Response>,
    },
    DumpTree {
        filter: Option<String>,
        visible_only: bool,
        verbose: bool,
        respond: mpsc::Sender<Response>,
    },
    Screenshot {
        output: String,
        width: u32,
        height: u32,
        filter: Option<String>,
        crop: Option<String>,
        respond: mpsc::Sender<Response>,
    },
}

/// Get the socket path for Lua REPL.
pub fn socket_path() -> PathBuf {
    PathBuf::from(format!("/tmp/wow-lua-{}.sock", std::process::id()))
}

/// Initialize the Lua server.
/// Returns a receiver for commands. The socket is cleaned up on drop.
pub fn init() -> mpsc::Receiver<LuaCommand> {
    // Clean up stale sockets from dead processes
    cleanup_stale_sockets();

    let (tx, rx) = mpsc::channel();
    let path = socket_path();

    // Clean up our own stale socket if it exists
    let _ = std::fs::remove_file(&path);

    // Store path and register signal handlers for cleanup on SIGTERM/SIGINT
    SOCKET_PATH.set(path.clone()).ok();
    register_signal_handlers();

    thread::spawn(move || {
        run_server(tx, path);
    });

    rx
}

/// Clean up stale sockets from dead processes.
fn cleanup_stale_sockets() {
    let pattern = "/tmp/wow-lua-*.sock";
    if let Ok(entries) = glob::glob(pattern) {
        for entry in entries.flatten() {
            // Extract PID from filename: /tmp/wow-lua-{pid}.sock
            if let Some(filename) = entry.file_name().and_then(|f| f.to_str())
                && let Some(pid_str) = filename
                    .strip_prefix("wow-lua-")
                    .and_then(|s| s.strip_suffix(".sock"))
                && let Ok(pid) = pid_str.parse::<i32>()
            {
                // Check if process is still alive using kill(pid, 0)
                let exists = unsafe { libc::kill(pid, 0) } == 0;
                if !exists && std::fs::remove_file(&entry).is_ok() {
                    crate::logging::eprintln_elapsed(&format!(
                        "[wow-sim] Cleaned up stale socket: {}",
                        entry.display()
                    ));
                }
            }
        }
    }
}

extern "C" fn signal_handler(sig: libc::c_int) {
    if let Some(path) = SOCKET_PATH.get() {
        let _ = std::fs::remove_file(path);
    }
    unsafe {
        libc::signal(sig, libc::SIG_DFL);
        libc::raise(sig);
    }
}

fn register_signal_handlers() {
    unsafe {
        libc::signal(
            libc::SIGTERM,
            signal_handler as *const () as libc::sighandler_t,
        );
        libc::signal(
            libc::SIGINT,
            signal_handler as *const () as libc::sighandler_t,
        );
    }
}

fn run_server(cmd_tx: mpsc::Sender<LuaCommand>, path: PathBuf) {
    let listener = match UnixListener::bind(&path) {
        Ok(l) => {
            crate::logging::eprintln_elapsed(&format!("[wow-sim] Listening on {}", path.display()));
            l
        }
        Err(e) => {
            crate::logging::eprintln_elapsed(&format!("[wow-sim] Failed to bind: {}", e));
            return;
        }
    };

    // Clean up socket on exit
    struct SocketGuard(PathBuf);
    impl Drop for SocketGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
    let _guard = SocketGuard(path);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream, &cmd_tx) {
                    crate::logging::eprintln_elapsed(&format!("[wow-sim] Connection error: {}", e));
                }
            }
            Err(e) => {
                crate::logging::eprintln_elapsed(&format!("[wow-sim] Accept error: {}", e));
            }
        }
    }
}

/// Send a command and wait for a response with timeout.
fn send_command(
    cmd_tx: &mpsc::Sender<LuaCommand>,
    build: impl FnOnce(mpsc::Sender<Response>) -> LuaCommand,
) -> Response {
    let (resp_tx, resp_rx) = mpsc::channel();
    if cmd_tx.send(build(resp_tx)).is_err() {
        return Response::Error("App closed".into());
    }
    match resp_rx.recv_timeout(std::time::Duration::from_secs(30)) {
        Ok(r) => r,
        Err(_) => Response::Error("Timeout".into()),
    }
}

fn handle_connection(
    mut stream: UnixStream,
    cmd_tx: &mpsc::Sender<LuaCommand>,
) -> std::io::Result<()> {
    let reader = BufReader::new(stream.try_clone()?);

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let request = match parse_request(&line) {
            Ok(request) => request,
            Err(response) => {
                write_response(&mut stream, &response)?;
                continue;
            }
        };

        let response = handle_request(request, cmd_tx);
        write_response(&mut stream, &response)?;
    }

    Ok(())
}

fn parse_request(line: &str) -> Result<Request, Response> {
    serde_json::from_str(line)
        .map_err(|error| Response::Error(format!("Invalid request: {}", error)))
}

fn handle_request(request: Request, cmd_tx: &mpsc::Sender<LuaCommand>) -> Response {
    match request {
        Request::Ping => Response::Pong,
        Request::Exec { code } => {
            send_command(cmd_tx, |respond| LuaCommand::Exec { code, respond })
        }
        Request::DumpTree {
            filter,
            visible_only,
            verbose,
        } => send_command(cmd_tx, |respond| LuaCommand::DumpTree {
            filter,
            visible_only,
            verbose,
            respond,
        }),
        Request::Screenshot {
            output,
            width,
            height,
            filter,
            crop,
        } => send_command(cmd_tx, |respond| LuaCommand::Screenshot {
            output,
            width,
            height,
            filter,
            crop,
            respond,
        }),
    }
}

fn write_response(stream: &mut UnixStream, response: &Response) -> std::io::Result<()> {
    writeln!(stream, "{}", serde_json::to_string(response).unwrap())
}

/// Client module for connecting to the Lua server.
pub mod client {
    use super::*;
    use std::io::{BufRead, BufReader, Write};
    use std::os::unix::net::UnixStream;
    use std::path::Path;

    /// Connect to a Lua server and execute code.
    pub fn exec<P: AsRef<Path>>(socket: P, code: &str) -> Result<String, String> {
        let mut stream =
            UnixStream::connect(socket).map_err(|e| format!("Connect failed: {}", e))?;

        let request = Request::Exec {
            code: code.to_string(),
        };
        writeln!(stream, "{}", serde_json::to_string(&request).unwrap())
            .map_err(|e| format!("Write failed: {}", e))?;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("Read failed: {}", e))?;

        let response: Response =
            serde_json::from_str(&line).map_err(|e| format!("Invalid response: {}", e))?;

        match response {
            Response::Output(s) => Ok(s),
            Response::Error(e) => Err(e),
            Response::Pong => Err("Unexpected pong".into()),
            Response::Tree(_) => Err("Unexpected tree".into()),
        }
    }

    /// Ping the server.
    pub fn ping<P: AsRef<Path>>(socket: P) -> Result<(), String> {
        let mut stream =
            UnixStream::connect(socket).map_err(|e| format!("Connect failed: {}", e))?;

        let request = Request::Ping;
        writeln!(stream, "{}", serde_json::to_string(&request).unwrap())
            .map_err(|e| format!("Write failed: {}", e))?;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("Read failed: {}", e))?;

        let response: Response =
            serde_json::from_str(&line).map_err(|e| format!("Invalid response: {}", e))?;

        match response {
            Response::Pong => Ok(()),
            Response::Error(e) => Err(e),
            _ => Err("Unexpected response".into()),
        }
    }

    /// Find running wow-lua servers.
    pub fn find_servers() -> Vec<PathBuf> {
        glob::glob("/tmp/wow-lua-*.sock")
            .map(|paths| paths.filter_map(Result::ok).collect())
            .unwrap_or_default()
    }

    /// Take a screenshot (rendered by the server, saved to output path).
    pub fn screenshot<P: AsRef<Path>>(
        socket: P,
        output: &str,
        width: u32,
        height: u32,
        filter: Option<String>,
        crop: Option<String>,
    ) -> Result<String, String> {
        let mut stream =
            UnixStream::connect(socket).map_err(|e| format!("Connect failed: {}", e))?;

        let request = Request::Screenshot {
            output: output.to_string(),
            width,
            height,
            filter,
            crop,
        };
        writeln!(stream, "{}", serde_json::to_string(&request).unwrap())
            .map_err(|e| format!("Write failed: {}", e))?;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("Read failed: {}", e))?;

        let response: Response =
            serde_json::from_str(&line).map_err(|e| format!("Invalid response: {}", e))?;

        match response {
            Response::Output(s) => Ok(s),
            Response::Error(e) => Err(e),
            _ => Err("Unexpected response".into()),
        }
    }

    /// Dump the frame tree.
    pub fn dump_tree<P: AsRef<Path>>(
        socket: P,
        filter: Option<String>,
        visible_only: bool,
        verbose: bool,
    ) -> Result<String, String> {
        let mut stream =
            UnixStream::connect(socket).map_err(|e| format!("Connect failed: {}", e))?;

        let request = Request::DumpTree {
            filter,
            visible_only,
            verbose,
        };
        writeln!(stream, "{}", serde_json::to_string(&request).unwrap())
            .map_err(|e| format!("Write failed: {}", e))?;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("Read failed: {}", e))?;

        let response: Response =
            serde_json::from_str(&line).map_err(|e| format!("Invalid response: {}", e))?;

        match response {
            Response::Tree(s) => Ok(s),
            Response::Error(e) => Err(e),
            _ => Err("Unexpected response".into()),
        }
    }
}
