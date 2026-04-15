//! Precompiled helper initialization.
//!
//! The old helper cache is gone on the live rilua path, but env init still
//! calls into this module. Keep only the initialization shim.

pub fn init<T>(_lua: &T) -> crate::Result<()> {
    Ok(())
}
