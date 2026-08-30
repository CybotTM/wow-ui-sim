use std::io;

#[cfg(target_os = "linux")]
use std::fs::{File, OpenOptions};
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;

#[cfg(target_os = "linux")]
const LOCK_FILE_NAME: &str = "wow-ui-sim-test-workloads.lock";

#[derive(Clone, Copy)]
pub(crate) enum Mode {
    Shared,
    Exclusive,
}

#[cfg(target_os = "linux")]
pub(crate) struct Permit {
    _file: File,
}

#[cfg(not(target_os = "linux"))]
pub(crate) struct Permit;

#[cfg(target_os = "linux")]
pub(crate) fn acquire(mode: Mode) -> io::Result<Permit> {
    let path = std::env::temp_dir().join(LOCK_FILE_NAME);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;
    lock(&file, mode)?;
    Ok(Permit { _file: file })
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn acquire(_mode: Mode) -> io::Result<Permit> {
    Ok(Permit)
}

pub(crate) fn with_lock<T>(mode: Mode, body: impl FnOnce() -> T) -> T {
    let _permit = acquire(mode)
        .unwrap_or_else(|error| panic!("acquire {} workload gate: {error}", mode.description()));
    body()
}

impl Mode {
    #[cfg(target_os = "linux")]
    fn operation(self) -> libc::c_int {
        match self {
            Self::Shared => libc::LOCK_SH,
            Self::Exclusive => libc::LOCK_EX,
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Shared => "shared",
            Self::Exclusive => "exclusive",
        }
    }
}

#[cfg(target_os = "linux")]
fn lock(file: &File, mode: Mode) -> io::Result<()> {
    loop {
        let result = unsafe { libc::flock(file.as_raw_fd(), mode.operation()) };
        if result == 0 {
            return Ok(());
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
}
