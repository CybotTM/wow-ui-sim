//! Era / Anniversary client-profile compatibility layer.
//!
//! Compiled when either `client-era` or `client-anniversary` is active. Both
//! profiles serve vanilla content; the only difference between them is the
//! source-repo build SHA, so they share `compat_bootstrap.lua` here rather
//! than maintain two near-identical files in `src/anniversary/`.

pub mod compat_bootstrap;
