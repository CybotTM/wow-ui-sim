//! C8 shared simulator candidate: UNCOMPILED / UNEXECUTED, not native parity.
//! Range reapplication depends on the pending C1 step policy.

use wow_ui_sim::lua_api::WowLuaEnv;

const HELPERS: &str = r#"
    local function slider_at_seven()
        local slider = CreateFrame('Slider')
        slider:SetMinMaxValues(0, 10)
        slider:SetValueStep(1)
        slider:SetValue(7)
        return slider
    end
    local function snapshot(trace, label, widget)
        local lo, hi = widget:GetMinMaxValues()
        trace[#trace + 1] = label .. ':' .. lo .. ':' .. hi .. ':' .. widget:GetValue()
    end
    local function observe_value(slider, trace)
        slider:SetScript('OnValueChanged', function(...)
            local self, value, mouse = ...
            assert(self == slider and select('#', ...) == 3 and mouse == false,
                'range-triggered Slider assignment must keep the ordinary false mouse payload')
            assert(self:GetValue() == value, 'value callback must observe committed payload')
            snapshot(trace, 'value=' .. value, self)
        end)
    end
    local function observe_range(slider, trace)
        slider:SetScript('OnMinMaxChanged', function(...)
            local self, lo, hi = ...
            assert(self == slider and select('#', ...) == 3, 'range callback payload changed')
            local committedLo, committedHi = self:GetMinMaxValues()
            assert(lo == committedLo and hi == committedHi, 'range must commit before notification')
            snapshot(trace, 'range', self)
        end)
    end
    local function expect(trace, expected)
        local actual = table.concat(trace, '|')
        assert(actual == expected, 'expected ' .. expected .. '; got ' .. actual)
    end
"#;

fn run_lua(body: &str) {
    WowLuaEnv::new()
        .unwrap()
        .exec(&format!("{HELPERS}\n{body}"))
        .unwrap();
}

#[test]
fn c8_range_handler_is_publicly_bindable_and_clearable() {
    run_lua(
        r#"
        local slider = slider_at_seven()
        assert(slider:HasScript('OnMinMaxChanged'), 'Slider must support OnMinMaxChanged')
        assert(slider:GetScript('OnMinMaxChanged') == nil)
        local handler = function() error('binding alone must not dispatch') end
        slider:SetScript('OnMinMaxChanged', handler)
        assert(slider:GetScript('OnMinMaxChanged') == handler)
        slider:SetScript('OnMinMaxChanged', nil)
        assert(slider:GetScript('OnMinMaxChanged') == nil)
        assert(slider:HasScript('OnMinMaxChanged'), 'clearing must not remove type capability')
    "#,
    );
}

#[test]
fn c8_changed_range_notifies_before_reapplying_value_synchronously() {
    run_lua(
        r#"
        local slider, trace = slider_at_seven(), {}
        slider:Hide()
        observe_value(slider, trace)
        observe_range(slider, trace)
        snapshot(trace, 'before', slider)
        slider:SetMinMaxValues(2, 5)
        snapshot(trace, 'after', slider)
        expect(trace, 'before:0:10:7|range:2:5:7|value=5:2:5:5|after:2:5:5')
        trace = {}
        observe_value(slider, trace)
        observe_range(slider, trace)
        snapshot(trace, 'before', slider)
        slider:SetMinMaxValues(3, 3)
        snapshot(trace, 'after', slider)
        expect(trace, 'before:2:5:5|range:3:3:5|value=3:3:3:3|after:3:3:3')
    "#,
    );
}

#[test]
fn c8_range_reapplication_dispatches_value_even_without_range_binding() {
    run_lua(
        r#"
        local slider, trace = slider_at_seven(), {}
        observe_value(slider, trace)
        snapshot(trace, 'before', slider)
        slider:SetMinMaxValues(2, 5)
        snapshot(trace, 'after', slider)
        expect(trace, 'before:0:10:7|value=5:2:5:5|after:2:5:5')
    "#,
    );
}

#[test]
fn c8_retained_value_notifies_range_once_and_identical_range_is_silent() {
    run_lua(
        r#"
        local slider, trace = slider_at_seven(), {}
        observe_value(slider, trace)
        observe_range(slider, trace)
        snapshot(trace, 'before', slider)
        slider:SetMinMaxValues(2, 9)
        snapshot(trace, 'changed', slider)
        slider:SetMinMaxValues(2, 9)
        snapshot(trace, 'same', slider)
        expect(trace, 'before:0:10:7|range:2:9:7|changed:2:9:7|same:2:9:7')
    "#,
    );
}

#[test]
fn c8_range_reapplication_uses_pending_c1_policy_with_explicit_step() {
    run_lua(
        r#"
        local slider, trace = CreateFrame('Slider'), {}
        slider:SetMinMaxValues(0, 10)
        slider:SetValueStep(4)
        slider:SetValue(8)
        observe_value(slider, trace)
        snapshot(trace, 'before', slider)
        slider:SetMinMaxValues(1, 10)
        snapshot(trace, 'after', slider)
        expect(trace, 'before:0:10:8|value=9:1:10:9|after:1:10:9')
        slider:SetMinMaxValues(1, 8)
        snapshot(trace, 'endpoint', slider)
        expect(trace, 'before:0:10:8|value=9:1:10:9|after:1:10:9|value=8:1:8:8|endpoint:1:8:8')
    "#,
    );
}
