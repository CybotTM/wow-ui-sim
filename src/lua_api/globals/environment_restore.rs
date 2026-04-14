//! Post-cleanup global restoration hooks.

pub fn restore_post_cleanup_globals<T>(_lua: &T) -> crate::Result<()> {
    Ok(())
}
