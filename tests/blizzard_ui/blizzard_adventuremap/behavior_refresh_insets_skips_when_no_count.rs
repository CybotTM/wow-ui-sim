//! `AdventureMapMixin:RefreshInsets` empty-count short-circuit behavior.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AdventureMapInset;

const ROOT: &str = "Blizzard_AdventureMap";

#[test]
fn adventure_map_refresh_insets_skips_when_count_is_nil() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.state().borrow_mut().adventure_map.insets = None;

        let calls = probe_refresh_insets_call_counts(env);

        assert_no_inset_refresh_body_calls("nil", calls);
    });
}

#[test]
fn adventure_map_refresh_insets_skips_when_count_is_zero() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.state().borrow_mut().adventure_map.insets = Some(Vec::<AdventureMapInset>::new());

        let calls = probe_refresh_insets_call_counts(env);

        assert_no_inset_refresh_body_calls("0", calls);
    });
}

type RefreshInsetsCallCounts = (i64, i64, i64);

fn probe_refresh_insets_call_counts(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
) -> RefreshInsetsCallCounts {
    env.eval(
        r#"
        local originalMapCanvasRefreshInsets = MapCanvasMixin.RefreshInsets
        local originalGetMapInsetInfo = C_AdventureMap.GetMapInsetInfo
        local originalAddInset = AdventureMapFrame.AddInset

        local baseRefreshCount = 0
        local getMapInsetInfoCount = 0
        local addInsetCount = 0

        MapCanvasMixin.RefreshInsets = function()
            baseRefreshCount = baseRefreshCount + 1
        end
        C_AdventureMap.GetMapInsetInfo = function(...)
            getMapInsetInfoCount = getMapInsetInfoCount + 1
            return originalGetMapInsetInfo(...)
        end
        AdventureMapFrame.AddInset = function(...)
            addInsetCount = addInsetCount + 1
            if originalAddInset then
                return originalAddInset(...)
            end
        end

        AdventureMapMixin.RefreshInsets(AdventureMapFrame)

        MapCanvasMixin.RefreshInsets = originalMapCanvasRefreshInsets
        C_AdventureMap.GetMapInsetInfo = originalGetMapInsetInfo
        AdventureMapFrame.AddInset = originalAddInset

        return baseRefreshCount, getMapInsetInfoCount, addInsetCount
        "#,
    )
    .expect("AdventureMap RefreshInsets empty-count probe must run cleanly")
}

fn assert_no_inset_refresh_body_calls(count_description: &str, calls: RefreshInsetsCallCounts) {
    let (base_refresh_count, get_map_inset_info_count, add_inset_count) = calls;

    assert_eq!(
        base_refresh_count, 1,
        "`AdventureMapMixin:RefreshInsets` must still call the base MapCanvas refresh"
    );
    assert_eq!(
        get_map_inset_info_count, 0,
        "`RefreshInsets` must not call `GetMapInsetInfo` when inset count is {count_description}"
    );
    assert_eq!(
        add_inset_count, 0,
        "`RefreshInsets` must not call `AddInset` when inset count is {count_description}"
    );
}
