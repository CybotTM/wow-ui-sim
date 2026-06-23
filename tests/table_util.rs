use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn table_count_counts_all_non_nil_entries() {
    let env = env();
    let counts: (i32, i32, i32) = env
        .eval(
            r#"
            local labels = {
                "first",
                "second",
                [4] = "fourth",
                rank = "leader",
                skipped = nil,
            }
            return table.count(labels)
            "#,
        )
        .unwrap();
    assert_eq!(
        counts,
        (4, 3, 4),
        "table.count should return total nodes, array nodes, and max array index"
    );
}

#[test]
fn table_count_returns_nothing_for_non_tables() {
    let env = env();
    let returned_count: i32 = env
        .eval("return select('#', table.count('not-table'))")
        .unwrap();
    assert_eq!(returned_count, 0, "non-table inputs should return nothing");
}

#[test]
fn table_util_find_indexed_mismatch_returns_nil_for_equal_arrays() {
    let env = env();
    let mismatch_index: Option<i32> = env
        .eval(
            r#"
            local t1 = { 1, 2, 3, 4 }
            local t2 = { 1, 2, 3, 4 }
            return C_TableUtil.FindIndexedMismatch(t1, t2)
            "#,
        )
        .unwrap();
    assert_eq!(mismatch_index, None, "equal arrays should not mismatch");
}

#[test]
fn table_util_find_indexed_mismatch_returns_first_mismatch_index() {
    let env = env();
    let mismatch_index: Option<i32> = env
        .eval(
            r#"
            local t1 = { "a", "b", "c", "d" }
            local t2 = { "a", "x", "c", "d" }
            return C_TableUtil.FindIndexedMismatch(t1, t2)
            "#,
        )
        .unwrap();
    assert_eq!(
        mismatch_index,
        Some(2),
        "first differing element should be reported"
    );
}

#[test]
fn table_util_find_indexed_mismatch_detects_length_difference() {
    let env = env();
    let mismatch_index: Option<i32> = env
        .eval(
            r#"
            local t1 = { 10, 20, 30 }
            local t2 = { 10, 20, 30, 40 }
            return C_TableUtil.FindIndexedMismatch(t1, t2)
            "#,
        )
        .unwrap();
    assert_eq!(
        mismatch_index,
        Some(4),
        "extra entries should be reported as mismatch at first missing index"
    );
}

#[test]
fn table_util_find_indexed_mismatch_supports_comparator_function() {
    let env = env();
    let mismatch_index: Option<i32> = env
        .eval(
            r#"
            local t1 = { "Alpha", "Beta", "Gamma" }
            local t2 = { "alpha", "BETA", "delta" }
            local function comparator(v1, v2, index)
                return string.lower(v1) == string.lower(v2)
            end
            return C_TableUtil.FindIndexedMismatch(t1, t2, comparator)
            "#,
        )
        .unwrap();
    assert_eq!(
        mismatch_index,
        Some(3),
        "comparator should control equality check for each indexed element"
    );
}

#[test]
fn table_util_find_indexed_mismatch_returns_nil_for_non_tables() {
    let env = env();
    let mismatch_index: Option<i32> = env
        .eval("return C_TableUtil.FindIndexedMismatch('not-table', { 1, 2, 3 })")
        .unwrap();
    assert_eq!(mismatch_index, None, "non-table inputs should return nil");
}
