//! F05 native callbacks. Contracts derive from the committed 12340 FrameXML.
//! Modern-only inventory remains in the original tests, not synthesized here.
use super::super::wrath_env;
use std::time::{Duration, Instant};
use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_events(frame: &str, events: &[&str]) {
    let env = wrath_env();
    for event in events {
        env.exec(&format!(
            "assert({frame}:IsEventRegistered({event:?}), {event:?}); assert({frame}:GetScript('OnEvent'))"
        ))
        .unwrap();
    }
}

pub fn load_errors() {
    let env = wrath_env();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
}

pub fn main_events() {
    assert_events(
        "MainMenuBar",
        &[
            "PLAYER_ENTERING_WORLD",
            "BAG_UPDATE",
            "ACTIONBAR_PAGE_CHANGED",
            "KNOWN_CURRENCY_TYPES_UPDATE",
            "CURRENCY_DISPLAY_UPDATE",
            "ADDON_LOADED",
            "UNIT_ENTERING_VEHICLE",
            "UNIT_ENTERED_VEHICLE",
            "UNIT_EXITING_VEHICLE",
            "UNIT_EXITED_VEHICLE",
        ],
    );
}

pub fn vehicle_leave_events() {
    assert_events(
        "MainMenuBarVehicleLeaveButton",
        &[
            "UPDATE_BONUS_ACTIONBAR",
            "UPDATE_MULTI_CAST_ACTIONBAR",
            "VEHICLE_UPDATE",
        ],
    );
}

pub fn pet_events() {
    assert_events(
        "PetActionBarFrame",
        &[
            "PLAYER_CONTROL_LOST",
            "PLAYER_CONTROL_GAINED",
            "PLAYER_FARSIGHT_FOCUS_CHANGED",
            "UNIT_PET",
            "UNIT_FLAGS",
            "UNIT_AURA",
            "PET_BAR_UPDATE",
            "PET_BAR_UPDATE_COOLDOWN",
            "PET_BAR_SHOWGRID",
            "PET_BAR_HIDEGRID",
            "PET_BAR_HIDE",
            "PET_BAR_UPDATE_USABLE",
        ],
    );
}

pub fn stance_events() {
    assert_events(
        "ShapeshiftBarFrame",
        &[
            "PLAYER_ENTERING_WORLD",
            "UPDATE_SHAPESHIFT_FORMS",
            "UPDATE_SHAPESHIFT_USABLE",
            "UPDATE_SHAPESHIFT_COOLDOWN",
            "UPDATE_SHAPESHIFT_FORM",
            "UPDATE_INVENTORY_ALERTS",
            "ACTIONBAR_PAGE_CHANGED",
        ],
    );
}

pub fn main_identity() {
    wrath_env()
        .exec(
            r#"
        assert(MainMenuBar:GetObjectType() == "Frame")
        assert(MainMenuBar:GetParent() == UIParent)
        assert(MainMenuBarArtFrame:GetParent() == MainMenuBar)
        assert(MainMenuBar:GetWidth() == 1024 and MainMenuBar:GetHeight() == 53)
    "#,
        )
        .unwrap();
}

pub fn main_buttons() {
    wrath_env()
        .exec(
            r#"
        for i = 1, 12 do
            local button = assert(_G["ActionButton"..i])
            assert(button:GetObjectType() == "CheckButton" and button:GetID() == i)
            assert(button:GetParent() == MainMenuBarArtFrame)
            assert(_G[button:GetName().."Icon"]:GetParent() == button)
            assert(button:GetScript("OnEvent") and button:GetScript("OnClick"))
        end
    "#,
        )
        .unwrap();
}

pub fn main_alias_absence() {
    wrath_env()
        .exec(
            r#"
        for i = 1, 12 do
            assert(_G["MainActionBarButton"..i] == nil)
            assert(_G["ActionButton"..i])
        end
    "#,
        )
        .unwrap();
}

pub fn multibar_identity() {
    wrath_env().exec(r#"
        for _, name in ipairs({"MultiBarBottomLeft", "MultiBarBottomRight", "MultiBarLeft", "MultiBarRight"}) do
            local bar = assert(_G[name])
            assert(bar:GetObjectType() == "Frame" and bar:GetParent() == UIParent)
            assert(bar:GetScript("OnLoad"))
        end
    "#).unwrap();
}

pub fn multibar_buttons() {
    wrath_env().exec(r#"
        for _, name in ipairs({"MultiBarBottomLeft", "MultiBarBottomRight", "MultiBarLeft", "MultiBarRight"}) do
            for i = 1, 12 do
                local button = assert(_G[name.."Button"..i])
                assert(button:GetObjectType() == "CheckButton" and button:GetID() == i)
                assert(button:GetParent() == _G[name])
                assert(button:GetScript("OnEvent") and button:GetScript("OnClick"))
            end
        end
    "#).unwrap();
}

pub fn input_globals() {
    wrath_env().exec(r#"
        for _, name in ipairs({"ActionButtonDown", "ActionButtonUp", "MultiActionButtonDown", "MultiActionButtonUp", "ActionBar_PageUp", "ActionBar_PageDown"}) do
            assert(type(_G[name]) == "function", name)
            local source = debug.getinfo(_G[name], "S").source
            assert(string.find(source, "ActionButton.lua", 1, true) or string.find(source, "MultiActionBars.lua", 1, true), source)
        end
    "#).unwrap();
}

pub fn multibar_globals() {
    wrath_env().exec(r#"
        for _, name in ipairs({"MultiActionBar_Update", "MultiActionBar_ShowAllGrids", "MultiActionBar_HideAllGrids"}) do
            assert(type(_G[name]) == "function", name)
            assert(string.find(debug.getinfo(_G[name], "S").source, "MultiActionBars.lua", 1, true))
        end
    "#).unwrap();
}

pub fn page_event() {
    let env = wrath_env();
    env.exec("ChangeActionBarPage(1); assert(ActionButton1.action == 1)")
        .unwrap();
    env.exec("ChangeActionBarPage(3)").unwrap();
    env.exec("assert(ActionButton1.action == 25 and MainMenuBarPageNumber:GetText() == '3')")
        .unwrap();
    env.exec("ChangeActionBarPage(1); assert(ActionButton1.action == 1)")
        .unwrap();
}

pub fn stance_callbacks() {
    let env = wrath_env();
    for event in [
        "UPDATE_SHAPESHIFT_FORM",
        "UPDATE_SHAPESHIFT_FORMS",
        "UPDATE_SHAPESHIFT_USABLE",
    ] {
        env.exec("ShapeshiftButton1:SetChecked(false)").unwrap();
        env.fire_event(event).unwrap();
        env.exec(
            r#"
            local _, _, active = GetShapeshiftFormInfo(1)
            assert(ShapeshiftButton1:GetChecked() == (active and true or false))
            assert(ShapeshiftButton1Icon:IsShown())
        "#,
        )
        .unwrap();
    }
}

pub fn vehicle_power() {
    let env = wrath_env();
    env.exec("VehicleMenuBarPowerBar:SetValue(0)").unwrap();
    env.fire_event_with_args("UNIT_DISPLAYPOWER", &[env.lua_string("player")])
        .unwrap();
    env.exec(
        r#"
        assert(VehicleMenuBar:IsEventRegistered("UNIT_DISPLAYPOWER"))
        local _, maximum = VehicleMenuBarPowerBar:GetMinMaxValues()
        assert(maximum == UnitPowerMax("vehicle"))
        assert(VehicleMenuBarPowerBar:GetValue() == UnitPower("vehicle"))
    "#,
    )
    .unwrap();
}

pub fn action_down() {
    let env = wrath_env();
    env.exec(
        r#"
        assert(ActionButton1:GetButtonState() == "NORMAL")
        ActionButtonDown(1)
        assert(ActionButton1:GetButtonState() == "PUSHED")
        ActionButtonUp(1)
        assert(ActionButton1:GetButtonState() == "NORMAL")
    "#,
    )
    .unwrap();
}

pub fn casting_visible() {
    let env = wrath_env();
    env.exec(
        r#"
        assert(CastingBarFrame.unit == "player")
        assert(CastingBarFrame:IsEventRegistered("UNIT_SPELLCAST_START"))
        TargetUnit("party1")
        UseAction(1)
        assert(CastingBarFrame.casting and CastingBarFrame:IsShown(), "native cast did not start")
        assert(CastingBarFrameText:GetText() == "Flash of Light")
        assert(CastingBarFrame.maxValue > 0)
    "#,
    )
    .unwrap();
}

pub fn casting_delay_failure() {
    let env = wrath_env();
    env.exec(
        r#"
        ClearTarget()
        CastSpellByID(19750)
        local bar = CastingBarFrame
        assert(bar.casting and bar:IsShown(), "native cast did not start")
        local maximum = bar.maxValue
        assert(A_Admin.DelayCasting(0.75))
        assert(math.abs(bar.maxValue - maximum - 0.75) < 0.000001, "native delay not reflected")
        assert(A_Admin.FailCasting())
        assert(not bar.casting, "native failure did not clear casting")
        assert(CastingBarFrameText:GetText() == FAILED)
        assert(UnitCastingInfo("player") == nil)
        CastSpellByID(19750)
        assert(bar.casting, "replacement cast did not start")
        assert(A_Admin.FailCasting(true))
        assert(not bar.casting and UnitCastingInfo("player") == nil)
        assert(CastingBarFrameText:GetText() == INTERRUPTED)
    "#,
    )
    .unwrap();
}

pub fn aura_cast_refresh() {
    let env = wrath_env();
    env.exec(
        r#"
        A_Admin.ClearBuffs()
        BuffFrame_Update()
        assert(BUFF_ACTUAL_DISPLAY == 0)
        CastSpellByID(465)
        assert(UnitBuff("player", 1) == "Devotion Aura")
        assert(BUFF_ACTUAL_DISPLAY > 0, "native UNIT_AURA did not refresh buffs")
        assert(BuffButton1:IsShown() and BuffButton1.unit == "player")
        assert(BuffButton1Icon:GetTexture() == select(3, UnitBuff("player", 1)))
    "#,
    )
    .unwrap();
}

fn seed_timed_aura(env: &WowLuaEnv) {
    env.exec(
        r#"
        A_Admin.ClearBuffs()
        A_Admin.AddBuff(465, "F05 timed aura", 136001, 300, 0)
        SHOW_BUFF_DURATIONS = "1"
        BuffFrame_Update()
        local button = assert(BuffButton1)
        assert(button.duration and button:GetScript("OnUpdate"))
        assert(button.timeLeft > 60, "timed supplied aura required")
        button:GetScript("OnUpdate")(button, 0)
    "#,
    )
    .unwrap();
}

pub fn aura_duration() {
    let env = wrath_env();
    seed_timed_aura(&env);
    env.exec(r#"
        local button = BuffButton1
        local handler = button:GetScript("OnUpdate")
        handler(button, 0)
        local text = button.duration:GetText()
        handler(button, 0)
        assert(button.duration:GetText() == text and button.duration:IsShown())
        button.timeLeft = 30
        handler(button, 0)
        assert(button.duration:GetText() == SecondsToTimeAbbrev(30))
        local r, g, b = button.duration:GetTextColor()
        assert(r == HIGHLIGHT_FONT_COLOR.r and g == HIGHLIGHT_FONT_COLOR.g and b == HIGHLIGHT_FONT_COLOR.b)
        assert(button:GetAlpha() == BuffFrame.BuffAlphaValue)
        SHOW_BUFF_DURATIONS = "0"
        handler(button, 0)
        assert(not button.duration:IsShown())
    "#).unwrap();
}

pub fn aura_perf() {
    let env = wrath_env();
    seed_timed_aura(&env);
    let budget = Duration::from_micros(500);
    for _ in 0..8 {
        let start = Instant::now();
        env.exec(
            r#"
            local button = BuffButton1
            local handler = button:GetScript("OnUpdate")
            for i = 1, 256 do handler(button, 0) end
        "#,
        )
        .unwrap();
        let cost = start.elapsed() / 256;
        eprintln!("native AuraButton_OnUpdate per tick: {cost:?}, budget: {budget:?}");
        assert!(cost <= budget);
    }
}
