use rilua_bridge_tests::benchmark_table_field_access;

const ITERATIONS_PER_ROUND: u32 = 1_000_000;
const ROUNDS: u32 = 20;

fn main() {
    let result = benchmark_table_field_access(ITERATIONS_PER_ROUND, ROUNDS)
        .expect("table field access benchmark should run");

    println!("table field access benchmark");
    println!("iterations_per_round={}", result.iterations_per_round);
    println!("rounds={}", result.rounds);
    println!("total_iterations={}", result.total_iterations());
    println!("plain_elapsed_ns={}", result.plain_elapsed.as_nanos());
    println!("backed_elapsed_ns={}", result.backed_elapsed.as_nanos());
    println!("plain_ns_per_access={:.3}", result.plain_ns_per_access());
    println!("backed_ns_per_access={:.3}", result.backed_ns_per_access());
    println!(
        "backed_over_plain_ratio={:.3}",
        result.backed_over_plain_ratio()
    );
}
