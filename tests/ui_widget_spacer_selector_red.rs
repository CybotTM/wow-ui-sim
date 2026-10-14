//! First Spacer selector contract; no new production records or seeded data.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn spacer_required_number_selectors_reject_non_numbers() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().expect("Spacer selector environment");
        env.exec(
            r#"
            local getter = C_UIWidgetManager.GetSpacerVisualizationInfo
            assert(type(getter) == 'function', 'Spacer query must be callable')
            local invalidCalls = {
                function() return getter() end,
                function() return getter(nil) end,
                function() return getter('invalid-widget') end,
                function() return getter('435') end,
                function() return getter(true) end,
                function() return getter({}) end,
                function() return getter(function() end) end,
                function() return getter(coroutine.create(function() end)) end,
            }
            for index, call in ipairs(invalidCalls) do
                local ok, err = pcall(call)
                assert(not ok, 'Spacer accepted non-number selector case ' .. index)
                assert(type(err) == 'string' and err:find('must be a number', 1, true),
                    'Spacer rejection must identify required number: ' .. tostring(err))
            end
            "#,
        )
        .expect("Spacer required-number selector behavior");
    });
}
