#![cfg(feature = "client-retail")]

use crate::currency_displayed_indices::with_token_ui;

#[test]
fn currency_watched_mutations_use_displayed_indices_and_survive_collapse() {
    with_token_ui(
        r#"
        -- TokenUI declares the virtual watcher, but normal container startup
        -- instantiates it. Use its original template and supply a bounded width.
        BackpackTokenFrame = CreateFrame("Frame", "BackpackTokenFrame", UIParent, "BackpackTokenFrameTemplate")
        BackpackTokenFrame:SetWidth(300)
        B5Header(1):ToggleCollapsed()
        -- Original TokenFrame:SetTokenWatched removes the displayed Honor row.
        assert(TokenFrame:SetTokenWatched(3, false))
        C_CurrencyInfo.SetCurrencyBackpackByID(2245, false)
        C_CurrencyInfo.SetCurrencyBackpackByID(3089, false)
        assert(C_CurrencyInfo.GetBackpackCurrencyInfo(1) == nil, "B5 watched: supplied removals must empty backpack")
        assert(TokenFrame:SetTokenWatched(4, true))
        assert(C_CurrencyInfo.GetCurrencyListInfo(4).currencyTypesID == 1901)
        assert(C_CurrencyInfo.GetCurrencyListInfo(4).isShowInBackpack == true)
        assert(C_CurrencyInfo.GetCurrencyInfo(1901).isShowInBackpack == true)
        assert(C_CurrencyInfo.GetBackpackCurrencyInfo(1).currencyTypesID == 1901)
        assert(C_CurrencyInfo.GetCurrencyInfo(1901).quantity == 0)
        assert(C_CurrencyInfo.GetCurrencyInfo(1792).quantity == 4350)
        B5Header(2):ToggleCollapsed()
        assert(C_CurrencyInfo.GetBackpackCurrencyInfo(1).currencyTypesID == 1901)
        C_CurrencyInfo.SetCurrencyBackpackByID(1901, false)
        assert(C_CurrencyInfo.GetBackpackCurrencyInfo(1) == nil)
        C_CurrencyInfo.SetCurrencyBackpack(0, true)
        C_CurrencyInfo.SetCurrencyBackpack(1, true)
        C_CurrencyInfo.SetCurrencyBackpack(99, true)
        C_CurrencyInfo.SetCurrencyBackpackByID(-1, true)
        assert(C_CurrencyInfo.GetBackpackCurrencyInfo(1) == nil)
        B5AssertProvider()
        "#,
    );
}
