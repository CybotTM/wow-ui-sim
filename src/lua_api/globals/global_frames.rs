//! Minimal global-frame helpers that still have live callers.

pub fn hide_runtime_hidden_frames<T>(_lua: T) -> crate::Result<()> {
    Ok(())
}
