use super::env;

#[test]
fn get_superceding_achievements_returns_one_empty_table_for_absent_relation() {
    let env = env();
    let empty: bool = env
        .eval(
            r#"
            local function capture(...) return select('#', ...), ... end
            local count, result = capture(C_AchievementInfo.GetSupercedingAchievements(999999))
            assert(count == 1, 'expected exactly one relation table')
            assert(type(result) == 'table', 'expected a relation table')
            return next(result) == nil
            "#,
        )
        .unwrap();
    assert!(empty);
}
