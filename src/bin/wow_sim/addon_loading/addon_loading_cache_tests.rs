use super::*;

#[test]
fn cache_info_includes_miss_breakdown() {
    let timing = LoadTiming {
        cache_hits: 3,
        cache_misses: 7,
        cache_lookup_misses: 5,
        cache_replay_failures: 2,
        ..Default::default()
    };

    assert_eq!(
        format_cache_info(&timing),
        ", bytecode cache: 3/10 hits (30%, lookup_miss=5 replay_fail=2)"
    );
}
