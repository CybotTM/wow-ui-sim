#![cfg(feature = "client-retail")]

use crate::common::blizzard_addon_harness::with_blizzard_addon_closure;

#[test]
fn original_time_util_preserves_non_ascii_format_literals() {
    with_blizzard_addon_closure(&["Blizzard_SharedXML"], &[], |env, loaded| {
        assert!(loaded.iter().any(|name| name == "Blizzard_SharedXML"));
        env.exec(r#"
            assert(debug.getinfo(TimeUtil.GetRecentTimeDate, 'S').source:find('TimeUtil.lua', 1, true))
            LASTONLINE_HOURS = 'été %1$d %%'
            assert(TimeUtil.GetRecentTimeDate(0, 0, 0, 2) == 'été 2 %')
            LASTONLINE_HOURS = '雪 %1$d %%'
            assert(TimeUtil.GetRecentTimeDate(0, 0, 0, 3) == '雪 3 %')
        "#).expect("original TimeUtil must preserve format literals");
    });
}
