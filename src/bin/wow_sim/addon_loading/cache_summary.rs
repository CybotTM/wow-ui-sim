use wow_ui_sim::loader::LoadTiming;

pub(super) fn print_cache_stats(t: &LoadTiming) {
    if t.cache_hits == 0 && t.cache_misses == 0 {
        return;
    }
    println!("Bytecode cache: {}", format_cache_summary(t));
}

pub(super) fn format_cache_info(t: &LoadTiming) -> String {
    if t.cache_hits == 0 && t.cache_misses == 0 {
        return String::new();
    }
    format!(", bytecode cache: {}", format_cache_summary(t))
}

fn format_cache_summary(t: &LoadTiming) -> String {
    let total = t.cache_hits + t.cache_misses;
    let pct = 100.0 * t.cache_hits as f64 / total as f64;
    format!(
        "{}/{} hits ({:.0}%, lookup_miss={} replay_fail={} stored={} store_fail={})",
        t.cache_hits,
        total,
        pct,
        t.cache_lookup_misses,
        t.cache_replay_failures,
        t.cache_store_successes,
        t.cache_store_failures
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_info_includes_miss_breakdown() {
        let timing = LoadTiming {
            cache_hits: 3,
            cache_misses: 7,
            cache_lookup_misses: 5,
            cache_replay_failures: 2,
            cache_store_successes: 4,
            cache_store_failures: 1,
            ..Default::default()
        };

        assert_eq!(
            format_cache_info(&timing),
            ", bytecode cache: 3/10 hits (30%, lookup_miss=5 replay_fail=2 stored=4 store_fail=1)"
        );
    }
}
