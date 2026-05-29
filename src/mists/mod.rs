//! Mists (5.4 / MoP Classic) client-profile compatibility layer.
//!
//! This entire module is only compiled when `--features client-mists` is
//! active. Mists is post-Cataclysm so it shares more of its API surface with
//! retail than wrath does — in practice this module is a much thinner shim
//! than `src/wrath/`.
//!
//! Initial scope is just `compat_bootstrap.lua`, stubbing the 46 globals that
//! show up as nil-call errors in the mists baseline. Frame methods are NOT
//! registered (mists handles backdrop via templates, not direct methods).

pub mod character_frame_preload;
pub mod compat_bootstrap;
pub mod post_load;
