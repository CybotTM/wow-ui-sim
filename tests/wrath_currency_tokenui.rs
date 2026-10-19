//! B5: original cached Wrath consumers with supplied categories and real widgets.
#![cfg(feature = "client-wrath")]

use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::globals::real::wrath_currency::{CurrencyCategory, CurrencyToken};

fn token(name: &str, count: i32, item_id: i32, watched: bool) -> CurrencyToken {
    CurrencyToken {
        name: name.into(),
        count,
        item_id: Some(item_id),
        watched,
        extra_currency_type: 0,
        icon: "Interface\\Icons\\INV_Misc_Token_ArgentDawn".into(),
    }
}

fn supply_categories(env: &WowLuaEnv) {
    env.state().borrow_mut().wrath_currency.categories = vec![
        CurrencyCategory {
            name: "Dungeon and Raid".into(),
            expanded: true,
            currencies: vec![
                token("Emblem of Frost", 17, 49426, true),
                token("Emblem of Triumph", 0, 47241, false),
                token("Emblem of Conquest", 23, 45624, true),
            ],
        },
        CurrencyCategory {
            name: "PvP".into(),
            expanded: true,
            currencies: vec![token("Wintergrasp Mark", 100001, 43307, false)],
        },
    ];
}

fn load_original(env: &WowLuaEnv, path: &str, expected_sha256: &str) {
    let path = dirs::cache_dir().unwrap().join(path);
    let path = path.to_str().unwrap();
    let bytes = std::fs::read(path).expect("SETUP: original cache file required");
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        expected_sha256,
        "SETUP: cached source drift: {path}"
    );
    let source = std::str::from_utf8(&bytes).expect("SETUP: cached source UTF-8");
    env.exec_named(source, &format!("@{path}"))
        .expect("SETUP: load entire original Lua file");
}

fn create_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("isolated environment");
    supply_categories(&env);
    env.exec(
        "UIPanelWindows = {}; SlashCmdList = {}; BoundaryErrors = {}; \
        seterrorhandler(function(e) BoundaryErrors[#BoundaryErrors+1] = tostring(e) end)",
    )
    .expect("SETUP: outer globals and error capture");
    load_original(
        &env,
        "wow-ui-sim/blizzard-ui/wrath/FrameXML/HybridScrollFrame.lua",
        "8bac3f6aec8d06d30f5e4fa31847972b8919534a76da35aa4504db72f10471cc",
    );
    load_original(
        &env,
        "wow-ui-sim/blizzard-ui/wrath/AddOns/Blizzard_TokenUI/Blizzard_TokenUI.lua",
        "bbea8468b25b88360b8c6ecbce0d52c0858dbd89aa2e60b21fc352e0d0c745fa",
    );
    env.exec(include_str!("fixtures/wrath_currency_tokenui.lua"))
        .expect("SETUP: supplied widget tree, no currency API replacements");
    env
}

fn assert_consumer(env: &WowLuaEnv, probe: &str) {
    let result = env.exec(probe);
    env.exec("assert(#BoundaryErrors == 0, table.concat(BoundaryErrors, '\\n'))")
        .expect("no captured callback errors");
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "no recorded Lua errors"
    );
    result.expect("original TokenUI behavioral assertions");
}

#[test]
fn wrath_tokenui_original_update_projects_supplied_header_and_entry_tuples() {
    let env = create_env();
    assert_consumer(
        &env,
        r#"
        assert(GetCurrencyListSize() == 6, 'six supplied display rows')
        assert(select('#', GetCurrencyListInfo(2)) == 9, 'legacy tuple, not modern table')
        local name, header, expanded, unused, watched, count, extra, icon, item = GetCurrencyListInfo(2)
        assert(name == 'Emblem of Frost' and not header and not expanded and not unused)
        assert(watched and count == 17 and extra == 0 and item == 49426)
        assert(icon == 'Interface\\Icons\\INV_Misc_Token_ArgentDawn')
        TokenFrame_Update()
        local rows = TokenFrameContainer.buttons
        assert(rows[1]:GetText() == 'Dungeon and Raid' and rows[1].isHeader and rows[1].isExpanded)
        assert(rows[2].name:GetText() == 'Emblem of Frost' and rows[2].count:GetText() == '17')
        assert(rows[2].itemID == 49426 and rows[2].check:IsShown())
        assert(rows[3].name:GetText() == 'Emblem of Triumph' and rows[3].count:GetText() == '0')
        assert(not rows[3].check:IsShown())
        assert(rows[5]:GetText() == 'PvP' and rows[6].itemID == 43307)
        assert(CharacterFrameTab5:IsShown() and TokenFrameContainer.scrollChild:GetHeight() == 120)
    "#,
    );
}

#[test]
fn wrath_tokenui_original_header_click_rebuilds_shifted_display_indices() {
    let env = create_env();
    assert_consumer(
        &env,
        r#"
        TokenFrame_Update()
        TokenFrame.selectedToken = 'Emblem of Frost'
        TokenFramePopup:Show()
        TokenButton_OnClick(TokenFrameContainer.buttons[1])
        assert(GetCurrencyListSize() == 3, 'first category children hidden')
        assert(select(3, GetCurrencyListInfo(1)) == false, 'numeric zero collapses')
        assert(GetCurrencyListInfo(2) == 'PvP' and GetCurrencyListInfo(3) == 'Wintergrasp Mark')
        assert(not TokenFramePopup:IsShown() and not TokenFrameContainer.buttons[4]:IsShown())
        TokenButton_OnClick(TokenFrameContainer.buttons[2])
        assert(GetCurrencyListSize() == 2, 'second header now at display index two')
        TokenButton_OnClick(TokenFrameContainer.buttons[2])
        assert(GetCurrencyListSize() == 3, 'numeric one expands')
        TokenButton_OnClick(TokenFrameContainer.buttons[1])
        assert(GetCurrencyListSize() == 6 and GetCurrencyListInfo(5) == 'PvP')
        assert(TokenFrameContainer.buttons[6].name:GetText() == 'Wintergrasp Mark')
        assert(select('#', ExpandCurrencyList(1, 0)) == 0, 'mutator returns zero values')
        assert(GetCurrencyListSize() == 3)
    "#,
    );
}

#[test]
fn wrath_tokenui_original_modified_click_caps_three_and_watches_shifted_row() {
    let env = create_env();
    env.state().borrow_mut().modifier_keys.shift = true;
    assert_consumer(
        &env,
        r#"
        assert(IsModifiedClick('TOKENWATCHTOGGLE'))
        TokenFrame_Update()
        BackpackTokenFrame_Update()
        assert(GetNumWatchedTokens() == 2)
        TokenButton_OnClick(TokenFrameContainer.buttons[3])
        assert(GetNumWatchedTokens() == 3 and TokenFrameContainer.buttons[3].check:IsShown())
        assert(TokenFramePopupBackpackCheckBox:GetChecked())
        assert(GetBackpackCurrencyInfo(1) == 'Emblem of Frost')
        assert(GetBackpackCurrencyInfo(2) == 'Emblem of Triumph')
        assert(GetBackpackCurrencyInfo(3) == 'Emblem of Conquest')
        TokenButton_OnClick(TokenFrameContainer.buttons[6])
    "#,
    );
    // Native Wrath UIErrorsFrame inserts its queued MessageFrame row on update.
    env.fire_on_update(0.0).unwrap();
    assert_consumer(
        &env,
        r#"
        assert(UIErrorsFrame:GetNumMessages() == 1, 'original UI rejects fourth watch')
        assert(UIErrorsFrame:GetMessageInfo(1) == 'Only 3 currencies may be watched')
        assert(select(5, GetCurrencyListInfo(6)) == false)
        TokenButton_OnClick(TokenFrameContainer.buttons[3])
        assert(GetNumWatchedTokens() == 2 and not TokenFrameContainer.buttons[3].check:IsShown())
        TokenButton_OnClick(TokenFrameContainer.buttons[1])
        TokenButton_OnClick(TokenFrameContainer.buttons[3])
        assert(GetNumWatchedTokens() == 3 and select(5, GetCurrencyListInfo(3)))
        assert(GetBackpackCurrencyInfo(3) == 'Wintergrasp Mark', 'watch display row, not catalog position')
        assert(BackpackTokenFrameToken3.count:GetText() == '*' and BackpackTokenFrameToken3.itemID == 43307)
        assert(BackpackTokenFrame:GetParent() == B5Backpack)
        assert(B5Backpack:GetHeight() == BACKPACK_HEIGHT + BACKPACK_TOKENFRAME_HEIGHT)
        TokenButton_OnClick(TokenFrameContainer.buttons[2])
        assert(GetCurrencyListSize() == 2 and GetBackpackCurrencyInfo(3) == 'Wintergrasp Mark')
    "#,
    );
}

#[test]
fn wrath_tokenui_backpack_slots_one_to_three_are_independent_of_expansion() {
    let env = create_env();
    assert_consumer(
        &env,
        r#"
        local modernSize = C_CurrencyInfo.GetCurrencyListSize()
        local modernWatch = C_CurrencyInfo.GetCurrencyInfo(2245).isShowInBackpack
        assert(select('#', SetCurrencyBackpack(3, 1)) == 0)
        assert(select('#', SetCurrencyBackpack(6, 1)) == 0)
        assert(select('#', SetCurrencyBackpack(4, 0)) == 0)
        ExpandCurrencyList(1, 0)
        ExpandCurrencyList(2, 0)
        BackpackTokenFrame_Update()
        local names = {'Emblem of Frost', 'Emblem of Triumph', 'Wintergrasp Mark'}
        local counts = {17, 0, 100001}
        local items = {49426, 47241, 43307}
        for i = 1, 3 do
            assert(select('#', GetBackpackCurrencyInfo(i)) == 5)
            local name, count, extra, icon, item = GetBackpackCurrencyInfo(i)
            assert(name == names[i] and count == counts[i] and extra == 0 and item == items[i])
            assert(icon == 'Interface\\Icons\\INV_Misc_Token_ArgentDawn')
            local button = _G['BackpackTokenFrameToken'..i]
            assert(button:IsShown() and button.itemID == items[i])
        end
        assert(BackpackTokenFrameToken1.count:GetText() == '17')
        assert(BackpackTokenFrameToken2.count:GetText() == '0')
        assert(BackpackTokenFrameToken3.count:GetText() == '*')
        assert(C_CurrencyInfo.GetCurrencyListSize() == modernSize)
        assert(C_CurrencyInfo.GetCurrencyInfo(2245).isShowInBackpack == modernWatch)
    "#,
    );
}
