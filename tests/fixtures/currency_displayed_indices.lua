-- Component integration over original cached TokenUI templates/methods and scroll provider.
-- Not a screenshot, pointer-dispatch, or native-client test.
function B5Snapshot()
    local rows = {}
    for i = 1, C_CurrencyInfo.GetCurrencyListSize() do
        local row = assert(C_CurrencyInfo.GetCurrencyListInfo(i))
        rows[#rows + 1] = table.concat({row.name, row.currencyTypesID,
            row.quantity, tostring(row.isHeaderExpanded)}, ":")
    end
    return table.concat(rows, "|")
end

function B5AssertProvider()
    local provider = assert(TokenFrame.ScrollBox:GetDataProvider())
    assert(provider:GetSize() == C_CurrencyInfo.GetCurrencyListSize())
    for i = 1, provider:GetSize() do
        local row = provider:Find(i)
        local expected = assert(C_CurrencyInfo.GetCurrencyListInfo(i))
        assert(row.currencyIndex == i)
        assert(row.name == expected.name and row.currencyTypesID == expected.currencyTypesID)
    end
end

function B5Header(index)
    local row = assert(C_CurrencyInfo.GetCurrencyListInfo(index))
    assert(row.isHeader)
    row.currencyIndex = index
    local header = CreateFrame("Button", nil, UIParent, "TokenHeaderTemplate")
    header:Initialize(row)
    return header
end

TokenFrame:Update()
B5AssertProvider()
