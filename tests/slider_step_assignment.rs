//! Shared clean-room Slider quantization policy, not native-client parity.

use wow_ui_sim::lua_api::WowLuaEnv;

const OBSERVERS: &str = r#"
    local function observe(slider)
        local events = {}
        slider:SetScript('OnValueChanged', function(...)
            local self, value, mouse = ...
            events[#events + 1] = {
                owner = self, value = value, mouse = mouse,
                arity = select('#', ...), committed = self:GetValue(),
            }
        end)
        return events
    end

    local function expect_event(slider, events, count, value, mouse)
        assert(slider:GetValue() == value, 'stored value must equal expected effective value')
        assert(#events == count, 'unexpected synchronous callback count')
        local event = events[count]
        assert(event.owner == slider, 'callback must receive the Slider')
        assert(event.value == value, 'callback payload must be quantized')
        assert(event.committed == value, 'callback must observe committed value')
        assert(event.arity == 3 and event.mouse == mouse, 'Slider mouse payload changed')
    end
"#;

fn run_lua(body: &str) {
    // Keep helpers and body in the same Lua lexical scope. No manual dispatch.
    let script = format!("{OBSERVERS}\n{body}");
    WowLuaEnv::new().unwrap().exec(&script).unwrap();
}

#[test]
fn slider_step_assignment_chooses_nearest_lattice_or_max_with_upward_ties() {
    run_lua(
        r#"
        local slider = CreateFrame('Slider')
        slider:SetMinMaxValues(0, 10)
        slider:SetValueStep(2)
        slider:SetValue(0)
        local events = observe(slider)

        slider:SetValue(3)
        expect_event(slider, events, 1, 4, false)
        slider:SetValueStep(4)
        assert(slider:GetValue() == 4 and #events == 1, 'step change retaining 4 must be silent')
        slider:SetValue(9, false)
        expect_event(slider, events, 2, 10, false)
        slider:SetValue(8.75, false)
        expect_event(slider, events, 3, 8, false)
        slider:SetValue(9.25, true)
        expect_event(slider, events, 4, 10, true)
    "#,
    );
}

#[test]
fn slider_step_assignment_anchors_lattice_at_nonzero_and_negative_minima() {
    run_lua(
        r#"
        local cases = {
            {5, 15, 4, 7, 9},
            {5, 15, 4, 13.75, 13},
            {5, 15, 4, 14, 15},
            {-10, 0, 4, -8, -6},
            {-10, 0, 4, -1, 0},
            {-5, 6, 4, -3, -1},
            {-5, 6, 4, 4.5, 6},
        }
        for _, case in ipairs(cases) do
            local slider = CreateFrame('Slider')
            slider:SetMinMaxValues(case[1], case[2])
            slider:SetValueStep(case[3])
            slider:SetValue(case[1])
            local events = observe(slider)
            slider:SetValue(case[4], false)
            expect_event(slider, events, 1, case[5], false)
        end
    "#,
    );
}

#[test]
fn slider_step_assignment_preserves_exact_bounds_and_suppresses_effective_duplicates() {
    run_lua(
        r#"
        local slider = CreateFrame('Slider')
        slider:SetMinMaxValues(0, 10)
        slider:SetValueStep(4)
        slider:SetValue(0)
        local events = observe(slider)

        slider:SetValue(-100)
        assert(slider:GetValue() == 0 and #events == 0, 'lower clamp must be silent at minimum')
        slider:SetValue(10)
        expect_event(slider, events, 1, 10, false)
        slider:SetValue(100)
        slider:SetValue(10)
        assert(slider:GetValue() == 10 and #events == 1, 'upper-bound repeats must be silent')
        slider:SetValue(0, false)
        expect_event(slider, events, 2, 0, false)
        slider:SetValue(4, false)
        expect_event(slider, events, 3, 4, false)
        slider:SetValue(4)
        slider:SetValue(4.1)
        slider:SetValue(4, true)
        slider:SetValueStep(4)
        slider:SetValueStep(2)
        slider:SetValueStep(1)
        assert(slider:GetValue() == 4 and #events == 3,
            'redundant assignment or step change must not dispatch')
    "#,
    );
}

#[test]
fn slider_step_assignment_step_changes_requantize_and_dispatch_only_effective_changes() {
    run_lua(
        r#"
        local slider = CreateFrame('Slider')
        slider:SetMinMaxValues(0, 10)
        slider:SetValueStep(1)
        slider:SetValue(3)
        local events = observe(slider)

        slider:SetValueStep(2)
        expect_event(slider, events, 1, 4, false)
        slider:SetValueStep(2)
        slider:SetValueStep(4)
        assert(slider:GetValue() == 4 and #events == 1, 'unchanged effective value must be silent')
        slider:SetValueStep(3)
        expect_event(slider, events, 2, 3, false)
        slider:SetValueStep(0)
        assert(slider:GetValue() == 3 and #events == 2, 'enabling continuous mode must retain value')
        slider:SetValue(3.25, false)
        expect_event(slider, events, 3, 3.25, false)
        slider:SetValueStep(2)
        expect_event(slider, events, 4, 4, false)
    "#,
    );
}

#[test]
fn slider_step_assignment_zero_is_continuous_and_statusbar_stays_unsnapped() {
    run_lua(
        r#"
        local slider = CreateFrame('Slider')
        slider:SetMinMaxValues(0, 10)
        slider:SetValueStep(0)
        slider:SetValue(0)
        local sliderEvents = observe(slider)
        local bar = CreateFrame('StatusBar')
        bar:SetMinMaxValues(0, 10)
        bar:SetValue(0)
        local barEvents = {}
        bar:SetScript('OnValueChanged', function(...)
            local self, value = ...
            barEvents[#barEvents + 1] = {
                owner = self, value = value, committed = self:GetValue(), arity = select('#', ...),
            }
        end)

        slider:SetValue(3.25, false)
        expect_event(slider, sliderEvents, 1, 3.25, false)
        slider:SetValue(3.25)
        assert(slider:GetValue() == 3.25 and #sliderEvents == 1)
        bar:SetValue(3.25)
        slider:SetValueStep(4)
        expect_event(slider, sliderEvents, 2, 4, false)
        assert(bar:GetValue() == 3.25 and #barEvents == 1, 'Slider changes must not affect StatusBar')
        bar:SetValue(9)
        bar:SetValue(9)
        bar:SetValue(100)
        assert(bar:GetValue() == 10 and #barEvents == 3, 'StatusBar must only clamp and deduplicate')
        for i, expected in ipairs({3.25, 9, 10}) do
            local event = barEvents[i]
            assert(event.owner == bar and event.value == expected and event.committed == expected,
                'StatusBar payload and committed value must remain unsnapped')
            assert(event.arity == 2, 'StatusBar must not inherit Slider mouse flag')
        end
    "#,
    );
}
