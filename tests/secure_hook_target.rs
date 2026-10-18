//! Receiver-local hooks must not join a wrapper inherited from a prototype.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn secure_hook_target_keeps_inherited_callbacks_receiver_local() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local order = {}
        local prototype = { Paint = function(self, value)
            self.value = value
            order[#order + 1] = 'original'
            return value, nil, 9
        end }
        hooksecurefunc(prototype, 'Paint', function() order[#order + 1] = 'shared' end)
        local first = setmetatable({}, { __index = prototype })
        local second = setmetatable({}, { __index = prototype })
        local firstCalls, secondCalls = 0, 0
        hooksecurefunc(first, 'Paint', function() firstCalls = firstCalls + 1 end)
        hooksecurefunc(first, 'Paint', function() order[#order + 1] = 'local' end)
        hooksecurefunc(second, 'Paint', function() secondCalls = secondCalls + 1 end)
        local value, gap, tail = second:Paint(17)
        assert(firstCalls == 0, 'second receiver invoked first receiver hook: ' .. firstCalls)
        assert(secondCalls == 1 and value == 17 and gap == nil and tail == 9)
        assert(table.concat(order, ',') == 'original,shared')
        first:Paint(23)
        assert(firstCalls == 1 and secondCalls == 1)
        assert(table.concat(order, ',') == 'original,shared,original,shared,local')
        prototype:Paint(31)
        assert(firstCalls == 1 and secondCalls == 1)
        assert(first.value == 23 and second.value == 17 and prototype.value == 31)
        "#,
    )
    .unwrap();
}

#[test]
fn secure_hook_target_keeps_copied_wrappers_key_local() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local object = { Paint = function() return 7, nil, 11 end }
        local paintCalls, aliasCalls = 0, 0
        hooksecurefunc(object, 'Paint', function() paintCalls = paintCalls + 1 end)
        object.Alias = object.Paint
        hooksecurefunc(object, 'Alias', function() aliasCalls = aliasCalls + 1 end)
        object.Paint()
        assert(aliasCalls == 0, 'alias hook leaked onto original key: ' .. aliasCalls)
        local first, gap, last = object.Alias()
        assert(first == 7 and gap == nil and last == 11)
        assert(paintCalls == 2 and aliasCalls == 1)
        "#,
    )
    .unwrap();
}

#[test]
fn secure_hook_target_ellesmere_timer_does_not_repaint_unrelated_caret() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(include_str!("fixtures/ellesmere_hook_consumers.lua"))
        .unwrap();
    env.exec(
        "C_Timer.After(0, function() EllesmereCaretPaints = RunEllesmereHookFixture() end)",
    )
    .unwrap();
    env.process_timers().unwrap();
    let caret_paints: i32 = env.eval("return EllesmereCaretPaints").unwrap();
    assert_eq!(caret_paints, 0, "border repaint reached unrelated caret hooks");
}
