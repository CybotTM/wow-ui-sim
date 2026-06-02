pub(super) fn print_slowest_lua_files() {
    let mut timings = wow_ui_sim::loader::lua_file_timings_snapshot();
    if timings.is_empty() {
        return;
    }

    timings.sort_by(|a, b| b.total_time().cmp(&a.total_time()));
    println!("\nSlowest Lua files:");
    for timing in timings.iter().take(20) {
        println!(
            "  {:>7.1?}  {} (compile={:.1?} call={:.1?} cache_hit={} miss={} lookup_miss={} replay_fail={})",
            timing.total_time(),
            timing.chunk_name,
            timing.compile_time,
            timing.call_time,
            timing.cache_hits,
            timing.cache_misses,
            timing.cache_lookup_misses,
            timing.cache_replay_failures,
        );
    }
}
