//! Startup intern-stats report for the wow-cli measurement command.

use std::time::Instant;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::settle_headless_startup;

#[cfg(feature = "intern-stats")]
const TOP_N: usize = 50;

pub fn run() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let started = Instant::now();
    settle_headless_startup(&env);
    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;

    println!("startup_elapsed_ms={elapsed_ms:.3}");
    print_intern_report();
}

#[cfg(feature = "intern-stats")]
fn print_intern_report() {
    let top = rilua::vm::intern_stats::snapshot_top(TOP_N);
    let total = rilua::vm::intern_stats::total_calls();
    let unique = rilua::vm::intern_stats::unique_strings();

    println!(
        "intern-stats total_calls={total} unique_strings={unique} top_{TOP_N} bucketed_report"
    );
    print_bucket_histogram(&top);
    print_top_entries(&top);
}

#[cfg(not(feature = "intern-stats"))]
fn print_intern_report() {
    println!("intern-stats feature disabled; rerun with --features intern-stats to collect counts");
}

#[cfg(feature = "intern-stats")]
fn print_bucket_histogram(top: &[(Vec<u8>, u64)]) {
    println!("bucket_histogram=");
    for (label, count) in bucket_histogram(top) {
        println!("  {label:>7}: {count}");
    }
}

#[cfg(feature = "intern-stats")]
fn print_top_entries(top: &[(Vec<u8>, u64)]) {
    println!("top_entries=");
    for (data, count) in top {
        let preview = String::from_utf8_lossy(data);
        let shown: String = preview.chars().take(48).collect();
        let suffix = if preview.len() > 48 { "…" } else { "" };
        println!("  {count:>10} x {shown:?}{suffix}");
    }
}

#[cfg(any(test, feature = "intern-stats"))]
pub(crate) fn bucket_histogram(top: &[(Vec<u8>, u64)]) -> Vec<(&'static str, usize)> {
    let mut buckets = [
        ("1", 0usize),
        ("2-3", 0),
        ("4-7", 0),
        ("8-15", 0),
        ("16-31", 0),
        ("32-63", 0),
        ("64+", 0),
    ];
    for (_, count) in top {
        let slot = match *count {
            0 | 1 => 0,
            2 | 3 => 1,
            4..=7 => 2,
            8..=15 => 3,
            16..=31 => 4,
            32..=63 => 5,
            _ => 6,
        };
        buckets[slot].1 += 1;
    }
    buckets.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::bucket_histogram;

    #[test]
    fn histogram_groups_counts_into_logish_buckets() {
        let top = vec![
            (b"a".to_vec(), 1),
            (b"b".to_vec(), 2),
            (b"c".to_vec(), 4),
            (b"d".to_vec(), 8),
            (b"e".to_vec(), 16),
            (b"f".to_vec(), 32),
            (b"g".to_vec(), 64),
        ];

        let histogram = bucket_histogram(&top);
        assert_eq!(
            histogram,
            vec![
                ("1", 1),
                ("2-3", 1),
                ("4-7", 1),
                ("8-15", 1),
                ("16-31", 1),
                ("32-63", 1),
                ("64+", 1),
            ]
        );
    }
}
