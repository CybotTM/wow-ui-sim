//! Wrath (3.3.5a) client-profile compatibility layer.
//!
//! This entire module is only compiled when `--features client-wrath` is
//! active (declared as `#[cfg(feature = "client-wrath")] pub mod wrath;` in
//! `src/lib.rs`), so retail and mists builds never see any of this code.
//!
//! Contents:
//! - `frame_methods` — Rust-side frame method stubs (`SetBackdropColor`,
//!   `SetBackdropBorderColor`, `IgnoreDepth`, `SetPlayerTextureHeight`) that
//!   wrath addons call directly on Frame; retail moved these to
//!   `BackdropTemplateMixin`.
//! - `compat_bootstrap` — loads the bundled Lua snippet that supplies global
//!   stubs and Lua-5.0-era string/math aliases wrath relies on.

pub mod compat_bootstrap;
pub mod frame_methods;
pub mod post_load;

/// Wrath/mists/era/anniversary profiles accept any non-empty event name;
/// the mainline `events.yaml` strict-list doesn't cover pre-Cataclysm,
/// MoP-Classic, or Vanilla.
pub fn is_registerable_event(name: &str) -> bool {
    !name.is_empty()
}
