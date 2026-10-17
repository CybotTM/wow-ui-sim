//! Empty original refilter only. Populated cursor semantics remain a decision.
#![cfg(feature = "client-wrath")]
#[path = "wrath_unmasked_support/mod.rs"]
mod support;
use support::{env, load, run};

#[test]
fn wrath_unmasked_original_empty_refilter_clears_and_completes() {
    let env = env();
    load(
        &env,
        &[
            "GlobalStrings.lua",
            "Constants.lua",
            "Fonts.xml",
            "FontStyles.xml",
        ],
    );
    // Ordinary widget inputs, not replacements for any CombatLog producer,
    // refilter callback, formatter or history getter. Full chat XML is separate.
    run(
        &env,
        r#"
        ChatFrame2 = CreateFrame('ScrollingMessageFrame', 'ChatFrame2', UIParent)
        CombatLogQuickButtonFrameProgressBar = CreateFrame('StatusBar', 'CombatLogQuickButtonFrameProgressBar', UIParent)
    "#,
    );
    load(
        &env,
        &["../AddOns/Blizzard_CombatLog/Blizzard_CombatLog.lua"],
    );
    // Original :707-732 sets cursor zero then immediately calls :737-765.
    // Before fix, scalar 0 is truthy, reaches original formatter :2841 with a
    // nil event. After fix, assert cleared output AND completed refilter state.
    run(
        &env,
        r#"
        -- Original :3712-3716 selects the original default filter/settings.
        Blizzard_CombatLog_RefreshGlobalLinks()
        assert(type(Blizzard_CombatLog_CurrentSettings.settings) == 'table')
        CombatLogClearEntries()
        assert(CombatLogGetNumEntries() == 0)
        C_CombatLog._state.currentEntry = 37
        ChatFrame2:AddMessage('stale sentinel')
        assert(ChatFrame2:GetNumMessages() == 1)
        Blizzard_CombatLog_Refilter()
        assert(ChatFrame2:GetNumMessages() == 0)
        assert(ChatFrame2:GetMaxLines() == 300)
        local low, high = CombatLogQuickButtonFrameProgressBar:GetMinMaxValues()
        assert(low == 0 and high == 1)
        assert(CombatLogQuickButtonFrameProgressBar:GetValue() == 0)
        assert(CombatLogUpdateFrame.refiltering == false)
        assert(CombatLogUpdateFrame:GetScript('OnUpdate') == nil)
        assert(not CombatLogQuickButtonFrameProgressBar:IsShown())
        assert(C_CombatLog.GetCurrentEntryInfo() == nil)
        C_CombatLog._state.currentEntry = 37
        assert(C_CombatLog.GetCurrentEntryInfo() == nil)
        assert(CombatLogGetCurrentEntry() == nil)
        assert(CombatLogAdvanceEntry(-1) == false)
    "#,
    );
}
