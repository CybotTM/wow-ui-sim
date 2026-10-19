//! Observable contracts for the constant HookScript and attribute dispatch factories.

use super::WowLuaEnv;
use std::rc::Rc;

#[test]
fn reload_factory_hooks_preserve_order_arguments_returns_and_private_upvalues() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local create, luaType = CreateFrame, type
        local log = {}
        local payload = {}
        local function callback(label, expected)
            local count = 0
            return function(self, ...)
                count = count + 1
                assert(self == expected)
                assert(select('#', ...) == 3)
                local object, missing, number = ...
                assert(object == payload and missing == nil and number == 7)
                log[#log + 1] = label .. count
                return label, nil, count
            end
        end
        local first, second = create('Frame'), create('Frame')
        for index, frame in ipairs({first, second}) do
            frame:SetScript('OnEvent', callback(index .. ':base:', frame))
            assert(frame:HookScript('OnEvent', callback(index .. ':hook:', frame)) == true)
            assert(frame:HookScript('OnEvent', callback(index .. ':last:', frame)) == true)
        end
        local firstChain, secondChain = first:GetScript('OnEvent'), second:GetScript('OnEvent')
        assert(firstChain ~= secondChain)
        assert(select('#', firstChain(first, payload, nil, 7)) == 0)
        secondChain(second, payload, nil, 7)
        firstChain(first, payload, nil, 7)
        assert(table.concat(log, ',') ==
            '1:base:1,1:hook:1,1:last:1,2:base:1,2:hook:1,2:last:1,1:base:2,1:hook:2,1:last:2')

        local solo = create('Frame')
        local hook = function(self, value)
            assert(self == solo and value == payload)
            return 'solo', nil, 13
        end
        assert(solo:HookScript('OnEvent', hook) == true)
        assert(solo:GetScript('OnEvent') == hook)
        local text, missing, number = solo:GetScript('OnEvent')(solo, payload)
        assert(text == 'solo' and missing == nil and number == 13)
        assert(CreateFrame == create and type == luaType)
        "#,
    )
    .unwrap();
}

#[test]
fn reload_factory_hook_errors_abort_the_chain_but_not_other_event_listeners() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        HookFailureLog = {}
        local bad, good = CreateFrame('Frame'), CreateFrame('Frame')
        bad:RegisterEvent('PLAYER_ENTERING_WORLD')
        good:RegisterEvent('PLAYER_ENTERING_WORLD')
        bad:SetScript('OnEvent', function()
            HookFailureLog[#HookFailureLog + 1] = 'base'
            error('factory old-handler failure')
        end)
        bad:HookScript('OnEvent', function() error('unreachable hook') end)
        good:SetScript('OnEvent', function(self, event)
            assert(self == good and event == 'PLAYER_ENTERING_WORLD')
            HookFailureLog[#HookFailureLog + 1] = 'other-frame'
        end)
        local ok, message = pcall(bad:GetScript('OnEvent'), bad, 'manual')
        assert(not ok and tostring(message):find('factory old-handler failure', 1, true))
        assert(table.concat(HookFailureLog, ',') == 'base')
        HookFailureLog = {}

        local hooked = CreateFrame('Frame')
        hooked:SetScript('OnEvent', function() end)
        hooked:HookScript('OnEvent', function() error('factory hook failure') end)
        hooked:HookScript('OnEvent', function() error('unreachable last hook') end)
        local success, failure = pcall(hooked:GetScript('OnEvent'), hooked)
        assert(not success and tostring(failure):find('factory hook failure', 1, true))
        "#,
    )
    .unwrap();
    env.fire_event("PLAYER_ENTERING_WORLD").unwrap();
    assert!(
        env.eval::<bool>(
            "return #HookFailureLog == 2 and \
             ((HookFailureLog[1] == 'base' and HookFailureLog[2] == 'other-frame') or \
              (HookFailureLog[2] == 'base' and HookFailureLog[1] == 'other-frame'))"
        )
        .unwrap()
    );
    assert!(
        env.state()
            .borrow()
            .lua_errors
            .iter()
            .any(|error| error.contains("factory old-handler failure"))
    );
}

#[test]
fn reload_factory_attribute_dispatch_preserves_arguments_returns_and_frame_isolation() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        for _, direct in ipairs({false, true}) do
            local frames = {CreateFrame('Frame'), CreateFrame('Frame')}
            local log = {}
            for index, frame in ipairs(frames) do
                local count = 0
                local expected = frame
                local function changed(self, ...)
                    count = count + 1
                    assert(self == expected and select('#', ...) == 2)
                    local key, value = ...
                    assert(key == 'probe')
                    log[#log + 1] = {index, count, value, self:GetAttribute(key)}
                    return 'discarded', nil, 41
                end
                if direct then frame.OnAttributeChanged = changed
                else frame:SetScript('OnAttributeChanged', changed) end
            end
            assert(select('#', frames[1]:SetAttribute('probe', 73)) == 0)
            frames[2]:SetAttribute('probe', 'second')
            frames[1]:SetAttribute('probe', false)
            assert(frames[1]:GetAttribute('probe') == false)
            frames[1]:SetAttribute('probe', nil)
            assert(#log == 4)
            assert(log[1][1] == 1 and log[1][2] == 1 and log[1][3] == 73 and log[1][4] == 73)
            assert(log[2][1] == 2 and log[2][2] == 1 and log[2][3] == 'second')
            assert(log[3][1] == 1 and log[3][2] == 2 and log[3][3] == false and log[3][4] == nil)
            assert(log[4][1] == 1 and log[4][2] == 3 and log[4][3] == nil and log[4][4] == nil)
        end
        "#,
    )
    .unwrap();
}

#[test]
fn reload_factory_attribute_errors_are_reported_without_escaping_set_attribute() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        AttributeFactoryErrors = {}
        seterrorhandler(function(message)
            AttributeFactoryErrors[#AttributeFactoryErrors + 1] = tostring(message)
        end)
        for _, direct in ipairs({false, true}) do
            local bad, good = CreateFrame('Frame'), CreateFrame('Frame')
            local function fail(self, key, value)
                assert(self == bad and key == 'probe' and value == 19)
                error('factory attribute failure')
            end
            if direct then bad.OnAttributeChanged = fail
            else bad:SetScript('OnAttributeChanged', fail) end
            local ok = pcall(bad.SetAttribute, bad, 'probe', 19)
            assert(ok and bad:GetAttribute('probe') == 19)
            good:SetScript('OnAttributeChanged', function(self, key, value)
                assert(self == good and key == 'probe' and value == 23)
                self.observed = value
            end)
            good:SetAttribute('probe', 23)
            assert(good.observed == 23)
        end
        assert(#AttributeFactoryErrors == 2)
        for _, message in ipairs(AttributeFactoryErrors) do
            assert(message:find('factory attribute failure', 1, true))
        end
        "#,
    )
    .unwrap();
}

#[test]
fn reload_factory_attribute_caller_environments_are_fresh_for_every_dispatch() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        for _, direct in ipairs({false, true}) do
            local calls = 0
            local function changed(self, key, value)
                assert(key == 'probe' and value == 5)
                -- A callback may inspect/change its Lua caller's environment.
                -- That must not poison a subsequent dispatch on either frame.
                assert(getfenv(2) == _G)
                setfenv(2, { poisoned = true })
                calls = calls + 1
            end
            local first, second = CreateFrame('Frame'), CreateFrame('Frame')
            for _, frame in ipairs({first, second}) do
                if direct then frame.OnAttributeChanged = changed
                else frame:SetScript('OnAttributeChanged', changed) end
            end
            first:SetAttribute('probe', 5)
            second:SetAttribute('probe', 5)
            first:SetAttribute('probe', 5)
            assert(calls == 3)
        end
        "#,
    )
    .unwrap();
}

const RELOAD_FACTORIES: &str = r#"
    FactoryReloadFrames = {}
    local function record(expected, label)
        local count = 0
        local callback = function(self, ...)
            assert(self == expected)
            count = count + 1
            self.calls[#self.calls + 1] = {label, count, debug.getstacktaint(), select('#', ...)}
        end
        debug.setobjecttaint(callback, 'ReloadFactoryOwner')
        return callback
    end
    for index = 1, 2 do
        local frame = CreateFrame('Frame')
        frame.calls = {}
        frame:RegisterEvent('PLAYER_ENTERING_WORLD')
        frame:SetScript('OnEvent', record(frame, 'base'))
        frame:HookScript('OnEvent', record(frame, 'hook'))
        local changed = record(frame, 'attribute')
        if index == 1 then frame:SetScript('OnAttributeChanged', changed)
        else frame.OnAttributeChanged = changed end
        FactoryReloadFrames[index] = frame
    end
"#;

fn exercise_reload_factories(env: &WowLuaEnv) {
    env.fire_event("PLAYER_ENTERING_WORLD").unwrap();
    env.exec(
        r#"
        assert(FactoryReloadFrames[1]:GetScript('OnEvent') ~= FactoryReloadFrames[2]:GetScript('OnEvent'))
        for _, frame in ipairs(FactoryReloadFrames) do
            frame:SetAttribute('probe', 7)
            assert(#frame.calls == 3)
            for index, label in ipairs({'base', 'hook', 'attribute'}) do
                local call = frame.calls[index]
                assert(call[1] == label and call[2] == 1)
                assert(call[3] == 'ReloadFactoryOwner')
                assert(call[4] == (label == 'attribute' and 2 or 1))
            end
        end
        "#,
    )
    .unwrap();
}

#[test]
fn reload_factory_rebuild_retires_old_closures_and_replays_fresh_tainted_handlers() {
    let mut env = WowLuaEnv::new().unwrap();
    env.set_reload_loader(|env, _| env.exec(RELOAD_FACTORIES));
    env.exec(RELOAD_FACTORIES).unwrap();
    exercise_reload_factories(&env);
    let old_lua = Rc::downgrade(&env.lua);
    env.exec(
        "FactoryOldSentinel = {}; C_Timer.After(0, function() FactoryOldTimer = true end); ReloadUI()",
    )
    .unwrap();
    assert!(env.service_reload(&mut None).unwrap());
    assert!(old_lua.upgrade().is_none(), "old Lua VM must be destroyed");
    env.process_timers().unwrap();
    assert!(
        env.eval::<bool>("return FactoryOldSentinel == nil and FactoryOldTimer == nil")
            .unwrap()
    );
    exercise_reload_factories(&env);
}
