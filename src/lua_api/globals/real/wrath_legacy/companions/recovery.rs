//! Accepted laptop req-13: own/category/GCD selection in supplied history order.
//! Concrete native history contents/order and conditional rune recovery are not supplied.
use crate::lua_api::globals::real::wrath_legacy_state::CompanionRecovery;
use crate::lua_api::state::SimState;

#[derive(Clone, Copy)]
struct RecoverySelection {
    latest: u32,
    start: u32,
    duration: u32,
    enable: bool,
}

impl RecoverySelection {
    // Req-13: signed wrapping endpoint comparison; equality overwrites. Enable is
    // sticky only for accepted held own/category candidates, never rejected ones.
    fn accept(&mut self, end: u32, start: u32, duration: u32, held: bool) {
        if (end.wrapping_sub(self.latest) as i32) < 0 {
            return;
        }
        self.latest = end;
        self.start = start;
        self.duration = duration;
        self.enable &= !held;
    }
}

pub(super) fn companion_cooldown_times(sim: &SimState, spell_id: u32, now: u32) -> (f64, f64, f64) {
    if crate::spell_lookup::get_spell(spell_id).is_none()
        || sim
            .wrath_legacy
            .cooldown_excluded_spells
            .contains(&spell_id)
    {
        return (0.0, 0.0, 1.0);
    }
    let mut selected = RecoverySelection {
        latest: now,
        start: 0,
        duration: 0,
        enable: true,
    };
    // Vec order is explicit host Head/Next order; no HashMap iteration or invented
    // own/category class priority across nodes. Within each: own, category, GCD.
    for history in &sim.wrath_legacy.companion_recovery_history {
        select_own(&mut selected, history, spell_id, now);
        select_category(&mut selected, history, sim, spell_id, now);
        select_start_recovery(&mut selected, history, sim, spell_id);
    }
    (
        f64::from(selected.start) / 1000.0,
        f64::from(selected.duration) / 1000.0,
        if selected.enable { 1.0 } else { 0.0 },
    )
}

fn select_own(
    selected: &mut RecoverySelection,
    history: &CompanionRecovery,
    spell_id: u32,
    now: u32,
) {
    if history.spell_id != spell_id || history.item_id != 0 {
        return;
    }
    if history.recovery_ms == 0 && !history.on_hold {
        return;
    }
    let comparison_start = if history.on_hold {
        now.wrapping_add(1)
    } else {
        history.time_ms
    };
    let (start, duration) = if history.on_hold {
        (now, 1)
    } else {
        (history.time_ms, history.recovery_ms)
    };
    selected.accept(
        comparison_start.wrapping_add(history.recovery_ms),
        start,
        duration,
        history.on_hold,
    );
}

fn select_category(
    selected: &mut RecoverySelection,
    history: &CompanionRecovery,
    sim: &SimState,
    spell_id: u32,
    now: u32,
) {
    const SHARED_CATEGORY_FLAG: u32 = 0x2;
    let same = history.category != 0
        && sim.spell_cooldown_categories.get(&spell_id) == Some(&history.category);
    let shared = sim
        .wrath_legacy
        .spell_category_flags
        .get(&history.category)
        .is_some_and(|flags| flags & SHARED_CATEGORY_FLAG != 0);
    if history.category_recovery_ms == 0 || !(same || shared) {
        return;
    }
    let start = if history.on_hold {
        now
    } else {
        history.category_time_ms
    };
    selected.accept(
        start.wrapping_add(history.category_recovery_ms),
        start,
        history.category_recovery_ms,
        history.on_hold,
    );
}

fn select_start_recovery(
    selected: &mut RecoverySelection,
    history: &CompanionRecovery,
    sim: &SimState,
    spell_id: u32,
) {
    let Some(category) = sim
        .wrath_legacy
        .spell_start_recovery_categories
        .get(&spell_id)
    else {
        return;
    };
    if *category != history.start_recovery_category || history.start_recovery_ms == 0 {
        return;
    }
    selected.accept(
        history.time_ms.wrapping_add(history.start_recovery_ms),
        history.time_ms,
        history.start_recovery_ms,
        false,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn idle(now: u32) -> RecoverySelection {
        RecoverySelection {
            latest: now,
            start: 0,
            duration: 0,
            enable: true,
        }
    }

    #[test]
    fn signed_endpoint_accepts_wrap_and_current_time_equality() {
        let mut selected = idle(u32::MAX - 10);
        selected.accept(5, u32::MAX - 4, 10, false);
        assert_eq!(
            (selected.start, selected.duration, selected.enable),
            (u32::MAX - 4, 10, true)
        );
        selected.accept(u32::MAX, 2, 3, true);
        assert_eq!(
            (selected.start, selected.duration, selected.enable),
            (u32::MAX - 4, 10, true)
        );
        let mut selected = idle(100);
        selected.accept(100, 90, 10, false);
        assert_eq!((selected.start, selected.duration), (90, 10));
    }

    #[test]
    fn held_own_compares_recovery_plus_one_but_emits_one_millisecond() {
        let mut selected = idle(100_000);
        let history = CompanionRecovery {
            spell_id: 458,
            on_hold: true,
            recovery_ms: 300_000,
            ..Default::default()
        };
        select_own(&mut selected, &history, 458, 100_000);
        assert_eq!(
            (selected.start, selected.duration, selected.enable),
            (100_000, 1, false)
        );
        selected.accept(400_000, 90_000, 310_000, false);
        assert_eq!(
            (selected.start, selected.duration, selected.enable),
            (100_000, 1, false)
        );
        selected.accept(400_001, 90_001, 310_000, false);
        assert_eq!(
            (selected.start, selected.duration, selected.enable),
            (90_001, 310_000, false)
        );
    }

    #[test]
    fn equal_end_accepted_hold_sticks_through_equal_end_gcd() {
        let mut selected = idle(100_000);
        selected.accept(400_000, 80_000, 320_000, false);
        selected.accept(400_000, 100_000, 300_000, true);
        selected.accept(400_000, 90_000, 310_000, false);
        assert_eq!(
            (selected.start, selected.duration, selected.enable),
            (90_000, 310_000, false)
        );
    }
}
