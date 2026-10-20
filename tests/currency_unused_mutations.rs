#![cfg(feature = "client-retail")]

use crate::currency_displayed_indices::with_token_ui;

#[test]
fn currency_unused_mutations_original_checkbox_updates_selected_currency_and_provider() {
    with_token_ui(
        r#"
        -- Resolve from current displayed indices after a preceding category collapses.
        B5Header(1):ToggleCollapsed()
        local selected = assert(C_CurrencyInfo.GetCurrencyListInfo(3))
        assert(selected.currencyTypesID == 1792)
        TokenFrame.selectedID = 3
        TokenFrame.selectedToken = selected.name
        local checkbox = TokenFramePopup.InactiveCheckbox
        checkbox:SetChecked(true)
        checkbox:OnClick()
        local moved = assert(C_CurrencyInfo.GetCurrencyListInfo(TokenFrame.selectedID))
        assert(moved.currencyTypesID == 1792, "original checkbox must reselect its currency")
        assert(moved.isTypeUnused == true, "B5 unused: original checkbox must mark selected currency unused")
        assert(C_CurrencyInfo.GetCurrencyInfo(1792).isTypeUnused == true)
        assert(C_CurrencyInfo.GetCurrencyInfo(1792).quantity == 4350)
        local provider = TokenFrame.ScrollBox:GetDataProvider()
        assert(provider:Find(TokenFrame.selectedID).isTypeUnused == true,
            "original TokenFrame must display the unused flag")
        B5AssertProvider()
        checkbox:SetChecked(false)
        checkbox:OnClick()
        assert(C_CurrencyInfo.GetCurrencyListInfo(TokenFrame.selectedID).currencyTypesID == 1792)
        assert(C_CurrencyInfo.GetCurrencyListInfo(TokenFrame.selectedID).isTypeUnused == false)
        assert(C_CurrencyInfo.GetCurrencyInfo(1792).isTypeUnused == false)
        assert(TokenFrame.ScrollBox:GetDataProvider():Find(TokenFrame.selectedID).isTypeUnused == false)
        B5AssertProvider()
        "#,
    );
}
