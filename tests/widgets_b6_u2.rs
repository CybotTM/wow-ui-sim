use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
fn wrath() -> bool {
    ACTIVE == ClientProfile::Wrath
}
#[path = "common/wrath_label_fixtures.rs"]
mod originals;
use originals::{lfr_env, tooltip_env};

#[test]
fn b6_u2_empty_clear_is_silent() {
    if !wrath() {
        return;
    }
    let env = tooltip_env();
    env.exec(
        r#"
        local calls=0
        B6Tip:SetScript('OnTooltipCleared',function() calls=calls+1 end)
        B6Tip:ClearLines(); assert(calls==0,'empty clear must not dispatch')
        B6Tip:AddLine('x'); B6Tip:ClearLines(); assert(calls==1)
        B6Tip:ClearLines(); assert(calls==1)
    "#,
    )
    .unwrap();
}
#[test]
fn b6_u2_clear_rows_produced_by_original_lfr() {
    if !wrath() {
        return;
    }
    let env = lfr_env();
    // Direct ClearLines continuation after original LFR production, NOT a claim
    // that LFR calls ClearLines (its entry calls SetOwner).
    env.exec(
        r#"
        local tip=B6Tip
        tip:SetMinimumWidth(190)
        local w,h=tip:GetWidth(),tip:GetHeight()
        local n=tip:NumLines()
        B6TipStatusBar:Show()
        local calls=0
        tip:SetScript('OnTooltipCleared',function(self)
            calls=calls+1
            assert(self:NumLines()==0 and self:IsShown())
            assert(self:GetWidth()==w and self:GetHeight()==h)
            assert(self:GetMinimumWidth()==0)
            for i=1,n do
                local left=_G['B6TipTextLeft'..i]
                local right=_G['B6TipTextRight'..i]
                if left then
                    assert(not left:IsShown() and left:GetWidth()==0)
                    assert(not left:GetText() or left:GetText()=='')
                end
                if right then assert(not right:IsShown()) end
            end
            for i=1,10 do assert(not _G['B6TipTexture'..i]:IsShown()) end
            assert(not B6TipStatusBar:IsShown())
        end)
        tip:ClearLines(); assert(calls==1,'callback synchronous')
        tip:ClearLines(); assert(calls==1)
        tip:SetScript('OnTooltipCleared',nil)
        tip:AddLine('no handler'); tip:ClearLines(); assert(calls==1)
    "#,
    )
    .unwrap();
}
