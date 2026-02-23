//! Timer processing: firing callbacks, cancellation, cleanup, and tick loop.

use super::env::WowLuaEnv;
use super::globals::timer_api::create_timer_proxy;
use super::state::PendingTimer;
use crate::Result;
use std::time::Instant;

impl WowLuaEnv {
    /// Cancel a timer by ID.
    pub fn cancel_timer(&self, timer_id: u64) {
        let mut state = self.state.borrow_mut();
        for timer in state.timers.iter_mut() {
            if timer.id == timer_id {
                timer.cancelled = true;
                break;
            }
        }
    }

    /// Remove registry keys for a finished or cancelled timer.
    fn cleanup_timer(&self, timer: PendingTimer) {
        self.lua.remove_registry_value(timer.callback_key).ok();
        if let Some(hk) = timer.handle_key {
            self.lua.remove_registry_value(hk).ok();
        }
    }

    /// Fire a single timer callback, returning true if it fired successfully.
    fn fire_timer_callback(&self, timer: &PendingTimer) -> bool {
        let Ok(callback) = self.lua.registry_value::<mlua::Function>(&timer.callback_key) else {
            return false;
        };
        let handle: Option<mlua::Table> = timer
            .handle_key
            .as_ref()
            .and_then(|k| self.lua.registry_value(k).ok());
        let result = match handle {
            Some(h) => {
                let arg = create_timer_proxy(&self.lua, &h).unwrap_or(h);
                callback.call::<()>(arg)
            }
            None => callback.call::<()>(()),
        };
        if let Err(e) = result {
            eprintln!("Timer callback error: {}", e);
        }
        true
    }

    /// Check if a ticker should repeat and decrement its remaining count.
    fn ticker_should_repeat(timer: &mut PendingTimer) -> bool {
        match &mut timer.remaining {
            Some(n) if *n > 1 => {
                *n -= 1;
                true
            }
            Some(_) => false,
            None => true,
        }
    }

    /// Process any timers that are ready to fire.
    /// Returns the number of callbacks invoked.
    pub fn process_timers(&self) -> Result<usize> {
        let now = Instant::now();
        let mut fired = 0;
        let mut to_reschedule = Vec::new();

        let mut state = self.state.borrow_mut();
        let mut i = 0;
        while i < state.timers.len() {
            if state.timers[i].cancelled {
                self.cleanup_timer(state.timers.remove(i).unwrap());
                continue;
            }

            if state.timers[i].fire_at <= now {
                let mut timer = state.timers.remove(i).unwrap();
                let timer_addon = timer.owner_addon;
                // Drop state borrow before calling Lua callback
                drop(state);

                let cb_start = Instant::now();
                if self.fire_timer_callback(&timer) {
                    let elapsed_ms = cb_start.elapsed().as_secs_f64() * 1000.0;
                    fired += 1;
                    state = self.state.borrow_mut();
                    if let Some(idx) = timer_addon {
                        if let Some(addon) = state.addons.get_mut(idx as usize) {
                            addon.runtime.current_frame_ms += elapsed_ms;
                        }
                    }

                    if let Some(interval) = timer.interval {
                        if Self::ticker_should_repeat(&mut timer) {
                            timer.fire_at = now + interval;
                            to_reschedule.push(timer);
                        } else {
                            self.cleanup_timer(timer);
                        }
                    } else {
                        self.cleanup_timer(timer);
                    }
                } else {
                    self.cleanup_timer(timer);
                    state = self.state.borrow_mut();
                }
                continue;
            }
            i += 1;
        }

        for timer in to_reschedule {
            state.timers.push_back(timer);
        }

        Ok(fired)
    }

    /// Check if there are any pending timers.
    pub fn has_pending_timers(&self) -> bool {
        !self.state.borrow().timers.is_empty()
    }
}
