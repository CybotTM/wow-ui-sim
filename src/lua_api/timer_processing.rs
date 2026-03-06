//! Timer processing: firing callbacks, cancellation, cleanup, and tick loop.

use super::env::WowLuaEnv;
use super::globals::timer_api::create_fc_proxy;
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
        let result = self.invoke_timer_callback(&callback, timer);
        if let Err(e) = result {
            eprintln!("Timer callback error: {}", e);
        }
        true
    }

    /// Invoke the timer callback, passing the handle proxy if available.
    fn invoke_timer_callback(
        &self,
        callback: &mlua::Function,
        timer: &PendingTimer,
    ) -> mlua::Result<()> {
        let handle: Option<mlua::AnyUserData> = timer
            .handle_key
            .as_ref()
            .and_then(|k| self.lua.registry_value(k).ok());
        match handle {
            Some(h) => {
                let proxy = create_fc_proxy(&self.lua, &h).unwrap_or(h);
                callback.call::<()>(proxy)
            }
            None => callback.call::<()>(()),
        }
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

    /// Reschedule a repeating timer or clean it up if exhausted.
    fn reschedule_or_cleanup(&self, mut timer: PendingTimer, now: Instant, to_reschedule: &mut Vec<PendingTimer>) {
        let Some(interval) = timer.interval else {
            self.cleanup_timer(timer);
            return;
        };
        if Self::ticker_should_repeat(&mut timer) {
            timer.fire_at = now + interval;
            to_reschedule.push(timer);
        } else {
            self.cleanup_timer(timer);
        }
    }

    /// Fire one ready timer, accounting for addon timing. Returns updated fired count.
    fn fire_ready_timer(
        &self,
        timer: PendingTimer,
        now: Instant,
        fired: usize,
        to_reschedule: &mut Vec<PendingTimer>,
    ) -> usize {
        let timer_addon = timer.owner_addon;
        let cb_start = Instant::now();
        self.state.borrow_mut().executing_addon_index = timer_addon;
        if self.fire_timer_callback(&timer) {
            let elapsed_ms = cb_start.elapsed().as_secs_f64() * 1000.0;
            let mut state = self.state.borrow_mut();
            if let Some(idx) = timer_addon {
                if let Some(addon) = state.addons.get_mut(idx as usize) {
                    addon.runtime.current_frame_ms += elapsed_ms;
                }
            }
            drop(state);
            self.state.borrow_mut().executing_addon_index = None;
            self.reschedule_or_cleanup(timer, now, to_reschedule);
            fired + 1
        } else {
            self.state.borrow_mut().executing_addon_index = None;
            self.cleanup_timer(timer);
            fired
        }
    }

    /// Process any timers that are ready to fire.
    /// Returns the number of callbacks invoked.
    pub fn process_timers(&self) -> Result<usize> {
        let now = Instant::now();
        let mut fired = 0;
        let mut to_reschedule = Vec::new();
        const MAX_FIRE: usize = 1000;

        let mut i = 0;
        loop {
            let len = self.state.borrow().timers.len();
            if i >= len || fired >= MAX_FIRE {
                break;
            }
            let (cancelled, ready) = {
                let state = self.state.borrow();
                (state.timers[i].cancelled, state.timers[i].fire_at <= now)
            };
            if cancelled {
                let timer = self.state.borrow_mut().timers.remove(i).unwrap();
                self.cleanup_timer(timer);
            } else if ready {
                let timer = self.state.borrow_mut().timers.remove(i).unwrap();
                fired = self.fire_ready_timer(timer, now, fired, &mut to_reschedule);
            } else {
                i += 1;
            }
        }

        for timer in to_reschedule {
            self.state.borrow_mut().timers.push_back(timer);
        }

        Ok(fired)
    }

    /// Check if there are any pending timers.
    pub fn has_pending_timers(&self) -> bool {
        !self.state.borrow().timers.is_empty()
    }
}
