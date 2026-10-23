//! Shape-only contracts; native semantics unknown. Source predictions are not runtime proof.
#![cfg(any(feature = "client-wowforever"))]

#[path = "common/cf2_shape.rs"]
mod shape;

// wowforever: Blizzard_APIDocumentationGenerated/PingManagerDocumentation.lua:44 (IsPingSystemEnabled).
// Source-predicted FAIL: IsPingSystemEnabled missing; no availability, option-count or CVar mapping; not executed; native semantics unknown.
#[cfg(any(feature = "client-wowforever"))]
#[test]
fn original_ping_manager_shape_blocker() {
    let env = shape::load(&["Blizzard_Menu", "Blizzard_CompactRaidFrames"]);
    shape::check(
        &env,
        r#"

scalar('boolean', C_Ping.IsPingSystemEnabled())
original(CompactRaidFrameManager_OnLoad, 'Blizzard_CompactRaidFrames/Mainline/Blizzard_CompactRaidFrameManager.lua')
CompactRaidFrameManager_OnLoad(CompactRaidFrameManager)

"#,
    );
}

// wowforever: Blizzard_APIDocumentationGenerated/PaperDollInfoDocumentation.lua:11 (AmmoNeeded).
// wowforever: Blizzard_APIDocumentationGenerated/PaperDollInfoDocumentation.lua:258 (IsRangedSlotShown).
// Source-predicted FAIL: AmmoNeeded missing; ranged boolean already shape-compatible, no visibility assertions; not executed; native semantics unknown.
#[cfg(any(feature = "client-wowforever"))]
#[test]
fn original_camelot_equipment_predicates_shape_blocker() {
    let env = shape::load(&["Blizzard_UIPanels_Game"]);
    shape::check(
        &env,
        r#"

scalar('boolean', C_PaperDollInfo.IsRangedSlotShown())
scalar('boolean', C_PaperDollInfo.AmmoNeeded())
original(PaperDollItemSlotButton_OnLoad, 'Blizzard_UIPanels_Game/Camelot/PaperDollFrame.lua')
assert(CharacterAmmoSlot and CharacterMainHandSlot, 'original equipment slot prerequisites')
PaperDollItemSlotButton_OnLoad(CharacterAmmoSlot)
PaperDollItemSlotButton_OnLoad(CharacterMainHandSlot)

"#,
    );
}

// wowforever: Blizzard_APIDocumentationGenerated/BankDocumentation.lua:61 (CanPurchaseBankTab).
// wowforever: Blizzard_APIDocumentationGenerated/BankDocumentation.lua:226 (FetchNumPurchasedBankTabs).
// wowforever: Blizzard_APIDocumentationGenerated/BankDocumentation.lua:241 (FetchPurchasedBankTabData).
// wowforever: Blizzard_APIDocumentationGenerated/BankDocumentation.lua:440 (BankTabData).
// wowforever: Blizzard_APIDocumentationGenerated/ItemConstantsDocumentation.lua:17 (BankType).
// Source-predicted FAIL: CanPurchaseBankTab missing; boolean, count and actual tab records, no eligibility/count expectations; not executed; native semantics unknown.
#[cfg(any(feature = "client-wowforever"))]
#[test]
fn original_bank_purchase_shape_blocker() {
    let env = shape::load(&["Blizzard_UIPanels_Game"]);
    shape::check(
        &env,
        r#"

local declared = {Character=0, Guild=1, Account=2}
enum(Enum.BankType, declared)
original(BankPanelMixin.SelectFirstAvailableTab, 'Blizzard_UIPanels_Game/Mainline/BankFrameTemplates.lua')
original(BankPanelMixin.ShouldShowPurchasePrompt, 'Blizzard_UIPanels_Game/Mainline/BankFrameTemplates.lua')
local schema = {{'ID','number',false}, {'bankType','number',false}, {'name','string',false},
    {'icon','number',false}, {'depositFlags','number',false},
    {'tabCleanupConfirmation','string',false}, {'tabNameEditBoxHeader','string',false}}
for _, bankType in ipairs({Enum.BankType.Character, Enum.BankType.Account}) do
    scalar('boolean', C_Bank.CanPurchaseBankTab(bankType))
    scalar('number', C_Bank.FetchNumPurchasedBankTabs(bankType))
    local tabs = scalar('table', C_Bank.FetchPurchasedBankTabData(bankType))
    for _, tab in ipairs(tabs) do
        fields(tab, schema)
        member(tab.bankType, declared)
    end
    -- Normal hidden panel configuration and original purchase-tab selection.
    BankPanel:Hide()
    BankPanel:SetBankType(bankType)
    BankPanel:FetchPurchasedBankTabData()
    BankPanel:SelectFirstAvailableTab()
    assert(BankPanel.PurchaseTab and BankPanel.PurchaseTab.tabData,
        'normal purchase-tab prerequisite')
    BankPanel:SelectTab(BankPanel.PurchaseTab.tabData.ID)
    BankPanel:ShouldShowPurchasePrompt()
    BankPanel:RefreshBankTabs()
    original(BankFrameMixin.OnShowOrHideBagCost, 'Blizzard_UIPanels_Game/Camelot/BankFrame.lua')
    BankFrameMixin.OnShowOrHideBagCost(BankFrame)
end

"#,
    );
}

// wowforever: Blizzard_APIDocumentationGenerated/SpellDocumentation.lua:631 (GetTargetSpellID).
// Source-predicted FAIL: GetTargetSpellID missing; numeric result only, no no-target sentinel or context outcome; not executed; native semantics unknown.
#[cfg(any(feature = "client-wowforever"))]
#[test]
fn original_target_spell_context_shape_blocker() {
    let env = shape::load(&["Blizzard_FrameXMLUtil"]);
    shape::check(
        &env,
        r#"

scalar('number', C_Spell.GetTargetSpellID())
original(ItemButtonUtil.GetItemContext, 'Blizzard_FrameXMLUtil/ItemUtil.lua')
for _, name in ipairs({'ScrappingMachineFrame','ItemInteractionFrame','ProfessionsFrame',
    'RuneforgeFrame','SoulbindViewer','ChallengesKeystoneFrame','ItemUpgradeFrame','BankPanel'}) do
    local frame = _G[name]
    if frame then frame:Hide() end
end
-- Normal non-targeting fixture reaches this final context check. No earlier predicate's
-- value or resulting context is a contract expectation.
ItemButtonUtil.GetItemContext()

"#,
    );
}
