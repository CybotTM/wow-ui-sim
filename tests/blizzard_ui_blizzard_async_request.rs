//! Wrapper binary for Blizzard_AsyncRequest tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_async_request/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_async_request/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_async_request/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_async_request/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_async_request/behavior_create_requires_callbacks.rs"]
mod behavior_create_requires_callbacks;

#[path = "blizzard_ui/blizzard_async_request/behavior_create_timeout_pair_required.rs"]
mod behavior_create_timeout_pair_required;

#[path = "blizzard_ui/blizzard_async_request/behavior_start_calls_request_function.rs"]
mod behavior_start_calls_request_function;

#[path = "blizzard_ui/blizzard_async_request/behavior_start_registers_response_event.rs"]
mod behavior_start_registers_response_event;

#[path = "blizzard_ui/blizzard_async_request/behavior_start_is_idempotent_when_running.rs"]
mod behavior_start_is_idempotent_when_running;

#[path = "blizzard_ui/blizzard_async_request/behavior_response_event_fires_callback.rs"]
mod behavior_response_event_fires_callback;

#[path = "blizzard_ui/blizzard_async_request/behavior_response_event_other_name_ignored.rs"]
mod behavior_response_event_other_name_ignored;

#[path = "blizzard_ui/blizzard_async_request/behavior_timeout_fires_callback.rs"]
mod behavior_timeout_fires_callback;

#[path = "blizzard_ui/blizzard_async_request/behavior_response_before_timeout_cancels_timer.rs"]
mod behavior_response_before_timeout_cancels_timer;

#[path = "blizzard_ui/blizzard_async_request/behavior_stop_without_start_is_safe.rs"]
mod behavior_stop_without_start_is_safe;
