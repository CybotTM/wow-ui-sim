//! Staged source-only regressions: runtime RED/GREEN has not been executed.
#![cfg(any(feature = "client-era", feature = "client-anniversary"))]

use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_original(env: &WowLuaEnv, name: &str) {
    let path = wow_ui_sim::client_profile::blizzard_ui_addons_dir().join(name);
    let source = std::fs::read(&path).expect("SETUP: cached original consumer");
    let pins: serde_json::Value =
        serde_json::from_str(include_str!("classic_startup_namespaces.json")).unwrap();
    let profile = wow_ui_sim::client_profile::ACTIVE.cache_subdir();
    assert_eq!(
        format!("{:x}", Sha256::digest(&source)),
        pins[profile][name].as_str().unwrap(),
        "SETUP: cached consumer changed: {}",
        path.display()
    );
    env.loader_env()
        .exec(std::str::from_utf8(&source).expect("SETUP: consumer UTF-8"))
        .expect("SETUP: load complete original chunk");
}

fn consumer_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        ClassicNamespaceErrors = {}
        seterrorhandler(function(error)
            ClassicNamespaceErrors[#ClassicNamespaceErrors + 1] = tostring(error)
        end)
    "#,
    )
    .unwrap();
    env
}

fn assert_no_errors(env: &WowLuaEnv) {
    env.exec(r"assert(#ClassicNamespaceErrors == 0, table.concat(ClassicNamespaceErrors, '\n'))")
        .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
}

#[test]
fn classic_namespace_loot_onload_and_update_recycle_stale_rows() {
    let env = consumer_env();
    load_original(&env, "Blizzard_FrameXML/Classic/LootHistory.lua");
    env.exec(
        r#"
        local loot = CreateFrame('Frame')
        LootHistoryFrame_OnLoad(loot)
        assert(#loot.itemFrames == 0 and #loot.usedPlayerFrames == 0)
        assert(loot:IsEventRegistered('LOOT_HISTORY_FULL_UPDATE'))
        loot:SetScript('OnEvent', LootHistoryFrame_OnEvent)

        local staleItem = CreateFrame('Frame', nil, loot)
        local stalePlayer = CreateFrame('Frame', nil, loot)
        staleItem:Show()
        stalePlayer:Show()
        stalePlayer.itemIdx, stalePlayer.playerIdx = 1, 2
        loot.itemFrames[1] = staleItem
        loot.usedPlayerFrames[1] = stalePlayer
        FireEvent('LOOT_HISTORY_FULL_UPDATE')
        assert(not staleItem:IsShown() and not stalePlayer:IsShown())
        assert(#loot.usedPlayerFrames == 0)
        assert(loot.unusedPlayerFrames[1] == stalePlayer)
        assert(stalePlayer.itemIdx == nil and stalePlayer.playerIdx == nil)
        FireEvent('LOOT_HISTORY_ROLL_COMPLETE')
        assert(#loot.unusedPlayerFrames == 1)
    "#,
    )
    .unwrap();
    assert_no_errors(&env);
}

fn load_product_choice(env: &WowLuaEnv) {
    load_original(env, "Blizzard_SharedXML/Classic/SharedUIPanelTemplates.lua");
    load_original(env, "Blizzard_UIPanels_Game/Classic/ProductChoice.lua");
}

#[test]
fn classic_namespace_product_onload_initializes_empty_reward_panel() {
    let env = consumer_env();
    load_product_choice(&env);
    env.exec(
        r#"
        local product = CreateFrame('Frame')
        product.TitleText = product:CreateFontString()
        product.portrait = product:CreateTexture()
        product.PortraitFrame = product:CreateTexture()
        product.TopLeftCorner = product:CreateTexture()
        product.TopBorder = product:CreateTexture()
        product.LeftBorder = product:CreateTexture()
        product.portrait:Show()
        product.PortraitFrame:Show()
        product.TopLeftCorner:Hide()
        RECRUIT_A_FRIEND_REWARDS = 'Fixture rewards'
        ProductChoiceFrame_OnLoad(product)
        assert(product.selectedPageNum == 1)
        assert(product.TitleText:GetText() == 'Fixture rewards')
        assert(not product.portrait:IsShown() and not product.PortraitFrame:IsShown())
        assert(product.TopLeftCorner:IsShown())
        assert(product:IsEventRegistered('PRODUCT_CHOICE_UPDATE'))
        assert(product.mainAlertFrame == nil and product.secondAlertFrame == nil)
    "#,
    )
    .unwrap();
    assert_no_errors(&env);
}

#[test]
fn classic_namespace_product_login_and_update_complete_without_reward_alerts() {
    let env = consumer_env();
    load_product_choice(&env);
    env.exec(
        r#"
        local product = CreateFrame('Frame')
        product.completedEvents = 0
        product:RegisterEvent('PLAYER_LOGIN')
        product:RegisterEvent('PRODUCT_CHOICE_UPDATE')
        product:SetScript('OnEvent', function(self, event, ...)
            ProductChoiceFrame_OnEvent(self, event, ...)
            self.completedEvents = self.completedEvents + 1
        end)
        FireEvent('PLAYER_LOGIN')
        FireEvent('PRODUCT_CHOICE_UPDATE')
        assert(product.completedEvents == 2)
        assert(product.mainAlertFrame == nil and product.secondAlertFrame == nil)
        product.selectedData = { id = 117 }
        ProductChoiceFrame_SetUp(product)
        assert(product.selectedData == nil and product.choiceID == nil)
        local first, second = C_ProductChoice.GetChoices(), C_ProductChoice.GetChoices()
        first[1] = 117
        assert(#second == 0)
    "#,
    )
    .unwrap();
    assert_no_errors(&env);
}

#[test]
fn classic_namespace_rune_onload_hides_initially_shown_control() {
    let env = consumer_env();
    load_original(&env, "Blizzard_CharacterFrame/Vanilla/PaperDollFrame.lua");
    env.exec(
        r#"
        local button = CreateFrame('CheckButton')
        button:Show()
        assert(button:IsShown())
        RuneFrameControlButton_OnLoad(button)
        assert(not button:IsShown())
        ToggleEngravingFrame()
        assert(EngravingFrame == nil)
    "#,
    )
    .unwrap();
    assert_no_errors(&env);
}

#[test]
fn classic_namespace_unknown_members_stay_nil_without_changing_other_factories() {
    let env = consumer_env();
    env.exec(
        r#"
        for _, name in ipairs({'C_LootHistory', 'C_ProductChoice', 'C_Engraving'}) do
            local namespace = rawget(_G, name)
            assert(type(namespace) == 'table', name)
            assert(namespace.NotARealMethod == nil)
            assert(rawget(namespace, 'NotARealMethod') == nil)
            assert(__wow_namespace(namespace) == namespace)
            assert(namespace.NotARealMethod == nil)
        end
        -- Known existing namespace: its member factory must remain unchanged.
        local method = C_CVar.NotARealMethod
        assert(type(method) == 'function' and C_CVar.NotARealMethod == method)
        assert(select('#', method()) == 1 and method() == nil)
        local unrelated = __wow_namespace()
        assert(type(unrelated.NotARealMethod) == 'function')
        assert(select('#', unrelated.NotARealMethod()) == 1)
    "#,
    )
    .unwrap();
    assert_no_errors(&env);
}
