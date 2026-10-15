//! Shared clean-room cursor geometry contract, not native WoW geometry.
//! Uses installed simulator fonts through the existing backend and aliases.
//! No manual callback dispatch, synthetic geometry injection, or new geometry API.

use std::cell::RefCell;
use std::rc::Rc;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::font::{DEFAULT_WOW_FONT, WowFontSystem};

const TICK_SECONDS: f64 = 0.016;

fn create_env_with_installed_fonts() -> WowLuaEnv {
    let fonts = WowFontSystem::new_without_casc();
    assert!(
        fonts.font_system.db().faces().next().is_some(),
        "SETUP FAILURE: installed simulator font database has no face; not behavioral RED"
    );
    assert!(
        fonts.family_name(Some(DEFAULT_WOW_FONT)).is_some(),
        "SETUP FAILURE: existing default simulator font alias is unavailable"
    );
    let env = WowLuaEnv::new().expect("SETUP FAILURE: create real Lua environment");
    env.set_font_system(Rc::new(RefCell::new(fonts)));
    env.exec(CREATE_FIXTURE)
        .expect("SETUP FAILURE: create public EditBox and FontString fixtures");
    env
}

const CREATE_FIXTURE: &str = r#"
    CursorBox = CreateFrame('EditBox', 'C2CursorLayoutBox', UIParent)
    CursorBox:SetAutoFocus(false)
    CursorBox:SetMultiLine(true)
    CursorBox:SetSize(400, 120)
    CursorBox:SetPoint('TOPLEFT', UIParent, 'TOPLEFT', 113, -79)
    assert(CursorBox:SetFont('Fonts\\FRIZQT__.TTF', 20))
    CursorBox:SetTextInsets(5, 7, 3, 4)
    CursorBox:SetText('é\n猫z')
    CursorBox:SetCursorPosition(0)
    CursorBox:Show()

    local parent = CreateFrame('Frame', nil, UIParent)
    CursorEWidth = parent:CreateFontString(nil, 'ARTWORK')
    CursorCatWidth = parent:CreateFontString(nil, 'ARTWORK')
    assert(CursorEWidth:SetFont('Fonts\\FRIZQT__.TTF', 20))
    assert(CursorCatWidth:SetFont('Fonts\\FRIZQT__.TTF', 20))
    CursorEWidth:SetText('é')
    CursorCatWidth:SetText('猫')

    CursorRecords, CursorUpdateObservations = {}, {}
    CursorBox:SetScript('OnCursorChanged', function(self, ...)
        local x, y, width, height = ...
        CursorRecords[#CursorRecords + 1] = {
            x = x, y = y, width = width, height = height,
            arity = select('#', ...),
            byte = self:GetCursorPosition(),
            scalar = self:GetUTF8CursorPosition(), text = self:GetText(),
        }
    end)
    -- Small ordinary OnUpdate observer, NOT a cached ScrollingEdit consumer.
    CursorBox:SetScript('OnUpdate', function(self)
        CursorUpdateObservations[#CursorUpdateObservations + 1] = {
            record = CursorRecords[#CursorRecords], count = #CursorRecords,
            byte = self:GetCursorPosition(),
            scalar = self:GetUTF8CursorPosition(), text = self:GetText(),
        }
    end)

    function AssertCursorPending(count, byte, scalar)
        assert(#CursorRecords == count, 'cursor callback must wait for layout tick')
        assert(CursorBox:GetCursorPosition() == byte, 'public byte cursor committed')
        assert(CursorBox:GetUTF8CursorPosition() == scalar, 'public scalar cursor committed')
        assert(CursorBox:GetText() == 'é\n猫z')
    end

    function AssertCursorDelivery(count, byte, scalar, x, y)
        assert(#CursorRecords == count, 'exactly one delivery per changed tick')
        local record = CursorRecords[count]
        assert(record ~= nil and record.arity == 4, 'four geometry arguments')
        local function near(actual, expected, tolerance)
            assert(type(actual) == 'number', 'numeric geometry required')
            assert(math.abs(actual - expected) < (tolerance or 0.001),
                tostring(actual) .. ' != ' .. tostring(expected))
        end
        -- Library Edit::cursor_position returns integer logical coordinates;
        -- FontString advances can be fractional. Line starts stay exact.
        if x > 5 then
            near(record.x, x, 1)
            assert(record.x > 5, 'caret advances past the shaped glyph')
        else
            near(record.x, x)
        end
        near(record.y, y)
        near(record.width, 1.5)
        near(record.height, 24)
        assert(record.byte == byte and record.scalar == scalar)
        assert(record.text == 'é\n猫z', 'callback sees committed text')
        local observation = CursorUpdateObservations[#CursorUpdateObservations]
        assert(observation ~= nil, 'ordinary OnUpdate actually ran')
        assert(observation.count == count and observation.record == record,
            'cursor delivery must precede ordinary OnUpdate in this tick')
        assert(observation.byte == byte and observation.scalar == scalar)
        assert(observation.text == record.text)
        AssertCursorPending(count, byte, scalar)
    end
"#;

#[test]
fn programmatic_cursor_layout_delivers_before_onupdate_and_suppresses_unchanged_ticks() {
    let env = create_env_with_installed_fonts();
    env.exec("AssertCursorPending(0, 0, 0); assert(#CursorUpdateObservations == 0)")
        .unwrap();

    // First actual layout delivery must already be visible to the first OnUpdate.
    env.fire_on_update(TICK_SECONDS).unwrap();
    env.exec("AssertCursorDelivery(1, 0, 0, 5, -3); assert(#CursorUpdateObservations == 1)")
        .unwrap();

    env.exec("CursorBox:SetCursorPosition(3); AssertCursorPending(1, 3, 2)")
        .unwrap();
    env.fire_on_update(TICK_SECONDS).unwrap();
    // Proposed policy: local x = 5 + shaped line-start x (0);
    // local y = -(3 + shaped second-line top (24)). No glyph advance oracle needed.
    env.exec("AssertCursorDelivery(2, 3, 2, 5, -27); assert(#CursorUpdateObservations == 2)")
        .unwrap();

    env.exec("CursorBox:SetCursorPosition(3); AssertCursorPending(2, 3, 2)")
        .unwrap();
    for _ in 0..2 {
        env.fire_on_update(TICK_SECONDS).unwrap();
        env.exec("AssertCursorDelivery(2, 3, 2, 5, -27)").unwrap();
    }
    env.exec("assert(#CursorUpdateObservations == 4)").unwrap();
}

#[test]
fn keyboard_cursor_layout_crosses_multibyte_and_newline_boundaries_on_real_ticks() {
    let env = create_env_with_installed_fonts();
    env.exec("CursorBox:SetFocus(); AssertCursorPending(0, 0, 0)")
        .unwrap();
    env.fire_on_update(TICK_SECONDS).unwrap();
    env.exec("AssertCursorDelivery(1, 0, 0, 5, -3); assert(#CursorUpdateObservations == 1)")
        .unwrap();

    // Whole-document UTF-8 boundaries: 0 --é--> 2 --newline--> 3 --猫--> 6.
    // Glyph-dependent x uses public FontString measurement with the same backend.
    let movements = [
        ("RIGHT", 2, 1, "5 + CursorEWidth:GetStringWidth()", -3),
        ("RIGHT", 3, 2, "5", -27),
        ("RIGHT", 6, 3, "5 + CursorCatWidth:GetStringWidth()", -27),
        ("LEFT", 3, 2, "5", -27),
        ("LEFT", 2, 1, "5 + CursorEWidth:GetStringWidth()", -3),
        ("LEFT", 0, 0, "5", -3),
    ];
    for (index, (key, byte, scalar, x, y)) in movements.iter().enumerate() {
        let previous_count = index + 1;
        let delivered_count = previous_count + 1;
        env.send_key_press(key, None).unwrap();
        env.exec(&format!(
            "AssertCursorPending({previous_count}, {byte}, {scalar})"
        ))
        .unwrap();
        env.fire_on_update(TICK_SECONDS).unwrap();
        env.exec(&format!(
            "AssertCursorDelivery({delivered_count}, {byte}, {scalar}, {x}, {y}); \
             assert(#CursorUpdateObservations == {delivered_count})"
        ))
        .unwrap();
    }

    for _ in 0..2 {
        env.fire_on_update(TICK_SECONDS).unwrap();
        env.exec("AssertCursorDelivery(7, 0, 0, 5, -3)").unwrap();
    }
    env.exec("assert(#CursorUpdateObservations == 9)").unwrap();
}
