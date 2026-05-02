//! AddOnPerformance warning predicate behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn addon_warning_predicate_returns_nil_for_unflagged_addon() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: UnflaggedWarningProbe = env
            .eval(
                r#"
                local addOnName = "UnflaggedPerformanceProbe"
                local warning = AddOnPerformance:AddOnHasPerformanceWarning(addOnName)
                return warning == nil,
                       not warning,
                       AddOnPerformance.addOnHasPerformanceWarning[addOnName] == nil
                "#,
            )
            .expect("AddOnPerformance unflagged warning probe must run cleanly");

        assert_unflagged_warning_probe(probe);
    });
}

type UnflaggedWarningProbe = (bool, bool, bool);

fn assert_unflagged_warning_probe(probe: UnflaggedWarningProbe) {
    let (predicate_returned_nil, predicate_is_falsey, table_entry_is_nil) = probe;

    assert!(
        predicate_returned_nil,
        "`AddOnHasPerformanceWarning(addOnName)` must return nil before an addon is flagged"
    );
    assert!(
        predicate_is_falsey,
        "unflagged addon warning predicate must behave as false in Lua conditionals"
    );
    assert!(
        table_entry_is_nil,
        "unflagged addon must not have a warning cache entry"
    );
}
