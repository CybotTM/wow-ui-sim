/// Ensure the main thread has a 32 MB stack.
///
/// Elune's taint tracking uses extra C stack that overflows the default 8 MB.
/// winit (via iced) requires the event loop on the main thread, so we can't
/// just spawn a worker thread with a larger stack. Instead, bump RLIMIT_STACK
/// and re-exec so the kernel allocates a larger main-thread stack.
pub fn ensure_large_stack() {
    use std::os::unix::process::CommandExt;
    const DESIRED: libc::rlim_t = 32 * 1024 * 1024;
    const MARKER: &str = "__WOW_SIM_STACK_SET";
    if std::env::var_os(MARKER).is_some() {
        return;
    }
    let mut rlim = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    unsafe {
        libc::getrlimit(libc::RLIMIT_STACK, &mut rlim);
    }
    if rlim.rlim_cur >= DESIRED {
        return;
    }
    rlim.rlim_cur = DESIRED;
    if rlim.rlim_max < DESIRED {
        rlim.rlim_max = DESIRED;
    }
    unsafe {
        libc::setrlimit(libc::RLIMIT_STACK, &rlim);
    }
    // SAFETY: called from main() before any threads are spawned.
    unsafe {
        std::env::set_var(MARKER, "1");
    }
    let err = std::process::Command::new(std::env::current_exe().unwrap())
        .args(std::env::args_os().skip(1))
        .exec();
    eprintln!("re-exec failed: {err}");
    std::process::exit(1);
}
