//! Original StoreUISecure consumer; hidden VAS controls have no presented cursor.
#![cfg(feature = "client-mists")]

use std::cell::RefCell;
use std::rc::Rc;

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

#[test]
fn hidden_editbox_keeps_text_notifications_and_presents_latest_cursor_when_shown() {
    let env = create_env();
    env.exec(
        r#"
        local panel = CreateFrame('Frame', nil, UIParent)
        panel:Hide()
        HiddenCursorPanel = panel
        local box = CreateFrame('EditBox', nil, panel)
        HiddenCursorBox = box
        box:SetAutoFocus(false)
        box:SetSize(120, 20)
        box:SetPoint('CENTER')
        assert(box:SetFont('Fonts\\FRIZQT__.TTF', 14))
        HiddenCursorTrace = {}
        box:SetScript('OnCursorChanged', function(self, x, y, w, h)
            assert(self:IsVisible(), 'cursor requires visible presentation')
            assert(self:GetText() == 'latest' and self:GetCursorPosition() == 3)
            assert(type(x) == 'number' and type(y) == 'number' and w > 0 and h > 0)
            HiddenCursorTrace[#HiddenCursorTrace + 1] = 'cursor'
        end)
        box:SetScript('OnTextChanged', function(self, userInput)
            assert(not self:IsVisible() and not userInput)
            HiddenCursorTrace[#HiddenCursorTrace + 1] = 'text'
        end)
        box:SetText('latest')
        box:SetCursorPosition(3)
    "#,
    )
    .unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("assert(table.concat(HiddenCursorTrace, ',') == 'text'); HiddenCursorPanel:Show()")
        .unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("assert(table.concat(HiddenCursorTrace, ',') == 'text,cursor')")
        .unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("assert(table.concat(HiddenCursorTrace, ',') == 'text,cursor')")
        .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
}
