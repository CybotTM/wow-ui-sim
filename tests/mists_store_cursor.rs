//! Cursor lifecycle derived from shipped consumer contracts (original-code inference).
//! No direct native observation.
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "client-mists")]
use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::font::{DEFAULT_WOW_FONT, WowFontSystem};

fn create_env() -> WowLuaEnv {
    let fonts = WowFontSystem::new_without_casc();
    assert!(fonts.family_name(Some(DEFAULT_WOW_FONT)).is_some());
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1600.0, 1200.0);
    env.set_font_system(Rc::new(RefCell::new(fonts)));
    env
}

#[cfg(feature = "client-mists")]
#[test]
fn original_mists_vas_cursor_does_not_run_before_hidden_character_selection() {
    let env = create_env();
    let path = wow_ui_sim::client_profile::blizzard_ui_addons_dir()
        .join("Blizzard_StoreUI/TBC/Blizzard_StoreUISecure.lua");
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "7f48cac9b28e29ed632dc5dc8486f5d11cc93fe7c481445d9c193ccc310027c8"
    );
    env.loader_env()
        .exec(std::str::from_utf8(&bytes).unwrap())
        .expect("load complete unmodified StoreUISecure original consumer");
    env.exec(r#"
        assert(type(VASCharacterSelectionTransferRealmEditBox_OnCursorChanged) == 'function')
        assert(#C_StoreSecure.GetVASRealmList() > 0, 'realm fixture must not mask nil character')
        StoreCursorErrors = {}
        seterrorhandler(function(message)
            StoreCursorErrors[#StoreCursorErrors + 1] = tostring(message)
        end)
        -- StoreUISecure keeps SelectedRealm/SelectedCharacter nil until selection.
        -- Both locally hidden and ancestor-hidden EditBoxes occur before that.
        local panel = CreateFrame('Frame', nil, UIParent)
        panel:Hide()
        for _, parent in ipairs({panel, UIParent}) do
            local box = CreateFrame('EditBox', nil, parent)
            box:SetAutoFocus(false)
            box:SetSize(120, 20)
            box:SetPoint('CENTER')
            assert(box:SetFont('Fonts\\FRIZQT__.TTF', 14))
            box:SetScript('OnCursorChanged', VASCharacterSelectionTransferRealmEditBox_OnCursorChanged)
            if parent == UIParent then box:Hide() end
            assert(not box:IsVisible())
        end
    "#).unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("assert(#StoreCursorErrors == 0, table.concat(StoreCursorErrors, '\\n'))")
        .expect("hidden VAS cursor must not enter uninitialized character autocomplete");
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
}

// Visible XML retains initial presentation; Wrath hidden edits still need a cursor
// before DebugTools.lua:449 calls UIPanelTemplates.lua:342/365 directly.
#[test]
fn pristine_hidden_cursor_defers_to_first_show_but_hidden_edits_present_before_text_change() {
    let env = create_env();
    env.exec(r#"
        local panel = CreateFrame('Frame', nil, UIParent)
        panel:Hide()
        DeferredPanel = panel
        local box = CreateFrame('EditBox', nil, panel)
        DeferredBox = box
        box:SetAutoFocus(false)
        box:SetSize(120, 20)
        box:SetPoint('CENTER')
        assert(box:SetFont('Fonts\\FRIZQT__.TTF', 14))
        DeferredTrace = {}
        box:SetScript('OnCursorChanged', function(self, x, y, w, h)
            assert(type(x) == 'number' and type(y) == 'number' and w > 0 and h > 0)
            DeferredTrace[#DeferredTrace + 1] = 'cursor:' .. self:GetText() .. ':' .. self:GetCursorPosition()
        end)
        box:SetScript('OnTextChanged', function(self, userInput)
            assert(not userInput)
            DeferredTrace[#DeferredTrace + 1] = 'text:' .. self:GetText()
        end)
        box:SetScript('OnUpdate', function(self)
            DeferredTrace[#DeferredTrace + 1] = 'update'
        end)
    "#).unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("assert(#DeferredTrace == 0); DeferredPanel:Show(); assert(#DeferredTrace == 0)")
        .unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec(
        r#"
        assert(table.concat(DeferredTrace, ',') == 'cursor::0,update')
        DeferredTrace = {}
        DeferredPanel:Hide()
        DeferredBox:SetText('latest')
        DeferredBox:SetCursorPosition(3)
    "#,
    )
    .unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("assert(table.concat(DeferredTrace, ',') == 'cursor:latest:3,text:latest'); DeferredTrace = {}").unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("assert(#DeferredTrace == 0)").unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
}

// BasicControls.xml:31 deliberately keeps DebugTools hidden; its real SetText
// at Blizzard_DebugTools.lua:445 must still produce cursorOffset before :449.
#[test]
fn never_shown_box_with_first_real_text_gets_hidden_cursor_before_text_change() {
    let env = create_env();
    env.exec(
        r#"
        local box = CreateFrame('EditBox', nil, UIParent)
        box:SetAutoFocus(false)
        box:Hide()
        box:SetSize(120, 20)
        box:SetPoint('CENTER')
        assert(box:SetFont('Fonts\\FRIZQT__.TTF', 14))
        HiddenEditTrace = {}
        box:SetScript('OnCursorChanged', function(self)
            assert(not self:IsVisible() and self:GetText() == 'first')
            HiddenEditTrace[#HiddenEditTrace + 1] = 'cursor'
        end)
        box:SetScript('OnTextChanged', function(self, userInput)
            assert(not self:IsVisible() and not userInput)
            HiddenEditTrace[#HiddenEditTrace + 1] = 'text'
        end)
        box:SetText('first')
    "#,
    )
    .unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("assert(table.concat(HiddenEditTrace, ',') == 'cursor,text')")
        .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
}
