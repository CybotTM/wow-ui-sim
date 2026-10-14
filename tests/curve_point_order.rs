//! Numeric curve ordering for the actual EncounterTimeline alpha point sequence.

use wow_ui_sim::lua_api::WowLuaEnv;

const TIMELINE_POINTS: &str = r#"
    curve = C_CurveUtil.CreateCurve()
    curve:AddPoint(3, 0.6)
    curve:AddPoint(2, 1.0)
    curve:AddPoint(1, 1.0)
    curve:AddPoint(0, 1.0)
    curve:AddPoint(4, 1.0)
"#;

#[test]
fn numeric_curve_evaluates_timeline_points_in_x_order() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(TIMELINE_POINTS).unwrap();
    env.exec(
        r#"
        assert(curve:GetPointCount() == 5)
        local samples = {{1, 1}, {0, 1}, {2, 1}, {2.5, 0.8}, {3, 0.6}, {4, 1}}
        for _, sample in ipairs(samples) do
            local actual = curve:Evaluate(sample[1])
            assert(math.abs(actual - sample[2]) < 0.000001,
                string.format('x=%g: expected %g, got %g', sample[1], sample[2], actual))
        end
        "#,
    )
    .expect("descending and ascending insertions evaluate by numeric x position");
}

#[test]
fn numeric_curve_copy_preserves_ordered_values_and_point_isolation() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(TIMELINE_POINTS).unwrap();
    env.exec(
        r#"
        local copy = curve:Copy()
        assert(copy ~= curve and copy:GetPointCount() == 5)
        assert(math.abs(copy:Evaluate(2.5) - 0.8) < 0.000001)
        copy:AddPoint(2.5, 0.75)
        assert(copy:GetPointCount() == 6 and curve:GetPointCount() == 5)
        assert(copy:Evaluate(2.5) == 0.75)
        assert(math.abs(curve:Evaluate(2.5) - 0.8) < 0.000001)
        "#,
    )
    .expect("copied numeric curves retain ordered interpolation and independent points");
}
