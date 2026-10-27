#![cfg(feature = "client-retail")]

use crate::common::blizzard_addon_harness::with_blizzard_addon_closure;

#[test]
fn original_time_util_preserves_non_ascii_format_literals() {
    with_blizzard_addon_closure(&["Blizzard_SharedXML"], &[], |env, loaded| {
        assert!(loaded.iter().any(|name| name == "Blizzard_SharedXML"));
        env.exec(r#"
            assert(debug.getinfo(TimeUtil.GetRecentTimeDate, 'S').source:find('TimeUtil.lua', 1, true))
            local function hex(value)
                return (value:gsub('.', function(byte)
                    return string.format('%02x', string.byte(byte))
                end))
            end
            local failures = {}
            for _, case in ipairs({
                {'été %1$d %%', 2, 'été 2 %'},
                {'雪 %1$d %%', 3, '雪 3 %'},
            }) do
                LASTONLINE_HOURS = case[1]
                local actual = TimeUtil.GetRecentTimeDate(0, 0, 0, case[2])
                if actual ~= case[3] then
                    table.insert(failures, 'actual=' .. hex(actual) .. ' expected=' .. hex(case[3]))
                end
            end
            assert(#failures == 0, table.concat(failures, '; '))
        "#).expect("original TimeUtil must preserve format literals");
    });
}
