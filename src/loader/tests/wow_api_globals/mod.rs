//! Tests for global WoW API functions and pre-created global frames.
//!
//! Split into focused submodules:
//! - `global_functions` — pure utility globals (locale, build info, string
//!   helpers, hooksecurefunc, Mixin, presence checks).
//! - `frames_and_attributes` — UIParent, CreateFrame, CreateTexture, attribute
//!   propagation, table.unpack.
//! - `startup_globals` — value/return shape of legacy global functions
//!   (UnitPower, GetTime, ACTIONBAR_HOTKEY_FONT_COLOR, etc.).
//! - `startup_namespaces` — `C_*` namespace surfaces created during bootstrap.
//! - `runtime_subsystems` — text helpers, scroll scripts, C_Texture/atlas,
//!   animation runtime, gamepad cursor, unit state, popup/tooltip frames.

mod account_state_flags;
mod caa_constants;
mod frames_and_attributes;
mod global_functions;
mod housing_result;
mod item_collection_secret_aspects;
mod patch_12_1_service_payloads;
mod runtime_subsystems;
mod startup_globals;
mod startup_namespaces;
