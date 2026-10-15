//! Tests for Slider widget methods.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

#[test]
fn statusbar_value_change_updates_text_synchronously() {
    env()
        .exec(
            r#"
        local bar = CreateFrame('StatusBar')
        bar:SetMinMaxValues(0, 100)
        local text = bar:CreateFontString()
        local calls = 0
        bar:SetScript('OnValueChanged', function(...)
            local self, value = ...
            assert(select('#', ...) == 2, 'StatusBar callbacks have no Slider mouse flag')
            assert(self == bar and self:GetValue() == value)
            calls = calls + 1
            text:SetText(tostring(value))
        end)
        bar:SetValue(35)
        assert(text:GetText() == '35', 'status bar callback must update its label before SetValue returns')
        bar:SetValue(75)
        assert(text:GetText() == '75' and calls == 2)
    "#,
        )
        .unwrap();
}

#[test]
fn slider_reversed_range_errors_without_changing_range_or_value() {
    env()
        .exec(
            r#"
        local slider = CreateFrame('Slider')
        slider:SetMinMaxValues(0, 10)
        slider:SetValue(7)
        local ok, err = pcall(slider.SetMinMaxValues, slider, 8, 2)
        assert(not ok and type(err) == 'string', 'reversed Slider range must raise a Lua error')
        local min, max = slider:GetMinMaxValues()
        assert(min == 0 and max == 10 and slider:GetValue() == 7,
            'failed range update must preserve range and value')
    "#,
        )
        .unwrap();
}

#[test]
fn statusbar_reversed_range_collapses_to_max_and_clamps_value() {
    env()
        .exec(
            r#"
        local bar = CreateFrame('StatusBar')
        bar:SetMinMaxValues(0, 10)
        bar:SetValue(7)
        local ok, err = pcall(bar.SetMinMaxValues, bar, 8, 2)
        assert(ok, 'reversed StatusBar range must succeed: ' .. tostring(err))
        local min, max = bar:GetMinMaxValues()
        assert(min == 2 and max == 2 and bar:GetValue() == 2,
            'reversed StatusBar range must collapse to the maximum bound')
    "#,
        )
        .unwrap();
}

#[test]
fn statusbar_reversed_range_clamps_pending_interpolation() {
    env()
        .exec(
            r#"
        local bar = CreateFrame('StatusBar')
        bar:SetMinMaxValues(0, 10)
        bar:SetValue(1)
        bar:SetValue(7, 'Smooth')
        assert(bar:GetValue() == 7 and bar:GetInterpolatedValue() == 1,
            'pending interpolation must have distinct target and displayed values')
        assert(bar:IsInterpolating(), 'interpolation must be pending before the range change')

        bar:SetMinMaxValues(8, 2)
        local min, max = bar:GetMinMaxValues()
        assert(min == 2 and max == 2, 'reversed range must collapse to its supplied maximum')
        assert(bar:GetValue() == 2 and bar:GetInterpolatedValue() == 2,
            'range change must clamp target and displayed value in opposite directions')
        assert(not bar:IsInterpolating(), 'collapsed range must settle the pending interpolation')
    "#,
        )
        .unwrap();
}

#[test]
fn slider_nan_bounds_error_atomically_and_allow_later_valid_updates() {
    env()
        .exec(
            r#"
        local slider = CreateFrame('Slider')
        slider:SetMinMaxValues(0, 10)
        slider:SetValue(7)
        local nan = 0 / 0
        for _, bounds in ipairs({{nan, 8}, {2, nan}}) do
            local ok, err = pcall(slider.SetMinMaxValues, slider, bounds[1], bounds[2])
            assert(not ok and type(err) == 'string', 'NaN Slider bound must raise a Lua error')
            local min, max = slider:GetMinMaxValues()
            assert(min == 0 and max == 10 and slider:GetValue() == 7,
                'failed NaN Slider update must preserve range and value')
            slider:SetMinMaxValues(2, 8)
            local validMin, validMax = slider:GetMinMaxValues()
            assert(validMin == 2 and validMax == 8 and slider:GetValue() == 7,
                'valid Slider range must work after rejected NaN')
            slider:SetMinMaxValues(0, 10)
        end
    "#,
        )
        .unwrap();
}

#[test]
fn statusbar_nan_bounds_error_atomically_and_allow_later_valid_updates() {
    env()
        .exec(
            r#"
        local bar = CreateFrame('StatusBar')
        bar:SetMinMaxValues(0, 10)
        bar:SetValue(7)
        local nan = 0 / 0
        for _, bounds in ipairs({{nan, 8}, {2, nan}}) do
            local ok, err = pcall(bar.SetMinMaxValues, bar, bounds[1], bounds[2])
            assert(not ok and type(err) == 'string', 'NaN StatusBar bound must raise a Lua error')
            local min, max = bar:GetMinMaxValues()
            assert(min == 0 and max == 10 and bar:GetValue() == 7,
                'failed NaN StatusBar update must preserve range and value')
            bar:SetMinMaxValues(2, 8)
            local validMin, validMax = bar:GetMinMaxValues()
            assert(validMin == 2 and validMax == 8 and bar:GetValue() == 7,
                'valid StatusBar range must work after rejected NaN')
            bar:SetMinMaxValues(0, 10)
        end
    "#,
        )
        .unwrap();
}

#[test]
fn equal_ranges_and_ordinary_range_clamps_work_for_slider_and_statusbar() {
    env()
        .exec(
            r#"
        for _, kind in ipairs({'Slider', 'StatusBar'}) do
            local widget = CreateFrame(kind)
            widget:SetMinMaxValues(0, 10)
            widget:SetValue(7)
            widget:SetMinMaxValues(2, 5)
            local min, max = widget:GetMinMaxValues()
            assert(min == 2 and max == 5 and widget:GetValue() == 5,
                kind .. ' must clamp an existing value to an ordinary range')
            widget:SetMinMaxValues(3, 3)
            min, max = widget:GetMinMaxValues()
            assert(min == 3 and max == 3 and widget:GetValue() == 3,
                kind .. ' must accept an equal-bound range and clamp its value')
        end
    "#,
        )
        .unwrap();
}

const VALUE_BINDINGS_XML: &str = r#"
<Ui>
    <Slider name="SliderValuePrecall" intrinsic="true">
        <Scripts>
            <OnValueChanged intrinsicOrder="precall">SliderValueProbe(self, 'pre', value)</OnValueChanged>
        </Scripts>
    </Slider>
    <Slider name="SliderValuePostcall" virtual="true">
        <Scripts>
            <OnValueChanged intrinsicOrder="postcall">SliderValueProbe(self, 'post', value)</OnValueChanged>
        </Scripts>
    </Slider>
</Ui>
"#;

fn value_binding_env() -> WowLuaEnv {
    let env = env();
    let root = tempfile::tempdir().unwrap();
    let toc = root.path().join("SliderValueBindings.toc");
    std::fs::write(&toc, "## Title: Slider value bindings\nBindings.xml\n").unwrap();
    std::fs::write(root.path().join("Bindings.xml"), VALUE_BINDINGS_XML).unwrap();
    let loaded = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    assert!(loaded.warnings.is_empty(), "{:?}", loaded.warnings);
    env
}

#[test]
fn set_value_dispatches_synchronously_with_mouse_payload() {
    env()
        .exec(
            r#"
        local slider = CreateFrame('Slider')
        slider:SetMinMaxValues(0, 10)
        slider:Hide()
        local calls = {}
        slider:SetScript('OnValueChanged', function(...)
            local self, value, mouse = ...
            calls[#calls + 1] = {self, value, mouse, select('#', ...), self:GetValue()}
        end)
        slider:SetValue(3)
        assert(#calls == 1, 'SetValue must dispatch before returning, even when hidden')
        assert(calls[1][1] == slider and calls[1][2] == 3 and calls[1][5] == 3)
        assert(calls[1][3] == false and calls[1][4] == 3, 'default mouse flag must be false')
        slider:SetValue(7, true)
        assert(#calls == 2 and calls[2][2] == 7 and calls[2][5] == 7)
        assert(calls[2][3] == true and calls[2][4] == 3, 'explicit mouse flag must be forwarded')
    "#,
        )
        .unwrap();
}

#[test]
fn set_value_clamps_suppresses_unchanged_and_allows_reentry() {
    env()
        .exec(
            r#"
        local slider = CreateFrame('Slider')
        slider:SetMinMaxValues(0, 10)
        local calls, errors = {}, {}
        seterrorhandler(function(message) errors[#errors + 1] = message end)
        slider:SetScript('OnValueChanged', function(self, value)
            calls[#calls + 1] = value
            self:SetValue(value)
        end)
        slider:SetValue(5)
        slider:SetValue(5, true)
        slider:SetValue(20)
        slider:SetValue(30)
        slider:SetValue(-3)
        slider:SetValue(-10)
        assert(#calls == 3, 'only changed clamped values must dispatch')
        assert(calls[1] == 5 and calls[2] == 10 and calls[3] == 0)
        assert(slider:GetValue() == 0)
        assert(#errors == 0, errors[1] or 'callback reentry must not fail')
    "#,
        )
        .unwrap();
}

#[test]
fn set_value_dispatches_intrinsic_bindings_after_reported_errors() {
    value_binding_env().exec(r#"
        local calls, errors = {}, {}
        local failPre = true
        seterrorhandler(function(message) errors[#errors + 1] = message end)
        function SliderValueProbe(self, binding, value)
            calls[#calls + 1] = binding .. ':' .. value
            assert(self:GetValue() == value, 'callback must observe committed value')
            if binding == 'pre' and failPre then error('slider precall failure') end
        end
        local slider = CreateFrame('Slider', nil, UIParent, 'SliderValuePrecall,SliderValuePostcall')
        slider:SetMinMaxValues(0, 10)
        slider:SetScript('OnValueChanged', function(self, value)
            SliderValueProbe(self, 'normal', value)
        end)
        slider:SetValue(3)
        assert(table.concat(calls, ',') == 'pre:3,normal:3,post:3', 'binding order or error continuation failed')
        assert(#errors == 1 and string.find(errors[1], 'slider precall failure', 1, true))
        failPre = false
        calls = {}
        slider:SetScript('OnValueChanged', nil)
        slider:SetValue(4)
        assert(table.concat(calls, ',') == 'pre:4,post:4', 'intrinsic bindings must run without normal script')
        assert(#errors == 1, 'unexpected later callback error')
    "#).unwrap();
}

#[test]
fn set_value_updates_castbar_style_scale_and_cvar_consumer() {
    env().exec(r#"
        local db = {scale = 1}
        local bar = CreateFrame('StatusBar')
        local slider = CreateFrame('Slider')
        slider:SetMinMaxValues(0.7, 1.8)
        C_CVar.RegisterCVar('__slider_castbar_scale', '')
        local calls = 0
        slider:SetScript('OnValueChanged', function(_, value)
            calls = calls + 1
            db.scale = tonumber(string.format('%.2f', value))
            bar:SetScale(db.scale)
            C_CVar.SetCVar('__slider_castbar_scale', tostring(db.scale))
        end)
        slider:SetValue(1.35)
        assert(calls == 1 and db.scale == 1.35, 'slider callback must update addon configuration')
        assert(math.abs(bar:GetScale() - 1.35) < 0.000001, 'castbar scale must update synchronously')
        assert(C_CVar.GetCVar('__slider_castbar_scale') == '1.35', 'callback CVar write missing')
        slider:SetValue(1)
        assert(calls == 2 and db.scale == 1 and bar:GetScale() == 1)
        assert(C_CVar.GetCVar('__slider_castbar_scale') == '1')
    "#).unwrap();
}

// ============================================================================
// GetThumbTexture / SetThumbTexture
// ============================================================================

#[test]
fn test_get_thumb_texture_defaults_to_thumb_child() {
    let env = env();
    let matches: bool = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        return s:GetThumbTexture() == s.ThumbTexture
    "#,
        )
        .unwrap();
    assert!(
        matches,
        "GetThumbTexture should return the default thumb child"
    );
}

#[test]
fn test_set_and_get_thumb_texture() {
    let env = env();
    let matches: bool = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        local t = s:CreateTexture()
        s:SetThumbTexture(t)
        return s:GetThumbTexture() == t
    "#,
        )
        .unwrap();
    assert!(
        matches,
        "GetThumbTexture should return the texture set via SetThumbTexture"
    );
}

#[test]
fn test_set_thumb_texture_fileid_keeps_same_object() {
    let env = env();
    let still_same: bool = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        local t = s:CreateTexture()
        s:SetThumbTexture(t)
        s:SetThumbTexture(12345)
        return s:GetThumbTexture() == t
    "#,
        )
        .unwrap();
    assert!(
        still_same,
        "SetThumbTexture with fileID should keep same texture object"
    );
}

#[test]
fn test_slider_set_thumb_texture_file_id_get_texture() {
    let env = env();
    let tex_id: i32 = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        s:SetThumbTexture(12345)
        local t = s:GetThumbTexture()
        return t:GetTexture()
    "#,
        )
        .unwrap();
    assert_eq!(tex_id, 12345);
}

#[test]
fn test_obey_step_on_drag_round_trip() {
    let env = env();
    let obeys: (bool, bool, bool) = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        local initial = s:GetObeyStepOnDrag()
        s:SetObeyStepOnDrag(true)
        local enabled = s:GetObeyStepOnDrag()
        s:SetObeyStepOnDrag(false)
        local disabled = s:GetObeyStepOnDrag()
        return initial, enabled, disabled
    "#,
        )
        .unwrap();
    assert_eq!(
        obeys,
        (false, true, false),
        "GetObeyStepOnDrag should round-trip the persisted slider flag"
    );
}

#[test]
fn slider_bound_thumb_tracks_value_range_orientation_and_resize() {
    env()
        .exec(
            r#"-- Public Slider contract, without guessing exact endpoint travel/insets.
-- HybridScrollFrame.xml uses a sized, unanchored ThumbTexture; reproduce that.
local slider = CreateFrame("Slider", "FeedbackThumbProbe", UIParent)
slider:SetSize(160, 40)
slider:SetPoint("CENTER", UIParent, "CENTER")
slider:SetMinMaxValues(10, 110)
slider:SetOrientation("HORIZONTAL")
local thumb = slider:GetThumbTexture()
thumb:SetSize(18, 24)
thumb:SetColorTexture(1, 0, 1, 1)
slider:SetThumbTexture(thumb)
local function center()
    local x, y = thumb:GetCenter()
    assert(type(x) == "number" and type(y) == "number",
        "THUMB_RED: bound sized thumb without explicit anchors must have a center")
    return x, y
end
local function near(a, b)
    return math.abs(a - b) < 0.01
end
slider:SetValue(35)
local lowX, lowY = center()
slider:SetValue(85)
local highX, highY = center()
assert(highX > lowX, "THUMB_RED: horizontal value increase must move thumb right")
local sliderX, sliderY = slider:GetCenter()
assert(near(lowY, sliderY) and near(highY, sliderY), "horizontal thumb must be vertically centered")
slider:SetValue(60)
local midX, midY = center()
assert(near(midX, sliderX) and near(midY, sliderY), "mid-range horizontal thumb must be centered")
slider:SetOrientation("VERTICAL")
slider:SetSize(40, 160)
slider:SetValue(60)
midX, midY = center()
sliderX, sliderY = slider:GetCenter()
assert(near(midX, sliderX) and near(midY, sliderY), "mid-range vertical thumb must be centered")
slider:SetValue(35)
lowX, lowY = center()
slider:SetValue(85)
highX, highY = center()
assert(highY < lowY, "THUMB_RED: vertical value increase must move thumb down")
assert(near(lowX, sliderX) and near(highX, sliderX), "vertical thumb must be horizontally centered")
slider:SetOrientation("HORIZONTAL")
slider:SetSize(160, 40)
slider:SetValue(60)
slider:SetMinMaxValues(10, 210)
local rangeX = center()
sliderX, sliderY = slider:GetCenter()
assert(rangeX < sliderX, "THUMB_RED: range change must update thumb geometry")
slider:SetMinMaxValues(10, 110)
slider:SetValue(35)
local beforeX = center()
slider:SetWidth(320)
local afterX = center()
assert(afterX < beforeX, "THUMB_RED: track resize must update off-center thumb")
thumb:SetSize(30, 20)
local left, bottom, width, height = thumb:GetRect()
assert(width == 30 and height == 20, "THUMB_RED: thumb resize must update cached dimensions")
slider:SetMinMaxValues(7, 7)
local zeroX, zeroY = center()
assert(zeroX == zeroX and zeroY == zeroY, "collapsed range must have finite geometry")
local ordinary = slider:CreateTexture()
ordinary:SetSize(18, 24)
assert(ordinary:GetRect() == nil, "ordinary unanchored texture remains unresolved")
print("FEEDBACK_THUMB_CONTRACT_PASS")
"#,
        )
        .unwrap();
}

#[test]
fn slider_thumb_rebinding_preserves_unresolved_and_authored_anchor_controls() {
    env()
        .exec(
            r#"
        local slider = CreateFrame("Slider", nil, UIParent)
        slider:SetSize(160, 40)
        slider:SetPoint("CENTER")
        slider:SetMinMaxValues(0, 100)
        slider:SetValue(50)
        local old = slider:GetThumbTexture()
        old:SetSize(18, 24)
        assert(old:GetLeft() ~= nil)
        local replacement = slider:CreateTexture()
        slider:SetThumbTexture(replacement)
        assert(old:GetRect() == nil, "unbound unanchored thumb loses implicit geometry")
        assert(replacement:GetRect() == nil, "unsized bound texture is not resolved geometry")
        replacement:SetSize(18, 24)
        local x, y = replacement:GetCenter()
        local sx, sy = slider:GetCenter()
        assert(math.abs(x-sx) < 0.01 and math.abs(y-sy) < 0.01)
        replacement:SetPoint("TOPLEFT", slider, "TOPLEFT", 3, -4)
        local left, bottom = replacement:GetRect()
        slider:SetValue(80)
        local nextLeft, nextBottom = replacement:GetRect()
        assert(left == nextLeft and bottom == nextBottom, "authored anchors remain authoritative")
        replacement:ClearAllPoints()
        local oldCenter = replacement:GetCenter()
        assert(oldCenter > slider:GetCenter(), "fixture starts above midpoint")
        local calls = 0
        local callbackCenter
        slider:SetScript("OnValueChanged", function()
            calls = calls + 1
            callbackCenter = replacement:GetCenter()
        end)
        slider:SetValue(20)
        assert(calls == 1)
        assert(type(callbackCenter) == "number" and callbackCenter < slider:GetCenter(),
            "callback must observe the new thumb position before SetValue returns")
    "#,
        )
        .unwrap();
}
